## Context

`r1cs_system` composes bounded `r1cs_constraint` rows into one finite multi-row envelope with a shared modulus and a single witness. The runtime helpers already prove equality to their specs. The remaining weakness is reviewability: the catch-all coverage proof is catalogue-only and therefore does not expose the concrete implications named by the system coverage requirement.

## Decisions

### 1. Strengthen coverage with local proof implications

**Choice:** Replace the catch-all `ensures true` coverage proof with postconditions that quantify over systems and witnesses and restate the concrete rejection and satisfaction-decomposition implications already provided by local definitions.

**Rationale:** These facts are definitional over existing specs, so the change improves reviewability without adding proof assumptions, widening runtime semantics, or touching trusted boundaries.

### 2. Add finite negative tests for missing system boundaries

**Choice:** Add focused cargo tests for invalid system length, malformed active row, malformed witness value, and failed third active row.

**Rationale:** Existing tests cover two-row happy paths, one failed active row, active modulus mismatch, witness length mismatch, and ignored inactive rows. The new tests make the finite edge cases named by the strengthened coverage contract explicit without introducing circuit builders or proving-system APIs.

## Risks / Trade-offs

- Quantified proof postconditions can become brittle if they inline too much row arithmetic. The mitigation is to phrase the coverage proof in terms of existing specs and local helper proof functions.
- This does not widen the R1CS system model; scope remains the bounded multi-row adapter only.
