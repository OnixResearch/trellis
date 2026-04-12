# seL4 ASID Pool Specification

## Purpose

Defines requirements for verified `sel4_asid` semantics in this repo. Scope is address-space identity behavior and preserved invariants, not raw ABI details.

## Requirements

### Requirement: Live ASIDs Are Unique

r[sel4-asid.uniqueness]
Each live ASID MUST be bound to at most one live address-space object at a time.

#### Scenario: Distinct allocations differ

- GIVEN two successful live ASID allocations
- WHEN their ASIDs are compared
- THEN the ASIDs differ

### Requirement: Released ASIDs May Be Reused Safely

r[sel4-asid.reuse]
A released ASID MAY be reused later, but only after no live address space still owns it.

#### Scenario: Freed ASID reused later

- GIVEN an ASID that has been fully released
- WHEN allocation later reuses that numeric ASID
- THEN no previously live address space still owns it

### Requirement: ASID Pool Bounds

r[sel4-asid.pool-bounds]
Allocation MUST respect the finite ASID pool capacity and fail or report exhaustion when no free ASIDs remain.

#### Scenario: Exhausted pool rejects allocation

- GIVEN all ASIDs in the pool are live
- WHEN another allocation is attempted
- THEN the allocation fails
