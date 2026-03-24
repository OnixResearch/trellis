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

- [x] Add requirements to `docs/requirements.md` for slab module ✅
- [x] Create `src/slab.rs` with `slot_index` (spec + exec + proof + tests) ✅
- [x] Add `slot_address` with roundtrip proof (slot_address ∘ slot_index on aligned addrs) ✅
- [x] Add `is_slot_aligned` with alignment-iff-zero-offset proof ✅
- [x] Add `slab_capacity` with bounded-index proof ✅
- [x] Add `generation_valid` and `generation_increment` with monotonicity proof ✅
- [x] Add `handle_pack`, `handle_unpack_index`, `handle_unpack_generation` with roundtrip proofs ✅
- [x] Add `pub mod slab;` to `src/lib.rs` ✅
- [x] Add cross-module properties: slab ↔ page decomposition duality ✅
- [x] Run `./scripts/verify.sh` — all gates pass ✅ (1411 verified, 0 errors)

## Phase 3: Buddy System Arithmetic

- [x] Add requirements to `docs/requirements.md` for buddy module ✅
- [x] Create `src/buddy.rs` with `buddy_of` (spec + exec + proof + tests) ✅
- [x] Add involution proof: buddy_of(buddy_of(addr, order), order) == addr ✅
- [x] Add `block_size` with power-of-two proof ✅
- [x] Add `order_for_size` with minimality proof ✅ (exec external_body, proptest covers)
- [x] Add `can_coalesce` predicate ✅
- [x] Add `parent_block` with parent-alignment proof ✅
- [x] Add `is_aligned_to_order` with alignment predicate ✅
- [x] Add `pub mod buddy;` to `src/lib.rs` ✅
- [x] Add cross-module properties: buddy ↔ bitwise (toggle_bit), block_size == 1<<order ✅
- [x] Run `./scripts/verify.sh` — all gates pass ✅ (1447 verified, 0 errors)

## Phase 4: Watermark Allocation

- [x] Add requirements to `docs/requirements.md` for watermark module ✅
- [x] Create `src/watermark.rs` with `watermark_alloc` (spec + exec + proof + tests) ✅
- [x] Add `watermark_after_alloc` with monotonicity proof ✅
- [x] Add `watermark_remaining` with capacity-conservation proof (remaining + used = capacity) ✅
- [x] Add `watermark_fits` as read-only check agreeing with alloc ✅
- [x] Add `watermark_reset` with reclamation proof ✅
- [x] Add `watermark_is_empty` predicate ✅
- [x] Add non-overlap proof: two successive allocations return disjoint ranges ✅
- [x] Add `pub mod watermark;` to `src/lib.rs` ✅
- [x] Add cross-module properties: watermark ↔ monotonic (advance), region (disjoint) ✅
- [x] Run `./scripts/verify.sh` — all gates pass ✅ (1466 verified, 0 errors)
