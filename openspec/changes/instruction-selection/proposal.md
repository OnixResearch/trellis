## Why

Instruction selection maps IR operations to machine instructions. The IR
is typically an expression tree (or DAG); machine instructions are tiles
that cover subtrees. The goal is to tile the entire tree with
non-overlapping tiles such that every IR node is covered and the total
cost is minimized.

The standard approach (Aho, Ganapathi, Tjiang) uses dynamic programming
on the expression tree. For each node, find all tiles whose root pattern
matches, compute the cost of each (tile cost plus optimal cost of
uncovered subtrees), and pick the minimum. The result is a tiling that
covers every node exactly once at minimum cost.

Correctness has two parts: the tiling must cover every node (completeness),
and each tile's semantics must match the subtree it replaces (soundness).
An incorrect tile produces wrong code. A gap in the tiling means an IR
operation has no machine instruction — the compiler crashes or emits garbage.

## What Changes

- **instruction_selection**: A new module modeling tree tiling for
  instruction selection via dynamic programming.

  Proves:
  - **Completeness**: Every node in the expression tree is covered by
    exactly one tile. No gaps, no overlaps.
  - **Soundness**: Each tile's semantics (spec function) matches the
    semantics of the subtree it covers. The tiled program computes the
    same value as the original expression tree.
  - **Optimality**: The dynamic programming algorithm produces a minimum-cost
    tiling among all valid tilings.
  - **Tile matching correctness**: A tile matches a subtree iff the tile's
    pattern structurally matches the subtree's shape and operation types.
  - **DP recurrence**: The optimal cost at a node equals the minimum over
    all matching tiles of (tile cost + sum of optimal costs at uncovered
    children). Proved by structural induction on the tree.
  - **Determinism**: For a given tree and tile set, the algorithm produces
    a unique tiling (ties broken deterministically).

## Capabilities

### New Capabilities

- `isel.completeness`: Verified every IR node covered by exactly one tile
- `isel.soundness`: Verified tile semantics matches subtree semantics
- `isel.optimality`: Verified minimum-cost tiling
- `isel.matching`: Verified pattern matching correctness
- `isel.dp-recurrence`: Verified dynamic programming recurrence
- `isel.determinism`: Verified unique output for given inputs

## Impact

- **Files**: One new `src/instruction_selection.rs`, module declaration
  in `lib.rs`
- **APIs**: `Tile`, `ExprTree`, `tile_matches`, `optimal_tiling`,
  `tiling_cost`, `check_coverage`, `check_soundness`
- **Dependencies**: Depends on `vstd`, uses tree structure for DP
- **Testing**: Verus proofs for all properties, proptest for coverage
  and cost optimality on random expression trees with random tile sets
