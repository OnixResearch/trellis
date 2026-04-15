## Why

Reply capabilities are one of seL4's sharpest linear resources. A separate change lets the repo verify one-shot call/reply discipline and revoke/cancel invalidation without bundling all IPC behavior together.

## What Changes

- Add reply-cap lifecycle states and call/reply transitions.
- Model caller/callee pairing carried by a live reply cap.
- Prove reply caps are one-shot and invalidated by revoke/cancel.

## Capabilities

### New Capabilities

- `sel4-reply.one-shot`: Verified single-use reply-cap discipline
- `sel4-reply.pairing`: Verified caller/callee pairing preservation
- `sel4-reply.invalidate`: Verified revoke/cancel invalidates stale reply caps

## Impact

- **Files**: New `src/sel4_reply.rs` module, `src/lib.rs` export, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for linear IPC authority
- **Dependencies**: Standalone for now; later changes may compose it with `sel4_endpoint` / `sel4_tcb`
- **Testing**: Verus proofs, unit tests, and proptests for reply, double-reply, and cancellation paths

## Out of Scope

- General endpoint queue semantics
- Notification badge accumulation
- Invocation decode
