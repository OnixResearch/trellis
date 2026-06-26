# seL4 Kernel Invariant Specification

## Purpose

Defines requirements for verified `sel4_invariants` semantics in this repo. Scope is global kernel-state soundness behavior and preserved invariants, not raw ABI details.

## Requirements

### Requirement: Kernel Invariants Compose

r[sel4-invariants.compose]
The kernel state MUST satisfy a composed invariant covering at least object-reference validity, capability derivation-tree well-formedness, CSpace root/slot validity, TCB state legality, ready-queue consistency, untyped-child region ownership, ASID pool well-formedness, VSpace mapping consistency, and IRQ binding uniqueness.

#### Scenario: Well-formed kernel state

- GIVEN a kernel state satisfying all local module invariants
- WHEN the composed invariant is checked
- THEN the kernel state satisfies the global invariant

### Requirement: Kernel Invariants Decompose Into Named Obligations

r[sel4-invariants.decomposition]
The composed kernel invariant MUST be equivalent to a conjunction of named sub-invariants so local proofs can depend on only the parts they use. The named obligations for this change are object-reference validity, CDT well-formedness, CSpace validity, TCB legality, scheduler consistency, untyped ownership/disjointness, ASID pool well-formedness, VSpace consistency, and IRQ uniqueness.

#### Scenario: Local reasoning uses one sub-invariant

- GIVEN a proof that requires only ready-queue consistency
- WHEN the composed invariant holds
- THEN that ready-queue sub-invariant is available

### Requirement: Primitive Steps Preserve Kernel Invariants

r[sel4-invariants.step-preservation]
Each modeled primitive kernel step in this change's initial preservation set MUST preserve the composed kernel invariant whenever its preconditions hold. The initial set is endpoint send on an empty receive queue, endpoint send rendezvous with a queued receiver, and untyped revoke. In this repo's starter surface, both endpoint-send wrappers operate on a running sender TCB; the rendezvous wrapper additionally requires the head queued receiver TCB to be blocked on that endpoint and to carry a priority that fits the ready-queue array.

#### Scenario: Endpoint send preserves invariant

- GIVEN a kernel state satisfying the composed invariant
- WHEN an endpoint send step executes with satisfied preconditions
- THEN the resulting kernel state also satisfies the composed invariant

#### Scenario: Untyped revoke preserves invariant

- GIVEN a kernel state satisfying the composed invariant
- WHEN an untyped revoke step executes with satisfied preconditions
- THEN the resulting kernel state also satisfies the composed invariant
