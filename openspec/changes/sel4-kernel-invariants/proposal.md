## Why

The local seL4 primitives become most valuable when they compose into a kernel-state invariant. This final change gives the repo a place to state the global invariant and prove primitive steps preserve it.

## What Changes

- Add a composed kernel invariant over object typing, capability ancestry, scheduler state, untyped ownership, ASIDs, mappings, and IRQ ownership.
- State named sub-invariants and a composition theorem.
- Prove modeled primitive steps preserve the composed invariant.

## Capabilities

### New Capabilities

- `sel4-invariants.compose`: Verified composition of named kernel sub-invariants
- `sel4-invariants.decomposition`: Verified global invariant decomposes into named local obligations
- `sel4-invariants.step-preservation`: Verified primitive steps preserve the kernel invariant

## Impact

- **Files**: New `src/sel4_invariants.rs` module, `src/lib.rs` export, `src/properties.rs` cross-module theorems as needed, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for global kernel-state soundness
- **Dependencies**: Reuses all other seL4 primitive modules created by this split
- **Testing**: Verus preservation proofs plus focused unit/proptest model checks for representative primitive transitions

## Out of Scope

- Refinement to generated C
- Hardware correspondence
- Whole-system liveness or timing guarantees
