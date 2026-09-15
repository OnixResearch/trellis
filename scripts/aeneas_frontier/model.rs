//! Typed model for the measured Aeneas/Lean coverage frontier and the
//! requirement statement registry.
//!
//! All types use `deny_unknown_fields` so a drifted producer fails closed
//! instead of silently dropping fields.

use serde::Deserialize;
use serde::Serialize;

pub(crate) const FRONTIER_SCHEMA: &str = "trellis-aeneas-frontier/v1";
pub(crate) const OBSERVATIONS_SCHEMA: &str = "trellis-aeneas-observations/v1";
pub(crate) const REGISTRY_SCHEMA: &str = "trellis-aeneas-registry/v1";
pub(crate) const TARGETS_SCHEMA: &str = "trellis-aeneas-targets/v1";

pub(crate) const CLASS_TIER_1: &str = "tier-1-primitive";
pub(crate) const CLASS_TIER_2: &str = "tier-2-collection";
pub(crate) const CLASS_TIER_3: &str = "tier-3-external";
pub(crate) const CLASS_TIER_4: &str = "tier-4-trust-boundary";
pub(crate) const CLASS_EXCLUDED: &str = "excluded-construct";
pub(crate) const CLASS_UNMEASURED: &str = "unmeasured";
pub(crate) const CLASS_BLOCKED_PREFIX: &str = "blocked-";

pub(crate) const OUTCOME_SUPPORTED: &str = "supported";
pub(crate) const OUTCOME_BLOCKED: &str = "blocked";
pub(crate) const OUTCOME_FAILED: &str = "failed";

pub(crate) const TIER_CLASSES: [&str; 4] = [CLASS_TIER_1, CLASS_TIER_2, CLASS_TIER_3, CLASS_TIER_4];

pub(crate) const MAX_MODULES_PER_BATCH: u64 = 8;
pub(crate) const MAX_THEOREMS_PER_MODULE: u64 = 4;
pub(crate) const MAX_ACTIVE_BATCHES: u64 = 1;

pub(crate) const RECEIPT_CLASS_TRELLIS: &str = "trellis-proof-receipt";
pub(crate) const RECEIPT_CLASS_OCTET: &str = "octet-proof-receipt";
pub(crate) const RECEIPT_CLASS_LOCAL: &str = "local-lake-build";
pub(crate) const RECEIPT_CLASSES: [&str; 3] = [RECEIPT_CLASS_TRELLIS, RECEIPT_CLASS_OCTET, RECEIPT_CLASS_LOCAL];

/// Closed diagnostic classes for a failed observation.
pub(crate) const DIAGNOSTIC_CLASS_TIMEOUT: &str = "stage-timeout";
pub(crate) const DIAGNOSTIC_CLASSES: [&str; 4] = [
    DIAGNOSTIC_CLASS_TIMEOUT,
    "process-spawn-failure",
    "provider-identity-drift",
    "artifact-read-failure",
];

pub(crate) const REQUIRED_NON_CLAIMS: [&str; 5] = [
    "does not prove whole-library Aeneas or Lean coverage",
    "does not prove Aeneas, Charon, or Lean soundness",
    "does not prove Verus and Aeneas semantic equivalence",
    "does not prove verifier independence",
    "does not establish release eligibility",
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ProviderBinding {
    pub(crate) package: String,
    pub(crate) revision: String,
    pub(crate) profile_identity: String,
    pub(crate) compatibility_matrix_identity: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Observation {
    pub(crate) module: String,
    pub(crate) stage: String,
    pub(crate) outcome: String,
    pub(crate) measured_class: String,
    #[serde(default)]
    pub(crate) reviewed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) construct: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) diagnostic_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) evidence: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ObservationsFile {
    pub(crate) schema_version: String,
    pub(crate) provider: ProviderBinding,
    pub(crate) observations: Vec<Observation>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Bounds {
    pub(crate) modules_per_batch: u64,
    pub(crate) theorems_per_module: u64,
    pub(crate) active_batches: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ModuleRow {
    pub(crate) module: String,
    pub(crate) hypothesis: String,
    pub(crate) measured_class: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) observation: Option<Observation>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Counts {
    pub(crate) total_modules: u64,
    pub(crate) measured: u64,
    pub(crate) blocked: u64,
    pub(crate) excluded: u64,
    pub(crate) unmeasured: u64,
    pub(crate) hypothesis_tier_1: u64,
    pub(crate) hypothesis_tier_2: u64,
    pub(crate) hypothesis_tier_3: u64,
    pub(crate) hypothesis_tier_4: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FrontierArtifact {
    pub(crate) schema_version: String,
    pub(crate) bounds: Bounds,
    pub(crate) provider: ProviderBinding,
    pub(crate) modules: Vec<ModuleRow>,
    pub(crate) counts: Counts,
    pub(crate) non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Target {
    pub(crate) requirement_id: String,
    pub(crate) module: String,
    pub(crate) statement_blake3: Vec<String>,
    pub(crate) claim_class: String,
    pub(crate) trust_closure: Vec<String>,
    pub(crate) receipt: String,
    pub(crate) receipt_class: String,
    pub(crate) receipt_evidence: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TargetsFile {
    pub(crate) schema_version: String,
    pub(crate) targets: Vec<Target>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RegistryRow {
    pub(crate) requirement_id: String,
    pub(crate) targeted: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) statement_blake3: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) claim_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) module: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) trust_closure: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) receipt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) receipt_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) receipt_evidence: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) non_target_reason: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RegistryCounts {
    pub(crate) total_requirements: u64,
    pub(crate) targeted: u64,
    pub(crate) non_target: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RegistryArtifact {
    pub(crate) schema_version: String,
    pub(crate) rows: Vec<RegistryRow>,
    pub(crate) counts: RegistryCounts,
    pub(crate) non_claims: Vec<String>,
}

pub(crate) fn bounds() -> Bounds {
    Bounds {
        modules_per_batch: MAX_MODULES_PER_BATCH,
        theorems_per_module: MAX_THEOREMS_PER_MODULE,
        active_batches: MAX_ACTIVE_BATCHES,
    }
}

pub(crate) fn non_claims() -> Vec<String> {
    REQUIRED_NON_CLAIMS.iter().map(|value| (*value).to_owned()).collect()
}
