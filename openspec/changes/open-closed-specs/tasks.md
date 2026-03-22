## Phase 1: Simple scans and filters

- [ ] Open `parse_drv_name_spec` — recursive helper scanning for '-' + digit
- [ ] Close `parse_drv_name` exec fn — loop with invariant matching recursive spec
- [ ] Open `remove_attrs_spec` — recursive filter checking key membership
- [ ] Close `remove_attrs` exec fn — loop with filter invariant
- [ ] Open `intersect_attrs_spec` — recursive filter checking key in other set
- [ ] Close `intersect_attrs` exec fn — loop with filter invariant
- [ ] Verus + cargo test green

## Phase 2: Ceiling log and gossip convergence

- [ ] Open `ceil_log_spec` — recursive with division, decreases via nonlinear_arith
- [ ] Close `convergence_rounds` exec fn — loop matching ceil_log_spec
- [ ] Verus + cargo test green

## Phase 3: Merge with lexicographic comparison

- [ ] Add `lex_lt_spec` helper — lexicographic less-than on Seq<u8>
- [ ] Open `merge_spec` — recursive two-pointer merge using lex_lt_spec
- [ ] Close `merge` exec fn — three-loop pattern matching recursive spec
- [ ] Verus + cargo test green

## Phase 4: Version comparison (stretch)

- [ ] Open `compare_versions_spec` — recursive component-by-component comparison
- [ ] Close `compare_versions` exec fn
- [ ] Close `compare_versions_antisymmetric` proof
- [ ] Close `compare_versions_reflexive` proof
- [ ] Close `compare_versions_range` proof
- [ ] Verus + cargo test green

## Deferred

- replace_strings_spec — multi-pattern matching too complex for spec
- list_to_attrs_spec — requires sort spec + dedup, out of effort scope
