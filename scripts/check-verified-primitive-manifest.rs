#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript
//! ```cargo
//! [dependencies]
//! blake3 = "=1.5.5"
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! ```

//! Deterministic checker for Trellis exported primitive proof-artifact manifests.
//!
//! The pure core validates parsed manifest rows against measured file identities.
//! The shell reads JSON, measures repository files with BLAKE3, and runs the
//! positive/negative fixture matrix.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::convert::TryFrom;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

use serde::Deserialize;

const SCHEMA_VERSION: &str = "trellis.proof-artifact-manifest.v1";
const MANIFEST_PATH: &str = "docs/proof-artifact-manifest.json";
const GUIDE_PATH: &str = "docs/proof-artifact-manifest.md";
const CONTRACT_PATH: &str = "docs/proof-artifact-manifest.contract.ncl";
const PRESERVES_ENVELOPE_PATH: &str = "docs/proof-artifact-manifest.preserves-envelope.json";
const PRESERVES_CARRIER_PATH: &str = "docs/proof-artifact-manifest.preserves";
const POSITIVE_FIXTURE_DIR: &str = "fixtures/proof-artifact-manifest/positive";
const NEGATIVE_FIXTURE_DIR: &str = "fixtures/proof-artifact-manifest/negative";
const POSITIVE_PRESERVES_FIXTURE_DIR: &str = "fixtures/proof-artifact-manifest-preserves/positive";
const NEGATIVE_PRESERVES_FIXTURE_DIR: &str = "fixtures/proof-artifact-manifest-preserves/negative";
const README_PATH: &str = "README.md";
const SELF_TEST_ARG: &str = "--self-test";
const HELP_ARG: &str = "--help";
const HELP_SHORT_ARG: &str = "-h";
const ARGS_WITH_ROOT_COUNT: usize = 1;
const BLAKE3_HEX_LENGTH: usize = 64;
const EMPTY_SLICE_COUNT: usize = 0;
const ZERO_BYTES: u64 = 0;
const MANIFEST_PREFIX: &str = "trellis.";
const EXPECTED_NONE: &str = "none";
const FORMAL_PROOF_TIER: &str = "formal_proof";
const VERIFIED_MODULE_BOUNDARY: &str = "verified_module";
const TRUSTED_BOUNDARY_PREFIX: &str = "trusted_";
const PROOF_GAP_BOUNDARY: &str = "proof_gap_boundary";
const MANUAL_REVIEW_BOUNDARY: &str = "manual_review_external";
const PRESERVES_SCHEMA_LABEL: &str = "trellis.proof-artifact-manifest.preserves.v1";
const PRESERVES_PREFIX: &str = "trellis.preserves.";
const CARRIER_CLAIM_WORD: &str = "carrier";
const OVERBROAD_CLAIM_WORDS: &[&str] = &["proves", "certifies", "authorizes", "release eligible"];

const ALLOWED_TIERS: &[&str] = &[FORMAL_PROOF_TIER, "manual_review", "recorded_identity"];
const ALLOWED_BOUNDARIES: &[&str] = &[
    VERIFIED_MODULE_BOUNDARY,
    "trusted_runtime_boundary",
    PROOF_GAP_BOUNDARY,
    MANUAL_REVIEW_BOUNDARY,
];
const REQUIRED_MANIFEST_GUIDE_TERMS: &[&str] = &[
    "UCAN",
    "Octet",
    "Valence",
    "Cairn",
    "Basalt",
    "Molten",
    "downstream integration correctness",
    "consumer release eligibility",
    "verifier soundness",
    "BLAKE3",
    "non-claims",
];
const REQUIRED_NON_CLAIMS: &[&str] = &[
    "downstream integration correctness",
    "consumer release eligibility",
    "verifier soundness",
];
const REQUIRED_PRESERVES_NON_CLAIMS: &[&str] = &[
    "not a proof artifact",
    "not downstream authorization semantics",
    "not verifier soundness",
];
const ALLOWED_PRESERVES_CONSUMERS: &[&str] = &[
    "ucan", "octet", "valence", "mantle", "cairn", "basalt", "aspen", "molten",
];
const PUBLIC_API_PREFIXES: &[&str] = &["pub fn ", "pub open spec fn ", "pub closed spec fn ", "pub proof fn "];

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct Manifest {
    schema_version: String,
    producer: String,
    boundary: String,
    primitives: Vec<PrimitiveRow>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct PrimitiveRow {
    primitive_id: String,
    module_path: String,
    public_api: Vec<String>,
    requirement_ids: Vec<String>,
    source: ArtifactIdentity,
    proof: Option<ArtifactIdentity>,
    verifier: VerifierIdentity,
    trusted_boundary: String,
    evidence_tier: String,
    non_claims: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct ArtifactIdentity {
    path: String,
    blake3: String,
    bytes: u64,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct VerifierIdentity {
    identity: String,
    argv: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct FixtureFile {
    name: String,
    expected_valid: bool,
    expected_diagnostic: String,
    manifest: Manifest,
    measurements: Vec<MeasurementInput>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct PreservesFixtureFile {
    name: String,
    expected_valid: bool,
    expected_diagnostic: String,
    source_manifest: Manifest,
    envelope_set: PreservesEnvelopeSet,
    measurements: Vec<MeasurementInput>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct PreservesEnvelopeSet {
    schema_version: String,
    producer: String,
    source_manifest: ArtifactIdentity,
    envelopes: Vec<PreservesEnvelopeRow>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct PreservesEnvelopeRow {
    envelope_id: String,
    schema_label: String,
    source_manifest_digest: String,
    primitive_ids: Vec<String>,
    canonical_preserves: ArtifactIdentity,
    allowed_consumers: Vec<String>,
    claim: String,
    non_claims: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct MeasurementInput {
    path: String,
    blake3: String,
    bytes: u64,
    #[serde(default)]
    text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Measurement {
    path: String,
    blake3: String,
    bytes: u64,
    text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ManifestCheck {
    primitive_count: usize,
    errors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PreservesCheck {
    envelope_count: usize,
    errors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RepoCheck {
    manifest_primitives: usize,
    preserves_envelopes: usize,
    fixtures_checked: usize,
    errors: Vec<String>,
}

fn measurements_from_inputs(inputs: &[MeasurementInput]) -> BTreeMap<String, Measurement> {
    let mut measurements = BTreeMap::new();
    for input in inputs {
        measurements.insert(input.path.clone(), Measurement {
            path: input.path.clone(),
            blake3: input.blake3.clone(),
            bytes: input.bytes,
            text: input.text.clone(),
        });
    }
    measurements
}

fn validate_manifest(manifest: &Manifest, measurements: &BTreeMap<String, Measurement>) -> ManifestCheck {
    // r[impl trellis.proof_artifact_manifest.contract]
    // r[impl trellis.proof_artifact_manifest.freshness]
    // r[impl trellis.proof_artifact_manifest.tiers]
    let mut errors = Vec::new();
    if manifest.schema_version != SCHEMA_VERSION {
        errors.push(format!("manifest.schema_version must be {SCHEMA_VERSION}, got {}", manifest.schema_version));
    }
    require_non_empty(&mut errors, &manifest.producer, "manifest.producer");
    validate_boundary_text(&mut errors, &manifest.boundary, "manifest.boundary");
    if manifest.primitives.is_empty() {
        errors.push("manifest.primitives must not be empty".to_string());
    }

    let mut ids = BTreeSet::new();
    for row in &manifest.primitives {
        validate_row(row, measurements, &mut ids, &mut errors);
    }

    ManifestCheck {
        primitive_count: manifest.primitives.len(),
        errors,
    }
}

fn validate_row(
    row: &PrimitiveRow,
    measurements: &BTreeMap<String, Measurement>,
    ids: &mut BTreeSet<String>,
    errors: &mut Vec<String>,
) {
    require_non_empty(errors, &row.primitive_id, "row.primitive_id");
    if !row.primitive_id.starts_with(MANIFEST_PREFIX) {
        errors.push(format!("row.primitive_id {} must start with {MANIFEST_PREFIX}", row.primitive_id));
    }
    if !row.primitive_id.is_empty() && !ids.insert(row.primitive_id.clone()) {
        errors.push(format!("row.primitive_id {} must be unique", row.primitive_id));
    }
    require_non_empty(errors, &row.module_path, "row.module_path");
    require_non_empty(errors, &row.trusted_boundary, "row.trusted_boundary");
    require_non_empty(errors, &row.evidence_tier, "row.evidence_tier");
    require_non_empty(errors, &row.verifier.identity, "row.verifier.identity");
    require_non_empty_list(errors, &row.public_api, "row.public_api");
    require_non_empty_list(errors, &row.requirement_ids, "row.requirement_ids");
    require_non_empty_list(errors, &row.non_claims, "row.non_claims");
    validate_allowed_value(errors, &row.trusted_boundary, ALLOWED_BOUNDARIES, "row.trusted_boundary");
    validate_allowed_value(errors, &row.evidence_tier, ALLOWED_TIERS, "row.evidence_tier");
    validate_non_claims(errors, &row.non_claims, "row.non_claims");
    validate_identity(errors, &row.source, measurements, "row.source");
    validate_api_surface(errors, row, measurements);

    if let Some(proof) = &row.proof {
        validate_identity(errors, proof, measurements, "row.proof");
    }

    if row.evidence_tier == FORMAL_PROOF_TIER {
        if row.trusted_boundary != VERIFIED_MODULE_BOUNDARY {
            errors.push("row.evidence_tier.promotion: trusted-boundary rows must not claim formal_proof".to_string());
        }
        if row.proof.is_none() {
            errors.push("row.proof must be present for formal_proof rows".to_string());
        }
        if row.verifier.argv.is_empty() {
            errors.push("row.verifier.argv must not be empty for formal_proof rows".to_string());
        }
    }

    if (row.trusted_boundary.starts_with(TRUSTED_BOUNDARY_PREFIX)
        || row.trusted_boundary == PROOF_GAP_BOUNDARY
        || row.trusted_boundary == MANUAL_REVIEW_BOUNDARY)
        && row.evidence_tier == FORMAL_PROOF_TIER
    {
        errors.push("row.evidence_tier.promotion: trusted or manual boundaries require downstream-owned verifier receipts before formal_proof use".to_string());
    }
}

fn validate_identity(
    errors: &mut Vec<String>,
    identity: &ArtifactIdentity,
    measurements: &BTreeMap<String, Measurement>,
    field: &str,
) {
    require_non_empty(errors, &identity.path, &format!("{field}.path"));
    validate_blake3(errors, &identity.blake3, &format!("{field}.blake3"));
    if identity.bytes == ZERO_BYTES {
        errors.push(format!("{field}.bytes must be positive"));
    }

    match measurements.get(&identity.path) {
        Some(measured) => {
            if measured.bytes != identity.bytes {
                errors.push(format!(
                    "{field}.stale_bytes: {} expected {} bytes, measured {} bytes",
                    identity.path, identity.bytes, measured.bytes
                ));
            }
            if measured.blake3 != identity.blake3 {
                errors.push(format!(
                    "{field}.stale_hash: {} expected {}, measured {}",
                    identity.path, identity.blake3, measured.blake3
                ));
            }
        }
        None => errors.push(format!("{field}.measurement missing for {}", identity.path)),
    }
}

fn validate_api_surface(errors: &mut Vec<String>, row: &PrimitiveRow, measurements: &BTreeMap<String, Measurement>) {
    let Some(measured) = measurements.get(&row.source.path) else {
        return;
    };
    for api in &row.public_api {
        if !contains_public_api(&measured.text, api) {
            errors.push(format!(
                "row.public_api.missing: {} does not export {} in {}",
                row.primitive_id, api, row.source.path
            ));
        }
    }
}

fn contains_public_api(source_text: &str, api: &str) -> bool {
    PUBLIC_API_PREFIXES.iter().any(|prefix| source_text.contains(&format!("{prefix}{api}")))
}

fn validate_boundary_text(errors: &mut Vec<String>, text: &str, field: &str) {
    require_non_empty(errors, text, field);
    let lower = text.to_ascii_lowercase();
    for required in REQUIRED_NON_CLAIMS {
        if !lower.contains(required) {
            errors.push(format!("{field} must mention `{required}`"));
        }
    }
}

fn validate_non_claims(errors: &mut Vec<String>, non_claims: &[String], field: &str) {
    let joined = non_claims.join("; ").to_ascii_lowercase();
    for required in REQUIRED_NON_CLAIMS {
        if !joined.contains(required) {
            errors.push(format!("{field}.required missing `{required}`"));
        }
    }
}

fn validate_allowed_value(errors: &mut Vec<String>, value: &str, allowed: &[&str], field: &str) {
    if !value.is_empty() && !allowed.contains(&value) {
        errors.push(format!("{field} must be one of {}", allowed.join(", ")));
    }
}

fn validate_blake3(errors: &mut Vec<String>, value: &str, field: &str) {
    if value.len() != BLAKE3_HEX_LENGTH || !value.chars().all(|character| character.is_ascii_hexdigit()) {
        errors.push(format!("{field} must be a {BLAKE3_HEX_LENGTH}-character BLAKE3 hex digest"));
    }
}

fn require_non_empty(errors: &mut Vec<String>, value: &str, field_name: &str) {
    if value.trim().is_empty() {
        errors.push(format!("{field_name} must not be empty"));
    }
}

fn require_non_empty_list(errors: &mut Vec<String>, values: &[String], field_name: &str) {
    if values.len() == EMPTY_SLICE_COUNT || values.iter().any(|value| value.trim().is_empty()) {
        errors.push(format!("{field_name} must contain only non-empty values"));
    }
}

fn validate_preserves_envelopes(
    envelope_set: &PreservesEnvelopeSet,
    source_manifest: &Manifest,
    measurements: &BTreeMap<String, Measurement>,
) -> PreservesCheck {
    // r[impl trellis.proof_artifact_manifest.preserves_envelope.contract]
    // r[impl trellis.proof_artifact_manifest.preserves_envelope.validation]
    // r[impl trellis.proof_artifact_manifest.preserves_envelope.docs]
    let mut errors = Vec::new();
    if envelope_set.schema_version != PRESERVES_SCHEMA_LABEL {
        errors.push(format!(
            "preserves.schema_version must be {PRESERVES_SCHEMA_LABEL}, got {}",
            envelope_set.schema_version
        ));
    }
    require_non_empty(&mut errors, &envelope_set.producer, "preserves.producer");
    validate_identity(&mut errors, &envelope_set.source_manifest, measurements, "preserves.source_manifest");

    let source_ids = source_manifest.primitives.iter().map(|row| row.primitive_id.clone()).collect::<BTreeSet<_>>();
    if source_ids.is_empty() {
        errors.push("preserves.source_manifest must contain primitive rows".to_string());
    }
    if envelope_set.envelopes.is_empty() {
        errors.push("preserves.envelopes must not be empty".to_string());
    }

    let mut envelope_ids = BTreeSet::new();
    for row in &envelope_set.envelopes {
        validate_preserves_row(
            row,
            &envelope_set.source_manifest,
            &source_ids,
            measurements,
            &mut envelope_ids,
            &mut errors,
        );
    }

    PreservesCheck {
        envelope_count: envelope_set.envelopes.len(),
        errors,
    }
}

fn validate_preserves_row(
    row: &PreservesEnvelopeRow,
    source_manifest_identity: &ArtifactIdentity,
    source_ids: &BTreeSet<String>,
    measurements: &BTreeMap<String, Measurement>,
    envelope_ids: &mut BTreeSet<String>,
    errors: &mut Vec<String>,
) {
    require_non_empty(errors, &row.envelope_id, "preserves.envelope_id");
    if !row.envelope_id.starts_with(PRESERVES_PREFIX) {
        errors.push(format!("preserves.envelope_id {} must start with {PRESERVES_PREFIX}", row.envelope_id));
    }
    if !row.envelope_id.is_empty() && !envelope_ids.insert(row.envelope_id.clone()) {
        errors.push(format!("preserves.envelope_id {} must be unique", row.envelope_id));
    }
    if row.schema_label != PRESERVES_SCHEMA_LABEL {
        errors.push(format!("preserves.schema_label must be {PRESERVES_SCHEMA_LABEL}, got {}", row.schema_label));
    }
    validate_blake3(errors, &row.source_manifest_digest, "preserves.source_manifest_digest");
    if row.source_manifest_digest != source_manifest_identity.blake3 {
        errors.push(format!(
            "preserves.source_manifest.stale_hash: expected {}, got {}",
            source_manifest_identity.blake3, row.source_manifest_digest
        ));
    }
    require_non_empty_list(errors, &row.primitive_ids, "preserves.primitive_ids");
    for primitive_id in &row.primitive_ids {
        if !source_ids.contains(primitive_id) {
            errors.push(format!("preserves.primitive_ids.missing: {primitive_id}"));
        }
    }
    validate_identity(errors, &row.canonical_preserves, measurements, "preserves.canonical_preserves");
    require_non_empty_list(errors, &row.allowed_consumers, "preserves.allowed_consumers");
    for consumer in &row.allowed_consumers {
        if !ALLOWED_PRESERVES_CONSUMERS.contains(&consumer.as_str()) {
            errors.push(format!("preserves.allowed_consumers unsupported `{consumer}`"));
        }
    }
    validate_preserves_claim(errors, &row.claim);
    validate_preserves_non_claims(errors, &row.non_claims);
}

fn validate_preserves_claim(errors: &mut Vec<String>, claim: &str) {
    require_non_empty(errors, claim, "preserves.claim");
    let lower = claim.to_ascii_lowercase();
    if !lower.contains(CARRIER_CLAIM_WORD) {
        errors.push("preserves.claim must describe a carrier boundary".to_string());
    }
    for word in OVERBROAD_CLAIM_WORDS {
        if lower.contains(word) {
            errors.push(format!("preserves.claim.overbroad must not contain `{word}`"));
        }
    }
}

fn validate_preserves_non_claims(errors: &mut Vec<String>, non_claims: &[String]) {
    require_non_empty_list(errors, non_claims, "preserves.non_claims");
    let joined = non_claims.join("; ").to_ascii_lowercase();
    for required in REQUIRED_PRESERVES_NON_CLAIMS {
        if !joined.contains(required) {
            errors.push(format!("preserves.non_claims.required missing `{required}`"));
        }
    }
}

fn check_fixture(fixture: &FixtureFile, path_name: &str) -> Vec<String> {
    // r[verify trellis.proof_artifact_manifest.positive]
    // r[verify trellis.proof_artifact_manifest.negative]
    // r[verify trellis.proof_artifact_manifest.freshness.pass]
    // r[verify trellis.proof_artifact_manifest.freshness.stale]
    // r[verify trellis.proof_artifact_manifest.tiers.no_promotion]
    // r[verify trellis.proof_artifact_manifest.tiers.non_claims]
    // r[verify trellis.proof_artifact_manifest.validation]
    // r[verify trellis.proof_artifact_manifest.validation.fixtures]
    let measurements = measurements_from_inputs(&fixture.measurements);
    let check = validate_manifest(&fixture.manifest, &measurements);
    check_expected_result(
        fixture.expected_valid,
        fixture.expected_diagnostic.trim(),
        &fixture.name,
        path_name,
        &check.errors,
    )
}

fn check_preserves_fixture(fixture: &PreservesFixtureFile, path_name: &str) -> Vec<String> {
    // r[verify trellis.proof_artifact_manifest.preserves_envelope.fixtures.positive]
    // r[verify trellis.proof_artifact_manifest.preserves_envelope.fixtures.negative]
    // r[verify trellis.proof_artifact_manifest.preserves_envelope.final_validation]
    // r[verify trellis.proof_artifact_manifest.preserves_envelope.final_validation.fixtures]
    let measurements = measurements_from_inputs(&fixture.measurements);
    let check = validate_preserves_envelopes(&fixture.envelope_set, &fixture.source_manifest, &measurements);
    check_expected_result(
        fixture.expected_valid,
        fixture.expected_diagnostic.trim(),
        &fixture.name,
        path_name,
        &check.errors,
    )
}

fn check_expected_result(
    expected_valid: bool,
    expected_diagnostic: &str,
    fixture_name: &str,
    path_name: &str,
    diagnostics: &[String],
) -> Vec<String> {
    let mut errors = Vec::new();
    if expected_valid {
        if !diagnostics.is_empty() {
            errors.push(format!(
                "{path_name}: {fixture_name} expected valid fixture but got: {}",
                diagnostics.join("; ")
            ));
        }
        if !expected_diagnostic.eq_ignore_ascii_case(EXPECTED_NONE) {
            errors.push(format!(
                "{path_name}: {fixture_name} valid fixture must set expected_diagnostic to `{EXPECTED_NONE}`"
            ));
        }
    } else if expected_diagnostic.is_empty() || expected_diagnostic.eq_ignore_ascii_case(EXPECTED_NONE) {
        errors.push(format!("{path_name}: {fixture_name} invalid fixture must declare an expected diagnostic"));
    } else if !diagnostics.iter().any(|error| error.contains(expected_diagnostic)) {
        errors.push(format!(
            "{path_name}: {fixture_name} expected diagnostic `{expected_diagnostic}` but got: {}",
            diagnostics.join("; ")
        ));
    }
    errors
}

fn check_docs(root: &Path) -> Result<Vec<String>, String> {
    // r[impl trellis.proof_artifact_manifest.downstream]
    // r[verify trellis.proof_artifact_manifest.downstream.ucan]
    // r[verify trellis.proof_artifact_manifest.downstream.octet]
    let readme = read_to_string(root, README_PATH)?;
    let guide = read_to_string(root, GUIDE_PATH)?;
    let contract = read_to_string(root, CONTRACT_PATH)?;
    let mut errors = Vec::new();

    if !readme.contains(GUIDE_PATH) {
        errors.push(format!("{README_PATH} must link {GUIDE_PATH}"));
    }
    if !readme.contains("check-verified-primitive-manifest.rs") {
        errors.push(format!("{README_PATH} must list scripts/check-verified-primitive-manifest.rs"));
    }
    if !contract.contains(SCHEMA_VERSION) || !contract.contains("PrimitiveRow") {
        errors.push(format!("{CONTRACT_PATH} must define the {SCHEMA_VERSION} PrimitiveRow contract"));
    }
    if !contract.contains(PRESERVES_SCHEMA_LABEL) || !contract.contains("PreservesEnvelopeRow") {
        errors.push(format!("{CONTRACT_PATH} must define the {PRESERVES_SCHEMA_LABEL} PreservesEnvelopeRow contract"));
    }
    if !guide.contains(PRESERVES_CARRIER_PATH) || !guide.contains("proof ownership") {
        errors.push(format!("{GUIDE_PATH} must document Preserves carrier proof ownership"));
    }
    for term in REQUIRED_MANIFEST_GUIDE_TERMS {
        if !guide.contains(term) {
            errors.push(format!("{GUIDE_PATH} must mention `{term}`"));
        }
    }

    Ok(errors)
}

fn check_repo(root: &Path) -> Result<RepoCheck, String> {
    let mut errors = check_docs(root)?;
    let manifest_path = root.join(MANIFEST_PATH);
    let manifest: Manifest = read_json(&manifest_path)?;
    let measurements = measure_manifest_files(root, &manifest)?;
    let manifest_check = validate_manifest(&manifest, &measurements);
    errors.extend(manifest_check.errors.into_iter().map(|error| format!("{MANIFEST_PATH}: {error}")));

    let preserves_path = root.join(PRESERVES_ENVELOPE_PATH);
    let preserves: PreservesEnvelopeSet = read_json(&preserves_path)?;
    let preserves_measurements = measure_preserves_files(root, &preserves)?;
    let preserves_check = validate_preserves_envelopes(&preserves, &manifest, &preserves_measurements);
    errors.extend(preserves_check.errors.into_iter().map(|error| format!("{PRESERVES_ENVELOPE_PATH}: {error}")));

    let mut fixtures_checked = 0usize;
    for path in fixture_paths(root)? {
        fixtures_checked += ARGS_WITH_ROOT_COUNT;
        let fixture: FixtureFile = read_json(&path)?;
        let path_name = path.display().to_string();
        errors.extend(check_fixture(&fixture, &path_name));
    }
    for path in preserves_fixture_paths(root)? {
        fixtures_checked += ARGS_WITH_ROOT_COUNT;
        let fixture: PreservesFixtureFile = read_json(&path)?;
        let path_name = path.display().to_string();
        errors.extend(check_preserves_fixture(&fixture, &path_name));
    }
    if fixtures_checked == EMPTY_SLICE_COUNT {
        errors.push(format!(
            "{POSITIVE_FIXTURE_DIR}, {NEGATIVE_FIXTURE_DIR}, {POSITIVE_PRESERVES_FIXTURE_DIR}, and {NEGATIVE_PRESERVES_FIXTURE_DIR} must contain fixture JSON"
        ));
    }

    Ok(RepoCheck {
        manifest_primitives: manifest_check.primitive_count,
        preserves_envelopes: preserves_check.envelope_count,
        fixtures_checked,
        errors,
    })
}

fn measure_manifest_files(root: &Path, manifest: &Manifest) -> Result<BTreeMap<String, Measurement>, String> {
    let mut paths = BTreeSet::new();
    for row in &manifest.primitives {
        paths.insert(row.source.path.clone());
        if let Some(proof) = &row.proof {
            paths.insert(proof.path.clone());
        }
    }

    measure_paths(root, paths)
}

fn measure_preserves_files(
    root: &Path,
    envelope_set: &PreservesEnvelopeSet,
) -> Result<BTreeMap<String, Measurement>, String> {
    let mut paths = BTreeSet::new();
    paths.insert(envelope_set.source_manifest.path.clone());
    for row in &envelope_set.envelopes {
        paths.insert(row.canonical_preserves.path.clone());
    }
    measure_paths(root, paths)
}

fn measure_paths(root: &Path, paths: BTreeSet<String>) -> Result<BTreeMap<String, Measurement>, String> {
    let mut measurements = BTreeMap::new();
    for path in paths {
        let measurement = measure_file(root, &path)?;
        measurements.insert(path, measurement);
    }
    Ok(measurements)
}

fn measure_file(root: &Path, relative_path: &str) -> Result<Measurement, String> {
    let path = root.join(relative_path);
    let bytes = fs::read(&path).map_err(|error| format!("reading {}: {error}", path.display()))?;
    let byte_count =
        u64::try_from(bytes.len()).map_err(|_| format!("{} is too large to count as u64 bytes", path.display()))?;
    let hash = blake3::hash(&bytes).to_hex().to_string();
    let text = String::from_utf8_lossy(&bytes).into_owned();
    Ok(Measurement {
        path: relative_path.to_string(),
        blake3: hash,
        bytes: byte_count,
        text,
    })
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    let text = fs::read_to_string(path).map_err(|error| format!("reading {}: {error}", path.display()))?;
    serde_json::from_str(&text).map_err(|error| format!("parsing {}: {error}", path.display()))
}

fn read_to_string(root: &Path, relative_path: &str) -> Result<String, String> {
    let path = root.join(relative_path);
    fs::read_to_string(&path).map_err(|error| format!("reading {}: {error}", path.display()))
}

fn fixture_paths(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    collect_fixture_paths(root, POSITIVE_FIXTURE_DIR, &mut paths)?;
    collect_fixture_paths(root, NEGATIVE_FIXTURE_DIR, &mut paths)?;
    paths.sort();
    Ok(paths)
}

fn preserves_fixture_paths(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    collect_fixture_paths(root, POSITIVE_PRESERVES_FIXTURE_DIR, &mut paths)?;
    collect_fixture_paths(root, NEGATIVE_PRESERVES_FIXTURE_DIR, &mut paths)?;
    paths.sort();
    Ok(paths)
}

fn collect_fixture_paths(root: &Path, relative_dir: &str, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    let dir = root.join(relative_dir);
    for entry in fs::read_dir(&dir).map_err(|error| format!("reading {}: {error}", dir.display()))? {
        let path = entry.map_err(|error| format!("reading {} entry: {error}", dir.display()))?.path();
        if path.extension().is_some_and(|extension| extension == "json") {
            paths.push(path);
        }
    }
    Ok(())
}

fn positive_self_test_fixture() -> FixtureFile {
    FixtureFile {
        name: "positive-self-test".to_string(),
        expected_valid: true,
        expected_diagnostic: EXPECTED_NONE.to_string(),
        manifest: Manifest {
            schema_version: SCHEMA_VERSION.to_string(),
            producer: "self-test".to_string(),
            boundary: boundary_text(),
            primitives: vec![base_row()],
        },
        measurements: vec![MeasurementInput {
            path: "src/example.rs".to_string(),
            blake3: digest_a(),
            bytes: 12,
            text: "pub fn example() {}".to_string(),
        }],
    }
}

fn stale_self_test_fixture() -> FixtureFile {
    let mut fixture = positive_self_test_fixture();
    fixture.name = "stale-self-test".to_string();
    fixture.expected_valid = false;
    fixture.expected_diagnostic = "row.source.stale_hash".to_string();
    fixture.measurements[0].blake3 = digest_b();
    fixture
}

fn promotion_self_test_fixture() -> FixtureFile {
    let mut fixture = positive_self_test_fixture();
    fixture.name = "promotion-self-test".to_string();
    fixture.expected_valid = false;
    fixture.expected_diagnostic = "row.evidence_tier.promotion".to_string();
    fixture.manifest.primitives[0].trusted_boundary = "trusted_runtime_boundary".to_string();
    fixture
}

fn base_row() -> PrimitiveRow {
    PrimitiveRow {
        primitive_id: "trellis.self_test.example".to_string(),
        module_path: "src/example.rs".to_string(),
        public_api: vec!["example".to_string()],
        requirement_ids: vec!["self-test.example".to_string()],
        source: ArtifactIdentity {
            path: "src/example.rs".to_string(),
            blake3: digest_a(),
            bytes: 12,
        },
        proof: Some(ArtifactIdentity {
            path: "src/example.rs".to_string(),
            blake3: digest_a(),
            bytes: 12,
        }),
        verifier: VerifierIdentity {
            identity: "verus".to_string(),
            argv: vec!["./scripts/verus.sh".to_string(), "src/lib.rs".to_string()],
        },
        trusted_boundary: VERIFIED_MODULE_BOUNDARY.to_string(),
        evidence_tier: FORMAL_PROOF_TIER.to_string(),
        non_claims: non_claims(),
    }
}

fn boundary_text() -> String {
    "identity and local checked proof status only; not downstream integration correctness; not consumer release eligibility; not verifier soundness beyond the declared proof boundary".to_string()
}

fn non_claims() -> Vec<String> {
    vec![
        "does not prove downstream integration correctness".to_string(),
        "does not prove consumer release eligibility".to_string(),
        "does not prove verifier soundness outside the declared proof boundary".to_string(),
    ]
}

fn digest_a() -> String {
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()
}

fn digest_b() -> String {
    "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string()
}

fn preserves_non_claims() -> Vec<String> {
    vec![
        "not a proof artifact".to_string(),
        "not downstream authorization semantics".to_string(),
        "not verifier soundness".to_string(),
    ]
}

fn positive_preserves_self_test_fixture() -> PreservesFixtureFile {
    PreservesFixtureFile {
        name: "positive-preserves-self-test".to_string(),
        expected_valid: true,
        expected_diagnostic: EXPECTED_NONE.to_string(),
        source_manifest: Manifest {
            schema_version: SCHEMA_VERSION.to_string(),
            producer: "self-test".to_string(),
            boundary: boundary_text(),
            primitives: vec![base_row()],
        },
        envelope_set: PreservesEnvelopeSet {
            schema_version: PRESERVES_SCHEMA_LABEL.to_string(),
            producer: "self-test".to_string(),
            source_manifest: ArtifactIdentity {
                path: MANIFEST_PATH.to_string(),
                blake3: digest_a(),
                bytes: 12,
            },
            envelopes: vec![PreservesEnvelopeRow {
                envelope_id: "trellis.preserves.self_test".to_string(),
                schema_label: PRESERVES_SCHEMA_LABEL.to_string(),
                source_manifest_digest: digest_a(),
                primitive_ids: vec!["trellis.self_test.example".to_string()],
                canonical_preserves: ArtifactIdentity {
                    path: PRESERVES_CARRIER_PATH.to_string(),
                    blake3: digest_b(),
                    bytes: 24,
                },
                allowed_consumers: vec!["ucan".to_string(), "cairn".to_string()],
                claim: "carrier for proof-manifest identity only".to_string(),
                non_claims: preserves_non_claims(),
            }],
        },
        measurements: vec![
            MeasurementInput {
                path: MANIFEST_PATH.to_string(),
                blake3: digest_a(),
                bytes: 12,
                text: "manifest".to_string(),
            },
            MeasurementInput {
                path: PRESERVES_CARRIER_PATH.to_string(),
                blake3: digest_b(),
                bytes: 24,
                text: "carrier".to_string(),
            },
        ],
    }
}

fn stale_preserves_self_test_fixture() -> PreservesFixtureFile {
    let mut fixture = positive_preserves_self_test_fixture();
    fixture.name = "stale-preserves-self-test".to_string();
    fixture.expected_valid = false;
    fixture.expected_diagnostic = "preserves.source_manifest.stale_hash".to_string();
    fixture.envelope_set.envelopes[0].source_manifest_digest = digest_b();
    fixture
}

fn overbroad_preserves_self_test_fixture() -> PreservesFixtureFile {
    let mut fixture = positive_preserves_self_test_fixture();
    fixture.name = "overbroad-preserves-self-test".to_string();
    fixture.expected_valid = false;
    fixture.expected_diagnostic = "preserves.claim.overbroad".to_string();
    fixture.envelope_set.envelopes[0].claim = "carrier proves downstream authorization semantics".to_string();
    fixture
}

fn run_self_test() -> Result<(), String> {
    let positive = positive_self_test_fixture();
    let positive_errors = check_fixture(&positive, "positive-self-test");
    if !positive_errors.is_empty() {
        return Err(format!("positive self-test failed: {}", positive_errors.join("; ")));
    }

    let stale = stale_self_test_fixture();
    let stale_errors = check_fixture(&stale, "stale-self-test");
    if !stale_errors.is_empty() {
        return Err(format!("stale self-test failed: {}", stale_errors.join("; ")));
    }

    let promotion = promotion_self_test_fixture();
    let promotion_errors = check_fixture(&promotion, "promotion-self-test");
    if !promotion_errors.is_empty() {
        return Err(format!("promotion self-test failed: {}", promotion_errors.join("; ")));
    }

    let missed_negative = FixtureFile {
        expected_diagnostic: "diagnostic that will not appear".to_string(),
        ..stale_self_test_fixture()
    };
    let missed_errors = check_fixture(&missed_negative, "missed-negative-self-test");
    if missed_errors.is_empty() {
        return Err("negative self-test must fail when expected diagnostic is absent".to_string());
    }

    let preserves_positive = positive_preserves_self_test_fixture();
    let preserves_positive_errors = check_preserves_fixture(&preserves_positive, "preserves-positive-self-test");
    if !preserves_positive_errors.is_empty() {
        return Err(format!("preserves positive self-test failed: {}", preserves_positive_errors.join("; ")));
    }

    let preserves_stale = stale_preserves_self_test_fixture();
    let preserves_stale_errors = check_preserves_fixture(&preserves_stale, "preserves-stale-self-test");
    if !preserves_stale_errors.is_empty() {
        return Err(format!("preserves stale self-test failed: {}", preserves_stale_errors.join("; ")));
    }

    let preserves_overbroad = overbroad_preserves_self_test_fixture();
    let preserves_overbroad_errors = check_preserves_fixture(&preserves_overbroad, "preserves-overbroad-self-test");
    if !preserves_overbroad_errors.is_empty() {
        return Err(format!("preserves overbroad self-test failed: {}", preserves_overbroad_errors.join("; ")));
    }

    Ok(())
}

fn parse_args(args: &[String]) -> Result<Option<PathBuf>, String> {
    match args {
        [] => Ok(Some(PathBuf::from("."))),
        [flag] if flag == SELF_TEST_ARG => Ok(None),
        [flag] if flag == HELP_ARG || flag == HELP_SHORT_ARG => Err(usage()),
        [root] => Ok(Some(PathBuf::from(root))),
        _ => Err(usage()),
    }
}

fn usage() -> String {
    "usage: check-verified-primitive-manifest.rs [--self-test|ROOT]".to_string()
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match parse_args(&args) {
        Ok(None) => run_self_test().map(|_| RepoCheck {
            manifest_primitives: ARGS_WITH_ROOT_COUNT,
            preserves_envelopes: ARGS_WITH_ROOT_COUNT,
            fixtures_checked: ARGS_WITH_ROOT_COUNT,
            errors: Vec::new(),
        }),
        Ok(Some(root)) => check_repo(&root),
        Err(error) => Err(error),
    };

    match result {
        Ok(check) if check.errors.is_empty() => {
            println!(
                "verified primitive manifest check passed: {} primitives, {} preserves envelopes, {} fixtures checked",
                check.manifest_primitives, check.preserves_envelopes, check.fixtures_checked
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
