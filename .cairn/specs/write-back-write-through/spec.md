# Write-Back / Write-Through Policy Specification

## Purpose

Defines requirements for verified write-back and write-through cache write
policies, covering dirty bit invariants, eviction writeback, always-clean
guarantees, policy non-interference, and bulk flush correctness.

## Requirements

### Requirement: Write-Back Dirty Invariant
r[write-policy.write-back-dirty]

Under write-back policy, a line MUST be dirty iff it has been written
since the last writeback or load.

#### Scenario: Write sets dirty

- GIVEN a clean line under write-back policy
- WHEN a write occurs
- THEN the line becomes dirty

#### Scenario: Writeback clears dirty

- GIVEN a dirty line under write-back policy
- WHEN the line is written back to memory
- THEN the line becomes clean

#### Scenario: Load is clean

- GIVEN a line just loaded from memory under write-back policy
- WHEN the dirty bit is inspected
- THEN the line is clean

### Requirement: Write-Back Eviction Flush
r[write-policy.write-back-eviction]

Evicting a dirty line under write-back policy MUST write the data to
memory. Evicting a clean line MUST NOT write to memory.

#### Scenario: Dirty eviction

- GIVEN a dirty line with data D under write-back policy
- WHEN the line is evicted
- THEN memory is updated to D

#### Scenario: Clean eviction

- GIVEN a clean line under write-back policy
- WHEN the line is evicted
- THEN memory is unchanged

### Requirement: Write-Through Always-Clean Invariant
r[write-policy.write-through-clean]

Under write-through policy, no line MUST ever be dirty.

#### Scenario: Write-through write

- GIVEN a line under write-through policy
- WHEN a write occurs
- THEN the dirty bit is false

#### Scenario: Write-through load

- GIVEN a line under write-through policy
- WHEN a load completes
- THEN the dirty bit is false

### Requirement: Write-Through Memory Propagation
r[write-policy.write-through-memory]

Every write under write-through policy MUST update memory immediately.

#### Scenario: Write updates memory

- GIVEN a line at address A under write-through policy
- WHEN value V is written
- THEN `memory[A] == V` after the write

#### Scenario: Memory always current

- GIVEN any sequence of writes under write-through policy
- WHEN memory is inspected at any point
- THEN `memory[A]` equals the most recent write to address A

### Requirement: Policy Non-Interference
r[write-back-write-through.policy-non-interference]

Write-back and write-through lines in the same cache MUST NOT affect
each other's invariants.

#### Scenario: Write-through line next to write-back line

- GIVEN line A under write-through and line B under write-back in the same cache
- WHEN line B is written (becomes dirty)
- THEN line A remains clean

#### Scenario: Write-back eviction doesn't affect write-through

- GIVEN line A under write-through and line B (dirty) under write-back
- WHEN line B is evicted (writeback occurs)
- THEN line A's state is unchanged

### Requirement: Flush-All Correctness
r[write-policy.flush-all]

Flushing all lines MUST make every line clean and make memory match
the cache for all lines, assuming lines that are already clean already
match memory before the flush begins.

#### Scenario: After flush

- GIVEN a cache with some dirty lines under write-back policy, and every already-clean line already matches memory
- WHEN flush-all is performed
- THEN every line is clean and `forall addr :: memory[addr] == cache[addr].data`

#### Scenario: Flush of clean cache

- GIVEN a cache with no dirty lines and memory already matching every line
- WHEN flush-all is performed
- THEN memory is unchanged and all lines remain clean

### Requirement: Read Equivalence
r[write-policy.read-equivalence]

A read hit MUST return the same data regardless of write policy.

#### Scenario: Read under write-back

- GIVEN a valid line with data D under write-back policy
- WHEN a read hit occurs
- THEN D is returned

#### Scenario: Read under write-through

- GIVEN a valid line with data D under write-through policy
- WHEN a read hit occurs
- THEN D is returned
