## Context

The current Wasm primitive model validates stack effects for direct arithmetic, branch/return arity, unreachable paths, and structured control frames. It explicitly does not model tables or indirect calls. For MVP `call_indirect`, validation and admission need two independent facts: a table slot must be initialized with a function whose signature matches the expected type, and reachable operand stacks must provide the expected parameters followed by an `i32` runtime table index.

## Decisions

### 1. Model a finite function-signature table

**Choice:** Add `WasmFuncType { params, results }` and `WasmTable { entries: Vec<Option<WasmFuncType>> }`.

**Rationale:** This is enough to express exact signature matching and uninitialized-slot rejection without pulling in full module, element-segment, reference-type, or linker semantics.

### 2. Make signature equality structural and explicit

**Choice:** Define sequence-matching specs over value types, then define function-type matching as parameter and result sequence matching.

**Rationale:** Explicit structural matching is friendlier to Verus than relying on derived vector equality, and it keeps tests/proofs focused on exact type agreement.

### 3. Keep `call_indirect` stack effects aligned with unreachable polymorphism

**Choice:** Reachable `validate_call_indirect` requires operand suffix `params + [i32]`, consumes that suffix, and pushes result types. Unreachable validation still requires a valid matching table entry but skips operand suffix checks and synthesizes result types.

**Rationale:** Existing validation helpers already treat unreachable paths as stack-polymorphic for operand pops while preserving static/shape checks. This change follows that boundary.

## Non-Goals

- Whole module validation or type-index spaces.
- Element segment initialization semantics beyond the finite table entry state.
- Multi-table/reference-type validation.
- Runtime callee execution, traps after dynamic table-index lookup, or host/linker behavior.

## Risks / Trade-offs

- A concrete table index is used as a finite admission witness rather than modeling dynamic runtime lookup over all possible indices.
- `WasmFuncType` duplicates parameter/result vectors already used by labels; this keeps the table slice independent from structured-control metadata.