use std::fs;
use std::path::Path;

use crate::model::ExportRequest;
use crate::model::InvalidFamily;
use crate::model::Profile;
use crate::model::REQUEST_SCHEMA;
use crate::model::ShellError;
use crate::shell;
use crate::shell::Tools;

const INVALID_PREFIX: &str = "invalid-";
const NICKEL_EXTENSION: &str = "ncl";
const SHELL_ERROR_SCHEMA: &str = "onix-nickel-export-shell-error/v1";
const EXPECTED_STAGE: &str = "evaluator-failure";
const INVALID_DESTINATION: &str = "generated/invalid.json";
const INVALID_REQUEST: &str = "request.json";
const INVALID_MANIFEST: &str = "generated/manifest.json";

pub fn check_fixture_parity(root: &Path, tools: &Tools, profile: &Profile) -> Result<usize, String> {
    let mut total = 0usize;
    for family in &profile.invalid_families {
        let fixtures = invalid_fixtures(root, family)?;
        if fixtures.is_empty() {
            return Err(format!("invalid family {} has no fixtures", family.id));
        }
        for fixture in fixtures {
            total = total.checked_add(1).ok_or("invalid fixture count overflow")?;
            check_fixture(root, tools, profile, family, &fixture, total)?;
        }
    }
    Ok(total)
}

fn invalid_fixtures(root: &Path, family: &InvalidFamily) -> Result<Vec<String>, String> {
    let directory = root.join(&family.directory);
    let mut paths = fs::read_dir(&directory)
        .map_err(|error| format!("read {}: {error}", directory.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("read invalid fixture entry: {error}"))?;
    paths.sort_by_key(std::fs::DirEntry::file_name);
    Ok(paths
        .into_iter()
        .filter_map(|entry| {
            let path = entry.path();
            let name = path.file_name()?.to_str()?;
            let extension = path.extension()?.to_str()?;
            if name.starts_with(INVALID_PREFIX) && extension == NICKEL_EXTENSION {
                path.strip_prefix(root).ok().map(|relative| relative.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect())
}

fn check_fixture(
    root: &Path,
    tools: &Tools,
    profile: &Profile,
    family: &InvalidFamily,
    fixture: &str,
    sequence: usize,
) -> Result<(), String> {
    let work = shell::fresh_temp_root(&format!("invalid-parity-{sequence}"))?;
    let result = check_fixture_in(root, &work, tools, profile, family, fixture);
    let cleanup = shell::remove_temp_root(&work);
    result.and(cleanup)
}

fn check_fixture_in(
    root: &Path,
    work: &Path,
    tools: &Tools,
    profile: &Profile,
    family: &InvalidFamily,
    fixture: &str,
) -> Result<(), String> {
    shell::copy_relative(root, work, fixture)?;
    for dependency in &family.dependencies {
        if dependency != fixture {
            shell::copy_relative(root, work, dependency)?;
        }
    }
    let file_name = Path::new(fixture)
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or("invalid fixture name is not UTF-8")?;
    let request = ExportRequest {
        schema: REQUEST_SCHEMA.to_owned(),
        family_id: format!("trellis.invalid.{}.{}", family.id, file_name),
        source: fixture.to_owned(),
        dependencies: family.dependencies.clone(),
        import_paths: Vec::new(),
        selector: String::new(),
        contract: String::new(),
        format: "json".to_owned(),
        destination: INVALID_DESTINATION.to_owned(),
        allow_secret_material: false,
    };
    let request_bytes = serde_json::to_vec_pretty(&request).map_err(|error| error.to_string())?;
    shell::write_relative(work, INVALID_REQUEST, &request_bytes)?;

    let legacy = shell::legacy_export(work, tools, &request)?;
    if legacy.status.success() || legacy.stderr.is_empty() {
        return Err(format!("legacy invalid fixture did not fail closed: {fixture}"));
    }
    let standalone = shell::standalone_export(work, tools, profile, INVALID_REQUEST, INVALID_MANIFEST, true)?;
    if standalone.status.success() || !standalone.stdout.is_empty() {
        return Err(format!("standalone invalid fixture emitted success evidence: {fixture}"));
    }
    let error: ShellError = serde_json::from_slice(&standalone.stderr)
        .map_err(|parse_error| format!("parse standalone error for {fixture}: {parse_error}"))?;
    if error.schema != SHELL_ERROR_SCHEMA || error.stage != EXPECTED_STAGE {
        return Err(format!("invalid fixture class drift for {fixture}: {} {}", error.schema, error.stage));
    }
    let legacy_message = String::from_utf8_lossy(&legacy.stderr);
    let normalized_legacy = normalize_process_addresses(legacy_message.trim());
    let normalized_standalone = normalize_process_addresses(&error.message);
    if normalized_standalone.contains(&normalized_legacy) {
        Ok(())
    } else {
        Err(format!(
            "standalone diagnostic drift for {fixture}\nlegacy:\n{normalized_legacy}\nstandalone:\n{normalized_standalone}"
        ))
    }
}

fn normalize_process_addresses(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut characters = input.chars().peekable();
    while let Some(character) = characters.next() {
        if character == '0' && characters.peek() == Some(&'x') {
            characters.next();
            let mut saw_hex = false;
            while characters.peek().is_some_and(|value| value.is_ascii_hexdigit()) {
                characters.next();
                saw_hex = true;
            }
            if saw_hex {
                output.push_str("<process-address>");
            } else {
                output.push_str("0x");
            }
        } else {
            output.push(character);
        }
    }
    output
}
