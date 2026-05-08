# BEAM Primitives Specification

## Purpose

Defines requirements for a verified model of core Erlang BEAM virtual machine
primitives: process lifecycle, mailbox and selective receive, asynchronous
message send, bidirectional links, unidirectional monitors, reduction-counted
scheduling, and the process registry. Scope is semantic behavior and preserved
invariants, not bytecode representation or distribution protocol details.

## Requirements

### Requirement: Process State Monotonicity

A process MUST transition through states `alive → exiting → dead` in
strictly forward order. No transition from `exiting` back to `alive` or
from `dead` to any other state is permitted.

r[beam-process.lifecycle]

#### Scenario: Normal exit

- GIVEN a process in state `alive`
- WHEN the process completes or receives an untrapped exit signal
- THEN the process transitions to `exiting` and then to `dead`

#### Scenario: Already dead process ignores signals

- GIVEN a process in state `dead`
- WHEN any exit signal or message is directed at the process
- THEN the process state remains `dead`

### Requirement: Process Heap Isolation

Processes with distinct pid identities MUST have logically isolated heaps.
A mutation to one process's heap MUST NOT be observable by any process with
another pid. The only inter-process data path is message copying.

r[beam-process.isolation]

#### Scenario: Distinct pids imply distinct heaps

- GIVEN processes `A` and `B` have different pids
- WHEN their heap identities are compared
- THEN the heap identities are different

#### Scenario: Message is a copy

- GIVEN process `A` sends term `T` to process `B`
- WHEN `B` receives the message
- THEN `B` holds an independent copy; mutation of `A`'s original does not
  affect `B`'s copy

### Requirement: Mailbox FIFO Ordering

A process mailbox MUST maintain FIFO ordering of received messages. Messages
from distinct senders MAY interleave, but messages from the same sender MUST
appear in send order.

r[beam-mailbox.ordering]

#### Scenario: Same-sender ordering preserved

- GIVEN process `A` sends messages `m1` then `m2` to process `B`
- WHEN `B` scans its mailbox
- THEN `m1` appears before `m2`

#### Scenario: Cross-sender interleave

- GIVEN process `A` sends `m1` and process `C` sends `m2` to `B`
- WHEN `B` scans its mailbox
- THEN both messages are present; their relative order is not constrained
  beyond per-sender FIFO

### Requirement: Selective Receive

A `receive` operation MUST scan the mailbox from the current save position
and return the first message matching any of the supplied patterns. Messages
that do not match MUST remain in the mailbox in their original order.

r[beam-mailbox.selective-receive]

#### Scenario: First match returned

- GIVEN mailbox `[a, b, c]` and pattern matching `b`
- WHEN selective receive executes
- THEN `b` is returned and the mailbox becomes `[a, c]`

#### Scenario: No match leaves mailbox unchanged

- GIVEN mailbox `[a, b]` and pattern matching nothing
- WHEN selective receive executes with no timeout
- THEN the process blocks and the mailbox is unchanged

### Requirement: Save Offset Cursor

Between receive attempts, the mailbox cursor (save offset) MUST be
maintained so that already-scanned non-matching messages are not re-scanned.
A successful receive or a new message arrival that resets the cursor MUST
set it to the appropriate position per BEAM semantics.

r[beam-mailbox.save-offset]

#### Scenario: Cursor advances past non-matches

- GIVEN mailbox `[a, b, c]` with cursor at 0 and pattern matching `c`
- WHEN receive scans and finds `c` at position 2
- THEN `c` is consumed and the cursor resets to 0

#### Scenario: New message resets cursor to saved position

- GIVEN a process blocked in receive with save offset `k`
- WHEN a new message arrives
- THEN the scan resumes from offset `k`, not from 0

### Requirement: Asynchronous Send Always Succeeds

A send operation MUST complete from the sender's perspective without
blocking, regardless of the recipient's state. The sender never observes
a send failure due to a full mailbox or dead recipient.

r[beam-send.async]

#### Scenario: Send to live process

- GIVEN process `A` sends message `m` to live process `B`
- WHEN the send executes
- THEN `m` is appended to `B`'s mailbox and `A` continues immediately

#### Scenario: Send to dead process

- GIVEN process `A` sends message `m` to dead process `B`
- WHEN the send executes
- THEN the message is silently dropped and `A` continues immediately

### Requirement: Link Symmetry

Links MUST be symmetric. If process `A` is linked to process `B`, then `B`
is linked to `A`. Creating or destroying a link MUST affect both endpoints
atomically.

r[beam-link.symmetry]

#### Scenario: Link is bidirectional

- GIVEN process `A` links to process `B`
- WHEN the link set of `A` is inspected
- THEN `B` is in `A`'s link set and `A` is in `B`'s link set

#### Scenario: Unlink removes both directions

- GIVEN processes `A` and `B` are linked
- WHEN `A` unlinks from `B`
- THEN neither `A` is in `B`'s link set nor `B` is in `A`'s link set

### Requirement: Exit Signal Propagation Through Links

When a process dies, it MUST send an exit signal carrying its exit reason to
every process in its link set. A process receiving an exit signal with reason
other than `normal` MUST itself begin exiting unless it is trapping exits.

r[beam-link.propagation]

#### Scenario: Abnormal exit cascades

- GIVEN `A` linked to `B` and `B` not trapping exits
- WHEN `A` dies with reason `error`
- THEN `B` receives exit signal `{A, error}` and begins exiting

#### Scenario: Normal exit does not cascade

- GIVEN `A` linked to `B` and `B` not trapping exits
- WHEN `A` dies with reason `normal`
- THEN `B` does not begin exiting

#### Scenario: Link removed before death prevents signal

- GIVEN `A` was linked to `B` but the link was removed
- WHEN `A` dies
- THEN `B` receives no exit signal from `A`

### Requirement: Trap Exit Converts Signals to Messages

A process with the `trap_exit` flag set MUST convert incoming exit signals
into `{'EXIT', Pid, Reason}` messages delivered to its mailbox instead of
triggering its own exit.

r[beam-link.trap-exit]

#### Scenario: Trapped exit becomes message

- GIVEN `B` has `trap_exit = true` and is linked to `A`
- WHEN `A` dies with reason `R`
- THEN `B`'s mailbox receives `{'EXIT', A, R}` and `B` remains alive

#### Scenario: Kill signal bypasses trap

- GIVEN `B` has `trap_exit = true`
- WHEN `B` receives exit signal with reason `kill`
- THEN `B` dies with reason `killed` regardless of trap_exit

### Requirement: Monitor is Unidirectional

A monitor MUST be unidirectional from the monitoring process to the monitored
process. Monitor creation produces watcher-owned monitor state, leaves the
monitored process state unchanged, and only the monitoring process observes
any resulting DOWN message.

r[beam-monitor.unidirectional]

#### Scenario: Monitor does not affect monitored process

- GIVEN `A` monitors `B`
- WHEN the monitor is created
- THEN the returned monitor state is owned by `A`, references `B`, and does
  not mutate `B`'s process state

### Requirement: DOWN Message Delivery

When a monitored process dies, the monitoring process MUST receive a
`{'DOWN', Ref, process, Pid, Reason}` message. This delivery MUST occur
exactly once per monitor reference.

r[beam-monitor.down]

#### Scenario: Monitored process dies

- GIVEN `A` monitors `B` with reference `Ref`
- WHEN `B` dies with reason `R`
- THEN `A` receives `{'DOWN', Ref, process, B, R}`

### Requirement: Late Monitor Delivers noproc

Creating a monitor for an already-dead process MUST immediately yield a
`{'DOWN', Ref, process, Pid, noproc}` message.

r[beam-monitor.late]

#### Scenario: Monitor after death

- GIVEN `B` is already dead when `A` calls `monitor(process, B)`
- WHEN the monitor is created
- THEN `A` immediately receives `{'DOWN', Ref, process, B, noproc}`

### Requirement: Demonitor Prevents DOWN

A successful `demonitor` MUST guarantee that no future DOWN message for that
monitor reference is delivered. If a DOWN message is already in the
monitoring process's mailbox, `demonitor` with the `flush` option MUST
remove it.

r[beam-monitor.demonitor]

#### Scenario: Demonitor before death

- GIVEN `A` monitors `B` with reference `Ref`
- WHEN `A` demonitors `Ref` and then `B` dies
- THEN `A` does not receive a DOWN for `Ref`

#### Scenario: Demonitor with flush

- GIVEN a DOWN message for `Ref` is already in `A`'s mailbox
- WHEN `A` calls `demonitor(Ref, [flush])`
- THEN the DOWN message is removed from `A`'s mailbox

### Requirement: Reduction Budget Preemption

Each process MUST be assigned a reduction budget per scheduling quantum. The
scheduler MUST preempt the running process when its reduction budget is
exhausted.

r[beam-scheduler.preemption]

#### Scenario: Budget exhausted causes preemption

- GIVEN a process running with `k` reductions remaining
- WHEN it performs `k` reductions
- THEN the scheduler preempts it and selects the next runnable process

#### Scenario: Process yields before exhaustion

- GIVEN a process with remaining budget
- WHEN the process explicitly yields
- THEN the process is returned to the run queue with zero remaining budget

### Requirement: Priority Run Queues

The scheduler MUST maintain separate run queues for priority levels `max`,
`high`, `normal`, and `low`. Higher-priority queues MUST be drained before
lower-priority queues receive CPU time, subject to starvation prevention
for `normal` and `low`.

r[beam-scheduler.priority]

#### Scenario: Max priority runs first

- GIVEN processes at `max` and `normal` priority are runnable
- WHEN the scheduler selects the next process
- THEN a `max` priority process is selected

#### Scenario: Low priority eventually runs

- GIVEN only `low` priority processes are runnable
- WHEN the scheduler selects the next process
- THEN a `low` priority process is selected

### Requirement: Registry Uniqueness

The process registry MUST enforce a bijective mapping: each registered name
maps to at most one pid, and each pid has at most one registered name.

r[beam-registry.uniqueness]

#### Scenario: Register succeeds for unique name

- GIVEN name `N` is not registered
- WHEN process `P` registers as `N`
- THEN `whereis(N)` returns `P`

#### Scenario: Duplicate name rejected

- GIVEN name `N` is already registered to process `Q`
- WHEN process `P` attempts to register as `N`
- THEN the registration fails

#### Scenario: Duplicate pid rejected

- GIVEN process `P` is already registered as name `M`
- WHEN `P` attempts to register as name `N`
- THEN the registration fails

### Requirement: Registry Cleanup on Death

When a registered process dies, its registry entry MUST be automatically
removed. After cleanup, the name is available for re-registration.

r[beam-registry.cleanup]

#### Scenario: Dead process unregistered

- GIVEN process `P` is registered as `N`
- WHEN `P` dies
- THEN `whereis(N)` returns `undefined`

**Cross-Module Properties**

### Requirement: Death Propagates Through Links

When a process dies, exit delivery through `beam_link` MUST respect the
link rules for each reached peer: non-trapping alive peers begin exiting on
abnormal signals, trapping peers receive `EXIT` messages, `kill` bypasses
`trap_exit`, and peers with no remaining link observe nothing.

r[properties.beam-death-links]

#### Scenario: Linked death applies link rules

- GIVEN a dying process with linked peers in trapping, non-trapping, and unlinked states
- WHEN exit delivery is evaluated through the link model
- THEN each peer observes exactly the outcome required by its remaining link and trap-exit state

### Requirement: Death Triggers Monitor DOWN

Every active monitor on a dying process MUST produce exactly one DOWN message
delivered to the monitoring process's `beam_mailbox`.

r[properties.beam-death-monitor]

#### Scenario: Active monitor receives one DOWN

- GIVEN a process has one active monitor from a watcher
- WHEN the monitored process dies
- THEN the watcher receives exactly one DOWN message for that monitor reference

### Requirement: Death Finalizes Mailbox

Once a process enters `dead` state, no new messages MUST be added to its
mailbox. Messages sent to a dead process via `beam_send` MUST be dropped.

r[properties.beam-death-mailbox]

#### Scenario: Dead mailbox rejects later sends

- GIVEN a process has entered the `dead` state
- WHEN another process sends it a message
- THEN the message is dropped and the dead process mailbox is unchanged

### Requirement: Death Cleans Registry

A dying process MUST have its registry entry removed so a fresh
register-then-cleanup roundtrip restores the prior registry and the same
name can be registered again without violating `beam_registry` uniqueness.

r[properties.beam-death-registry]

#### Scenario: Death cleanup frees registry name

- GIVEN a registered process dies
- WHEN registry cleanup runs for that pid
- THEN the prior name is removed and can be registered again without violating uniqueness

### Requirement: Scheduler Respects Process State

The scheduler MUST only place `alive` processes with pending work on the run
queue. Processes in `exiting` or `dead` state MUST NOT be scheduled for
reduction execution.

r[properties.beam-scheduler-state]

#### Scenario: Non-alive processes are not scheduled

- GIVEN processes in `alive`, `exiting`, and `dead` states
- WHEN the scheduler builds or selects from its run queue
- THEN only alive processes with pending work are eligible for reduction execution
