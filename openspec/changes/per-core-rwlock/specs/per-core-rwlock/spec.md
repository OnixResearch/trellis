# Per-Core Read-Write Lock Specification

## Purpose

Defines requirements for a per-core read-write lock that distributes reader
counts across cores to eliminate contention on the read fast-path. Writers
aggregate per-core counts to establish mutual exclusion. The per-core lock
refines the global rwlock model from `rwlock.rs`.

## Requirements

### Requirement: Aggregation Correctness

The sum of per-core reader counts MUST be computed correctly.

#### Scenario: Empty sequence

- GIVEN a sequence of zero cores
- WHEN the sum is computed
- THEN the result is zero

#### Scenario: Single core

- GIVEN a sequence of one core with reader count c
- WHEN the sum is computed
- THEN the result equals c

#### Scenario: Increment one core

- GIVEN a per-core sequence where core i is incremented by 1
- WHEN the sum is computed
- THEN the new sum equals the old sum plus 1

#### Scenario: Commutativity

- GIVEN any permutation of core counts
- WHEN the sum is computed
- THEN the result is the same regardless of summation order

### Requirement: Per-Core Independence

Modifying one core's reader count MUST NOT affect any other core's count.

#### Scenario: Read acquire on core i

- GIVEN core i acquires a read lock
- WHEN core j (j ≠ i) is inspected
- THEN core j's reader count is unchanged

#### Scenario: Read release on core i

- GIVEN core i releases a read lock
- WHEN core j (j ≠ i) is inspected
- THEN core j's reader count is unchanged

### Requirement: Aggregated Mutual Exclusion

The per-core lock MUST maintain mutual exclusion when reader counts are
aggregated.

#### Scenario: Write held with zero readers

- GIVEN the write flag is held
- WHEN the sum of all per-core reader counts is zero
- THEN mutual exclusion holds

#### Scenario: Write held with nonzero readers

- GIVEN the write flag is held
- WHEN the sum of per-core reader counts is greater than zero
- THEN mutual exclusion is violated (inconsistent state)

#### Scenario: Readers active without writer

- GIVEN the write flag is not held
- WHEN one or more cores have nonzero reader counts
- THEN mutual exclusion holds

### Requirement: Writer Drain Safety

Once the writer sets the write flag, the aggregate reader count MUST be
monotonically non-increasing.

#### Scenario: No new readers after write flag

- GIVEN the write flag is set
- WHEN a core attempts to acquire a read lock
- THEN the acquisition is denied

#### Scenario: Existing readers drain

- GIVEN the write flag is set and the aggregate reader count is n > 0
- WHEN a reader on some core releases
- THEN the new aggregate reader count is n - 1

### Requirement: Mode Equivalence

The per-core lock's observable mode MUST match the global rwlock's mode when
per-core counts are aggregated.

#### Scenario: All counts zero, write not held

- GIVEN per-core reader counts all zero and write flag false
- WHEN the mode is computed
- THEN the mode is MODE_FREE

#### Scenario: Some counts nonzero, write not held

- GIVEN at least one core has a nonzero reader count and write flag false
- WHEN the mode is computed
- THEN the mode is MODE_READ

#### Scenario: Write held, all counts zero

- GIVEN per-core reader counts all zero and write flag true
- WHEN the mode is computed
- THEN the mode is MODE_WRITE

### Requirement: Per-Core Read Acquire

A core SHOULD be able to acquire a read lock by incrementing only its own
counter when the write flag is not set.

#### Scenario: Write not held

- GIVEN the write flag is not set
- WHEN core i acquires a read lock
- THEN core i's count increases by 1 and no other state changes

#### Scenario: Write held

- GIVEN the write flag is set
- WHEN core i attempts to acquire a read lock
- THEN the acquisition fails

### Requirement: Per-Core Read Release

A core MUST release a read lock by decrementing only its own counter.

#### Scenario: Core has readers

- GIVEN core i has a reader count > 0
- WHEN core i releases a read lock
- THEN core i's count decreases by 1 and no other state changes

#### Scenario: Core has no readers

- GIVEN core i has a reader count of 0
- WHEN core i attempts to release a read lock
- THEN the count remains 0 (saturating decrement)

### Requirement: Writer Acquire

A writer MUST set the global write flag and wait for all per-core reader
counts to drain to zero.

#### Scenario: Lock is free

- GIVEN the mode is MODE_FREE (all counts zero, write not held)
- WHEN a writer acquires
- THEN the write flag is set and the mode becomes MODE_WRITE

#### Scenario: Readers still draining

- GIVEN the write flag is set but the aggregate reader count is nonzero
- WHEN the writer checks drain status
- THEN the writer continues waiting

### Requirement: Writer Release

A writer MUST clear the write flag, returning the lock to MODE_FREE.

#### Scenario: Writer releases

- GIVEN the write flag is set and the aggregate reader count is zero
- WHEN the writer releases
- THEN the write flag is cleared and the mode becomes MODE_FREE
