## ADDED Requirements

### Requirement: Kernel Preservation Matrix

The seL4 invariant layer MUST maintain a local preservation matrix for modeled kernel steps. Each matrix entry MUST identify the step name, required prestate facts, mutated state fields, unchanged state fields, consumed sub-invariants, and proof or test evidence that links the entry back to `kernel_invariant_spec`.

r[sel4-invariants.preservation-matrix]

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

The seL4 invariant layer SHOULD expand the modeled preservation set around endpoint, TCB, ready-queue, and untyped interactions. Every accepted step in this expanded set MUST preserve `kernel_invariant_spec` whenever its stated preconditions hold, and rejected or out-of-scope requests MUST preserve the prestate without mutating global kernel state.

r[sel4-invariants.step-preservation-expansion]

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
