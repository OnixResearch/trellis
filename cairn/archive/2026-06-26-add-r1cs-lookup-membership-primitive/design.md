## Context

The current R1CS stack is intentionally finite: witnesses and rows use small active prefixes, booleanity composes through one row, and bit decomposition handles a two-bit value. Lookup membership should follow the same shape rather than introducing vectors or proof-system-specific lookup protocols.

## Goals / Non-Goals

**Goals:**
- Represent a fixed three-entry lookup table with an active prefix length.
- Admit only valid witness lengths, valid value witness indices, valid active table lengths, and bounded table elements.
- Evaluate satisfaction as exact membership of the selected witness value in the active table prefix.
- Provide Verus-checked exec/spec equivalence, proof/catalogue functions, and positive/negative tests.

**Non-Goals:**
- No PLONK-style lookup argument, polynomial commitment, sorted table, multiplicity, table hashing, or unbounded table vector.
- No full circuit builder or proving-system API.

## Decisions

### 1. Fixed three-entry table

**Choice:** Store `table_len`, `table0`, `table1`, and `table2` directly in the primitive.

**Rationale:** This mirrors existing bounded R1CS surfaces and keeps Verus proofs direct. A three-entry table is enough to exercise active-prefix membership and inactive-slot behavior.

**Alternative:** Use `Vec<u16>` or arbitrary table widths. Rejected for this slice because the current R1CS primitive layer intentionally avoids unbounded structures.

### 2. Witness-slot value membership

**Choice:** The lookup value is read from a nonzero witness slot inside the active witness prefix; satisfaction checks that value against the active table prefix.

**Rationale:** This composes naturally with later gadgets that write semantic values into witness slots.

**Alternative:** Store the lookup value directly in the lookup struct. Rejected because witness-slot membership is closer to circuit semantics.

## Risks / Trade-offs

**Tiny table scope** → This is intentionally not a practical lookup argument. The mitigation is explicit docs/spec wording and tests for inactive entries so future widening has a clean semantic base.

**Element bounds drift** → Shape admission reuses the same bounded element predicate as R1CS rows/witnesses, and Tracey/Verus coverage tags bind the docs to the implementation.
