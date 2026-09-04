use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Profile {
    pub(crate) schema_version: String,
    pub(crate) pilot_id: String,
    pub(crate) outcome: String,
    pub(crate) octet_source: OctetSource,
    pub(crate) consumer: ConsumerProfile,
    pub(crate) provider_tools: ProviderTools,
    pub(crate) extraction: ExtractionProfile,
    pub(crate) resource_policy: ArtifactReference,
    pub(crate) artifacts: Vec<ArtifactIdentity>,
    pub(crate) theorem_mappings: Vec<TheoremMapping>,
    pub(crate) trust_closure: TrustClosure,
    pub(crate) verus: VerusBinding,
    pub(crate) negative_fixture_catalog: String,
    pub(crate) required_non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct OctetSource {
    pub(crate) repository: String,
    pub(crate) revision: String,
    pub(crate) consumer_contract_path: String,
    pub(crate) consumer_contract_identity: String,
    pub(crate) policy_artifact_identity: String,
    pub(crate) provider_profile_identity: String,
    pub(crate) compatibility_matrix_identity: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ConsumerProfile {
    pub(crate) schema_version: String,
    pub(crate) profile_id: String,
    pub(crate) consumer_id: String,
    pub(crate) provider: ConsumerProvider,
    pub(crate) source: ConsumerSource,
    pub(crate) theorems: Vec<ConsumerTheorem>,
    pub(crate) external_models: Vec<TrustInput>,
    pub(crate) trust_inputs: Vec<TrustInput>,
    pub(crate) resource_policy: ConsumerResourcePolicy,
    pub(crate) outcomes: Vec<String>,
    pub(crate) trust_closure_complete: bool,
    pub(crate) non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ConsumerProvider {
    pub(crate) backend: String,
    pub(crate) profile_id: String,
    pub(crate) profile_identity: String,
    pub(crate) compatibility_matrix_id: String,
    pub(crate) compatibility_matrix_identity: String,
    pub(crate) network_allowed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ConsumerSource {
    pub(crate) root: String,
    pub(crate) crate_name: String,
    pub(crate) paths: Vec<SourceFileBinding>,
    pub(crate) selected_items: Vec<SelectedItem>,
    pub(crate) broad_selection_allowed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SourceFileBinding {
    pub(crate) path: String,
    pub(crate) identity: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SelectedItem {
    pub(crate) qualified_name: String,
    pub(crate) kind: String,
    pub(crate) source_file: String,
    pub(crate) source_identity: String,
    pub(crate) begin_line: u64,
    pub(crate) end_line: u64,
    pub(crate) generated_name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ConsumerTheorem {
    pub(crate) path: String,
    pub(crate) theorem_id: String,
    pub(crate) declaration_name: String,
    pub(crate) requirement_id: String,
    pub(crate) claim_class: String,
    pub(crate) approved_statement_blake3: String,
    pub(crate) source_disposition: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TrustInput {
    pub(crate) input_id: String,
    pub(crate) identity: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ConsumerResourcePolicy {
    pub(crate) timeout_ms: u64,
    pub(crate) stdout_max_bytes: u64,
    pub(crate) stderr_max_bytes: u64,
    pub(crate) environment_allowlist: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ProviderTools {
    pub(crate) package: String,
    pub(crate) identity_schema: String,
    pub(crate) aeneas_revision: String,
    pub(crate) charon_revision: String,
    pub(crate) lean_toolchain: String,
    pub(crate) aeneas_binary_blake3: String,
    pub(crate) charon_binary_blake3: String,
    pub(crate) lean_binary_blake3: String,
    pub(crate) lake_binary_blake3: String,
    pub(crate) leantar_binary_blake3: String,
    pub(crate) network_sandbox_blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ExtractionProfile {
    pub(crate) charon_argv: Vec<String>,
    pub(crate) cargo_argv: Vec<String>,
    pub(crate) aeneas_argv: Vec<String>,
    pub(crate) lean_argv: Vec<String>,
    pub(crate) expected_has_errors: bool,
    pub(crate) expected_skip_borrowck: bool,
    pub(crate) expected_target: String,
    pub(crate) expected_pointer_width_bytes: u64,
    pub(crate) expected_little_endian: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ArtifactReference {
    pub(crate) path: String,
    pub(crate) identity: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ArtifactIdentity {
    pub(crate) role: String,
    pub(crate) path: String,
    pub(crate) bytes: u64,
    pub(crate) blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TheoremMapping {
    pub(crate) theorem_id: String,
    pub(crate) path: String,
    pub(crate) declaration_name: String,
    pub(crate) requirement_id: String,
    pub(crate) claim_class: String,
    pub(crate) approved_statement_blake3: String,
    pub(crate) verus_proof_symbol: String,
    pub(crate) lean_theorem_symbol: String,
    pub(crate) integer_model: String,
    pub(crate) overflow_model: String,
    pub(crate) axioms: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TrustClosure {
    pub(crate) complete: bool,
    pub(crate) selected_axioms: Vec<String>,
    pub(crate) selected_opaque_definitions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct VerusBinding {
    pub(crate) receipt_path: String,
    pub(crate) requirement_id: String,
    pub(crate) proof_symbols: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ResourcePolicy {
    pub(crate) schema_version: String,
    pub(crate) policy_id: String,
    pub(crate) timeout_ms: u64,
    pub(crate) stdout_max_bytes: u64,
    pub(crate) stderr_max_bytes: u64,
    pub(crate) artifact_max_bytes: u64,
    pub(crate) environment_allowlist: Vec<String>,
    pub(crate) network_allowed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NegativeCatalog {
    pub(crate) schema_version: String,
    pub(crate) cases: Vec<NegativeCase>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NegativeCase {
    pub(crate) id: String,
    pub(crate) expected_diagnostic: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FinalOutcome {
    Supported,
    Blocked,
    Failed,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum StageStatus {
    Passed,
    Unsupported,
    ProcessFailed,
    PolicyFailed,
    ArtifactFailed,
    TimedOut,
    BudgetExceeded,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct StageObservation {
    pub(crate) stage: String,
    pub(crate) status: StageStatus,
    pub(crate) construct: Option<String>,
    pub(crate) diagnostic_identity: Option<String>,
    pub(crate) next_action: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct StageReceipt {
    pub(crate) stage: String,
    pub(crate) status: StageStatus,
    pub(crate) argv: Vec<String>,
    pub(crate) artifact_roles: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ProofReceipt {
    pub(crate) schema_version: String,
    pub(crate) outcome: FinalOutcome,
    pub(crate) profile_id: String,
    pub(crate) profile_identity: String,
    pub(crate) source_path: String,
    pub(crate) source_identity: String,
    pub(crate) selected_item: SelectedItem,
    pub(crate) run_identity: String,
    pub(crate) stages: Vec<StageReceipt>,
    pub(crate) resource_policy_identity: String,
    pub(crate) theorem_mappings: Vec<TheoremMapping>,
    pub(crate) external_models: Vec<TrustInput>,
    pub(crate) trust_inputs: Vec<TrustInput>,
    pub(crate) selected_axioms: Vec<String>,
    pub(crate) artifacts: Vec<ArtifactIdentity>,
    pub(crate) artifact_verification: String,
    pub(crate) verus_receipt_path: String,
    pub(crate) verus_receipt_identity: String,
    pub(crate) octet_policy_artifact_identity: String,
    pub(crate) non_claims: Vec<String>,
    pub(crate) receipt_identity: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CrossVerifierManifest {
    pub(crate) schema_version: String,
    pub(crate) status: String,
    pub(crate) source_path: String,
    pub(crate) source_identity: String,
    pub(crate) run_identity: String,
    pub(crate) receipt_path: String,
    pub(crate) receipt_identity: String,
    pub(crate) properties: Vec<PropertyLink>,
    pub(crate) authority_merged: bool,
    pub(crate) artifact_verification: String,
    pub(crate) non_claims: Vec<String>,
    pub(crate) manifest_identity: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PropertyLink {
    pub(crate) theorem_id: String,
    pub(crate) requirement_id: String,
    pub(crate) verus_proof_symbol: String,
    pub(crate) lean_theorem_symbol: String,
    pub(crate) theorem_source_identity: String,
    pub(crate) integer_model: String,
    pub(crate) overflow_model: String,
    pub(crate) axioms: Vec<String>,
}
