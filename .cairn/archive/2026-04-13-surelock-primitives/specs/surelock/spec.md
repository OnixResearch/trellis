# Surelock Core Specification

## Purpose

Defines requirements for a machine-checked core model of surelock's
deadlock-freedom primitives: stable lock identity, total level ordering,
consumed-and-re-emitted scope keys, canonical same-level acquisition, and the
absence of circular wait.

This spec covers the semantic core described by the upstream surelock README,
design notes, compile-fail examples, and the Surelock blog post. It does not
attempt to reproduce the full external API surface.

## Requirements

### Lock Identity

#### Requirement: Unique Lock IDs

Every lock MUST have a unique identity. Two distinct locks created by the
model MUST never share the same lock ID.

##### Scenario: Distinct creation yields distinct IDs

- GIVEN two locks created by successive allocation steps
- WHEN their lock IDs are compared
- THEN the IDs MUST differ

#### Requirement: Monotonic Lock IDs

Lock IDs MUST increase with creation order. If lock `b` is created after lock
`a`, then `id(a) < id(b)`.

##### Scenario: Creation order induces ordering

- GIVEN locks `a`, `b`, and `c` created in that order
- WHEN their IDs are examined
- THEN `id(a) < id(b) < id(c)` MUST hold

#### Requirement: Stable Identity Under Movement

A lock's identity MUST be attached to the lock itself, not to a temporary
position in a collection. Reordering a list of locks MUST NOT change any lock
ID.

##### Scenario: Reordering preserves IDs

- GIVEN a sequence of locks with assigned IDs
- WHEN the sequence is permuted
- THEN each lock MUST retain its original ID

### Level Ordering

#### Requirement: Strict Total Level Order

Levels MUST form a strict total order. For any two levels `a` and `b`, exactly
one of `a < b`, `a = b`, or `b < a` MUST hold.

##### Scenario: Comparable levels

- GIVEN levels `Level<1>` and `Level<3>`
- WHEN they are compared
- THEN `Level<1>` MUST be lower than `Level<3>`

##### Scenario: Equality case

- GIVEN two references to `Level<2>`
- WHEN they are compared
- THEN they MUST compare equal

#### Requirement: Lock-After Relation

The lock-after relation MUST agree with the level order. A lock group at level
`next` may be acquired after a key at level `current` exactly when
`current <= next` in the model's ordering policy.

##### Scenario: Upward acquisition allowed

- GIVEN a key at level `Level<1>`
- WHEN attempting to acquire a lock at `Level<2>`
- THEN the acquisition MUST be admitted

##### Scenario: Backward acquisition rejected

- GIVEN a key at level `Level<3>`
- WHEN attempting to acquire a lock at `Level<1>`
- THEN the acquisition MUST be rejected

#### Requirement: Derived Higher Levels

`new_higher`-style construction MUST derive the next level from existing
parents. A single parent yields `parent + 1`. Two parents yield
`max(parent_a, parent_b) + 1`.

##### Scenario: Single parent derivation

- GIVEN a parent lock at level `Level<2>`
- WHEN deriving a higher child
- THEN the child MUST be at `Level<3>`

##### Scenario: Two-parent derivation

- GIVEN parent locks at levels `Level<1>` and `Level<4>`
- WHEN deriving a higher child
- THEN the child MUST be at `Level<5>`

### Scope Keys

#### Requirement: Key Progression

Acquiring a lock or lock group MUST consume the current key and return a new
key whose level is the maximum level reached by that acquisition.

##### Scenario: Single-lock progression

- GIVEN a key at `Level<0>` and a lock at `Level<2>`
- WHEN the lock is acquired
- THEN the returned key MUST be at `Level<2>`

##### Scenario: Group progression

- GIVEN a key at `Level<1>` and an admitted lock group whose maximum level is `Level<3>`
- WHEN the group is acquired
- THEN the returned key MUST be at `Level<3>`

#### Requirement: Subscope Preservation

Entering and leaving a subscope MUST preserve the current key level. A subscope
may use the inherited key, but the outer scope resumes at the same level.

##### Scenario: Subscope returns same level

- GIVEN a key at `Level<2>`
- WHEN a subscope is entered and then exited
- THEN the outer scope MUST receive a key at `Level<2>`

#### Requirement: No Backward Progression

A key MUST never move to a lower level after an admitted acquisition.

##### Scenario: Acquisitions are nondecreasing

- GIVEN a sequence of admitted acquisitions starting at `Level<0>`
- WHEN the returned key levels are collected
- THEN the sequence of key levels MUST be nondecreasing

### Lock Sets

#### Requirement: Canonical Same-Level Ordering

A same-level lock set MUST compute a canonical acquisition plan by sorting its
member locks by lock ID.

##### Scenario: Reversed input yields same acquisition plan

- GIVEN the same two same-level locks presented in opposite orders
- WHEN each lock set computes its acquisition plan
- THEN both plans MUST contain the same locks in the same ID-sorted order

#### Requirement: Duplicate Rejection

A lock set MUST reject duplicate locks. The same lock ID may not appear twice
in one acquisition group.

##### Scenario: Duplicate member fails

- GIVEN a candidate lock set containing the same lock twice
- WHEN the lock set is prepared
- THEN preparation MUST fail

#### Requirement: Membership Preservation

Canonicalization MUST preserve the logical contents of the lock set. Sorting
changes acquisition order only; it MUST NOT add, drop, or replace locks.

##### Scenario: Same members after sorting

- GIVEN a same-level lock set with distinct member locks
- WHEN the canonical acquisition plan is produced
- THEN the multiset of lock IDs in the plan MUST equal the multiset of input lock IDs

#### Requirement: Caller Order Preservation

The model MUST distinguish between internal acquisition order and the caller's
logical lock list. Internal sorting MAY reorder acquisition, but the logical
result mapping MUST still refer to the caller's original lock positions.

##### Scenario: Returned positions match caller order

- GIVEN two distinct same-level locks passed as `(b, a)` with `id(a) < id(b)`
- WHEN the lock set is acquired through its canonical plan
- THEN the logical result for the first position MUST still correspond to `b`
- AND the logical result for the second position MUST still correspond to `a`

### Deadlock Freedom

#### Requirement: Forward Wait Rank

Every admitted wait edge in the core model MUST move strictly forward in the
lexicographic rank `(level, id)`, where `id` breaks ties only within a level.

##### Scenario: Same-level wait moves by ID

- GIVEN two same-level locks with distinct IDs
- WHEN canonical acquisition waits from the lower-ID lock to the higher-ID lock
- THEN the wait edge MUST increase rank

##### Scenario: Cross-level wait moves by level

- GIVEN locks at different levels acquired through admitted key progression
- WHEN a later acquisition depends on an earlier one
- THEN the wait edge MUST increase rank by level

#### Requirement: No Circular Wait

A wait-for graph induced by admitted acquisitions MUST be acyclic.

##### Scenario: Same-level sibling locking is acyclic

- GIVEN multiple threads acquiring the same sibling lock set through canonical ordering
- WHEN the induced wait-for edges are collected
- THEN the graph MUST contain no cycle

##### Scenario: Upward-only cross-level locking is acyclic

- GIVEN threads that only perform admitted upward acquisitions across levels
- WHEN the induced wait-for edges are collected
- THEN the graph MUST contain no cycle

#### Requirement: Deadlock-Freedom of Admitted Traces

Any acquisition trace admitted by the core surelock model MUST be deadlock-free
with respect to circular wait.

##### Scenario: Admitted trace cannot deadlock

- GIVEN an execution trace that uses only admitted keys, levels, and canonical lock sets
- WHEN the trace is interpreted as a wait-for graph
- THEN no deadlock cycle MUST exist
