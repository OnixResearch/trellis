# Cache Hierarchy Specification

## Purpose

Defines requirements for verified two-level cache hierarchy behavior in this repo. Scope is line presence and policy-preserving transitions for inclusive, exclusive, and non-inclusive L1/L2 hierarchies.

## Requirements

### Requirement: Inclusive Invariant Checker

r[hierarchy.inclusive-invariant]
`check_inclusive_invariant` MUST return true if and only if every line present in L1 is also present in L2.

#### Scenario: Inclusive hit in both levels

- GIVEN a hierarchy where line `L` is present in both L1 and L2
- WHEN `check_inclusive_invariant` is called
- THEN the result is true for that line relationship

#### Scenario: Missing L2 copy rejects inclusive state

- GIVEN a hierarchy where line `L` is present in L1 and absent from L2
- WHEN `check_inclusive_invariant` is called
- THEN the result is false

#### Scenario: Inclusive `l1_fill` installs both levels

- GIVEN an Inclusive hierarchy where line `L` is absent from both levels
- WHEN `l1_fill(L)` is applied
- THEN `L` is present in both L1 and L2

#### Scenario: Inclusive `l1_evict` removes only L1 copy

- GIVEN an Inclusive hierarchy where line `L` is present in both levels
- WHEN `l1_evict(L)` is applied
- THEN `L` is absent from L1 and unchanged in L2

#### Scenario: Inclusive `l2_fill` extends only lower level

- GIVEN an Inclusive hierarchy where line `L` is absent from both levels
- WHEN `l2_fill(L)` is applied
- THEN `L` is present in L2 and L1 is unchanged

### Requirement: Inclusive L2 Eviction Back-Invalidates L1

r[hierarchy.back-invalidation]
Under the Inclusive policy, `l2_evict` MUST remove the evicted line from L2 and MUST also remove it from L1.

#### Scenario: Inclusive back-invalidation

- GIVEN an Inclusive hierarchy with line `L` present in both L1 and L2
- WHEN `l2_evict(L)` is applied
- THEN `L` is absent from both levels

### Requirement: Exclusive Invariant Checker

r[hierarchy.exclusive-invariant]
`check_exclusive_invariant` MUST return true if and only if no line is simultaneously present in both L1 and L2.

#### Scenario: Exclusive split ownership

- GIVEN a hierarchy where `A` is only in L1 and `B` is only in L2
- WHEN `check_exclusive_invariant` is called
- THEN the result is true

#### Scenario: Shared line rejects exclusive state

- GIVEN a hierarchy where line `L` is present in both L1 and L2
- WHEN `check_exclusive_invariant` is called
- THEN the result is false

#### Scenario: Exclusive `l1_fill` removes lower-level copy

- GIVEN an Exclusive hierarchy with line `L` present in L2 and absent from L1
- WHEN `l1_fill(L)` is applied
- THEN `L` is present in L1 and absent from L2

#### Scenario: Exclusive `l1_evict` demotes into L2

- GIVEN an Exclusive hierarchy with line `L` present in L1 and absent from L2
- WHEN `l1_evict(L)` is applied
- THEN `L` is absent from L1 and present in L2

#### Scenario: Exclusive `l2_fill` removes upper-level copy

- GIVEN an Exclusive hierarchy with line `L` present in L1 and absent from L2
- WHEN `l2_fill(L)` is applied
- THEN `L` is absent from L1 and present in L2

### Requirement: Exclusive Promotion Removes Lower-Level Copy

r[hierarchy.exclusive-swap]
Under the Exclusive policy, `l2_to_l1_promote` MUST move a present line from L2 to L1 without leaving a duplicate copy in L2.

#### Scenario: Exclusive promote swaps ownership

- GIVEN an Exclusive hierarchy with line `L` present in L2 and absent from L1
- WHEN `l2_to_l1_promote(L)` is applied
- THEN `L` is present in L1 and absent from L2

### Requirement: Non-Inclusive Policy Imposes No Cross-Level Constraint

r[hierarchy.non-inclusive]
Under the NonInclusive policy, `check_valid` MUST accept any L1/L2 relationship, including presence in neither level, either level, or both levels.

#### Scenario: Non-inclusive duplicate presence allowed

- GIVEN a NonInclusive hierarchy where line `L` is present in both L1 and L2
- WHEN `check_valid` is called
- THEN the result is true

### Requirement: Effective Capacity Formula

r[hierarchy.capacity]
`effective_capacity` MUST return `|L2|` for Inclusive hierarchies and `|L1| + |L2|` for Exclusive and NonInclusive hierarchies.

#### Scenario: Inclusive capacity counts only lower level

- GIVEN an Inclusive hierarchy with two lines in L2
- WHEN `effective_capacity` is called
- THEN the result is `2`

#### Scenario: Exclusive capacity sums both levels

- GIVEN an Exclusive hierarchy with one line in L1 and one line in L2
- WHEN `effective_capacity` is called
- THEN the result is `2`

#### Scenario: Non-inclusive capacity counts both levels independently

- GIVEN a NonInclusive hierarchy with one line in L1 and one line in L2
- WHEN `effective_capacity` is called
- THEN the result is `2`

### Requirement: Inclusive Snoop Scope

r[hierarchy.snoop-scope]
`l2_miss_implies_l1_miss` MUST encode the inclusive snoop-scope rule: when a line is absent from L2 in an Inclusive hierarchy, it is also absent from L1.

#### Scenario: Inclusive L2 miss excludes L1 hit

- GIVEN an Inclusive hierarchy where line `L` is absent from L2
- WHEN `l2_miss_implies_l1_miss(L)` is called
- THEN the result is true only if `L` is also absent from L1

#### Scenario: Exclusive hierarchy can violate L2-only snooping

- GIVEN an Exclusive hierarchy where line `L` is present in L1 and absent from L2
- WHEN `l2_miss_implies_l1_miss(L)` is called
- THEN the result is false

#### Scenario: Non-inclusive hierarchy can violate L2-only snooping

- GIVEN a NonInclusive hierarchy where line `L` is present in L1 and absent from L2
- WHEN `l2_miss_implies_l1_miss(L)` is called
- THEN the result is false
