# seL4 ASID Pool Specification

## Purpose

Defines requirements for verified `sel4_asid` semantics in this repo. Scope is one abstract finite ASID pool instance, not raw ABI details.

For this change:
- an **owner** is an opaque address-space object ID,
- an ASID is **live** exactly when its pool slot currently stores `Some(owner)`,
- a **valid release** is an exact `(asid, owner)` match that clears that slot, and
- one owner may hold multiple live ASIDs; only ASID -> owner uniqueness is constrained here.

Invalid release attempts — out-of-range ASIDs, free ASIDs, owner mismatch, or duplicate release — return failure and leave the pool unchanged.

## Requirements

### Requirement: Allocation Creates Unique Live Ownership
r[sel4-asid.uniqueness]

A successful allocation MUST make the returned ASID live for the allocating owner, change only that returned slot, and each live ASID in the abstract pool MUST be bound to at most one live owner at a time.

#### Scenario: Successful allocation creates live ownership

- GIVEN a free ASID slot in the abstract pool
- WHEN allocation for an owner succeeds
- THEN the returned ASID becomes live for that owner

#### Scenario: Distinct allocations differ

- GIVEN one abstract pool with two free ASIDs
- WHEN two allocations both succeed
- THEN the returned ASIDs differ

#### Scenario: Allocation preserves unrelated live ownership

- GIVEN a live ASID owned by some owner and a different free ASID
- WHEN allocation succeeds for another owner
- THEN the original live ASID keeps its owner

### Requirement: Release Clears Live Ownership
r[sel4-asid.release-state]

A valid release MUST return success, clear only the released ASID's live ownership, and leave every unrelated live binding unchanged. An invalid release MUST return failure and leave the pool unchanged.

#### Scenario: Valid release clears owner

- GIVEN a live ASID owned by an address-space object ID
- WHEN that exact owner releases the ASID
- THEN the release succeeds and the ASID is no longer live

#### Scenario: Invalid release leaves state unchanged

- GIVEN an ASID that is free, out of range, or owned by a different address-space object ID
- WHEN release is attempted
- THEN the release fails and pool ownership is unchanged

#### Scenario: Duplicate release fails

- GIVEN an owner has already validly released an ASID
- WHEN the same `(asid, owner)` release is attempted again
- THEN the release fails and pool ownership is unchanged

#### Scenario: Valid release preserves unrelated live ownership

- GIVEN two distinct live ASIDs
- WHEN one owner validly releases exactly one of them
- THEN the other live ASID keeps its owner

### Requirement: Released ASIDs May Be Reused Safely
r[sel4-asid.reuse]

A released ASID MAY be reused later, but only after no live owner still owns that numeric ASID.

#### Scenario: Freed ASID reused later

- GIVEN an ASID whose valid release has already cleared ownership
- WHEN allocation later reuses that numeric ASID
- THEN no previously live owner still owns it

### Requirement: ASID Pool Bounds
r[sel4-asid.pool-bounds]

Allocation MUST succeed when some free ASID exists, return only in-range ASIDs on success, and report exhaustion exactly when no free ASIDs remain. Failed allocation MUST leave pool ownership unchanged.

#### Scenario: Free slot allocates

- GIVEN a pool with at least one free ASID
- WHEN allocation is attempted
- THEN it succeeds

#### Scenario: Successful allocation stays in range

- GIVEN a pool with capacity `n > 0`
- WHEN allocation succeeds
- THEN the returned ASID is in `0..n-1`

#### Scenario: Exhausted pool rejects allocation

- GIVEN all ASIDs in the pool are live
- WHEN another allocation is attempted
- THEN the allocation fails and pool ownership is unchanged

#### Scenario: Zero-capacity pool is immediately exhausted

- GIVEN a pool with capacity `0`
- WHEN allocation is attempted
- THEN the allocation fails and pool ownership is unchanged
