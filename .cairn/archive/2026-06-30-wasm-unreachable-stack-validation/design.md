## Context

The existing Wasm module exposes stack-value execution for `i32.add` and control-label arity checks for branches and returns. It does not yet model validator reachability. WebAssembly validation treats unreachable code as stack-polymorphic: later instructions may pop expected operands even when the current operand stack does not contain them, until a control boundary restores a reachable context.

## Decisions

### 1. Model validation state separately from runtime values

**Choice:** Add `WasmValidationState { operand_types, control, reachable }`.

**Rationale:** Validation only needs value types, not runtime value bits. Keeping this separate avoids mixing runtime execution with validation admission.

### 2. Reachable paths require exact suffixes

**Choice:** When `reachable == true`, validation helpers check the operand-type stack suffix against the instruction's expected input types.

**Rationale:** This preserves the existing exact stack-discipline model and gives negative tests clear failure behavior.

### 3. Unreachable paths are stack-polymorphic

**Choice:** When `reachable == false`, input-pop checks are admitted without requiring the operand-type suffix. Result types are still recorded on the operand-type stack.

**Rationale:** This captures the useful WebAssembly unreachable-polymorphism rule without modeling control-frame stack heights yet.

### 4. Control transfers mark the current path unreachable

**Choice:** Successful `br`, `return`, and `unreachable` validation set `reachable` to false and leave control/operand vectors otherwise intact, except typed operations may append their declared result types.

**Rationale:** The slice validates admission and reachability transitions, not branch execution or stack truncation. Later sequence-validation work can compose exact control-frame height restoration on top.

## Non-Goals

- Full instruction-sequence validation.
- Control-frame operand-stack height restoration.
- Block entry/exit validation.
- Table, `call_indirect`, binary decoding, or module validation.

## Risks / Trade-offs

- The state intentionally does not truncate operands after branch/return. That keeps this slice small and honest: it proves admission plus unreachable transition, while deferring structured control-frame stack-height accounting.
- The unreachable model records result pushes even while unreachable. This is enough to test polymorphic pops and gives later validator slices concrete state to consume.
