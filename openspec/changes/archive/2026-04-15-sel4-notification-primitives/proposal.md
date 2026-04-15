## Why

Notifications are separate from endpoints in seL4 and have their own sharp rule: pending badges accumulate by OR until delivery. This change isolates signal accumulation, wait/poll consumption, and TCB binding behavior.

## What Changes

- Add notification pending-badge state and signal/wait/poll operations.
- Model bound TCB wake behavior.
- Prove badge accumulation and exact delivery semantics.

## Capabilities

### New Capabilities

- `sel4-notification.pending-badge`: Verified OR-accumulated pending badges
- `sel4-notification.consume`: Verified wait/poll consume delivered pending state
- `sel4-notification.binding`: Verified bound-TCB wake rules

## Impact

- **Files**: New `src/sel4_notification.rs` module, `src/lib.rs` export, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for notification signaling
- **Dependencies**: Standalone for now; later changes may compose it with `sel4_tcb` / ready-queue state
- **Testing**: Verus proofs, unit tests, and proptests for accumulation and consumption scenarios

## Out of Scope

- Endpoint rendezvous queues
- Reply-cap semantics
- Hardware IRQ routing details
