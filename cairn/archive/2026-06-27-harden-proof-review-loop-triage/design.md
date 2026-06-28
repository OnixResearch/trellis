## Context

The proof-review loop is intentionally bounded and advisory-first. It should make the next human/verifier target clearer, not create noisy active-change candidates from archived evidence or accept speculative model suggestions as validation. The existing scanner reads all active-change markdown, generated packages preserve only a packet path, and Tracey retry behavior lives in operator memory rather than an executable gate.

## Decisions

### 1. Treat evidence markdown as historical context, not active source scope

**Choice:** Skip directories named `evidence` while recursively collecting active-change markdown for source-path extraction. Continue reading `tasks.md` directly and continue using proposal/design/spec/task text as active scope.

**Rationale:** Archived scanner reports often contain many source paths that should not steer the next active-change score. Tasks and authored specs remain the authoritative active work surface.

### 2. Make advisory packets executable-action oriented

**Choice:** Extend the packet output contract to require public/runtime API tests, reject `requires`-violating or infeasible stress suggestions, and separate accepted actionable items from rejected advisory items.

**Rationale:** VibeThinker remains useful for critique, but future packets should bias toward small checks that a maintainer can actually implement and validate.

### 3. Encode Tracey retry behavior in a script

**Choice:** Add `scripts/proof-review-tracey-coverage.rs` and update policy validation to invoke it. The script runs uncovered and untested queries and retries a failing query once only when the combined output contains `Error: Cancelled`.

**Rationale:** A narrow executable retry handles the known daemon auto-start race while still failing closed for real coverage or tool failures.

### 4. Generate stronger archive tasks

**Choice:** Update assisted package generation to require target-cleared scanner evidence, Tracey retry evidence, and frozen validation logs with post-archive refreshes written separately.

**Rationale:** Future iterations should show that the selected finding was cleared or consciously left advisory, and archived logs should remain stable for review.

## Risks / Trade-offs

- Evidence markdown under active changes no longer contributes source references; if an evidence file is the only place mentioning the real target, the task/spec text should be fixed instead of relying on generated logs.
- The Tracey wrapper intentionally does not retry arbitrary non-zero output, so persistent failures still block validation.
