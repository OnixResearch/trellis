## Why

R1CS row and system satisfaction now exist, but statements still cannot bind public inputs to witness slots. Circuit semantics need a small public/private boundary before higher-level gates, range checks, lookups, or proving-system work.

## What Changes

- Add a bounded `r1cs_public_inputs` primitive for public input bindings over witness indices.
- Compose public-input binding with `r1cs_system_satisfied` into statement satisfaction.
- Keep scope finite and semantic: no commitments, transcripts, proof objects, or cryptographic soundness.

## Impact

- **Files**: `src/r1cs_public_inputs.rs`, `src/lib.rs`, README, requirements, rust-primitives spec.
- **APIs**: new bounded structs and checked helpers for public input binding and statement satisfaction.
- **Testing**: focused positive and negative tests plus repo verification evidence.
