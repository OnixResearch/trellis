## Why

The existing `memory_order.rs` verifies the ordering lattice and fence
composition, and `mesi.rs` verifies cache coherence transitions. Between
the CPU pipeline and the cache sits the store buffer — a queue of pending
writes that haven't yet been committed to the cache. Store-to-load
forwarding allows a load to read from the store buffer instead of waiting
for the store to commit, but only when the forwarding is correct: the
store must cover the load's address entirely, and the forwarded value
must be from the youngest matching store.

Store buffer bugs are the source of most memory ordering violations in
real hardware. Spectre-class attacks exploit speculative loads that bypass
the store buffer. Getting the forwarding predicate wrong means a load
sees a stale or partial value. This module closes the gap between the
abstract ordering model and the concrete cache.

## What Changes

- **store_buffer**: A new module modeling a FIFO store buffer with
  store-to-load forwarding.

  Proves:
  - **Youngest match**: Forwarding selects the most recent store to the
    same address. If multiple stores to the same address are in the buffer,
    the one closest to the head (youngest) wins.
  - **Full coverage**: Forwarding is only valid when the store fully covers
    the load's byte range. A narrow store to a wider load must not forward.
  - **No forwarding on miss**: When no store in the buffer matches the
    load address, the load goes to the cache. Proves the fallback path.
  - **Drain ordering**: Stores drain from the buffer to the cache in FIFO
    order. The oldest store commits first. Proves ordering is preserved.
  - **Fence semantics**: A store fence drains all buffered stores before
    any subsequent store enters the buffer. Proves fence correctness
    composes with `memory_order.rs` ordering.
  - **Buffer capacity**: The buffer has a fixed capacity. When full, new
    stores stall. Proves no silent drops.

## Capabilities

### New Capabilities

- `store-buffer.youngest-match`: Verified forwarding selects most recent matching store
- `store-buffer.full-coverage`: Verified partial-store rejection
- `store-buffer.cache-fallback`: Verified miss path correctness
- `store-buffer.drain-ordering`: Verified FIFO commit order
- `store-buffer.fence`: Verified store fence drains buffer
- `store-buffer.capacity`: Verified no silent store drops

## Impact

- **Files**: One new `src/store_buffer.rs`, module declaration in `lib.rs`
- **APIs**: `StoreBufferEntry` struct, `forward_lookup`, `drain_oldest`,
  `is_fence_satisfied`, capacity checks
- **Dependencies**: References `memory_order.rs` for fence composition,
  depends on `vstd`
- **Testing**: Verus proofs for all forwarding and ordering properties,
  proptest for forwarding correctness against a brute-force oracle
