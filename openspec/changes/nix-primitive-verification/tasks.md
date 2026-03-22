## Phase 1: Version and String Primitives

- [ ] Add `nix_version` module: split_components, classify_component, compare_versions spec+exec
- [ ] Add `nix_version` ordering proofs: total, transitive, antisymmetric
- [ ] Add `nix_version` parse_drv_name: name-version split at first digit after hyphen
- [ ] Add proptest oracles for nix_version (test against known Nix version orderings)
- [ ] Add `nix_string` module: string_length, substring spec+exec
- [ ] Add `nix_string` replace_strings: left-to-right non-overlapping replacement
- [ ] Add proptest oracles for nix_string
- [ ] Add requirements to docs/requirements.md for nix-version and nix-string
- [ ] Add tracey annotations (r[depends], r[impl], r[verify]) for all Phase 1 functions
- [ ] Verify: `scripts/verus.sh` passes, `tracey query uncovered` clean

## Phase 2: Attribute Set Operations

- [ ] Add `nix_attrset` module: sorted-key invariant, attr_names, attr_values, has_attr, get_attr
- [ ] Add `nix_attrset` merge: two-pointer merge with right-precedence, sorted output proof
- [ ] Add `nix_attrset` merge associativity proof
- [ ] Add `nix_attrset` intersect_attrs: key intersection with values from second set
- [ ] Add `nix_attrset` intersect commutativity proof (on key sets)
- [ ] Add `nix_attrset` remove_attrs: filter by excluded key set
- [ ] Add `nix_attrset` list_to_attrs: sort-and-deduplicate constructor
- [ ] Add `nix_attrpath` module: attr_path_lookup, attr_path_exists
- [ ] Add proptest oracles for nix_attrset and nix_attrpath
- [ ] Add requirements to docs/requirements.md for nix-attrset and nix-attrpath
- [ ] Add tracey annotations for all Phase 2 functions
- [ ] Verify: `scripts/verus.sh` passes, `tracey query uncovered` clean

## Phase 3: List Operations

- [ ] Add `nix_list` module: map, filter, foldl spec+exec
- [ ] Add `nix_list` concat_lists, gen_list, elem, elem_at, head, tail, length
- [ ] Add `nix_list` functor identity proof: map(list, id) == list
- [ ] Add `nix_list` concat associativity proof
- [ ] Add `nix_sort` module: sort spec+exec with comparator
- [ ] Add `nix_sort` permutation proof: output is permutation of input
- [ ] Add `nix_sort` idempotent proof: sort(sort(list)) == sort(list)
- [ ] Add `nix_partition` module: partition spec+exec with predicate
- [ ] Add `nix_partition` length conservation proof
- [ ] Add `nix_partition` filter duality proof
- [ ] Add proptest oracles for nix_list, nix_sort, nix_partition
- [ ] Add requirements to docs/requirements.md for nix-list, nix-sort, nix-partition
- [ ] Add tracey annotations for all Phase 3 functions
- [ ] Verify: `scripts/verus.sh` passes, `tracey query uncovered` clean

## Phase 4: Generic Closure

- [ ] Add `nix_closure` module: generic_closure spec with fuel-based termination
- [ ] Add `nix_closure` exec implementation with key-set tracking
- [ ] Add `nix_closure` monotonicity proof: key set grows monotonically
- [ ] Add `nix_closure` idempotent proof: closure(closure(S)) == closure(S)
- [ ] Add `nix_closure` contains-start proof: start_set ⊆ result
- [ ] Add proptest oracles for nix_closure
- [ ] Add requirements to docs/requirements.md for nix-closure
- [ ] Add tracey annotations for all Phase 4 functions
- [ ] Verify: `scripts/verus.sh` passes, `tracey query uncovered` clean

## Phase 5: Cross-Module Properties and Integration

- [ ] Add cross-module property: nix_version ordering consistent with ordering.sign
- [ ] Add cross-module property: nix_attrset.merge singleton reduces to collections.sorted_insert
- [ ] Add cross-module property: nix_partition/nix_list.filter duality
- [ ] Add cross-module property: nix_list.foldl over merge computes chained //
- [ ] Add cross-module requirements to docs/requirements.md
- [ ] Add all modules to src/lib.rs with doc comments
- [ ] Final verification pass: verus, tracey, clippy, tests all green
- [ ] Update .agent/napkin.md with Nix module lessons learned
