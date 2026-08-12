## Why

The Wasm primitive model now covers single-depth branches, returns, unreachable-stack validation, control-frame restoration, and finite `call_indirect` table/signature admission. A remaining structured-control validation gap is `br_table`: multi-branch validation must ensure every branch depth is in range, every branch target has the same expected operand types as the default target, and reachable states provide those target operands plus the `i32` selector.

This change lands a compact `br_table` validation slice without attempting instruction-sequence validation, runtime branch selection, or full module/control-flow execution.

## What Changes

- Add branch-table target matching specs over finite depth lists and the default depth.
- Add a `validate_br_table` helper that checks all branch-table targets against the default target and marks successful validation paths unreachable.
- Preserve existing unreachable-stack polymorphism: unreachable paths skip operand suffix checks but still reject malformed depths or mismatched target arity.
- Add positive and negative tests for matching block/loop targets, out-of-range depths, target-type mismatch, reachable stack mismatch, and unreachable stack-polymorphic admission.
- Sync requirements, accepted specs, README, crate docs, and archive evidence.

## Impact

- **Files**: `src/wasm_primitives.rs`, `docs/requirements.md`, `cairn/specs/wasm/spec.md`, legacy `openspec/specs/wasm/spec.md`, `README.md`, `src/lib.rs`, and this Cairn change package.
- **APIs**: new branch-table validation helper for the existing Wasm validation state.
- **Testing**: focused Wasm tests before/after implementation, Verus proof check, full cargo/clippy/Tracey/ensures validation, and Cairn gates.