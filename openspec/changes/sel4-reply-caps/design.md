## Context

Reply capabilities are one of seL4's sharpest linear resources. A separate change lets the repo verify one-shot call/reply discipline and revoke/cancel invalidation without bundling all IPC behavior together.

## Goals / Non-Goals

**Goals:**
- Make reply caps one-shot.
- Preserve caller/callee pairing.
- Invalidate stale reply caps on cancel or revoke.

**Non-Goals:**
- General send/receive queue logic
- Notification wakeup
- Scheduler selection

## Decisions

### 1. Model reply caps as explicit linear state

**Choice:** Model reply caps as explicit linear state
**Rationale:** One-shot use is easiest to prove with a small lifecycle FSM.

### 2. Store caller/callee identities in the live state

**Choice:** Store caller/callee identities in the live state
**Rationale:** That makes stale or cross-thread replies impossible by construction.

### 3. Keep creation and consumption separate

**Choice:** Keep creation and consumption separate
**Rationale:** Call creates a reply cap; reply consumes it. This matches the proof obligations.

## Risks / Trade-offs

**[Cancellation paths can create stale-state bugs]** Add an explicit invalidated state and prove no transition reactivates it.
**[Interaction with endpoint semantics]** State only pairing and single-use guarantees here; leave full IPC composition to cross-module properties.
