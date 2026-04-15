## Why

Many seL4 primitives depend on TCB legality: runnable threads must differ from blocked threads, and blocked states must name the object causing the block. A dedicated change gives later IPC, notification, reply, and scheduler modules a precise thread-state base.

## What Changes

- Add legal TCB states and explicit blocking reasons.
- Model state transitions for block, wake, restart, and deactivate.
- Prove blocked states always identify their wait object.

## Capabilities

### New Capabilities

- `sel4-tcb.state`: Verified legal TCB-state classification
- `sel4-tcb.blocking-object`: Verified blocked states name their wait object
- `sel4-tcb.transitions`: Verified legal TCB state transitions

## Impact

- **Files**: New `src/sel4_tcb.rs` module, `src/lib.rs` export, `src/properties.rs` cross-module theorems as needed, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for thread state machine
- **Dependencies**: Reuses `process_state` and later IPC modules
- **Testing**: Verus proofs, unit tests, and proptests for block/wake/restart transitions

## Out of Scope

- Ready-queue membership
- Endpoint waiter ordering
- ASID or VSpace state
