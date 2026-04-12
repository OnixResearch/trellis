## Why

ASID uniqueness is one of the smallest but most reusable global kernel facts. A dedicated change gives VSpace and invariant proofs a clean allocation/release model without requiring full architecture-specific MMU logic.

## What Changes

- Add ASID pool allocation and release semantics.
- Model live ownership of numeric ASIDs by address spaces.
- Prove uniqueness, safe reuse, and pool-bound behavior.

## Capabilities

### New Capabilities

- `sel4-asid.uniqueness`: Verified unique live ASID ownership
- `sel4-asid.reuse`: Verified safe reuse only after release
- `sel4-asid.pool-bounds`: Verified allocation respects pool capacity

## Impact

- **Files**: New `src/sel4_asid.rs` module, `src/lib.rs` export, `src/properties.rs` cross-module theorems as needed, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for address-space identity
- **Dependencies**: Reuses `bitmap`/allocation-style helpers as needed
- **Testing**: Verus proofs, unit tests, and proptests for allocate/release/exhaustion scenarios

## Out of Scope

- Page-table walk semantics
- Mapping overlap proofs
- IRQ or scheduler state
