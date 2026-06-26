## Phase 1: Specification

- [x] [serial] Define the proposal, design, tasks, and downstream evidence requirements for Aspen verified-logic snapshots.

## Phase 2: Index and Snapshot Contract

- [x] [depends:spec] Add the local Aspen verified-logic evidence index format and at least one checked example row.
- [x] [depends:spec] Add a source-commit/path drift checker for the index that fails closed on malformed, stale, or unsupported rows.
- [x] [depends:spec] Document the optional local proof snapshot layout and source-of-truth boundary.

## Phase 3: Evidence and Gates

- [x] [depends:index] Capture positive index validation evidence and at least one negative drift/malformed-row evidence artifact.
- [x] [depends:index] If any proof code is snapshotted, run the local Verus command for that snapshot and capture evidence. No proof code is snapshotted in this slice; see `verification.md`.
- [x] [depends:index] Run the repository's focused verification gates, including OpenSpec validation and proof-gap inventory checks if affected.
