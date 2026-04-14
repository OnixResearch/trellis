## Context

Natural loops are defined in terms of dominators and back edges. The repo
will have dominator tree and reachability modules. Loop detection builds
directly on both: classify edges as back/forward/cross using dominance,
then compute loop bodies via backward reachability from the back-edge
source to the header.

## Goals / Non-Goals

**Goals:**
- Back edge classification via dominance
- Loop body computation via backward reachability
- Header dominance proof (header dominates all body nodes)
- Single entry proof (only the header has external predecessors)
- Loop nesting as subset of body sets
- Nesting forest structure
- Exit edge identification

**Non-Goals:**
- Irreducible loop handling (irreducible loops don't have natural loop
  structure — they're detected by cycle detection instead)
- Loop transformations (unrolling, peeling, fusion)
- Loop-carried dependency analysis
- Trip count estimation

## Decisions

### 1. Back Edge Classification

**Choice:** `is_back_edge(cfg, domtree, src, dst) -> bool` returns
`dominates(domtree, dst, src)`. An edge (src → dst) where dst dominates
src is a back edge.

**Rationale:** Standard definition. Forward edges go from dominator to
dominated. Cross edges connect unrelated subtrees. Only back edges form
loops.

### 2. Loop Body Computation

**Choice:** Given back edge (B → H), the loop body is
`{H} ∪ {N | N can reach B without going through H}`. Computed by
backward BFS from B, stopping at H.

**Rationale:** The standard "natural loop" body definition. The backward
search finds all nodes that "feed into" the back edge. H is included
because it's the loop header. Implemented using `reachability.rs` on
the reverse CFG with H removed.

### 3. Loop Representation

**Choice:** `NaturalLoop { header: u32, back_edges: Seq<u32>, body: Set<u32>, exits: Set<(u32, u32)> }`.

**Rationale:** A loop can have multiple back edges to the same header
(e.g., `continue` from different points). The body is the union of bodies
from all back edges. Exits are edges from body nodes to non-body nodes.

### 4. Header Dominance Proof

**Choice:** Prove `forall n in body :: dominates(domtree, header, n)`.
By construction: H dominates B (back edge definition), and every node
in the body reaches B without leaving through H. Since H dominates the
entry to the body (H itself) and every path to a body node from outside
goes through H, H dominates all body nodes.

**Rationale:** This is the defining property of natural loops. The proof
follows from the back-edge definition and the transitivity of dominance.

### 5. Loop Nesting

**Choice:** Loop L1 is nested in loop L2 iff `L1.body ⊂ L2.body` and
`L1.header ∈ L2.body`. The nesting relation is a partial order (proved
by subset transitivity).

**Rationale:** Standard definition. The nesting depth of a node is the
number of loops whose body contains it. The nesting forest has outermost
loops as roots and innermost loops as leaves.

### 6. Exit Edges

**Choice:** `loop_exits(loop) -> Set<(u32, u32)>` returns all edges
(src, dst) where `src ∈ loop.body` and `dst ∉ loop.body`.

**Rationale:** Exit edges are where loop-specific optimizations must
maintain correctness (e.g., LCSSA phi placement at exit blocks). The
set is computed by scanning all edges from body nodes.

## Risks / Trade-offs

**[Irreducible loops missed]** → Irreducible loops (multiple entries)
aren't natural loops. They're detected by `cycle_detect.rs` but don't
have the header-dominates-body property. Some compilers (GCC) transform
irreducible loops into reducible ones; that's a transformation, not
detection.

**[Multiple back edges]** → A loop with multiple back edges to the same
header is one loop, not multiple. The body is the union. Some algorithms
treat each back edge separately; we merge per header.
