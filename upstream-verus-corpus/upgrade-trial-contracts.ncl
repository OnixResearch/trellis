let NonEmptyString = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in
let NonEmptyArray = std.contract.from_predicate (fun value =>
  std.is_array value && std.array.length value > 0
) in
let GitRevision = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{40}$" value
) in
let Blake3OrNone = std.contract.from_predicate (fun value =>
  std.is_string value && (value == "none" || std.string.is_match "^[0-9a-f]{64}$" value)
) in
let PositiveInteger = std.contract.from_predicate (fun value =>
  std.is_number value && value > 0 && value % 1 == 0
) in
let NonNegativeInteger = std.contract.from_predicate (fun value =>
  std.is_number value && value >= 0 && value % 1 == 0
) in
let FalseOnly = std.contract.from_predicate (fun value => std.is_bool value && !value) in
let OneOf = fun allowed => std.contract.from_predicate (fun value =>
  std.is_string value && std.array.elem value allowed
) in
let CorpusId = OneOf [
  "paper-sosp24-artifact",
  "verified-ironkv",
  "verified-memory-allocator",
  "verified-node-replication",
  "verus-belt",
] in
let Mechanism = OneOf ["artifact-microcase", "native-verus", "semantic-reference"] in
let PromotionRole = OneOf ["required-native", "diagnostic-only", "audit-only"] in
let SourceVariant = OneOf ["adapted", "unmodified", "reference-only"] in
let ExpectedOutcome = OneOf [
  "passed-adapted",
  "passed-unmodified",
  "source-api-incompatible",
  "reference-audit-only",
] in
let Command = {
  id | NonEmptyString,
  argv | NonEmptyArray | Array NonEmptyString,
} in
let Adaptation = {
  patch_path | NonEmptyString,
  patch_blake3 | Blake3OrNone,
  semantic_audit | Array NonEmptyString,
} in
let TrialRowShape = {
  id | CorpusId,
  source_revision | GitRevision,
  source_variant | SourceVariant,
  adaptation | Adaptation,
  mechanism | Mechanism,
  promotion_role | PromotionRole,
  commands | NonEmptyArray | Array Command,
  expected_outcome | ExpectedOutcome,
  expected_exit_codes | NonEmptyArray | Array NonNegativeInteger,
  result_anchors | NonEmptyArray | Array NonEmptyString,
  timeout_seconds | PositiveInteger,
  maximum_saved_log_bytes | PositiveInteger,
  non_claims | NonEmptyArray | Array NonEmptyString,
} in
let TrialRowInvariant = std.contract.from_predicate (fun row =>
  let adaptation_valid =
    if row.source_variant == "adapted" then
      row.adaptation.patch_path != "none"
      && row.adaptation.patch_blake3 != "none"
      && std.array.length row.adaptation.semantic_audit > 0
      && row.expected_outcome == "passed-adapted"
    else
      row.adaptation.patch_path == "none"
      && row.adaptation.patch_blake3 == "none"
      && row.expected_outcome != "passed-adapted"
  in
  let required_native_passes =
    row.promotion_role != "required-native"
    || row.expected_outcome == "passed-adapted"
    || row.expected_outcome == "passed-unmodified"
  in
  adaptation_valid && required_native_passes
) in
{
  UpgradeTrial = {
    schema_version | OneOf ["trellis-octet-verus-upgrade-trial/v1"],
    trial_id | NonEmptyString,
    baseline | {
      verifier | OneOf ["verus@0.2026.03.22.5e66329"],
      expected_result | OneOf ["passed"],
      expected_verified_count | NonNegativeInteger,
      expected_error_count | NonNegativeInteger,
    },
    candidate | {
      provider | OneOf ["octet"],
      provider_revision | GitRevision,
      profile_id | OneOf ["octet-verus-0.2026.05.17.e479cce"],
      verifier | OneOf ["verus@0.2026.05.17.e479cce"],
      expected_result | OneOf ["passed"],
      expected_verified_count | NonNegativeInteger,
      expected_error_count | NonNegativeInteger,
      result_anchors | NonEmptyArray | Array NonEmptyString,
    },
    rows | NonEmptyArray | Array TrialRowShape | Array TrialRowInvariant,
    decision | {
      expected | OneOf ["promote"],
      preserve_current_pin | FalseOnly,
      octet_compatibility_status | OneOf ["local-pin", "native"],
      recommended_octet_compatibility_status | OneOf ["native"],
      blockers | Array NonEmptyString,
    },
    claim_scope | OneOf ["bounded-verifier-upgrade-compatibility-trial"],
    non_claims | NonEmptyArray | Array NonEmptyString,
  },
}
