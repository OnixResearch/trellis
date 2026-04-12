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

### 1. Model explicit waiter queues

**Choice:** Model explicit waiter queues
**Rationale:** Queue order and cancellation become simple sequence properties.

### 2. Keep payload semantics abstract

**Choice:** Keep payload semantics abstract
**Rationale:** The important invariant is rendezvous and badge transfer, not register layout.

### 3. Wakeup is part of the endpoint step

**Choice:** Wakeup is part of the endpoint step
**Rationale:** Endpoint semantics should directly expose which threads become runnable.

## Risks / Trade-offs

**[Cancellation touches both queue and TCB state]** Store explicit waiter entries keyed by TCB ID and prove remove-preserves-order lemmas.
**[Fairness could creep in]** Specify local queue discipline only, not broader scheduling guarantees.
