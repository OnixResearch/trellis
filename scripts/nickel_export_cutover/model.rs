use serde::Deserialize;
use serde::Serialize;

pub const PROFILE_SCHEMA: &str = "trellis-nickel-export-cutover/v1";
pub const REQUEST_SCHEMA: &str = "onix-nickel-export-request/v1";
pub const RECEIPT_SCHEMA: &str = "onix-nickel-export-receipt/v1";
pub const MANIFEST_SCHEMA: &str = "onix-nickel-export-manifest/v1";
pub const GENERATOR_ID: &str = "nickel-export-core/v1";
pub const AUTHORITY_SCHEMA: &str = "trellis-nickel-export-authority-baseline/v1";
pub const EXPECTED_EXPORTER_REPOSITORY: &str = "github.com/OnixResearch/nickel-export";
pub const EXPECTED_EXPORTER_REVISION: &str = "257fafc1c746f1faf156207043a4c826bfb16d49";
pub const EXPECTED_NICKEL_IDENTITY: &str = "github:nickel-lang/nickel@1320a983e6c3d1e2fb53dd2464b084b4903b1426";
pub const EXPECTED_NICKEL_VERSION: &str = "1.17.0";
pub const EXPECTED_NICKEL_VERSION_OUTPUT: &str = "nickel-lang-cli nickel 1.17.0 (rev 1320a98)";
pub const NON_CLAIM_MARKERS: &[&str] = &[
    "evaluator equivalence",
    "product-policy conformance",
    "release eligibility",
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Profile {
    pub schema: String,
    pub exporter: Exporter,
    pub evaluator: EvaluatorProfile,
    pub authority_baseline: String,
    pub secret_material_opt_ins: Vec<SecretMaterialOptIn>,
    pub requests: serde_json::Value,
    pub cases: Vec<ExportCase>,
    pub invalid_families: Vec<InvalidFamily>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Exporter {
    pub repository: String,
    pub revision: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct EvaluatorProfile {
    pub identity: String,
    pub version: String,
    pub version_output: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ExportCase {
    pub id: String,
    pub request_field: String,
    pub request_path: String,
    pub manifest_path: String,
    pub compatibility_destinations: Vec<String>,
    pub destination_preexisting: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct InvalidFamily {
    pub id: String,
    pub directory: String,
    pub dependencies: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SecretMaterialOptIn {
    pub family_id: String,
    pub path: String,
    pub marker: String,
    pub rationale: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct ExportRequest {
    pub schema: String,
    pub family_id: String,
    pub source: String,
    pub dependencies: Vec<String>,
    pub import_paths: Vec<String>,
    pub selector: String,
    pub contract: String,
    pub format: String,
    pub destination: String,
    pub allow_secret_material: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct ArtifactIdentity {
    pub path: String,
    pub identity: String,
    pub bytes: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct EvaluatorDescriptor {
    pub identity: String,
    pub version: String,
    pub options: Vec<String>,
    pub import_path_policy: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct ExportReceipt {
    pub schema: String,
    pub family_id: String,
    pub source: ArtifactIdentity,
    pub dependencies: Vec<ArtifactIdentity>,
    pub import_paths: Vec<String>,
    pub selector: String,
    pub contract: String,
    pub format: String,
    pub output: ArtifactIdentity,
    pub evaluator: EvaluatorDescriptor,
    pub diagnostics: Vec<serde_json::Value>,
    pub non_claim: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct ExportManifest {
    pub schema: String,
    pub generator: String,
    pub evaluator: EvaluatorDescriptor,
    pub exports: Vec<ExportReceipt>,
    pub manifest_identity: String,
}

#[derive(Debug, Deserialize)]
pub struct ShellError {
    pub schema: String,
    pub stage: String,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct AuthorityBaseline {
    pub schema: String,
    pub entries: Vec<AuthorityEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct AuthorityEntry {
    pub path: String,
    pub token: String,
    pub count: usize,
}
