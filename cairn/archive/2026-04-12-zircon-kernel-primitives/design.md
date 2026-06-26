## Context

Zircon's kernel API is built around a few rules that fit this repo well:

- object identity is stable (`koid`)
- authority is carried by handle-local rights
- observable object state is a signal mask
- waiting is signal evaluation, not polling
- IPC can move handles between processes
- futexes handle contended blocking without modeling a full mutex API

The repo already has reusable proof pieces for those rules. `signal.rs`
proves bitmask algebra. `ref_count.rs` proves handle-style lifetime facts.
`futex_state.rs` captures the uncontended single-word state machine.
`process_state.rs` already has `Running -> Blocked -> Ready` transitions.
This change should reuse those pieces instead of re-proving generic facts in
Zircon-specific modules.

## Goals / Non-Goals

**Goals:**
- Model Zircon rights as a monotone capability mask
- Model handles as references to stable objects with per-handle rights
- Model in-transit handles during channel send/receive
- Model signal-driven wait readiness for waitable objects
- Model events and eventpairs, including peer-close semantics
- Model channels as datagram IPC with optional transferred handles
- Model futex wait/wake/requeue semantics with multi-waiter state
- Model sockets as paired byte-stream IPC with bounded buffers, short
  reads/writes, write disposition, and threshold signals
- Model ports as async wait multiplexers with bind/deliver/one-shot/repeat
- Model timers as deadline-based waitable objects with set/cancel/reset
- Model FIFOs as fixed-element paired ring buffers with peer-closed
- Model jobs as a containment hierarchy with kill propagation, critical
  process designation, and parent-lifetime rules
- Prove cross-module refinement against existing generic modules

**Non-Goals:**
- Exact syscall ABI or FIDL surface
- User-visible handle integers and reuse policy
- Full thread creation, TLS, and per-thread exception handling
- VMOs, VMARs, pagers, and address-space management
- Timer slack coarsening policy internals
- Kernel-internal lock validation or low-level arch invariants
- Driver objects (interrupts, MSIs, BTIs, PMTs, debuglog, resources)
- Stream objects and counter objects

## Decisions

### 1. Model object identity separately from handle identity

**Choice:** Represent the kernel object by a stable `koid` and treat handle
values as abstract references carrying `(koid, rights, owner_state)`.
Do not model raw `zx_handle_t` integers.

**Rationale:** The process-local integer value of a handle is not the
interesting invariant. Zircon explicitly allows reuse of closed handle
integers. The stable fact is that duplicate/replace/transfer preserve the
underlying object identity while changing rights or owner.

**Alternative:** Model raw 32-bit handle numbers. Rejected because number
reuse and process-local namespace rules would add noise without strengthening
the proofs we want.

### 2. Rights are monotone and live on handles

**Choice:** Put rights in a dedicated `zircon_rights` module with subset,
intersection, attenuation, and operation-check predicates.

**Rationale:** Zircon rights are per-handle, not per-object. Duplicate and
replace may drop rights but must not create new rights. Channel transfer must
preserve the transferred handle's rights exactly. Pulling these facts into one
module makes the handle proofs small.

### 3. Reuse generic signal algebra, add Zircon-specific wrappers

**Choice:** Build `zircon_wait` and `zircon_event` on top of `signal.rs`
instead of inventing another bitmask theory.

**Rationale:** `signal.rs` already proves closure, membership, union,
intersection, and complement properties. Zircon-specific logic should add only
what the generic layer lacks: wait satisfaction, waitability constraints,
user-signal mutation, and peer-derived signals.

### 4. Model channels as endpoint queues plus in-transit handles

**Choice:** Represent a channel endpoint as a finite queue of messages, where
messages may carry zero or more handles. Sending a message moves transferred
handles to `in_transit(channel_id)`. Receiving binds them to the destination
process. Closing the destination endpoint discards unread messages and closes
any handles still in flight to that endpoint.

**Rationale:** This is the sharpest Zircon-specific rule in the object model.
It combines handle lifetime, IPC, and peer state. The queue model is enough to
prove the key invariants without modeling actual bytes or full syscall error
handling.

### 5. Eventpairs derive peer-close from handle lifetime

**Choice:** Do not store `PEER_CLOSED` as independent mutable state.
Instead derive it from the absence of any live handles to the peer endpoint.

**Rationale:** Zircon documents `PEER_CLOSED` as a state change caused by the
other side going away. Deriving it from handle lifetime removes a whole class
of stale-state bugs from the model.

### 6. Waiting is readiness logic, not a scheduler model

**Choice:** `zircon_wait` proves readiness predicates (`wait_one`, wait-set
satisfaction) and only connects to `process_state` at the boundary needed for
futex blocking/unblocking.

**Rationale:** We do not need to verify a scheduler to verify wait semantics.
The key fact is when a wait becomes satisfiable, not how a CPU chooses the next
thread.

### 7. Futexes use explicit waiter sets and requeue conservation

**Choice:** Keep the existing `futex_state` single-word intuition, but add a
Zircon wrapper that tracks waiter counts (or queues) per futex address and
proves:
- wait blocks only when the compared value still matches
- wake releases at most `n` waiters
- requeue preserves the total number of blocked waiters minus the ones woken

**Rationale:** Zircon futexes matter because of wake/requeue behavior, not
because of raw atomic instructions. The model should stay at the kernel
primitive boundary.

### 8. Sockets reuse ring-buffer internals

**Choice:** Model the per-endpoint receive buffer as a bounded byte ring,
reusing concepts from `ring_buffer.rs`. Track write disposition as a
per-endpoint boolean pair.

**Rationale:** Socket buffers are classic ring buffers. Short reads/writes
fall naturally out of a bounded-capacity model. Write disposition is a
two-bit state that controls `WRITABLE` / `PEER_WRITE_DISABLED` signals.

### 9. Ports are packet queues with bound registrations

**Choice:** Model a port as a packet queue plus a set of registrations
`(object_koid, mask, key, mode)`. Signal delivery checks the intersection.
One-shot removes the registration after delivery; repeating keeps it.

**Rationale:** Ports extend the `zircon_wait` model to async / many-object
scale. The key insight is that a port packet is just a materialized
wait-readiness result.

### 10. Timers wrap deadline ordering

**Choice:** Model a timer as `(state: Idle | Armed(deadline) | Signaled,
koid)`. Set transitions to Armed, deadline passage transitions to Signaled,
cancel/reset transitions back. Reuse `deadline_queue` for ordering proofs.

**Rationale:** Timers are the simplest waitable object beyond events. The
only interesting invariant is signal persistence until cancel/reset.

### 11. FIFOs are element-aligned paired ring buffers

**Choice:** Model a FIFO as two `ring_buffer`-style queues with a fixed
element size. Reads and writes operate on whole elements. Peer-closed
follows the same peered-object pattern as eventpairs, sockets, and channels.

**Rationale:** FIFOs exist for efficient fixed-size element transfer on the
control plane of shared-memory transports. Element alignment is their defining
constraint vs. sockets.

### 12. Jobs model the containment tree, not the full task API

**Choice:** Model the job tree as parent/child edges, with kill propagation
as a recursive descent. Track critical-process designation per job. Prove
parent-lifetime: a job with live descendants cannot be finalized.

**Rationale:** The interesting Zircon job invariants are tree structure,
kill propagation, and critical-process crash-kills-job. We do not need
full task_create/task_kill ABI details.

### 13. Keep VMOs, VMARs, and scheduler out of scope

**Choice:** Leave VM, pager, and scheduler objects out of this change.

**Rationale:** The eleven modules (rights, handles, wait, events, channels,
futexes, sockets, ports, timers, FIFOs, jobs) already cover the core object
model, all IPC primitives, synchronization, async waiting, and task
containment. VM and scheduler are coherent follow-up changes.

## Module Plan

- `src/zircon_rights.rs`
- `src/zircon_handle.rs`
- `src/zircon_wait.rs`
- `src/zircon_event.rs`
- `src/zircon_channel.rs`
- `src/zircon_futex.rs`
- `src/zircon_socket.rs`
- `src/zircon_port.rs`
- `src/zircon_timer.rs`
- `src/zircon_fifo.rs`
- `src/zircon_job.rs`
- `src/properties.rs` additions for cross-module theorems

## Risks / Trade-offs

**[Channel transfer proof shape]** The hardest part is likely unread in-flight
handle cleanup on peer close. Mitigation: make messages explicit queue items so
"all unread handles are closed" becomes a fold/filter property over the queue.

**[Signal namespace precision]** Zircon signals are object-specific and only a
subset apply to each object type. Mitigation: keep one generic signal mask but
add per-object validity predicates where needed.

**[Futex requeue complexity]** Requeue touches two wait queues at once.
Mitigation: prove a small conservation theorem first, then layer wake-count and
blocked->ready properties on top.

**[Over-modeling ABI details]** Exact syscall error codes could bog the work
down. Mitigation: keep the model at the semantic level: preconditions,
state transitions, and preserved invariants.

**[Socket buffer proof shape]** Short read/write proofs need careful bounding.
Mitigation: reuse `ring_buffer` well-formedness invariant as the abstraction
boundary.

**[Port registration set management]** Tracking many registrations per port
could be complex. Mitigation: model registrations as a `Seq` with lookup and
removal; avoid indexing tricks.

**[Job tree recursion]** Kill propagation requires structural recursion over
the job tree. Mitigation: use `decreases` on tree depth, same pattern as
`nix_closure`.
