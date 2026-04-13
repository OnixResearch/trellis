## Why

WebAssembly is small, typed, and ubiquitous. Browsers, plugin systems,
server sandboxes, and embedded runtimes all rely on same core rules:
typed operand stacks, structured control flow, linear memory with page
growth, tables for indirect calls, and validation that rules out stack
underflow and mismatched call signatures before execution.

This repo already verifies many low-level primitives around memory,
control flow, tables, queues, and type discipline, but it does not yet
have a WebAssembly-specific semantic layer. Missing pieces are the
typed value model, operand and control stacks, local variable access,
linear memory bounds and growth, table lookup, indirect-call signature
checks, and the validation rules that connect static typing to runtime
safety.

A verified WebAssembly primitive layer would give downstream users a
compact reference for sandboxed bytecode execution, validator design,
and compiler/runtime reasoning without trying to verify an entire Wasm
engine or binary decoder.

## What Changes

- **`wasm_value`** — Typed WebAssembly values and value-type predicates.
  Proves values carry exact type tags and cannot be confused across
  stack, locals, memory, and table signatures.
- **`wasm_stack`** — Operand stack operations and stack-typing rules.
  Proves instructions consume the required operand prefix, produce the
  declared result types, and support WebAssembly's unreachable-stack
  polymorphism.
- **`wasm_locals`** — Local frame operations for `local.get`,
  `local.set`, and `local.tee`. Proves index bounds, exact type
  preservation, and non-target locals remain unchanged.
- **`wasm_memory`** — Linear memory loads, stores, and `memory.grow`.
  Proves bounds checks, exact affected-byte range, 64KiB page growth,
  monotone size changes, and prefix preservation across growth.
- **`wasm_table`** — Table lookup and update for function references.
  Proves bounds checks and exact-slot update semantics.
- **`wasm_call_indirect`** — Indirect call dispatch through tables.
  Proves successful calls require a present table entry with the exact
  expected function type and that mismatches trap.
- **`wasm_control`** — Structured control stack, labels, branch-depth
  resolution, and function return arity. Proves branches target valid
  labels and restore the target stack shape.
- **`wasm_validation`** — Validation rules for the modeled instruction
  subset. Proves validated programs avoid operand-stack underflow and
  reject mismatched stack effects before execution.
- **Cross-module properties** — Proofs linking validation to runtime
  safety for memory operations, indirect calls, and structured branch
  behavior.

## Capabilities

### New Capabilities

- `wasm-value.types`: Verified exact type tags for Wasm values
- `wasm-stack.discipline`: Verified operand-stack consume/produce rules
- `wasm-stack.polymorphic-unreachable`: Verified unreachable-stack polymorphism
- `wasm-locals.access`: Verified typed local get/set/tee behavior
- `wasm-memory.bounds`: Verified bounds-checked linear-memory access
- `wasm-memory.grow`: Verified monotone 64KiB page growth with prefix preservation
- `wasm-table.bounds`: Verified bounds-checked table access
- `wasm-call-indirect.signature`: Verified exact signature matching for indirect calls
- `wasm-control.labels`: Verified branch depth resolves to valid labels with correct arity
- `wasm-control.return`: Verified return consumes and produces declared function results
- `wasm-validation.stack-safety`: Verified validated instruction sequences cannot underflow the operand stack
- `properties.wasm-validated-memory`: Verified validated memory ops preserve typing or trap
- `properties.wasm-indirect-call-safe`: Verified validated indirect calls never dispatch to mismatched signatures
- `properties.wasm-branch-stack`: Verified branches restore the target stack shape

## Impact

- **Files**: New `src/wasm_*.rs` modules, `src/lib.rs` exports,
  `src/properties.rs` additions, `docs/requirements.md` updates, and
  this OpenSpec change
- **APIs**: Public spec/exec/proof functions for values, stack effects,
  locals, memory, tables, indirect-call checks, control labels, and
  validation judgments
- **Dependencies**: Reuses existing value-level modeling style and
  sequence-based proofs; no new runtime dependencies expected
- **Testing**: Verus proofs, unit tests for concrete Wasm scenarios, and
  proptest model oracles for memory bounds, stack typing, and indirect-call dispatch

## Out of Scope

- Wasm binary decoding, section parsing, and module instantiation
- Imports, exports, start functions, and host embedding APIs
- Exact floating-point arithmetic and NaN canonicalization
- Reference types beyond function-table slots
- Threads, atomics, exceptions, SIMD, GC, and component-model features
- Full interpreter loops, JIT compilation, or host scheduler integration
- Fuel metering, gas accounting, or optimization passes
