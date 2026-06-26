# Final Tasks Snapshot

## Phase 1: Core slot model

- [x] Add `src/stack_frame_layout.rs` with `SlotKind::{Local, Spill}`, `StackSlot`, `LiveRange`, `LiveSlot`, and `FrameLayout`
- [x] Add byte-range, overlap, alignment, and exact frame-size specs plus exec wrappers (`byte_range`, `ranges_overlap`, `check_alignment`, `compute_frame_size`)
- [x] Add append-only frame construction and assignment helpers (`empty_frame`, `assign_slot`)
- [x] Add explicit safe-reuse and live-range compatibility helpers (`can_reuse_slot`, `check_non_overlap`)
- [x] Export the module from `src/lib.rs`

## Phase 2: Proofs and tests

- [x] Add proof lemmas for aligned round-up lower bounds and reuse safety (`round_up_ge`, `reusable_same_range_requires_disjoint_liveness`)
- [x] Add positive unit tests for empty-frame initialization, aligned assignment, max-extent placement, and disjoint-live-range reuse
- [x] Add negative unit tests for zero stack alignment, malformed slot rejection, and overlapping-live-range rejection
- [x] Add proptests for assignment alignment, frame coverage, and shared-slot safety for disjoint live ranges

## Phase 3: Spec sync

- [x] Add `r[stack-frame.*]` requirements to `docs/requirements.md`
- [x] Sync the active change proposal/design/specs/tasks to the landed local/spill stack-frame scope
- [x] Add the canonical spec at `openspec/specs/stack-frame-layout/spec.md`
- [x] Update README module tables and headline counts for `stack_frame_layout`

## Phase 4: Validation and archive

- [x] Run `cargo test --quiet stack_frame_layout` and capture `evidence/cargo-test.txt`
- [x] Run `./scripts/verus.sh src/stack_frame_layout.rs` and capture `evidence/verus-stack-frame-layout.txt`
- [x] Run `openspec_gate` for proposal, design, and tasks and capture `evidence/openspec-*.txt`
- [x] Archive the change under `openspec/changes/archive/` with evidence
