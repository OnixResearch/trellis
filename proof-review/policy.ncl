let NonEmptyString = std.contract.from_predicate (fun value => std.is_string value && std.string.length value > 0) in
let NonNegativeNumber = std.contract.from_predicate (fun value => std.is_number value && value >= 0) in
let PositiveNumber = std.contract.from_predicate (fun value => std.is_number value && value > 0) in
let Boolean = std.contract.from_predicate (fun value => std.is_bool value) in
let StringList = Array NonEmptyString in
let RuleWeight = {
  id | NonEmptyString,
  score | NonNegativeNumber,
  severity | NonEmptyString,
  rationale | NonEmptyString,
} in
let LoopMode = {
  name | NonEmptyString,
  description | NonEmptyString,
  mutates_lifecycle | Boolean,
  mutates_source | Boolean,
  runs_validation | Boolean,
} in
let StopCondition = {
  id | NonEmptyString,
  severity | NonEmptyString,
  rationale | NonEmptyString,
} in
let ValidationCommand = {
  id | NonEmptyString,
  modes | StringList,
  command | StringList,
  evidence | NonEmptyString,
  required | Boolean,
} in
let ApplyRecipe = {
  id | NonEmptyString,
  rule_id | NonEmptyString,
  requires_no_trust_markers | Boolean,
  mutates_source | Boolean,
  validation_gate | NonEmptyString,
  rationale | NonEmptyString,
} in
let LoopPolicy = {
  default_mode | NonEmptyString,
  default_max_iterations | PositiveNumber,
  max_iterations_limit | PositiveNumber,
  receipt_dir | NonEmptyString,
  no_push_default | Boolean,
  slug_prefix | NonEmptyString,
  preferred_candidate_rules | StringList,
  skip_trust_boundary_rules | StringList,
  modes | Array LoopMode,
  stop_conditions | Array StopCondition,
  validation_commands | Array ValidationCommand,
  apply_recipes | Array ApplyRecipe,
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
  ignored_permanent_markers | StringList,
  loop | LoopPolicy,
} in
{
  version = "2",
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
  loop = {
    default_mode = "dry-run",
    default_max_iterations = 1,
    max_iterations_limit = 3,
    receipt_dir = "target/proof-review/loop-receipts",
    no_push_default = true,
    slug_prefix = "proof-review",
    preferred_candidate_rules = [
      "vacuous-ensures-true",
      "empty-nontrivial-proof",
      "missing-negative-test-hints",
      "active-prefix-semantics",
    ],
    skip_trust_boundary_rules = [
      "untrusted-boundary-marker",
      "permanent-boundary-marker",
    ],
    modes = [
      {
        name = "dry-run",
        description = "Plan one candidate and validation receipt without mutating source, lifecycle state, archives, commits, or remotes.",
        mutates_lifecycle = false,
        mutates_source = false,
        runs_validation = false,
      },
      {
        name = "assisted",
        description = "Create one reviewable Cairn change package and stop before source edits.",
        mutates_lifecycle = true,
        mutates_source = false,
        runs_validation = false,
      },
      {
        name = "apply",
        description = "Apply only deterministic proof-hardening recipes that pass the no-trust-boundary guard and configured validation gates.",
        mutates_lifecycle = true,
        mutates_source = true,
        runs_validation = true,
      },
    ],
    stop_conditions = [
      {
        id = "dirty-tree",
        severity = "blocker",
        rationale = "The loop must not mix unrelated local edits into generated evidence or commits.",
      },
      {
        id = "active-cairn-change",
        severity = "blocker",
        rationale = "Only one active Cairn package may be driven by the loop at a time.",
      },
      {
        id = "trust-boundary-only",
        severity = "blocker",
        rationale = "Permanent and untrusted boundary findings are review context, not automatic proof-hardening targets.",
      },
      {
        id = "unsupported-apply-pattern",
        severity = "blocker",
        rationale = "Apply mode is limited to deterministic vacuous coverage proof recipes.",
      },
      {
        id = "validation-failed",
        severity = "blocker",
        rationale = "Validation output, not advisory model output, decides whether an iteration may archive.",
      },
      {
        id = "push-disabled",
        severity = "blocker",
        rationale = "The loop never pushes by default and this repo keeps push as an explicit human action.",
      },
      {
        id = "missing-evidence",
        severity = "blocker",
        rationale = "Archive completion requires scanner, packet, validation, sync, archive, and final status evidence.",
      },
    ],
    validation_commands = [
      {
        id = "scanner",
        modes = ["dry-run", "assisted", "apply"],
        command = ["./scripts/proof-review-candidates.rs", "--check-policy"],
        evidence = "proof-review-candidates.log",
        required = true,
      },
      {
        id = "focused-tests",
        modes = ["apply"],
        command = ["cargo", "test", "--quiet"],
        evidence = "focused-tests.log",
        required = true,
      },
      {
        id = "verus-lib",
        modes = ["apply"],
        command = ["./scripts/verus.sh", "src/lib.rs"],
        evidence = "verus-src-lib.log",
        required = true,
      },
      {
        id = "tracey-coverage",
        modes = ["apply"],
        command = ["./scripts/proof-review-tracey-coverage.rs"],
        evidence = "tracey-coverage.log",
        required = true,
      },
      {
        id = "cairn-gates",
        modes = ["assisted", "apply"],
        command = ["cairn", "validate", "--root", "."],
        evidence = "cairn-gates.log",
        required = true,
      },
      {
        id = "verify-wrapper",
        modes = ["apply"],
        command = ["TMPDIR=/tmp", "./scripts/verify.rs"],
        evidence = "verify-rs.log",
        required = true,
      },
    ],
    apply_recipes = [
      {
        id = "vacuous-coverage-proof",
        rule_id = "vacuous-ensures-true",
        requires_no_trust_markers = true,
        mutates_source = true,
        validation_gate = "verify-wrapper",
        rationale = "Only catalogue proofs with vacuous ensures are eligible for deterministic apply-mode hardening, and only when no trust-boundary finding is present for the selected target.",
      },
    ],
  },
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
