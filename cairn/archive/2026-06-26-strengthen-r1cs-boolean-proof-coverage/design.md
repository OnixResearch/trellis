## Context

`r1cs_boolean` models a deliberately finite booleanity constraint as one existing bounded R1CS row. The runtime helpers already prove equality to their specs. The remaining weakness is reviewability: the catch-all coverage proof is catalogue-only and therefore does not expose the concrete implications named by the boolean coverage requirement.

## Decisions

### 1. Strengthen coverage with local proof implications

**Choice:** Replace the catch-all `ensures true` coverage proof with postconditions that quantify over booleans and witnesses and restate the concrete rejection and row-validity implications already provided by local helper proofs.

**Rationale:** These facts are already definitional or proved locally, so the change improves reviewability without adding proof assumptions, widening runtime semantics, or touching trusted boundaries.

### 2. Add finite negative tests for missing boolean boundaries

**Choice:** Add focused cargo tests for malformed witness values, a valid-but-unsatisfied encoded row, and row encoding at a different boolean slot.

**Rationale:** Existing tests cover common happy/sad paths. The new tests make the finite edge cases named by the strengthened coverage contract explicit without introducing vectors, proof systems, or cryptographic APIs.

## Risks / Trade-offs

- Quantified proof postconditions can become brittle if they inline too much row arithmetic. The mitigation is to phrase the coverage proof in terms of existing specs and local helper proof functions.
- This does not widen the R1CS boolean model; scope remains the bounded row adapter only.
