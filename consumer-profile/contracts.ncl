let NonEmptyString = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in
let GitRevision = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{40}$" value
) in
let Blake3Hex = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{64}$" value
) in
let SchemaVersion = std.contract.from_predicate (fun value =>
  std.is_string value && value == "trellis-verified-logic-consumer-profile/v1"
) in
let ConsumerSchemaVersion = std.contract.from_predicate (fun value =>
  std.is_string value && value == "trellis-verified-logic-consumer/v1"
) in
let ConsumerMode = std.contract.from_predicate (fun value =>
  std.is_string value && std.array.elem value ["direct_dependency", "vendored_snapshot"]
) in
let DirtyStatePolicy = std.contract.from_predicate (fun value =>
  std.is_string value && std.array.elem value ["not_applicable", "deny_unrecorded_changes"]
) in
let Artifact = {
  path | NonEmptyString,
  bytes | Number,
  blake3 | Blake3Hex,
} in
let PatchException = {
  approved | Bool,
  rationale | String,
  verification_evidence | String,
  claim_boundary | String,
} in
let LocalPatch = {
  path | NonEmptyString,
  bytes | Number,
  blake3 | Blake3Hex,
  exception | PatchException,
} in
let PackageProfile = {
  name | NonEmptyString,
  version | NonEmptyString,
  dependency_coordinate | NonEmptyString,
  source_repository | NonEmptyString,
  revision | GitRevision,
  default_features | Bool,
  supported_features | Array String,
} in
let VerificationEvidence = {
  proof_owner | NonEmptyString,
  proof_command | NonEmptyString,
  proof_manifest_path | NonEmptyString,
  proof_manifest_bytes | Number,
  proof_manifest_blake3 | Blake3Hex,
  supported_claim | NonEmptyString,
} in
let VendoredSnapshotPolicy = {
  snapshot_root | NonEmptyString,
  dirty_state_policy | DirtyStatePolicy,
  local_patch_policy | NonEmptyString,
} in
let ConsumerProfileContract = {
  schema_version | SchemaVersion,
  report_schema | NonEmptyString,
  release_id | NonEmptyString,
  package | PackageProfile,
  vendored_snapshot | VendoredSnapshotPolicy,
  ucan_modules | Array NonEmptyString,
  artifacts | Array Artifact,
  verification | VerificationEvidence,
  required_non_claims | Array NonEmptyString,
} in
let ConsumerManifestContract = {
  schema_version | ConsumerSchemaVersion,
  consumer_id | NonEmptyString,
  mode | ConsumerMode,
  dirty_state_policy | DirtyStatePolicy,
  package_name | String,
  package_version | String,
  dependency_coordinate | String,
  source_repository | String,
  revision | String,
  default_features | Bool,
  features | Array String,
  modules | Array String,
  snapshot_root | String,
  artifacts | Array Artifact,
  local_patches | Array LocalPatch,
  non_claims | Array String,
} in
let ConsumerFixtureContract = {
  name | NonEmptyString,
  expected_valid | Bool,
  expected_diagnostic | NonEmptyString,
  consumer | ConsumerManifestContract,
} in
{
  ConsumerProfile = ConsumerProfileContract,
  ConsumerManifest = ConsumerManifestContract,
  ConsumerFixture = ConsumerFixtureContract,
}
