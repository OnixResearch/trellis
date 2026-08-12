## Context

The existing Wasm module models values, `i32.add`, locals, and linear memory. Its docs intentionally excluded control labels and validation judgments. This change keeps that broader validator out of scope and adds only the pure arity predicates that the next validator slice can reuse.

## Decisions

### 1. Keep label modeling explicit and finite

**Choice:** Add `WasmLabelKind::{Function, Block, Loop}` and `WasmControlLabel { kind, params, results }`.

**Rationale:** Block/function branch targets consume their result types, while loop branch targets consume their parameter types. Keeping both vectors on the label makes that distinction local and testable without a full instruction-sequence validator.

### 2. Validate branch depth from the top of the control stack

**Choice:** Interpret branch depth `0` as the innermost label, stored at the end of the control-stack vector.

**Rationale:** This matches WebAssembly branch-depth convention while keeping the exec helper a simple bounds check plus suffix-type scan.

### 3. Return arity uses the root function label

**Choice:** `return_arity_ok` accepts only control stacks whose first label is `Function`, then checks the operand-stack suffix against that label's result types.

**Rationale:** A return targets the current function result arity, not the innermost block. This gives callers a narrow executable admission check and rejects malformed control stacks explicitly.

### 4. Preserve functional-core shape

**Choice:** Add pure spec predicates for target-type selection and stack-suffix typing, then keep public exec helpers as thin scans over those predicates.

**Rationale:** The core behavior remains deterministic and testable without I/O, global state, or a full Wasm interpreter.

## Non-Goals

- Whole instruction-sequence validation.
- Unreachable-stack polymorphism.
- Table, element, `call_indirect`, or binary decoding semantics.
- Branch execution or control transfer side effects.

## Risks / Trade-offs

- The model admits only arity/type checks, not reachability or stack truncation after a branch. This is intentional; later validation slices can compose with these helpers.
- Label stacks are caller-provided. Malformed stacks are rejected by the public helpers where relevant instead of normalized by constructors.
