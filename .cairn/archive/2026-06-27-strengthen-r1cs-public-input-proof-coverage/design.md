## Context

`r1cs_public_inputs` binds bounded public values to witness indices and composes those bindings with `r1cs_system` satisfaction. The runtime helpers already prove equality to their specs. The remaining weakness is reviewability: the catch-all coverage proof is catalogue-only and therefore does not expose the concrete implications named by the public-input coverage requirement.

## Decisions

### 1. Strengthen coverage with local proof implications

**Choice:** Replace the catch-all `ensures true` coverage proof with postconditions that quantify over public bindings, witnesses, moduli, and systems and restate the concrete rejection and satisfaction-decomposition implications already provided by local definitions.

**Rationale:** These facts are definitional over existing specs, so the change improves reviewability without adding proof assumptions, widening runtime semantics, or touching trusted boundaries.

### 2. Add finite negative tests for missing binding boundaries

**Choice:** Add focused cargo tests for invalid public-input length, invalid modulus, malformed public value, and active third-slot binding behaviour.

**Rationale:** Existing tests cover two active bindings, public statements, mismatched values, out-of-range indices, malformed witnesses, unsatisfied systems, ignored inactive slots, and empty public lists. The new tests make the finite edge cases named by the strengthened coverage contract explicit without introducing proof objects or cryptographic APIs.

## Risks / Trade-offs

- Quantified proof postconditions can become brittle if they inline too much modular arithmetic. The mitigation is to phrase the coverage proof in terms of existing specs and local helper proof functions.
- This does not widen the public-input statement model; scope remains the bounded statement-admission adapter only.
