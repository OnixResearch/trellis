## Context

The accepted seL4 invariant spec names a composed kernel invariant and an initial preservation set. The implementation already factors local helpers for endpoint sender/receiver queues, TCB legal states, ready-queue consistency, CDT/CSpace/VSpace well-formedness, untyped ownership, ASID pool well-formedness, and IRQ uniqueness.

## Goals

- Make each modeled preservation proof state its preconditions, mutation footprint, and preserved sub-invariants.
- Keep proof obligations local: unchanged sub-invariants should be carried by frame lemmas instead of reproving the full composed invariant.
- Add rejected/no-op boundaries so invalid step inputs cannot silently mutate kernel state.
- Preserve existing public semantics for the modeled primitives.

## Non-Goals

- Do not attempt a full kernel transition system.
- Do not introduce new `external_body` or `assume_specification` bridges.
- Do not require runtime mocks for proof-only invariant facts.

## Decisions

### Preservation matrix

The change SHALL add a local matrix, in docs or code comments, that names each modeled step, required prestate facts, mutated fields, unchanged fields, and the sub-invariants consumed by its proof. This matrix is review evidence, not a runtime dispatch table.

### Frame lemmas

Proof work SHOULD use small frame lemmas for unchanged object collections, queue counts, legal TCB states, endpoint correspondence, and disjoint untyped regions. These lemmas form the functional core of the proof; wrapper proofs compose them into `kernel_invariant_spec` preservation.

### Reject boundaries

Invalid or out-of-scope step requests SHALL be modeled as rejected/no-op transitions where appropriate. Tests MUST include negative cases for missing preconditions, invalid references, and mismatched endpoint/TCB state.

### Validation scope

The first implementation slice MAY strengthen the existing endpoint-send, rendezvous, and untyped-revoke proofs before adding additional primitive steps. If extra steps are added, they MUST appear in the matrix and carry matching proof/test evidence.
