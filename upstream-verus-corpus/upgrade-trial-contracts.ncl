let NonEmptyString = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in
let NonEmptyArray = std.contract.from_predicate (fun value =>
  std.is_array value && std.array.length value > 0
) in
let GitRevision = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{40}$" value
) in
let PositiveInteger = std.contract.from_predicate (fun value =>
  std.is_number value && value > 0 && value % 1 == 0
) in
let NonNegativeInteger = std.contract.from_predicate (fun value =>
  std.is_number value && value >= 0 && value % 1 == 0
) in
let TrueOnly = std.contract.from_predicate (fun value => std.is_bool value && value) in
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
let ExpectedOutcome = OneOf [
  "source-api-incompatible",
  "verifier-internal-error",
  "reference-audit-only",
] in
let Command = {
  id | NonEmptyString,
  argv | NonEmptyArray | Array NonEmptyString,
} in
let TrialRow = {
  id | CorpusId,
  source_revision | GitRevision,
  mechanism | Mechanism,
  promotion_role | PromotionRole,
  commands | NonEmptyArray | Array Command,
  expected_outcome | ExpectedOutcome,
  diagnostic_anchors | NonEmptyArray | Array NonEmptyString,
  timeout_seconds | PositiveInteger,
  maximum_saved_log_bytes | PositiveInteger,
  non_claims | NonEmptyArray | Array NonEmptyString,
} in
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
      profile_id | OneOf ["octet-verus-0.2026.04.03.21dfcd2"],
      verifier | OneOf ["verus@0.2026.04.03.21dfcd2"],
      expected_result | OneOf ["proof-failure"],
      expected_verified_count | NonNegativeInteger,
      expected_error_count | NonNegativeInteger,
      diagnostic_anchors | NonEmptyArray | Array NonEmptyString,
    },
    rows | NonEmptyArray | Array TrialRow,
    decision | {
      expected | OneOf ["blocked"],
      preserve_current_pin | TrueOnly,
      octet_compatibility_status | OneOf ["local-pin"],
      blockers | NonEmptyArray | Array NonEmptyString,
    },
    claim_scope | OneOf ["bounded-verifier-upgrade-compatibility-trial"],
    non_claims | NonEmptyArray | Array NonEmptyString,
  },
}
