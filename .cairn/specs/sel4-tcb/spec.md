# seL4 TCB State Specification

## Purpose

Defines requirements for verified `sel4_tcb` semantics in this repo. Scope is thread state machine behavior and preserved invariants, not raw ABI details.

## Requirements

### Requirement: TCB State Legality
r[sel4-tcb.state]

A TCB MUST be in exactly one legal state: runnable, running, blocked on send, blocked on receive, blocked on notification, blocked on reply, restart, or inactive.

#### Scenario: Blocked-on-send carries endpoint

- GIVEN TCB `T` blocked on endpoint send
- WHEN `T` is inspected
- THEN its state identifies the endpoint causing the block

### Requirement: Blocked States Name Their Wait Object
r[sel4-tcb.blocking-object]

Every blocked TCB state MUST identify the kernel object responsible for the block.

#### Scenario: Notification block names notification

- GIVEN TCB `T` blocked on notification `N`
- WHEN `T` is inspected
- THEN the state identifies `N`

### Requirement: TCB State Transitions Are Legal
r[sel4-tcb.transitions]

Block, wake, restart, and deactivate operations MUST move a TCB only along allowed state transitions.

#### Scenario: Wake moves blocked thread to runnable

- GIVEN a TCB blocked on receive
- WHEN the wake condition occurs
- THEN the TCB becomes runnable
