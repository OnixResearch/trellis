## Why

Loop detection identifies natural loops in a CFG — the most frequent
targets for optimization (loop-invariant code motion, strength reduction,
unrolling, vectorization). A natural loop has a single entry point (the
header), which dominates all nodes in the loop body, and at least one
back edge from a body node to the header.

The definition of "natural loop" depends on dominators: an edge (B → H) is
a back edge iff H dominates B. The loop body is the set of nodes that can
reach B without leaving through H. Loop nesting (one loop inside another)
forms a forest that drives optimization pass ordering.

Getting loop detection wrong means optimizations are applied to the wrong
code region. Hoisting an instruction out of a loop that doesn't actually
contain it changes semantics. Missing a loop means missing an optimization
opportunity.

## What Changes

- **loop_detection**: A new module modeling natural loop identification,
  back edge classification, loop body computation, and loop nesting.

  Proves:
  - **Back edge classification**: An edge (B → H) is a back edge iff H
    dominates B. All other edges are forward or cross edges.
  - **Header dominance**: The loop header dominates every node in the loop
    body.
  - **Loop body correctness**: The loop body is exactly the set of nodes
    that can reach the back-edge source B without passing through the
    header H (plus H itself).
  - **Single entry**: The header is the only entry point to the loop. All
    edges from outside the loop enter through the header.
  - **Loop nesting**: If loop L1 is nested inside loop L2, then
    L1's body ⊂ L2's body and L1's header is in L2's body.
  - **Nesting forest**: The loop nesting relation forms a forest (tree per
    outermost loop, no cycles in nesting).
  - **Exit edges**: An exit edge goes from a loop body node to a node
    outside the loop. All exit targets are identified.

## Capabilities

### New Capabilities

- `loop.back-edge`: Verified back edge classification via dominance
- `loop.header-dominance`: Verified header dominates all body nodes
- `loop.body`: Verified loop body computation
- `loop.single-entry`: Verified header is sole entry point
- `loop.nesting`: Verified loop nesting subset relationship
- `loop.nesting-forest`: Verified nesting relation is a forest
- `loop.exits`: Verified loop exit edge identification

## Impact

- **Files**: One new `src/loop_detection.rs`, module declaration in `lib.rs`
- **APIs**: `NaturalLoop`, `find_back_edges`, `compute_loop_body`,
  `loop_nesting_forest`, `loop_exits`, `is_loop_header`
- **Dependencies**: Depends on `dominator_tree` for dominance,
  `reachability.rs` for path-based body computation, `forest.rs` for
  nesting tree, `vstd`
- **Testing**: Verus proofs for all properties, proptest for loop body
  correctness against brute-force path enumeration
