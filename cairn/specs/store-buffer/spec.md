# Store Buffer Specification

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

### Requirement: Youngest Match [r[store-buffer.youngest-match]]

Forwarding MUST select the youngest buffered store that matches the load.

#### Scenario: Single match [r[store-buffer.youngest-match.scenario-single-match]]

- GIVEN a buffer with one matching store for address A
- WHEN a load of address A checks the buffer
- THEN that store is forwarded

#### Scenario: Multiple matches [r[store-buffer.youngest-match.scenario-multiple-matches]]

- GIVEN a buffer with older store `S1` and younger store `S2` that both
  match the same load
- WHEN the load checks the buffer
- THEN `S2` is forwarded and `S1` is ignored

### Requirement: Full Coverage [r[store-buffer.full-coverage]]

Forwarding MUST occur only when one buffered store fully covers the load
byte range.

#### Scenario: Store covers load [r[store-buffer.full-coverage.scenario-store-covers-load]]

- GIVEN a 4-byte store at address 100 and a 2-byte load at address 101
- WHEN the load checks the buffer
- THEN forwarding is allowed

#### Scenario: Store does not cover load [r[store-buffer.full-coverage.scenario-store-does-not-cover-load]]

- GIVEN a 2-byte store at address 100 and a 4-byte load at address 100
- WHEN the load checks the buffer
- THEN forwarding is denied

### Requirement: Cache Fallback [r[store-buffer.cache-fallback]]

When no buffered store matches a load, the load MUST use the cache path.

#### Scenario: Empty buffer [r[store-buffer.cache-fallback.scenario-empty-buffer]]

- GIVEN an empty store buffer
- WHEN a load occurs
- THEN the load uses the cache path

#### Scenario: No address match [r[store-buffer.cache-fallback.scenario-no-address-match]]

- GIVEN a buffer whose stores do not match the load
- WHEN the load checks the buffer
- THEN the load uses the cache path

### Requirement: Drain Ordering [r[store-buffer.drain-ordering]]

Buffered stores MUST drain in FIFO order.

#### Scenario: Oldest drains first [r[store-buffer.drain-ordering.scenario-oldest-drains-first]]

- GIVEN stores `S1`, `S2`, `S3` enqueued in that order
- WHEN one store is drained
- THEN `S1` is removed
- AND `S2`, `S3` remain in their original relative order

#### Scenario: Drain-all is FIFO sequence [r[store-buffer.drain-ordering.scenario-drain-all-fifo-sequence]]

- GIVEN any valid buffer
- WHEN `drain_all` is observed
- THEN the drained sequence matches the original buffer order exactly

### Requirement: Store Fence [r[store-buffer.fence]]

A completed store fence MUST leave the buffer empty before any post-fence
store is enqueued.

#### Scenario: Fence drains buffer [r[store-buffer.fence.scenario-fence-drains-buffer]]

- GIVEN a buffer with pending stores
- WHEN the store fence completes
- THEN the resulting buffer is empty

#### Scenario: Post-fence store [r[store-buffer.fence.scenario-post-fence-store]]

- GIVEN a completed store fence
- WHEN a new store is enqueued
- THEN it enters an empty buffer as the only buffered store

### Requirement: Buffer Capacity [r[store-buffer.capacity]]

The buffer MUST NOT silently drop stores when full.

#### Scenario: Buffer full [r[store-buffer.capacity.scenario-buffer-full]]

- GIVEN a buffer at maximum capacity
- WHEN enqueue is attempted
- THEN enqueue returns failure and the buffer stays unchanged

#### Scenario: Buffer not full [r[store-buffer.capacity.scenario-buffer-not-full]]

- GIVEN a buffer below maximum capacity
- WHEN enqueue is attempted
- THEN enqueue succeeds and the buffer grows by exactly one entry
