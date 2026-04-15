## Context

Many seL4 primitives depend on TCB legality: runnable threads must differ from blocked threads, and blocked states must name the object causing the block. A dedicated change gives later IPC, notification, reply, and scheduler modules a precise thread-state base.

## Goals / Non-Goals

**Goals:**
- Enumerate legal TCB states.
- Attach blocked states to explicit objects.
- Prove only legal transitions occur.

**Non-Goals:**
- Scheduler selection
- Capability lookup
- Object creation and memory layout

## Decisions

### 1. Use explicit blocked-state variants

**Choice:** Use explicit blocked-state variants
**Rationale:** A blocked state should carry its cause, not a generic blocked bit.

### 2. Reuse `process_state` proof shape, not its types

**Choice:** Define a dedicated `sel4_tcb::TcbState` enum and reuse `process_state` only as a state-machine pattern reference.
**Rationale:** seL4 blocked states must carry object identifiers for endpoints, notifications, and reply objects; the existing generic process-state model does not encode those payloads.

### 3. Keep scheduling separate

**Choice:** Keep scheduling separate
**Rationale:** This change states what runnable means; ready queues will model where runnable threads live. `Running` remains an inspectable legal state, but this phase does not add scheduler dispatch helpers that construct it.

## Constructor vs requires policy

Public helpers stay total and use constructor-normalized inputs:
- `new_tcb(tid, priority)` constructs a legal runnable TCB.
- Transition helpers such as `block_on_send`, `block_on_receive`, `block_on_notification`, `block_on_reply`, `wake`, `restart`, and `deactivate` have no `requires` clauses.
- Invalid transitions are represented by returning the unchanged state or `false`, not by precondition failure.

## Verification approach

- `sel4-tcb.state`: define a `TcbState` enum plus `legal_state_spec` / `runnable_state_spec`, then prove each constructor and transition result satisfies the legality predicate.
- `sel4-tcb.blocking-object`: define a spec accessor/predicate over blocked variants that extracts the blocking object id, then add `proof fn` showing every blocked constructor carries one.
- `sel4-tcb.transitions`: define a `legal_transition_spec(old_state, new_state)` relation, implement exec transition helpers with `ensures` clauses that match it, and add proof lemmas for wake/restart/deactivate legality.

## Risks / Trade-offs

**[Blocked-reason drift across modules]** Make the blocked reason part of the TCB state itself.
**[Over-modeling low-level thread internals]** Stick to semantic state and transitions only.
