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

### Requirement: Kernel Preservation Matrix
r[sel4-invariants.preservation-matrix]

The seL4 invariant layer MUST maintain a local preservation matrix for modeled kernel steps. Each matrix entry MUST identify the step name, required prestate facts, mutated state fields, unchanged state fields, consumed sub-invariants, and proof or test evidence that links the entry back to `kernel_invariant_spec`.


#### Scenario: Modeled step has a preservation row

- GIVEN a modeled primitive kernel step is claimed to preserve `kernel_invariant_spec`
- WHEN the preservation matrix is reviewed
- THEN the matrix contains a row naming the step, its preconditions, its mutation footprint, and the sub-invariants used by the proof

#### Scenario: Unchanged domains are framed explicitly

- GIVEN a modeled step mutates only endpoint, TCB, or ready-queue state
- WHEN its preservation proof is reviewed
- THEN unrelated CDT, CSpace, untyped, ASID, VSpace, and IRQ facts are carried by explicit frame reasoning rather than hidden assumptions

#### Scenario: Matrix rejects unsupported claims

- GIVEN a step has no preservation proof or scoped test evidence
- WHEN the preservation matrix is checked
- THEN the step is not listed as a proven invariant-preserving step

### Requirement: Expanded Kernel Step Preservation
r[sel4-invariants.step-preservation-expansion]

The seL4 invariant layer SHOULD expand the modeled preservation set around endpoint, TCB, ready-queue, and untyped interactions. Every accepted step in this expanded set MUST preserve `kernel_invariant_spec` whenever its stated preconditions hold, and rejected or out-of-scope requests MUST preserve the prestate without mutating global kernel state.


#### Scenario: Accepted endpoint transition preserves invariant

- GIVEN a kernel state satisfying `kernel_invariant_spec` and an endpoint/TCB transition whose stated preconditions hold
- WHEN the modeled transition is applied
- THEN the resulting state satisfies `kernel_invariant_spec`

#### Scenario: Rejected transition is no-op

- GIVEN a kernel state satisfying `kernel_invariant_spec` and a modeled transition request with invalid references or mismatched endpoint/TCB state
- WHEN the request is rejected by the wrapper
- THEN the returned state is unchanged and still satisfies `kernel_invariant_spec`

#### Scenario: Untyped preservation remains framed

- GIVEN an untyped operation only changes the modeled untyped subtree allowed by its preconditions
- WHEN preservation is proved
- THEN object-reference, scheduler, CSpace, ASID, VSpace, and IRQ sub-invariants remain available for the composed invariant
