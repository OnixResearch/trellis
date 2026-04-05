## Why

Liveness analysis computes which variables are live (have a future use
before a redefinition) at each program point. It's the bridge between
SSA and register allocation: two variables that are simultaneously live
need different registers. Liveness also drives dead code elimination —
a definition with no live range is dead.

The analysis is a backward dataflow problem: live-out of a block is the
union of live-in of its successors, and live-in is (live-out minus defs)
union uses. The fixed point converges because the lattice (sets of
variables) has finite height and the transfer function is monotone.

Getting liveness wrong means register allocation assigns two live
variables to the same register (silent corruption) or fails to spill
a variable that's actually live (crash). Both are miscompilation bugs
that are extremely hard to diagnose.

## What Changes

- **liveness_analysis**: A new module modeling backward dataflow liveness
  analysis on a CFG.

  Proves:
  - **Fixed-point convergence**: The iterative analysis terminates.
    The lattice (powerset of variables, ordered by ⊆) has finite height,
    and the transfer function is monotone.
  - **Live-in / live-out equations**: At fixed point, the live-in and
    live-out sets satisfy the dataflow equations for every block.
  - **Use-def correctness**: A variable is live at a point iff there exists
    a path from that point to a use with no intervening definition.
  - **Connected live ranges**: The set of program points where a variable
    is live forms a connected subgraph of the CFG.
  - **Def kills liveness**: If a block defines variable V and does not use
    V before the definition, then V is not live-in to that block.
  - **Exit block**: Variables not used after the exit block are not live-out
    of the exit block.

## Capabilities

### New Capabilities

- `liveness.convergence`: Verified dataflow fixed-point termination
- `liveness.equations`: Verified live-in/live-out equation satisfaction
- `liveness.use-def`: Verified liveness iff path-to-use without intervening def
- `liveness.connected`: Verified live range connectivity
- `liveness.def-kills`: Verified definition kills liveness
- `liveness.exit`: Verified exit block liveness boundary

## Impact

- **Files**: One new `src/liveness_analysis.rs`, module declaration in `lib.rs`
- **APIs**: `LiveSets`, `compute_liveness`, `is_live_at`, `live_range`,
  `check_equations`, transfer function and fixed-point infrastructure
- **Dependencies**: Operates on CFG representation, depends on `vstd`
- **Testing**: Verus proofs for all properties, proptest for liveness
  correctness against brute-force path search on small CFGs
