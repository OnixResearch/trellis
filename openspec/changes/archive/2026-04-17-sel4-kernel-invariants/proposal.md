## Why

The local seL4 primitives become most valuable when they compose into a kernel-state invariant. This final change gives the repo a place to state the global invariant and prove primitive steps preserve it.

## What Changes

- Add `src/sel4_invariants.rs` with a concrete `KernelState` aggregate over the existing seL4 module types in this repo.
- State named sub-invariants for object references, CDT well-formedness, CSpace child/root validity, TCB legality, ready-queue consistency, untyped ownership/disjointness, ASID pool well-formedness, VSpace well-formedness, and IRQ uniqueness.
- State a composed kernel invariant plus decomposition lemmas over those named obligations.
- Prove an initial preservation set of modeled primitive steps: endpoint send by a running sender on an empty receive queue, endpoint send by a running sender that rendezvous-matches the queued head receiver and wakes it onto the ready queue, and untyped revoke.

## Capabilities

### New Capabilities

- `sel4-invariants.compose`: Verified composition of named kernel sub-invariants
- `sel4-invariants.decomposition`: Verified global invariant decomposes into named local obligations
- `sel4-invariants.step-preservation`: Verified primitive steps preserve the kernel invariant

## Impact

- **Files**: New `src/sel4_invariants.rs` module, `src/lib.rs` export, `docs/requirements.md` entries, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for `KernelState`, named sub-invariants, composition/decomposition, and the initial preservation-step set
- **Dependencies**: Reuses `sel4_asid`, `sel4_cdt`, `sel4_cspace`, `sel4_endpoint`, `sel4_irq`, `sel4_ready_queue`, `sel4_tcb`, `sel4_untyped`, and `sel4_vspace`
- **Testing**: Verus preservation proofs plus focused unit/proptest model checks for the initial primitive-step set

## Out of Scope

- Refinement to generated C
- Hardware correspondence
- Whole-system liveness or timing guarantees
