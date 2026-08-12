# Tasks

## Phase 1: Spec foundation

- [x] [serial] Create the OpenSpec package for proof-gap inventory drift detection.
- [x] [parallel] Specify the Rust-script-only inventory gate and non-goals.

## Phase 2: Implementation

- [x] [serial] Add `scripts/check-proof-gap-inventory.rs` with source scanning, expected-count comparison, report output, and negative fixture mode.
- [x] [depends:implementation] Wire the Rust script into `./scripts/verify.sh` before expensive verification stages.
- [x] [depends:implementation] Add Tracey-covered verified constants/proofs for the drift-gate requirement.

## Phase 3: Evidence

- [x] [depends:implementation] Capture passing script evidence and failing negative-fixture evidence under this change.
- [x] [depends:evidence] Run focused/full verification: strict OpenSpec validation, cargo tests, clippy, Verus, Tracey uncovered/untested, and `./scripts/verify.sh`.
- [x] [depends:verification] Sync canonical docs/specs, archive, commit, push, and verify clean state.
