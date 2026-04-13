## Context

Liveness is a backward dataflow problem on the CFG. A variable is live at
a point if there's a path to a use without an intervening definition.
The standard algorithm iterates backward, computing live-in and live-out
sets per block until a fixed point is reached.

## Goals / Non-Goals

**Goals:**
- Per-block live-in and live-out sets
- Backward dataflow equations
- Fixed-point convergence via finite lattice argument
- Use-def characterization of liveness
- Connected live ranges
- Def-kills and exit boundary properties

**Non-Goals:**
- Per-instruction liveness (only per-block in this model; per-instruction
  is a refinement)
- Sparse liveness representations
- SSA-based liveness (live range splitting, etc.)
- Interference graph construction (that's register allocation)

## Decisions

### 1. Dataflow Equations

**Choice:** Per block B:
```
use(B) = variables used in B before any def in B
def(B) = variables defined in B
live_in(B) = use(B) ∪ (live_out(B) \ def(B))
live_out(B) = ∪ { live_in(S) | S ∈ successors(B) }
```

**Rationale:** Standard backward dataflow equations from any compiler
textbook. The lattice is the powerset of all variables, ordered by
subset. Each block's transfer function is `f(out) = use ∪ (out \ def)`,
which is monotone on this lattice.

### 2. Fixed-Point Iteration

**Choice:** Initialize all `live_in` and `live_out` rows to empty and run
whole-graph synchronous rounds. Each round recomputes every block from the
previous round's `live_in` matrix. Stop when one round makes no change.

**Rationale:** Synchronous rounds keep the implementation and tests simple.
The lattice has finite height, the transfer function is monotone, and each
bit can only flip from false to true once in `live_in` and once in
`live_out`, so stabilization happens within `2 × V × B` rounds.

### 3. Lattice

**Choice:** `Set<u32>` per block for live_in and live_out. The lattice
is `(P(Vars), ⊆)` with meet = union (since liveness flows upward:
more live variables is "lower" in the backward analysis).

**Rationale:** Using union (not intersection) because liveness is a
"may" analysis: a variable is live if it *may* be used along some path.

### 4. Use-Def Characterization

**Choice:** Prove equivalence: `v ∈ live_in(B)` iff there exists a path
from the start of B to a use of v with no intervening definition of v.
The path-based characterization uses fuel-bounded reachability.

**Rationale:** This is the semantic correctness of the analysis. The
dataflow equations are a procedure; the path-based definition is the
specification.

### 5. Connected Live Ranges

**Choice:** Model connectivity through witness paths: if `v` is live-in at a
block because some def-free path reaches a later use, then every block on
that witness path belongs to `live_range(v)`.

**Rationale:** This is the path-local form the implementation can check and
property-test directly. It still captures the "no gaps along the reason v is
live" fact needed by downstream analyses.

## Risks / Trade-offs

**[Per-block granularity]** → Per-instruction liveness is more precise
but quadratically more expensive. Per-block is the standard starting
point. A refinement module could extend to per-instruction.

**[Empty exit live-out]** → The model keeps `live_out(exit)` empty. Return
values are represented as local uses in the exit block rather than as a
special post-exit live-out convention.

**[Path quantification in spec]** → The use-def characterization
quantifies over paths. Same fuel-bounded approach as `reachability.rs`
and `dominator_tree`.
