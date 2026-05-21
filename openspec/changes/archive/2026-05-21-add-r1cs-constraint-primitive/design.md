## Context

R1CS satisfaction is normally defined over a field as `<A,w> * <B,w> = <C,w>`. Trellis should start with the smallest finite executable admission boundary rather than a broad circuit framework.

## Goals / Non-Goals

**Goals:**
- Provide an overflow-safe bounded row model with row length, modulus, three coefficient vectors, and witness length/value metadata.
- Make executable helpers refine explicit Verus specs.
- Include positive and negative runtime evidence for valid satisfaction, unsatisfied equations, and length mismatch.

**Non-Goals:**
- No proof-system integration or cryptographic claims.
- No unbounded vectors or arbitrary field arithmetic.
- No multi-row constraint-system aggregation yet.

## Decisions

### 1. Fixed arity-three envelope

**Choice:** Represent one row and witness with up to three active slots.
**Rationale:** This matches existing Trellis finite-envelope primitives and keeps proofs local.
**Alternative:** `Vec`-backed rows were rejected for the first slice because length/index invariants would dominate the initial circuit primitive.

### 2. Bounded field elements

**Choice:** Admit moduli in `[2, 251]` and active coefficients/witness values at most `15`.
**Rationale:** Dot products and the final product stay within `u16`, avoiding overflow trust while preserving exact modular semantics for the finite model.
**Alternative:** Wider `u64` arithmetic would need more proof scaffolding or checked-arithmetic plumbing before the first circuit primitive.

### 3. Satisfaction includes shape admission

**Choice:** `r1cs_row_satisfied` returns true only when row and witness are valid, lengths match, and the modular equation holds.
**Rationale:** Public callers get one conservative admission predicate and cannot accidentally ignore malformed shape.

## Risks / Trade-offs

- **Limited arity** -> This is intentional. Later slices can add row arrays or compiled gate helpers once the single-row contract is stable.
- **Small element bound** -> Keeps the first primitive proof-friendly; future field modules can widen it with checked arithmetic.
