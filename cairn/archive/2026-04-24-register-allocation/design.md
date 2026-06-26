## Context

This repo verifies small compiler/dataflow kernels as exact value-level models.
For register allocation, the stable first landing is the allocation core rather
than a whole backend pass: live-range overlap, interference edges, conflict-free
register assignment, and explicit spill fallback.

## Goals / Non-Goals

**Goals:**
- Model live ranges as a rectangular bool matrix over variables × program points
- Build an exact symmetric interference graph with no self-loops
- Represent each variable assignment as either a register or a spill slot
- Provide a deterministic first-fit allocator with spill fallback
- Prove the produced assignment is complete and conflict-free

**Non-Goals:**
- Rewriting IR with store/reload spill code
- Preserving whole-program semantics across spill insertion
- Coalescing, Briggs criteria, or copy elimination
- Architecture-specific register classes or calling-convention constraints
- Optimal coloring or minimum-spill claims

## Decisions

### 1. Live ranges are caller-supplied matrix rows

**Choice:** Input live ranges as `Vec<Vec<bool>>` where each row is one variable
and each column is one program point.

**Rationale:** This keeps the allocation core independent from a particular IR or
CFG while still matching the exact overlap predicate used by liveness-style
reasoning.

### 2. Interference graph stays an adjacency matrix

**Choice:** Use `Vec<Vec<bool>>` / `Seq<Vec<bool>>` with `graph[i][j]` meaning
variables `i` and `j` interfere.

**Rationale:** Symmetry, no-self-loop, and exact-overlap proofs are straightforward
with matrix indexing and mirror existing verified graph/dataflow modules.

### 3. Allocation is register-or-spill, never unassigned

**Choice:** Represent one variable's outcome as either `register: Some(r)` or
`spill_slot: Some(s)`, but not both and not neither. Spill slots are assigned
deterministically as `s = variable_index`, so spilled variables always receive
distinct slot IDs in this core model.

**Rationale:** This makes completeness explicit and avoids a second pass just to
prove every variable ended up somewhere. Reusing `variable_index` also avoids a
second allocator just to number spill slots.

### 4. Greedy order is deterministic prefix order

**Choice:** Allocate variables in increasing variable index order. For each
variable, pick the lowest register not already used by an interfering earlier
variable; otherwise assign spill slot `variable_index`.

**Rationale:** Deterministic prefix order is easy to test, easy to review, and its
safety proof only needs a prefix conflict invariant.

### 5. Spill fallback is abstract, not a program rewrite

**Choice:** A spill means "no register available in the current allocator model";
it records a spill slot but does not insert loads/stores.

**Rationale:** This keeps the landed slice honest and small. Program rewriting can
be a later change built on top of the completed allocation core.

### 6. Public builders fail closed on malformed matrices

**Choice:** Public constructors return `None` on ragged live-range matrices or
ragged adjacency matrices instead of using caller-side `requires` clauses.

**Rationale:** This keeps the public surface executable and makes malformed-input
negative tests straightforward.

## Risks / Trade-offs

**[Greedy order may spill unnecessarily]** -> Accepted. This change proves
correctness of the produced assignment, not optimality.

**[No semantic spill proof]** -> Accepted. The landed model exposes spill slots as
an allocation result only; later work can refine this into rewrite semantics.

**[Rectangular live-range input required]** -> Accepted. Public builders/checkers
return `None` for ragged matrices instead of silently normalizing them or
panicking.