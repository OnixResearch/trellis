use std::fs;
use std::path::Path;

use crate::boundary;
use crate::model::ExportCase;
use crate::model::ExportManifest;
use crate::model::ExportReceipt;
use crate::model::ExportRequest;
use crate::model::Profile;
use crate::shell;
use crate::shell::Tools;
use crate::validation;

const PROFILE_GENERATED: &str = "nickel-export-cutover/generated/profile.json";

pub fn write(root: &Path, tools: &Tools, source_revision: &str) -> Result<Vec<String>, String> {
    let profile_bytes = shell::generate_profile(root, tools)?;
    shell::write_relative(root, PROFILE_GENERATED, &profile_bytes)?;
    let profile: Profile = parse_json(&profile_bytes, PROFILE_GENERATED)?;
    validation::validate_profile(&profile, source_revision)?;
    write_requests(root, tools, &profile)?;
    boundary::write(root, &profile.authority_baseline)?;

    let mut rows = Vec::new();
    for case in &profile.cases {
        let request = load_request(root, &profile, case)?;
        validate_legacy_destination(root, tools, case, &request, case.destination_preexisting)?;
        let output = shell::standalone_export(root, tools, &profile, &case.request_path, &case.manifest_path, true)?;
        let output = shell::require_success(output, &format!("standalone write {}", case.id))?;
        let receipt: ExportReceipt = parse_json(&output.stdout, "standalone receipt")?;
        validate_case(root, tools, &profile, case, &request, &receipt)?;
        rows.push(format!("{} {}", case.id, receipt.output.identity));
    }
    check(root, tools, source_revision)?;
    Ok(rows)
}

// r[impl trellis.nickel_export_cutover.dual_run]
pub fn check(root: &Path, tools: &Tools, source_revision: &str) -> Result<Vec<String>, String> {
    let generated_profile = shell::generate_profile(root, tools)?;
    let checked_profile = shell::read_relative(root, PROFILE_GENERATED)?;
    require_equal(&generated_profile, &checked_profile, "typed cutover profile is stale")?;
    let profile: Profile = parse_json(&checked_profile, PROFILE_GENERATED)?;
    validation::validate_profile(&profile, source_revision)?;
    check_requests(root, tools, &profile)?;
    boundary::check(root, &profile.authority_baseline)?;

    let mut rows = Vec::new();
    for case in &profile.cases {
        let request = load_request(root, &profile, case)?;
        let legacy_output = validate_legacy_destination(root, tools, case, &request, true)?;
        let standalone =
            shell::standalone_export(root, tools, &profile, &case.request_path, &case.manifest_path, false)?;
        let standalone = shell::require_success(standalone, &format!("standalone check {}", case.id))?;
        let receipt: ExportReceipt = parse_json(&standalone.stdout, "standalone receipt")?;
        validate_materials(root, &profile, case, &request, &receipt, &legacy_output)?;
        rows.push(format!("{} {}", case.id, receipt.output.identity));
    }
    Ok(rows)
}

pub fn load_profile(root: &Path) -> Result<Profile, String> {
    let bytes = shell::read_relative(root, PROFILE_GENERATED)?;
    parse_json(&bytes, PROFILE_GENERATED)
}

fn write_requests(root: &Path, tools: &Tools, profile: &Profile) -> Result<(), String> {
    for case in &profile.cases {
        let bytes = shell::generate_request(root, tools, &case.request_field)?;
        shell::write_relative(root, &case.request_path, &bytes)?;
    }
    Ok(())
}

fn check_requests(root: &Path, tools: &Tools, profile: &Profile) -> Result<(), String> {
    for case in &profile.cases {
        let generated = shell::generate_request(root, tools, &case.request_field)?;
        let checked = shell::read_relative(root, &case.request_path)?;
        require_equal(&generated, &checked, &format!("checked request is stale: {}", case.request_path))?;
    }
    Ok(())
}

fn load_request(root: &Path, profile: &Profile, case: &ExportCase) -> Result<ExportRequest, String> {
    let bytes = shell::read_relative(root, &case.request_path)?;
    let request: ExportRequest = parse_json(&bytes, &case.request_path)?;
    validation::validate_request(profile, case, &request)?;
    let profile_request: ExportRequest = serde_json::from_value(
        profile
            .requests
            .get(&case.request_field)
            .cloned()
            .ok_or_else(|| format!("missing profile request {}", case.request_field))?,
    )
    .map_err(|error| error.to_string())?;
    if request == profile_request {
        Ok(request)
    } else {
        Err(format!("request/profile drift for {}", case.id))
    }
}

// r[impl trellis.nickel_export_cutover.rollback]
fn validate_legacy_destination(
    root: &Path,
    tools: &Tools,
    case: &ExportCase,
    request: &ExportRequest,
    require_existing: bool,
) -> Result<Vec<u8>, String> {
    let legacy = shell::legacy_export(root, tools, request)?;
    let legacy = shell::require_success(legacy, &format!("legacy export {}", case.id))?;
    let destination_path = root.join(&request.destination);
    match fs::symlink_metadata(&destination_path) {
        Ok(_) => {
            let destination = shell::read_relative(root, &request.destination)?;
            require_equal(&legacy.stdout, &destination, &format!("legacy destination drift for {}", case.id))?;
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && !require_existing => {}
        Err(error) => {
            return Err(format!("inspect {}: {error}", destination_path.display()));
        }
    }
    for compatibility in &case.compatibility_destinations {
        let bytes = shell::read_relative(root, compatibility)?;
        require_equal(&legacy.stdout, &bytes, &format!("compatibility destination drift for {}", case.id))?;
    }
    Ok(legacy.stdout)
}

fn validate_case(
    root: &Path,
    tools: &Tools,
    profile: &Profile,
    case: &ExportCase,
    request: &ExportRequest,
    receipt: &ExportReceipt,
) -> Result<(), String> {
    let legacy = validate_legacy_destination(root, tools, case, request, true)?;
    validate_materials(root, profile, case, request, receipt, &legacy)
}

fn validate_materials(
    root: &Path,
    profile: &Profile,
    case: &ExportCase,
    request: &ExportRequest,
    receipt: &ExportReceipt,
    output: &[u8],
) -> Result<(), String> {
    let source = shell::read_relative(root, &request.source)?;
    let dependencies = request
        .dependencies
        .iter()
        .map(|path| shell::read_relative(root, path).map(|bytes| (path.clone(), bytes)))
        .collect::<Result<Vec<_>, _>>()?;
    validation::validate_receipt(profile, request, receipt, &source, &dependencies, output)?;
    let manifest_bytes = shell::read_relative(root, &case.manifest_path)?;
    let manifest: ExportManifest = parse_json(&manifest_bytes, &case.manifest_path)?;
    validation::validate_manifest(profile, request, receipt, &manifest)
}

fn parse_json<T: serde::de::DeserializeOwned>(bytes: &[u8], label: &str) -> Result<T, String> {
    serde_json::from_slice(bytes).map_err(|error| format!("parse {label}: {error}"))
}

fn require_equal(left: &[u8], right: &[u8], message: &str) -> Result<(), String> {
    if left == right { Ok(()) } else { Err(message.to_owned()) }
}
