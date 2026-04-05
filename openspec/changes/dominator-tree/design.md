## Context

The repo has graph reachability (`reachability.rs`), cycle detection
(`cycle_detect.rs`), topological sort (`topo_sort.rs`), and forest
primitives (`forest.rs`). Dominators build on reachability — A dominates
B iff every path from entry to B includes A — and produce a forest
(the dominator tree).

## Goals / Non-Goals

**Goals:**
- Spec-level dominator predicate based on path enumeration
- Iterative CHK algorithm with convergence proof
- Immediate dominator as closest strict dominator
- Tree structure from idom relation
- Dominance frontier computation

**Non-Goals:**
- Lengauer-Tarjan algorithm (more complex, same result)
- Postdominators (symmetric, separate module)
- CDG (control dependence graph, derived from postdominators)
- Incremental dominator updates

## Decisions

### 1. Dominator Predicate

**Choice:** Spec function `dominates(cfg, entry, a, b) -> bool` defined as:
for all paths from `entry` to `b`, `a` appears on the path. Uses the
fuel-bounded path existence model from `reachability.rs`.

**Rationale:** Path-based definition is the mathematical standard. It's
not executable (quantifies over all paths), but it's the spec that the
algorithm must match.

### 2. CFG Representation

**Choice:** Adjacency list as `Seq<Seq<u32>>` where `cfg[i]` is the
list of successors of node i. Node 0 is the entry.

**Rationale:** Matches the representation used by `reachability.rs` and
`topo_sort.rs`. Node 0 as entry is a convention that simplifies the proofs.

### 3. Iterative Algorithm

**Choice:** CHK iterative data-flow: initialize `idom[entry] = entry`,
all others undefined. Repeatedly process nodes in reverse postorder,
computing `idom[n] = intersect(idom[p1], idom[p2], ...)` for all
predecessors p_i. Stop when no `idom` changes.

**Rationale:** CHK is the simplest correct dominator algorithm. It
converges in O(n²) worst case on reducible CFGs (most real programs)
and O(n³) on irreducible ones. The convergence proof uses the dominator
lattice: idom values can only move up (toward the entry), and the lattice
has finite height (bounded by the number of nodes).

### 4. Dominator Lattice

**Choice:** The lattice is `{entry, ..., n, ⊤}` where ⊤ means "undefined"
and the ordering is: a ≤ b iff a dominates b. Meet is the common dominator
closest to the node (the intersect function).

**Rationale:** Finite height = number of nodes. Monotone transfer
function (predecessors' idoms can only go up). Standard fixed-point
convergence follows.

### 5. Dominance Frontier

**Choice:** `dominance_frontier(cfg, idom, a) -> Set<u32>` defined as:
`{y | exists pred of y such that a dominates pred but a does not strictly
dominate y}`.

**Rationale:** Standard definition. Computed by iterating over all edges
and checking the dominance relationship. The DF is what SSA construction
needs for phi placement.

### 6. Immediate Dominator and Tree

**Choice:** `idom: Seq<Option<u32>>` where `idom[n] = Some(m)` means m is
n's immediate dominator. The entry has `idom[0] = None` (or self-loop).
The tree is the forest from `forest.rs` with `idom` as the parent array.

**Rationale:** Reuses the existing `forest.rs` for tree structure proofs.
The immediate dominator is unique (proved), so the parent array is
well-defined.

## Risks / Trade-offs

**[All-paths quantification in spec]** → The dominator spec quantifies
over all paths, which is not directly decidable. Verus can handle this
with fuel-bounded paths (as `reachability.rs` does), but deep CFGs may
need high fuel for completeness.

**[Irreducible CFGs]** → Most real CFGs are reducible (no irreducible
loops). The CHK algorithm handles irreducible CFGs but converges slower.
The proofs don't assume reducibility.
