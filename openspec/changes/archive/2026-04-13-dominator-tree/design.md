## Context

The repo already has graph helpers such as `reachability.rs`,
`cycle_detect.rs`, `topo_sort.rs`, and `control_flow_graph.rs`.
Dominator analysis fits beside them as the compiler-facing layer that later
unlocks loop detection and SSA work.

The shipped implementation in `src/dominator_tree.rs` works over an edge list
`Seq<(usize, usize)>` plus an explicit node count and entry index. It keeps
proof obligations small inside exec code, then checks semantic behavior with
unit tests and proptests against small brute-force CFG oracles.

## Goals / Non-Goals

**Goals:**
- Validate CFG edge endpoints
- Compute a stable dominance relation for reachable nodes
- Extract immediate dominators as a parent map rooted at the entry
- Compute dominance frontiers for join points
- Keep the change aligned across code, requirements, README, and OpenSpec

**Non-Goals:**
- Lengauer-Tarjan or other asymptotically faster dominator algorithms
- Postdominators
- Control dependence graphs
- Incremental updates after CFG mutation

## Decisions

### 1. Graph Representation

**Choice:** Represent the CFG as an edge list `(src, dst)` plus `n` and
`entry` arguments.

**Rationale:** This matches other graph modules in the repo and keeps
predecessor scanning simple inside verified loops.

### 2. Dominance Model

**Choice:** Represent computed dominance as a boolean matrix
`dom[node][candidate]`.

**Rationale:** The matrix form gives direct shape invariants, simple indexed
queries, and an easy bridge to immediate-dominator extraction and frontier
checks.

### 3. Iterative Algorithm

**Choice:** Use classic iterative dominator-set equations over the matrix.
Initialize the entry row to only itself, reachable non-entry rows to `true`
for all candidates, unreachable rows to `false`, then repeatedly recompute
reachable non-entry rows as the intersection of predecessor rows plus the
node itself.

**Rationale:** This is straightforward to implement with loops and compact
Verus invariants. It also matches the fixed-point story required by the
spec without introducing a more specialized idom-only algorithm.

### 4. Immediate Dominators

**Choice:** Derive `Seq<Option<usize>>` immediate dominators from the final
matrix by selecting the strict dominator that is closest to the node.

**Rationale:** The public tree view is what downstream passes want, but the
matrix is the easier internal proof and execution structure.

### 5. Dominance Frontier

**Choice:** Compute the frontier by iterating edges and checking whether a
candidate node dominates a predecessor of the edge target without strictly
dominating the target itself.

**Rationale:** This matches the standard SSA-facing definition and avoids a
second graph representation.

## Risks / Trade-offs

**[Semantic proof depth]** -> A fully internalized all-paths dominator proof is
heavier than the rest of this change needs. The current module proves shape
and safety facts directly in Verus and checks semantic behavior with focused
unit tests and proptests against brute-force small CFG oracles.

**[Iteration bound]** -> The implementation runs a bounded number of outer
iterations (`n`). That is easy to reason about and sufficient for the
current finite fixed-point model, but it is not tuned for asymptotic speed.
