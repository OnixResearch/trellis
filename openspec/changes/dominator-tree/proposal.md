## Why

Dominators are the backbone of every optimizing compiler's analysis phase.
Node A dominates node B if every path from the entry to B passes through A.
The dominator tree organizes this relation: each node's parent is its
immediate dominator (the closest strict dominator). SSA construction, loop
detection, code motion, and dead code elimination all depend on a correct
dominator tree.

The existing repo has graph reachability, cycle detection, and topological
sort, but no dominance analysis. Without dominators, none of the
downstream compiler analyses can be built.

The Cooper-Harvey-Kennedy (CHK) iterative algorithm is the standard
implementation. It converges monotonically on a lattice of dominator sets.
The convergence proof requires showing that the lattice has finite height
and the update function is monotone — both natural Verus targets.

## What Changes

- **dominator_tree**: A new module modeling dominator computation on a
  CFG and proving its properties.

  Proves:
  - **Entry dominates all**: The entry node dominates every reachable node.
  - **Reflexivity**: Every node dominates itself.
  - **Transitivity**: If A dominates B and B dominates C, then A dominates C.
  - **Antisymmetry**: If A dominates B and B dominates A, then A == B.
  - **Immediate dominator uniqueness**: Each non-entry node has exactly one
    immediate dominator.
  - **Tree structure**: The immediate dominator relation forms a tree rooted
    at the entry node.
  - **Convergence**: The iterative fixed-point algorithm terminates and
    produces the correct dominator tree.
  - **Dominance frontier**: The dominance frontier of node A is the set of
    nodes where A's dominance ends. A node Y is in DF(A) iff A dominates
    a predecessor of Y but does not strictly dominate Y.

## Capabilities

### New Capabilities

- `domtree.entry-dominates`: Verified entry dominates all reachable nodes
- `domtree.partial-order`: Verified reflexivity, transitivity, antisymmetry
- `domtree.idom-unique`: Verified unique immediate dominator
- `domtree.tree-structure`: Verified dominator tree is a rooted tree
- `domtree.convergence`: Verified iterative algorithm terminates with correct result
- `domtree.dominance-frontier`: Verified dominance frontier computation

## Impact

- **Files**: One new `src/dominator_tree.rs`, module declaration in `lib.rs`
- **APIs**: `dominates`, `immediate_dominator`, `compute_domtree`,
  `dominance_frontier`, convergence proof lemmas
- **Dependencies**: Uses `reachability.rs` for path existence, `forest.rs`
  for tree structure, depends on `vstd`
- **Testing**: Verus proofs for all properties, proptest for dominator tree
  correctness against brute-force path enumeration on small CFGs
