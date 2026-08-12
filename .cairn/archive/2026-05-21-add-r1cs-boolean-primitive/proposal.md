## Why

Trellis now has bounded R1CS rows, systems, public-input bindings, and readable gate-to-row adapters. The next foundational circuit semantic is booleanity: constraining a witness slot to be either 0 or 1 before later bit-decomposition and range-check primitives.

## What Changes

- Add a bounded `r1cs_boolean` primitive that admits a boolean witness index and modulus.
- Encode booleanity as one R1CS row using `b * (1 - b) = 0` over the existing row checker.
- Require the existing constant-one convention (`w0 == 1`) and a nonzero boolean slot index.
- Add positive/negative tests, docs/spec coverage, and archived verification evidence.

## Scope

In scope: finite boolean-index admission, row encoding, satisfaction checking, malformed input rejection, and runtime tests.

Out of scope: bit decomposition, range checks, lookup arguments, full circuit builders, transcripts, commitments, verifier keys, Groth16, PLONK, or cryptographic soundness.

## Verification

Run focused boolean tests, Verus over `src/lib.rs`, clippy, full cargo tests, Tracey coverage, strict OpenSpec validation, and `TMPDIR=/tmp ./scripts/verify.rs` with logs saved under the change archive.
