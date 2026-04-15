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

**Choice:** Represent ready queues as `Vec<Vec<usize>>` indexed by priority, with `queues.len() == NUM_PRIORITIES` and each inner queue storing TCB ids in scheduler order.
**Rationale:** Membership and order proofs are straightforward over sequences, and a concrete vector-of-vectors model keeps exec scans iterative.

### 2. Separate queue correctness from state legality

**Choice:** Separate queue correctness from state legality
**Rationale:** TCB-state rules belong in `sel4_tcb`; this change depends on them.

### 3. Specify highest-priority selection only

**Choice:** Specify highest-priority selection only
**Rationale:** That is the strongest reusable scheduler fact needed by other modules.

### 4. Model priority changes as remove + enqueue

**Choice:** Keep priority change out of scope for this phase; callers must remove from the old queue and enqueue into the new one.
**Rationale:** That keeps the API surface small and makes uniqueness/order proofs local to enqueue/dequeue/remove/select.

## Constructor vs requires policy

Public helpers stay total and use constructor-normalized queue state:
- `new_ready_queues(num_priorities)` builds an empty ready-queue set.
- `enqueue`, `dequeue`, `remove`, and `select_highest` have no `requires` clauses.
- Invalid enqueue/remove/dequeue cases return `false`/`None` or leave state unchanged instead of failing preconditions.

## Verification approach

- `sel4-ready-queue.consistency`: define a `consistent_ready_queues_spec(queues, tcbs)` predicate that captures bidirectional runnable iff queued-at-priority plus uniqueness; prove enqueue/dequeue/remove preserve it.
- `sel4-ready-queue.selection`: define a `select_result_spec(queues)` / highest-priority predicate and prove `select_highest` returns a TCB from a highest-priority non-empty queue with no higher-priority runnable queue skipped.
- `sel4-ready-queue.order`: specify unaffected-order preservation as a filter/subsequence relation on each priority queue and prove it for enqueue/dequeue/remove.
- Keep exec helpers iterative for queue scans and removals; avoid recursive exec helpers even if proofs would be shorter.

## Risks / Trade-offs

**[Priority updates can violate uniqueness]** State queue membership as an explicit invariant and prove remove-before-insert patterns.
**[Ordering policy could overreach]** Specify only local queue order, not system-wide fairness.
