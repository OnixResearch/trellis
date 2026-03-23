## Phase 1: seq_set — Set Operations over Sequences

- [ ] Add seq-set requirements to specs/seq-set/spec.md ✅ (done during decomposition)
- [ ] Implement `is_member_spec` + `is_member` exec (linear scan, returns bool)
- [ ] Prove empty_no_members: is_member([], x) == false
- [ ] Implement `set_difference_spec` + `set_difference` exec (boolean array filter)
- [ ] Prove difference_subset: every element in result is member of a
- [ ] Prove difference_disjoint: no element in result is member of b
- [ ] Prove difference_complete: member of a && !member of b ==> in result
- [ ] Implement `set_intersection_spec` + `set_intersection` exec
- [ ] Prove intersection_in_both: every element in result is member of a and b
- [ ] Prove intersection_complete: member of a && member of b ==> in result
- [ ] Implement `is_subset_spec` + `is_subset` exec
- [ ] Prove subset_reflexive, subset_transitive
- [ ] Prove empty_subset: is_subset([], b) for any b
- [ ] Implement `all_distinct_spec` + `all_distinct` exec
- [ ] Prove difference_intersection_partition: for distinct a, diff.len() + inter.len() == a.len()
- [ ] Prove difference_preserves_distinct, intersection_preserves_distinct
- [ ] Unit tests + proptest (random integer sequences, set algebra identities)
- [ ] Verus + cargo test green

## Phase 2: forest — Well-formedness and Structure

- [ ] Add forest requirements to specs/forest/spec.md ✅ (done during decomposition)
- [ ] Implement `well_formed_spec` + `well_formed` exec
- [ ] Implement `is_root_spec`, `is_leaf_spec` predicates
- [ ] Implement `children_of_spec` (recursive filter) + `build_children` exec
- [ ] Prove children_of_forward: parents[c] == Some(p) ==> c in children_of(Some(p))
- [ ] Prove children_of_reverse: c in children_of(p) ==> parents[c] == p
- [ ] Prove children_of_ordered: result is in ascending index order
- [ ] Unit tests + proptest (random well-formed parent arrays)
- [ ] Verus + cargo test green

## Phase 3: forest — Ancestor Path and Depth

- [ ] Implement `ancestor_path_spec` (recursive, decreases node) + `ancestor_path` exec
- [ ] Prove path_root_anchored: parents[result[0]] == None
- [ ] Prove path_valid: parents[result[i]] == Some(result[i-1]) for consecutive elements
- [ ] Prove path_contains_node: result[result.len()-1] == node
- [ ] Prove path_length_bound: result.len() <= parents.len()
- [ ] Prove path_elements_in_bounds: all elements 0 <= e < parents.len()
- [ ] Prove path_distinct: all_distinct(ancestor_path(parents, node))
- [ ] Implement `depth_spec` + `depth` exec
- [ ] Prove root_depth_zero: is_root ==> depth == 0
- [ ] Unit tests + proptest (linear chains, wide trees, single-node)
- [ ] Verus + cargo test green

## Phase 4: forest — Is-ancestor and LCA

- [ ] Implement `is_ancestor_spec` (membership in ancestor_path)
- [ ] Prove is_ancestor_reflexive, is_ancestor_transitive, is_ancestor_antisymmetric
- [ ] Implement `lca_spec` + `lca` exec (walk both paths, find last common element)
- [ ] Prove lca_is_ancestor_of_both: is_ancestor(lca, a) && is_ancestor(lca, b)
- [ ] Prove lca_maximal: no descendant of lca is ancestor of both a and b
- [ ] Prove lca_symmetric: lca(a, b) == lca(b, a)
- [ ] Prove lca_self: lca(a, a) == a
- [ ] Unit tests + proptest (forked trees, ancestor-descendant pairs, disjoint roots)
- [ ] Verus + cargo test green

## Phase 5: forest — Leaves and Subtree

- [ ] Implement `leaves_spec` + `leaves` exec (scan for childless nodes)
- [ ] Prove leaves_sound: every returned node satisfies is_leaf
- [ ] Prove leaves_complete: every is_leaf node is returned
- [ ] Implement `reachable_spec` (recursive predicate) + `subtree` exec (forward scan)
- [ ] Prove subtree_sound: every returned node is reachable from root
- [ ] Prove subtree_complete: every reachable node is returned
- [ ] Prove subtree_contains_root
- [ ] Prove leaf_subtree_singleton: is_leaf(root) ==> subtree == [root]
- [ ] Unit tests + proptest (varying branching factors, deep vs wide trees)
- [ ] Verus + cargo test green

## Phase 6: session_tree — Composition Layer

- [ ] Implement `find_unique` = set_difference(ancestor_path(src), ancestor_path(tgt))
- [ ] Implement `find_divergence` = lca(parents, a, b)
- [ ] Implement `selective_merge` = set_intersection(selected, unique)
- [ ] Implement `cherry_pick_subtree` = subtree(parents, root)
- [ ] Implement `merge_branch_count` = find_unique(...).len()
- [ ] Implement `find_latest_leaf` (backward scan for max-index leaf)
- [ ] Prove latest_leaf_is_leaf: is_leaf(result)
- [ ] Prove latest_leaf_maximal: no leaf has higher index
- [ ] Unit tests + proptest (random trees, merge scenarios)
- [ ] Verus + cargo test green

## Phase 7: Cross-module Properties

- [ ] Prove unique_subset_of_source: find_unique result ⊆ ancestor_path(src)
- [ ] Prove unique_disjoint_from_target: find_unique result ∩ ancestor_path(tgt) == []
- [ ] Prove selective_subset_of_unique: selective_merge result ⊆ find_unique result
- [ ] Prove subtree_walk_connection: ancestor_path(node) ⊆ subtree(root) when is_ancestor(root, node)
- [ ] Add cross-module property tests in src/properties.rs
- [ ] Final verus + tracey pass
