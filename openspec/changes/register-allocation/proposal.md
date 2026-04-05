## Why

Register allocation assigns physical registers to the (potentially
unbounded) set of virtual registers in the IR. It's the most complex
backend pass and the last place where miscompilation can be introduced
before code emission. The interference graph connects simultaneously-live
variables; coloring it with K colors assigns K registers.

The repo already has graph 2-coloring (`graph_color.rs`) but not
K-coloring, interference graph construction, spill insertion, or
coalescing. Register allocation ties together liveness analysis
(for interference), graph algorithms (for coloring), and code
transformation (for spill/reload insertion).

## What Changes

- **register_allocation**: A new module modeling interference graph
  construction, K-coloring, spilling, and coalescing.

  Proves:
  - **Interference correctness**: Two variables have an interference edge
    iff their live ranges overlap. No false edges (would over-constrain),
    no missing edges (would cause two live variables to share a register).
  - **Coloring validity**: The coloring assigns colors (registers) such
    that no two interfering variables share a color.
  - **Spill correctness**: When the graph is not K-colorable, spilled
    variables are stored to memory before their last use and reloaded
    before their next use. The spill/reload insertion does not change
    program semantics.
  - **Coalescing safety**: Two non-interfering variables with a copy
    between them can be assigned the same register. The coalesced
    interference graph is valid (no new interference violations).
  - **Completeness**: Every variable receives either a register or a
    spill slot. No variable is left unassigned.
  - **Briggs criterion**: Two variables can be coalesced if the merged
    node has fewer than K high-degree neighbors. Proves the criterion
    is conservative (never produces an uncolorable graph from a
    colorable one).

## Capabilities

### New Capabilities

- `regalloc.interference`: Verified interference graph from live ranges
- `regalloc.coloring`: Verified K-coloring assigns no conflicting registers
- `regalloc.spill`: Verified spill/reload preserves semantics
- `regalloc.coalesce`: Verified coalescing safety
- `regalloc.completeness`: Verified all variables assigned
- `regalloc.briggs`: Verified conservative coalescing criterion

## Impact

- **Files**: One new `src/register_allocation.rs`, module declaration
  in `lib.rs`
- **APIs**: `InterferenceGraph`, `build_interference`, `color_graph`,
  `spill_variable`, `coalesce`, `check_coloring`, `briggs_test`
- **Dependencies**: Depends on `liveness_analysis` for live ranges,
  `graph_color.rs` for coloring foundations, `vstd`
- **Testing**: Verus proofs for all properties, proptest for coloring
  validity and interference correctness on random programs
