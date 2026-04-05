# Store Buffer Forwarding Specification

## Purpose

Defines requirements for a verified store buffer with store-to-load
forwarding. The buffer queues pending writes; loads check it before
going to the cache. Forwarding must select the correct store and reject
partial matches.

## Requirements

### Requirement: Youngest Match

Forwarding MUST select the most recent store to the matching address.

#### Scenario: Single match

- GIVEN a buffer with one store to address A
- WHEN a load of address A checks the buffer
- THEN that store's data is forwarded

#### Scenario: Multiple matches

- GIVEN a buffer with stores S1 (older) and S2 (younger) both to address A
- WHEN a load of address A checks the buffer
- THEN S2's data is forwarded (not S1's)

#### Scenario: No match

- GIVEN a buffer with no stores to address A
- WHEN a load of address A checks the buffer
- THEN no forwarding occurs and the load goes to cache

### Requirement: Full Coverage

Forwarding MUST only occur when the store fully covers the load's byte range.

#### Scenario: Store covers load

- GIVEN a 4-byte store at address 100 and a 2-byte load at address 101
- WHEN the load checks the buffer
- THEN forwarding is allowed (store covers bytes 100-103, load needs 101-102)

#### Scenario: Store does not cover load

- GIVEN a 2-byte store at address 100 and a 4-byte load at address 100
- WHEN the load checks the buffer
- THEN forwarding is denied (store only covers 100-101, load needs 100-103)

### Requirement: Cache Fallback

When no store in the buffer matches the load, the load MUST be served
from the cache.

#### Scenario: Empty buffer

- GIVEN an empty store buffer
- WHEN a load occurs
- THEN the load goes to cache

#### Scenario: No address match

- GIVEN a buffer with stores to addresses B and C
- WHEN a load of address A (A ≠ B, A ≠ C) occurs
- THEN the load goes to cache

### Requirement: Drain Ordering

Stores MUST drain from the buffer to the cache in FIFO order.

#### Scenario: Two stores drain

- GIVEN stores S1 (enqueued first) and S2 (enqueued second)
- WHEN the buffer drains
- THEN S1 commits to cache before S2

#### Scenario: Drain preserves remaining order

- GIVEN stores S1, S2, S3 in the buffer
- WHEN S1 is drained
- THEN S2 and S3 remain in their original relative order

### Requirement: Store Fence

A store fence MUST ensure all buffered stores are drained before any
subsequent store enters the buffer.

#### Scenario: Fence drains buffer

- GIVEN a buffer with pending stores and a store fence is issued
- WHEN the fence completes
- THEN the buffer is empty

#### Scenario: Post-fence store

- GIVEN a store fence has completed
- WHEN a new store is enqueued
- THEN it enters an empty buffer

### Requirement: Buffer Capacity

The buffer MUST NOT silently drop stores when full.

#### Scenario: Buffer full

- GIVEN a buffer at maximum capacity
- WHEN a new store is attempted
- THEN the store stalls (is not enqueued and is not dropped)

#### Scenario: Buffer not full

- GIVEN a buffer below maximum capacity
- WHEN a new store is attempted
- THEN the store is enqueued and the buffer size increases by 1
