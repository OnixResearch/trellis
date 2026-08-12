## Context

The rustc-dev-guide presents dataflow as an analysis trait over a finite domain with direction, initialization, transfer effects, and a fixpoint engine. Trellis already has specialized liveness and constant-propagation convergence facts. A generic finite framework can make future MIR analyses more uniform and reduce duplicate proof work.

## Decisions

### 1. Use a finite indexed domain

**Choice:** The framework should model domains as finite rows over declared element states with explicit domain height metadata.

**Rationale:** Verus convergence proofs need a finite measure. The model should avoid abstract host traits whose laws cannot be checked.

### 2. Admit semilattice laws explicitly

**Choice:** Analysis admission must require deterministic join, bottom, top, idempotence, commutativity, associativity, bottom identity, and top absorption over the finite domain.

**Rationale:** These laws are the preconditions for stable fixed-point behavior. Keeping them explicit makes invalid analyses reject before iteration.

### 3. Separate direction from effect order

**Choice:** Forward and backward analyses determine traversal of statements/terminators, while `before` effects are always applied immediately before the primary effect for the same MIR location.

**Rationale:** This matches the guide's important distinction and prevents consumers from assuming backward analyses reverse before/primary semantics.

### 4. Model per-edge effects as edge-local transfer

**Choice:** Terminators may provide outgoing-edge effects that refine the state for a specific successor edge, with invalid successor edges rejected by analysis admission.

**Rationale:** Call-return, unwind, and branch-sensitive effects need edge-local precision without requiring full MIR semantics.

### 5. Prove convergence through monotone bounded ascent

**Choice:** Framework proofs should require monotone transfer and finite height, then establish stabilization within an explicit named budget derived from domain height and block/location count.

**Rationale:** This is reusable across liveness, initializedness, and similar bitvector analyses.

### 6. Keep results inspection pure

**Choice:** Results cursors and visitors are modeled as pure queries over fixed-point state snapshots and MIR locations, not mutable compiler cursors.

**Rationale:** Trellis should prove result availability and location consistency, not rustc object lifecycle behavior.

## Risks / Trade-offs

- A generic framework can become too abstract for Verus. Keep the first version over finite explicit tables, not arbitrary Rust traits.
- Convergence budgets may be conservative. That is acceptable if named and documented.
- Analysis-specific soundness remains out of scope; each instantiated analysis must prove its own semantic theorem.
