use std::collections::BTreeSet;
use std::path::Component;
use std::path::Path;

use serde::Serialize;

use crate::model::ArtifactIdentity;
use crate::model::EXPECTED_EXPORTER_REPOSITORY;
use crate::model::EXPECTED_EXPORTER_REVISION;
use crate::model::EXPECTED_NICKEL_IDENTITY;
use crate::model::EXPECTED_NICKEL_VERSION;
use crate::model::EXPECTED_NICKEL_VERSION_OUTPUT;
use crate::model::EvaluatorDescriptor;
use crate::model::ExportCase;
use crate::model::ExportManifest;
use crate::model::ExportReceipt;
use crate::model::ExportRequest;
use crate::model::GENERATOR_ID;
use crate::model::MANIFEST_SCHEMA;
use crate::model::NON_CLAIM_MARKERS;
use crate::model::PROFILE_SCHEMA;
use crate::model::Profile;
use crate::model::RECEIPT_SCHEMA;
use crate::model::REQUEST_SCHEMA;

const DECLARED_ONLY: &str = "declared_only";
const BLAKE3_PREFIX: &str = "b3:";
const ALLOWED_FORMATS: &[&str] = &["json", "yaml"];
const EXPECTED_SECRET_OPT_INS: usize = 6;
const EXPECTED_MARKER_HITS: usize = 1;
const SECRET_MARKERS: &[&str] = &[
    "begin private key",
    "password=",
    "password =",
    "token=",
    "token =",
    "secret=",
    "secret =",
    "api_key=",
    "api_key =",
];
const ALLOWED_SECRET_OPT_INS: &[(&str, &str, &str)] = &[
    ("trellis.cairn-policy", "cairn-policy/contracts.ncl", "MarkerToken ="),
    ("trellis.cairn-policy", "cairn-policy/generated/cairn-policy.base.json", "SECRET="),
    ("trellis.cairn-policy", "cairn-policy/generated/cairn-policy.base.json", "token="),
    ("trellis.json-artifact.cairn-policy", "cairn-policy/contracts.ncl", "MarkerToken ="),
    ("trellis.json-artifact.cairn-policy", "cairn-policy/generated/cairn-policy.json", "SECRET="),
    ("trellis.json-artifact.cairn-policy", "cairn-policy/generated/cairn-policy.json", "token="),
];

pub fn validate_profile(profile: &Profile, source_revision: &str) -> Result<(), String> {
    require(profile.schema == PROFILE_SCHEMA, "profile schema drift")?;
    require(profile.exporter.repository == EXPECTED_EXPORTER_REPOSITORY, "exporter repository drift")?;
    require(profile.exporter.revision == EXPECTED_EXPORTER_REVISION, "profile exporter revision drift")?;
    require(source_revision == EXPECTED_EXPORTER_REVISION, "release exporter revision drift")?;
    require(profile.evaluator.identity == EXPECTED_NICKEL_IDENTITY, "evaluator identity drift")?;
    require(profile.evaluator.version == EXPECTED_NICKEL_VERSION, "evaluator version drift")?;
    require(profile.evaluator.version_output == EXPECTED_NICKEL_VERSION_OUTPUT, "evaluator version output drift")?;
    require(safe_relative(&profile.authority_baseline), "unsafe authority baseline path")?;
    require(profile.secret_material_opt_ins.len() == EXPECTED_SECRET_OPT_INS, "secret marker opt-in count drift")?;
    let mut opt_ins = BTreeSet::new();
    for opt_in in &profile.secret_material_opt_ins {
        let key = (opt_in.family_id.as_str(), opt_in.path.as_str(), opt_in.marker.as_str());
        require(ALLOWED_SECRET_OPT_INS.contains(&key), "unreviewed secret marker opt-in")?;
        require(safe_relative(&opt_in.path), "unsafe secret marker opt-in path")?;
        require(!opt_in.rationale.trim().is_empty(), "empty secret marker rationale")?;
        require(opt_ins.insert(key), "duplicate secret marker opt-in")?;
    }
    require(!profile.cases.is_empty(), "profile requires positive cases")?;
    require(!profile.invalid_families.is_empty(), "profile requires invalid fixture families")?;

    let mut ids = BTreeSet::new();
    let mut request_paths = BTreeSet::new();
    let mut manifest_paths = BTreeSet::new();
    for case in &profile.cases {
        require(ids.insert(case.id.as_str()), "duplicate case id")?;
        require(request_paths.insert(case.request_path.as_str()), "duplicate request path")?;
        require(manifest_paths.insert(case.manifest_path.as_str()), "duplicate manifest path")?;
        require(safe_relative(&case.request_path), "unsafe request path")?;
        require(safe_relative(&case.manifest_path), "unsafe manifest path")?;
        require(profile.requests.get(&case.request_field).is_some(), "case request field is absent")?;
        for destination in &case.compatibility_destinations {
            require(safe_relative(destination), "unsafe compatibility destination")?;
        }
    }
    Ok(())
}

pub fn validate_request(profile: &Profile, case: &ExportCase, request: &ExportRequest) -> Result<(), String> {
    require(request.schema == REQUEST_SCHEMA, "request schema drift")?;
    require(!request.family_id.trim().is_empty(), "empty request family")?;
    require(safe_relative(&request.source), "unsafe request source")?;
    require(safe_relative(&request.destination), "unsafe request destination")?;
    require(request.contract.is_empty(), "Trellis requests must preserve empty contract metadata")?;
    let reviewed_opt_in = profile.secret_material_opt_ins.iter().any(|opt_in| opt_in.family_id == request.family_id);
    require(
        request.allow_secret_material == reviewed_opt_in,
        "request secret marker policy differs from its reviewed opt-in",
    )?;
    require(ALLOWED_FORMATS.contains(&request.format.as_str()), "unsupported production output format")?;
    require(
        case.request_path.starts_with("nickel-export-cutover/generated/requests/"),
        "request is outside the checked request root",
    )?;
    let mut paths = BTreeSet::new();
    for path in &request.dependencies {
        require(safe_relative(path), "unsafe dependency path")?;
        require(paths.insert(path.as_str()), "duplicate request dependency")?;
    }
    for path in &request.import_paths {
        require(safe_relative(path), "unsafe import path")?;
    }
    Ok(())
}

pub fn validate_receipt(
    profile: &Profile,
    request: &ExportRequest,
    receipt: &ExportReceipt,
    source: &[u8],
    dependencies: &[(String, Vec<u8>)],
    output: &[u8],
) -> Result<(), String> {
    require(receipt.schema == RECEIPT_SCHEMA, "receipt schema drift")?;
    require(receipt.family_id == request.family_id, "receipt family drift")?;
    require(receipt.source == artifact(&request.source, source)?, "receipt source identity drift")?;
    let mut expected_dependencies =
        dependencies.iter().map(|(path, bytes)| artifact(path, bytes)).collect::<Result<Vec<_>, _>>()?;
    expected_dependencies.sort_by(|left, right| left.path.cmp(&right.path));
    require(receipt.dependencies == expected_dependencies, "receipt dependency identity drift")?;
    validate_secret_opt_in(profile, request, source, dependencies)?;
    require(receipt.import_paths == sorted(&request.import_paths), "receipt import path drift")?;
    require(receipt.selector == request.selector, "receipt selector drift")?;
    require(receipt.contract == request.contract, "receipt contract drift")?;
    require(receipt.format == request.format, "receipt format drift")?;
    require(receipt.output == artifact(&request.destination, output)?, "receipt output identity drift")?;
    require(receipt.diagnostics.is_empty(), "production receipt contains diagnostics")?;
    require(receipt.evaluator == expected_evaluator(profile, request), "receipt evaluator drift")?;
    for marker in NON_CLAIM_MARKERS {
        require(receipt.non_claim.contains(marker), "receipt omitted a required non-claim")?;
    }
    Ok(())
}

pub fn validate_manifest(
    profile: &Profile,
    request: &ExportRequest,
    receipt: &ExportReceipt,
    manifest: &ExportManifest,
) -> Result<(), String> {
    require(manifest.schema == MANIFEST_SCHEMA, "manifest schema drift")?;
    require(manifest.generator == GENERATOR_ID, "manifest generator drift")?;
    require(manifest.evaluator == expected_evaluator(profile, request), "manifest evaluator drift")?;
    require(manifest.exports.as_slice() == core::slice::from_ref(receipt), "manifest export drift")?;
    let payload = ManifestPayload {
        schema: &manifest.schema,
        generator: &manifest.generator,
        evaluator: &manifest.evaluator,
        exports: &manifest.exports,
    };
    let bytes = serde_json::to_vec(&payload).map_err(|error| error.to_string())?;
    require(manifest.manifest_identity == blake3_identity(&bytes), "manifest identity drift")
}

pub fn reject_overclaim(value: &serde_json::Value) -> Result<(), String> {
    let claims = value.get("claims").and_then(serde_json::Value::as_array);
    let no_claims = match claims {
        Some(values) => values.is_empty(),
        None => true,
    };
    require(no_claims, "export evidence contains proof or release claims")
}

pub fn safe_relative(path: &str) -> bool {
    let candidate = Path::new(path);
    !path.is_empty()
        && !candidate.is_absolute()
        && candidate
            .components()
            .all(|component| !matches!(component, Component::ParentDir | Component::RootDir | Component::Prefix(_)))
}

pub fn blake3_identity(bytes: &[u8]) -> String {
    format!("{BLAKE3_PREFIX}{}", blake3::hash(bytes).to_hex())
}

fn validate_secret_opt_in(
    profile: &Profile,
    request: &ExportRequest,
    source: &[u8],
    dependencies: &[(String, Vec<u8>)],
) -> Result<(), String> {
    if !request.allow_secret_material {
        return Ok(());
    }
    let materials = core::iter::once((request.source.as_str(), source))
        .chain(dependencies.iter().map(|(path, bytes)| (path.as_str(), bytes.as_slice())))
        .collect::<Vec<_>>();
    let opt_ins = profile
        .secret_material_opt_ins
        .iter()
        .filter(|opt_in| opt_in.family_id == request.family_id)
        .collect::<Vec<_>>();
    require(!opt_ins.is_empty(), "secret marker request lacks reviewed opt-ins")?;
    for opt_in in &opt_ins {
        let bytes = materials
            .iter()
            .find(|(path, _)| *path == opt_in.path)
            .map(|(_, bytes)| *bytes)
            .ok_or_else(|| format!("secret marker opt-in path is undeclared: {}", opt_in.path))?;
        let text = String::from_utf8_lossy(bytes);
        require(
            text.matches(&opt_in.marker).count() == EXPECTED_MARKER_HITS,
            "reviewed secret marker occurrence drift",
        )?;
    }
    for (path, bytes) in materials {
        let mut scrubbed = String::from_utf8_lossy(bytes).into_owned();
        for opt_in in &opt_ins {
            if opt_in.path == path {
                scrubbed = scrubbed.replace(&opt_in.marker, "");
            }
        }
        let lowercase = scrubbed.to_ascii_lowercase();
        if SECRET_MARKERS.iter().any(|marker| lowercase.contains(marker)) {
            return Err(format!("unreviewed secret marker in `{path}`"));
        }
    }
    Ok(())
}

fn artifact(path: &str, bytes: &[u8]) -> Result<ArtifactIdentity, String> {
    Ok(ArtifactIdentity {
        path: path.to_owned(),
        identity: blake3_identity(bytes),
        bytes: u64::try_from(bytes.len()).map_err(|error| error.to_string())?,
    })
}

fn expected_evaluator(profile: &Profile, request: &ExportRequest) -> EvaluatorDescriptor {
    let mut options = vec![format!("format={}", request.format)];
    options.extend(request.import_paths.iter().map(|path| format!("import-path={path}")));
    if !request.selector.is_empty() {
        options.push(format!("selector={}", request.selector));
    }
    if !request.contract.is_empty() {
        options.push(format!("contract={}", request.contract));
    }
    options.sort();
    options.dedup();
    EvaluatorDescriptor {
        identity: profile.evaluator.identity.clone(),
        version: profile.evaluator.version.clone(),
        options,
        import_path_policy: DECLARED_ONLY.to_owned(),
    }
}

fn sorted(values: &[String]) -> Vec<String> {
    let mut result = values.to_vec();
    result.sort();
    result.dedup();
    result
}

fn require(condition: bool, message: &str) -> Result<(), String> {
    if condition { Ok(()) } else { Err(message.to_owned()) }
}

#[derive(Serialize)]
struct ManifestPayload<'a> {
    schema: &'a str,
    generator: &'a str,
    evaluator: &'a EvaluatorDescriptor,
    exports: &'a [ExportReceipt],
}
