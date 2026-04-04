## Phase 1: Sign-Extension Check + Radix Index (reusable primitives)

- [x] Add requirements to `docs/requirements.md` for bitwise.is-sign-extended ✅
- [x] Add `is_sign_extended(value, width)` to `src/bitwise.rs` (spec + exec + proof) ✅ (exec external_body — variable-shift overflow, same class as order_for_size)
- [x] Add sign-extended-full proof: is_sign_extended(v, 64) always true ✅
- [x] Add sign-extended-zero and sign-extended-all-ones proofs ✅ (external_body — variable shift in spec, covered by proptest)
- [x] Add proptest: random values and widths, verify monotonicity, full-width, model oracle ✅ (6 proptests + 1 unit test)
- [x] Add requirements to `docs/requirements.md` for page.radix-index ✅
- [x] Add `radix_index(value, level, bits_per_level, base_offset)` to `src/page.rs` (spec + exec + proof) ✅
- [x] Add radix-index-bounded proof: result < 2^bits_per_level ✅
- [x] Add radix-index-bounded-9bit proof: result < 512 for Sv39 VPN fields ✅
- [x] Add proptest: random values, verify boundedness and Sv39 roundtrip ✅ (3 proptests + 1 unit test)
- [x] Verus single-file verification: bitwise.rs 21 verified 0 errors, page.rs passes in full-crate ✅
- [x] Full-crate Verus: 0 new errors introduced (19 pre-existing from other modules) ✅

## Phase 2: PTE Parse/Construct

- [ ] Add requirements to `docs/requirements.md` for pte module
- [ ] Create `src/pte.rs` with `PteFields` struct and `parse_pte` (spec + exec + proof + tests)
- [ ] Add `construct_pte` with bitfield::insert_field at each offset
- [ ] Add construct-then-parse roundtrip proof (all fields recovered)
- [ ] Add parse-then-construct roundtrip proof (word recovered when reserved=0)
- [ ] Add `is_leaf_pte` with R|W|X != 0 detection proof
- [ ] Add `has_reserved_bits` (bits 54–63 nonzero check)
- [ ] Add `is_valid_permission_bits` (W=1,R=0 rejection)
- [ ] Add `pub mod pte;` to `src/lib.rs`
- [ ] Cross-module: parse_pte fields agree with bitfield::extract_field at specified offsets
- [ ] Proptest: random u64 → parse → construct → equals original (when reserved=0)
- [ ] Run `./scripts/verify.sh` — all gates pass

## Phase 3: Virtual Address Decomposition

- [ ] Add requirements to `docs/requirements.md` for vpn module
- [ ] Create `src/vpn.rs` with `vpn_extract_sv39` delegating to page::radix_index (spec + exec + proof + tests)
- [ ] Add `vpn_extract_sv48` for 4-level paging via radix_index
- [ ] Add decomposition roundtrip proof (delegates to radix_index_roundtrip)
- [ ] Add field boundedness proofs (delegates to radix_index_bounded)
- [ ] Add `is_canonical_sv39` delegating to bitwise::is_sign_extended(va, 39)
- [ ] Add `is_canonical_sv48` delegating to bitwise::is_sign_extended(va, 48)
- [ ] Add `pub mod vpn;` to `src/lib.rs`
- [ ] Cross-module: vpn offset agrees with page::page_offset(va, 4096)
- [ ] Proptest: random VA → extract → recombine → matches original masked address
- [ ] Run `./scripts/verify.sh` — all gates pass

## Phase 4: PTE Permission Checking

- [ ] Add requirements to `docs/requirements.md` for pte_permission module
- [ ] Create `src/pte_permission.rs` with `check_pte_access` (spec + exec + proof + tests)
- [ ] Add `check_pte_privilege` for U-bit vs privilege mode
- [ ] Add permission monotonicity proof: adding R/W/X bits never denies access
- [ ] Add MXR widening proof: MXR=true allows >= MXR=false
- [ ] Add SUM widening proof: SUM=true allows >= SUM=false
- [ ] Add `pub mod pte_permission;` to `src/lib.rs`
- [ ] Cross-module: R=1,W=1,X=1 allows all access types (matches FULL mode)
- [ ] Proptest: random PTE fields + access types, verify monotonicity and widening
- [ ] Run `./scripts/verify.sh` — all gates pass

## Phase 5: Dirty/Accessed Bit Management

- [ ] Add requirements to `docs/requirements.md` for dirty/accessed
- [ ] Add `update_accessed`, `update_dirty`, `clear_accessed_dirty` to `src/pte.rs`
- [ ] Add dirty-implies-accessed invariant proof
- [ ] Add idempotency proofs: update_accessed(update_accessed(pte)) == update_accessed(pte)
- [ ] Add update_dirty sets both D and A proof
- [ ] Add clear_accessed_dirty zeroes both bits proof
- [ ] Proptest: random PTEs, verify D=1 → A=1 after every update operation
- [ ] Run `./scripts/verify.sh` — all gates pass

## Phase 6: Walk-Level PTE Interpretation (reusable primitive)

- [ ] Add requirements to `docs/requirements.md` for walk_level module
- [ ] Create `src/walk_level.rs` with `pte_addr(table_base, vpn_index)` (spec + exec + proof + tests)
- [ ] Add `interpret_pte(pte_fields, level)` → StepResult (leaf/non-leaf/fault)
- [ ] Add `leaf_pa(ppn, level, va)` → physical address with level-dependent offset
- [ ] Add `next_table_base(ppn)` → ppn * 4096 with page-alignment proof
- [ ] Add `is_superpage_aligned(ppn, level)` predicate
- [ ] Add `superpage_size(level)` → page size with power-of-two proof
- [ ] Add pte-addr-bounded proof: result within [table_base, table_base + 4096)
- [ ] Add leaf-pa-in-region proof: PA ∈ [ppn_base, ppn_base + page_size)
- [ ] Add `pub mod walk_level;` to `src/lib.rs`
- [ ] Cross-module: leaf_pa satisfies region::contains_addr
- [ ] Proptest: random PPN/level/VA combos, verify PA in expected region
- [ ] Run `./scripts/verify.sh` — all gates pass

## Phase 7: Bounded ID Pool (reusable primitive)

- [ ] Add requirements to `docs/requirements.md` for bounded_id_pool module
- [ ] Create `src/bounded_id_pool.rs` with `pool_allocate` (spec + exec + proof + tests)
- [ ] Add `pool_deallocate` returning ID to free set
- [ ] Add `pool_contains` membership check
- [ ] Add `pool_active_count` with increment/decrement proofs
- [ ] Add `pool_is_exhausted` predicate
- [ ] Add unique proof: allocate never returns already-allocated ID
- [ ] Add bounded proof: active_count ≤ max_id, ID 0 never allocated
- [ ] Add recycle proof: deallocated ID eligible for reuse
- [ ] Add `pub mod bounded_id_pool;` to `src/lib.rs`
- [ ] Proptest: allocation/deallocation sequences, verify uniqueness and bounds
- [ ] Run `./scripts/verify.sh` — all gates pass

## Phase 8: Bounded Tagged Cache (reusable primitive)

- [ ] Add requirements to `docs/requirements.md` for tagged_lookup module
- [ ] Create `src/tagged_lookup.rs` with `tagged_lookup(entries, tag, key)` (spec + exec + proof + tests)
- [ ] Add `tagged_insert(entries, tag, key, value, capacity)` with eviction
- [ ] Add `tagged_remove_by_tag(entries, tag)` removing all matching tag
- [ ] Add `tagged_remove_by_key(entries, key)` removing all matching key
- [ ] Add `tagged_remove_all(entries)` clearing everything
- [ ] Add `tagged_count(entries)` entry count
- [ ] Add insert-lookup-roundtrip proof: insert then lookup returns inserted value
- [ ] Add remove-complete proof: remove_by_tag leaves no matching entries
- [ ] Add capacity-bounded proof: count ≤ capacity after any operation
- [ ] Add remove-all proof: count == 0 after clear
- [ ] Add `pub mod tagged_lookup;` to `src/lib.rs`
- [ ] Proptest: insert/lookup/remove sequences, verify roundtrip and capacity
- [ ] Run `./scripts/verify.sh` — all gates pass

## Phase 9: Page Table Walk

- [ ] Add requirements to `docs/requirements.md` for page_walk module
- [ ] Create `src/page_walk.rs` with `walk_sv39` composing walk_level (spec + exec + proof + tests)
- [ ] Add `walk_sv48` extending with 4th level
- [ ] Add determinism proof: same inputs + same tables → same result
- [ ] Add bounded-steps proof: terminates in ≤ 3 steps (Sv39) / ≤ 4 steps (Sv48)
- [ ] Add PA-in-region proof (delegates to walk_level::leaf_pa_in_region)
- [ ] Add `pub mod page_walk;` to `src/lib.rs`
- [ ] Cross-module: walk delegates to walk_level at each step, PA satisfies region::contains_addr
- [ ] Proptest: construct valid page tables, walk, verify PA in expected region
- [ ] Run `./scripts/verify.sh` — all gates pass

## Phase 10: Page Fault Classification

- [ ] Add requirements to `docs/requirements.md` for page_fault module
- [ ] Create `src/page_fault.rs` with `FaultReason` enum (5 variants) and `classify_fault`
- [ ] Add exhaustive proof: every failed walk maps to exactly one FaultReason
- [ ] Add exclusive proof: no walk produces two simultaneous reasons
- [ ] Add valid-implies-clear proof: successful walk → V=1, perms match, aligned, reserved=0
- [ ] Add `pub mod page_fault;` to `src/lib.rs`
- [ ] Cross-module: fault reasons partition walk outcomes
- [ ] Proptest: random PTE configurations, verify exactly one fault or success
- [ ] Run `./scripts/verify.sh` — all gates pass

## Phase 11: TLB Model

- [ ] Add requirements to `docs/requirements.md` for tlb module
- [ ] Create `src/tlb.rs` with `TlbEntry` struct and `tlb_lookup` delegating to tagged_lookup (spec + exec + proof + tests)
- [ ] Add global-entry matching: G=1 entries match any ASID on lookup
- [ ] Add `tlb_insert` delegating to tagged_insert
- [ ] Add `tlb_flush_asid` delegating to tagged_remove_by_tag, preserving G=1 entries
- [ ] Add `tlb_flush_vpn` delegating to tagged_remove_by_key
- [ ] Add `tlb_flush_full` delegating to tagged_remove_all
- [ ] Add soundness proof: lookup hit agrees with walk result (given unchanged tables)
- [ ] Add global-preserves proof: ASID flush does not remove G=1 entries
- [ ] Add capacity-bounded proof (delegates to tagged_lookup::capacity_bounded)
- [ ] Add `pub mod tlb;` to `src/lib.rs`
- [ ] Cross-module: tlb operations delegate to tagged_lookup, soundness agrees with page_walk
- [ ] Proptest: insert/lookup/flush sequences with global entries, verify soundness
- [ ] Run `./scripts/verify.sh` — all gates pass

## Phase 12: ASID Management

- [ ] Add requirements to `docs/requirements.md` for asid module
- [ ] Create `src/asid.rs` with `asid_allocate` delegating to bounded_id_pool (spec + exec + proof + tests)
- [ ] Add generation stamp to allocated ASIDs
- [ ] Add `asid_deallocate` delegating to pool_deallocate
- [ ] Add `asid_new_generation` with generation increment, pool reset, flush_needed = true
- [ ] Add `asid_switch` with stale-generation detection
- [ ] Add unique-in-generation proof (from pool uniqueness + generation stamp)
- [ ] Add pool-bounded proof (delegates to bounded_id_pool::bounded)
- [ ] Add generation-rollover-flush proof: new_generation → flush_needed
- [ ] Add `pub mod asid;` to `src/lib.rs`
- [ ] Cross-module: ASID generation follows slab::generation_valid pattern
- [ ] Proptest: allocation/deallocation/rollover sequences, verify uniqueness and flush
- [ ] Run `./scripts/verify.sh` — all gates pass

## Phase 13: Cross-Module Properties

- [ ] Add to `src/properties.rs`: pte fields agree with bitfield::extract_field
- [ ] Add to `src/properties.rs`: vpn uses page::radix_index
- [ ] Add to `src/properties.rs`: vpn canonical uses bitwise::is_sign_extended
- [ ] Add to `src/properties.rs`: vpn offset agrees with page::page_offset
- [ ] Add to `src/properties.rs`: walk_level leaf_pa satisfies region::contains_addr
- [ ] Add to `src/properties.rs`: page_walk delegates to walk_level per step
- [ ] Add to `src/properties.rs`: pte_permission full-access matches access_mode FULL
- [ ] Add to `src/properties.rs`: tlb operations delegate to tagged_lookup
- [ ] Add to `src/properties.rs`: tlb soundness agrees with page_walk
- [ ] Add to `src/properties.rs`: asid delegates to bounded_id_pool
- [ ] Add to `src/properties.rs`: asid generation follows slab generation pattern
- [ ] Add to `src/properties.rs`: fault reasons partition all walk outcomes
- [ ] Add cross-module requirements to `docs/requirements.md`
- [ ] Proptest for all cross-module properties
- [ ] Run `./scripts/verify.sh` — all gates pass
