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

### 2. Refine generic process-state ideas

**Choice:** Refine generic process-state ideas
**Rationale:** Existing lifecycle/state-machine patterns already exist and should be reused, not replaced.

### 3. Keep scheduling separate

**Choice:** Keep scheduling separate
**Rationale:** This change states what runnable means; ready queues will model where runnable threads live.

## Risks / Trade-offs

**[Blocked-reason drift across modules]** Make the blocked reason part of the TCB state itself.
**[Over-modeling low-level thread internals]** Stick to semantic state and transitions only.
