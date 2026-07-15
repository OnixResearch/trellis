#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang nixpkgs#nickel nixpkgs#b3sum -c cargo -q -Zscript
//! ```cargo
//! [dependencies]
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! ```

//! Deterministic verified-logic consumer release-profile validation.
//!
//! The pure core compares parsed profile and consumer values plus observed file
//! identities. The imperative shell exports Nickel, reads bounded files, emits
//! compatibility reports, and owns process exit behavior.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitCode;

use serde::Deserialize;
use serde::Serialize;

const PROFILE_SCHEMA: &str = "trellis-verified-logic-consumer-profile/v1";
const CONSUMER_SCHEMA: &str = "trellis-verified-logic-consumer/v1";
const REPORT_SCHEMA: &str = "trellis-verified-logic-compatibility-report/v1";
const PROFILE_PATH: &str = "consumer-profile/profile.ncl";
const GENERATED_PROFILE_PATH: &str = "consumer-profile/generated/profile.json";
const POSITIVE_FIXTURE_DIR: &str = "consumer-profile/fixtures/positive";
const NEGATIVE_FIXTURE_DIR: &str = "consumer-profile/fixtures/negative";
const GUIDE_PATH: &str = "docs/verified-logic-consumer-profile.md";
const README_PATH: &str = "README.md";
const CARGO_TOML_PATH: &str = "Cargo.toml";
const PACKAGE_NAME: &str = "verified-logic";
const SOURCE_REPOSITORY: &str = "https://github.com/OnixResearch/trellis";
const PROOF_OWNER: &str = "Trellis";
const DIRECT_MODE: &str = "direct_dependency";
const VENDORED_MODE: &str = "vendored_snapshot";
const VENDORED_ROOT: &str = "vendor/verified-logic";
const DIRECT_DIRTY_STATE_POLICY: &str = "not_applicable";
const VENDORED_DIRTY_STATE_POLICY: &str = "deny_unrecorded_changes";
const EXPECTED_NONE: &str = "none";
const SELF_TEST_ARG: &str = "--self-test";
const MANIFEST_ARG: &str = "--manifest";
const REPORT_ARG: &str = "--report";
const SOURCE_ROOT_ARG: &str = "--source-root";
const SOURCE_REVISION_ARG: &str = "--source-revision";
const HELP_ARG: &str = "--help";
const HELP_SHORT_ARG: &str = "-h";
const GIT_REVISION_HEX_LENGTH: usize = 40;
const BLAKE3_HEX_LENGTH: usize = 64;
const REQUIRED_BASE_ARTIFACT_COUNT: usize = 3;
const ADJACENT_WINDOW_SIZE: usize = 2;
const MINIMUM_POSITIVE_FIXTURES: usize = 2;
const MINIMUM_NEGATIVE_FIXTURES: usize = 7;
const MAXIMUM_MODULES: usize = 128;
const MAXIMUM_ARTIFACTS: usize = 256;
const MAXIMUM_PATCHES: usize = 64;
const MAXIMUM_NON_CLAIMS: usize = 64;
const MAXIMUM_ARTIFACT_BYTES: u64 = 1_048_576;
const REQUIRED_BASE_ARTIFACTS: &[&str] = &["Cargo.toml", "LICENSE", "src/lib.rs"];
const REQUIRED_NON_CLAIMS: &[&str] = &[
    "Imported Trellis primitives do not prove UCAN token correctness.",
    "Imported Trellis primitives do not prove downstream authorization policy.",
    "Imported Trellis primitives do not prove verifier soundness beyond Trellis's stated boundary.",
    "Imported Trellis primitives do not prove external consumer readiness.",
    "Imported Trellis primitives do not prove whole-system security.",
];
const GUIDE_REQUIRED_TERMS: &[&str] = &[
    "Direct dependency is the normal consumption route",
    "Vendoring is an audited fallback",
    "MPL-2.0",
    "do not prove UCAN token correctness",
    "do not prove downstream authorization policy",
    "do not prove verifier soundness beyond Trellis's stated boundary",
    "do not prove external consumer readiness",
    "do not prove whole-system security",
];
const PATCH_BOUNDARY_TERM: &str = "does not extend imported Trellis proof claims";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
struct ConsumerProfile {
    schema_version: String,
    report_schema: String,
    release_id: String,
    package: PackageProfile,
    vendored_snapshot: VendoredSnapshotPolicy,
    ucan_modules: Vec<String>,
    artifacts: Vec<ArtifactIdentity>,
    verification: VerificationEvidence,
    required_non_claims: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
struct PackageProfile {
    name: String,
    version: String,
    dependency_coordinate: String,
    source_repository: String,
    revision: String,
    default_features: bool,
    supported_features: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct ArtifactIdentity {
    path: String,
    bytes: u64,
    blake3: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
struct VerificationEvidence {
    proof_owner: String,
    proof_command: String,
    proof_manifest_path: String,
    proof_manifest_bytes: u64,
    proof_manifest_blake3: String,
    supported_claim: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
struct VendoredSnapshotPolicy {
    snapshot_root: String,
    dirty_state_policy: String,
    local_patch_policy: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
struct ConsumerManifest {
    schema_version: String,
    consumer_id: String,
    mode: String,
    dirty_state_policy: String,
    package_name: String,
    package_version: String,
    dependency_coordinate: String,
    source_repository: String,
    revision: String,
    default_features: bool,
    features: Vec<String>,
    modules: Vec<String>,
    snapshot_root: String,
    artifacts: Vec<ArtifactIdentity>,
    local_patches: Vec<LocalPatch>,
    non_claims: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
struct LocalPatch {
    path: String,
    bytes: u64,
    blake3: String,
    exception: PatchException,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
struct PatchException {
    approved: bool,
    rationale: String,
    verification_evidence: String,
    claim_boundary: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct ConsumerFixture {
    name: String,
    expected_valid: bool,
    expected_diagnostic: String,
    consumer: ConsumerManifest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RepositoryObservation {
    artifacts: BTreeMap<String, ArtifactIdentity>,
    proof_manifest: ArtifactIdentity,
    cargo_default_features_empty: bool,
    cargo_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct CompatibilityReport {
    schema_version: String,
    decision: String,
    release_id: String,
    consumer_id: String,
    mode: String,
    source_revision: String,
    modules_checked: usize,
    artifacts_checked: usize,
    supported_claim: String,
    non_claims: Vec<String>,
    diagnostics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct CompatibilityReportEnvelope {
    report: CompatibilityReport,
    report_blake3: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RepoCheck {
    modules_checked: usize,
    artifacts_checked: usize,
    fixtures_checked: usize,
    errors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Args {
    root: PathBuf,
    source_root: Option<PathBuf>,
    source_revision: Option<String>,
    manifest: Option<PathBuf>,
    report: Option<PathBuf>,
    self_test: bool,
}

fn validate_profile(profile: &ConsumerProfile, observed: &RepositoryObservation) -> Vec<String> {
    // r[impl trellis.crate_packaging.verified_logic_consumer_profile]
    // r[impl trellis.crate_packaging.verified_logic_consumer_profile.surface]
    // r[impl trellis.crate_packaging.verified_logic_consumer_profile.ucan_modules]
    // r[impl trellis.crate_packaging.verified_logic_consumer_profile.validation]
    let mut errors = BTreeSet::new();

    if profile.schema_version != PROFILE_SCHEMA {
        errors.insert(format!("profile.schema_version.unsupported: expected {PROFILE_SCHEMA}"));
    }
    if profile.report_schema != REPORT_SCHEMA {
        errors.insert(format!("profile.report_schema.unsupported: expected {REPORT_SCHEMA}"));
    }
    if profile.vendored_snapshot.snapshot_root != VENDORED_ROOT {
        errors.insert(format!("profile.snapshot_root.invalid: expected {VENDORED_ROOT}"));
    }
    if profile.vendored_snapshot.dirty_state_policy != VENDORED_DIRTY_STATE_POLICY {
        errors.insert(format!("profile.dirty_state_policy.invalid: expected {VENDORED_DIRTY_STATE_POLICY}"));
    }
    require_non_empty(&mut errors, &profile.vendored_snapshot.local_patch_policy, "profile.local_patch_policy.missing");
    require_non_empty(&mut errors, &profile.release_id, "profile.release_id.missing");
    if profile.package.name != PACKAGE_NAME {
        errors.insert(format!("profile.package.name.invalid: expected {PACKAGE_NAME}"));
    }
    require_non_empty(&mut errors, &profile.package.version, "profile.package.version.missing");
    if profile.package.source_repository != SOURCE_REPOSITORY {
        errors.insert(format!("profile.source_repository.invalid: expected {SOURCE_REPOSITORY}"));
    }
    validate_git_revision(&mut errors, &profile.package.revision, "profile.revision.invalid");
    let expected_coordinate = dependency_coordinate(&profile.package);
    if profile.package.dependency_coordinate != expected_coordinate {
        errors.insert(format!("profile.dependency_coordinate.invalid: expected {expected_coordinate}"));
    }
    if profile.package.default_features {
        errors.insert("profile.default_features.must_be_false".to_string());
    }
    if !observed.cargo_default_features_empty {
        errors.insert("profile.cargo.default_features_not_empty".to_string());
    }
    validate_sorted_unique(
        &mut errors,
        &profile.package.supported_features,
        "profile.supported_features",
        MAXIMUM_MODULES,
        true,
    );
    if profile.package.supported_features != observed.cargo_features {
        errors.insert("profile.supported_features.cargo_mismatch".to_string());
    }

    validate_sorted_unique(&mut errors, &profile.ucan_modules, "profile.ucan_modules", MAXIMUM_MODULES, false);
    validate_profile_artifacts(profile, observed, &mut errors);
    validate_verification(profile, observed, &mut errors);
    validate_required_non_claims(&profile.required_non_claims, "profile", &mut errors);

    errors.into_iter().collect()
}

fn validate_profile_artifacts(
    profile: &ConsumerProfile,
    observed: &RepositoryObservation,
    errors: &mut BTreeSet<String>,
) {
    let expected_artifact_count = profile.ucan_modules.len().checked_add(REQUIRED_BASE_ARTIFACT_COUNT);
    match expected_artifact_count {
        Some(expected) if profile.artifacts.len() == expected => {}
        Some(expected) => {
            errors.insert(format!("profile.artifacts.count: expected {expected}, got {}", profile.artifacts.len()));
        }
        None => {
            errors.insert("profile.artifacts.count_overflow".to_string());
        }
    }
    if profile.artifacts.len() > MAXIMUM_ARTIFACTS {
        errors.insert(format!("profile.artifacts.bound: maximum {MAXIMUM_ARTIFACTS}, got {}", profile.artifacts.len()));
    }
    let paths = profile.artifacts.iter().map(|artifact| artifact.path.clone()).collect::<Vec<_>>();
    validate_sorted_unique(errors, &paths, "profile.artifacts.paths", MAXIMUM_ARTIFACTS, false);

    let expected_paths = REQUIRED_BASE_ARTIFACTS
        .iter()
        .map(|path| (*path).to_string())
        .chain(profile.ucan_modules.iter().map(|module| format!("src/{module}.rs")))
        .collect::<BTreeSet<_>>();
    let actual_paths = paths.iter().cloned().collect::<BTreeSet<_>>();
    if actual_paths != expected_paths {
        errors.insert("profile.artifacts.module_coverage_mismatch".to_string());
    }

    for artifact in &profile.artifacts {
        validate_artifact(artifact, "profile.artifact", errors);
        match observed.artifacts.get(&artifact.path) {
            Some(measured) if measured == artifact => {}
            Some(measured) => {
                errors.insert(format!(
                    "profile.artifact.drift: {} expected {}:{} got {}:{}",
                    artifact.path, artifact.bytes, artifact.blake3, measured.bytes, measured.blake3
                ));
            }
            None => {
                errors.insert(format!("profile.artifact.missing: {}", artifact.path));
            }
        }
    }
}

fn validate_verification(profile: &ConsumerProfile, observed: &RepositoryObservation, errors: &mut BTreeSet<String>) {
    if profile.verification.proof_owner != PROOF_OWNER {
        errors.insert(format!("profile.verification.proof_owner: expected {PROOF_OWNER}"));
    }
    require_non_empty(errors, &profile.verification.proof_command, "profile.verification.proof_command.missing");
    if !is_safe_relative_path(&profile.verification.proof_manifest_path) {
        errors.insert("profile.verification.proof_manifest_path.unsafe".to_string());
    }
    if profile.verification.proof_manifest_path != observed.proof_manifest.path
        || profile.verification.proof_manifest_bytes != observed.proof_manifest.bytes
        || profile.verification.proof_manifest_blake3 != observed.proof_manifest.blake3
    {
        errors.insert("profile.verification.proof_manifest.drift".to_string());
    }
    let claim = profile.verification.supported_claim.to_ascii_lowercase();
    if !claim.contains("source and profile alignment") || !claim.contains("only") {
        errors.insert("profile.verification.supported_claim.overbroad".to_string());
    }
}

fn validate_consumer(profile: &ConsumerProfile, consumer: &ConsumerManifest) -> Vec<String> {
    // r[impl trellis.crate_packaging.verified_logic_consumer_profile.snapshot_compat]
    // r[impl trellis.crate_packaging.verified_logic_consumer_profile.direct_dependency]
    // r[impl trellis.crate_packaging.verified_logic_consumer_profile.validation]
    let mut errors = BTreeSet::new();

    if consumer.schema_version != CONSUMER_SCHEMA {
        errors.insert(format!("consumer.schema_version.unsupported: expected {CONSUMER_SCHEMA}"));
    }
    require_non_empty(&mut errors, &consumer.consumer_id, "consumer.consumer_id.missing");
    compare_required_string(&mut errors, &consumer.package_name, &profile.package.name, "consumer.package_name");
    compare_required_string(
        &mut errors,
        &consumer.package_version,
        &profile.package.version,
        "consumer.package_version",
    );
    compare_required_string(
        &mut errors,
        &consumer.dependency_coordinate,
        &profile.package.dependency_coordinate,
        "consumer.dependency_coordinate",
    );
    compare_required_string(
        &mut errors,
        &consumer.source_repository,
        &profile.package.source_repository,
        "consumer.source_repository",
    );
    compare_required_string(&mut errors, &consumer.revision, &profile.package.revision, "consumer.revision");
    if consumer.default_features != profile.package.default_features {
        errors.insert("consumer.default_features.mismatch".to_string());
    }
    if consumer.features != profile.package.supported_features {
        errors.insert("consumer.features.mismatch".to_string());
    }
    if consumer.modules != profile.ucan_modules {
        errors.insert("consumer.modules.mismatch".to_string());
    }
    validate_sorted_unique(&mut errors, &consumer.features, "consumer.features", MAXIMUM_MODULES, true);
    validate_sorted_unique(&mut errors, &consumer.modules, "consumer.modules", MAXIMUM_MODULES, false);
    validate_required_non_claims(&consumer.non_claims, "consumer", &mut errors);

    match consumer.mode.as_str() {
        DIRECT_MODE => validate_direct_consumer(consumer, &mut errors),
        VENDORED_MODE => validate_vendored_consumer(profile, consumer, &mut errors),
        other => {
            errors.insert(format!("consumer.mode.unsupported: {other}"));
        }
    }

    errors.into_iter().collect()
}

fn validate_direct_consumer(consumer: &ConsumerManifest, errors: &mut BTreeSet<String>) {
    if consumer.dirty_state_policy != DIRECT_DIRTY_STATE_POLICY {
        errors.insert(format!("consumer.direct.dirty_state_policy.mismatch: expected {DIRECT_DIRTY_STATE_POLICY}"));
    }
    if !consumer.snapshot_root.is_empty() {
        errors.insert("consumer.direct.snapshot_root.must_be_empty".to_string());
    }
    if !consumer.artifacts.is_empty() {
        errors.insert("consumer.direct.artifacts.must_be_empty".to_string());
    }
    if !consumer.local_patches.is_empty() {
        errors.insert("consumer.direct.local_patches.must_be_empty".to_string());
    }
}

fn validate_vendored_consumer(profile: &ConsumerProfile, consumer: &ConsumerManifest, errors: &mut BTreeSet<String>) {
    if consumer.dirty_state_policy != profile.vendored_snapshot.dirty_state_policy {
        errors.insert(format!(
            "consumer.dirty_state_policy.mismatch: expected {}",
            profile.vendored_snapshot.dirty_state_policy
        ));
    }
    if consumer.snapshot_root != VENDORED_ROOT {
        errors.insert(format!("consumer.snapshot_root.mismatch: expected {VENDORED_ROOT}"));
    }
    if consumer.artifacts != profile.artifacts {
        errors.insert("consumer.artifacts.mismatch".to_string());
    }
    if consumer.artifacts.len() > MAXIMUM_ARTIFACTS {
        errors
            .insert(format!("consumer.artifacts.bound: maximum {MAXIMUM_ARTIFACTS}, got {}", consumer.artifacts.len()));
    }
    for artifact in &consumer.artifacts {
        validate_artifact(artifact, "consumer.artifact", errors);
    }
    if consumer.local_patches.len() > MAXIMUM_PATCHES {
        errors.insert(format!(
            "consumer.local_patches.bound: maximum {MAXIMUM_PATCHES}, got {}",
            consumer.local_patches.len()
        ));
    }
    let mut patch_paths = BTreeSet::new();
    for patch in &consumer.local_patches {
        validate_patch(patch, &mut patch_paths, errors);
    }
}

fn validate_patch(patch: &LocalPatch, paths: &mut BTreeSet<String>, errors: &mut BTreeSet<String>) {
    if !is_safe_relative_path(&patch.path) {
        errors.insert(format!("consumer.patch.path.unsafe: {}", patch.path));
    }
    if !paths.insert(patch.path.clone()) {
        errors.insert(format!("consumer.patch.path.duplicate: {}", patch.path));
    }
    if patch.bytes == 0 {
        errors.insert(format!("consumer.patch.bytes.empty: {}", patch.path));
    }
    validate_blake3(errors, &patch.blake3, "consumer.patch.blake3.invalid");
    if !patch.exception.approved {
        errors.insert(format!("consumer.patch.unapproved: {}", patch.path));
        return;
    }
    if patch.exception.rationale.trim().is_empty()
        || patch.exception.verification_evidence.trim().is_empty()
        || patch.exception.claim_boundary.trim().is_empty()
    {
        errors.insert(format!("consumer.patch.exception_incomplete: {}", patch.path));
    }
    if !patch.exception.claim_boundary.to_ascii_lowercase().contains(PATCH_BOUNDARY_TERM) {
        errors.insert(format!("consumer.patch.claim_boundary.weakened: {}", patch.path));
    }
}

fn validate_artifact(artifact: &ArtifactIdentity, field: &str, errors: &mut BTreeSet<String>) {
    if !is_safe_relative_path(&artifact.path) {
        errors.insert(format!("{field}.path.unsafe: {}", artifact.path));
    }
    if artifact.bytes == 0 {
        errors.insert(format!("{field}.bytes.empty: {}", artifact.path));
    }
    if artifact.bytes > MAXIMUM_ARTIFACT_BYTES {
        errors.insert(format!("{field}.bytes.bound: {} exceeds {MAXIMUM_ARTIFACT_BYTES}", artifact.path));
    }
    validate_blake3(errors, &artifact.blake3, &format!("{field}.blake3.invalid"));
}

fn validate_required_non_claims(values: &[String], field: &str, errors: &mut BTreeSet<String>) {
    if values.len() > MAXIMUM_NON_CLAIMS {
        errors.insert(format!("{field}.non_claims.bound: maximum {MAXIMUM_NON_CLAIMS}, got {}", values.len()));
    }
    let claims = values.iter().map(String::as_str).collect::<BTreeSet<_>>();
    for required in REQUIRED_NON_CLAIMS {
        if !claims.contains(required) {
            errors.insert(format!("{field}.non_claims.missing: {required}"));
        }
    }
}

fn validate_sorted_unique(
    errors: &mut BTreeSet<String>,
    values: &[String],
    field: &str,
    maximum: usize,
    allow_empty: bool,
) {
    if !allow_empty && values.is_empty() {
        errors.insert(format!("{field}.empty"));
    }
    if values.len() > maximum {
        errors.insert(format!("{field}.bound: maximum {maximum}, got {}", values.len()));
    }
    if values.iter().any(|value| value.trim().is_empty()) {
        errors.insert(format!("{field}.contains_empty"));
    }
    if !values.windows(ADJACENT_WINDOW_SIZE).all(|pair| pair[0] < pair[1]) {
        errors.insert(format!("{field}.must_be_sorted_unique"));
    }
}

fn compare_required_string(errors: &mut BTreeSet<String>, actual: &str, expected: &str, field: &str) {
    if actual.trim().is_empty() {
        errors.insert(format!("{field}.missing"));
    } else if actual != expected {
        errors.insert(format!("{field}.stale: expected {expected}, got {actual}"));
    }
}

fn validate_git_revision(errors: &mut BTreeSet<String>, value: &str, diagnostic: &str) {
    if value.len() != GIT_REVISION_HEX_LENGTH
        || !value.chars().all(|character| character.is_ascii_hexdigit() && !character.is_ascii_uppercase())
    {
        errors.insert(diagnostic.to_string());
    }
}

fn validate_blake3(errors: &mut BTreeSet<String>, value: &str, diagnostic: &str) {
    if value.len() != BLAKE3_HEX_LENGTH
        || !value.chars().all(|character| character.is_ascii_hexdigit() && !character.is_ascii_uppercase())
    {
        errors.insert(diagnostic.to_string());
    }
}

fn require_non_empty(errors: &mut BTreeSet<String>, value: &str, diagnostic: &str) {
    if value.trim().is_empty() {
        errors.insert(diagnostic.to_string());
    }
}

fn dependency_coordinate(package: &PackageProfile) -> String {
    format!("git+{}?rev={}#{}@{}", package.source_repository, package.revision, package.name, package.version)
}

fn is_safe_relative_path(value: &str) -> bool {
    let path = Path::new(value);
    !value.is_empty()
        && !value.contains('\\')
        && !path.is_absolute()
        && path.components().all(|component| matches!(component, Component::Normal(_)))
}

fn build_report(profile: &ConsumerProfile, consumer: &ConsumerManifest) -> CompatibilityReport {
    let diagnostics = validate_consumer(profile, consumer);
    CompatibilityReport {
        schema_version: REPORT_SCHEMA.to_string(),
        decision: if diagnostics.is_empty() { "pass" } else { "deny" }.to_string(),
        release_id: profile.release_id.clone(),
        consumer_id: consumer.consumer_id.clone(),
        mode: consumer.mode.clone(),
        source_revision: consumer.revision.clone(),
        modules_checked: consumer.modules.len(),
        artifacts_checked: consumer.artifacts.len(),
        supported_claim: profile.verification.supported_claim.clone(),
        non_claims: profile.required_non_claims.clone(),
        diagnostics,
    }
}

fn envelope_report(report: CompatibilityReport) -> Result<CompatibilityReportEnvelope, String> {
    let canonical = serde_json::to_vec(&report).map_err(|error| format!("serializing report: {error}"))?;
    Ok(CompatibilityReportEnvelope {
        report,
        report_blake3: hash_bytes(&canonical)?,
    })
}

fn check_fixture(profile: &ConsumerProfile, fixture: &ConsumerFixture, path_name: &str) -> Vec<String> {
    // r[verify trellis.crate_packaging.verified_logic_consumer_profile.positive]
    // r[verify trellis.crate_packaging.verified_logic_consumer_profile.negative]
    // r[verify trellis.crate_packaging.verified_logic_consumer_profile.final_validation]
    let report = build_report(profile, &fixture.consumer);
    let diagnostics = &report.diagnostics;
    let mut errors = Vec::new();
    if fixture.expected_valid {
        if !diagnostics.is_empty() {
            errors.push(format!("{path_name}: {} expected valid but got {}", fixture.name, diagnostics.join("; ")));
        }
        if !fixture.expected_diagnostic.eq_ignore_ascii_case(EXPECTED_NONE) {
            errors.push(format!("{path_name}: valid fixture must set expected_diagnostic to {EXPECTED_NONE}"));
        }
    } else if fixture.expected_diagnostic.eq_ignore_ascii_case(EXPECTED_NONE) {
        errors.push(format!("{path_name}: invalid fixture must name an expected diagnostic"));
    } else if !diagnostics.iter().any(|diagnostic| diagnostic.contains(&fixture.expected_diagnostic)) {
        errors.push(format!(
            "{path_name}: expected diagnostic `{}` but got {}",
            fixture.expected_diagnostic,
            diagnostics.join("; ")
        ));
    }
    errors
}

fn observe_repository(root: &Path, profile: &ConsumerProfile) -> Result<RepositoryObservation, String> {
    let mut artifacts = BTreeMap::new();
    for expected in &profile.artifacts {
        let measured = measure_path(root, &expected.path)?;
        artifacts.insert(expected.path.clone(), measured);
    }
    let proof_manifest = measure_path(root, &profile.verification.proof_manifest_path)?;
    let cargo_manifest = read_bounded_file(&root.join(CARGO_TOML_PATH))?;
    let (cargo_default_features_empty, cargo_features) = parse_cargo_features(&cargo_manifest)?;
    Ok(RepositoryObservation {
        artifacts,
        proof_manifest,
        cargo_default_features_empty,
        cargo_features,
    })
}

fn measure_path(root: &Path, relative_path: &str) -> Result<ArtifactIdentity, String> {
    if !is_safe_relative_path(relative_path) {
        return Err(format!("unsafe observed path: {relative_path}"));
    }
    let path = root.join(relative_path);
    let bytes = read_bounded_file(&path)?;
    let byte_count = u64::try_from(bytes.len()).map_err(|_| format!("file length overflow: {}", path.display()))?;
    Ok(ArtifactIdentity {
        path: relative_path.to_string(),
        bytes: byte_count,
        blake3: hash_bytes(&bytes)?,
    })
}

fn hash_bytes(bytes: &[u8]) -> Result<String, String> {
    use std::io::Write;
    use std::process::Stdio;

    let mut child = Command::new("b3sum")
        .arg("--no-names")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("starting b3sum: {error}"))?;
    let mut stdin = child.stdin.take().ok_or("b3sum stdin unavailable")?;
    stdin.write_all(bytes).map_err(|error| format!("writing b3sum input: {error}"))?;
    drop(stdin);
    let output = child.wait_with_output().map_err(|error| format!("waiting for b3sum: {error}"))?;
    if !output.status.success() {
        return Err(format!("b3sum failed: {}", String::from_utf8_lossy(&output.stderr).trim()));
    }
    let digest = String::from_utf8(output.stdout)
        .map_err(|error| format!("b3sum output was not UTF-8: {error}"))?
        .trim()
        .to_string();
    let mut validation_errors = BTreeSet::new();
    validate_blake3(&mut validation_errors, &digest, "b3sum.output.invalid");
    if validation_errors.is_empty() {
        Ok(digest)
    } else {
        Err(validation_errors.into_iter().collect::<Vec<_>>().join("; "))
    }
}

fn read_bounded_file(path: &Path) -> Result<Vec<u8>, String> {
    let metadata = fs::metadata(path).map_err(|error| format!("reading metadata {}: {error}", path.display()))?;
    if !metadata.is_file() {
        return Err(format!("expected regular file: {}", path.display()));
    }
    if metadata.len() > MAXIMUM_ARTIFACT_BYTES {
        return Err(format!("file exceeds {MAXIMUM_ARTIFACT_BYTES} byte observation bound: {}", path.display()));
    }
    fs::read(path).map_err(|error| format!("reading {}: {error}", path.display()))
}

fn parse_cargo_features(bytes: &[u8]) -> Result<(bool, Vec<String>), String> {
    let text = std::str::from_utf8(bytes).map_err(|error| format!("Cargo.toml is not UTF-8: {error}"))?;
    let mut in_features = false;
    let mut default_empty = false;
    let mut features = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_features = trimmed == "[features]";
            continue;
        }
        if !in_features || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((name, value)) = trimmed.split_once('=') else {
            return Err(format!("malformed Cargo feature row: {trimmed}"));
        };
        let feature = name.trim();
        if feature == "default" {
            default_empty = value.trim() == "[]";
        } else {
            features.push(feature.to_string());
        }
    }
    features.sort();
    Ok((default_empty, features))
}

fn export_nickel<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    let output = Command::new("nickel")
        .args(["export", "--format", "json"])
        .arg(path)
        .output()
        .map_err(|error| format!("running nickel for {}: {error}", path.display()))?;
    if !output.status.success() {
        return Err(format!(
            "nickel export failed for {}: {}",
            path.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|error| format!("parsing Nickel JSON {}: {error}", path.display()))
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    let bytes = read_bounded_file(path)?;
    serde_json::from_slice(&bytes).map_err(|error| format!("parsing {}: {error}", path.display()))
}

fn check_docs(root: &Path) -> Result<Vec<String>, String> {
    // r[impl trellis.crate_packaging.verified_logic_consumer_profile.docs]
    let readme = String::from_utf8(read_bounded_file(&root.join(README_PATH))?)
        .map_err(|error| format!("{README_PATH} is not UTF-8: {error}"))?;
    let guide = String::from_utf8(read_bounded_file(&root.join(GUIDE_PATH))?)
        .map_err(|error| format!("{GUIDE_PATH} is not UTF-8: {error}"))?;
    let mut errors = Vec::new();
    if !readme.contains(GUIDE_PATH) || !readme.contains("check-verified-logic-consumer-profile.rs") {
        errors.push(format!(
            "{README_PATH} must link {GUIDE_PATH} and list scripts/check-verified-logic-consumer-profile.rs"
        ));
    }
    for term in GUIDE_REQUIRED_TERMS {
        if !guide.contains(term) {
            errors.push(format!("{GUIDE_PATH} must mention `{term}`"));
        }
    }
    Ok(errors)
}

fn fixture_paths(root: &Path, relative_dir: &str) -> Result<Vec<PathBuf>, String> {
    let directory = root.join(relative_dir);
    let mut paths = Vec::new();
    for entry in fs::read_dir(&directory).map_err(|error| format!("reading {}: {error}", directory.display()))? {
        let path = entry.map_err(|error| format!("reading {} entry: {error}", directory.display()))?.path();
        if path.extension().is_some_and(|extension| extension == "ncl") {
            paths.push(path);
        }
    }
    paths.sort();
    Ok(paths)
}

fn check_repo(root: &Path, source_root: &Path, source_revision: Option<&str>) -> Result<RepoCheck, String> {
    let profile: ConsumerProfile = export_nickel(&root.join(PROFILE_PATH))?;
    let generated: ConsumerProfile = read_json(&root.join(GENERATED_PROFILE_PATH))?;
    let observed = observe_repository(source_root, &profile)?;
    let mut errors = check_docs(root)?;
    validate_observed_source_revision(&profile, source_revision, &mut errors);
    if generated != profile {
        errors.push(format!("{GENERATED_PROFILE_PATH} is stale; regenerate it from {PROFILE_PATH}"));
    }
    errors.extend(validate_profile(&profile, &observed));

    let positive_paths = fixture_paths(root, POSITIVE_FIXTURE_DIR)?;
    let negative_paths = fixture_paths(root, NEGATIVE_FIXTURE_DIR)?;
    if positive_paths.len() < MINIMUM_POSITIVE_FIXTURES {
        errors.push(format!("{POSITIVE_FIXTURE_DIR} needs at least {MINIMUM_POSITIVE_FIXTURES} fixtures"));
    }
    if negative_paths.len() < MINIMUM_NEGATIVE_FIXTURES {
        errors.push(format!("{NEGATIVE_FIXTURE_DIR} needs at least {MINIMUM_NEGATIVE_FIXTURES} fixtures"));
    }
    let all_paths = positive_paths.into_iter().chain(negative_paths).collect::<Vec<_>>();
    for path in &all_paths {
        let fixture: ConsumerFixture = export_nickel(path)?;
        errors.extend(check_fixture(&profile, &fixture, &path.display().to_string()));
    }

    Ok(RepoCheck {
        modules_checked: profile.ucan_modules.len(),
        artifacts_checked: profile.artifacts.len(),
        fixtures_checked: all_paths.len(),
        errors,
    })
}

fn check_manifest(
    root: &Path,
    source_root: &Path,
    source_revision: Option<&str>,
    manifest_path: &Path,
    report_path: Option<&Path>,
) -> Result<CompatibilityReportEnvelope, String> {
    let profile: ConsumerProfile = export_nickel(&root.join(PROFILE_PATH))?;
    let observed = observe_repository(source_root, &profile)?;
    let mut profile_errors = validate_profile(&profile, &observed);
    validate_observed_source_revision(&profile, source_revision, &mut profile_errors);
    if !profile_errors.is_empty() {
        return Err(format!("release profile invalid: {}", profile_errors.join("; ")));
    }
    let consumer: ConsumerManifest = export_nickel(manifest_path)?;
    let report = envelope_report(build_report(&profile, &consumer))?;
    let json = serde_json::to_string_pretty(&report).map_err(|error| format!("serializing report: {error}"))?;
    match report_path {
        Some(path) => {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|error| format!("creating {}: {error}", parent.display()))?;
            }
            fs::write(path, format!("{json}\n")).map_err(|error| format!("writing {}: {error}", path.display()))?;
        }
        None => println!("{json}"),
    }
    Ok(report)
}

fn validate_observed_source_revision(
    profile: &ConsumerProfile,
    source_revision: Option<&str>,
    errors: &mut Vec<String>,
) {
    if let Some(observed) = source_revision {
        if observed != profile.package.revision {
            errors.push(format!(
                "profile.source_revision.mismatch: expected {}, got {observed}",
                profile.package.revision
            ));
        }
    }
}

fn parse_args(values: &[String]) -> Result<Args, String> {
    if values.iter().any(|value| value == HELP_ARG || value == HELP_SHORT_ARG) {
        return Err(usage());
    }
    if values == [SELF_TEST_ARG] {
        return Ok(Args {
            root: PathBuf::from("."),
            source_root: None,
            source_revision: None,
            manifest: None,
            report: None,
            self_test: true,
        });
    }
    let mut root = PathBuf::from(".");
    let mut source_root = None;
    let mut source_revision = None;
    let mut manifest = None;
    let mut report = None;
    let mut index = 0usize;
    while index < values.len() {
        match values[index].as_str() {
            MANIFEST_ARG => {
                index = index.checked_add(1).ok_or("argument index overflow")?;
                manifest = Some(PathBuf::from(values.get(index).ok_or_else(usage)?));
            }
            REPORT_ARG => {
                index = index.checked_add(1).ok_or("argument index overflow")?;
                report = Some(PathBuf::from(values.get(index).ok_or_else(usage)?));
            }
            SOURCE_ROOT_ARG => {
                index = index.checked_add(1).ok_or("argument index overflow")?;
                source_root = Some(PathBuf::from(values.get(index).ok_or_else(usage)?));
            }
            SOURCE_REVISION_ARG => {
                index = index.checked_add(1).ok_or("argument index overflow")?;
                source_revision = Some(values.get(index).ok_or_else(usage)?.clone());
            }
            value if !value.starts_with('-') && root == Path::new(".") => {
                root = PathBuf::from(value);
            }
            _ => return Err(usage()),
        }
        index = index.checked_add(1).ok_or("argument index overflow")?;
    }
    if report.is_some() && manifest.is_none() {
        return Err(format!("{REPORT_ARG} requires {MANIFEST_ARG}"));
    }
    if source_revision.is_some() && source_root.is_none() {
        return Err(format!("{SOURCE_REVISION_ARG} requires {SOURCE_ROOT_ARG}"));
    }
    Ok(Args {
        root,
        source_root,
        source_revision,
        manifest,
        report,
        self_test: false,
    })
}

fn usage() -> String {
    "usage: check-verified-logic-consumer-profile.rs [ROOT] [--source-root PATH --source-revision REV] [--manifest PATH [--report PATH]] | --self-test".to_string()
}

fn fixture_profile() -> (ConsumerProfile, RepositoryObservation) {
    let module = "ucan_fixture".to_string();
    let artifacts = [
        fixture_artifact("Cargo.toml", "cargo"),
        fixture_artifact("LICENSE", "license"),
        fixture_artifact("src/lib.rs", "lib"),
        fixture_artifact("src/ucan_fixture.rs", "module"),
    ]
    .into_iter()
    .collect::<Vec<_>>();
    let package = PackageProfile {
        name: PACKAGE_NAME.to_string(),
        version: "0.2.0".to_string(),
        dependency_coordinate: String::new(),
        source_repository: SOURCE_REPOSITORY.to_string(),
        revision: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        default_features: false,
        supported_features: Vec::new(),
    };
    let mut profile = ConsumerProfile {
        schema_version: PROFILE_SCHEMA.to_string(),
        report_schema: REPORT_SCHEMA.to_string(),
        release_id: "fixture-release".to_string(),
        package,
        vendored_snapshot: VendoredSnapshotPolicy {
            snapshot_root: VENDORED_ROOT.to_string(),
            dirty_state_policy: VENDORED_DIRTY_STATE_POLICY.to_string(),
            local_patch_policy: "Every patch requires an approved bounded exception.".to_string(),
        },
        ucan_modules: vec![module],
        artifacts: artifacts.clone(),
        verification: VerificationEvidence {
            proof_owner: PROOF_OWNER.to_string(),
            proof_command: "./scripts/verus.sh src/lib.rs".to_string(),
            proof_manifest_path: "docs/proof-artifact-manifest.json".to_string(),
            proof_manifest_bytes: fixture_bytes("proof"),
            proof_manifest_blake3: fixture_digest("proof"),
            supported_claim: "Source and profile alignment for imported Trellis verified primitives only.".to_string(),
        },
        required_non_claims: required_non_claims(),
    };
    profile.package.dependency_coordinate = dependency_coordinate(&profile.package);
    let observation = RepositoryObservation {
        artifacts: artifacts.into_iter().map(|artifact| (artifact.path.clone(), artifact)).collect(),
        proof_manifest: fixture_artifact("docs/proof-artifact-manifest.json", "proof"),
        cargo_default_features_empty: true,
        cargo_features: Vec::new(),
    };
    (profile, observation)
}

fn fixture_consumer(profile: &ConsumerProfile, mode: &str) -> ConsumerManifest {
    ConsumerManifest {
        schema_version: CONSUMER_SCHEMA.to_string(),
        consumer_id: "fixture-consumer".to_string(),
        mode: mode.to_string(),
        dirty_state_policy: if mode == VENDORED_MODE {
            VENDORED_DIRTY_STATE_POLICY.to_string()
        } else {
            DIRECT_DIRTY_STATE_POLICY.to_string()
        },
        package_name: profile.package.name.clone(),
        package_version: profile.package.version.clone(),
        dependency_coordinate: profile.package.dependency_coordinate.clone(),
        source_repository: profile.package.source_repository.clone(),
        revision: profile.package.revision.clone(),
        default_features: profile.package.default_features,
        features: profile.package.supported_features.clone(),
        modules: profile.ucan_modules.clone(),
        snapshot_root: if mode == VENDORED_MODE {
            VENDORED_ROOT.to_string()
        } else {
            String::new()
        },
        artifacts: if mode == VENDORED_MODE {
            profile.artifacts.clone()
        } else {
            Vec::new()
        },
        local_patches: Vec::new(),
        non_claims: profile.required_non_claims.clone(),
    }
}

fn fixture_artifact(path: &str, content: &str) -> ArtifactIdentity {
    ArtifactIdentity {
        path: path.to_string(),
        bytes: fixture_bytes(content),
        blake3: fixture_digest(content),
    }
}

fn fixture_bytes(content: &str) -> u64 {
    u64::try_from(content.len()).expect("fixture content length fits u64")
}

fn fixture_digest(_content: &str) -> String {
    "a".repeat(BLAKE3_HEX_LENGTH)
}

fn required_non_claims() -> Vec<String> {
    REQUIRED_NON_CLAIMS.iter().map(|claim| (*claim).to_string()).collect()
}

fn run_self_test() -> Result<(), String> {
    let (profile, observed) = fixture_profile();
    let profile_errors = validate_profile(&profile, &observed);
    if !profile_errors.is_empty() {
        return Err(format!("positive profile self-test failed: {}", profile_errors.join("; ")));
    }
    let mut source_revision_errors = Vec::new();
    validate_observed_source_revision(
        &profile,
        Some("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"),
        &mut source_revision_errors,
    );
    if !source_revision_errors.iter().any(|error| error.contains("profile.source_revision.mismatch")) {
        return Err("source revision mismatch self-test did not fail closed".to_string());
    }
    for mode in [DIRECT_MODE, VENDORED_MODE] {
        let report = build_report(&profile, &fixture_consumer(&profile, mode));
        if !report.diagnostics.is_empty() || report.decision != "pass" {
            return Err(format!("positive {mode} self-test failed"));
        }
    }

    let mut missing_revision = fixture_consumer(&profile, VENDORED_MODE);
    missing_revision.revision.clear();
    assert_diagnostic(&profile, &missing_revision, "consumer.revision.missing")?;

    let mut stale_modules = fixture_consumer(&profile, VENDORED_MODE);
    stale_modules.modules.clear();
    assert_diagnostic(&profile, &stale_modules, "consumer.modules.mismatch")?;

    let mut weakened_dirty_state = fixture_consumer(&profile, VENDORED_MODE);
    weakened_dirty_state.dirty_state_policy = DIRECT_DIRTY_STATE_POLICY.to_string();
    assert_diagnostic(&profile, &weakened_dirty_state, "consumer.dirty_state_policy.mismatch")?;

    let mut unapproved_patch = fixture_consumer(&profile, VENDORED_MODE);
    unapproved_patch.local_patches.push(LocalPatch {
        path: "src/local.rs".to_string(),
        bytes: fixture_bytes("patch"),
        blake3: fixture_digest("patch"),
        exception: PatchException {
            approved: false,
            rationale: String::new(),
            verification_evidence: String::new(),
            claim_boundary: String::new(),
        },
    });
    assert_diagnostic(&profile, &unapproved_patch, "consumer.patch.unapproved")?;

    let mut weakened = fixture_consumer(&profile, DIRECT_MODE);
    weakened.non_claims.clear();
    assert_diagnostic(&profile, &weakened, "consumer.non_claims.missing")?;
    Ok(())
}

fn assert_diagnostic(profile: &ConsumerProfile, consumer: &ConsumerManifest, expected: &str) -> Result<(), String> {
    let diagnostics = validate_consumer(profile, consumer);
    if diagnostics.iter().any(|diagnostic| diagnostic.contains(expected)) {
        Ok(())
    } else {
        Err(format!("expected diagnostic `{expected}`, got {}", diagnostics.join("; ")))
    }
}

fn main() -> ExitCode {
    let values = env::args().skip(1).collect::<Vec<_>>();
    let args = match parse_args(&values) {
        Ok(args) => args,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    };

    if args.self_test {
        return match run_self_test() {
            Ok(()) => {
                println!("verified-logic consumer profile self-test passed");
                ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("error: {error}");
                ExitCode::FAILURE
            }
        };
    }

    let source_root = args.source_root.as_deref().unwrap_or(&args.root);
    let source_revision = args.source_revision.as_deref();

    if let Some(manifest) = args.manifest.as_deref() {
        return match check_manifest(&args.root, source_root, source_revision, manifest, args.report.as_deref()) {
            Ok(report) if report.report.diagnostics.is_empty() => ExitCode::SUCCESS,
            Ok(report) => {
                for diagnostic in report.report.diagnostics {
                    eprintln!("error: {diagnostic}");
                }
                ExitCode::FAILURE
            }
            Err(error) => {
                eprintln!("error: {error}");
                ExitCode::FAILURE
            }
        };
    }

    match check_repo(&args.root, source_root, source_revision) {
        Ok(check) if check.errors.is_empty() => {
            println!(
                "verified-logic consumer profile passed: {} modules, {} artifacts, {} fixtures",
                check.modules_checked, check.artifacts_checked, check.fixtures_checked
            );
            ExitCode::SUCCESS
        }
        Ok(check) => {
            for error in check.errors {
                eprintln!("error: {error}");
            }
            ExitCode::FAILURE
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
