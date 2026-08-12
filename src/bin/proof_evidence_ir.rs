//! Trellis proof-evidence IR chain: links source envelopes, canonical proof
//! evidence IR, and verifier receipts into a deterministic BLAKE3-bound chain
//! plus a Kamacite-ready handoff that does not require parsing Verus source.
//!
//! The pure core validates IR documents and recomputes canonical identities
//! from the checked-in proof-evidence bundle. The imperative shell reads
//! bounded repo files, writes the generated IR document, and owns process
//! exit behavior. It never claims evaluator, compiler, verifier, or release
//! soundness; it proves deterministic linkage facts only.
//!
//! Usage: proof_evidence_ir [check|write|self-test]
//!   check      (default) validate docs/proof-evidence-ir-chain.json and
//!              recompute the expected document from the current tree.
//!   write      regenerate docs/proof-evidence-ir-chain.json from the bundle.
//!   self-test  run positive and negative fixtures against the pure core.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;

const SCHEMA_VERSION: &str = "trellis-proof-evidence-ir/v1";
const HANDOFF_SCHEMA_VERSION: &str = "trellis-proof-evidence-kamacite-handoff/v1";
const IR_CHAIN_PATH: &str = "docs/proof-evidence-ir-chain.json";
const BUNDLE_PATH: &str = "docs/proof-evidence-bundle.json";
const REQUIREMENTS_PATH: &str = "docs/requirements.md";
const PROOF_GAP_PATH: &str = "docs/proof-gap-sites.tsv";
const GENERATOR_PATH: &str = "src/bin/proof_evidence_ir.rs";
const REQUIREMENT_PREFIX: &str = "r[";
const REQUIREMENT_SUFFIX: char = ']';
const PROOF_GAP_SEPARATOR: char = '\t';
const BLAKE3_ALGORITHM: &str = "blake3";
const RESULT_PASSED: &str = "passed";
const RESULT_FAILED: &str = "failed";
const STATUS_VERIFIED: &str = "verified";
const COVERAGE_COVERED: &str = "covered";
const DECLARED_SOLVER_IDENTITY: &str = "z3 (declared solver of the pinned Verus toolchain; no version claim)";
const TRUSTED_BOUNDARY_CLASSIFICATION: &str = "permanent-trust-boundary";
const REQUIRED_NON_CLAIMS: &[&str] = &[
    "not release approval",
    "not whole-program correctness",
    "not third-party conformance",
    "not verifier soundness",
    "not compiler correctness",
];
const FORBIDDEN_CLAIM_FRAGMENTS: &[&str] = &[
    "release approval",
    "whole-program correctness",
    "certification",
    "third-party conformance",
    "production safety",
    "verifier soundness",
    "compiler correctness",
    "release eligibility",
];

// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.contract]
// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.verifier_receipt]
// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.assumptions]
// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.handoff]
// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.negative]
// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.positive]
// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.evidence]
// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.docs]
// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.validation]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct IrChain {
    schema_version: String,
    generated_by: String,
    source_bundle_path: String,
    records: Vec<IrRecord>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct IrRecord {
    scope: String,
    requirement_ids: Vec<String>,
    property_ids: Vec<String>,
    source_identities: Vec<ContentIdentity>,
    proof_ir_identity: String,
    verifier_receipt: VerifierReceipt,
    assumptions: Assumptions,
    status: String,
    recorded_only: bool,
    claims: Vec<String>,
    non_claims: Vec<String>,
    chain_identity: String,
    handoff: KamaciteHandoff,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct ContentIdentity {
    path: String,
    algorithm: String,
    digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct VerifierReceipt {
    verifier_identity: String,
    toolchain_identity: String,
    solver_identity: String,
    command_identity: String,
    input_roots: Vec<String>,
    result_identity: String,
    result_status: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Assumptions {
    trusted_boundaries: Vec<String>,
    external_bodies: Vec<String>,
    proof_gaps: Vec<String>,
    dependencies: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct KamaciteHandoff {
    schema: String,
    source_identity: String,
    proof_ir_identity: String,
    verifier_receipt_identity: String,
    assumption_identity: String,
    requirement_ids: Vec<String>,
    non_claims: Vec<String>,
}

/// Proof bundle record shape consumed from the checked-in bundle document.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct BundleRecord {
    scope: String,
    requirement_ids: Vec<String>,
    source_paths: Vec<String>,
    proof_command: String,
    verifier_toolchain: String,
    proof_outcome: String,
    tracey_coverage: String,
    proof_gap_references: Vec<String>,
    content_identities: Vec<ContentIdentity>,
    claims: Vec<String>,
    non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct ProofBundle {
    schema_version: String,
    generated_by: String,
    records: Vec<BundleRecord>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct RepoFacts {
    requirements: BTreeSet<String>,
    gap_classifications: Vec<(String, String)>,
}

fn blake3_hex(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

/// Canonical sorted-key JSON bytes for deterministic identity computation.
fn canonical_json_bytes(value: &Value) -> Vec<u8> {
    serde_json::to_vec(value).expect("serializable JSON value")
}

fn canonical_object(entries: Vec<(&str, Value)>) -> Value {
    let mut map = Map::new();
    for (key, value) in entries {
        map.insert(key.to_string(), value);
    }
    Value::Object(map)
}

fn sorted_strings(values: &[String]) -> Vec<String> {
    let mut sorted = values.to_vec();
    sorted.sort();
    sorted
}

fn source_identity(source_identities: &[ContentIdentity]) -> String {
    let rows: Vec<Value> = source_identities
        .iter()
        .map(|identity| {
            canonical_object(vec![
                ("path".to_string().as_str(), Value::String(identity.path.clone())),
                ("algorithm".to_string().as_str(), Value::String(identity.algorithm.clone())),
                ("digest".to_string().as_str(), Value::String(identity.digest.clone())),
            ])
        })
        .collect();
    let sorted = {
        let mut rows = rows;
        rows.sort_by_key(ToString::to_string);
        rows
    };
    let payload = canonical_object(vec![("source_identities".to_string().as_str(), Value::Array(sorted))]);
    blake3_hex(&canonical_json_bytes(&payload))
}

fn receipt_identity(receipt: &VerifierReceipt) -> String {
    let payload = canonical_object(vec![
        ("verifier_identity".to_string().as_str(), Value::String(receipt.verifier_identity.clone())),
        ("toolchain_identity".to_string().as_str(), Value::String(receipt.toolchain_identity.clone())),
        ("solver_identity".to_string().as_str(), Value::String(receipt.solver_identity.clone())),
        ("command_identity".to_string().as_str(), Value::String(receipt.command_identity.clone())),
        ("input_roots".to_string().as_str(), json_array(sorted_strings(&receipt.input_roots))),
        ("result_identity".to_string().as_str(), Value::String(receipt.result_identity.clone())),
        ("result_status".to_string().as_str(), Value::String(receipt.result_status.clone())),
    ]);
    blake3_hex(&canonical_json_bytes(&payload))
}

fn assumptions_identity(assumptions: &Assumptions) -> String {
    let payload = canonical_object(vec![
        ("trusted_boundaries".to_string().as_str(), json_array(sorted_strings(&assumptions.trusted_boundaries))),
        ("external_bodies".to_string().as_str(), json_array(sorted_strings(&assumptions.external_bodies))),
        ("proof_gaps".to_string().as_str(), json_array(sorted_strings(&assumptions.proof_gaps))),
        ("dependencies".to_string().as_str(), json_array(sorted_strings(&assumptions.dependencies))),
    ]);
    blake3_hex(&canonical_json_bytes(&payload))
}

fn json_array(values: Vec<String>) -> Value {
    Value::Array(values.into_iter().map(Value::String).collect())
}

/// Proof IR identity binds the local property surface of one record. It is a
/// scoped local fact, not a model-equivalence or soundness claim.
fn proof_ir_identity(record: &IrRecordSeed) -> String {
    let payload = canonical_object(vec![
        ("scope".to_string().as_str(), Value::String(record.scope.clone())),
        ("requirement_ids".to_string().as_str(), json_array(sorted_strings(&record.requirement_ids))),
        ("property_ids".to_string().as_str(), json_array(sorted_strings(&record.property_ids))),
    ]);
    blake3_hex(&canonical_json_bytes(&payload))
}

/// Chain identity deterministically binds source identities, proof IR
/// identity, verifier receipt, and assumptions. Changing any assumption
/// changes the chain identity.
fn chain_identity(
    source_identities: &[ContentIdentity],
    ir_identity: &str,
    receipt: &VerifierReceipt,
    assumptions: &Assumptions,
) -> String {
    let payload = canonical_object(vec![
        ("source_identity".to_string().as_str(), Value::String(source_identity(source_identities))),
        ("proof_ir_identity".to_string().as_str(), Value::String(ir_identity.to_string())),
        ("verifier_receipt_identity".to_string().as_str(), Value::String(receipt_identity(receipt))),
        ("assumption_identity".to_string().as_str(), Value::String(assumptions_identity(assumptions))),
    ]);
    blake3_hex(&canonical_json_bytes(&payload))
}

/// Kamacite-ready handoff carries canonical identity fields only. Kamacite can
/// canonicalize it without parsing Verus source.
// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.handoff]
fn kamacite_handoff(
    requirement_ids: &[String],
    source_identities: &[ContentIdentity],
    ir_identity: &str,
    receipt: &VerifierReceipt,
    assumptions: &Assumptions,
    non_claims: &[String],
) -> KamaciteHandoff {
    KamaciteHandoff {
        schema: HANDOFF_SCHEMA_VERSION.to_string(),
        source_identity: source_identity(source_identities),
        proof_ir_identity: ir_identity.to_string(),
        verifier_receipt_identity: receipt_identity(receipt),
        assumption_identity: assumptions_identity(assumptions),
        requirement_ids: sorted_strings(requirement_ids),
        non_claims: sorted_strings(non_claims),
    }
}

struct IrRecordSeed {
    scope: String,
    requirement_ids: Vec<String>,
    property_ids: Vec<String>,
}

struct ProofLinkage {
    scope: String,
    requirement_ids: Vec<String>,
    property_ids: Vec<String>,
}

fn proof_ir_identity_from(linkage: &ProofLinkage) -> String {
    let seed = IrRecordSeed {
        scope: linkage.scope.clone(),
        requirement_ids: linkage.requirement_ids.clone(),
        property_ids: linkage.property_ids.clone(),
    };
    proof_ir_identity(&seed)
}

/// Validates one IR record fail-closed. Returns deterministic diagnostics.
// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.verifier_receipt]
// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.assumptions]
// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.negative]
fn validate_record(record: &IrRecord, expected_requirements: &BTreeSet<String>, errors: &mut Vec<String>) {
    if record.scope.is_empty() {
        errors.push("record scope must be non-empty".to_string());
    }
    if record.requirement_ids.is_empty() {
        errors.push(format!("{} missing requirement_ids", record.scope));
    }
    if record.property_ids.is_empty() {
        errors.push(format!("{} missing property_ids", record.scope));
    }
    if record.source_identities.is_empty() {
        errors.push(format!("{} missing source identities", record.scope));
    }
    if record.proof_ir_identity.is_empty() {
        errors.push(format!("{} missing proof IR identity", record.scope));
    }
    if record.claims.is_empty() {
        errors.push(format!("{} missing claims", record.scope));
    }
    if record.non_claims.is_empty() {
        errors.push(format!("{} missing non-claims", record.scope));
    }
    validate_requirement_links(record, expected_requirements, errors);
    validate_verifier_receipt(record, errors);
    validate_assumptions(record, errors);
    validate_claims_boundary(record, errors);
    validate_chain_identity(record, errors);
    validate_handoff(record, errors);
}

fn validate_requirement_links(record: &IrRecord, expected_requirements: &BTreeSet<String>, errors: &mut Vec<String>) {
    for requirement in &record.requirement_ids {
        if !expected_requirements.contains(requirement) {
            errors.push(format!("{} names unknown requirement `{requirement}`", record.scope));
        }
    }
}

// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.verifier_receipt]
fn validate_verifier_receipt(record: &IrRecord, errors: &mut Vec<String>) {
    let receipt = &record.verifier_receipt;
    if receipt.verifier_identity.is_empty() {
        errors.push(format!("{} missing verifier identity", record.scope));
    }
    if receipt.toolchain_identity.is_empty() {
        errors.push(format!("{} missing toolchain identity", record.scope));
    }
    if receipt.solver_identity.is_empty() {
        errors.push(format!("{} missing solver identity", record.scope));
    }
    if receipt.command_identity.is_empty() {
        errors.push(format!("{} missing command identity", record.scope));
    }
    if receipt.input_roots.is_empty() {
        errors.push(format!("{} missing input roots", record.scope));
    }
    if receipt.result_identity.is_empty() {
        errors.push(format!("{} missing result identity", record.scope));
    }
    if receipt.result_status != RESULT_PASSED && receipt.result_status != RESULT_FAILED {
        errors.push(format!("{} unsupported result status `{}`", record.scope, receipt.result_status));
    }
}

// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.assumptions]
fn validate_assumptions(record: &IrRecord, errors: &mut Vec<String>) {
    // A typed empty assumption set is a valid representation for verified
    // modules without trusted boundaries or gaps. Fields stay typed either way.
    let duplicate_detector: BTreeSet<&String> = record
        .assumptions
        .trusted_boundaries
        .iter()
        .chain(record.assumptions.external_bodies.iter())
        .chain(record.assumptions.proof_gaps.iter())
        .chain(record.assumptions.dependencies.iter())
        .collect();
    let total = record.assumptions.trusted_boundaries.len()
        + record.assumptions.external_bodies.len()
        + record.assumptions.proof_gaps.len()
        + record.assumptions.dependencies.len();
    if duplicate_detector.len() != total {
        errors.push(format!("{} assumptions overlap across typed fields", record.scope));
    }
}

fn validate_claims_boundary(record: &IrRecord, errors: &mut Vec<String>) {
    for claim in &record.claims {
        let lower = claim.to_ascii_lowercase();
        for forbidden in FORBIDDEN_CLAIM_FRAGMENTS {
            if lower.contains(forbidden) {
                errors.push(format!("{} contains overbroad claim `{claim}`", record.scope));
            }
        }
    }
    let non_claims: BTreeSet<String> = record.non_claims.iter().map(|claim| claim.to_ascii_lowercase()).collect();
    for required in REQUIRED_NON_CLAIMS {
        if !non_claims.iter().any(|claim| claim.contains(required)) {
            errors.push(format!("{} missing explicit non-claim `{required}`", record.scope));
        }
    }
    if record.recorded_only {
        for claim in &record.claims {
            let lower = claim.to_ascii_lowercase();
            if lower.contains("promoted") || lower.contains("verified in this context") {
                errors.push(format!("{} recorded-only record claims promotion", record.scope));
            }
        }
    }
}

// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.assumptions]
fn validate_chain_identity(record: &IrRecord, errors: &mut Vec<String>) {
    let expected = chain_identity(
        &record.source_identities,
        &record.proof_ir_identity,
        &record.verifier_receipt,
        &record.assumptions,
    );
    if expected != record.chain_identity {
        errors.push(format!(
            "{} chain identity is stale; assumption or receipt drift changes evidence identity",
            record.scope
        ));
    }
}

// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.handoff]
fn validate_handoff(record: &IrRecord, errors: &mut Vec<String>) {
    let handoff = &record.handoff;
    if handoff.schema != HANDOFF_SCHEMA_VERSION {
        errors.push(format!("{} unsupported handoff schema `{}`", record.scope, handoff.schema));
        return;
    }
    if handoff.source_identity != source_identity(&record.source_identities) {
        errors.push(format!("{} handoff source identity is inconsistent", record.scope));
    }
    if handoff.proof_ir_identity != record.proof_ir_identity {
        errors.push(format!("{} handoff proof IR identity is inconsistent", record.scope));
    }
    if handoff.verifier_receipt_identity != receipt_identity(&record.verifier_receipt) {
        errors.push(format!("{} handoff verifier receipt identity is inconsistent", record.scope));
    }
    if handoff.assumption_identity != assumptions_identity(&record.assumptions) {
        errors.push(format!("{} handoff assumption identity is inconsistent", record.scope));
    }
}

// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.contract]
fn validate_ir_chain(chain: &IrChain, expected_requirements: &BTreeSet<String>) -> Vec<String> {
    let mut errors = Vec::new();
    if chain.schema_version != SCHEMA_VERSION {
        errors.push(format!(
            "unsupported schema `{}`; expected `{SCHEMA_VERSION}`",
            chain.schema_version
        ));
        return errors;
    }
    if chain.generated_by.is_empty() {
        errors.push("generated_by must be non-empty".to_string());
    }
    if chain.source_bundle_path.is_empty() {
        errors.push("source_bundle_path must be non-empty".to_string());
    }
    if chain.records.is_empty() {
        errors.push("IR chain must contain at least one record".to_string());
    }
    for record in &chain.records {
        validate_record(record, expected_requirements, &mut errors);
    }
    errors
}

/// Derives typed assumptions from the proof-gap manifest rows for the given
/// source paths. Trusted permanent boundaries become trusted_boundaries;
/// other classified gaps stay as proof_gaps.
fn assumptions_from_gaps(
    gap_references: &[String],
    gap_classifications: &[(String, String)],
) -> Assumptions {
    let mut trusted_boundaries = Vec::new();
    let mut proof_gaps = Vec::new();
    for reference in gap_references {
        let classification = gap_classifications
            .iter()
            .find(|(path, _)| reference.starts_with(path))
            .map(|(_, classification)| classification.as_str());
        match classification {
            Some(TRUSTED_BOUNDARY_CLASSIFICATION) => trusted_boundaries.push(reference.clone()),
            _ => proof_gaps.push(reference.clone()),
        }
    }
    trusted_boundaries.sort();
    trusted_boundaries.dedup();
    proof_gaps.sort();
    proof_gaps.dedup();
    Assumptions {
        trusted_boundaries,
        external_bodies: Vec::new(),
        proof_gaps,
        dependencies: Vec::new(),
    }
}

fn extract_requirement_ids(requirements: &str) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for line in requirements.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with(REQUIREMENT_PREFIX) {
            continue;
        }
        let Some(end) = trimmed.find(REQUIREMENT_SUFFIX) else {
            continue;
        };
        let id = &trimmed[REQUIREMENT_PREFIX.len()..end];
        if is_requirement_id(id) {
            ids.insert(id.to_string());
        }
    }
    ids
}

fn is_requirement_id(id: &str) -> bool {
    !id.is_empty()
        && !id.contains(char::is_whitespace)
        && id.chars().all(|ch| ch == '-' || ch == '.' || ch == '_' || ch.is_ascii_alphanumeric())
}

fn parse_gap_classifications(tsv: &str) -> Vec<(String, String)> {
    let mut rows = Vec::new();
    for line in tsv.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = trimmed.split(PROOF_GAP_SEPARATOR).collect();
        let Some(path) = fields.first().copied() else {
            continue;
        };
        let Some(signature) = fields.get(1).copied() else {
            continue;
        };
        let Some(classification) = fields.get(2).copied() else {
            continue;
        };
        rows.push((format!("{path}::{signature}"), classification.to_string()));
    }
    rows
}

fn command_identity(proof_command: &str) -> String {
    let payload = canonical_object(vec![
        ("proof_command".to_string().as_str(), Value::String(proof_command.to_string())),
    ]);
    blake3_hex(&canonical_json_bytes(&payload))
}

fn result_identity(proof_outcome: &str, tracey_coverage: &str) -> String {
    let payload = canonical_object(vec![
        ("proof_outcome".to_string().as_str(), Value::String(proof_outcome.to_string())),
        ("tracey_coverage".to_string().as_str(), Value::String(tracey_coverage.to_string())),
    ]);
    blake3_hex(&canonical_json_bytes(&payload))
}

fn input_roots_for(record: &BundleRecord) -> Vec<String> {
    let mut roots = record.source_paths.clone();
    roots.push(REQUIREMENTS_PATH.to_string());
    if !record.proof_gap_references.is_empty() {
        roots.push(PROOF_GAP_PATH.to_string());
    }
    roots.sort();
    roots.dedup();
    roots
}

/// Records one bundle record as a linked IR record. Structural identity only;
/// no soundness claim is made.
fn record_for_bundle(record: &BundleRecord, gap_classifications: &[(String, String)]) -> IrRecord {
    let linkage = ProofLinkage {
        scope: record.scope.clone(),
        requirement_ids: record.requirement_ids.clone(),
        property_ids: record.requirement_ids.clone(),
    };
    let ir_identity = proof_ir_identity_from(&linkage);
    let assumptions = assumptions_from_gaps(&record.proof_gap_references, gap_classifications);
    let result_status = if record.proof_outcome == STATUS_VERIFIED && record.tracey_coverage == COVERAGE_COVERED {
        RESULT_PASSED.to_string()
    } else {
        RESULT_FAILED.to_string()
    };
    let receipt = VerifierReceipt {
        verifier_identity: "verus".to_string(),
        toolchain_identity: record.verifier_toolchain.clone(),
        solver_identity: DECLARED_SOLVER_IDENTITY.to_string(),
        command_identity: command_identity(&record.proof_command),
        input_roots: input_roots_for(record),
        result_identity: result_identity(&record.proof_outcome, &record.tracey_coverage),
        result_status: result_status.clone(),
    };
    let chain_identity = chain_identity(&record.content_identities, &ir_identity, &receipt, &assumptions);
    let mut non_claims: Vec<String> = REQUIRED_NON_CLAIMS.iter().map(|claim| (*claim).to_string()).collect();
    non_claims.extend(record.non_claims.iter().cloned());
    non_claims.sort();
    non_claims.dedup();
    let handoff = kamacite_handoff(
        &record.requirement_ids,
        &record.content_identities,
        &ir_identity,
        &receipt,
        &assumptions,
        &non_claims,
    );
    let recorded_only = result_status != RESULT_PASSED;
    IrRecord {
        scope: record.scope.clone(),
        requirement_ids: record.requirement_ids.clone(),
        property_ids: record.requirement_ids.clone(),
        source_identities: record.content_identities.clone(),
        proof_ir_identity: ir_identity,
        verifier_receipt: receipt,
        assumptions,
        status: result_status,
        recorded_only,
        claims: record.claims.clone(),
        non_claims,
        chain_identity,
        handoff,
    }
}

// r[impl trellis.proof_evidence_bundles.proof_evidence_ir_chain.contract]
fn generate_ir_chain(bundle: &ProofBundle, gap_classifications: &[(String, String)]) -> IrChain {
    IrChain {
        schema_version: SCHEMA_VERSION.to_string(),
        generated_by: GENERATOR_PATH.to_string(),
        source_bundle_path: BUNDLE_PATH.to_string(),
        records: bundle
            .records
            .iter()
            .map(|record| record_for_bundle(record, gap_classifications))
            .collect(),
    }
}

fn sorted_json(chain: &IrChain) -> Result<String, String> {
    serde_json::to_string_pretty(chain)
        .map(|mut json| {
            json.push('\n');
            json
        })
        .map_err(|error| error.to_string())
}

fn read_text(path: &Path, display: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("failed to read {display}: {error}"))
}

fn load_repo_facts(root: &Path) -> Result<RepoFacts, String> {
    let requirements = extract_requirement_ids(&read_text(&root.join(REQUIREMENTS_PATH), REQUIREMENTS_PATH)?);
    let gap_classifications = parse_gap_classifications(&read_text(&root.join(PROOF_GAP_PATH), PROOF_GAP_PATH)?);
    Ok(RepoFacts {
        requirements,
        gap_classifications,
    })
}

fn load_bundle(root: &Path) -> Result<ProofBundle, String> {
    let text = read_text(&root.join(BUNDLE_PATH), BUNDLE_PATH)?;
    serde_json::from_str(&text).map_err(|error| format!("invalid bundle JSON: {error}"))
}

fn run_write(root: &Path) -> Result<(), String> {
    let bundle = load_bundle(root)?;
    let facts = load_repo_facts(root)?;
    let chain = generate_ir_chain(&bundle, &facts.gap_classifications);
    let errors = validate_ir_chain(&chain, &facts.requirements);
    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }
    fs::write(root.join(IR_CHAIN_PATH), sorted_json(&chain)?)
        .map_err(|error| format!("failed to write {IR_CHAIN_PATH}: {error}"))?;
    println!("✓ wrote {IR_CHAIN_PATH}");
    Ok(())
}

fn run_check(root: &Path) -> Result<(), String> {
    let bundle = load_bundle(root)?;
    let facts = load_repo_facts(root)?;
    let expected = generate_ir_chain(&bundle, &facts.gap_classifications);
    let expected_text = sorted_json(&expected)?;
    let actual_text = read_text(&root.join(IR_CHAIN_PATH), IR_CHAIN_PATH)?;
    if actual_text != expected_text {
        return Err(format!(
            "{IR_CHAIN_PATH} is stale; run `cargo run --quiet --bin proof_evidence_ir -- write`"
        ));
    }
    let chain: IrChain =
        serde_json::from_str(&actual_text).map_err(|error| format!("invalid IR chain JSON: {error}"))?;
    let errors = validate_ir_chain(&chain, &facts.requirements);
    if errors.is_empty() {
        println!("✓ proof evidence IR chain validates {} records", chain.records.len());
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}

fn positive_fixture_requirements() -> BTreeSet<String> {
    [
        "trellis.proof_evidence_bundles.proof_evidence_ir_chain.contract",
        "trellis.proof_evidence_bundles.proof_evidence_ir_chain.verifier_receipt",
        "trellis.proof_evidence_bundles.proof_evidence_ir_chain.assumptions",
        "trellis.proof_evidence_bundles.proof_evidence_ir_chain.handoff",
        "trellis.proof_evidence_bundles.proof_evidence_ir_chain.negative",
        "trellis.proof_evidence_bundles.proof_evidence_ir_chain.positive",
        "trellis.proof_evidence_bundles.proof_evidence_ir_chain.evidence",
        "trellis.proof_evidence_bundles.proof_evidence_ir_chain.docs",
        "trellis.proof_evidence_bundles.proof_evidence_ir_chain.validation",
    ]
    .iter()
    .map(|id| (*id).to_string())
    .collect()
}

fn fixture_source_identities() -> Vec<ContentIdentity> {
    vec![ContentIdentity {
        path: "src/fixture.rs".to_string(),
        algorithm: BLAKE3_ALGORITHM.to_string(),
        digest: blake3_hex(b"// r[impl fixture.req]\\n// r[verify fixture.req]\\npub fn fixture() {}\\n"),
    }]
}

fn fixture_receipt() -> VerifierReceipt {
    VerifierReceipt {
        verifier_identity: "verus".to_string(),
        toolchain_identity: "pinned Verus toolchain fixture".to_string(),
        solver_identity: DECLARED_SOLVER_IDENTITY.to_string(),
        command_identity: command_identity("./scripts/verus.sh src/fixture.rs"),
        input_roots: vec!["src/fixture.rs".to_string(), "docs/requirements.md".to_string()],
        result_identity: result_identity(STATUS_VERIFIED, COVERAGE_COVERED),
        result_status: RESULT_PASSED.to_string(),
    }
}

fn fixture_assumptions() -> Assumptions {
    Assumptions {
        trusted_boundaries: vec!["src/fixture.rs::pub fn trusted(".to_string()],
        external_bodies: Vec::new(),
        proof_gaps: Vec::new(),
        dependencies: Vec::new(),
    }
}

fn fixture_empty_assumptions() -> Assumptions {
    Assumptions {
        trusted_boundaries: Vec::new(),
        external_bodies: Vec::new(),
        proof_gaps: Vec::new(),
        dependencies: Vec::new(),
    }
}

/// Complete valid fixture IR chain used as the positive case and mutation base.
fn positive_fixture_chain() -> IrChain {
    let requirements = vec![
        "trellis.proof_evidence_bundles.proof_evidence_ir_chain.contract".to_string(),
        "trellis.proof_evidence_bundles.proof_evidence_ir_chain.handoff".to_string(),
    ];
    let linkage = ProofLinkage {
        scope: "module:fixture".to_string(),
        requirement_ids: requirements.clone(),
        property_ids: requirements.clone(),
    };
    let ir_identity = proof_ir_identity_from(&linkage);
    let receipt = fixture_receipt();
    let assumptions = fixture_assumptions();
    let source_identities = fixture_source_identities();
    let chain_id = chain_identity(&source_identities, &ir_identity, &receipt, &assumptions);
    let non_claims: Vec<String> = REQUIRED_NON_CLAIMS.iter().map(|claim| (*claim).to_string()).collect();
    let handoff = kamacite_handoff(
        &requirements,
        &source_identities,
        &ir_identity,
        &receipt,
        &assumptions,
        &non_claims,
    );
    IrChain {
        schema_version: SCHEMA_VERSION.to_string(),
        generated_by: GENERATOR_PATH.to_string(),
        source_bundle_path: BUNDLE_PATH.to_string(),
        records: vec![IrRecord {
            scope: "module:fixture".to_string(),
            requirement_ids: requirements.clone(),
            property_ids: requirements.clone(),
            source_identities,
            proof_ir_identity: ir_identity,
            verifier_receipt: receipt,
            assumptions,
            status: RESULT_PASSED.to_string(),
            recorded_only: false,
            claims: vec!["records local Trellis proof facts for module:fixture".to_string()],
            non_claims,
            chain_identity: chain_id,
            handoff,
        }],
    }
}

/// Recomputes chain identity and handoff for a record whose typed fields were
/// changed. Keeps proof IR identity stable across assumption drift so review
/// can see that drift moves chain and handoff identities, never the claimed IR.
fn rebuild_record_identities(record: &IrRecord) -> IrRecord {
    let mut rebuilt = record.clone();
    rebuilt.chain_identity = chain_identity(
        &rebuilt.source_identities,
        &rebuilt.proof_ir_identity,
        &rebuilt.verifier_receipt,
        &rebuilt.assumptions,
    );
    rebuilt.handoff = kamacite_handoff(
        &rebuilt.requirement_ids,
        &rebuilt.source_identities,
        &rebuilt.proof_ir_identity,
        &rebuilt.verifier_receipt,
        &rebuilt.assumptions,
        &rebuilt.non_claims,
    );
    rebuilt
}

// r[verify trellis.proof_evidence_bundles.proof_evidence_ir_chain.contract]
// r[verify trellis.proof_evidence_bundles.proof_evidence_ir_chain.verifier_receipt]
// r[verify trellis.proof_evidence_bundles.proof_evidence_ir_chain.assumptions]
// r[verify trellis.proof_evidence_bundles.proof_evidence_ir_chain.handoff]
// r[verify trellis.proof_evidence_bundles.proof_evidence_ir_chain.negative]
// r[verify trellis.proof_evidence_bundles.proof_evidence_ir_chain.positive]
// r[verify trellis.proof_evidence_bundles.proof_evidence_ir_chain.evidence]
// r[verify trellis.proof_evidence_bundles.proof_evidence_ir_chain.docs]
// r[verify trellis.proof_evidence_bundles.proof_evidence_ir_chain.validation]
fn run_self_test() -> Result<(), String> {
    let requirements = positive_fixture_requirements();
    let valid = positive_fixture_chain();
    expect_valid(&valid, &requirements, "positive fixture")?;

    let mut missing_verifier = valid.clone();
    missing_verifier.records[0].verifier_receipt.verifier_identity.clear();
    expect_invalid(&missing_verifier, &requirements, "missing verifier identity", "missing verifier identity")?;

    let mut unsupported_status = valid.clone();
    unsupported_status.records[0].verifier_receipt.result_status = "unknown".to_string();
    expect_invalid(&unsupported_status, &requirements, "unsupported result status", "unsupported result status")?;

    let mut missing_input_roots = valid.clone();
    missing_input_roots.records[0].verifier_receipt.input_roots.clear();
    expect_invalid(&missing_input_roots, &requirements, "missing input roots", "missing input roots")?;

    let mut overbroad = valid.clone();
    overbroad.records[0].claims = vec!["release approval for whole-program correctness".to_string()];
    expect_invalid(&overbroad, &requirements, "overbroad claim", "overbroad claim")?;

    let mut stale_chain = valid.clone();
    stale_chain.records[0].chain_identity = "stale".to_string();
    expect_invalid(&stale_chain, &requirements, "chain identity is stale", "stale chain identity")?;

    let mut stale_handoff = valid.clone();
    stale_handoff.records[0].handoff.verifier_receipt_identity = "stale".to_string();
    expect_invalid(&stale_handoff, &requirements, "handoff verifier receipt identity is inconsistent", "stale handoff")?;

    let mut empty_input_roots = valid.clone();
    empty_input_roots.records[0].verifier_receipt.input_roots = vec![];
    expect_invalid(&empty_input_roots, &requirements, "missing input roots", "empty input roots")?;

    let mut missing_non_claims = valid.clone();
    missing_non_claims.records[0].non_claims.clear();
    expect_invalid(&missing_non_claims, &requirements, "missing explicit non-claim", "missing non-claims")?;

    // The empty typed assumption set is valid for gap-free verified modules.
    let mut empty_assumptions = valid.clone();
    empty_assumptions.records[0].assumptions.trusted_boundaries.clear();
    empty_assumptions.records[0].assumptions = fixture_empty_assumptions();
    empty_assumptions.records[0].chain_identity = chain_identity(
        &empty_assumptions.records[0].source_identities,
        &empty_assumptions.records[0].proof_ir_identity,
        &empty_assumptions.records[0].verifier_receipt,
        &empty_assumptions.records[0].assumptions,
    );
    empty_assumptions.records[0].handoff.assumption_identity =
        assumptions_identity(&empty_assumptions.records[0].assumptions);
    expect_valid(&empty_assumptions, &requirements, "empty typed assumptions fixture")?;

    // Assumption drift must change chain identity (typed assumptions). A stale
    // chain after drift is rejected; recomputed chain and handoff are accepted.
    let mut drift = valid.clone();
    drift.records[0].assumptions.trusted_boundaries.push("src/fixture.rs::pub fn drifted(".to_string());
    expect_invalid(&drift, &requirements, "chain identity is stale", "assumption drift with stale chain")?;
    let drifted = rebuild_record_identities(&drift.records[0].clone());
    let mut drift_chain = drift.clone();
    drift_chain.records[0] = drifted;
    if drift_chain.records[0].chain_identity == valid.records[0].chain_identity {
        return Err("assumption drift did not change chain identity".to_string());
    }
    expect_valid(&drift_chain, &requirements, "assumption-drift fixture recomputed chain and handoff")?;

    // A handoff whose source identity disagrees with the record must fail.
    let mut mishandled = valid.clone();
    mishandled.records[0].handoff.source_identity = "different".to_string();
    expect_invalid(&mishandled, &requirements, "handoff source identity is inconsistent", "mismatched handoff source")?;

    // Unsupported schema must fail before semantic validation.
    let mut unsupported_schema = valid.clone();
    unsupported_schema.schema_version = "future-schema".to_string();
    expect_invalid(&unsupported_schema, &requirements, "unsupported schema", "unsupported schema")?;

    println!("✓ proof evidence IR chain self-test fixtures passed");
    Ok(())
}

fn expect_valid(chain: &IrChain, requirements: &BTreeSet<String>, name: &str) -> Result<(), String> {
    let errors = validate_ir_chain(chain, requirements);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(format!("{name} failed: {}", errors.join("; ")))
    }
}

fn expect_invalid(chain: &IrChain, requirements: &BTreeSet<String>, expected: &str, name: &str) -> Result<(), String> {
    let errors = validate_ir_chain(chain, requirements);
    if errors.iter().any(|error| error.contains(expected)) {
        Ok(())
    } else {
        Err(format!("{name} missed `{expected}`: {}", errors.join("; ")))
    }
}

fn repo_root() -> PathBuf {
    if let Ok(output) = std::process::Command::new("git").args(["rev-parse", "--show-toplevel"]).output() {
        if output.status.success() {
            return PathBuf::from(String::from_utf8_lossy(&output.stdout).trim());
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let root = repo_root();
    let result = match args.as_slice() {
        [] => run_check(&root),
        [command] if command == "check" => run_check(&root),
        [command] if command == "write" => run_write(&root),
        [command] if command == "self-test" => run_self_test(),
        _ => Err("usage: proof_evidence_ir [check|write|self-test]".to_string()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("✗ {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // r[verify trellis.proof_evidence_bundles.proof_evidence_ir_chain.positive]
    #[test]
    fn positive_fixture_validates() {
        let requirements = positive_fixture_requirements();
        let chain = positive_fixture_chain();
        assert!(validate_ir_chain(&chain, &requirements).is_empty());
    }

    // r[verify trellis.proof_evidence_bundles.proof_evidence_ir_chain.verifier_receipt]
    #[test]
    fn missing_verifier_identity_is_rejected() {
        let requirements = positive_fixture_requirements();
        let mut chain = positive_fixture_chain();
        chain.records[0].verifier_receipt.verifier_identity.clear();
        let errors = validate_ir_chain(&chain, &requirements);
        assert!(errors.iter().any(|error| error.contains("missing verifier identity")));
    }

    // r[verify trellis.proof_evidence_bundles.proof_evidence_ir_chain.assumptions]
    #[test]
    fn assumption_drift_changes_chain_identity() {
        let base = positive_fixture_chain();
        let base_identity = base.records[0].chain_identity.clone();
        let mut mutated = base.clone();
        mutated.records[0].assumptions.trusted_boundaries.push("drifted".to_string());
        let new_identity = chain_identity(
            &mutated.records[0].source_identities,
            &mutated.records[0].proof_ir_identity,
            &mutated.records[0].verifier_receipt,
            &mutated.records[0].assumptions,
        );
        assert_ne!(base_identity, new_identity);
    }

    // r[verify trellis.proof_evidence_bundles.proof_evidence_ir_chain.negative]
    #[test]
    fn overbroad_claim_is_rejected() {
        let requirements = positive_fixture_requirements();
        let mut chain = positive_fixture_chain();
        chain.records[0].claims = vec!["release approval for production safety".to_string()];
        let errors = validate_ir_chain(&chain, &requirements);
        assert!(errors.iter().any(|error| error.contains("overbroad claim")));
    }
}
