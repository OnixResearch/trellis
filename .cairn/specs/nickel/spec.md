# Nickel Verified Components Specification

## Purpose

Defines requirements for machine-checked implementations of Nickel language
core semantics: record merge, field priority, type subtyping, contract
polarity, and pattern coverage. Each requirement maps to a `spec fn` /
`exec fn` pair verified by Verus.

## Requirements

### Merge Split

### Requirement: Three-Way Split
r[nickel-merge.split]

split(m1, m2) MUST partition keys into three disjoint sets: left (keys
only in m1), center (keys in both), and right (keys only in m2). The
union of left, center, and right key sets MUST equal the union of m1 and
m2 key sets. No key MUST appear in more than one partition.

### Requirement: Split Completeness
r[nickel-merge.split-completeness]

Every key in m1 MUST appear in either left or center. Every key in m2
MUST appear in either center or right. No keys are lost.

### Requirement: Split Value Preservation
r[nickel-merge.split-value-preservation]

For keys in left, the value MUST come from m1. For keys in right, the
value MUST come from m2. For keys in center, both values MUST be
preserved as a pair (v1, v2).

#### Scenario: Disjoint maps
- GIVEN m1 = {a: 1} and m2 = {b: 2}
- WHEN split
- THEN left = {a: 1}, center = {}, right = {b: 2}

#### Scenario: Overlapping maps
- GIVEN m1 = {a: 1, b: 2} and m2 = {b: 3, c: 4}
- WHEN split
- THEN left = {a: 1}, center = {b: (2, 3)}, right = {c: 4}

#### Scenario: Identical keys
- GIVEN m1 = {a: 1} and m2 = {a: 2}
- WHEN split
- THEN left = {}, center = {a: (1, 2)}, right = {}

### Merge Priority

### Requirement: Priority Total Order
r[nickel-priority.total-order]

MergePriority MUST form a total order: Bottom < Neutral < Numeral(n) < Top
for all n > 0. Neutral MUST compare equal to Numeral(0). The order MUST
be reflexive, antisymmetric, and transitive.

### Requirement: Priority Extrema
r[nickel-priority.extrema]

Bottom MUST be less than or equal to every other priority. Top MUST be
greater than or equal to every other priority. These are the lattice
bounds.

### Requirement: Numeral Ordering
r[nickel-priority.numeral-ordering]

Numeral(a) < Numeral(b) if and only if a < b. Numeral ordering MUST
agree with the underlying integer ordering.

#### Scenario: Bottom is minimum
- GIVEN priorities Bottom, Neutral, Numeral(5), Top
- WHEN compared with Bottom
- THEN Bottom <= all others

#### Scenario: Neutral equals zero
- GIVEN Neutral and Numeral(0)
- WHEN compared
- THEN they are equal

### Field Merge

### Requirement: Higher Priority Wins
r[nickel-field-merge.higher-wins]

When merging two fields with different priorities, the field with strictly
higher priority MUST be selected. The losing field's value MUST be
discarded.

### Requirement: Equal Priority Recurse
r[nickel-field-merge.equal-recurse]

When merging two fields with equal priority, the merge MUST recurse into
the values. For non-record values at equal priority, both values MUST be
equal or the merge MUST fail (conflict).

### Requirement: Default Override
r[nickel-field-merge.default-override]

A field with priority Bottom (default) MUST be overridden by any field
with priority Neutral or higher. A field with priority Top (force) MUST
override any field with priority less than Top.

#### Scenario: Force overrides default
- GIVEN field f1 with priority Bottom, value 1 and field f2 with priority Top, value 2
- WHEN merged
- THEN result has priority Top, value 2

#### Scenario: Equal priority conflict
- GIVEN field f1 with priority Neutral, value 1 and field f2 with priority Neutral, value 2
- WHEN merged with non-equal values
- THEN merge fails (conflict)

### Record Merge

### Requirement: Merge Identity
r[nickel-record-merge.identity]

Merging any record with the empty record MUST return the original record
unchanged. merge(r, {}) == r and merge({}, r) == r.

### Requirement: Merge Associativity
r[nickel-record-merge.associativity]

merge(a, merge(b, c)) MUST equal merge(merge(a, b), c) when all merges
succeed and priorities are uniform (all Neutral).

### Requirement: Merge Key Coverage
r[nickel-record-merge.key-coverage]

The keys of merge(a, b) MUST equal the union of keys(a) and keys(b).
No keys are lost or invented.

### Requirement: Disjoint Merge Commutativity
r[nickel-record-merge.disjoint-commutativity]

When records a and b have no keys in common, merge(a, b) MUST equal
merge(b, a). Disjoint merge is commutative.

#### Scenario: Three-way associativity
- GIVEN records {a: 1}, {b: 2}, {c: 3} all at Neutral priority
- WHEN merged left-to-right vs right-to-left
- THEN results are identical: {a: 1, b: 2, c: 3}

### Type Subtyping

### Requirement: Subtype Reflexivity
r[nickel-subtype.reflexivity]

Every type T MUST be a subtype of itself: T <: T.

### Requirement: Subtype Transitivity
r[nickel-subtype.transitivity]

Subtyping MUST be transitive: if T1 <: T2 and T2 <: T3, then T1 <: T3.

### Requirement: Record-Dictionary Subsumption
r[nickel-subtype.record-dict]

A record type {a1: T1, ..., an: Tn} MUST be a subtype of dictionary
type {_: U} if and only if Ti <: U for every field i. This is the core
subtyping axiom in Nickel.

### Requirement: Array Covariance
r[nickel-subtype.array-covariance]

Array types MUST be covariant: Array T <: Array U if and only if T <: U.
Arrays are covariant in their element type.

### Requirement: Dictionary Covariance
r[nickel-subtype.dict-covariance]

Dictionary types MUST be covariant: {_: T} <: {_: U} if and only if T <: U.
Dictionaries are covariant in their value type.

### Requirement: Closed Record Matching
r[nickel-subtype.closed-records]

A closed record MUST be a subtype of another closed record if and only if
both records have exactly the same field names and the corresponding
field types are pairwise subtypes:
{a: T, b: U} <: {a: T', b: U'} when T <: T' and U <: U'.

#### Scenario: Record subsumes dictionary
- GIVEN record type {x: Number, y: Number} and dictionary type {_: Number}
- WHEN checked
- THEN the record is a subtype of the dictionary

### Contract Polarity

### Requirement: Polarity Flip on Domain
r[nickel-polarity.flip-domain]

When a contract checks a function argument (domain position), the
polarity MUST flip: Positive becomes Negative, Negative becomes Positive.

### Requirement: Polarity Preserve on Codomain
r[nickel-polarity.preserve-codomain]

When a contract checks a function return value (codomain position), the
polarity MUST be preserved.

### Requirement: Double Flip Identity
r[nickel-polarity.double-flip]

Flipping polarity twice MUST return to the original polarity:
flip(flip(p)) == p for all p.

### Requirement: Polarity Path Tracking
r[nickel-polarity.path-tracking]

The polarity at any point in a type path MUST equal the initial polarity
flipped once for each domain step in the path. Codomain steps do not
change polarity.

#### Scenario: First-order function
- GIVEN a contract Number -> Number with initial Positive polarity
- WHEN checking the argument
- THEN polarity is Negative (one flip for domain)

#### Scenario: Higher-order function
- GIVEN a contract (Number -> Number) -> Number with initial Positive polarity
- WHEN checking the inner function's argument
- THEN polarity is Positive (two flips: outer domain + inner domain)

### Pattern Coverage

### Requirement: Coverage Completeness
r[nickel-pattern.coverage-completeness]

A set of patterns MUST be complete for a type when every possible value
of that type matches at least one pattern. An incomplete pattern set
MUST be detected and reported.

### Requirement: Overlap Detection
r[nickel-pattern.overlap-detection]

When two patterns in a match expression can match the same value, this
overlap MUST be detectable. The first matching pattern takes precedence.

### Requirement: Wildcard Covers All
r[nickel-pattern.wildcard-covers-all]

A wildcard pattern `_` MUST match any value. A pattern set containing
a wildcard MUST always be complete.

### Requirement: Enum Coverage
r[nickel-pattern.enum-coverage]

For an enum type with variants v1, ..., vn, a pattern set MUST be
complete if and only if it contains a pattern for each vi (or a wildcard).
Missing variants MUST be identified.

#### Scenario: Complete enum match
- GIVEN enum type [| 'a, 'b, 'c |] and patterns 'a, 'b, 'c
- WHEN checked for completeness
- THEN the pattern set is complete

#### Scenario: Incomplete enum match
- GIVEN enum type [| 'a, 'b, 'c |] and patterns 'a, 'b
- WHEN checked for completeness
- THEN 'c is reported as uncovered

## ADDED Requirements

All requirements in this document are new additions.

## MODIFIED Requirements

None.

## REMOVED Requirements

None.
