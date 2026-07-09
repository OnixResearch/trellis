# Trellis proof artifact manifest contract.

let NonEmptyString = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in
let Blake3Hex = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-fA-F]{64}$" value
) in
let PositiveBytes = std.contract.from_predicate (fun value =>
  std.is_number value && value > 0
) in
let PrimitiveId = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^trellis\\.[a-z0-9_.-]+$" value
) in
let TrustedBoundary = std.contract.from_predicate (fun value =>
  std.is_string value && std.array.elem value ["verified_module", "trusted_runtime_boundary", "proof_gap_boundary", "manual_review_external"]
) in
let EvidenceTier = std.contract.from_predicate (fun value =>
  std.is_string value && std.array.elem value ["formal_proof", "manual_review", "recorded_identity"]
) in
let SchemaVersion = std.contract.from_predicate (fun value =>
  std.is_string value && value == "trellis.proof-artifact-manifest.v1"
) in
rec {
  ArtifactIdentity = {
    path | NonEmptyString,
    blake3 | Blake3Hex,
    bytes | PositiveBytes,
  },

  VerifierIdentity = {
    identity | NonEmptyString,
    argv | Array NonEmptyString,
  },

  PrimitiveRow = {
    primitive_id | PrimitiveId,
    module_path | NonEmptyString,
    public_api | Array NonEmptyString,
    requirement_ids | Array NonEmptyString,
    source | ArtifactIdentity,
    proof | ArtifactIdentity,
    verifier | VerifierIdentity,
    trusted_boundary | TrustedBoundary,
    evidence_tier | EvidenceTier,
    non_claims | Array NonEmptyString,
  },

  Manifest = {
    schema_version | SchemaVersion,
    producer | NonEmptyString,
    boundary | NonEmptyString,
    primitives | Array PrimitiveRow,
  },
}
