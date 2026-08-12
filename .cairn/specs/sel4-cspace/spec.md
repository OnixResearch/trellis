# seL4 CSpace Lookup Specification

## Purpose

Defines requirements for verified `sel4_cspace` semantics in this repo. Scope is capability-space traversal behavior and preserved invariants, not raw ABI details.

## Requirements

### Requirement: Guarded CSpace Lookup
r[sel4-cspace.lookup]

CSpace lookup MUST resolve a slot by guarded radix traversal. Each level MUST check the guard, consume the matching guard depth plus one slot-selection segment, and select exactly one child slot within the current CNode.

#### Scenario: Lookup succeeds when guard and radix match

- GIVEN a root CNode and a path matching one child slot
- WHEN lookup runs with sufficient depth
- THEN the result is that slot

#### Scenario: Guard mismatch fails

- GIVEN a CNode whose guard does not match the next path bits
- WHEN lookup runs
- THEN the lookup fails

### Requirement: Unique In-Bounds Slot Resolution
r[sel4-cspace.unique-slot]

A successful lookup MUST identify exactly one slot, and that slot MUST be within the bounds of the resolved CNode.

#### Scenario: One path, one slot

- GIVEN one lookup context and one path
- WHEN lookup succeeds
- THEN no different slot index is also a valid result

### Requirement: Lookup Failure Is Non-Mutating
r[sel4-cspace.failure-no-mutation]

A failed lookup MUST NOT modify CSpace state, consumed-depth accounting, or capability contents.

#### Scenario: Insufficient depth leaves state unchanged

- GIVEN a path shorter than the required lookup depth
- WHEN lookup fails
- THEN the CSpace state is unchanged
