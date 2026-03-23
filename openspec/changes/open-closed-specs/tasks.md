## Phase 1: Simple scans and filters

- [x] Open `parse_drv_name_spec` — recursive helper scanning for '-' + digit ✅
- [x] Close `parse_drv_name` exec fn — loop with invariant matching recursive spec ✅
- [x] Open `remove_attrs_spec` — recursive filter checking key membership ✅
- [x] Close `remove_attrs` exec fn — loop with filter invariant ✅
- [x] Open `intersect_attrs_spec` — recursive filter checking key in other set ✅
- [x] Close `intersect_attrs` exec fn — loop with filter invariant ✅
- [x] Verus + cargo test green ✅

## Phase 2: Ceiling log and gossip convergence

- [x] Open `ceil_log_spec` — fuel-bounded recursion with ceil_log_helper ✅
- [x] Close `convergence_rounds` exec fn — loop with unfold lemma ✅
- [x] Verus + cargo test green ✅

## Phase 3: Merge with lexicographic comparison

- [x] Add `seq_cmp`/`seq_lt` spec helpers — lexicographic byte comparison ✅
- [x] Open `merge_spec` — recursive two-pointer merge using seq_lt ✅
- [x] Add verified `vec_u8_cmp` exec fn matching seq_cmp spec ✅
- [x] Close `merge` exec fn — three-loop invariant with map_values bridging ✅
- [x] Add `seq_cmp_antisymmetric` proof — needed for merge spec unfolding ✅
- [x] Verus + cargo test green ✅

## Phase 4: Version comparison

- [x] Open `compare_versions_spec` — fuel-bounded recursive comparison ✅
- [x] Add helper specs: skip_seps, comp_end, parse_num_from, lex_cmp ✅
- [x] Add lex_cmp proofs: antisymmetric, reflexive, range ✅
- [x] Add skip_seps_bounds, comp_end_bounds proofs ✅
- [ ] Close `compare_versions_antisymmetric` proof — Z3 can't unfold deeply nested spec
- [ ] Close `compare_versions_reflexive` proof — same issue
- [ ] Close `compare_versions_range` proof — same issue
- [ ] Close `compare_versions` exec fn — complex loop structure

**Note:** The cv_fuel spec has 8+ levels of nested let+if/else. Z3 can't
unfold it deeply enough to verify the inductive proofs, even with high
rlimit. The proof structure (case split + recursive call at equal-value
branches) is correct — Z3 just runs out of quantifier instantiation
budget. These remain external_body with the structural proof bodies
preserved as documentation.

## Phase 5: list_to_attrs sort+dedup

- [x] Add `insert_into_sorted_spec` — recursive insertion using seq_cmp ✅
- [x] Open `list_to_attrs_spec` — fold via insert_into_sorted_spec ✅
- [x] Add `insert_into_sorted` exec fn — two-phase loop with early return ✅
- [x] Close `list_to_attrs` exec fn — iterative insertion sort ✅
- [x] Add proptest: sorted, unique, last-wins, completeness ✅
- [x] Verus + cargo test green ✅

## Remaining closed specs (2)

- chain::hash_spec — genuinely uninterpreted (models blake3)
- nix_string::replace_strings_spec — multi-pattern matching

## Remaining external_body (11)

### Genuine axioms / Z3 limits (3)
- chain::hash_deterministic, compute_entry_hash
- hash_bucket::pow2_mask_is_mod

### Nonlinear (3)
- rate_limit::tokens_after_refill, tokens_to_wait_ms, wait_enables_consumption

### Z3 unfolding limits (4)
- nix_version::cv_fuel_antisymmetric, cv_fuel_reflexive, cv_fuel_range
- nix_version::compare_versions

### Closed spec dependent (1)
- nix_string::replace_strings
