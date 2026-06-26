## Why

Loop detection identifies natural loops in a CFG — frequent optimization
targets for loop-invariant code motion, strength reduction, unrolling, and
vectorization. This change exposes exact helpers over a CFG plus previously
computed reachability/dominator views. The main body/entry/nesting
properties target reachable non-self natural loops: a loop has a single
entry point (the header), the header dominates its body, and at least one
back edge comes from a distinct body node to the header.

The definition of "natural loop" still depends on dominators: an edge
(B → H) is a back edge iff H dominates B. The shipped body helper computes
a seeded backward closure restricted to reachable, header-dominated nodes.
Randomized tests compare that closure against the brute-force path oracle
for reachable non-self loops. Loop nesting forms a forest that drives pass
ordering.

Getting loop detection wrong means optimizations are applied to the wrong
code region. Hoisting an instruction out of a loop that doesn't actually
contain it changes semantics. Missing a loop means missing an optimization
opportunity.

## What Changes

- **loop_detection**: A new module modeling natural loop identification,
  back edge classification, loop body computation, and loop nesting.

  Proves:
  - **Back edge classification**: `find_back_edges(edges, dom)` returns
    exactly the CFG edges whose target dominates their source.
  - **Header dominance**: The loop header dominates every node in a checked
    loop body.
  - **Loop body closure**: `compute_loop_body(edges, reachable, dom, H, B)`
    returns exactly the seeded backward closure over reachable,
    header-dominated nodes.
  - **Single entry**: `has_single_entry(edges, reachable, H, body)` returns
    true iff every reachable edge from outside `body` enters through `H`.
  - **Loop nesting**: If loop L1 is nested inside loop L2, then
    L1.body ⊂ L2.body and L1.header is in L2.body.
  - **Nesting forest**: Parent links are always strict-body supersets; the
    implementation deterministically chooses the smallest such superset,
    with tests covering parent selection.
  - **Exit edges**: `loop_exits(edges, body)` returns exactly the edges from
    body nodes to non-body nodes.

  Tests additionally compare computed non-self loop bodies against a
  brute-force path oracle on random CFGs.

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
  `has_single_entry`, `loop_nesting_forest`, `loop_exits`, `is_loop_header`
- **Dependencies**: Depends on `dominator_tree` for dominance views,
  `reachability` for reachability views, `vstd`
- **Testing**: Verus proofs for exact helper contracts, unit tests for
  nested and while/if loop shapes, and proptest against brute-force path
  enumeration / CFG-derived parent selection
