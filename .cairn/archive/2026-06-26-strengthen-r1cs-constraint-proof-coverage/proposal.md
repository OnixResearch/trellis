## Why

The proof-review scanner found that the base R1CS row module still has two catalogue proofs with only `ensures true`. These proofs satisfy Tracey mechanically but do not state the concrete row/dot-product behaviours reviewers expect from the finite primitive.

## What Changes

- Replace vacuous row and dot-product coverage proofs in `src/r1cs_constraint.rs` with explicit postconditions for valid lengths, malformed row/witness rejection, length mismatch rejection, and satisfied-row decomposition.
- Add focused positive and negative runtime tests for missing finite boundary cases if needed by the coverage contract.
- Keep the trusted-boundary inventory unchanged; this is proof/test coverage hardening only.
- Save focused validation evidence before archive.

## Impact

- **Files**: `src/r1cs_constraint.rs`, Cairn change artifacts, synced requirement text, and archived evidence.
- **Testing**: focused `r1cs_constraint` cargo tests, Verus over `src/lib.rs`, proof-review scanner, Tracey direct queries, Cairn validation/gates, and `TMPDIR=/tmp ./scripts/verify.rs` before archive.
