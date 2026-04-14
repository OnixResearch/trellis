## Context

Natural loops are defined from dominators and back edges. This repo already
has reachability and dominator modules, so loop detection builds on those
views instead of recomputing graph facts inside every helper.

The shipped module exposes exact helpers over:
- a CFG edge list
- a reachability bitmap for the chosen entry
- a dominance matrix for the same graph

Randomized tests then compare the computed reachable non-self loop bodies
against a brute-force path oracle.

## Goals / Non-Goals

**Goals:**
- Back edge classification via dominance
- Exact loop-body closure over supplied reachability/dominator views
- Header dominance check for loop bodies
- Exact single-entry check over supplied reachability view
- Loop nesting as strict body inclusion
- Nesting forest structure
- Exit edge identification

**Non-Goals:**
- Irreducible loop handling
- Loop transformations (unrolling, peeling, fusion)
- Loop-carried dependency analysis
- Trip count estimation
- Proving the path-oracle equivalence directly inside Verus

## Decisions

### 1. Back Edge Classification

**Choice:** `is_back_edge(dom, src, dst) -> bool` returns whether `dst`
dominates `src`. `find_back_edges(edges, dom)` filters the CFG edge list
exactly by that predicate.

**Rationale:** Standard definition. It is easy to state and verify exactly
against a dominance matrix.

### 2. Loop Body Computation

**Choice:** `compute_loop_body(edges, reachable, dom, header, source)`
returns the seeded backward closure for `(source -> header)`:
start from `{header, source}`, then for `|V|` rounds add any reachable node
that is dominated by `header` and has an edge into the current body.
For non-self loops, `header` is treated as a barrier during the interior
closure step.

**Rationale:** This closure matches the actual verified implementation and
is strong enough for the natural-loop properties used here. Tests compare it
against the path-style natural-loop oracle for reachable non-self loops.

### 3. Loop Representation

**Choice:** `NaturalLoop { header: usize, body: Vec<bool> }`.

**Rationale:** The public struct stores only the facts reused across nesting
operations. Back edges and exits are queried separately by dedicated helpers.
If callers want to merge several back edges with the same header into one
body, they can union the `Vec<bool>` bodies explicitly.

### 4. Header Dominance Proof

**Choice:** `header_dominates_body(dom, header, body)` checks exactly that
all body nodes are dominated by `header` in the supplied dominance matrix.

**Rationale:** This is the defining structural fact used by later passes.
It is independent from how the body was obtained.

### 5. Single Entry

**Choice:** `has_single_entry(edges, reachable, header, body)` checks exactly
that every reachable edge entering `body` from outside targets `header`.

**Rationale:** This is the natural-loop single-entry property stated in terms
of the supplied reachability view, so the exec helper can prove an exact
`ensures` clause.

### 6. Loop Nesting

**Choice:** `nested_in(inner, outer)` means strict body inclusion.
`loop_nesting_forest` records a strict-superset parent for each loop and,
when several ancestors are possible, deterministically chooses the smallest
strict superset (ties break by lower index).

**Rationale:** Strict inclusion gives the right partial order. Deterministic
parent selection makes tests and downstream use stable.

### 7. Exit Edges

**Choice:** `loop_exits(edges, body)` returns exactly the edges `(src, dst)`
with `src` inside the body and `dst` outside.

**Rationale:** Exit edges are a direct body partition query and verify cleanly
as an exact filtered edge list.

## Risks / Trade-offs

**[Irreducible loops]** → Irreducible loops are still out of scope. This
module focuses on natural-loop structure.

**[Semantic equivalence lives in tests]** → The strongest proof-level body
contract is the closure definition above. Agreement with the path-style
oracle is established by unit/property tests, not a separate Verus theorem.

**[Merged-by-header loops are caller-side]** → The public `NaturalLoop`
struct does not store multiple back edges or precomputed exits. That keeps
the verified core smaller but leaves header-merging as an explicit caller
step when needed.
