## Why

Trellis now has bounded single-row R1CS satisfaction, multi-row system satisfaction, and public-input statement binding. The next reusable circuit layer is a tiny gate adapter that turns readable multiplication/addition/constant gates into the existing row semantics without introducing a proof protocol.

## What Changes

- Add a bounded `r1cs_gates` primitive for gate variable admission and gate-to-row encoding.
- Support multiplication, addition, and constant gates over the existing three-slot witness/R1CS row model.
- Verify that emitted rows are valid and that gate satisfaction agrees with the emitted row checked by `r1cs_row_satisfied`.

## Scope

In scope: finite executable gate adapters, positive/negative tests, documentation, Tracey/OpenSpec coverage, and archived evidence.

Out of scope: general circuit builders, unbounded vectors, boolean/range gadgets, lookup arguments, transcripts, commitments, verifier keys, Groth16, PLONK, or cryptographic soundness.

## Verification

Run focused gate tests, Verus over `src/lib.rs`, clippy, full cargo tests, Tracey coverage, strict OpenSpec validation, and `TMPDIR=/tmp ./scripts/verify.rs` with evidence saved under the change archive.
