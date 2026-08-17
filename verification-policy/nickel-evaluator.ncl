let NonEmptyString = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in
let Descriptor = {
  schema_version | NonEmptyString,
  evaluator_identity | NonEmptyString,
  evaluator_version | NonEmptyString,
  source_commit | NonEmptyString,
  proof_authority | NonEmptyString,
  non_claims | Array NonEmptyString,
} in
({
  schema_version = "trellis-nickel-evaluator/v1",
  evaluator_identity = "nickel-lang-cli nickel 1.17.0 (rev 1320a98)",
  evaluator_version = "1.17.0",
  source_commit = "1320a983e6c3d1e2fb53dd2464b084b4903b1426",
  proof_authority = "named-proof-tools",
  non_claims = [
    "configuration acceptance does not prove a theorem",
    "proof evidence does not prove downstream correctness",
  ],
} | Descriptor)
