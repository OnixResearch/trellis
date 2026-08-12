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

### Requirement: Socket Stream Semantics
r[zircon-socket.stream]

Sockets MUST behave as paired, bidirectional byte streams. Write appends
bytes to the peer's receive buffer. Read consumes bytes from the local
receive buffer. Short reads (fewer bytes available than requested) and
short writes (fewer bytes of buffer space than offered) MUST be permitted.
Sockets MUST NOT transfer handles.

#### Scenario: Write fills peer receive buffer

- GIVEN socket endpoint `A` writes `n` bytes
- WHEN endpoint `B` reads
- THEN `B` receives the same `n` bytes in order

#### Scenario: Short write on full buffer

- GIVEN a socket with `k` bytes of free buffer space and a write of `n > k` bytes
- WHEN the write occurs
- THEN at most `k` bytes are accepted

#### Scenario: Short read on partial data

- GIVEN a socket with `k` bytes available and a read requesting `n > k` bytes
- WHEN the read occurs
- THEN exactly `k` bytes are returned

### Requirement: Socket Write Disposition
r[zircon-socket.disposition]

Each socket endpoint's write path MUST be independently enableable and
disableable. When writing is disabled on an endpoint, writes to that
endpoint MUST fail and `ZX_SOCKET_WRITE_DISABLED` MUST be asserted.
The peer MUST observe `ZX_SOCKET_PEER_WRITE_DISABLED`. Disabling writes
MUST NOT discard data already in the buffer.

#### Scenario: Disable write then attempt write

- GIVEN endpoint `A` has writing disabled
- WHEN `A` attempts to write
- THEN the write is rejected

#### Scenario: Peer observes write disabled

- GIVEN endpoint `A` has writing disabled
- WHEN endpoint `B` checks signals
- THEN `B` observes `PEER_WRITE_DISABLED`

#### Scenario: Existing data survives disable

- GIVEN endpoint `A` has written data, then disables writes
- WHEN endpoint `B` reads
- THEN `B` receives the previously written data

### Requirement: Socket Threshold Signals
r[zircon-socket.thresholds]

A socket MUST support configurable read and write thresholds. When the
queued byte count meets or exceeds the read threshold,
`ZX_SOCKET_READ_THRESHOLD` MUST be asserted. When available buffer space
meets or exceeds the write threshold, `ZX_SOCKET_WRITE_THRESHOLD` MUST
be asserted. A threshold of zero disables the corresponding signal.

#### Scenario: Read threshold fires

- GIVEN a read threshold of `T` and `T` bytes queued
- WHEN signals are evaluated
- THEN `READ_THRESHOLD` is asserted

#### Scenario: Write threshold fires

- GIVEN a write threshold of `T` and at least `T` bytes of free space
- WHEN signals are evaluated
- THEN `WRITE_THRESHOLD` is asserted

### Requirement: Socket Peer Closed
r[zircon-socket.peer-closed]

Closing all handles to one socket endpoint MUST assert `PEER_CLOSED` on
the opposite endpoint, following the same rule as eventpairs.

#### Scenario: Peer closes

- GIVEN both socket endpoints are live
- WHEN all handles to endpoint `B` are closed
- THEN endpoint `A` observes `PEER_CLOSED`

### Requirement: Port Async Wait Registration
r[zircon-port.async-wait]

A waitable object MUST be bindable to a port with a signal mask and a
user-provided key. When the observed signals intersect the registered mask,
a packet containing the key and observed signals MUST be delivered to the
port queue.

#### Scenario: Signal fires after bind

- GIVEN object `O` is bound to port `P` with mask `M` and key `K`
- WHEN `O` asserts a signal in `M`
- THEN `P` receives a packet with key `K` and the matching signals

#### Scenario: No matching signal

- GIVEN object `O` is bound to port `P` with mask `M`
- WHEN `O` asserts signals disjoint from `M`
- THEN no packet is delivered to `P`

### Requirement: Port One-Shot vs Repeating
r[zircon-port.one-shot]

An async wait registration MUST be one-shot by default: after one packet
is delivered the registration is consumed. A repeating registration
MUST re-arm automatically after each delivery.

#### Scenario: One-shot consumed after delivery

- GIVEN a one-shot registration of `O` on port `P`
- WHEN the first matching signal fires and is delivered
- THEN the registration is removed and subsequent signals do not deliver

#### Scenario: Repeating re-arms

- GIVEN a repeating registration of `O` on port `P`
- WHEN a matching signal fires and is delivered
- THEN the registration remains active for future signals

### Requirement: Port User-Queued Packets
r[zircon-port.user-queue]

A port MUST accept explicitly queued packets via a queue operation.
User-queued packets MUST be delivered in FIFO order relative to other
user-queued packets.

#### Scenario: Two user packets in order

- GIVEN packets `p1` then `p2` are queued to port `P`
- WHEN `P` is waited on twice
- THEN `p1` is delivered before `p2`

### Requirement: Timer Set/Cancel/Reset Lifecycle
r[zircon-timer.lifecycle]

A timer MUST support set, cancel, and reset operations.
`ZX_TIMER_SIGNALED` MUST be asserted when the deadline passes and MUST
remain asserted until the timer is canceled or reset. Canceling a
non-signaled timer MUST be a no-op. Resetting a signaled timer MUST
clear the signal and arm a new deadline.

#### Scenario: Deadline passes

- GIVEN a timer set with deadline `D`
- WHEN the current time reaches `D`
- THEN `ZX_TIMER_SIGNALED` is asserted

#### Scenario: Signal persists until cancel

- GIVEN a signaled timer
- WHEN no cancel or reset has occurred
- THEN `ZX_TIMER_SIGNALED` remains asserted

#### Scenario: Cancel clears signal

- GIVEN a signaled timer
- WHEN cancel is called
- THEN `ZX_TIMER_SIGNALED` is deasserted

#### Scenario: Reset re-arms

- GIVEN a signaled timer
- WHEN reset is called with new deadline `D'`
- THEN `ZX_TIMER_SIGNALED` is deasserted and the timer arms for `D'`

#### Scenario: Cancel non-signaled is no-op

- GIVEN a timer that is armed but not yet signaled
- WHEN cancel is called
- THEN the timer is disarmed and no signal change occurs

### Requirement: FIFO Element-Aligned Semantics
r[zircon-fifo.element-aligned]

FIFOs MUST transfer data in fixed-size elements. Reads and writes MUST
operate on whole elements only — partial element reads or writes MUST
NOT occur. A FIFO MUST have a bounded element capacity set at creation.

#### Scenario: Write and read whole elements

- GIVEN a FIFO with element size `E` and capacity `C`
- WHEN `k` elements are written and then read
- THEN exactly `k` elements are returned, each of size `E`, in FIFO order

#### Scenario: Write beyond capacity

- GIVEN a FIFO with `f` free element slots and a write of `n > f` elements
- WHEN the write occurs
- THEN at most `f` elements are accepted

### Requirement: FIFO Peer Closed
r[zircon-fifo.peer-closed]

Closing all handles to one FIFO endpoint MUST assert `PEER_CLOSED` on the
opposite endpoint.

#### Scenario: Peer closes

- GIVEN both FIFO endpoints are live
- WHEN all handles to one endpoint are closed
- THEN the other endpoint observes `PEER_CLOSED`

### Requirement: FIFO Signals
r[zircon-fifo.signals]

A FIFO endpoint MUST assert `READABLE` when its receive queue is non-empty
and `WRITABLE` when its transmit queue has free element slots.

#### Scenario: Readable after write

- GIVEN an empty FIFO
- WHEN the peer writes one element
- THEN the receiving endpoint becomes `READABLE`

#### Scenario: Writable with free slots

- GIVEN a FIFO with free capacity
- WHEN the endpoint checks signals
- THEN `WRITABLE` is asserted

### Requirement: Job Containment Hierarchy
r[zircon-job.containment]

Jobs MUST form a tree. Every job except the root job MUST have exactly one
parent job. A job's children are other jobs and member processes. A process
MUST belong to exactly one job.

#### Scenario: Job has unique parent

- GIVEN a non-root job `J`
- WHEN `J`'s parent is queried
- THEN exactly one parent job is returned

#### Scenario: Process belongs to one job

- GIVEN a process `P` created in job `J`
- WHEN `P`'s owning job is queried
- THEN `J` is returned

### Requirement: Job Kill Propagation
r[zircon-job.kill-propagation]

Killing a job MUST recursively kill all child jobs and all member processes.
After a kill, no process or child job in the subtree MUST remain in a
runnable state.

#### Scenario: Kill propagates to children

- GIVEN job `J` with child job `C` containing process `P`
- WHEN `J` is killed
- THEN `C` is killed and `P` is terminated

#### Scenario: Leaf job kill

- GIVEN job `J` with no child jobs and processes `P1`, `P2`
- WHEN `J` is killed
- THEN both `P1` and `P2` are terminated

### Requirement: Job Critical Process
r[zircon-job.critical-process]

A process MAY be designated as critical to a job. If a critical process
exits abnormally (non-zero return code), the owning job MUST be killed.

#### Scenario: Critical process crashes

- GIVEN process `P` is critical to job `J`
- WHEN `P` exits with a non-zero return code
- THEN `J` is killed

#### Scenario: Critical process exits cleanly

- GIVEN process `P` is critical to job `J`
- WHEN `P` exits with return code zero
- THEN `J` is not killed

### Requirement: Job Keeps Parents Alive
r[zircon-job.parent-lifetime]

A job with live children (child jobs or running processes) MUST keep its
parent chain up to the root job alive, even if all handles to intermediate
jobs are closed.

#### Scenario: Handleless parent kept alive by child

- GIVEN job `J` is the parent of job `C` with a running process
- WHEN all handles to `J` are closed
- THEN `J` remains live because `C` has live descendants

## Cross-Module Properties

### Requirement: Rights Explain Handle Operations
r[properties.zircon-rights-handle]

Every successful handle operation in the Zircon model MUST be justified by a
corresponding rights predicate from `zircon_rights`.

### Requirement: Peered Object Signals Use Signal Algebra
r[properties.zircon-signal-refinement]

Event, eventpair, channel, socket, and FIFO signal computations MUST refine
the generic set operations from `signal.rs`.

### Requirement: Channel Transfer Preserves Lifetime
r[properties.zircon-channel-lifetime]

Channel send/receive/peer-close behavior MUST preserve the lifetime invariant
that objects remain live while any bound or in-transit handle exists.

### Requirement: Port Delivery Refines Wait Readiness
r[properties.zircon-port-wait]

A port packet delivery from an async wait MUST correspond to a satisfied
wait-readiness predicate from `zircon_wait`.

### Requirement: Timer Signaling Respects Deadline Order
r[properties.zircon-timer-deadline]

Timer signaling MUST be consistent with the deadline ordering from
`deadline_queue`: if timer `A` has an earlier deadline than timer `B`,
`A` MUST become signaled no later than `B`.

### Requirement: Job Kill Transitions Process State
r[properties.zircon-job-process-state]

A process terminated by job kill MUST undergo the same state transition
(`Running -> Terminated`) modeled in `process_state`.

### Requirement: Futex Wake Unblocks Tasks
r[properties.zircon-futex-process-state]

A waiter released by futex wake or requeue MUST correspond to a
`Blocked -> Ready` transition in the abstract task-state model.
