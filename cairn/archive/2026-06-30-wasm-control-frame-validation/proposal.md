## Why

The Wasm validator model now tracks operand types, control labels, and unreachable-stack polymorphism. The next structured-control gap is control-frame stack-height validation: block and loop entry must consume declared parameters, block end must validate result types, and ending a control frame must restore the operand stack to the frame base plus results.

This change lands a compact control-frame slice for block/loop entry and block end without attempting full instruction-sequence validation.

## What Changes

- Add a `WasmControlFrame` model with label kind, parameter types, result types, entry height, and entry reachability.
- Add validation helpers for entering blocks, entering loops, and ending the innermost frame.
- Preserve the existing control-label arity model while adding stack-height restoration at `end`.
- Add positive and negative tests for entry parameter mismatch, block result mismatch, loop parameter handling, and unreachable block end synthesis/restoration.
- Sync requirements, accepted specs, README, crate docs, and archive evidence.

## Impact

- **Files**: `src/wasm_primitives.rs`, `docs/requirements.md`, `cairn/specs/wasm/spec.md`, legacy `openspec/specs/wasm/spec.md`, `README.md`, `src/lib.rs`, and this Cairn change package.
- **APIs**: new control-frame helpers for a small structured-validation subset.
- **Testing**: focused Wasm tests, full cargo tests, clippy, Verus proof check, Tracey coverage, and ensures scan.
