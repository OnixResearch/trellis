## Why

Scheduler correctness in seL4 begins with one invariant: runnable threads live in exactly one ready queue, and selection comes from the highest-priority non-empty queue. This change isolates those facts from broader TCB and MCS details.

## What Changes

- Add per-priority ready queues and selection helpers.
- Model enqueue, dequeue, and remove operations.
- Prove runnable/queued consistency and highest-priority selection.

## Capabilities

### New Capabilities

- `sel4-ready-queue.consistency`: Verified runnable iff queued consistency
- `sel4-ready-queue.selection`: Verified highest-priority runnable selection
- `sel4-ready-queue.order`: Verified queue-order preservation within one priority

## Impact

- **Files**: New `src/sel4_ready_queue.rs` module, `src/lib.rs` export, `src/properties.rs` cross-module theorems as needed, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for scheduler queues
- **Dependencies**: Reuses `sel4_tcb` and sequence helpers
- **Testing**: Verus proofs, unit tests, and proptests for enqueue/dequeue/remove/select cases

## Out of Scope

- Budgeting and refill semantics
- IRQ delivery
- ASID or VSpace state
