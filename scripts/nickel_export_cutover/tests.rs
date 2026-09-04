use std::fs;
use std::path::Path;

use crate::boundary;
use crate::model::AUTHORITY_SCHEMA;
use crate::model::AuthorityBaseline;
use crate::model::AuthorityEntry;
use crate::model::EXPECTED_EXPORTER_REVISION;
use crate::model::ExportRequest;
use crate::model::Profile;
use crate::model::REQUEST_SCHEMA;
use crate::model::ShellError;
use crate::shell;
use crate::shell::Tools;
use crate::validation;

const REQUEST_PATH: &str = "requests/valid.json";
const MANIFEST_PATH: &str = "generated/manifest.json";
const OUTPUT_PATH: &str = "generated/value.txt";
const SOURCE_PATH: &str = "source.ncl";
const FIXTURE_ROOT: &str = "nickel-export-cutover/fixtures/negative";
const SHELL_ERROR_SCHEMA: &str = "onix-nickel-export-shell-error/v1";
const SAFE_SOURCE: &[u8] = b"{ value = \"safe\" }\n";
const TAMPERED_OUTPUT: &[u8] = b"tampered\n";
const TAMPERED_IDENTITY: &str = "b3:0000000000000000000000000000000000000000000000000000000000000000";
const EXPECTED_NEGATIVE_CASES: usize = 12;

// r[impl trellis.nickel_export_cutover.validation]
pub fn run(root: &Path, tools: &Tools, profile: &Profile) -> Result<usize, String> {
    let work = shell::fresh_temp_root("self-test")?;
    let result = run_in(root, &work, tools, profile);
    shell::remove_temp_root(&work)?;
    result
}

fn run_in(root: &Path, work: &Path, tools: &Tools, profile: &Profile) -> Result<usize, String> {
    shell::write_relative(work, SOURCE_PATH, SAFE_SOURCE)?;
    let request = valid_request();
    write_request(work, REQUEST_PATH, &request)?;
    let receipt = require_export(work, tools, profile, true)?;
    let receipt_value: serde_json::Value = serde_json::from_slice(&receipt).map_err(|error| error.to_string())?;
    validation::reject_overclaim(&receipt_value)?;
    require_export(work, tools, profile, false)?;

    shell::write_relative(work, OUTPUT_PATH, TAMPERED_OUTPUT)?;
    expect_stage(work, tools, profile, REQUEST_PATH, MANIFEST_PATH, false, "check-output")?;
    require_export(work, tools, profile, true)?;

    tamper_manifest(work)?;
    expect_stage(work, tools, profile, REQUEST_PATH, MANIFEST_PATH, false, "check-manifest")?;
    require_export(work, tools, profile, true)?;

    let mut drifted = profile.clone();
    drifted.evaluator.version = "0.0.0".to_owned();
    expect_stage(work, tools, &drifted, REQUEST_PATH, MANIFEST_PATH, false, "evaluator-version")?;

    copy_fixture(root, work, "malformed-request.json")?;
    expect_stage(work, tools, profile, "malformed-request.json", MANIFEST_PATH, false, "parse-spec")?;
    copy_fixture(root, work, "path-escape.json")?;
    expect_stage(work, tools, profile, "path-escape.json", MANIFEST_PATH, false, "validate-spec")?;
    copy_fixture(root, work, "invalid-contract.json")?;
    expect_stage(work, tools, profile, "invalid-contract.json", MANIFEST_PATH, false, "validate-spec")?;

    let secret = shell::read_relative(root, &format!("{FIXTURE_ROOT}/secret-source.ncl"))?;
    shell::write_relative(work, SOURCE_PATH, &secret)?;
    expect_stage(work, tools, profile, REQUEST_PATH, MANIFEST_PATH, true, "admit-receipt")?;
    shell::write_relative(work, SOURCE_PATH, SAFE_SOURCE)?;

    symlink_negative(work, tools, profile)?;
    overclaim_negative(root)?;
    boundary_negative()?;
    profile_opt_in_negatives(profile)?;

    Ok(EXPECTED_NEGATIVE_CASES)
}

fn valid_request() -> ExportRequest {
    ExportRequest {
        schema: REQUEST_SCHEMA.to_owned(),
        family_id: "trellis.cutover.self-test".to_owned(),
        source: SOURCE_PATH.to_owned(),
        dependencies: Vec::new(),
        import_paths: Vec::new(),
        selector: "value".to_owned(),
        contract: String::new(),
        format: "raw".to_owned(),
        destination: OUTPUT_PATH.to_owned(),
        allow_secret_material: false,
    }
}

fn write_request(root: &Path, path: &str, request: &ExportRequest) -> Result<(), String> {
    let bytes = serde_json::to_vec_pretty(request).map_err(|error| error.to_string())?;
    shell::write_relative(root, path, &bytes)
}

fn require_export(root: &Path, tools: &Tools, profile: &Profile, write: bool) -> Result<Vec<u8>, String> {
    let output = shell::standalone_export(root, tools, profile, REQUEST_PATH, MANIFEST_PATH, write)?;
    Ok(shell::require_success(output, "self-test standalone export")?.stdout)
}

fn expect_stage(
    root: &Path,
    tools: &Tools,
    profile: &Profile,
    request: &str,
    manifest: &str,
    write: bool,
    expected_stage: &str,
) -> Result<(), String> {
    let output = shell::standalone_export(root, tools, profile, request, manifest, write)?;
    if output.status.success() || !output.stdout.is_empty() {
        return Err(format!("negative case unexpectedly emitted success for {expected_stage}"));
    }
    let error: ShellError = serde_json::from_slice(&output.stderr)
        .map_err(|parse_error| format!("parse {expected_stage} error: {parse_error}"))?;
    if error.schema == SHELL_ERROR_SCHEMA && error.stage == expected_stage {
        Ok(())
    } else {
        Err(format!(
            "negative stage drift: expected {expected_stage}, got {} {}: {}",
            error.schema, error.stage, error.message
        ))
    }
}

fn tamper_manifest(root: &Path) -> Result<(), String> {
    let bytes = shell::read_relative(root, MANIFEST_PATH)?;
    let mut manifest: serde_json::Value = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    manifest["manifest_identity"] = serde_json::Value::String(TAMPERED_IDENTITY.to_owned());
    let mut rendered = serde_json::to_vec_pretty(&manifest).map_err(|error| error.to_string())?;
    rendered.push(b'\n');
    shell::write_relative(root, MANIFEST_PATH, &rendered)
}

fn copy_fixture(root: &Path, work: &Path, name: &str) -> Result<(), String> {
    shell::copy_relative(root, work, &format!("{FIXTURE_ROOT}/{name}"))?;
    let source = work.join(FIXTURE_ROOT).join(name);
    let destination = work.join(name);
    fs::rename(&source, &destination).map_err(|error| format!("move fixture {}: {error}", source.display()))
}

#[cfg(unix)]
fn symlink_negative(root: &Path, tools: &Tools, profile: &Profile) -> Result<(), String> {
    use std::os::unix::fs::symlink;

    require_export(root, tools, profile, true)?;
    let destination = root.join(OUTPUT_PATH);
    fs::remove_file(&destination).map_err(|error| format!("remove {}: {error}", destination.display()))?;
    symlink(root.join("outside.txt"), &destination).map_err(|error| format!("create test symlink: {error}"))?;
    expect_stage(root, tools, profile, REQUEST_PATH, MANIFEST_PATH, true, "unsafe-path")
}

#[cfg(not(unix))]
fn symlink_negative(_root: &Path, _tools: &Tools, _profile: &Profile) -> Result<(), String> {
    Err("symlink negative requires a Unix host".to_owned())
}

fn overclaim_negative(root: &Path) -> Result<(), String> {
    let path = format!("{FIXTURE_ROOT}/overclaim-receipt.json");
    let bytes = shell::read_relative(root, &path)?;
    let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    if validation::reject_overclaim(&value).is_err() {
        Ok(())
    } else {
        Err("proof/release overclaim fixture unexpectedly passed".to_owned())
    }
}

fn profile_opt_in_negatives(profile: &Profile) -> Result<(), String> {
    validation::validate_profile(profile, EXPECTED_EXPORTER_REVISION)?;
    let mut missing = profile.clone();
    missing.secret_material_opt_ins.pop();
    if validation::validate_profile(&missing, EXPECTED_EXPORTER_REVISION).is_ok() {
        return Err("missing secret marker opt-in unexpectedly passed".to_owned());
    }
    let mut drifted = profile.clone();
    let first = drifted.secret_material_opt_ins.first_mut().ok_or("secret marker opt-in fixture is empty")?;
    first.path = "cairn-policy/unreviewed.ncl".to_owned();
    if validation::validate_profile(&drifted, EXPECTED_EXPORTER_REVISION).is_ok() {
        Err("secret marker opt-in path drift unexpectedly passed".to_owned())
    } else {
        Ok(())
    }
}

fn boundary_negative() -> Result<(), String> {
    let expected = AuthorityBaseline {
        schema: AUTHORITY_SCHEMA.to_owned(),
        entries: Vec::new(),
    };
    boundary::compare(&expected, &expected)?;
    let actual = AuthorityBaseline {
        schema: AUTHORITY_SCHEMA.to_owned(),
        entries: vec![AuthorityEntry {
            path: "src/verified.rs".to_owned(),
            token: "std::process".to_owned(),
            count: 1,
        }],
    };
    if boundary::compare(&expected, &actual).is_err() {
        Ok(())
    } else {
        Err("authority drift negative unexpectedly passed".to_owned())
    }
}
