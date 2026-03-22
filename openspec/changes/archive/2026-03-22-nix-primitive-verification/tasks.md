## Phase 1: Version and String Primitives

- [x] Add `nix_version` module: split_components, classify_component, compare_versions spec+exec ✅
- [x] Add `nix_version` ordering proofs: total, transitive, antisymmetric ✅
- [x] Add `nix_version` parse_drv_name: name-version split at first digit after hyphen ✅
- [x] Add proptest oracles for nix_version (test against known Nix version orderings) ✅
- [x] Add `nix_string` module: string_length, substring spec+exec ✅
- [x] Add `nix_string` replace_strings: left-to-right non-overlapping replacement ✅
- [x] Add proptest oracles for nix_string ✅
- [x] Add requirements to docs/requirements.md for nix-version and nix-string ✅
- [x] Add tracey annotations (r[depends], r[impl], r[verify]) for all Phase 1 functions ✅
- [x] Verify: `scripts/verus.sh` passes, `tracey query uncovered` clean ✅

## Phase 2: Attribute Set Operations

- [x] Add `nix_attrset` module: sorted-key invariant, attr_names, attr_values, has_attr, get_attr ✅
- [x] Add `nix_attrset` merge: two-pointer merge with right-precedence, sorted output proof ✅
- [x] Add `nix_attrset` merge associativity proof ✅
- [x] Add `nix_attrset` intersect_attrs: key intersection with values from second set ✅
- [x] Add `nix_attrset` intersect commutativity proof (on key sets) ✅
- [x] Add `nix_attrset` remove_attrs: filter by excluded key set ✅
- [x] Add `nix_attrset` list_to_attrs: sort-and-deduplicate constructor ✅
- [x] Add `nix_attrpath` module: attr_path_lookup, attr_path_exists ✅
- [x] Add proptest oracles for nix_attrset and nix_attrpath ✅
- [x] Add requirements to docs/requirements.md for nix-attrset and nix-attrpath ✅
- [x] Add tracey annotations for all Phase 2 functions ✅
- [x] Verify: `scripts/verus.sh` passes, `tracey query uncovered` clean ✅

## Phase 3: List Operations

- [x] Add `nix_list` module: map, filter, foldl spec+exec ✅
- [x] Add `nix_list` concat_lists, gen_list, elem, elem_at, head, tail, length ✅
- [x] Add `nix_list` functor identity proof: map(list, id) == list ✅
- [x] Add `nix_list` concat associativity proof ✅
- [x] Add `nix_sort` module: sort spec+exec with comparator ✅
- [x] Add `nix_sort` permutation proof: output is permutation of input ✅
- [x] Add `nix_sort` idempotent proof: sort(sort(list)) == sort(list) ✅
- [x] Add `nix_partition` module: partition spec+exec with predicate ✅
- [x] Add `nix_partition` length conservation proof ✅
- [x] Add `nix_partition` filter duality proof ✅
- [x] Add proptest oracles for nix_list, nix_sort, nix_partition ✅
- [x] Add requirements to docs/requirements.md for nix-list, nix-sort, nix-partition ✅
- [x] Add tracey annotations for all Phase 3 functions ✅
- [x] Verify: `scripts/verus.sh` passes, `tracey query uncovered` clean ✅

## Phase 4: Generic Closure

- [x] Add `nix_closure` module: generic_closure spec with fuel-based termination ✅
- [x] Add `nix_closure` exec implementation with key-set tracking ✅
- [x] Add `nix_closure` monotonicity proof: key set grows monotonically ✅
- [x] Add `nix_closure` idempotent proof: closure(closure(S)) == closure(S) ✅
- [x] Add `nix_closure` contains-start proof: start_set ⊆ result ✅
- [x] Add proptest oracles for nix_closure ✅
- [x] Add requirements to docs/requirements.md for nix-closure ✅
- [x] Add tracey annotations for all Phase 4 functions ✅
- [x] Verify: `scripts/verus.sh` passes, `tracey query uncovered` clean ✅

## Phase 5: Cross-Module Properties and Integration

- [x] Add cross-module property: nix_version ordering consistent with ordering.sign ✅
- [x] Add cross-module property: nix_attrset.merge singleton reduces to collections.sorted_insert ✅
- [x] Add cross-module property: nix_partition/nix_list.filter duality ✅
- [x] Add cross-module property: nix_list.foldl over merge computes chained // ✅
- [x] Add cross-module requirements to docs/requirements.md ✅
- [x] Add all modules to src/lib.rs with doc comments ✅
- [x] Final verification pass: verus, tracey, clippy, tests all green ✅
- [x] Update .agent/napkin.md with Nix module lessons learned ✅
