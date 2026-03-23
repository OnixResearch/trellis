# Session Tree Specification

## Purpose

Defines the correctness properties for branching conversation DAG
operations: tree construction, parent-pointer walks, set operations
on branches, leaf enumeration, and merge/cherry-pick logic. These
are extracted from the `clankers-session` tree module and verified
in isolation over abstract integer node IDs.

## Definitions

- **Node**: An entry in the DAG identified by an integer index
  (0 ≤ index < len).
- **Parent map**: A `Seq<Option<int>>` where `parents[i]` is the
  parent of node `i`, or `None` if `i` is a root.
- **Children map**: A `Map<Option<int>, Seq<int>>` mapping each
  parent (or `None` for roots) to its children indices.
- **Branch path**: The ordered sequence of nodes from root to a
  given leaf, following parent pointers upward then reversing.
- **Leaf**: A node with no children.
- **Divergence point (LCA)**: The last node shared by two branch
  paths before they diverge.

## Requirements

### Requirement: Parent Map Well-formedness

A parent map of length `n` is well-formed if every parent reference
points to a valid index strictly less than the referencing node.

The system MUST reject parent maps where a node's parent index is
out of bounds.

The system MUST reject parent maps where a node is its own parent.

#### Scenario: Valid parent map

- GIVEN a parent map `[None, Some(0), Some(0), Some(1)]`
- WHEN well-formedness is checked
- THEN the map is accepted (all parents < their child index)

#### Scenario: Self-referential parent

- GIVEN a parent map `[Some(0)]`
- WHEN well-formedness is checked
- THEN the map is rejected

#### Scenario: Forward reference

- GIVEN a parent map `[Some(1), None]` (node 0 points to node 1)
- WHEN well-formedness is checked
- THEN the map is rejected (parent index must be less than child)

### Requirement: Index Consistency

Building a children map from a well-formed parent map MUST produce
an index where every child's parent points back correctly:
`children[p]` contains `c` implies `parents[c] == p`.

The reverse MUST also hold: `parents[c] == Some(p)` implies `c`
is in `children[Some(p)]`.

#### Scenario: Two-child root

- GIVEN parents `[None, Some(0), Some(0)]`
- WHEN the children map is built
- THEN `children[Some(0)]` contains both 1 and 2
- AND `children[None]` contains 0
- AND no other entries exist

### Requirement: Walk Termination

`walk_branch` on a well-formed parent map MUST terminate in at
most `n` steps, where `n` is the number of nodes.

#### Scenario: Linear chain

- GIVEN parents `[None, Some(0), Some(1), Some(2)]`
- WHEN walking from node 3
- THEN the walk visits 4 nodes and terminates

#### Scenario: Deep tree

- GIVEN a well-formed parent map of length `n` with a chain
  of depth `n`
- WHEN walking from the deepest node
- THEN the walk terminates in exactly `n` steps

### Requirement: Walk Root-anchored

The first element of a `walk_branch` result MUST have parent
`None` (it is a root node).

#### Scenario: Branch path starts at root

- GIVEN parents `[None, Some(0), Some(1)]`
- WHEN walking from node 2
- THEN the result is `[0, 1, 2]`
- AND `parents[result[0]] == None`

### Requirement: Walk Path-valid

Consecutive elements in a `walk_branch` result MUST have a
parent→child relationship: for all `0 < i < len(path)`,
`parents[path[i]] == Some(path[i-1])`.

#### Scenario: Branching tree

- GIVEN parents `[None, Some(0), Some(0), Some(2)]`
- WHEN walking from node 3
- THEN the result is `[0, 2, 3]`
- AND `parents[2] == Some(0)` and `parents[3] == Some(2)`

### Requirement: Walk Contains Leaf

The last element of a `walk_branch` result MUST equal the
requested leaf node.

#### Scenario: Walk ends at requested leaf

- GIVEN any well-formed parent map
- WHEN walking from node `k`
- THEN `result[result.len() - 1] == k`

### Requirement: Unique Messages Correctness

`find_unique_messages(source_leaf, target_leaf)` MUST return
exactly the nodes on the source branch path that are NOT on
the target branch path.

#### Scenario: Forked branches

- GIVEN parents `[None, Some(0), Some(0), Some(1), Some(2)]`
  (root 0, branch A: 0→1→3, branch B: 0→2→4)
- WHEN finding unique messages from leaf 3 to leaf 4
- THEN the result is `[1, 3]` (nodes on A's path but not B's)

#### Scenario: Ancestor to descendant

- GIVEN a linear chain `[None, Some(0), Some(1)]`
- WHEN finding unique messages from leaf 1 to leaf 2
- THEN the result is empty (node 1 is on both paths)

### Requirement: Divergence Point Correctness

`find_divergence_point(leaf_a, leaf_b)` MUST return the last
common ancestor of the two branches.

The result MUST be on both branch paths.

The node after the divergence point MUST differ between the
two branches (or one branch ends at the divergence point).

#### Scenario: Fork at root

- GIVEN parents `[None, Some(0), Some(0)]`
- WHEN finding divergence of leaf 1 and leaf 2
- THEN the result is node 0

#### Scenario: Fork mid-chain

- GIVEN parents `[None, Some(0), Some(1), Some(1)]`
- WHEN finding divergence of leaf 2 and leaf 3
- THEN the result is node 1

#### Scenario: No divergence (same branch)

- GIVEN parents `[None, Some(0), Some(1)]`
- WHEN finding divergence of leaf 1 and leaf 2
- THEN the result is node 1 (the shorter branch's endpoint)

### Requirement: Leaf Enumeration Soundness

Every node in the result of `find_all_leaves` MUST have no
children in the children map.

#### Scenario: Mixed tree

- GIVEN parents `[None, Some(0), Some(0), Some(1)]`
  (leaves are 2 and 3)
- WHEN finding all leaves
- THEN every returned node has empty children list

### Requirement: Leaf Enumeration Completeness

Every node with no children in the children map MUST appear
in the result of `find_all_leaves`.

#### Scenario: Multiple leaves

- GIVEN the same tree as above
- WHEN finding all leaves
- THEN both node 2 and node 3 are in the result

### Requirement: Latest Leaf Is a Leaf

`find_latest_leaf` MUST return a node with no children.

#### Scenario: Branching tree latest leaf

- GIVEN any well-formed parent map
- WHEN finding the latest leaf
- THEN the returned node has no children

### Requirement: Selective Merge Correctness

Given a set of selected node IDs and the set of unique
messages between source and target branches, the selective
merge copies exactly the intersection: selected ∩ unique.

#### Scenario: Partial selection

- GIVEN unique messages `[1, 3, 5]` and selection `[1, 5]`
- WHEN computing the selective merge set
- THEN the result is `[1, 5]`

#### Scenario: Selection includes non-unique

- GIVEN unique messages `[1, 3]` and selection `[1, 2, 3]`
- WHEN computing the selective merge set
- THEN the result is `[1, 3]` (node 2 is not unique, excluded)

### Requirement: Cherry-pick Subtree Completeness

Cherry-pick with children MUST collect every node reachable
from the source node via child pointers (the full subtree).

#### Scenario: Subtree of three

- GIVEN parents `[None, Some(0), Some(1), Some(1)]`
- WHEN cherry-picking node 1 with children
- THEN the result contains nodes 1, 2, and 3

### Requirement: Cherry-pick Subtree Soundness

Every node collected by cherry-pick with children MUST be
a descendant of the source node (reachable via child pointers).

#### Scenario: Sibling not included

- GIVEN parents `[None, Some(0), Some(0)]`
- WHEN cherry-picking node 1 with children
- THEN node 2 is NOT in the result (it's a sibling, not a descendant)

### Requirement: Merge Branch Count

`merge_branch` reports a count equal to the number of
unique messages (source path minus target path).

#### Scenario: Two unique messages

- GIVEN a tree with 2 messages unique to the source branch
- WHEN merge_branch is invoked
- THEN the reported count is 2
