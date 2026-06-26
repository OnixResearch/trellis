## Why

ASID uniqueness is one of the smallest but most reusable global kernel facts. A dedicated change gives VSpace and invariant proofs a clean allocation/release model without requiring full architecture-specific MMU logic.

## What Changes

- Model one abstract finite ASID pool instance as slots `0..capacity-1`, where each live slot owns exactly one opaque address-space object ID. One owner may hold multiple live ASIDs; this change constrains ASID -> owner ownership, not owner -> ASID exclusivity.
- Add explicit allocation and release state transitions for that single pool.
- Define valid release as an exact `(asid, owner)` match that clears ownership; invalid or duplicate release reports failure and leaves the pool unchanged.
- Prove uniqueness, safe reuse only after ownership has been cleared, and finite-pool exhaustion behavior.

## Capabilities

### New Capabilities

- `sel4-asid.uniqueness`: Verified unique live ASID ownership
- `sel4-asid.release-state`: Verified release removes live ownership and invalid release leaves state unchanged
- `sel4-asid.reuse`: Verified safe reuse only after release
- `sel4-asid.pool-bounds`: Verified allocation respects pool capacity

## Impact

- **Files**: New `src/sel4_asid.rs` module, `src/lib.rs` export, `docs/requirements.md`, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for single-pool allocation/release plus reusable liveness/ownership predicates over opaque address-space object IDs
- **Dependencies**: Standalone finite-pool model; no architecture-specific MMU helpers required
- **Testing**: Verus proofs, unit tests, proptests, and repo traceability updates for allocate/release/exhaustion scenarios; completion also requires mirrored `r[sel4-asid.*]` entries in `docs/requirements.md`

## Out of Scope

- Multiple-pool composition or architecture-specific ASID table layouts
- Page-table walk semantics
- Mapping overlap proofs
- IRQ or scheduler state
