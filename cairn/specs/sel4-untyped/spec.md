# seL4 Untyped Memory Specification

## Purpose

Defines requirements for verified `sel4_untyped` semantics in this repo. Scope is kernel object memory behavior and preserved invariants, not raw ABI details.

## Requirements

### Requirement: Untyped Retype Conserves Memory

r[sel4-untyped.retype]
Retyping an untyped region into typed kernel objects MUST conserve address space. Each created child region MUST lie wholly within the source untyped region.

#### Scenario: Aligned children fit within source

- GIVEN an untyped region large enough for two aligned endpoint objects
- WHEN it is retyped into those objects
- THEN both child regions are contained within the source region

### Requirement: Untyped Child Layout Is Aligned And Disjoint

r[sel4-untyped.child-layout]
Live child objects derived from the same untyped source MUST satisfy their required alignment and MUST NOT overlap one another.

#### Scenario: Overlapping child layout rejected

- GIVEN a requested child layout that would overlap an existing live child
- WHEN retype is attempted
- THEN the operation is invalid

### Requirement: Untyped Reclaim Restores Parent Coverage

r[sel4-untyped.reclaim]
Revoking or resetting an untyped object MUST remove or invalidate all typed children derived from it and return ownership of the covered region to the parent untyped object.

#### Scenario: Revoke removes typed children

- GIVEN an untyped region with live frame and endpoint children
- WHEN the untyped object is revoked or reset
- THEN the parent regains that covered region
