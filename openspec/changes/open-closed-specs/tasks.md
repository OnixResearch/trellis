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
- [ ] Close `merge` exec fn — map_values bridging deferred (complex)
- [x] Verus + cargo test green ✅

## Phase 4: Version comparison (stretch)

- [ ] Open `compare_versions_spec` — recursive component-by-component comparison
- [ ] Close `compare_versions` exec fn
- [ ] Close `compare_versions_antisymmetric` proof
- [ ] Close `compare_versions_reflexive` proof
- [ ] Close `compare_versions_range` proof

## Remaining closed specs (4)

- chain::hash_spec — genuinely uninterpreted (models blake3)
- nix_attrset::list_to_attrs_spec — needs sort spec + dedup
- nix_string::replace_strings_spec — multi-pattern matching
- nix_version::compare_versions_spec — multi-phase byte-level comparison

## Remaining external_body (13)

### Genuine axioms / Z3 limits (3)
- chain::hash_deterministic, compute_entry_hash
- hash_bucket::pow2_mask_is_mod

### Nonlinear (3)
- rate_limit::tokens_after_refill, tokens_to_wait_ms, wait_enables_consumption

### Open spec but complex exec bridging (1)
- nix_attrset::merge — spec is open, exec needs map_values bridging

### Closed spec dependent (6)
- nix_attrset::list_to_attrs
- nix_string::replace_strings
- nix_version::compare_versions + 3 proofs
