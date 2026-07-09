#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript
//! ```cargo
//! [dependencies]
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! ```

//! Deterministic checker for Trellis-owned UCAN bridge coverage manifests.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

use serde::Deserialize;

const MANIFEST_SCHEMA_VERSION: &str = "trellis.proof-artifact-manifest.v1";
const COVERAGE_SCHEMA_VERSION: &str = "trellis.ucan-bridge-coverage.v1";
const PROOF_MANIFEST_PATH: &str = "docs/proof-artifact-manifest.json";
const COVERAGE_PATH: &str = "docs/ucan-bridge-coverage.json";
const GUIDE_PATH: &str = "docs/ucan-bridge-coverage.md";
const CONTRACT_PATH: &str = "docs/ucan-bridge-coverage.contract.ncl";
const POSITIVE_FIXTURE_DIR: &str = "fixtures/ucan-bridge-coverage/positive";
const NEGATIVE_FIXTURE_DIR: &str = "fixtures/ucan-bridge-coverage/negative";
const README_PATH: &str = "README.md";
const SELF_TEST_ARG: &str = "--self-test";
const HELP_ARG: &str = "--help";
const HELP_SHORT_ARG: &str = "-h";
const ARGS_WITH_ROOT_COUNT: usize = 1;
const EMPTY_COUNT: usize = 0;
const BLAKE3_HEX_LENGTH: usize = 64;
const EXPECTED_NONE: &str = "none";
const FORMAL_PROOF_TIER: &str = "formal_proof";
const COVERAGE_PREFIX: &str = "trellis.ucan_bridge.";
const VENDORED_PATH_PREFIX: &str = "vendor/verified-logic/";
const OVERBROAD_CLAIM_WORDS: &[&str] = &[
    "proves ucan",
    "authorizes",
    "complete authorization",
    "parser correctness",
];
const ALLOWED_TIERS: &[&str] = &[FORMAL_PROOF_TIER, "manual_review", "recorded_identity"];
const REQUIRED_NON_CLAIMS: &[&str] = &[
    "does not prove ucan parser correctness",
    "does not prove resolver truth",
    "does not prove replay-store durability",
    "does not prove consumer authorization completeness",
];
const GUIDE_REQUIRED_TERMS: &[&str] = &[
    "Trellis owns primitive proof identity",
    "UCAN owns runtime bridge orchestration",
    "parser correctness",
    "resolver truth",
    "replay-store durability",
    "consumer authorization completeness",
];

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct PrimitiveManifest {
    schema_version: String,
    primitives: Vec<PrimitiveRow>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct PrimitiveRow {
    primitive_id: String,
    source: ArtifactIdentity,
    proof: Option<ArtifactIdentity>,
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
struct CoverageManifest {
    schema_version: String,
    producer: String,
    boundary: String,
    rows: Vec<CoverageRow>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct CoverageRow {
    bridge_id: String,
    ucan_helper: String,
    trellis_primitive_id: String,
    source_blake3: String,
    proof_blake3: String,
    expected_vendored_path: String,
    evidence_tier: String,
    claim: String,
    non_claims: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct FixtureFile {
    name: String,
    expected_valid: bool,
    expected_diagnostic: String,
    primitive_manifest: PrimitiveManifest,
    coverage_manifest: CoverageManifest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CoverageCheck {
    rows_checked: usize,
    errors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RepoCheck {
    rows_checked: usize,
    fixtures_checked: usize,
    errors: Vec<String>,
}

fn validate_coverage(primitive_manifest: &PrimitiveManifest, coverage: &CoverageManifest) -> CoverageCheck {
    // r[impl trellis.ucan_bridge_coverage.contract]
    // r[impl trellis.ucan_bridge_coverage.validation]
    // r[impl trellis.ucan_bridge_coverage.docs]
    let mut errors = Vec::new();
    if primitive_manifest.schema_version != MANIFEST_SCHEMA_VERSION {
        errors.push(format!(
            "primitive_manifest.schema_version must be {MANIFEST_SCHEMA_VERSION}, got {}",
            primitive_manifest.schema_version
        ));
    }
    if coverage.schema_version != COVERAGE_SCHEMA_VERSION {
        errors.push(format!(
            "coverage.schema_version must be {COVERAGE_SCHEMA_VERSION}, got {}",
            coverage.schema_version
        ));
    }
    require_non_empty(&mut errors, &coverage.producer, "coverage.producer");
    validate_boundary_text(&mut errors, &coverage.boundary, "coverage.boundary");
    if coverage.rows.is_empty() {
        errors.push("coverage.rows must not be empty".to_string());
    }

    let primitives = primitive_manifest
        .primitives
        .iter()
        .map(|primitive| (primitive.primitive_id.clone(), primitive.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut bridge_ids = BTreeSet::new();
    for row in &coverage.rows {
        validate_row(row, &primitives, &mut bridge_ids, &mut errors);
    }

    CoverageCheck {
        rows_checked: coverage.rows.len(),
        errors,
    }
}

fn validate_row(
    row: &CoverageRow,
    primitives: &BTreeMap<String, PrimitiveRow>,
    bridge_ids: &mut BTreeSet<String>,
    errors: &mut Vec<String>,
) {
    require_non_empty(errors, &row.bridge_id, "coverage.bridge_id");
    if !row.bridge_id.starts_with(COVERAGE_PREFIX) {
        errors.push(format!("coverage.bridge_id {} must start with {COVERAGE_PREFIX}", row.bridge_id));
    }
    if !row.bridge_id.is_empty() && !bridge_ids.insert(row.bridge_id.clone()) {
        errors.push(format!("coverage.bridge_id {} must be unique", row.bridge_id));
    }
    require_non_empty(errors, &row.ucan_helper, "coverage.ucan_helper");
    require_non_empty(errors, &row.trellis_primitive_id, "coverage.trellis_primitive_id");
    validate_blake3(errors, &row.source_blake3, "coverage.source_blake3");
    validate_blake3(errors, &row.proof_blake3, "coverage.proof_blake3");
    validate_allowed_value(errors, &row.evidence_tier, ALLOWED_TIERS, "coverage.evidence_tier");
    if !row.expected_vendored_path.starts_with(VENDORED_PATH_PREFIX) {
        errors.push(format!("coverage.expected_vendored_path must start with {VENDORED_PATH_PREFIX}"));
    }
    validate_claim(errors, &row.claim);
    validate_non_claims(errors, &row.non_claims);

    let Some(primitive) = primitives.get(&row.trellis_primitive_id) else {
        errors.push(format!("coverage.primitive.missing: {}", row.trellis_primitive_id));
        return;
    };
    if primitive.source.blake3 != row.source_blake3 {
        errors.push(format!(
            "coverage.source.stale_hash: {} expected {}, got {}",
            row.trellis_primitive_id, primitive.source.blake3, row.source_blake3
        ));
    }
    match &primitive.proof {
        Some(proof) if proof.blake3 == row.proof_blake3 => {}
        Some(proof) => errors.push(format!(
            "coverage.proof.stale_hash: {} expected {}, got {}",
            row.trellis_primitive_id, proof.blake3, row.proof_blake3
        )),
        None => errors.push(format!("coverage.proof.missing: {} has no proof artifact", row.trellis_primitive_id)),
    }
    if row.evidence_tier == FORMAL_PROOF_TIER && primitive.evidence_tier != FORMAL_PROOF_TIER {
        errors.push("coverage.evidence_tier.promotion: bridge tier must not exceed Trellis primitive tier".to_string());
    }
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

fn validate_claim(errors: &mut Vec<String>, claim: &str) {
    require_non_empty(errors, claim, "coverage.claim");
    let lower = claim.to_ascii_lowercase();
    if !lower.contains("coverage") || !lower.contains("identity") {
        errors.push("coverage.claim must describe identity coverage".to_string());
    }
    for word in OVERBROAD_CLAIM_WORDS {
        if lower.contains(word) {
            errors.push(format!("coverage.claim.overbroad must not contain `{word}`"));
        }
    }
}

fn validate_non_claims(errors: &mut Vec<String>, non_claims: &[String]) {
    require_non_empty_list(errors, non_claims, "coverage.non_claims");
    let joined = non_claims.join("; ").to_ascii_lowercase();
    for required in REQUIRED_NON_CLAIMS {
        if !joined.contains(required) {
            errors.push(format!("coverage.non_claims.required missing `{required}`"));
        }
    }
}

fn validate_blake3(errors: &mut Vec<String>, value: &str, field: &str) {
    if value.len() != BLAKE3_HEX_LENGTH || !value.chars().all(|character| character.is_ascii_hexdigit()) {
        errors.push(format!("{field} must be a {BLAKE3_HEX_LENGTH}-character BLAKE3 hex digest"));
    }
}

fn validate_allowed_value(errors: &mut Vec<String>, value: &str, allowed: &[&str], field: &str) {
    if !allowed.contains(&value) {
        errors.push(format!("{field} must be one of {}", allowed.join(", ")));
    }
}

fn require_non_empty(errors: &mut Vec<String>, value: &str, field_name: &str) {
    if value.trim().is_empty() {
        errors.push(format!("{field_name} must not be empty"));
    }
}

fn require_non_empty_list(errors: &mut Vec<String>, values: &[String], field_name: &str) {
    if values.len() == EMPTY_COUNT || values.iter().any(|value| value.trim().is_empty()) {
        errors.push(format!("{field_name} must contain only non-empty values"));
    }
}

fn check_fixture(fixture: &FixtureFile, path_name: &str) -> Vec<String> {
    // r[verify trellis.ucan_bridge_coverage.fixtures.positive]
    // r[verify trellis.ucan_bridge_coverage.fixtures.negative]
    // r[verify trellis.ucan_bridge_coverage.validation.boundary]
    // r[verify trellis.ucan_bridge_coverage.docs.boundary]
    // r[verify trellis.ucan_bridge_coverage.final_validation]
    // r[verify trellis.ucan_bridge_coverage.final_validation.fixtures]
    let check = validate_coverage(&fixture.primitive_manifest, &fixture.coverage_manifest);
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
    let readme = read_to_string(root, README_PATH)?;
    let guide = read_to_string(root, GUIDE_PATH)?;
    let contract = read_to_string(root, CONTRACT_PATH)?;
    let mut errors = Vec::new();
    if !readme.contains(GUIDE_PATH) || !readme.contains("check-ucan-bridge-coverage.rs") {
        errors.push(format!("{README_PATH} must link {GUIDE_PATH} and list scripts/check-ucan-bridge-coverage.rs"));
    }
    if !contract.contains(COVERAGE_SCHEMA_VERSION) || !contract.contains("CoverageRow") {
        errors.push(format!("{CONTRACT_PATH} must define the {COVERAGE_SCHEMA_VERSION} CoverageRow contract"));
    }
    for term in GUIDE_REQUIRED_TERMS {
        if !guide.contains(term) {
            errors.push(format!("{GUIDE_PATH} must mention `{term}`"));
        }
    }
    Ok(errors)
}

fn check_repo(root: &Path) -> Result<RepoCheck, String> {
    let mut errors = check_docs(root)?;
    let primitive_manifest: PrimitiveManifest = read_json(&root.join(PROOF_MANIFEST_PATH))?;
    let coverage_manifest: CoverageManifest = read_json(&root.join(COVERAGE_PATH))?;
    let check = validate_coverage(&primitive_manifest, &coverage_manifest);
    errors.extend(check.errors.into_iter().map(|error| format!("{COVERAGE_PATH}: {error}")));

    let mut fixtures_checked = 0usize;
    for path in fixture_paths(root)? {
        fixtures_checked += ARGS_WITH_ROOT_COUNT;
        let fixture: FixtureFile = read_json(&path)?;
        let path_name = path.display().to_string();
        errors.extend(check_fixture(&fixture, &path_name));
    }
    if fixtures_checked == EMPTY_COUNT {
        errors.push(format!("{POSITIVE_FIXTURE_DIR} and {NEGATIVE_FIXTURE_DIR} must contain fixture JSON"));
    }

    Ok(RepoCheck {
        rows_checked: check.rows_checked,
        fixtures_checked,
        errors,
    })
}

fn fixture_paths(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    collect_fixture_paths(root, POSITIVE_FIXTURE_DIR, &mut paths)?;
    collect_fixture_paths(root, NEGATIVE_FIXTURE_DIR, &mut paths)?;
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

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    let text = fs::read_to_string(path).map_err(|error| format!("reading {}: {error}", path.display()))?;
    serde_json::from_str(&text).map_err(|error| format!("parsing {}: {error}", path.display()))
}

fn read_to_string(root: &Path, relative_path: &str) -> Result<String, String> {
    let path = root.join(relative_path);
    fs::read_to_string(&path).map_err(|error| format!("reading {}: {error}", path.display()))
}

fn self_test_manifest(evidence_tier: &str) -> PrimitiveManifest {
    PrimitiveManifest {
        schema_version: MANIFEST_SCHEMA_VERSION.to_string(),
        primitives: vec![PrimitiveRow {
            primitive_id: "trellis.fixture.primitive".to_string(),
            source: ArtifactIdentity {
                path: "src/fixture.rs".to_string(),
                blake3: digest_a(),
                bytes: 12,
            },
            proof: Some(ArtifactIdentity {
                path: "src/fixture.rs".to_string(),
                blake3: digest_b(),
                bytes: 12,
            }),
            evidence_tier: evidence_tier.to_string(),
            non_claims: base_non_claims(),
        }],
    }
}

fn self_test_coverage() -> CoverageManifest {
    CoverageManifest {
        schema_version: COVERAGE_SCHEMA_VERSION.to_string(),
        producer: "self-test".to_string(),
        boundary: boundary_text(),
        rows: vec![CoverageRow {
            bridge_id: "trellis.ucan_bridge.fixture".to_string(),
            ucan_helper: "ucan::fixture".to_string(),
            trellis_primitive_id: "trellis.fixture.primitive".to_string(),
            source_blake3: digest_a(),
            proof_blake3: digest_b(),
            expected_vendored_path: "vendor/verified-logic/src/fixture.rs".to_string(),
            evidence_tier: FORMAL_PROOF_TIER.to_string(),
            claim: "identity coverage for a UCAN bridge helper".to_string(),
            non_claims: base_non_claims(),
        }],
    }
}

fn base_fixture(name: &str, expected_valid: bool, expected_diagnostic: &str) -> FixtureFile {
    FixtureFile {
        name: name.to_string(),
        expected_valid,
        expected_diagnostic: expected_diagnostic.to_string(),
        primitive_manifest: self_test_manifest(FORMAL_PROOF_TIER),
        coverage_manifest: self_test_coverage(),
    }
}

fn base_non_claims() -> Vec<String> {
    REQUIRED_NON_CLAIMS.iter().map(|claim| claim.to_string()).collect()
}

fn boundary_text() -> String {
    REQUIRED_NON_CLAIMS.join("; ")
}

fn digest_a() -> String {
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()
}

fn digest_b() -> String {
    "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string()
}

fn run_self_test() -> Result<(), String> {
    let positive = base_fixture("positive-self-test", true, EXPECTED_NONE);
    let positive_errors = check_fixture(&positive, "positive-self-test");
    if !positive_errors.is_empty() {
        return Err(format!("positive self-test failed: {}", positive_errors.join("; ")));
    }

    let mut stale = base_fixture("stale-self-test", false, "coverage.source.stale_hash");
    stale.coverage_manifest.rows[0].source_blake3 = digest_b();
    let stale_errors = check_fixture(&stale, "stale-self-test");
    if !stale_errors.is_empty() {
        return Err(format!("stale self-test failed: {}", stale_errors.join("; ")));
    }

    let mut missing = base_fixture("missing-self-test", false, "coverage.primitive.missing");
    missing.coverage_manifest.rows[0].trellis_primitive_id = "trellis.fixture.missing".to_string();
    let missing_errors = check_fixture(&missing, "missing-self-test");
    if !missing_errors.is_empty() {
        return Err(format!("missing self-test failed: {}", missing_errors.join("; ")));
    }

    let mut promotion = base_fixture("promotion-self-test", false, "coverage.evidence_tier.promotion");
    promotion.primitive_manifest = self_test_manifest("recorded_identity");
    let promotion_errors = check_fixture(&promotion, "promotion-self-test");
    if !promotion_errors.is_empty() {
        return Err(format!("promotion self-test failed: {}", promotion_errors.join("; ")));
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
    "usage: check-ucan-bridge-coverage.rs [--self-test|ROOT]".to_string()
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match parse_args(&args) {
        Ok(None) => run_self_test().map(|_| RepoCheck {
            rows_checked: ARGS_WITH_ROOT_COUNT,
            fixtures_checked: ARGS_WITH_ROOT_COUNT,
            errors: Vec::new(),
        }),
        Ok(Some(root)) => check_repo(&root),
        Err(error) => Err(error),
    };

    match result {
        Ok(check) if check.errors.is_empty() => {
            println!(
                "UCAN bridge coverage check passed: {} rows, {} fixtures checked",
                check.rows_checked, check.fixtures_checked
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
