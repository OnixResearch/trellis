## Phase 1: Well-formedness and Tree Construction

- [ ] Add session-tree requirements to specs/session-tree/spec.md (well-formedness, index consistency)
- [ ] Implement `well_formed` spec predicate: bounds, no self-ref, strict ordering
- [ ] Implement `children_of` spec function: filter parent array for children of a given node
- [ ] Implement `build_children` exec function: single-pass Vec<Vec<usize>> construction
- [ ] Prove index consistency: `children_of(parents, Some(p))` contains `c` iff `parents[c] == Some(p)`
- [ ] Prove root consistency: `children_of(parents, None)` contains `c` iff `parents[c] == None`
- [ ] Prove partition: every node appears in exactly one children list
- [ ] Unit tests + proptest (random well-formed parent arrays)
- [ ] Verus + tracey clean

## Phase 2: Walk Branch

- [ ] Implement `walk_branch_spec` recursive spec function (parent-pointer trace, reversed)
- [ ] Implement `walk_branch` exec function (iterative loop with counter)
- [ ] Prove termination: walk completes in at most `parents.len()` steps
- [ ] Prove root-anchored: `result[0]` has parent `None`
- [ ] Prove path-valid: consecutive elements have parent→child relationship
- [ ] Prove leaf-present: `result[result.len() - 1] == leaf_id`
- [ ] Prove length bound: `result.len() <= parents.len()`
- [ ] Unit tests + proptest (linear chains, deep trees, single-node trees)
- [ ] Verus + tracey clean

## Phase 3: Set Operations on Branches

- [ ] Implement `find_unique_spec` spec function (set difference of two paths)
- [ ] Implement `find_unique` exec function (boolean membership array + filter)
- [ ] Prove correctness: result contains `n` iff `n` on source path AND `n` not on target path
- [ ] Prove subset: every element in result is on the source path
- [ ] Implement `find_divergence_spec` spec function (longest common prefix of two paths)
- [ ] Implement `find_divergence` exec function (parallel scan)
- [ ] Prove on-both-branches: result is on both walk paths
- [ ] Prove maximality: the node after divergence (if any) differs between branches
- [ ] Unit tests + proptest (forked trees, ancestor/descendant pairs, disjoint roots)
- [ ] Verus + tracey clean

## Phase 4: Leaf Enumeration

- [ ] Implement `is_leaf_spec` predicate: node has no children
- [ ] Implement `find_all_leaves_spec` spec function: all nodes where `is_leaf_spec` holds
- [ ] Implement `find_all_leaves` exec function (scan parent array, collect childless nodes)
- [ ] Prove soundness: every returned node has no children
- [ ] Prove completeness: every childless node is returned
- [ ] Implement `find_latest_leaf` exec function (follow last-child pointers)
- [ ] Prove result is a leaf
- [ ] Unit tests + proptest (trees with varying branching factors)
- [ ] Verus + tracey clean

## Phase 5: Merge and Cherry-pick Selection

- [ ] Implement `selective_merge_set_spec`: intersection of selected IDs with unique set
- [ ] Implement `selective_merge_set` exec function
- [ ] Prove: result == selected ∩ unique (both directions)
- [ ] Implement `reachable_spec` recursive predicate: transitive closure of children
- [ ] Implement `collect_subtree` exec function (iterative DFS with visited array)
- [ ] Prove subtree soundness: every collected node is reachable from source
- [ ] Prove subtree completeness: every reachable node is collected
- [ ] Implement `merge_branch_count_spec`: count equals `find_unique(source, target).len()`
- [ ] Prove count correctness
- [ ] Unit tests + proptest (random trees, random selections, subtree shapes)
- [ ] Verus + tracey clean

## Phase 6: Cross-module Properties and Integration

- [ ] Link `walk_branch` termination to existing `drain` module's bounded iteration pattern
- [ ] Link `find_unique` to `collections::find_first` / set membership patterns
- [ ] Link `is_leaf_spec` to `collections` membership predicates
- [ ] Add cross-module property tests in `src/properties.rs`
- [ ] Final `verus` pass on full module
- [ ] Final `tracey query uncovered` — zero uncovered requirements
