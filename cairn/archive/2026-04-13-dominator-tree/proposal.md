## Why

Dominators are the backbone of compiler analysis. Node A dominates node B
when every path from the entry to B passes through A. Immediate dominators
turn that relation into a tree, and dominance frontiers mark exactly where
that tree stops being strict enough for phi placement.

This repo already has CFG reachability and related graph helpers, but it did
not have a verified dominator layer in the main spec set. Completing this
change gives downstream compiler work a stable base for loop detection, SSA
construction, and later optimization passes.

## What Changes

- Add and verify `src/dominator_tree.rs`.
- Model graph validity, predecessor collection, iterative dominance-matrix
  computation, immediate-dominator extraction, and dominance-frontier
  computation.
- Track the shipped behavior in `docs/requirements.md`, the main OpenSpec
  domain spec, and the README module inventory.

## Capabilities

### New Capabilities

- `dominator-tree.valid-graph`: Verified rejection of out-of-range CFG edges
- `dominator-tree.entry-dominates`: Verified entry dominates every reachable node
- `dominator-tree.partial-order`: Verified dominance is reflexive, transitive, and antisymmetric on reachable nodes
- `dominator-tree.idom-unique`: Verified each reachable non-entry node has exactly one immediate dominator
- `dominator-tree.tree-structure`: Verified immediate-dominator links form a rooted parent tree
- `dominator-tree.convergence`: Verified the iterative dominator computation converges to a stable result
- `dominator-tree.dominance-frontier`: Verified frontier membership matches predecessor dominance and non-strict dominance at joins

## Impact

- **Files**: `src/dominator_tree.rs`, `src/lib.rs`, `docs/requirements.md`, `README.md`, and the synced `openspec/specs/dominator-tree/spec.md`
- **APIs**: `predecessors`, `compute_dominators`, `dominates`, `immediate_dominators`, `dominance_frontier`, `compute_domtree`
- **Dependencies**: Reuses `reachability.rs` and standard `vstd` sequence/vector reasoning
- **Testing**: Verus-checked shape/safety obligations plus unit tests and proptests against small brute-force CFG oracles

## Out of Scope

- Postdominators
- Control dependence graphs
- Incremental dominator maintenance
