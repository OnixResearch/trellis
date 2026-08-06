## Why

Trellis already has raw IR, CFG, liveness, SSA, DCE, constant-folding, and backend-frame primitives, but those models are intentionally compiler-generic. The rustc-dev-guide MIR chapters point to a reusable Rust-shaped core layer that Trellis does not yet model: finite MIR bodies, locals, places, projections, operands, rvalues, statements, and terminators.

This change creates a narrow MIR core primitive that later drop, dataflow, pass-pipeline, and backend-codegen work can depend on without importing rustc internals or claiming whole-compiler correctness.

## What Changes

- Add a `mir-core` Cairn spec for finite Rust-shaped MIR primitives.
- Plan a `src/mir_core.rs` Verus module with pure specs and checked exec admission helpers.
- Define finite body, basic-block, local, place/projection, operand, rvalue, statement, and terminator envelopes.
- Require successor-range, final-terminator, return-place, non-nested-rvalue, and deterministic failure metadata checks.
- Require positive and negative tests plus Tracey, Verus, README, and Cairn validation evidence before archive.

## Impact

- **Files**: new Cairn change package now; later implementation should touch `src/mir_core.rs`, `src/lib.rs`, `docs/requirements.md`, README/module catalog text, and focused tests.
- **Testing**: implementation must include happy-path and sad-path fixtures for valid bodies, malformed blocks, invalid locals, dangling projections, malformed rvalues, and out-of-range successors.
- **Boundaries**: no borrow checker, no full rustc MIR parity, no type checker, no lifetime reasoning, no codegen, no optimization soundness, and no claim that parsed rustc MIR text is accepted.
