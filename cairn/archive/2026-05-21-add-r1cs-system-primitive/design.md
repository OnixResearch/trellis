## Context

`r1cs_constraint` models one admitted row and one witness. Multi-row system satisfaction should compose this primitive directly, preserving the same finite bounds and avoiding a separate arithmetic policy.

## Decisions

### Bounded three-row system

**Choice:** Represent a system as `len`, `modulus`, and three physical `R1csRow` slots.

**Rationale:** This mirrors the existing three-slot row/witness primitive, keeps proof obligations small, and is enough to establish the semantic layer that future gate encodings can target.

### Shared witness and modulus admission

**Choice:** A system is valid only when active rows are individually valid and use the same modulus as the system. Satisfaction additionally requires the shared witness to satisfy each active row.

**Rationale:** R1CS systems are sets of constraints over one field and one assignment. Encoding that boundary now prevents future gates from silently mixing fields or row-local witnesses.

## Risks

- **Overclaiming proving-system soundness:** Mitigated by documentation and requirements that scope the primitive to deterministic admission only.
- **Inactive row confusion:** Mitigated by tests showing inactive malformed rows are ignored while active malformed rows reject.
