# seL4 Ready Queue Specification

## Purpose

Defines requirements for verified `sel4_ready_queue` semantics in this repo. Scope is scheduler queues behavior and preserved invariants, not raw ABI details.

## Requirements

### Requirement: Ready Queue Consistency
r[sel4-ready-queue.consistency]

A runnable TCB MUST appear in exactly one ready queue at its runnable priority, and a TCB present in a ready queue MUST be runnable.

#### Scenario: Runnable thread enqueued once

- GIVEN a runnable TCB of priority `P`
- WHEN it is inserted into scheduler state
- THEN it appears in exactly one ready queue for `P`

### Requirement: Scheduler Selects Highest Priority Runnable TCB
r[sel4-ready-queue.selection]

Scheduler selection MUST choose a runnable TCB from a highest-priority non-empty ready queue.

#### Scenario: Higher priority wins

- GIVEN runnable TCBs at priorities `1` and `3`
- WHEN the scheduler selects a thread
- THEN the selected TCB has priority `3`

### Requirement: Ready Queue Order Is Preserved
r[sel4-ready-queue.order]

Within one priority queue, enqueue, dequeue, and remove operations MUST preserve the relative order of unaffected TCBs.

#### Scenario: Remove preserves remaining order

- GIVEN queue `[T1, T2, T3]`
- WHEN `T2` is removed
- THEN the remaining queue order is `[T1, T3]`
