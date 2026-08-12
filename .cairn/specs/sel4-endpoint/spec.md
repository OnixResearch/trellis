# seL4 Endpoint IPC Specification

## Purpose

Defines requirements for verified `sel4_endpoint` semantics in this repo. Scope is IPC rendezvous behavior and preserved invariants, not raw ABI details.

## Requirements

### Requirement: Endpoint Rendezvous IPC
r[sel4-endpoint.rendezvous]

When a sender and receiver are both available on the same endpoint, the kernel MUST match exactly one sender with exactly one receiver, dequeue the FIFO head waiter from each populated side, and wake both threads. Public send/receive helpers require caller TCB IDs that are not already queued on that endpoint.

#### Scenario: Waiting receiver matches send

- GIVEN a receiver blocked on endpoint `E`
- WHEN a sender sends on `E`
- THEN exactly one sender and that receiver are matched and unblocked

#### Scenario: Waiting sender matches receive

- GIVEN a sender blocked on endpoint `E`
- WHEN a receiver receives on `E`
- THEN exactly one receiver and that sender are matched and unblocked

#### Scenario: FIFO sender head matches first

- GIVEN queued senders `[S1, S2]` on endpoint `E`
- WHEN a receiver receives on `E`
- THEN `S1` is matched and `S2` remains queued

#### Scenario: FIFO receiver head matches first

- GIVEN queued receivers `[R1, R2]` on endpoint `E`
- WHEN a sender sends on `E`
- THEN `R1` is matched and `R2` remains queued

#### Scenario: No waiting receiver queues sender

- GIVEN no receiver blocked on endpoint `E`
- WHEN a sender sends on `E`
- THEN that sender is queued and remains blocked on `E`

#### Scenario: No waiting sender queues receiver

- GIVEN no sender blocked on endpoint `E`
- WHEN a receiver receives on `E`
- THEN that receiver is queued and remains blocked on `E`

### Requirement: Endpoint Queue Discipline
r[sel4-endpoint.queue-discipline]

Unmatched waiters on an endpoint MUST preserve queue order under match and cancellation. Cancelling a blocked sender or blocked receiver MUST remove only that waiter from its queue, and cancelling a non-queued waiter MUST leave endpoint queues unchanged.

#### Scenario: Cancel middle sender

- GIVEN queued senders `[S1, S2, S3]`
- WHEN `S2` is cancelled
- THEN the remaining sender queue order is `[S1, S3]`

#### Scenario: Cancel middle receiver

- GIVEN queued receivers `[R1, R2, R3]`
- WHEN `R2` is cancelled
- THEN the remaining receiver queue order is `[R1, R3]`

#### Scenario: Cancel absent waiter is no-op

- GIVEN endpoint queues with no waiter `T`
- WHEN `T` is cancelled
- THEN both endpoint queues are unchanged

### Requirement: Endpoint Badge Transfer
r[sel4-endpoint.badge]

A successful endpoint send/receive rendezvous MUST deliver the matched sender's caller-provided badge value to the matched receiver.

#### Scenario: Badge delivered on send-to-receive match

- GIVEN a sender using badge `B` on endpoint `E`
- WHEN send and receive rendezvous
- THEN the matched receiver observes badge `B`
