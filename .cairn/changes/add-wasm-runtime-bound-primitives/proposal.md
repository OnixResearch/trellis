## Why

Trellis already verifies a bounded WebAssembly MVP semantic slice covering typed values, operand/local discipline, linear-memory bounds and growth, and structured control validation. SpaceWasm highlights additional runtime-control laws needed by downstream deterministic systems: exact fuel accounting, bounded pause/resume segmentation, initialization-only capped allocation, fail-closed allocation errors, exact streaming chunk coverage, and explicit `memory.grow` policy.

Those laws are useful to ChaosControl and Molten, but importing SpaceWasm source or restating its requirements would not prove its decoder, interpreter, allocator, unsafe containers, or flight suitability. Trellis should add implementation-independent verified primitives that downstream adapters can instantiate and revalidate against their own runtime observations.

## What Changes

- Add a finite abstract runtime-step model with explicit running, finished, trapped, host-paused, and out-of-fuel outcomes.
- Define and verify exact non-underflowing fuel consumption and no-state-change failure behavior.
- Define and verify bounded execution segmentation laws: under a deterministic step relation, admitted segmented execution matches uninterrupted execution for the same step budget and host observations.
- Add generation/profile/input-bound continuation admission and stale-continuation rejection primitives.
- Add an initialization/running/finished allocation-phase model with capped monotone page allocation, explicit OOM, and no execution-phase allocation.
- Add exact streaming chunk-schedule validation proving ordered coverage without omission, overlap, reordering, or trailing bytes.
- Compose the existing Wasm memory-growth primitive with an explicit allow/deny profile whose denied path leaves memory unchanged.
- Add positive and negative tests, Verus proofs, Tracey links, proof evidence, and explicit non-claims.

## Impact

- **Surfaces**: the accepted `wasm` domain, new verified runtime-bound data types/specs/exec helpers/proofs, public exports, requirements, tests, proof evidence, and module catalog.
- **Consumers**: ChaosControl may reuse comparison/segmentation predicates; Molten may reuse continuation/resource admission laws; Octet may cite static policy vocabulary. Each consumer must map and revalidate its concrete runtime facts.
- **Trust boundary**: no SpaceWasm source is vendored or verified, and no external runtime call occurs in Trellis.
- **Claims**: proofs apply only to the finite abstract models and explicit preconditions. They do not prove WebAssembly conformance, SpaceWasm behavior, unsafe memory safety, host scheduling, serialization, migration, sandboxing, or certification.
