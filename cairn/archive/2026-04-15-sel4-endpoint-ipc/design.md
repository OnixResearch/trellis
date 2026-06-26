## Context

Endpoint IPC is seL4's core rendezvous primitive. A dedicated change lets the repo verify sender/receiver matching, queue discipline, and badge transfer without dragging in notifications or reply-cap linearity.

## Goals / Non-Goals

**Goals:**
- Match exactly one sender and one receiver.
- Preserve waiter queue order for unmatched threads.
- Transfer badges on successful rendezvous.

**Non-Goals:**
- Reply-cap creation
- Notification objects
- Full syscall ABI packing

## Decisions

### 1. Model explicit sender and receiver queues

**Choice:** Store explicit sender waiters `(tcb, badge)` and explicit receiver waiter TCB IDs in `Endpoint { senders, receivers }`, with `new_endpoint` as the constructor that establishes empty queues and the normalized endpoint invariant.
**Rationale:** Queue order, exact head matching, and badge transfer become simple sequence properties.

### 2. Keep payload semantics abstract

**Choice:** Keep payload semantics abstract and model badge transfer as a caller-provided send value.
**Rationale:** The important invariant is rendezvous and badge transfer, not register layout or capability lookup.

### 3. Wakeup is part of the endpoint step

**Choice:** Return the matched sender/receiver pair directly from the endpoint step.
**Rationale:** Endpoint semantics should directly expose which threads become runnable.

### 4. Public send/receive use explicit validity preconditions

**Choice:** `send` and `receive` require the caller TCB ID not already be queued as a sender or receiver on the endpoint, so one TCB never appears in both queues at once. `new_endpoint` constructs the normalized empty state, cancellation accepts any TCB ID, and non-queued cancellation is a no-op.
**Rationale:** This keeps the queue model single-membership and avoids invalid duplicate blocked waiters while still giving total cancellation helpers.

### 5. Proofs follow sequence head-pop and remove-helper structure

**Choice:** Use head-pop postconditions for non-empty match steps and sequence remove/filter helpers for sender/receiver cancellation.
**Rationale:** The rendezvous and queue-discipline requirements are fundamentally queue-shape statements, so the proofs should mirror those sequence updates directly.

## Risks / Trade-offs

**[Cancellation touches both queue and TCB state]** Store explicit waiter entries keyed by TCB ID and prove remove-preserves-order facts over the remaining queue.
**[Fairness could creep in]** Specify local queue discipline only, not broader scheduling guarantees.
