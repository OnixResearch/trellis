## Why

seL4 authority lives on capabilities, so a reusable model needs a sharp rights-mask core. This change isolates attenuation, masking, and operation-gating rules so later CSpace, IPC, mapping, and decode modules can depend on one verified authority algebra.

## What Changes

- Add rights-mask predicates for subset, attenuation, and effective authority checks.
- Define operation gates for send, receive, grant-like transfer, map, and control actions.
- Prove masking laws that keep derivation monotone and non-amplifying.

## Capabilities

### New Capabilities

- `sel4-cap-rights.attenuation`: Verified no-privilege-gain derivation
- `sel4-cap-rights.operation-gates`: Verified authority checks for kernel operations
- `sel4-cap-rights.masking`: Verified idempotent and monotone rights masking

## Impact

- **Files**: New `src/sel4_cap_rights.rs` module, `src/lib.rs` export, `src/properties.rs` cross-module theorems as needed, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for authority
- **Dependencies**: Reuses `access_mode`, `authorize`, and `containment`
- **Testing**: Verus proofs, unit tests, and proptests for subset, masking, and gate checks

## Out of Scope

- Raw cap-slot bit encoding
- Capability derivation-tree structure
- Invocation decode beyond generic operation gating
