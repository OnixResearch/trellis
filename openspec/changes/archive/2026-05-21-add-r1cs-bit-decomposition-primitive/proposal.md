## Why

Trellis now has bounded R1CS rows, systems, public-input bindings, gate adapters, and booleanity constraints. The next foundational circuit semantic is bit decomposition: relating a small public value to boolean witness slots so later range-check and gadget composition work can reuse a verified boundary.

## What Changes

- Add a bounded `r1cs_bit_decomposition` primitive for a two-bit value decomposition.
- Reuse `r1cs_boolean` satisfaction for both witness bit slots.
- Check deterministic recomposition `value = bit0 + 2 * bit1` over the admitted witness.
- Add positive/negative tests, docs/spec coverage, and archived verification evidence.

## Scope

In scope: finite two-bit shape admission, boolean bit composition, recomposition satisfaction, malformed input rejection, and runtime tests.

Out of scope: arbitrary-width bit vectors, lookup arguments, full range-check gadgets beyond the two-bit consequence, circuit builders, transcripts, commitments, verifier keys, Groth16, PLONK, or cryptographic soundness.

## Verification

Run focused bit-decomposition tests, Verus over `src/lib.rs`, clippy, full cargo tests, Tracey coverage, strict OpenSpec validation, and `TMPDIR=/tmp ./scripts/verify.rs` with logs saved under the change archive.
