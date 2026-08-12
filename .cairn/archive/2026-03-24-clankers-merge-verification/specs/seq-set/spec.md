# Sequence-as-Set Specification

## Purpose

Verified set-theoretic operations over integer sequences. Treats
`Seq<int>` as an unordered collection and provides membership,
difference, intersection, subset, and distinctness predicates with
their algebraic properties.

These primitives are missing from the codebase. The existing
`collections` module has sorted-sequence operations (`binary_search`,
`sorted_insert`, `find_first`, `count_matches`) and `slice` has
structural comparisons (`prefix_eq`, `mismatch_index`), but neither
provides set-algebraic operations. `pattern::seq_contains_int` is a
one-off membership predicate without any associated proofs.

## Definitions

- **Member**: `is_member(s, x)` holds iff `exists i: 0 <= i < s.len()
  && s[i] == x`. Wraps `Seq::contains` with an exec implementation.
- **Set difference**: `set_difference(a, b)` returns elements of `a`
  that are not members of `b`, preserving order from `a`.
- **Set intersection**: `set_intersection(a, b)` returns elements of
  `a` that are also members of `b`, preserving order from `a`.
- **Subset**: `is_subset(a, b)` holds iff every element of `a` is a
  member of `b`.
- **Distinct**: `all_distinct(s)` holds iff no element appears twice.

## Requirements

### Requirement: Membership

`is_member(s, x)` returns true iff `x` appears somewhere in `s`.

#### Scenario: Present element

- GIVEN s = [3, 1, 4] and x = 1
- THEN is_member(s, 1) = true

#### Scenario: Absent element

- GIVEN s = [3, 1, 4] and x = 5
- THEN is_member(s, 5) = false

#### Scenario: Empty sequence

- GIVEN s = [] and any x
- THEN is_member(s, x) = false

### Requirement: Set Difference Correctness

`set_difference(a, b)` contains element `x` iff `x` is a member
of `a` AND `x` is not a member of `b`.

#### Scenario: Disjoint sequences

- GIVEN a = [1, 2, 3] and b = [4, 5]
- THEN set_difference(a, b) = [1, 2, 3]

#### Scenario: Overlapping sequences

- GIVEN a = [1, 2, 3] and b = [2, 4]
- THEN set_difference(a, b) = [1, 3]

#### Scenario: Subset

- GIVEN a = [1, 2] and b = [1, 2, 3]
- THEN set_difference(a, b) = []

#### Scenario: Empty b

- GIVEN a = [1, 2, 3] and b = []
- THEN set_difference(a, b) = [1, 2, 3]

### Requirement: Set Difference Subset of Source

Every element in `set_difference(a, b)` is a member of `a`.

### Requirement: Set Difference Disjoint from Filter

No element in `set_difference(a, b)` is a member of `b`.

### Requirement: Set Intersection Correctness

`set_intersection(a, b)` contains element `x` iff `x` is a member
of `a` AND `x` is a member of `b`.

#### Scenario: Overlapping

- GIVEN a = [1, 2, 3] and b = [2, 3, 4]
- THEN set_intersection(a, b) = [2, 3]

#### Scenario: Disjoint

- GIVEN a = [1, 2] and b = [3, 4]
- THEN set_intersection(a, b) = []

### Requirement: Set Intersection Subset of Both

Every element in `set_intersection(a, b)` is a member of both
`a` and `b`.

### Requirement: Difference-Intersection Partition

For distinct `a`, `set_difference(a, b).len() + set_intersection(a, b).len() == a.len()`.

Every element of `a` appears in exactly one of `set_difference(a, b)`
or `set_intersection(a, b)`.

### Requirement: Subset

`is_subset(a, b)` holds iff every member of `a` is a member of `b`.

Reflexive: `is_subset(a, a)` for any `a`.

Transitive: `is_subset(a, b) && is_subset(b, c)` implies `is_subset(a, c)`.

Empty subset: `is_subset([], b)` for any `b`.

#### Scenario: Proper subset

- GIVEN a = [1, 2] and b = [1, 2, 3]
- THEN is_subset(a, b) = true

#### Scenario: Not a subset

- GIVEN a = [1, 4] and b = [1, 2, 3]
- THEN is_subset(a, b) = false

### Requirement: All Distinct

`all_distinct(s)` holds iff for all `i != j`, `s[i] != s[j]`.

#### Scenario: Distinct

- GIVEN s = [3, 1, 4, 1]
- THEN all_distinct(s) = false (1 appears twice)

#### Scenario: All unique

- GIVEN s = [3, 1, 4, 5]
- THEN all_distinct(s) = true

### Requirement: Difference Preserves Distinctness

If `all_distinct(a)` then `all_distinct(set_difference(a, b))`.

### Requirement: Intersection Preserves Distinctness

If `all_distinct(a)` then `all_distinct(set_intersection(a, b))`.
