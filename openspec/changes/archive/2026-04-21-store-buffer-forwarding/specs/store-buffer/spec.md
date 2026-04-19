# Store Buffer Specification

## Purpose

Defines requirements for this change's verified FIFO store buffer with
full-coverage store-to-load forwarding, FIFO drain semantics, fence
emptiness, and fail-closed capacity behavior.

## Model Boundaries

- Modeled store and load widths are valid byte counts in `1..=8`.
- `enqueue` accepts caller-provided `StoreEntry` values and requires the new entry's `seq` to be strictly greater than the current tail entry's `seq`; non-increasing `seq` inputs are outside the modeled input domain.
- The address domain is byte-addressed `u64`, and modeled coverage uses non-wrapping offset arithmetic rather than wrapped end-address addition. High-address cases use that same offset rule; the model never depends on wrapped `addr + width` arithmetic.
- Forwarding never combines multiple partial stores to satisfy one load.

## Requirements

### Requirement: Input Domain and Buffer Order

r[store-buffer.input-domain]
The modeled store-buffer domain uses `StoreEntry` values carrying at
least `addr`, `size`, `data`, and `seq`, byte-addressed `u64` addresses,
store/load widths in `1..=8`, and buffers whose entries are kept in
strictly increasing sequence-number order. Successful enqueue preserves
that modeled order; non-increasing sequence inputs are outside the
modeled input domain.

#### Scenario: Valid widths stay within the modeled domain

- GIVEN a modeled store or load width
- WHEN it is used by the store-buffer APIs
- THEN the width is in `1..=8`

#### Scenario: Successful enqueue preserves increasing sequence order

- GIVEN a well-formed buffer and a new entry whose `seq` is greater than the current tail `seq`
- WHEN enqueue succeeds
- THEN the resulting buffer remains in strictly increasing `seq` order

#### Scenario: Enqueued store entry size stays in modeled width domain

- GIVEN a caller-provided `StoreEntry` accepted by enqueue
- WHEN its `size` field is inspected
- THEN that size is in `1..=8`

### Requirement: Full-Coverage Forwarding

r[store-buffer.full-coverage]
A buffered store MUST forward to a load only when `can_forward` holds for
that store and load, meaning the store fully covers the load's byte
range. Partial overlap or reversed address ordering MUST NOT forward.

#### Scenario: Exact match forwards

- GIVEN a buffered store whose address and size exactly match a load
- WHEN forwarding is checked
- THEN the store is eligible to forward

#### Scenario: Partial overlap rejected

- GIVEN a buffered store that overlaps only part of the load range
- WHEN forwarding is checked
- THEN the store is not eligible to forward

#### Scenario: Coverage uses non-wrapping offset arithmetic

- GIVEN byte-addressed `u64` addresses and valid byte widths in `1..=8`
- WHEN coverage is checked
- THEN coverage is decided with non-wrapping offset arithmetic rather than wrapped end-address addition

### Requirement: Youngest Match Wins

r[store-buffer.youngest-match]
When multiple buffered stores satisfy `can_forward` for the same load,
`forward_lookup` MUST return the youngest matching `StoreEntry`.

#### Scenario: Later store supersedes earlier store

- GIVEN two covering stores for the same load and the later store has the larger sequence number
- WHEN forwarding is looked up
- THEN the later store is returned

### Requirement: Cache Fallback on Miss

r[store-buffer.cache-fallback]
When no buffered store satisfies `can_forward` for a load,
`forward_lookup` MUST return no match so the load may fall back to the
cache.

#### Scenario: No covering store

- GIVEN a buffer with no store that fully covers the load range
- WHEN forwarding is looked up
- THEN the result is no match

### Requirement: FIFO Drain Ordering

r[store-buffer.drain-ordering]
Draining MUST remove the oldest buffered store first. Repeated drains MUST
produce entries in enqueue order and preserve the remaining entries'
sequence order.

#### Scenario: Drain returns head entry

- GIVEN a non-empty buffer
- WHEN the oldest store is drained
- THEN the returned entry is the head of the buffer and the tail order is preserved

#### Scenario: Empty buffer returns no entry

- GIVEN an empty buffer
- WHEN the oldest store is drained
- THEN the result is no entry and the buffer remains empty

#### Scenario: Repeated drain reproduces enqueue order

- GIVEN a buffer with multiple entries in enqueue order
- WHEN stores are drained until empty
- THEN the drained sequence matches the original enqueue order

### Requirement: Fence Requires Emptiness

r[store-buffer.fence]
A store fence MUST be satisfied exactly when the buffer is empty.
Draining all buffered stores MUST make the fence satisfied. Once the
buffer is empty, the next enqueue starts from the empty-buffer state.

#### Scenario: Empty buffer satisfies fence

- GIVEN an empty store buffer
- WHEN fence satisfaction is checked
- THEN the result is true

#### Scenario: Non-empty buffer blocks fence

- GIVEN a store buffer with at least one pending store
- WHEN fence satisfaction is checked
- THEN the result is false

#### Scenario: First enqueue after drain starts from empty

- GIVEN a store buffer that has been drained to empty
- WHEN the next store is enqueued successfully
- THEN the resulting buffer has length 1 and contains only that new entry

### Requirement: Capacity Fails Closed

r[store-buffer.capacity]
Capacity is an explicit per-call limit parameter to `enqueue`, and a
buffer is at or above capacity whenever its current length is greater
than or equal to that supplied capacity. The buffer MUST reject enqueue
in that state and MUST NOT silently drop stores. Successful enqueue MUST
increase the length by one, and draining a non-empty buffer MUST
decrease the length by one.

#### Scenario: Full buffer rejects enqueue

- GIVEN a buffer already at capacity
- WHEN a new store is enqueued
- THEN enqueue returns no result and the existing buffer is unchanged

#### Scenario: Non-full buffer grows

- GIVEN a buffer below capacity
- WHEN a new store is enqueued
- THEN enqueue succeeds and the length increases by one
