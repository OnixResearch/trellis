# Instruction Selection Specification

## Purpose

Defines requirements for verified tree-tiling instruction selection via
dynamic programming. Covers exact coverage for complete tile sets, pure
semantic preservation, minimum-cost selection, structural pattern
matching, the DP recurrence, deterministic tie-breaking, and explicit
failure on incomplete tile sets.

## Model

- Expression trees use a finite typed opcode set with pure mathematical-integer
  semantics restricted to trees whose evaluated values fit in `u64`; modeled
  public/runtime surfaces are defined only on inputs satisfying that no-overflow
  well-formedness boundary.
- Each node is identified by a root-relative `NodePath`; coverage facts are
  stated over sets of node paths.
- Canonical preorder path order compares `NodePath` values lexicographically by
  components, with shorter equal-prefix paths ordered before longer extensions.
- `TilePattern::Specific` contributes covered nodes; `TilePattern::Any`
  exposes uncovered child roots to be tiled recursively.
- A valid tile pattern is rooted at `Specific { ... }`; `Any` may appear only
  below that root, so every uncovered child root returned by `tile_matches` is
  a proper descendant of the current subtree root.
- Const tiles are value-parameterized at the root: a const tile matches a const
  node only when the node's literal `value` equals the tile's modeled const value.
- Tile semantics inputs are the evaluations of uncovered child roots in
  canonical preorder path order, exactly the order returned by `tile_matches`.
- `optimal_tiling` takes an ordered tile sequence; each tile has a stable
  `tie_break_rank`, and `tile index` means the tile's position in that input
  sequence.
- Candidate and tiling costs are compared in a non-overflowing mathematical
  integer accumulator; modeled public/runtime surfaces are defined only on tile
  sequences and trees whose total selected cost fits the runtime result type.
- All modeled matching/selection surfaces require valid tile patterns rooted at
  `Specific { ... }`; invalid root-`Any` patterns are outside the modeled API
  contract.
- A valid tiling is a successful tiling whose selected coverage sets are exact
  and non-overlapping for the full tree and whose selected tile semantics agree
  with subtree evaluation under the canonical input order.
- `complete_tile_set(tree, tiles)` means: for every opcode/arity used anywhere
  in `tree`, the ordered tile sequence contains at least one base tile with
  root `Specific { op, children }` matching that opcode, literal boundary, and arity
  and whose immediate children are all `Any`.
- All modeled public/runtime surfaces (`subtree_eval`, `tile_matches`,
  `optimal_tiling`, `tiling_cost`, `check_coverage`, and `check_soundness`)
  are defined only for well-formed expression trees.
- `check_coverage` validates exact coverage from the selected tiles plus the tree;
  it does not trust caller-supplied `covered_paths` blindly.
- `optimal_tiling` may fail when no tile matches some subtree root.
- When multiple subtree roots are unmatched, failure reports the canonical first
  failing root in preorder path order.
- Exact coverage theorems apply only when `complete_tile_set(tree, tiles)` holds.

## Requirements

### Requirement: Completeness [r[isel.completeness]]

For complete tile sets, every node in the expression tree MUST be covered
by exactly one selected tile.

#### Scenario: Full coverage [r[isel.completeness.full-coverage]]

- GIVEN an expression tree, a tile sequence satisfying `complete_tile_set(tree, tiles)`,
  and a successful tiling
- WHEN every root-relative node path in the tree is checked
- THEN it belongs to exactly one selected tile coverage set

#### Scenario: No gaps [r[isel.completeness.no-gaps]]

- GIVEN an expression tree, a tile sequence satisfying `complete_tile_set(tree, tiles)`, and a successful tiling
- WHEN the union of all selected tile coverage sets is computed
- THEN it equals the set of all node paths in the tree

#### Scenario: No overlaps [r[isel.completeness.no-overlaps]]

- GIVEN two distinct selected tiles in a successful tiling
- WHEN their covered node-path sets are compared
- THEN the sets are disjoint

#### Scenario: Complete tile sets succeed [r[isel.completeness.success-on-complete-tile-set]]

- GIVEN an expression tree and an ordered tile sequence satisfying
  `complete_tile_set(tree, tiles)`
- WHEN `optimal_tiling` is invoked
- THEN it returns a successful tiling instead of a missing-tile failure

### Requirement: Incomplete Tile-Set Failure [r[isel.incomplete-tile-set-failure]]

If some subtree root has no matching tile, instruction selection MUST
return an explicit failure result rather than fabricate a tiling.

#### Scenario: Missing tile at root [r[isel.incomplete-tile-set-failure.root]]

- GIVEN an expression tree whose root matches no tile in the provided tile sequence
- WHEN `optimal_tiling` is invoked
- THEN it returns `SelectionError::NoMatchingTile { root_path = [] }`

#### Scenario: Missing tile below matched parent [r[isel.incomplete-tile-set-failure.child]]

- GIVEN a root tile that matches but at least one uncovered child root has no
  matching tile in the provided tile sequence
- WHEN `optimal_tiling` recurses into uncovered child roots
- THEN the overall result is `SelectionError::NoMatchingTile { root_path = P }`
  where `P` is the canonical first unmatched child-root path

#### Scenario: Multiple unmatched roots choose canonical first [r[isel.incomplete-tile-set-failure.global-first]]

- GIVEN an expression tree with multiple unmatched subtree roots at different
  `NodePath` positions
- WHEN `optimal_tiling` fails
- THEN it reports the canonical preorder-minimal unmatched root path

### Requirement: Soundness [r[isel.soundness]]

Each selected tile's pure semantics MUST match the pure evaluation of the
subtree it replaces.

#### Scenario: Tile produces correct value [r[isel.soundness.tile-value]]

- GIVEN a selected tile `T` covering subtree root `P`, with uncovered child
  roots evaluated in canonical preorder path order
- WHEN `T.semantics(inputs)` is compared to `subtree_eval(subtree_at(P))`
- THEN the results are equal

#### Scenario: Composed soundness [r[isel.soundness.composed]]

- GIVEN a successful tiling of the full expression tree
- WHEN the tiled root output is compared to `subtree_eval(root)`
- THEN the results are equal

### Requirement: Optimality [r[isel.optimality]]

The DP algorithm MUST return a tiling with minimum total cost among all valid
tilings for the same tree and ordered tile sequence.

#### Scenario: Minimum cost [r[isel.optimality.minimum-cost]]

- GIVEN a successful tiling returned by the DP algorithm with cost `C`
- WHEN any other valid tiling for the same tree and ordered tile sequence has cost `C'`
- THEN `C <= C'`

#### Scenario: Leaf node [r[isel.optimality.leaf]]

- GIVEN a leaf node and the set of matching leaf tiles
- WHEN the optimal cost is computed
- THEN it equals the minimum total candidate cost among those leaf tiles

### Requirement: Tile Matching [r[isel.matching]]

A tile MUST match a subtree iff the tile pattern structurally matches the
subtree opcode, literal boundary, and child shape.

#### Scenario: Pattern match [r[isel.matching.pattern-match]]

- GIVEN a tile with pattern `Specific { op: Add, children: [Any, Any] }`
  and a subtree `Add(X, Y)`
- WHEN matching is checked
- THEN the tile matches and returns uncovered child roots for `X` and `Y`

#### Scenario: Canonical uncovered-root order [r[isel.matching.canonical-uncovered-order]]

- GIVEN a matching tile pattern with multiple `Any` leaves at different child-root paths
- WHEN `tile_matches` returns uncovered child roots
- THEN those roots are returned in canonical preorder `NodePath` order

#### Scenario: Pattern mismatch [r[isel.matching.pattern-mismatch]]

- GIVEN a tile with pattern `Specific { op: Add, children: [Any, Any] }`
  and a subtree `Mul(X, Y)`
- WHEN matching is checked
- THEN the tile does not match

#### Scenario: Const literal mismatch [r[isel.matching.const-literal-mismatch]]

- GIVEN a const tile modeled for literal `A` and a const subtree with literal `B != A`
- WHEN matching is checked
- THEN the tile does not match

#### Scenario: Deep pattern [r[isel.matching.deep-pattern]]

- GIVEN a tile with pattern `Specific { op: Add, children: [Specific { op: Mul, children: [Any, Any] }, Any] }`
  and subtree `Add(Mul(A, B), C)`
- WHEN matching is checked
- THEN the tile matches with uncovered child roots for `A`, `B`, and `C`

#### Scenario: Wrong arity [r[isel.matching.wrong-arity]]

- GIVEN a tile pattern whose `Specific` child count differs from the subtree
  child count
- WHEN matching is checked
- THEN the tile does not match

### Requirement: DP Recurrence [r[isel.dp-recurrence]]

The optimal cost at each subtree root MUST satisfy the dynamic-programming
recurrence over matching tiles and uncovered child roots.

#### Scenario: Recurrence [r[isel.dp-recurrence.general]]

- GIVEN subtree root `P` with matching tiles `T1`, `T2`, ...
- WHEN the optimal cost is computed
- THEN it equals the minimum over `Ti.cost + sum(optimal cost at Ti uncovered child roots)`

#### Scenario: Failure has no candidate [r[isel.dp-recurrence.failure]]

- GIVEN subtree root `P` with no matching tiles
- WHEN the DP step for `P` is computed
- THEN the result is failure rather than a numeric cost

### Requirement: Determinism [r[isel.determinism]]

For a fixed tree and ordered tile sequence, instruction selection MUST produce
 a unique result by using a fixed tie-break rule on equal-cost candidates.

#### Scenario: Same inputs same output [r[isel.determinism.same-inputs]]

- GIVEN the same expression tree and ordered tile sequence
- WHEN the algorithm runs twice
- THEN the resulting tilings are identical

#### Scenario: Equal-cost tie break [r[isel.determinism.tie-break]]

- GIVEN two or more matching candidates with equal total cost at the same
  subtree root in the same ordered tile sequence
- WHEN candidate selection is performed
- THEN the algorithm chooses the lexicographically minimum candidate by
  `(total_cost, tie_break_rank, tile index)`
