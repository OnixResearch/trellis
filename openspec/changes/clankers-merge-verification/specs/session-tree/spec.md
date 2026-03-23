# Session Tree Specification

## Purpose

Thin composition layer that combines `forest` and `seq_set` primitives
to express clankers session-tree operations. Each function here is a
direct composition of generic primitives — the session-tree module
contains no novel algorithms, only domain-specific wiring.

These map 1:1 to clankers-session tree operations:
`find_unique_messages`, `find_divergence_point`, `find_all_leaves`,
`find_latest_leaf`, `selective_merge_set`, `collect_subtree`,
`merge_branch_count`.

## Dependencies

- `forest`: well_formed, ancestor_path, lca, is_leaf, leaves, subtree
- `seq_set`: set_difference, set_intersection, is_member

## Definitions

Inherits all definitions from `forest` and `seq_set` specs.

- **Unique messages**: `set_difference(ancestor_path(src), ancestor_path(tgt))`
  — nodes on the source branch that are not on the target branch.
- **Divergence point**: `lca(parents, leaf_a, leaf_b)` — direct alias.
- **Selective merge set**: `set_intersection(selected, unique)` —
  nodes the user chose that are actually unique to the source branch.
- **Merge branch count**: `set_difference(ancestor_path(src), ancestor_path(tgt)).len()`.

## Requirements

### Requirement: Unique Messages Correctness

`find_unique(parents, src, tgt)` returns
`set_difference(ancestor_path(parents, src), ancestor_path(parents, tgt))`.

A node is in the result iff it is on the source branch path AND not
on the target branch path.

#### Scenario: Forked branches

- GIVEN parents `[None, Some(0), Some(0), Some(1), Some(2)]`
  (branch A: 0→1→3, branch B: 0→2→4)
- WHEN finding unique from leaf 3 to leaf 4
- THEN result is [1, 3] (on A's path, not on B's)

#### Scenario: Ancestor to descendant

- GIVEN parents `[None, Some(0), Some(1)]`
- WHEN finding unique from leaf 1 to leaf 2
- THEN result is [] (node 1 is on both paths)

### Requirement: Divergence Point Correctness

`find_divergence(parents, a, b)` equals `lca(parents, a, b)`.

Properties inherited from forest::lca: result is ancestor of both,
result is maximal.

#### Scenario: Fork at root

- GIVEN parents `[None, Some(0), Some(0)]`
- THEN find_divergence(parents, 1, 2) = 0

#### Scenario: Fork mid-chain

- GIVEN parents `[None, Some(0), Some(1), Some(1)]`
- THEN find_divergence(parents, 2, 3) = 1

### Requirement: Selective Merge Correctness

`selective_merge(selected, unique)` equals
`set_intersection(selected, unique)`.

Result contains node `n` iff `n` is in `selected` AND `n` is in
`unique`.

#### Scenario: Partial selection

- GIVEN unique = [1, 3, 5] and selected = [1, 5]
- THEN result = [1, 5]

#### Scenario: Selection includes non-unique

- GIVEN unique = [1, 3] and selected = [1, 2, 3]
- THEN result = [1, 3]

### Requirement: Cherry-pick Subtree

`cherry_pick_subtree(parents, root)` equals
`subtree(parents, root)`.

Properties inherited from forest::subtree: soundness, completeness,
contains root, leaf subtree is singleton.

#### Scenario: Subtree of three

- GIVEN parents `[None, Some(0), Some(1), Some(1)]`
- WHEN cherry-picking node 1
- THEN result contains 1, 2, 3

#### Scenario: Sibling excluded

- GIVEN parents `[None, Some(0), Some(0)]`
- WHEN cherry-picking node 1
- THEN result = [1] (node 2 is a sibling, not a descendant)

### Requirement: Merge Branch Count

`merge_branch_count(parents, src, tgt)` equals
`find_unique(parents, src, tgt).len()`.

#### Scenario: Two unique messages

- GIVEN a tree where source branch has 2 nodes not on target branch
- THEN merge_branch_count = 2

### Requirement: Latest Leaf

`find_latest_leaf(parents)` returns the highest-indexed leaf. In a
well-formed forest where nodes are appended chronologically, this is
the most recently added leaf.

Result MUST satisfy `is_leaf(parents, result)`.

Result MUST be maximal: no leaf has a higher index.

#### Scenario: Branching tree

- GIVEN parents `[None, Some(0), Some(0), Some(1)]`
  (leaves are 2 and 3)
- THEN find_latest_leaf = 3
