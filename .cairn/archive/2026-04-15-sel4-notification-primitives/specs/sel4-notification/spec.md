# seL4 Notification Specification

## Purpose

Defines requirements for verified `sel4_notification` semantics in this repo. Scope is notification signaling behavior and preserved invariants, not raw ABI details.

## Requirements

### Requirement: Notification Pending Badge Accumulation

r[sel4-notification.pending-badge]
A notification object MUST accumulate undelivered badge bits by bitwise OR of all delivered signals that have not yet been consumed.

#### Scenario: Two signals accumulate

- GIVEN an idle notification
- WHEN badges `0x2` and `0x8` are signaled before any wait consumes them
- THEN the pending badge is `0xA`

### Requirement: Wait And Poll Consume Delivered State

r[sel4-notification.consume]
Waiting or polling a signaled notification MUST return exactly the pending badge value being delivered and consume that delivered pending state according to notification semantics.

#### Scenario: Wait consumes pending badge

- GIVEN a notification with pending badge `0xA`
- WHEN one waiter successfully consumes the notification
- THEN the delivered badge is `0xA`

### Requirement: Notification Binding

r[sel4-notification.binding]
If a notification is bound to a TCB, delivery eligibility and wakeup MUST respect that binding.

#### Scenario: Bound waiter wakes

- GIVEN notification `N` bound to TCB `T` and `T` waiting on `N`
- WHEN `N` is signaled
- THEN `T` becomes runnable
