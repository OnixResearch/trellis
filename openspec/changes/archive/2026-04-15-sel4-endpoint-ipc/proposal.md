## Why

Endpoint IPC is seL4's core rendezvous primitive. A dedicated change lets the repo verify sender/receiver matching, queue discipline, and badge transfer without dragging in notifications or reply-cap linearity.

## What Changes

- Add endpoint sender/receiver queues and rendezvous matching.
- Model blocked waiters, wakeup, and sender/receiver cancellation.
- Prove exact pair matching, queue preservation for unmatched waiters, and badge transfer semantics for matched pairs.

## Capabilities

### New Capabilities

- `sel4-endpoint.rendezvous`: Verified sender/receiver rendezvous
- `sel4-endpoint.queue-discipline`: Verified queue preservation under match and cancel
- `sel4-endpoint.badge`: Verified badge transfer on successful IPC

## Impact

- **Files**: New `src/sel4_endpoint.rs` module, `src/lib.rs` export, `docs/requirements.md`, synced `openspec/specs/sel4-endpoint/spec.md`, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for IPC rendezvous plus sender/receiver cancellation
- **Dependencies**: Reuses sequence-style queue helpers; this change treats thread IDs as opaque identifiers and does not depend on a `sel4_tcb` module
- **Testing**: Verus proofs, unit tests, and proptests for send/receive/cancel interleavings

## Verification Expectations

- Prove rendezvous dequeues exactly one waiter from each side and uses FIFO head selection when both sides are queued.
- Prove no-match sends/receives enqueue exactly one waiter and preserve existing queue order.
- Prove sender/receiver cancellation removes only the targeted waiter and leaves non-queued cancellation as a no-op.
- Prove successful rendezvous delivers the caller-provided send badge to the matched receiver.
- Keep all public verified functions on ordinary `ensures` contracts with no new `#[verifier::external_body]` sites.

## Out of Scope

- Notification OR-accumulation
- Reply-cap lifecycle
- User-level IPC buffer ABI details
- Full payload/register transfer beyond matched badge delivery
- Capability lookup for badge origin; this change models the badge as a caller-provided send value
