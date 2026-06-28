## Phase 1: Policy and planning core

- [x] [serial] Add a typed Nickel loop policy and checked-in generated JSON for target ranking, stop conditions, modes, and validation commands. r[proof-review.agent-loop]
- [x] [serial] Implement pure Rust planning helpers for candidate filtering, mode planning, slug generation, validation-plan construction, and receipt summaries. r[proof-review.agent-loop]
- [x] [serial] Add positive and negative self-tests for target selection, trust-boundary skipping, dirty-tree rejection, active-change rejection, unsupported apply patterns, and push-disabled behavior. r[proof-review.agent-loop]

## Phase 2: Imperative shell and evidence

- [x] [serial] Implement `scripts/proof-review-loop.rs` shell orchestration for dry-run and assisted modes using the existing scanner, generated packets, Cairn commands, evidence directories, and receipts. r[proof-review.agent-loop]
- [x] [serial] Add guarded apply-mode support only for deterministic vacuous coverage proof recipes that do not add trust boundaries. r[proof-review.agent-loop]
- [x] [serial] Capture per-iteration evidence logs for scanner output, VibeThinker packet path, focused tests, Verus, Tracey, Cairn gates, verify wrapper, sync/archive, and final status. r[proof-review.agent-loop]

## Phase 3: Validation and archive

- [x] [serial] Run loop self-tests, a dry-run loop, an assisted-mode smoke run that stops before source edits, Cairn validation/gates, Tracey coverage, and `TMPDIR=/tmp ./scripts/verify.rs`; save evidence logs. r[proof-review.agent-loop]
- [x] [serial] Sync/archive the change and commit the completed evidence. r[proof-review.agent-loop]
