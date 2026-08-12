# Dominator Tree Specification

## Purpose

Defines requirements for verified dominator computation on a control flow
graph. Covers the dominator predicate, immediate dominators, tree structure,
iterative algorithm convergence, and dominance frontier computation.

## Requirements

### Requirement: Entry Dominates All
r[dominator-tree.entry-dominates]

The entry node MUST dominate every reachable node in the CFG.

#### Scenario: Entry dominates any reachable node

- GIVEN a CFG with entry node 0 and a reachable node N
- WHEN the dominance relation is checked
- THEN `dominates(0, N)` is true

#### Scenario: Unreachable nodes

- GIVEN a node U not reachable from the entry
- WHEN the dominance relation is checked
- THEN U has no dominator (undefined)

### Requirement: Dominance Partial Order
r[dominator-tree.partial-order]

The dominance relation MUST be a partial order: reflexive, transitive,
and antisymmetric.

#### Scenario: Reflexivity

- GIVEN any reachable node N
- WHEN `dominates(N, N)` is checked
- THEN the result is true

#### Scenario: Transitivity

- GIVEN `dominates(A, B)` and `dominates(B, C)`
- WHEN `dominates(A, C)` is checked
- THEN the result is true

#### Scenario: Antisymmetry

- GIVEN `dominates(A, B)` and `dominates(B, A)`
- WHEN A and B are compared
- THEN A == B

### Requirement: Immediate Dominator Uniqueness
r[dominator-tree.idom-unique]

Each non-entry reachable node MUST have exactly one immediate dominator.

#### Scenario: Non-entry node

- GIVEN a reachable non-entry node N
- WHEN the immediate dominator is computed
- THEN exactly one node M satisfies: M strictly dominates N, and no other
  node strictly dominates N while being dominated by M

#### Scenario: Entry has no idom

- GIVEN the entry node 0
- WHEN the immediate dominator is queried
- THEN no immediate dominator exists (entry is the root)

### Requirement: Tree Structure
r[dominator-tree.tree-structure]

The immediate dominator relation MUST form a rooted tree with the entry
as root.

#### Scenario: Tree property

- GIVEN the idom relation over all reachable nodes
- WHEN the structure is checked
- THEN it forms a tree: each non-root has exactly one parent, no cycles,
  and the root is the entry

#### Scenario: All reachable nodes in tree

- GIVEN the dominator tree
- WHEN every reachable node is checked
- THEN it appears in the tree

### Requirement: Algorithm Convergence
r[dominator-tree.convergence]

The iterative dominator algorithm MUST terminate and produce the correct
dominator tree.

#### Scenario: Termination

- GIVEN any CFG
- WHEN the iterative algorithm runs
- THEN it terminates within O(n²) iterations

#### Scenario: Correctness

- GIVEN the iterative algorithm's output
- WHEN compared to the path-based dominator definition
- THEN the output matches: `idom[N] == M` iff M is the immediate dominator of N

### Requirement: Dominance Frontier
r[dominator-tree.dominance-frontier]

The dominance frontier of a node MUST contain exactly the nodes where
the node's dominance ends.

#### Scenario: Frontier membership

- GIVEN node A and node Y
- WHEN Y is in the dominance frontier of A
- THEN A dominates a predecessor of Y but does not strictly dominate Y

#### Scenario: Non-membership

- GIVEN node A and node Y not in DF(A)
- WHEN every predecessor of Y is checked
- THEN either A does not dominate any predecessor of Y, or A strictly dominates Y
