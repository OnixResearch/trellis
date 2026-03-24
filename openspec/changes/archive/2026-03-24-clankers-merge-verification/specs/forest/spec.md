# Forest Specification

## Purpose

Verified primitives for parent-pointer forests: ordered collections
of rooted trees where each node stores its parent index (or None for
roots) and parent indices are strictly less than child indices. This
representation appears in git commit graphs, file system trees,
dependency DAGs, DOM trees, and conversation branching structures.

The strict-ordering invariant (parent < child) guarantees acyclicity
by construction, making walk termination trivial without separate
cycle detection.

## Definitions

- **Node**: An entry identified by integer index `0 <= i < len`.
- **Parent map**: `Seq<Option<int>>` where `parents[i] = Some(j)`
  means node `j` is the parent of node `i`, and `parents[i] = None`
  means node `i` is a root.
- **Well-formed**: All parent references in bounds and strictly less
  than the child index. Implies acyclicity.
- **Root**: A node whose parent is `None`.
- **Leaf**: A node that is not the parent of any other node.
- **Ancestor path**: The sequence of nodes from root to a given node,
  following parent pointers upward then reversing.
- **Depth**: Length of the ancestor path (number of edges from root).
- **LCA (least common ancestor)**: The last node shared by two
  ancestor paths before they diverge.
- **Subtree**: All nodes reachable from a given root via child
  pointers (transitive closure of the parent-inverse relation).

## Requirements

### Requirement: Well-formedness

A parent map of length `n` is well-formed iff for every node `i`:
- `parents[i] = Some(j)` implies `0 <= j < i`
- `parents[i] = None` is always valid (root)

This implies no self-references (`j < i` excludes `j = i`) and no
cycles (indices strictly decrease along any parent chain).

#### Scenario: Valid forest

- GIVEN parents `[None, Some(0), Some(0), Some(1)]`
- WHEN well-formedness is checked
- THEN accepted (0 < 1, 0 < 2, 1 < 3)

#### Scenario: Self-reference rejected

- GIVEN parents `[Some(0)]`
- WHEN well-formedness is checked
- THEN rejected (0 is not < 0)

#### Scenario: Forward reference rejected

- GIVEN parents `[Some(1), None]`
- WHEN well-formedness is checked
- THEN rejected (1 is not < 0)

#### Scenario: Out-of-bounds rejected

- GIVEN parents `[None, Some(5)]` (length 2, parent index 5)
- WHEN well-formedness is checked
- THEN rejected (5 >= 2)

### Requirement: Root and Leaf Classification

`is_root(parents, i)` holds iff `parents[i] = None`.

`is_leaf(parents, i)` holds iff no node `j` has `parents[j] = Some(i)`.

Every node is exactly one of: internal (has parent, has children),
root-internal (no parent, has children), root-leaf (no parent, no
children), or non-root leaf (has parent, no children).

#### Scenario: Single-node forest

- GIVEN parents `[None]`
- THEN node 0 is both root and leaf

#### Scenario: Root with children

- GIVEN parents `[None, Some(0)]`
- THEN node 0 is root but not leaf; node 1 is leaf but not root

### Requirement: Children-of Consistency

`children_of(parents, p)` returns the sequence of indices whose
parent equals `p`. For `p = Some(k)`, this is all `i` where
`parents[i] = Some(k)`. For `p = None`, this is all roots.

The result MUST contain `c` iff `parents[c] = p`.

Children indices MUST appear in ascending order (natural scan order).

#### Scenario: Two children

- GIVEN parents `[None, Some(0), Some(0), Some(1)]`
- THEN children_of(parents, Some(0)) = [1, 2]
- AND children_of(parents, Some(1)) = [3]
- AND children_of(parents, None) = [0]

### Requirement: Ancestor Path Termination

`ancestor_path(parents, node)` on a well-formed parent map MUST
terminate in at most `parents.len()` steps.

#### Scenario: Full-depth chain

- GIVEN parents `[None, Some(0), Some(1), Some(2)]`
- WHEN computing ancestor_path(parents, 3)
- THEN terminates after 4 steps

### Requirement: Ancestor Path Root-anchored

The first element of `ancestor_path(parents, node)` MUST be a root:
`parents[result[0]] = None`.

#### Scenario: Path starts at root

- GIVEN parents `[None, Some(0), Some(1)]`
- WHEN computing ancestor_path(parents, 2)
- THEN result = [0, 1, 2] and parents[0] = None

### Requirement: Ancestor Path Valid

Consecutive elements in the ancestor path MUST have a parent→child
relationship: `parents[path[i]] = Some(path[i-1])` for all
`0 < i < path.len()`.

#### Scenario: Branching tree path

- GIVEN parents `[None, Some(0), Some(0), Some(2)]`
- WHEN computing ancestor_path(parents, 3)
- THEN result = [0, 2, 3] with parents[2]=Some(0), parents[3]=Some(2)

### Requirement: Ancestor Path Contains Node

The last element of `ancestor_path(parents, node)` MUST equal `node`.

#### Scenario: Walk ends at requested node

- GIVEN any well-formed parent map and valid node index `k`
- WHEN computing ancestor_path(parents, k)
- THEN result[result.len() - 1] = k

### Requirement: Ancestor Path Length Bound

`ancestor_path(parents, node).len() <= parents.len()`.

### Requirement: Depth

`depth(parents, node)` equals `ancestor_path(parents, node).len() - 1`
(number of edges from root to node). Roots have depth 0.

### Requirement: Is-ancestor

`is_ancestor(parents, a, d)` holds iff `a` appears in
`ancestor_path(parents, d)`.

Reflexive: every node is its own ancestor.

Transitive: if `a` is ancestor of `b` and `b` is ancestor of `c`,
then `a` is ancestor of `c`.

Antisymmetric: if `a` is ancestor of `b` and `b` is ancestor of `a`,
then `a = b`.

### Requirement: Least Common Ancestor

`lca(parents, a, b)` returns the last node shared by
`ancestor_path(parents, a)` and `ancestor_path(parents, b)`.

The result MUST be an ancestor of both `a` and `b`.

The result MUST be maximal: no descendant of the result is an
ancestor of both `a` and `b`.

#### Scenario: Fork at root

- GIVEN parents `[None, Some(0), Some(0)]`
- THEN lca(parents, 1, 2) = 0

#### Scenario: Fork mid-chain

- GIVEN parents `[None, Some(0), Some(1), Some(1)]`
- THEN lca(parents, 2, 3) = 1

#### Scenario: Ancestor-descendant pair

- GIVEN parents `[None, Some(0), Some(1)]`
- THEN lca(parents, 1, 2) = 1

### Requirement: Leaf Enumeration Soundness

Every node in `leaves(parents)` MUST satisfy `is_leaf(parents, node)`.

### Requirement: Leaf Enumeration Completeness

Every node satisfying `is_leaf(parents, node)` MUST appear in
`leaves(parents)`.

### Requirement: Subtree Soundness

Every node in `subtree(parents, root)` MUST be reachable from `root`
via zero or more child-pointer hops.

### Requirement: Subtree Completeness

Every node reachable from `root` via child pointers MUST appear in
`subtree(parents, root)`.

### Requirement: Subtree Contains Root

`root` is always in `subtree(parents, root)`.

### Requirement: Subtree of Leaf

`subtree(parents, leaf)` where `is_leaf(parents, leaf)` contains
only `leaf` itself.
