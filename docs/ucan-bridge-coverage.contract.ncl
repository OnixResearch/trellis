# Trellis UCAN bridge coverage contract.

let NonEmptyString = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in
let Blake3Hex = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-fA-F]{64}$" value
) in
let BridgeId = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^trellis\\.ucan_bridge\\.[a-z0-9_.-]+$" value
) in
let CoverageSchemaVersion = std.contract.from_predicate (fun value =>
  std.is_string value && value == "trellis.ucan-bridge-coverage.v1"
) in
let EvidenceTier = std.contract.from_predicate (fun value =>
  std.is_string value && (value == "formal_proof" || value == "manual_review" || value == "recorded_identity")
) in
rec {
  CoverageRow = {
    bridge_id | BridgeId,
    ucan_helper | NonEmptyString,
    trellis_primitive_id | NonEmptyString,
    source_blake3 | Blake3Hex,
    proof_blake3 | Blake3Hex,
    expected_vendored_path | NonEmptyString,
    evidence_tier | EvidenceTier,
    claim | NonEmptyString,
    non_claims | Array NonEmptyString,
  },

  CoverageManifest = {
    schema_version | CoverageSchemaVersion,
    producer | NonEmptyString,
    boundary | NonEmptyString,
    rows | Array CoverageRow,
  },
}
