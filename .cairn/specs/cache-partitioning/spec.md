# Cache Partitioning Specification

## Purpose

Defines requirements for verified way-based cache partitioning, covering
cross-partition isolation, eviction containment, capacity accounting,
partition assignment validity, resizing, and equivalence to an
unpartitioned cache.

## Requirements

### Requirement: Partition Isolation
r[cache-partition.isolation]

A lookup for partition P MUST NOT hit a line stored in a way assigned to
partition Q (Q ≠ P).

#### Scenario: Cross-partition miss

- GIVEN way W assigned to partition Q and holding tag T
- WHEN partition P (P ≠ Q) performs a lookup for tag T in the same set
- THEN the lookup does not return way W

#### Scenario: Same-partition hit

- GIVEN way W assigned to partition P and holding tag T
- WHEN partition P performs a lookup for tag T
- THEN the lookup returns way W

#### Scenario: No data leakage

- GIVEN partitions A and B with data in the same set
- WHEN partition A performs a lookup
- THEN only ways assigned to A are examined

### Requirement: Eviction Containment
r[cache-partition.eviction-containment]

Eviction within partition P MUST only select victim ways assigned to P.

#### Scenario: Eviction respects partition

- GIVEN a full set with ways assigned to partitions P and Q
- WHEN partition P needs to evict
- THEN the victim way is assigned to P (not Q)

#### Scenario: Other partition untouched

- GIVEN partition P evicts a way
- WHEN partition Q's ways are inspected
- THEN Q's ways are unchanged

### Requirement: Capacity Accounting
r[cache-partition.capacity]

The sum of all partition sizes (in ways) MUST equal the total
associativity.

#### Scenario: Two partitions

- GIVEN partition A with 4 ways and partition B with 4 ways in an 8-way cache
- WHEN the total is computed
- THEN 4 + 4 = 8 = total associativity

#### Scenario: All partitions sum

- GIVEN N partitions with sizes S_1, S_2, ..., S_N
- WHEN the sum is computed
- THEN S_1 + S_2 + ... + S_N equals the total number of ways

### Requirement: Assignment Validity
r[cache-partition.assignment]

Every way MUST be assigned to exactly one valid partition.

#### Scenario: No unassigned ways

- GIVEN a valid partition assignment
- WHEN every way is inspected
- THEN each way has a partition ID in [0, num_partitions)

#### Scenario: No double assignment

- GIVEN a valid partition assignment
- WHEN way W is inspected
- THEN W belongs to exactly one partition

### Requirement: Resize Correctness
r[cache-partition.resize]

Resizing a partition MUST invalidate transferred ways before reassignment.

#### Scenario: Way transfer

- GIVEN way W assigned to partition A
- WHEN W is transferred to partition B
- THEN W is invalidated (data cleared) and reassigned to B

#### Scenario: Non-transferred ways unchanged

- GIVEN a resize that transfers ways from A to B
- WHEN a way not in the transfer set is inspected
- THEN its partition assignment and data are unchanged

#### Scenario: Capacity preserved

- GIVEN a resize that moves K ways from A to B
- WHEN the sizes are checked
- THEN A's size decreased by K, B's size increased by K, total unchanged

### Requirement: Full-Cache Equivalence
r[cache-partition.full-equivalence]

A single partition containing all ways MUST behave identically to an
unpartitioned cache.

#### Scenario: Single partition lookup

- GIVEN one partition owning all ways
- WHEN a lookup is performed
- THEN all ways are examined (same as unpartitioned lookup)

#### Scenario: Single partition eviction

- GIVEN one partition owning all ways
- WHEN an eviction is needed
- THEN all ways are candidates (same as unpartitioned eviction)
