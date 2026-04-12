## Context

The local seL4 primitives become most valuable when they compose into a kernel-state invariant. This final change gives the repo a place to state the global invariant and prove primitive steps preserve it.

## Goals / Non-Goals

**Goals:**
- State a composed kernel invariant.
- Decompose it into named local obligations.
- Prove primitive steps preserve it.

**Non-Goals:**
- C correspondence proofs
- Liveness or timing proofs
- Driver or boot invariants outside the modeled primitives

## Decisions

### 1. Compose from named sub-invariants

**Choice:** Compose from named sub-invariants
**Rationale:** Global proofs stay maintainable when broken into object, capability, scheduler, memory, and IRQ layers.

### 2. Add this change last

**Choice:** Add this change last
**Rationale:** The composition proof should depend on stable local modules rather than force early representation choices.

### 3. Preservation is per primitive step

**Choice:** Preservation is per primitive step
**Rationale:** This matches the repo's style of small verified transitions with sharp ensures.

## Risks / Trade-offs

**[Monolithic invariant proofs can become brittle]** Use named sub-invariants and preserve them independently where possible.
**[Dependency churn from earlier modules]** Land this change after local representations settle.
