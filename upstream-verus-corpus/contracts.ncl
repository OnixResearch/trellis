let NonEmptyString = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in
let NonEmptyArray = std.contract.from_predicate (fun value =>
  std.is_array value && std.array.length value > 0
) in
let GitRevision = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{40}$" value
) in
let NixSha256 = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^sha256-[A-Za-z0-9+/]+={0,2}$" value
) in
let OneOf = fun allowed => std.contract.from_predicate (fun value =>
  std.is_string value && std.array.elem value allowed
) in
let CorpusEntry = {
  id | NonEmptyString,
  repository_url | NonEmptyString,
  revision | GitRevision,
  source_hash | NixSha256,
  source_hash_protocol | OneOf ["nix-sha256-unpacked"],
  license_status | OneOf ["mit-reviewed", "mixed-reviewed-reference-only"],
  license_sentinel | NonEmptyString,
  role | OneOf [
    "semantic-foundation",
    "reproducibility-benchmark",
    "allocator-proof-stress",
    "distributed-host-boundary-stress",
    "local-numa-proof-stress",
  ],
  execution_policy | OneOf ["reference-only", "opt-in-pinned", "blocked-current-verifier"],
  verifier_context | NonEmptyString,
  sentinels | NonEmptyArray | Array NonEmptyString,
  limitations | NonEmptyArray | Array NonEmptyString,
  non_claims | NonEmptyArray | Array NonEmptyString,
} in
{
  CorpusCatalog = {
    schema_version | OneOf ["trellis-upstream-verus-corpus/v1"],
    claim_scope | OneOf ["source-intake-and-regression-selection-only"],
    entries | NonEmptyArray | Array CorpusEntry,
    global_non_claims | NonEmptyArray | Array NonEmptyString,
  },
}
