## Why

Fault delivery in seL4 turns structured fault information into IPC-visible state. A dedicated change gives the repo a reusable proof layer for preserving fault class and payload through encode/decode and handler delivery.

## What Changes

- Add structured fault records for the modeled fault classes.
- Model fault-message encode/decode and handler delivery.
- Prove metadata roundtrips without corruption.

## Capabilities

### New Capabilities

- `sel4-fault-ipc.preservation`: Verified fault metadata roundtrip through IPC
- `sel4-fault-ipc.delivery`: Verified delivery reaches the configured handler context
- `sel4-fault-ipc.payload`: Verified payload fields are not corrupted by encoding

## Impact

- **Files**: New `src/sel4_fault_ipc.rs` module, `src/lib.rs` export, `src/properties.rs` cross-module theorems as needed, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for fault handling
- **Dependencies**: Standalone for now; later changes may compose fault delivery with `sel4_endpoint`
- **Testing**: Verus proofs, unit tests, and proptests for page fault and generic fault roundtrips

## Out of Scope

- Trap-entry hardware details
- Invocation decode authorization
- Scheduler policy after fault delivery
