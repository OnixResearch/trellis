## Why

The rustc-dev-guide backend lowering chapter breaks MIR-to-codegen lowering into blocks, statements, operands, places, rvalues, and terminators using a backend-agnostic builder interface. Trellis already has backend frame and instruction-selection primitives, but not a finite trace model for backend builder operations.

This change defines a backend-codegen trace primitive that checks structural correctness of emitted builder operations without committing to LLVM, Cranelift, GCC, or any concrete machine code.

## What Changes

- Add a `backend-codegen-trace` Cairn spec.
- Plan a `src/backend_codegen_trace.rs` Verus module with finite builder-handle, block, value, place, and operation-trace specs.
- Define MIR block-to-backend block mapping, statement/operand/place/rvalue lowering traces, terminator successor/call/unwind traces, SSA-like local metadata, and handle-validity admission.
- Require positive and negative tests for valid traces, dangling handles, invalid block mapping, call expansion, missing terminator emission, and malformed builder operation ordering.

## Impact

- **Files**: new Cairn change package now; later implementation should touch `src/backend_codegen_trace.rs`, `src/lib.rs`, `docs/requirements.md`, README/module catalog text, and tests.
- **Testing**: implementation must include accepted and rejected backend trace fixtures.
- **Boundaries**: no LLVM IR correctness, no target ABI correctness, no machine-code correctness, no linker/LTO/debug-info claim, and no backend performance claim.
