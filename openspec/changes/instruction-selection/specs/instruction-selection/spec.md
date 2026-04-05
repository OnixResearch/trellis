# Instruction Selection Specification

## Purpose

Defines requirements for verified tree-tiling instruction selection via
dynamic programming. Covers completeness, soundness, optimality, pattern
matching, the DP recurrence, and determinism.

## Requirements

### Requirement: Completeness

Every node in the expression tree MUST be covered by exactly one tile.

#### Scenario: Full coverage

- GIVEN an expression tree and a tiling
- WHEN every node is checked
- THEN it belongs to exactly one tile's coverage

#### Scenario: No gaps

- GIVEN an expression tree and a tiling
- WHEN the union of all tiles' covered nodes is computed
- THEN it equals the set of all nodes in the tree

#### Scenario: No overlaps

- GIVEN two tiles T1 and T2 in the tiling
- WHEN their covered node sets are compared
- THEN the sets are disjoint

### Requirement: Soundness

Each tile's semantics MUST match the semantics of the subtree it covers.

#### Scenario: Tile produces correct value

- GIVEN tile T covering subtree S with input values V1, V2, ...
- WHEN `tile.semantics(V1, V2, ...)` is compared to `subtree_eval(S)`
- THEN the results are equal

#### Scenario: Composed soundness

- GIVEN the full tiling applied to the root
- WHEN the tiled program's output is compared to `tree_eval(root)`
- THEN the results are equal

### Requirement: Optimality

The DP algorithm MUST produce a minimum-cost tiling.

#### Scenario: Minimum cost

- GIVEN the DP algorithm's tiling with cost C
- WHEN any other valid tiling's cost C' is computed
- THEN C ≤ C'

### Requirement: Tile Matching

A tile MUST match a subtree iff the tile's pattern structurally matches
the subtree's shape and opcodes.

#### Scenario: Pattern match

- GIVEN a tile with pattern `(Add, [Any, Any])` and a subtree `Add(X, Y)`
- WHEN matching is checked
- THEN the tile matches with uncovered children [X, Y]

#### Scenario: Pattern mismatch

- GIVEN a tile with pattern `(Add, [Any, Any])` and a subtree `Mul(X, Y)`
- WHEN matching is checked
- THEN the tile does not match

#### Scenario: Deep pattern

- GIVEN a tile with pattern `(Add, [Specific(Mul, [Any, Any]), Any])`
  and subtree `Add(Mul(A, B), C)`
- WHEN matching is checked
- THEN the tile matches with uncovered children [A, B, C]

### Requirement: DP Recurrence

The optimal cost at each node MUST satisfy the DP recurrence relation.

#### Scenario: Recurrence

- GIVEN node N with matching tiles T1, T2, ...
- WHEN the optimal cost is computed
- THEN it equals `min over T_i of (T_i.cost + sum of optimal costs at T_i's uncovered children)`

#### Scenario: Leaf node

- GIVEN a leaf node with matching tiles
- WHEN the optimal cost is computed
- THEN it equals the minimum tile cost (no children to recurse on)

### Requirement: Determinism

The algorithm MUST produce a unique tiling for a given tree and tile set.

#### Scenario: Same inputs same output

- GIVEN the same expression tree and tile set
- WHEN the algorithm runs twice
- THEN the resulting tilings are identical
