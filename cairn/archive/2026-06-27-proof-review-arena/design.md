## Context

The loop selected `src/arena.rs` with score `255` and findings `empty-nontrivial-proof, vacuous-ensures-true`. VibeThinker packet output is advisory-only; repository checks remain authoritative.

## Decisions

### 1. Keep the iteration bounded

**Choice:** Handle one proof-review target in this package.

**Rationale:** One target keeps source diffs, evidence, and archive review small.

### 2. Do not expand trust

**Choice:** Reject any patch that adds `external_body`, `assume_specification`, `assume(`, or `@verified-logic:no-ensures`.

**Rationale:** Proof-review automation exists to harden coverage, not to hide gaps.

### 3. Validation decides completion

**Choice:** Archive only after focused checks, Verus, Tracey, Cairn gates, and the repo verification wrapper succeed.

**Rationale:** Advisory model critique can guide review, but cannot prove correctness.
