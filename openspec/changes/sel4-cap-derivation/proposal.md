## Why

seL4 revocation and authority ancestry depend on a capability derivation tree. Splitting this out gives one place for copy/mint/move/revoke structure proofs without mixing in CSpace traversal or local rights algebra.

## What Changes

- Add a capability derivation tree with parent-child ancestry.
- Model copy, mint, move, revoke, and recycle transitions.
- Prove revocation removes exactly the target subtree.

## Capabilities

### New Capabilities

- `sel4-cdt.structure`: Verified acyclic derivation ancestry
- `sel4-cdt.move`: Verified move preserves derivation ancestry
- `sel4-cdt.revoke`: Verified exact subtree revocation

## Impact

- **Files**: New `src/sel4_cdt.rs` module, `src/lib.rs` export, `src/properties.rs` cross-module theorems as needed, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for capability ancestry
- **Dependencies**: Reuses `forest`, `reachability`, and `sel4_cap_rights` once implemented
- **Testing**: Verus proofs, unit tests, and proptests for copy/move/revoke topologies

## Out of Scope

- CNode path lookup
- Endpoint IPC
- Global kernel invariants beyond the CDT facts themselves
