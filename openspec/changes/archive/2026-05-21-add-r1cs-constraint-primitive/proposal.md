## Why

Trellis has many verified state and domain primitives but no first circuit/ZK-style arithmetic constraint primitive. A single-row R1CS checker is the smallest useful circuit foundation because Boolean gates, arithmetic gates, range checks, and later full constraint systems can compile down to row satisfaction.

## What Changes

- Add `r1cs_constraint`, a bounded finite R1CS row-satisfaction primitive.
- Model one row `(A, B, C)` and one finite witness vector with explicit row/witness lengths.
- Reject malformed rows, malformed witnesses, length mismatches, and unsatisfied equations.

## Out of Scope

- Multi-row constraint systems, variable allocation, gate builders, proving systems, elliptic curves, pairings, transcript/Fiat-Shamir, PLONK, Groth16, and cryptographic soundness claims.
- Arbitrary-precision field arithmetic; this slice uses bounded field elements that keep executable arithmetic overflow-free.

## Impact

- **Files**: `src/r1cs_constraint.rs`, `src/lib.rs`, `README.md`, `docs/requirements.md`, and rust-primitives OpenSpec docs.
- **APIs**: New public finite row/witness structs plus row validation, witness validation, dot product, and satisfaction helpers.
- **Verification**: Focused unit tests, Verus proof checks, Tracey requirement coverage, OpenSpec validation, and the maintained repo verify rail.
