## Why

The single-row R1CS primitive is useful, but real circuit semantics require checking a finite set of rows against one shared witness. A bounded multi-row system is the next smallest reusable layer before adding gate encodings, range checks, lookups, or proving systems.

## What Changes

- Add `r1cs_system`, a finite bounded R1CS system over up to three existing `R1csRow` values.
- Require every active row to be well-shaped, share the system modulus, and be satisfied by the same admitted witness.
- Add positive and negative tests for satisfied systems, failed rows, modulus mismatch, and malformed witnesses.

## Non-Goals

- No Groth16, PLONK, commitments, transcripts, cryptographic soundness, or witness generation.
- No dynamic-length vectors beyond the existing finite Trellis primitive style.

## Verification

Run focused tests for `r1cs_system`, Verus over `src/lib.rs`, clippy, full cargo tests, Tracey coverage queries, `./scripts/verify.rs`, and OpenSpec validation.
