## Why

The Wasm primitive model now has value typing, operand-stack effects, typed locals, memory, and branch/return arity against control labels. The next validation gap is unreachable-stack polymorphism: after `unreachable`, `br`, or `return`, later stack operations in the same path should be admitted without requiring preexisting operand suffixes, while reachable paths must still enforce exact stack types.

This change lands a compact validation-state slice that keeps whole instruction-sequence validation out of scope while making unreachable-stack behavior executable and testable.

## What Changes

- Add `WasmValidationState` with operand-type stack, control stack, and reachability flag.
- Add type-stack suffix predicates and executable helpers for reachable versus unreachable admission.
- Add validation helpers for `i32.add`, `br`, `return`, and `unreachable` over the existing control-label model.
- Add positive and negative tests for reachable mismatch rejection, unreachable polymorphism, branch/return transitions to unreachable state, and malformed control-stack rejection.
- Sync requirements, accepted specs, README, crate docs, and archive evidence.

## Impact

- **Files**: `src/wasm_primitives.rs`, `docs/requirements.md`, `cairn/specs/wasm/spec.md`, legacy `openspec/specs/wasm/spec.md`, `README.md`, `src/lib.rs`, and this Cairn change package.
- **APIs**: new validation state and public validation helpers for a tiny instruction subset.
- **Testing**: focused Wasm tests, full cargo tests, clippy, Verus proof check, Tracey coverage, and ensures scan.
