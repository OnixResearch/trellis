## Context

Scheduler correctness in seL4 begins with one invariant: runnable threads live in exactly one ready queue, and selection comes from the highest-priority non-empty queue. This change isolates those facts from broader TCB and MCS details.

## Goals / Non-Goals

**Goals:**
- Keep runnable TCBs queued exactly once.
- Select from the highest-priority non-empty queue.
- Preserve per-queue order under local updates.

**Non-Goals:**
- MCS scheduling-context budgeting
- Full fairness proofs
- TCB blocked-state definitions

## Decisions

### 1. Represent ready queues as per-priority sequences

**Choice:** Represent ready queues as per-priority sequences
**Rationale:** Membership and order proofs are straightforward over sequences.

### 2. Separate queue correctness from state legality

**Choice:** Separate queue correctness from state legality
**Rationale:** TCB-state rules belong in `sel4_tcb`; this change depends on them.

### 3. Specify highest-priority selection only

**Choice:** Specify highest-priority selection only
**Rationale:** That is the strongest reusable scheduler fact needed by other modules.

## Risks / Trade-offs

**[Priority updates can violate uniqueness]** State queue membership as an explicit invariant and prove remove-before-insert patterns.
**[Ordering policy could overreach]** Specify only local queue order, not system-wide fairness.
