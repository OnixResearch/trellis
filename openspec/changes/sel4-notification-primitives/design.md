## Context

Notifications are separate from endpoints in seL4 and have their own sharp rule: pending badges accumulate by OR until delivery. This change isolates signal accumulation, wait/poll consumption, and TCB binding behavior.

## Goals / Non-Goals

**Goals:**
- Accumulate pending badges by OR.
- Deliver exactly the pending badge state.
- Wake the bound waiter when eligible.

**Non-Goals:**
- Endpoint sender/receiver queues
- Reply-cap creation
- Global scheduler proofs

## Decisions

### 1. Pending state is badge-valued

**Choice:** Pending state is badge-valued
**Rationale:** The key semantic object is the accumulated badge word, not a generic queue.

### 2. Keep binding explicit

**Choice:** Keep binding explicit
**Rationale:** Bound-TCB wakeup should be visible in the notification state machine.

### 3. Separate signal and consume steps

**Choice:** Separate signal and consume steps
**Rationale:** This keeps OR-accumulation and delivery proofs small and compositional.

## Risks / Trade-offs

**[Consumption semantics can blur wait vs poll]** Specify both against the same delivered-pending abstraction.
**[Binding could overreach into scheduler theory]** State only local wake eligibility here and leave ready-queue composition to later changes.
