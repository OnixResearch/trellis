# seL4 Endpoint IPC Specification

## Purpose

Defines requirements for verified `sel4_endpoint` semantics in this repo. Scope is IPC rendezvous behavior and preserved invariants, not raw ABI details.

## Requirements

### Requirement: Endpoint Rendezvous IPC

r[sel4-endpoint.rendezvous]
When a sender and receiver are both available on the same endpoint, the kernel MUST match exactly one sender with exactly one receiver and wake both threads.

#### Scenario: Waiting receiver matches send

- GIVEN a receiver blocked on endpoint `E`
- WHEN a sender sends on `E`
- THEN exactly one sender and that receiver are matched and unblocked

### Requirement: Endpoint Queue Discipline

r[sel4-endpoint.queue-discipline]
Unmatched waiters on an endpoint MUST preserve queue order under match and cancellation. Cancelling a blocked thread MUST remove only that thread from the queue.

#### Scenario: Cancel middle waiter

- GIVEN queued waiters `[T1, T2, T3]`
- WHEN `T2` is cancelled
- THEN the remaining queue order is `[T1, T3]`

### Requirement: Endpoint Badge Transfer

r[sel4-endpoint.badge]
A successful endpoint send/receive rendezvous MUST deliver the sender capability's badge to the matched receiver.

#### Scenario: Badge delivered on match

- GIVEN a sender using badge `B` on endpoint `E`
- WHEN send and receive rendezvous
- THEN the receiver observes badge `B`
