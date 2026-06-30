## Why

The landed Wasm MVP slice explicitly deferred control labels, branch arity, and return arity. That leaves the operand-stack model useful for `i32.add`, locals, and memory, but unable to state the core validation rule that branches and returns must supply exactly the value types required by their target label or function result.

This change lands the next small self-contained slice: typed control labels plus executable branch-depth, branch-arity, and return-arity admission helpers.

## What Changes

- Extend `src/wasm_primitives.rs` with control-label kinds for function, block, and loop frames.
- Add spec and exec helpers for branch-depth validity, branch target result/parameter arity, and function return arity.
- Add positive and negative unit tests for block branches, loop branches, out-of-range branch depths, return arity, empty control stacks, and non-function roots.
- Sync `docs/requirements.md`, accepted Wasm specs, README, and module docs.

## Impact

- **Files**: `src/wasm_primitives.rs`, `docs/requirements.md`, `cairn/specs/wasm/spec.md`, legacy `openspec/specs/wasm/spec.md`, `README.md`, `src/lib.rs`, and this change package.
- **APIs**: new Wasm control-label structs/enums plus validation helpers for branch and return arity.
- **Testing**: focused Wasm cargo tests, clippy, Tracey coverage checks, and available repo verification gates.
