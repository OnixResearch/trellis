# Zircon Kernel Primitives Specification

## Purpose

Defines requirements for a verified model of core Zircon kernel primitives:
rights, handles, signals and waiting, events/eventpairs, channels, and
futex wait/wake/requeue behavior. Scope is semantic behavior and preserved
invariants, not raw syscall ABI details.

## Requirements

### Requirement: Rights Attenuation

r[zircon-rights.attenuation]
A derived handle MUST NOT gain rights that the source handle did not hold.
`duplicate` and `replace` MAY preserve rights or drop a subset of rights,
but MUST NOT add new rights.

#### Scenario: Duplicate with same rights

- GIVEN a handle with rights set `R`
- WHEN the handle is duplicated with requested rights `R`
- THEN the new handle has rights `R`

#### Scenario: Duplicate with reduced rights

- GIVEN a handle with rights set `R`
- WHEN the handle is duplicated with requested rights `R'` where `R' ⊆ R`
- THEN the new handle has rights `R'`

#### Scenario: Duplicate with expanded rights is rejected

- GIVEN a handle with rights set `R`
- WHEN duplication requests rights `R'` where `R'` contains a bit not in `R`
- THEN the duplication is invalid

### Requirement: Rights Gate Operations

r[zircon-rights.operation-gates]
Handle operations MUST require the corresponding right on the acting handle.
At minimum: duplicate requires `DUPLICATE`, transfer requires `TRANSFER`,
wait requires `WAIT`, object-signal requires `SIGNAL`, and peer-signal
requires `SIGNAL_PEER`.

#### Scenario: Wait requires wait right

- GIVEN a handle without `WAIT`
- WHEN a wait operation is attempted
- THEN the operation is invalid

#### Scenario: Signal peer requires peer right

- GIVEN a handle with `SIGNAL` but without `SIGNAL_PEER`
- WHEN peer signaling is attempted
- THEN the operation is invalid

### Requirement: Stable Object Identity

r[zircon-handle.identity]
Every live kernel object MUST have a stable object identity (`koid`).
Duplicate, replace, transfer, and receive MUST preserve the underlying
object `koid`.

#### Scenario: Duplicate preserves koid

- GIVEN a handle bound to object `K`
- WHEN the handle is duplicated
- THEN both source and duplicate refer to object `K`

#### Scenario: Transfer preserves koid

- GIVEN a handle bound to object `K`
- WHEN the handle is sent through a channel and later received
- THEN the received handle refers to object `K`

### Requirement: Handle Ownership State

r[zircon-handle.ownership]
A handle MUST be in exactly one ownership state: bound to a process,
in transit through the kernel, or closed. Bound handles are visible to the
owning process. In-transit handles are not visible to user space. Closed
handles own nothing.

#### Scenario: Channel send makes handle in transit

- GIVEN a handle bound to process `P`
- WHEN the handle is written into a channel message
- THEN the sender no longer owns the handle and the handle becomes in transit

#### Scenario: Receive rebinds ownership

- GIVEN a handle in transit to process `Q`
- WHEN `Q` reads the containing message
- THEN the handle becomes bound to `Q`

#### Scenario: Close ends ownership

- GIVEN any live handle
- WHEN the handle is closed
- THEN the handle enters the closed state

### Requirement: Object Lifetime Follows Live Handles

r[zircon-handle.lifetime]
An object MUST remain live while any bound or in-transit handle refers to it.
Closing the last such handle MAY destroy the object or place it into an
irreversible final state, but the object MUST NOT be considered live after
all references are gone.

#### Scenario: In-transit handle keeps object live

- GIVEN the sender has transferred the last bound handle to an object
- WHEN the handle is still in a channel message
- THEN the object remains live

#### Scenario: Last handle closes object

- GIVEN an object with exactly one remaining live handle
- WHEN that handle is closed and no in-transit references remain
- THEN the object is no longer live

### Requirement: Wait Readiness

r[zircon-wait.readiness]
Waiting on a waitable object MUST be driven by its signal mask. A wait is
ready exactly when the intersection of the observed signals and the requested
signals is non-empty.

#### Scenario: Matching signal wakes wait

- GIVEN a wait request for mask `M` and object signals `S`
- WHEN `S ∩ M` becomes non-empty
- THEN the wait is satisfied

#### Scenario: No matching signal keeps wait pending

- GIVEN a wait request for mask `M` and object signals `S`
- WHEN `S ∩ M` is empty
- THEN the wait remains unsatisfied

### Requirement: Wait-Set Evaluation

r[zircon-wait.wait-set]
A wait over multiple watched handles MUST report exactly the watched entries
whose observed signal masks intersect their requested masks.

#### Scenario: One watched handle becomes ready

- GIVEN two watched handles `A` and `B`
- WHEN only `A` has a non-empty requested/observed intersection
- THEN the wait-set result marks `A` ready and `B` not ready

#### Scenario: Multiple watched handles become ready

- GIVEN watched handles `A` and `B`
- WHEN both have non-empty requested/observed intersections
- THEN the wait-set result marks both ready

### Requirement: Event User Signals

r[zircon-event.user-signals]
An event MUST behave as a pure signal carrier. User-signal set/clear
operations MUST affect only the named user-signal bits and MUST be idempotent.

#### Scenario: Set same user signal twice

- GIVEN an event with user signal `U0` inactive
- WHEN `U0` is set twice
- THEN `U0` is active and no other bit changes

#### Scenario: Clear inactive user signal

- GIVEN an event with user signal `U1` inactive
- WHEN `U1` is cleared
- THEN the event state is unchanged

### Requirement: Eventpair Peer Closed

r[zircon-event.peer-closed]
For an eventpair, loss of all live handles to one endpoint MUST assert the
peer-closed signal on the opposite endpoint.

#### Scenario: One side closes

- GIVEN both endpoints of an eventpair are live
- WHEN all handles to endpoint `B` are closed
- THEN endpoint `A` observes `PEER_CLOSED`

#### Scenario: Peer still live

- GIVEN at least one live handle to endpoint `B`
- WHEN endpoint `A` is inspected
- THEN `PEER_CLOSED` is not asserted solely from handle state

### Requirement: Channel Datagram Semantics

r[zircon-channel.datagram]
Channels MUST behave as datagram endpoints. Each write appends one whole
message. Reads consume one whole message. The model MUST NOT split a single
message into partial reads or partial writes.

#### Scenario: Read returns one whole message

- GIVEN a channel queue with messages `m1`, `m2`
- WHEN one read occurs
- THEN the read returns `m1` and `m2` remains queued

#### Scenario: Empty channel not readable

- GIVEN an empty channel queue
- WHEN readability is checked
- THEN the channel is not readable

### Requirement: Channel Handle Transfer

r[zircon-channel.transfer]
Sending handles through a channel MUST preserve object identity and handle
rights while changing ownership state. After a successful send, transferred
handles are no longer owned by the sender. After a successful receive, the
receiver owns fresh handles to the same objects with the same rights.

#### Scenario: Send removes sender ownership

- GIVEN process `P` sends handle `H` through a channel
- WHEN the send succeeds
- THEN `P` no longer owns `H`

#### Scenario: Receive restores ownership with same rights

- GIVEN process `Q` receives a message carrying handle `H`
- WHEN the receive succeeds
- THEN `Q` owns a handle to the same `koid` with the same rights as `H`

### Requirement: Unread In-Flight Handles Close On Peer Loss

r[zircon-channel.discard]
If a channel endpoint is closed while messages carrying handles are still in
flight to that endpoint, those unread messages MUST be discarded and the
transferred handles they contain MUST be closed.

#### Scenario: Peer closes before receive

- GIVEN a message carrying handles is queued to endpoint `B`
- WHEN all handles to endpoint `B` are closed before the message is read
- THEN the queued message is discarded and its transferred handles are closed

### Requirement: Channel Signals

r[zircon-channel.signals]
Channel signals MUST reflect endpoint state. At minimum, a non-empty receive
queue asserts `READABLE`, and loss of the peer endpoint asserts
`PEER_CLOSED`.

#### Scenario: Queue becomes readable after write

- GIVEN an empty channel endpoint
- WHEN a peer writes one message
- THEN the receiving endpoint becomes `READABLE`

#### Scenario: Read clears readable on last message

- GIVEN a channel endpoint with exactly one queued message
- WHEN that message is read
- THEN `READABLE` becomes inactive

### Requirement: Futex Wait Compare

r[zircon-futex.wait-compare]
A futex wait MUST block only when the observed futex value still matches the
expected value supplied by the waiter. If the value differs, the wait does not
join the waiter set.

#### Scenario: Value matches expected

- GIVEN futex value `v` and expected value `v`
- WHEN wait is attempted
- THEN the waiter joins the futex wait set

#### Scenario: Value differs from expected

- GIVEN futex value `v` and expected value `w` where `v != w`
- WHEN wait is attempted
- THEN the waiter does not block on the futex

### Requirement: Futex Wake Bound

r[zircon-futex.wake-bound]
A futex wake operation MUST release at most the requested number of waiters.
Waking an empty futex MUST be a no-op.

#### Scenario: Wake fewer than waiters

- GIVEN a futex with `n` blocked waiters and wake count `k < n`
- WHEN wake occurs
- THEN exactly `k` waiters are released and `n-k` remain blocked

#### Scenario: Wake empty futex

- GIVEN a futex with zero blocked waiters
- WHEN wake occurs
- THEN zero waiters are released

### Requirement: Futex Requeue Conservation

r[zircon-futex.requeue]
A futex requeue operation MUST wake the requested prefix of waiters from the
source futex and move the remaining requested waiters to the destination
futex without duplication or loss. Total blocked waiters after the operation
MUST equal the original blocked waiters minus the explicitly woken waiters.

#### Scenario: Wake some and requeue rest

- GIVEN source futex `A` with `n` waiters and destination futex `B` with `m` waiters
- WHEN requeue wakes `k` waiters from `A` and moves `r` others to `B`
- THEN `A` loses `k + r` waiters, `B` gains `r`, and total blocked waiters become `m + n - k`

#### Scenario: Requeue with no destination moves

- GIVEN source futex `A`
- WHEN requeue wakes waiters but moves zero waiters
- THEN the result is equivalent to wake-only on `A`

## Cross-Module Properties

### Requirement: Rights Explain Handle Operations

r[properties.zircon-rights-handle]
Every successful handle operation in the Zircon model MUST be justified by a
corresponding rights predicate from `zircon_rights`.

### Requirement: Event and Channel State Use Signal Algebra

r[properties.zircon-signal-refinement]
Event, eventpair, and channel signal computations MUST refine the generic set
operations from `signal.rs`.

### Requirement: Channel Transfer Preserves Lifetime

r[properties.zircon-channel-lifetime]
Channel send/receive/peer-close behavior MUST preserve the lifetime invariant
that objects remain live while any bound or in-transit handle exists.

### Requirement: Futex Wake Unblocks Tasks

r[properties.zircon-futex-process-state]
A waiter released by futex wake or requeue MUST correspond to a
`Blocked -> Ready` transition in the abstract task-state model.
