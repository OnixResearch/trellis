## Context

The loop selected `src/beam_link.rs` with score `255` and findings `empty-nontrivial-proof`. VibeThinker packet output is advisory-only; repository checks remain authoritative.

## Decisions

### 1. Keep the iteration bounded

**Choice:** Handle one proof-review target in this package.

**Rationale:** One target keeps source diffs, evidence, and archive review small.

### 2. Do not expand trust

**Choice:** Reject any patch that adds `external_body`, `assume_specification`, `assume(`, or `@verified-logic:no-ensures`.

**Rationale:** Proof-review automation exists to harden coverage, not to hide gaps.

### 3. Validation decides completion

**Choice:** Archive only after focused checks, Verus, Tracey, Cairn gates, the repo verification wrapper, and a target-cleared scanner rerun succeed or document the remaining finding.

**Rationale:** Advisory model critique can guide review, but cannot prove correctness.

### 4. Retry only known transient Tracey failures

**Choice:** If Tracey daemon auto-start reports transient cancellation, rerun direct uncovered and untested queries once and keep both attempts in evidence.

**Rationale:** A retry handles the documented daemon race without hiding persistent coverage failures.

### 5. Freeze archive evidence

**Choice:** Treat pre-archive validation logs as immutable once copied into the archive; save later status checks as post-archive evidence files.

**Rationale:** Frozen logs keep archive review reproducible and prevent accidental evidence drift.
