# Consensus Primitives Specification

## Purpose

Defines the mathematical primitives underlying Raft consensus: quorum
arithmetic, monotonic advancement, append-only logs with the Log Matching
Property, and lexicographic epoch comparison. These are protocol-independent
building blocks — they apply to any majority-based consensus system.

## Requirements

### Requirement: Majority Threshold

The majority function MUST return the smallest count that constitutes a
strict majority of `n` members. For any `n > 0`, `majority(n)` MUST
equal `n / 2 + 1`. `majority(1)` MUST be 1. `majority(2)` MUST be 2.

#### Scenario: Odd cluster

- GIVEN a cluster of 5 nodes
- WHEN majority is computed
- THEN the result is 3

#### Scenario: Even cluster

- GIVEN a cluster of 4 nodes
- WHEN majority is computed
- THEN the result is 3

### Requirement: Quorum Predicate

`is_quorum(count, total)` MUST return true if and only if `count >= majority(total)`.
A quorum with zero total members MUST be false. A quorum requires `total > 0`.

### Requirement: Quorum Intersection

If `is_quorum(a, n)` and `is_quorum(b, n)` both hold, then `a + b > n`
MUST hold. Two quorums of the same set MUST always share at least one
member. This is the pigeonhole argument.

#### Scenario: Typical overlap

- GIVEN a cluster of 5 nodes
- WHEN two independent quorums of 3 are formed
- THEN they share at least 1 member (3 + 3 = 6 > 5)

### Requirement: Monotonic Advance

`is_advance(old, new)` MUST return true if and only if `new >= old`.
Advance MUST be reflexive (x advances to x), transitive (a ≤ b ≤ c
implies a ≤ c), and antisymmetric (a ≤ b and b ≤ a implies a == b).

### Requirement: Strict Advance

`is_strict_advance(old, new)` MUST return true if and only if `new > old`.
Strict advance MUST be irreflexive and transitive.

### Requirement: Epoch Comparison

An epoch is a (term, index) pair. `epoch_cmp(a, b)` MUST compare epochs
lexicographically: first by term, then by index within the same term.
The ordering MUST be total, transitive, and antisymmetric.

#### Scenario: Different terms

- GIVEN epoch (3, 5) and epoch (4, 0)
- WHEN compared
- THEN (4, 0) is greater (higher term wins regardless of index)

#### Scenario: Same term

- GIVEN epoch (3, 5) and epoch (3, 8)
- WHEN compared
- THEN (3, 8) is greater (same term, higher index wins)

### Requirement: Epoch At-Least-As-Up-To-Date

`epoch_ge(a, b)` MUST return true if and only if epoch `a` is greater
than or equal to epoch `b` in the lexicographic ordering. This is the
predicate Raft uses for vote granting: a candidate's log must be at
least as up-to-date as the voter's.

### Requirement: Log Entry Agreement

Two logs `agree_at(a, b, i)` at index `i` if both have an entry at
index `i` and the entries have the same term. Agreement does not
require value equality — only term equality.

### Requirement: Log Prefix Matching

If two logs agree at every index in `[0, k]`, they MUST have the same
entries (same terms) at all those indices. `logs_match_prefix(a, b, k)`
MUST hold when `agree_at(a, b, i)` holds for all `0 <= i <= k`.

### Requirement: Log Append

Appending an entry to a log MUST increase its length by 1. The appended
entry MUST appear at the new last index. All prior entries MUST be
unchanged.

### Requirement: Log Truncate

Truncating a log to length `k` MUST discard all entries at index `k`
and beyond. The first `k` entries MUST be unchanged. The resulting
length MUST be `k`.
