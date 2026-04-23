# Store Buffer Forwarding Specification

## Purpose

Defines correctness requirements for a verified fixed-capacity store
buffer. Loads may forward only from the youngest buffered store that
fully covers the load range; otherwise they must use the cache path.
Drain and fence behavior are modeled explicitly over FIFO order.

## Definitions

- **Valid store/load width**: one of `{1, 2, 4, 8}` bytes.
- **Youngest match**: the matching buffered store with the largest
  sequence number / tail-most position.
- **Full coverage**: `entry.addr <= load_addr`, `load_size <= entry.size`,
  and `load_addr - entry.addr <= entry.size - load_size`.
- **Fence completion**: a completed store fence leaves the buffer empty.

## Requirements

### Requirement: Youngest Match

Forwarding MUST select the youngest buffered store that matches the load.

#### Scenario: Single match

- GIVEN a buffer with one matching store for address A
- WHEN a load of address A checks the buffer
- THEN that store is forwarded

#### Scenario: Multiple matches

- GIVEN a buffer with older store `S1` and younger store `S2` that both
  match the same load
- WHEN the load checks the buffer
- THEN `S2` is forwarded and `S1` is ignored

### Requirement: Full Coverage

Forwarding MUST occur only when one buffered store fully covers the load
byte range.

#### Scenario: Store covers load

- GIVEN a 4-byte store at address 100 and a 2-byte load at address 101
- WHEN the load checks the buffer
- THEN forwarding is allowed

#### Scenario: Store does not cover load

- GIVEN a 2-byte store at address 100 and a 4-byte load at address 100
- WHEN the load checks the buffer
- THEN forwarding is denied

### Requirement: Cache Fallback

When no buffered store matches a load, the load MUST use the cache path.

#### Scenario: Empty buffer

- GIVEN an empty store buffer
- WHEN a load occurs
- THEN the load uses the cache path

#### Scenario: No address match

- GIVEN a buffer whose stores do not match the load
- WHEN the load checks the buffer
- THEN the load uses the cache path

### Requirement: Drain Ordering

Buffered stores MUST drain in FIFO order.

#### Scenario: Oldest drains first

- GIVEN stores `S1`, `S2`, `S3` enqueued in that order
- WHEN one store is drained
- THEN `S1` is removed
- AND `S2`, `S3` remain in their original relative order

#### Scenario: Drain-all is FIFO sequence

- GIVEN any valid buffer
- WHEN `drain_all` is observed
- THEN the drained sequence matches the original buffer order exactly

### Requirement: Store Fence

A completed store fence MUST leave the buffer empty before any post-fence
store is enqueued.

#### Scenario: Fence drains buffer

- GIVEN a buffer with pending stores
- WHEN the store fence completes
- THEN the resulting buffer is empty

#### Scenario: Post-fence store

- GIVEN a completed store fence
- WHEN a new store is enqueued
- THEN it enters an empty buffer as the only buffered store

### Requirement: Buffer Capacity

The buffer MUST NOT silently drop stores when full.

#### Scenario: Buffer full

- GIVEN a buffer at maximum capacity
- WHEN enqueue is attempted
- THEN enqueue returns failure and the buffer stays unchanged

#### Scenario: Buffer not full

- GIVEN a buffer below maximum capacity
- WHEN enqueue is attempted
- THEN enqueue succeeds and the buffer grows by exactly one entry
