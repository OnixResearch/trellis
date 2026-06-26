## Phase 1: Baseline and implementation

- [x] [serial] Run focused baseline `r1cs_gates` tests before core edits. r[rust-primitives.r1cs-gate-verification-coverage]
- [x] [serial] Replace the vacuous gate coverage proof with concrete rejection implications. r[rust-primitives.r1cs-gate-verification-coverage]
- [x] [serial] Add focused positive and negative runtime tests for the missing finite gate cases. r[rust-primitives.r1cs-gate-verification-coverage]

## Phase 2: Validation and archive

- [x] [serial] Run focused gate tests, Verus over `src/lib.rs`, proof-review scanner, Tracey coverage, Cairn validation/gates, and `TMPDIR=/tmp ./scripts/verify.rs`; save evidence logs. r[rust-primitives.r1cs-gate-verification-coverage]
- [x] [serial] Sync/archive the change and commit the completed evidence. r[rust-primitives.r1cs-gate-verification-coverage]
