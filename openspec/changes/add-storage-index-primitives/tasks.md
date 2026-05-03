## Coverage Index

| Requirement ID | Future module |
|---|---|
| `storage.page-range` | `src/storage_page.rs` |
| `storage.record-slot-table` | `src/storage_slot.rs` |
| `storage.key-range` | `src/storage_key_range.rs` |
| `storage.cursor-bounds` | `src/storage_cursor.rs` |
| `storage.segment-map` | `src/storage_segment.rs` |
| `storage.lookup-result` | `src/storage_lookup.rs` |
| `storage.compaction-envelope` | `src/storage_compaction.rs` |
| `storage.versioned-slot` | `src/storage_versioned_slot.rs` |

## Phase 1: Spec foundation

- [x] Add focused storage/index primitive OpenSpec with exact finite requirements.

## Phase 2: Page, slot, and key-range primitives

- [ ] Implement `storage_page` finite page ID/range admission with checked constructors, no-mutation rejection tests, and Verus bounds proofs. `[covers=storage.page-range]`
- [ ] Implement `storage_slot` finite record-slot table metadata with occupied/free state proofs. `[covers=storage.record-slot-table]`
- [ ] Implement `storage_key_range` ordered half-open key range membership and overlap/disjointness proofs. `[covers=storage.key-range]`

## Phase 3: Cursor, segment, and lookup primitives

- [ ] Implement `storage_cursor` checked cursor bounds and monotone movement proofs. `[covers=storage.cursor-bounds]`
- [ ] Implement `storage_segment` logical-to-physical finite segment map admission and span coverage proofs. `[covers=storage.segment-map]`
- [ ] Implement `storage_lookup` present/absent/out-of-range lookup-result admission and deterministic decision tests. `[covers=storage.lookup-result]`

## Phase 4: Rewrite/version primitives

- [ ] Implement `storage_compaction` finite remap envelope preserving live logical records while allowing physical span changes. `[covers=storage.compaction-envelope]`
- [ ] Implement `storage_versioned_slot` latest-visible finite-version selection with deterministic tie/rejection semantics. `[covers=storage.versioned-slot]`

## Phase 5: Evidence and archive

- [ ] Add `docs/requirements.md`, `r[impl]`, `r[depends]`, and `r[verify]` coverage for every storage/index primitive requirement.
- [ ] Run focused cargo tests for storage/index primitives and capture evidence.
- [ ] Run `./scripts/verus.sh src/lib.rs` and capture evidence.
- [ ] Run Tracey uncovered/untested checks and final repo validation.
- [ ] Sync OpenSpec deltas and archive `add-storage-index-primitives`.
