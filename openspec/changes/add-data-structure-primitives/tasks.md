## Coverage Index

| Requirement ID | Module task |
|---|---|
| `domain-primitives.ds-bounded-array` | `ds_bounded_array` |
| `domain-primitives.ds-key-value-map` | `ds_key_value_map` |
| `domain-primitives.ds-fifo-queue` | `ds_fifo_queue` |
| `domain-primitives.ds-ring-buffer` | `ds_ring_buffer` |
| `domain-primitives.ds-priority-queue` | `ds_priority_queue` |
| `domain-primitives.ds-union-find` | `ds_union_find` |

## Phase 1: Spec foundation

- [x] Create focused data-structure primitive OpenSpec with finite models, validity-boundary, and evidence expectations.

## Phase 2: Sequence and associative primitives

- [x] Implement and prove `ds_bounded_array`; cover length/capacity validity, index admission, point update, and no-mutation-on-reject. `[covers=domain-primitives.ds-bounded-array]`
- [x] Implement and prove `ds_key_value_map`; cover finite key uniqueness, lookup/insert/remove semantics, replacement, and missing-key behavior. `[covers=domain-primitives.ds-key-value-map]`

## Phase 3: Queue and buffer primitives

- [x] Implement and prove `ds_fifo_queue`; cover bounded enqueue/dequeue order, overflow rejection, underflow rejection, and unchanged state on rejection. `[covers=domain-primitives.ds-fifo-queue]`
- [x] Implement and prove `ds_ring_buffer`; cover wraparound indexing, occupied-slot invariants, push/pop order, and full/empty rejection. `[covers=domain-primitives.ds-ring-buffer]`

## Phase 4: Ordered and connectivity primitives

- [x] Implement and prove `ds_priority_queue`; cover heap/order validity, deterministic pop-min with tie policy, bounded push, and empty-pop rejection. `[covers=domain-primitives.ds-priority-queue]`
- [x] Implement and prove `ds_union_find`; cover parent/rank validity, find-root bounds, union connectivity preservation, and invalid-index rejection. `[covers=domain-primitives.ds-union-find]`

## Phase 5: Validation and archive

- [x] Sync completed requirement text to `docs/requirements.md` and canonical `openspec/specs/domain-primitives/spec.md`.
- [x] Run focused cargo/Verus/Tracey evidence for implemented modules.
- [x] Run OpenSpec gates after implementation evidence is synced.
- [x] Archive only after implementation and validation tasks complete.
