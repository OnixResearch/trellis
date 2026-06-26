# Per-Core Read-Write Lock Specification

## Purpose

Defines requirements for a per-core read-write lock that distributes reader
counts across cores to eliminate contention on the read fast path. Writers
aggregate per-core counts to establish mutual exclusion. The per-core lock
refines the global rwlock model from `rwlock.rs`.

## Requirements

### Requirement: Aggregation Correctness

The mathematical per-core reader sum MUST be exact, and the exec-level
aggregator MUST compute the same result with `u64` saturation.

#### Scenario: Empty sequence

- GIVEN a sequence of zero cores
- WHEN the sum is computed
- THEN the result is zero

#### Scenario: Single core

- GIVEN a sequence of one core with reader count c
- WHEN the sum is computed
- THEN the result equals c

#### Scenario: Increment one core

- GIVEN a per-core sequence where core i is incremented by 1 below saturation
- WHEN the mathematical sum is computed
- THEN the new sum equals the old sum plus 1

#### Scenario: Exec aggregation

- GIVEN any finite sequence of per-core reader counts
- WHEN the exec aggregator runs
- THEN it matches the naive left-to-right `u64::saturating_add` fold

### Requirement: Per-Core Independence

Modifying one core's reader count MUST NOT affect any other core's count.

#### Scenario: Read acquire on core i

- GIVEN core i acquires a read lock
- WHEN core j (j != i) is inspected
- THEN core j's reader count is unchanged

#### Scenario: Read release on core i

- GIVEN core i releases a read lock
- WHEN core j (j != i) is inspected
- THEN core j's reader count is unchanged

### Requirement: Aggregated Mutual Exclusion

The per-core lock MUST expose the same mutual-exclusion surface as the global
rwlock once reader counts are aggregated.

#### Scenario: Write held with zero readers

- GIVEN the write flag is held
- WHEN the aggregate reader count is zero
- THEN mutual exclusion holds

#### Scenario: Write held with nonzero readers

- GIVEN the write flag is held
- WHEN the aggregate reader count is greater than zero
- THEN mutual exclusion is violated

#### Scenario: No new readers after write flag

- GIVEN the write flag is set
- WHEN a core attempts to acquire a read lock
- THEN the read-acquire gate rejects the attempt

### Requirement: Writer Drain Safety

Setting the write flag MUST begin a drain phase in which existing readers can
release and drain completion is observed exactly at aggregate zero.

#### Scenario: Writer phase begins

- GIVEN any per-core reader state
- WHEN the writer sets the write flag
- THEN the write flag becomes true

#### Scenario: Existing reader releases during drain

- GIVEN the write flag is set and core i has a positive reader count
- WHEN core i releases one read lock
- THEN the mathematical aggregate reader sum decreases by exactly 1

#### Scenario: Drain completion

- GIVEN any per-core reader state
- WHEN the writer checks drain completion
- THEN it reports true exactly when the aggregate reader count is zero

### Requirement: Mode Equivalence

The per-core lock's observable mode MUST match the global rwlock mode surface
when the write flag and aggregate reader count are projected into
Free/Read/Write states.

#### Scenario: All counts zero, write flag clear

- GIVEN all per-core reader counts are zero and the write flag is false
- WHEN the observable mode is computed
- THEN the mode is MODE_FREE

#### Scenario: Some counts nonzero, write flag clear

- GIVEN at least one core has a nonzero reader count and the write flag is false
- WHEN the observable mode is computed
- THEN the mode is MODE_READ

#### Scenario: Write flag set

- GIVEN the write flag is true
- WHEN the observable mode is computed
- THEN the mode is MODE_WRITE

#### Scenario: Writer release after drain

- GIVEN the aggregate reader count is zero and the writer clears the write flag
- WHEN the observable mode is computed
- THEN the mode is MODE_FREE
