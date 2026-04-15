## Why

Guarded radix lookup is the naming rule for seL4 authority. A dedicated change keeps lookup correctness separate from derivation, IPC, and decode, and gives later modules one precise slot-resolution primitive to depend on.

## What Changes

- Add guarded radix traversal over CNodes with explicit depth accounting.
- Model guard comparison, radix selection, and slot-bound checks.
- Prove successful lookup is unique and failed lookup fabricates no slot.

## Capabilities

### New Capabilities

- `sel4-cspace.lookup`: Verified guarded radix traversal
- `sel4-cspace.unique-slot`: Verified unique in-bounds slot resolution
- `sel4-cspace.failure-no-mutation`: Verified failure leaves lookup state unchanged

## Impact

- **Files**: New `src/sel4_cspace.rs` module, `src/lib.rs` export, `src/properties.rs` cross-module theorems as needed, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for capability-space traversal
- **Dependencies**: Reuses `bounds`, `bitfield`, and tree/sequence helpers
- **Testing**: Verus proofs, unit tests, and proptests for guard mismatch, depth mismatch, and successful traversal

## Out of Scope

- Capability derivation-tree semantics
- Invocation decode policy
- Raw machine-word CNode encodings
