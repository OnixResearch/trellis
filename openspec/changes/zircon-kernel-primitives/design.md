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
- Prove cross-module refinement against existing generic modules

**Non-Goals:**
- Exact syscall ABI or FIDL surface
- User-visible handle integers and reuse policy
- Full task tree semantics for jobs/processes/threads
- VM, pager, port, and scheduler subsystems
- Kernel-internal lock validation or low-level arch invariants

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

### 8. Keep scope to object and synchronization primitives

**Choice:** Leave ports, VMOs, VMARs, pager objects, and scheduler policies
out of this change.

**Rationale:** Handles, rights, signals, channels, events, and futexes already
make a coherent and useful slice. Pulling in VM or scheduler state would turn
this into a much larger kernel-modeling project.

## Module Plan

- `src/zircon_rights.rs`
- `src/zircon_handle.rs`
- `src/zircon_wait.rs`
- `src/zircon_event.rs`
- `src/zircon_channel.rs`
- `src/zircon_futex.rs`
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
