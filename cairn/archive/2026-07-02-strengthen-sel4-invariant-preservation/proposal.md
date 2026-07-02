## Why

`kernel_invariant_spec` already composes the first seL4 global invariant layer and proves an initial preservation set. Future kernel-step work needs a stricter preservation map so each new step states the sub-invariants it consumes, the mutable fields it may change, and the unchanged facts it must carry forward.

## What Changes

- Add a preservation matrix for modeled seL4 kernel steps.
- Expand step-preservation evidence around endpoint, TCB, ready-queue, and untyped interactions without widening the trusted surface.
- Add positive and negative tests/proofs for accepted steps and rejected/no-op boundaries.

## Scope

In scope: `sel4_invariants` proof helpers, step wrappers already modeled by local seL4 modules, preservation-matrix documentation, and requirement/Tracey sync.

Out of scope:
- Modeling the full seL4 ABI.
- Adding new trusted assumptions for hard preservation cases.
- Changing existing primitive semantics solely to make proofs easier.

## Verification

Run focused `cargo test` for seL4 modules, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested checks, and Cairn validate/gates before archive.
