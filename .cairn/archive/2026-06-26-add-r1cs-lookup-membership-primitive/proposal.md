## Why

Trellis now has bounded R1CS rows, systems, public-input bindings, gate adapters, booleanity, and two-bit decomposition. The next high-ROI circuit semantic is finite lookup-table membership: proving a small value is one of a bounded table of admitted constants without introducing a proving-system lookup argument.

## What Changes

- Add a bounded `r1cs_lookup` primitive for membership in a fixed three-entry table.
- Reuse the existing bounded R1CS witness and element admission surfaces.
- Check deterministic membership over active table entries while rejecting malformed table shapes.
- Add positive/negative tests, docs/spec coverage, and archived verification evidence.

## Scope

In scope: finite three-entry table shape admission, active-prefix table membership, witness-slot value extraction, malformed input rejection, and runtime tests.

Out of scope: PLONK lookup arguments, commitments, transcripts, table hashing, arbitrary-size vectors, circuit builders, verifier keys, Groth16, PLONK, or cryptographic soundness.

## Verification

Run focused lookup tests, Verus over `src/lib.rs`, clippy, full cargo tests, Tracey coverage, Cairn validation, and `TMPDIR=/tmp ./scripts/verify.rs` with logs saved under the change evidence directory.
