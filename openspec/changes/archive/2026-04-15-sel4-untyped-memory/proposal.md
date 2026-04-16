## Why

Untyped memory is the source from which seL4 creates typed kernel objects. A standalone change gives the repo a reusable proof layer for retype, alignment, containment, non-overlap, and reclaim without mixing in scheduler or mapping details.

## What Changes

- Add untyped regions and typed child-object descriptors.
- Model retype, revoke, and reset operations over child regions.
- Prove conservation, alignment, containment, and non-overlap.

## Capabilities

### New Capabilities

- `sel4-untyped.retype`: Verified aligned typed object creation from untyped memory
- `sel4-untyped.child-layout`: Verified contained and disjoint child layout
- `sel4-untyped.reclaim`: Verified revoke/reset reclaim of child coverage

## Impact

- **Files**: New `src/sel4_untyped.rs` module, `src/lib.rs` export, `src/properties.rs` cross-module theorems as needed, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for kernel object memory
- **Dependencies**: Reuses `region`, `page`, `buddy`, `slab`, and `watermark`
- **Testing**: Verus proofs, unit tests, and proptests for layout, overlap rejection, and reclaim scenarios

## Out of Scope

- ASID allocation
- Ready-queue membership
- Architecture-specific page-table encodings
