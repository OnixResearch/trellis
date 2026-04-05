## Context

Register allocation follows liveness analysis: the interference graph
encodes which variables can't share a register, and graph coloring
assigns registers. The repo has 2-coloring (`graph_color.rs`) but not
K-coloring, interference construction, spilling, or coalescing.

## Goals / Non-Goals

**Goals:**
- Interference graph from live ranges
- K-coloring for K physical registers
- Spill insertion (store/reload) with semantics preservation
- Move coalescing with Briggs criterion
- All-variables-assigned completeness

**Non-Goals:**
- Linear scan allocation (different algorithm, separate module)
- Register pressure estimation
- Calling convention register constraints (separate module)
- SIMD/vector register classes
- Iterated register coalescing (IRC) — we model basic coalescing

## Decisions

### 1. Interference Graph

**Choice:** Adjacency matrix `Seq<Seq<bool>>` where `graph[i][j] = true`
means variables i and j interfere. Symmetric: `graph[i][j] == graph[j][i]`.

**Rationale:** Simple and amenable to Verus quantifier reasoning. The
matrix size is bounded by the number of variables (known at analysis time).

### 2. Interference Construction

**Choice:** For each program point P, for each pair of variables (v, w)
both live at P: add edge (v, w). Two variables interfere iff there exists
a program point where both are live.

**Rationale:** Direct from the definition. Uses `live_out(B)` from
liveness analysis. The proof is: if the edge is present, both are live
somewhere simultaneously; if absent, no such point exists.

### 3. K-Coloring

**Choice:** Greedy coloring with simplification: repeatedly remove a node
with degree < K (it can always be colored), push on stack. When no such
node exists, select a node to spill. Then pop and color.

**Rationale:** Chaitin's algorithm. Not optimal (graph coloring is
NP-complete) but correct: the proof shows that a node with degree < K
can always receive a color not used by its at most K-1 neighbors.

### 4. Coloring Validity

**Choice:** `valid_coloring(graph, colors, K) -> bool` checks: for all
edges (i, j), `colors[i] != colors[j]`, and `forall i, colors[i] < K`.

**Rationale:** The standard graph coloring validity predicate. The proof
of the algorithm shows it produces a valid coloring (or identifies spills).

### 5. Spill Insertion

**Choice:** Spilling variable v: insert a store after v's definition,
insert a reload before each use of v. The reloaded value is a fresh
variable with a new (short) live range that doesn't interfere with the
original spill set.

**Rationale:** Correct spilling must not change semantics: the reload
reads the value that was stored. The proof shows the store-reload pair
preserves the data flow: the reloaded variable has the same value as v
at the definition point.

### 6. Coalescing

**Choice:** Two variables connected by a copy can be coalesced (merged
into one) if they don't interfere. The Briggs criterion adds a safety
check: the merged node has fewer than K high-degree neighbors (degree ≥ K).

**Rationale:** Briggs is conservative — it never makes a colorable graph
uncolorable. The proof shows: if the original graph is K-colorable and
the Briggs criterion is satisfied, the coalesced graph is K-colorable.

## Risks / Trade-offs

**[Greedy coloring suboptimality]** → The greedy approach may spill
unnecessarily. Optimal coloring is NP-complete. The model proves
correctness of whatever coloring is produced, not optimality.

**[No register classes]** → Real architectures have integer, float, and
vector register classes with different K values. The model uses a single K.
Extending to multiple classes requires separate coloring per class.

**[Spill cost not modeled]** → The choice of which variable to spill
affects performance but not correctness. The model proves any spill
decision is safe.
