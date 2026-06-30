## Why

The Wasm validator model now covers typed values, local/memory checks, structured control labels, unreachable-stack polymorphism, and control-frame stack-height restoration. A remaining MVP safety gap is indirect calls: `call_indirect` relies on a finite function table and must only admit entries whose actual function signature matches the expected type while the operand-type stack supplies the call arguments plus the runtime table index.

This change lands a compact table/signature slice without attempting full module linking, element segment initialization, multi-table reference types, or execution of the callee.

## What Changes

- Add finite `WasmFuncType` and `WasmTable` models to `wasm_primitives`.
- Add table-entry signature matching helpers for initialized entries and exact parameter/result agreement.
- Add `call_indirect` operand-stack and validation helpers that require expected parameters plus an `i32` table index when reachable, while preserving existing unreachable-stack polymorphism for operand pops.
- Add positive and negative tests for matching entries, out-of-range/uninitialized/signature-mismatch rejection, reachable stack mutation, and unreachable result synthesis.
- Sync requirements, accepted specs, README, crate docs, and archive evidence.

## Impact

- **Files**: `src/wasm_primitives.rs`, `docs/requirements.md`, `cairn/specs/wasm/spec.md`, legacy `openspec/specs/wasm/spec.md`, `README.md`, `src/lib.rs`, and this Cairn change package.
- **APIs**: new finite table/function-type helpers and a `validate_call_indirect` validation helper.
- **Testing**: focused Wasm tests before/after implementation, Verus proof check for `src/lib.rs`, Tracey coverage, ensures scan, cargo/clippy validation, and Cairn gates.