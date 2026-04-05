# Inclusive/Exclusive Cache Hierarchy Specification

## Purpose

Defines requirements for verified multi-level cache hierarchy invariants
under inclusive, exclusive, and non-inclusive policies. Covers inductive
invariant preservation, back-invalidation, atomic swaps, capacity
accounting, and coherence snoop scope.

## Requirements

### Requirement: Inclusive Invariant

Under inclusive policy, every line in L1 MUST also be in L2.

#### Scenario: Initial state

- GIVEN both L1 and L2 are empty
- WHEN the inclusive invariant is checked
- THEN it holds (empty set is subset of any set)

#### Scenario: After L1 fill

- GIVEN the inclusive invariant holds and a line is filled into L1
- WHEN L2 is inspected
- THEN L2 also contains the line (it was added to L2 as part of the fill)

#### Scenario: After L1 evict

- GIVEN the inclusive invariant holds and a line is evicted from L1
- WHEN L2 is inspected
- THEN L2 may or may not contain the line (L1 ⊆ L2 still holds)

### Requirement: Inclusive Back-Invalidation

Under inclusive policy, evicting a line from L2 MUST also invalidate it
in L1 if present.

#### Scenario: L2 eviction with L1 copy

- GIVEN line L is in both L1 and L2 under inclusive policy
- WHEN L is evicted from L2
- THEN L is also removed from L1

#### Scenario: L2 eviction without L1 copy

- GIVEN line L is in L2 but not L1
- WHEN L is evicted from L2
- THEN L1 is unchanged

#### Scenario: Invariant preserved

- GIVEN the inclusive invariant holds before L2 eviction
- WHEN L2 eviction with back-invalidation completes
- THEN the inclusive invariant still holds

### Requirement: Exclusive Invariant

Under exclusive policy, a line MUST exist in at most one cache level.

#### Scenario: L1 and L2 disjoint

- GIVEN any state under exclusive policy
- WHEN L1 and L2 are inspected
- THEN no line appears in both

#### Scenario: After L2-to-L1 promotion

- GIVEN line L is in L2 under exclusive policy
- WHEN L is promoted to L1
- THEN L is in L1 and not in L2

#### Scenario: After L1 eviction

- GIVEN line L is in L1 under exclusive policy
- WHEN L is evicted from L1
- THEN L is moved to L2 (victim cache) and no longer in L1

### Requirement: Exclusive Swap Atomicity

Moving a line between levels under exclusive policy MUST be atomic — no
intermediate state where the line is in both or neither level.

#### Scenario: Promotion

- GIVEN line L is in L2 only
- WHEN L is promoted to L1
- THEN the resulting state has L in L1 only (no transient both-levels state)

#### Scenario: Demotion

- GIVEN line L is in L1 only
- WHEN L is demoted to L2
- THEN the resulting state has L in L2 only

### Requirement: Non-Inclusive Freedom

Under non-inclusive policy, there MUST be no structural constraint between
L1 and L2 contents.

#### Scenario: L1 line not in L2

- GIVEN non-inclusive policy
- WHEN a line is in L1 but not L2
- THEN this is a valid state

#### Scenario: L2 line not in L1

- GIVEN non-inclusive policy
- WHEN a line is in L2 but not L1
- THEN this is a valid state

#### Scenario: Line in both levels

- GIVEN non-inclusive policy
- WHEN a line is in both L1 and L2
- THEN this is a valid state

### Requirement: Capacity Accounting

The effective cache capacity MUST be correctly computed for each policy.

#### Scenario: Inclusive capacity

- GIVEN L1 with capacity C1 and L2 with capacity C2 under inclusive policy
- WHEN the effective capacity is computed
- THEN it equals C2 (L1 is a subset of L2, adds no unique lines)

#### Scenario: Exclusive capacity

- GIVEN L1 with capacity C1 and L2 with capacity C2 under exclusive policy
- WHEN the effective capacity is computed
- THEN it equals C1 + C2 (no overlap)

### Requirement: Coherence Snoop Scope

Under inclusive policy, snooping L2 MUST be sufficient to determine
whether any level holds a line.

#### Scenario: L2 miss implies L1 miss

- GIVEN inclusive policy and a snoop for line L
- WHEN L is not found in L2
- THEN L is guaranteed not in L1

#### Scenario: Exclusive requires both levels

- GIVEN exclusive policy and a snoop for line L
- WHEN L is not found in L2
- THEN L MAY still be in L1 (both levels must be checked)
