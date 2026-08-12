## Context

`r1cs_constraint` is the finite base layer for row shape, dot-product helpers, and single-row satisfaction. The exec functions already prove equality to the specs. The review gap is that two coverage proofs are catalogue-only and therefore do not expose the concrete implications named by the requirements.

## Decisions

### 1. State coverage as local implications

**Choice:** Strengthen the catalogue proofs by taking row/witness parameters and stating implications over existing specs: valid length predicates imply admitted lengths, malformed rows/witnesses reject satisfaction, length mismatch rejects satisfaction, and satisfied rows decompose into valid row, valid witness, matching length, and dot-product equality.

**Rationale:** These facts are already present in the definitions or local helper lemmas, so the change improves reviewability without adding proof assumptions or widening runtime semantics.

### 2. Keep runtime tests finite

**Choice:** Add only focused tests for missing row-boundary cases surfaced by the strengthened coverage contract.

**Rationale:** The module intentionally uses bounded rows and witnesses with three slots. Tests should cover finite happy and sad paths without introducing vectors, circuit builders, or proving-system APIs.

## Risks / Trade-offs

- Overly arithmetic-heavy postconditions can make Verus brittle. The mitigation is to phrase coverage in terms of existing specs rather than raw modular arithmetic where possible.
- This does not close permanent crypto trust boundaries and does not change R1CS semantics.
