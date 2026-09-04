let NonEmptyString = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in
let PositiveNumber = std.contract.from_predicate (fun value =>
  std.is_number value && value > 0
) in
let B3Identity = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^b3:[0-9a-f]{64}$" value
) in
let HexDigest = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{64}$" value
) in
let GitRevision = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{40}$" value
) in
let RelativePath = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.string.length value > 0
  && !std.string.is_match "^/" value
  && !std.string.is_match "(^|/)\\.\\.(/|$)" value
) in
let OneOf = fun allowed => std.contract.from_predicate (fun value =>
  std.is_string value && std.array.elem value allowed
) in
let NonEmptyArray = std.contract.from_predicate (fun value =>
  std.is_array value && std.array.length value > 0
) in
let Artifact = {
  role | NonEmptyString,
  path | RelativePath,
  bytes | PositiveNumber,
  blake3 | B3Identity,
} in
let TrustInput = {
  input_id | NonEmptyString,
  identity | B3Identity,
} in
let Theorem = {
  theorem_id | NonEmptyString,
  path | RelativePath,
  declaration_name | NonEmptyString,
  requirement_id | NonEmptyString,
  claim_class | OneOf ["named-theorem-over-generated-model"],
  approved_statement_blake3 | B3Identity,
  verus_proof_symbol | NonEmptyString,
  lean_theorem_symbol | NonEmptyString,
  integer_model | NonEmptyString,
  overflow_model | NonEmptyString,
  axioms | Array NonEmptyString,
} in
{
  NegativeCatalog = {
    schema_version | OneOf ["trellis-aeneas-negative-catalog/v1"],
    cases | NonEmptyArray | Array {
      id | NonEmptyString,
      expected_diagnostic | NonEmptyString,
    },
  },
  ResourcePolicy = {
    schema_version | OneOf ["trellis-aeneas-resource-policy/v1"],
    policy_id | NonEmptyString,
    timeout_ms | PositiveNumber,
    stdout_max_bytes | PositiveNumber,
    stderr_max_bytes | PositiveNumber,
    artifact_max_bytes | PositiveNumber,
    environment_allowlist | NonEmptyArray | Array NonEmptyString,
    network_allowed | Bool,
  },
  Profile = {
    schema_version | OneOf ["trellis-aeneas-cross-verifier-profile/v1"],
    pilot_id | NonEmptyString,
    outcome | OneOf ["supported"],
    octet_source | {
      repository | NonEmptyString,
      revision | GitRevision,
      consumer_contract_path | RelativePath,
      consumer_contract_identity | B3Identity,
      policy_artifact_identity | B3Identity,
      provider_profile_identity | B3Identity,
      compatibility_matrix_identity | B3Identity,
    },
    consumer | {
      schema_version | NonEmptyString,
      profile_id | NonEmptyString,
      consumer_id | NonEmptyString,
      provider | {
        backend | NonEmptyString,
        profile_id | NonEmptyString,
        profile_identity | B3Identity,
        compatibility_matrix_id | NonEmptyString,
        compatibility_matrix_identity | B3Identity,
        network_allowed | Bool,
      },
      source | {
        root | RelativePath,
        crate_name | NonEmptyString,
        paths | NonEmptyArray,
        selected_items | NonEmptyArray,
        broad_selection_allowed | Bool,
      },
      theorems | NonEmptyArray,
      external_models | Array TrustInput,
      trust_inputs | Array TrustInput,
      resource_policy | {
        timeout_ms | PositiveNumber,
        stdout_max_bytes | PositiveNumber,
        stderr_max_bytes | PositiveNumber,
        environment_allowlist | NonEmptyArray | Array NonEmptyString,
      },
      outcomes | NonEmptyArray | Array NonEmptyString,
      trust_closure_complete | Bool,
      non_claims | NonEmptyArray | Array NonEmptyString,
    },
    provider_tools | {
      package | NonEmptyString,
      identity_schema | NonEmptyString,
      aeneas_revision | GitRevision,
      charon_revision | GitRevision,
      lean_toolchain | NonEmptyString,
      aeneas_binary_blake3 | HexDigest,
      charon_binary_blake3 | HexDigest,
      lean_binary_blake3 | HexDigest,
      lake_binary_blake3 | HexDigest,
      leantar_binary_blake3 | HexDigest,
      network_sandbox_blake3 | HexDigest,
    },
    extraction | {
      charon_argv | NonEmptyArray | Array NonEmptyString,
      cargo_argv | NonEmptyArray | Array NonEmptyString,
      aeneas_argv | NonEmptyArray | Array NonEmptyString,
      lean_argv | NonEmptyArray | Array NonEmptyString,
      expected_has_errors | Bool,
      expected_skip_borrowck | Bool,
      expected_target | NonEmptyString,
      expected_pointer_width_bytes | PositiveNumber,
      expected_little_endian | Bool,
    },
    resource_policy | {
      path | RelativePath,
      identity | B3Identity,
    },
    artifacts | NonEmptyArray | Array Artifact,
    theorem_mappings | NonEmptyArray | Array Theorem,
    trust_closure | {
      complete | Bool,
      selected_axioms | NonEmptyArray | Array NonEmptyString,
      selected_opaque_definitions | Array NonEmptyString,
    },
    verus | {
      receipt_path | RelativePath,
      requirement_id | NonEmptyString,
      proof_symbols | NonEmptyArray | Array NonEmptyString,
    },
    negative_fixture_catalog | RelativePath,
    required_non_claims | NonEmptyArray | Array NonEmptyString,
  },
}
