## Phase 1: Baseline and implementation

- [x] [serial] Run focused baseline `r1cs_public` tests before core edits. r[rust-primitives.r1cs-public-input-verification-coverage]
- [x] [serial] Replace the vacuous public-input coverage proof with concrete implications. r[rust-primitives.r1cs-public-input-verification-coverage]
- [x] [serial] Add focused positive and negative runtime tests for missing finite public-input cases. r[rust-primitives.r1cs-public-input-verification-coverage]

## Phase 2: Validation and archive

- [x] [serial] Run focused public-input tests, Verus over `src/lib.rs`, proof-review scanner, Tracey coverage, Cairn validation/gates, and `TMPDIR=/tmp ./scripts/verify.rs`; save evidence logs. r[rust-primitives.r1cs-public-input-verification-coverage]
- [x] [serial] Sync/archive the change and commit the completed evidence. r[rust-primitives.r1cs-public-input-verification-coverage]
