let NonEmptyString = std.contract.from_predicate (fun value => std.is_string value && std.string.length value > 0) in
let NonNegativeNumber = std.contract.from_predicate (fun value => std.is_number value && value >= 0) in
let PositiveNumber = std.contract.from_predicate (fun value => std.is_number value && value > 0) in
let RuleWeight = {
  id | NonEmptyString,
  score | NonNegativeNumber,
  severity | NonEmptyString,
  rationale | NonEmptyString,
} in
let Policy = {
  version | NonEmptyString,
  generated_artifact | NonEmptyString,
  default_report | NonEmptyString,
  default_packet_dir | NonEmptyString,
  max_candidates | PositiveNumber,
  max_packet_findings | PositiveNumber,
  max_score_findings_per_rule | PositiveNumber,
  rules | Array RuleWeight,
  ignored_permanent_markers | Array NonEmptyString,
} in
{
  version = "1",
  generated_artifact = "proof-review/generated/policy.json",
  default_report = "target/proof-review/candidates.md",
  default_packet_dir = "target/proof-review/packets",
  max_candidates = 12,
  max_packet_findings = 6,
  max_score_findings_per_rule = 6,
  ignored_permanent_markers = [
    "@verified-logic:permanent-axiom",
    "@verified-logic:permanent-z3-limit",
  ],
  rules = [
    {
      id = "active-cairn-change",
      score = 600,
      severity = "review",
      rationale = "Active Cairn changes should be proof-reviewed before archive evidence is claimed.",
    },
    {
      id = "worktree-changed-source",
      score = 250,
      severity = "review",
      rationale = "Changed verified source is the most actionable proof-review surface.",
    },
    {
      id = "untrusted-boundary-marker",
      score = 100,
      severity = "high",
      rationale = "New or non-permanent trusted boundaries should be investigated before validation claims.",
    },
    {
      id = "permanent-boundary-marker",
      score = 10,
      severity = "note",
      rationale = "Permanent trusted boundaries are lower-priority but useful context for proof review.",
    },
    {
      id = "vacuous-ensures-true",
      score = 80,
      severity = "high",
      rationale = "Catalogue proofs with only `ensures true` do not establish a useful local property.",
    },
    {
      id = "empty-nontrivial-proof",
      score = 35,
      severity = "review",
      rationale = "Empty proof bodies with nontrivial postconditions are worth checking for vacuity or missing helper lemmas.",
    },
    {
      id = "active-prefix-semantics",
      score = 25,
      severity = "review",
      rationale = "Active prefixes, fuel, and bounded lengths often need boundary tests and explicit helper lemmas.",
    },
    {
      id = "missing-negative-test-hints",
      score = 20,
      severity = "review",
      rationale = "Proof slices should include sad-path tests for malformed, zero, inactive, or out-of-range cases.",
    },
  ],
} | Policy
