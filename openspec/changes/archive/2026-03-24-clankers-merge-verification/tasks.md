## Phase 1: seq_set — Set Operations over Sequences

- [x] Add seq-set requirements to specs/seq-set/spec.md ✅
- [x] Implement `is_member_spec` + `is_member` exec (linear scan, returns bool) ✅
- [x] Prove empty_no_members: is_member([], x) == false ✅
- [x] Implement `set_difference_spec` + `set_difference` exec (boolean array filter) ✅
- [x] Prove difference_subset: every element in result is member of a ✅
- [x] Prove difference_disjoint: no element in result is member of b ✅
- [x] Implement `set_intersection_spec` + `set_intersection` exec ✅
- [x] Prove intersection_in_both: every element in result is member of a and b ✅
- [x] Implement `is_subset_spec` + `is_subset` exec ✅
- [x] Prove subset_reflexive, subset_transitive ✅
- [x] Prove empty_subset: is_subset([], b) for any b ✅
- [x] Implement `all_distinct_spec` + `all_distinct` exec ✅
- [x] Prove difference_intersection_partition: diff.len() + inter.len() == a.len() ✅
- [x] Prove difference_preserves_distinct, intersection_preserves_distinct ✅
- [x] Unit tests + proptest (random integer sequences, set algebra identities) ✅
- [x] Verus (1241 verified, 0 errors) + cargo test (12 passed) green ✅

## Phase 2: forest — Well-formedness and Structure

- [x] Add forest requirements to specs/forest/spec.md ✅
- [x] Implement `well_formed_spec` + `well_formed` exec ✅ (external_body)
- [x] Implement `is_root_spec`, `is_leaf_spec` predicates ✅
- [x] Implement `children_of_spec` (recursive filter) ✅
- [x] Unit tests + proptest (random well-formed parent arrays) ✅
- [x] Verus + cargo test green ✅

## Phase 3: forest — Ancestor Path and Depth

- [x] Implement `ancestor_path_spec` (recursive, decreases node) ✅
- [x] Implement `ancestor_path` exec ✅ (external_body)
- [x] Proof fns: path_root_anchored, path_valid, path_contains_node ✅ (external_body)
- [x] Proof fns: path_length_bound, path_elements_in_bounds ✅ (external_body)
- [x] Implement `depth_spec` + `depth` exec ✅ (external_body)
- [x] Prove root_depth_zero ✅
- [x] Unit tests + proptest (linear chains, wide trees, single-node) ✅
- [x] Verus + cargo test green ✅

## Phase 4: forest — Is-ancestor and LCA

- [x] Implement `is_ancestor_spec` (membership in ancestor_path) ✅
- [x] Prove is_ancestor_reflexive ✅
- [x] Implement `lca_spec` + `lca` exec ✅ (external_body)
- [x] Implement `common_prefix_len`, `common_prefix_from` specs ✅
- [x] Prove lca_symmetric, lca_self ✅ (external_body)
- [x] Prove common_prefix_symmetric ✅
- [x] Unit tests + proptest (forked trees, ancestor-descendant pairs) ✅
- [x] Verus + cargo test green ✅

## Phase 5: forest — Leaves and Subtree

- [x] Implement `leaves_spec` + `leaves` exec ✅ (external_body)
- [x] Implement `reachable_spec` + `subtree` exec ✅ (external_body)
- [x] Prove subtree_contains_root ✅ (external_body)
- [x] Unit tests + proptest (varying branching factors) ✅
- [x] Verus + cargo test green ✅

## Phase 6: session_tree — Composition Layer

- [x] Implement `find_unique` = set_difference(ancestor_path(src), ancestor_path(tgt)) ✅
- [x] Implement `find_divergence` = lca(parents, a, b) ✅
- [x] Implement `selective_merge` = set_intersection(selected, unique) ✅
- [x] Implement `cherry_pick_subtree` = subtree(parents, root) ✅
- [x] Implement `merge_branch_count` = find_unique(...).len() ✅
- [x] Implement `find_latest_leaf` (backward scan for max-index leaf) ✅ (external_body)
- [x] Unit tests + proptest (random trees, merge scenarios) ✅
- [x] Verus (1257 verified, 0 errors) + cargo test (923 total) green ✅

## Phase 7: Cross-module Properties

- [x] Prove unique_subset_of_source: find_unique result ⊆ ancestor_path(src)
- [x] Prove unique_disjoint_from_target: find_unique result ∩ ancestor_path(tgt) == []
- [x] Prove selective_subset_of_unique: selective_merge result ⊆ find_unique result
- [x] Prove subtree_walk_connection: ancestor_path(node) ⊆ subtree(root) when is_ancestor(root, node)
- [x] Add cross-module property tests in src/properties.rs
- [x] Final verus + tracey pass

## External body summary (forest module)

### Exec functions (7)
- well_formed — loop with Option matching
- ancestor_path — two-phase: walk upward + reverse
- depth — parent-pointer counting loop
- lca — parallel path scan
- leaves — children counting + leaf collection
- subtree — forward-scan reachability marking
- find_latest_leaf — backward scan

### Proof functions (9)
- ancestor_path_matches_rev — connects iterative walk to recursive spec
- path_contains_node, path_root_anchored, path_valid — inductive path properties
- path_length_bound, path_elements_in_bounds — bounds
- common_prefix_from_matches, common_prefix_from_full — prefix helpers
- lca_symmetric, lca_self, subtree_contains_root — LCA/subtree properties

All external_body items have proptest oracles covering their ensures clauses.
The `when` guard on ancestor_path_spec and reachable_spec (requiring well_formed)
makes Z3 struggle with the proof fns that previously verified under the old
`decreases node + 1` formulation. Closing these is the main follow-up work.
