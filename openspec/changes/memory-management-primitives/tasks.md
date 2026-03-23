## Phase 1: Region Arithmetic

- [x] Add requirements to `docs/requirements.md` for region module ✅
- [x] Create `src/region.rs` with `region_contains_addr` (spec + exec + proof + tests) ✅
- [x] Add `regions_disjoint` with symmetry proof ✅
- [x] Add `region_contains_region` with reflexivity and transitivity proofs ✅
- [x] Add `region_adjacent` with symmetry proof ✅
- [x] Add `region_merge` with merge-of-split roundtrip proof ✅
- [x] Add `region_split` with adjacent-halves and size-sum proofs ✅
- [x] Add `pub mod region;` to `src/lib.rs` ✅
- [x] Add cross-module properties: region ↔ interval agreement (contains, overlaps, adjacent) ✅
- [x] Run `./scripts/verify.sh` — all gates pass ✅ (1367 verified, 0 errors)

## Phase 2: Slab Allocation Arithmetic

- [ ] Add requirements to `docs/requirements.md` for slab module
- [ ] Create `src/slab.rs` with `slot_index` (spec + exec + proof + tests)
- [ ] Add `slot_address` with roundtrip proof (slot_address ∘ slot_index on aligned addrs)
- [ ] Add `is_slot_aligned` with alignment-iff-zero-offset proof
- [ ] Add `slab_capacity` with bounded-index proof
- [ ] Add `generation_valid` and `generation_increment` with monotonicity proof
- [ ] Add `handle_pack`, `handle_unpack_index`, `handle_unpack_generation` with roundtrip proofs
- [ ] Add `pub mod slab;` to `src/lib.rs`
- [ ] Add cross-module properties: slab ↔ page decomposition duality
- [ ] Run `./scripts/verify.sh` — all gates pass

## Phase 3: Buddy System Arithmetic

- [ ] Add requirements to `docs/requirements.md` for buddy module
- [ ] Create `src/buddy.rs` with `buddy_of` (spec + exec + proof + tests)
- [ ] Add involution proof: buddy_of(buddy_of(addr, order), order) == addr
- [ ] Add `block_size` with power-of-two proof
- [ ] Add `order_for_size` with minimality proof
- [ ] Add `can_coalesce` predicate
- [ ] Add `parent_block` with parent-alignment proof
- [ ] Add `is_aligned_to_order` with alignment predicate
- [ ] Add `pub mod buddy;` to `src/lib.rs`
- [ ] Add cross-module properties: buddy ↔ integer (log2, next_pow2), bitwise (toggle)
- [ ] Run `./scripts/verify.sh` — all gates pass

## Phase 4: Watermark Allocation

- [ ] Add requirements to `docs/requirements.md` for watermark module
- [ ] Create `src/watermark.rs` with `watermark_alloc` (spec + exec + proof + tests)
- [ ] Add `watermark_after_alloc` with monotonicity proof
- [ ] Add `watermark_remaining` with capacity-conservation proof (remaining + used = capacity)
- [ ] Add `watermark_fits` as read-only check agreeing with alloc
- [ ] Add `watermark_reset` with reclamation proof
- [ ] Add `watermark_is_empty` predicate
- [ ] Add non-overlap proof: two successive allocations return disjoint ranges
- [ ] Add `pub mod watermark;` to `src/lib.rs`
- [ ] Add cross-module properties: watermark ↔ monotonic (advance), integer (align_up)
- [ ] Run `./scripts/verify.sh` — all gates pass
