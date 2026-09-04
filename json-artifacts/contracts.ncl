let Common = import "../policy-contracts/common.ncl" in
let proof_schema_contract = Common.enum ["trellis-proof-evidence-v1"] in
let proof_algorithm_contract = Common.enum ["blake3"] in
let proof_outcome_contract = Common.enum ["verified"] in
let tracey_coverage_contract = Common.enum ["covered"] in
let octet_schema_version_contract = std.contract.from_predicate (fun value => std.is_number value && value == 1) in
let octet_finding_schema_version_contract = std.contract.from_predicate (fun value => std.is_number value && value == 2) in
let octet_identity_status_contract = Common.enum ["computed"] in
let octet_severity_contract = Common.enum ["warning", "error"] in
let content_identity_contract = {
  path | Common.SafeRelativePath,
  algorithm | proof_algorithm_contract,
  digest | Common.Blake3Hex,
} in
let proof_record_contract = {
  scope | Common.NonEmptyString,
  evidence_tier | Common.NonEmptyString,
  requirement_ids | Common.NonEmptyArray | Array Common.StableId,
  source_paths | Common.NonEmptyArray | Array Common.SafeRelativePath,
  proof_functions | Array Common.NonEmptyString,
  algorithm_profile | Common.NonEmptyString,
  executable_oracle_symbol | Common.NonEmptyString,
  downstream_validation_requirement | Common.NonEmptyString,
  proof_command | Common.NonEmptyString,
  verifier_toolchain | Common.NonEmptyString,
  proof_outcome | proof_outcome_contract,
  tracey_coverage | tracey_coverage_contract,
  proof_gap_references | Array Common.NonEmptyString,
  maturity_facts | Array Common.NonEmptyString,
  content_identities | Common.NonEmptyArray | Array content_identity_contract,
  claims | Common.NonEmptyArray | Array Common.NonEmptyString,
  non_claims | Common.NonEmptyArray | Array Common.NonEmptyString,
} in
let required_non_claims = [
  "not release approval",
  "not whole-program correctness",
  "not third-party conformance",
] in
let claim_is_scoped = fun claim =>
  !std.string.is_match ".*release approval.*" claim
  && !std.string.is_match ".*whole-program correctness.*" claim
  && !std.string.is_match ".*certification.*" claim
  && !std.string.is_match ".*third-party conformance.*" claim
  && !std.string.is_match ".*production safety.*" claim
  && !std.string.is_match ".*downstream certified.*" claim
  && !std.string.is_match ".*verified external.*" claim
  && !std.string.is_match ".*bend correctness.*" claim
  && !std.string.is_match ".*hvm2 correctness.*" claim
  && !std.string.is_match ".*generated-c equivalence.*" claim
  && !std.string.is_match ".*hvm interactions.*" claim in
let proof_record_integrity = fun record =>
  Common.all_in required_non_claims record.non_claims
  && std.array.all claim_is_scoped record.claims in
let proof_bundle_contract = {
  schema_version | proof_schema_contract,
  generated_by | Common.SafeRelativePath,
  records | Common.NonEmptyArray | Array proof_record_contract,
} in
let proof_bundle_integrity_contract = std.contract.from_predicate (fun bundle =>
  std.array.all proof_record_integrity bundle.records
) in
let octet_metadata_contract = {
  tool_name | Common.NonEmptyString,
  tool_version | Common.NonEmptyString,
  rustc_version | Common.NonEmptyString,
  toolchain | Common.NonEmptyString,
  profile_name | Common.NonEmptyString,
  profile_hash | Common.Blake3DigestText,
  config_hash | Common.Blake3DigestText,
} in
let octet_finding_contract = {
  stable_id | Common.ShortBlake3DigestText,
  semantic_id | Common.ShortBlake3DigestText,
  identity_status | octet_identity_status_contract,
  lint | Common.StableId,
  group | Common.NonEmptyString,
  severity | octet_severity_contract,
  file | Common.SafeRelativePath,
  line | Common.PositiveNumber,
  column | Common.PositiveNumber,
  message | Common.NonEmptyString,
  crate_name | Common.StableId,
} in
let octet_baseline_contract = {
  schema_version | octet_schema_version_contract,
  finding_schema_version | octet_finding_schema_version_contract,
  metadata | octet_metadata_contract,
  baseline_hash | Common.Blake3DigestText,
  findings | Common.NonEmptyArray | Array octet_finding_contract,
} in
{
  ContentIdentity = content_identity_contract,
  ProofRecord = proof_record_contract,
  ProofEvidenceBundle = proof_bundle_contract,
  ProofEvidenceBundleIntegrity = proof_bundle_integrity_contract,
  OctetMetadata = octet_metadata_contract,
  OctetFinding = octet_finding_contract,
  OctetBaseline = octet_baseline_contract,
}
