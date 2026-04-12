## Why

Endpoint IPC is seL4's core rendezvous primitive. A dedicated change lets the repo verify sender/receiver matching, queue discipline, and badge transfer without dragging in notifications or reply-cap linearity.

## What Changes

- Add endpoint sender/receiver queues and rendezvous matching.
- Model blocked waiters, wakeup, and cancellation.
- Prove message and badge transfer semantics for matched pairs.

## Capabilities

### New Capabilities

- `sel4-endpoint.rendezvous`: Verified sender/receiver rendezvous
- `sel4-endpoint.queue-discipline`: Verified queue preservation under match and cancel
- `sel4-endpoint.badge`: Verified badge transfer on successful IPC

## Impact

- **Files**: New `src/sel4_endpoint.rs` module, `src/lib.rs` export, `src/properties.rs` cross-module theorems as needed, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for IPC rendezvous
- **Dependencies**: Reuses `process_state`, queue helpers, and later `sel4_tcb` integration
- **Testing**: Verus proofs, unit tests, and model-oracle proptests for send/receive/cancel interleavings

## Out of Scope

- Notification OR-accumulation
- Reply-cap lifecycle
- User-level IPC buffer ABI details
