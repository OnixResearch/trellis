## Why

The primitive-family taxonomy now covers numeric foundations and domain-specific
GIS, ML, physics, and vision modules, but later proofs will also need reusable
finite data-structure primitives. Arrays, maps, queues, ring buffers, heaps, and
union-find structures recur across those families and should have one focused
OpenSpec rather than being re-specified inside each domain change.

This change implements a focused data-structure primitive package. It keeps the
models exact and finite, focuses on admission, index/key safety, deterministic
ordering, and mutation/no-mutation guarantees.

## What Changes

- Add a focused `add-data-structure-primitives` OpenSpec for reusable finite
  data-structure modules:
  - `ds_bounded_array`: length/capacity/index admission and point updates.
  - `ds_key_value_map`: finite key uniqueness, lookup, insert, and remove.
  - `ds_fifo_queue`: bounded FIFO order and overflow/underflow rejection.
  - `ds_ring_buffer`: wraparound index arithmetic and occupied-slot safety.
  - `ds_priority_queue`: bounded heap/order admission and deterministic pop-min.
  - `ds_union_find`: parent/rank admission, root stability, and union connectivity.
- Require public exec surfaces to reject invalid caller input or return explicit
  empty/error decisions; proof-only helpers may use explicit `requires` after
  validity is established.
- Sync implementation evidence and canonical requirement text in this slice.

## Capabilities

### New Capabilities

- `domain-primitives.ds-bounded-array`: reusable bounded sequence/index-update
  primitive.
- `domain-primitives.ds-key-value-map`: reusable finite key/value map primitive.
- `domain-primitives.ds-fifo-queue`: reusable bounded FIFO queue primitive.
- `domain-primitives.ds-ring-buffer`: reusable circular-buffer admission and
  indexing primitive.
- `domain-primitives.ds-priority-queue`: reusable priority-order heap primitive.
- `domain-primitives.ds-union-find`: reusable disjoint-set connectivity primitive.

## Impact

- **Files**: OpenSpec artifacts plus
  `src/ds_bounded_array.rs`, `src/ds_key_value_map.rs`, `src/ds_fifo_queue.rs`,
  `src/ds_ring_buffer.rs`, `src/ds_priority_queue.rs`, `src/ds_union_find.rs`,
  `src/lib.rs`, `docs/requirements.md`, tests, and evidence.
- **APIs**: adds focused `src/ds_*` exact finite admission/checker APIs.
- **Dependencies**: no new runtime dependencies.
- **Testing**: focused cargo tests, Verus, Tracey, OpenSpec validation, and repo validation.

## Out of Scope

- probabilistic, concurrent, lock-free, persistent, or external-storage data
  structures
- asymptotic-complexity or hardware-performance claims
- graph algorithms beyond union-find connectivity admission
