# Cache Hierarchy Specification

## Purpose

Defines correctness requirements for a verified two-level cache hierarchy
model with inclusive, exclusive, and non-inclusive policies.

## Definitions

- **Canonical level**: a deduplicated finite collection of line tags.
- **Inclusive policy**: every line in L1 is also present in L2.
- **Exclusive policy**: no line is present in both L1 and L2.
- **Non-inclusive policy**: L1 and L2 have no subset/disjointness
  requirement.
- **Back-invalidation**: evicting a line from L2 also removes it from L1.
- **Exclusive promote**: moving a line from L2 to L1 removes it from L2
  in the same abstract step.

## Requirements

### Requirement: Inclusive Invariant
r[hierarchy.inclusive-invariant]
Under the inclusive policy, every line present in L1 MUST also be
present in L2 after every modeled operation.

`r[hierarchy.inclusive-invariant]`

#### Scenario: L1 fill keeps inclusion
`r[hierarchy.inclusive-invariant.scenario-l1-fill]`

- GIVEN an inclusive hierarchy state
- WHEN a line is filled into L1
- THEN that line is also present in L2

#### Scenario: Initial state
`r[hierarchy.inclusive-invariant.scenario-initial]`

- GIVEN empty L1 and empty L2
- WHEN the inclusive invariant is checked
- THEN it holds

#### Scenario: L1 eviction keeps inclusion
`r[hierarchy.inclusive-invariant.scenario-l1-evict]`

- GIVEN an inclusive hierarchy state
- WHEN a line is evicted from L1
- THEN the resulting L1 remains a subset of L2

#### Scenario: L2 fill keeps inclusion
`r[hierarchy.inclusive-invariant.scenario-l2-fill]`

- GIVEN an inclusive hierarchy state
- WHEN a line is filled into L2
- THEN the resulting L1 remains a subset of L2

### Requirement: Back Invalidation
r[hierarchy.back-invalidation]
Under the inclusive policy, evicting a line from L2 MUST also invalidate
that line from L1.

`r[hierarchy.back-invalidation]`

#### Scenario: L2 eviction removes L1 copy
`r[hierarchy.back-invalidation.scenario-l2-evict]`

- GIVEN an inclusive hierarchy where line `L` is in both L1 and L2
- WHEN `L` is evicted from L2
- THEN `L` is no longer present in L1

#### Scenario: L2-only eviction leaves L1 unchanged
`r[hierarchy.back-invalidation.scenario-l2-only]`

- GIVEN an inclusive hierarchy where line `L` is in L2 and absent from L1
- WHEN `L` is evicted from L2
- THEN L1 is unchanged

#### Scenario: Missing back-invalidation breaks inclusion
`r[hierarchy.back-invalidation.scenario-no-back-invalidation]`

- GIVEN an inclusive hierarchy where line `L` is in both L1 and L2
- WHEN `L` is removed from L2 without removing it from L1
- THEN the inclusive invariant is violated

### Requirement: Exclusive Invariant
r[hierarchy.exclusive-invariant]
Under the exclusive policy, a line MUST NOT be present in both L1 and L2 at
the same time.

`r[hierarchy.exclusive-invariant]`

#### Scenario: L1 fill removes L2 copy
`r[hierarchy.exclusive-invariant.scenario-l1-fill]`

- GIVEN an exclusive hierarchy state
- WHEN a line is filled into L1
- THEN that line is absent from L2

#### Scenario: L1 eviction victims to L2
`r[hierarchy.exclusive-invariant.scenario-l1-evict]`

- GIVEN an exclusive hierarchy state with line `L` in L1
- WHEN `L` is evicted from L1
- THEN `L` is absent from L1 and present in L2
- AND no line is present in both levels

### Requirement: Exclusive Swap
r[hierarchy.exclusive-swap]
Under the exclusive policy, moving a line between levels MUST remove it
from the source level and install it in the destination level in the
same abstract step. Demotion is modeled by `l1_evict`; promotion is
modeled by `l2_to_l1_promote`.

`r[hierarchy.exclusive-swap]`

#### Scenario: L2-to-L1 promote
`r[hierarchy.exclusive-swap.scenario-promote]`

- GIVEN an exclusive hierarchy where line `L` is present in L2
- WHEN `L` is promoted to L1
- THEN `L` is present in L1 and absent from L2

### Requirement: Non-Inclusive Freedom
r[hierarchy.non-inclusive]
Under the non-inclusive policy, the model MUST impose no subset or
exclusion constraint between L1 and L2.

`r[hierarchy.non-inclusive]`

#### Scenario: Same line in both levels
`r[hierarchy.non-inclusive.scenario-same-line]`

- GIVEN a non-inclusive hierarchy
- WHEN the same line is present in both levels
- THEN the state remains valid

### Requirement: Capacity Accounting
r[hierarchy.capacity]
For canonical levels, the inclusive effective capacity MUST equal the
current unique-line count `|L2|`, and the exclusive effective capacity
MUST equal the current unique-line count `|L1| + |L2|`.

`r[hierarchy.capacity]`

#### Scenario: Inclusive capacity
`r[hierarchy.capacity.scenario-inclusive]`

- GIVEN a canonical inclusive hierarchy state
- WHEN effective capacity is computed
- THEN the result equals the number of lines in L2

#### Scenario: Exclusive capacity
`r[hierarchy.capacity.scenario-exclusive]`

- GIVEN a canonical exclusive hierarchy state
- WHEN effective capacity is computed
- THEN the result equals the number of lines in L1 plus the number of
  lines in L2

### Requirement: Snoop Scope
r[hierarchy.snoop-scope]
Under the inclusive policy, an L2 miss MUST imply an L1 miss for the
same line. Under the exclusive policy, checking L2 alone MUST NOT be
assumed sufficient.

`r[hierarchy.snoop-scope]`

#### Scenario: Inclusive L2 miss
`r[hierarchy.snoop-scope.scenario-inclusive]`

- GIVEN a canonical inclusive hierarchy state
- WHEN a snoop misses in L2 for line `L`
- THEN line `L` is absent from L1

#### Scenario: Exclusive requires both levels
`r[hierarchy.snoop-scope.scenario-exclusive]`

- GIVEN an exclusive hierarchy where line `L` is in L1 and not in L2
- WHEN only L2 is checked
- THEN the snoop would miss even though L1 still contains `L`
