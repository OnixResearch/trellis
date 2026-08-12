## Phase 1: Spec and Skeleton

- [x] [serial] Add docs/requirements anchors and OpenSpec requirement coverage for `r[rust-primitives.r1cs-row-shape]`, `r[rust-primitives.r1cs-dot-product]`, `r[rust-primitives.r1cs-row-satisfaction]`, and `r[rust-primitives.r1cs-verification-coverage]`.
- [x] [depends:rust-primitives.r1cs-row-shape] Add `src/r1cs_constraint.rs`, `src/lib.rs` export, and README catalog entry.

## Phase 2: Implementation

- [x] [depends:rust-primitives.r1cs-row-shape] Implement finite row/witness validity helpers with executable/spec refinement and negative malformed-shape tests.
- [x] [depends:rust-primitives.r1cs-dot-product] Implement bounded modular dot-product helper with proof bridge and positive runtime evidence.
- [x] [depends:rust-primitives.r1cs-row-satisfaction] Implement one-row R1CS satisfaction helper with positive/negative tests for satisfied row, unsatisfied row, and length mismatch.

## Phase 3: Validation and Archive

- [x] [serial] Run focused tests, rustfmt, clippy, Verus, Tracey coverage, OpenSpec validation, and maintained full verification; save transcripts under `evidence/`.
- [x] [serial] Sync rust-primitives spec, archive the completed change, and commit the implementation plus evidence.
