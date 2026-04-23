## Why

The existing `memory_order.rs` verifies the ordering lattice and fence
composition, but it stops at the abstract ordering boundary. Real cores
place a store buffer between execution and the cache: stores queue there
before draining, and younger loads may forward from the youngest buffered
store that fully covers the load range.

If that forwarding predicate is wrong, the load can observe stale or
partial data. If the buffer drains out of order, fences stop being a
useful reasoning boundary. This change lands a small verified
`store_buffer` model that closes that gap without trying to model the
full speculative pipeline.

## What Changes

- Add `src/store_buffer.rs` with a fixed-capacity FIFO store buffer and
  verified store-to-load forwarding.
- Export the module from `src/lib.rs` and track the new requirements in
  `docs/requirements.md`.
- Capture focused baseline/runtime/proof evidence for the neighboring
  `memory_order` surface and the new `store_buffer` surface.
- Reconcile the OpenSpec artifacts to the landed runtime boundary and
  archive the change once fresh validation is green.

## Capabilities

### New Capabilities

- `store-buffer.youngest-match`: forwarding selects the youngest
  matching buffered store.
- `store-buffer.full-coverage`: forwarding is allowed only when the
  store fully covers the load byte range.
- `store-buffer.cache-fallback`: a miss in the buffer falls back to the
  cache path.
- `store-buffer.drain-ordering`: draining removes the oldest store first
  and preserves the remaining relative order.
- `store-buffer.fence`: `drain_all` plus `store_fence` model fence
  completion as an empty post-fence buffer.
- `store-buffer.capacity`: enqueue rejects a full buffer instead of
  silently dropping stores.

## Impact

- **Files**: new `src/store_buffer.rs`; updates to `src/lib.rs`,
  `docs/requirements.md`, `README.md`, and synced OpenSpec specs/archive
  artifacts.
- **APIs**: `StoreEntry`, `enqueue`, `drain_oldest`, `drain_all`,
  `can_forward`, `forward_lookup`, `load_source`, `is_fence_satisfied`,
  `store_fence`.
- **Dependencies**: `vstd`; conceptual alignment with `memory_order.rs`
  for fence reasoning, but no new cross-module proof obligations in this
  slice.
- **Testing**: focused unit tests, negative tests, and proptests against
  a brute-force forwarding oracle plus FIFO/capacity regressions.
