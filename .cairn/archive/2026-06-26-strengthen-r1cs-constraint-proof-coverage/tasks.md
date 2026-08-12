## Phase 1: Baseline and implementation

- [x] [serial] Run focused baseline `r1cs_constraint` tests before core edits. r[rust-primitives.r1cs-verification-coverage]
- [x] [serial] Replace vacuous row and dot-product coverage proofs with concrete implications. r[rust-primitives.r1cs-verification-coverage]
- [x] [serial] Add focused positive and negative runtime tests for any missing finite row-boundary cases. r[rust-primitives.r1cs-verification-coverage]

## Phase 2: Validation and archive

- [x] [serial] Run focused constraint tests, Verus over `src/lib.rs`, proof-review scanner, Tracey coverage, Cairn validation/gates, and `TMPDIR=/tmp ./scripts/verify.rs`; save evidence logs. r[rust-primitives.r1cs-verification-coverage]
- [x] [serial] Sync/archive the change and commit the completed evidence. r[rust-primitives.r1cs-verification-coverage]
