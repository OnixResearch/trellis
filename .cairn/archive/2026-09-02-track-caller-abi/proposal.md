## Why

The rustc-dev-guide backend section on implicit caller location describes a compact, high-value ABI transformation: `#[track_caller]` validity, hidden caller-location argument propagation, trait inheritance behavior, direct-call handling, and reify shims for function pointers.

This change defines finite `track_caller` ABI primitives that Trellis can verify without modeling all panic machinery, const evaluation, or code generation.

## What Changes

- Add a `track-caller-abi` Cairn spec.
- Add a `track_caller_abi` facade at `src/track/caller/abi.rs` with bounded model, admission, chain, trace, proof, and test components.
- Define attribute-validity admission, propagated caller-location metadata, direct-call hidden argument selection, tracked-chain forwarding, function-pointer reify shims, and const/codegen agreement metadata.
- Require positive and negative tests for valid tracked functions, invalid attribute targets, caller-location propagation, untracked boundaries, trait/default inheritance, and missing shims.

## Impact

- **Files**: `src/track/caller/`, `src/lib.rs`, `docs/requirements.md`, README/module catalog text, and focused tests.
- **Testing**: implementation must include accepted and rejected ABI metadata fixtures.
- **Boundaries**: no panic formatting, stack walking, runtime allocation, actual ABI lowering, rustc intrinsic implementation, or platform debug-info claim.
