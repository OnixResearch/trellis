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

## Data Model

The core model is:
- `ReplyState = Live | Consumed | Invalidated`
- `ReplyCap { caller: usize, callee: usize, state: ReplyState }`

Caller/callee identity never changes after creation. Only `state` changes.

## Public API Surface

- `new_reply_cap(caller, callee) -> ReplyCap`
- `reply(&mut ReplyCap, caller, callee) -> bool`
- `cancel(&mut ReplyCap)`
- `revoke(&mut ReplyCap)`

A successful `reply` requires both matching caller/callee identity and `state == Live`; it returns `true` and transitions to `Consumed`. All other reply attempts return `false` and leave state unchanged. `cancel` and `revoke` transition `Live -> Invalidated` and leave non-live caps unchanged.

## Constructor vs requires policy

Public helpers use constructor-normalized inputs instead of `requires`:
- `new_reply_cap` constructs a well-formed live reply cap.
- `reply`, `cancel`, and `revoke` are total over all reply-cap states.
- Mismatched caller/callee pairs are handled by a `false` return from `reply`, not a precondition failure.

## Trusted boundary expectation

This design expects `0` new `#[verifier::external_body]` or `assume_specification[...]` bridges.

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

### 4. Mirror Tracey anchors into `docs/requirements.md`

**Choice:** Add `r[sel4-reply.one-shot]`, `r[sel4-reply.pairing]`, and `r[sel4-reply.invalidate]` to `docs/requirements.md` and tag the defining code artifacts with `r[impl]` / `r[verify]`.
**Rationale:** Tracey coverage in this repo is driven by `docs/requirements.md`.

## Verification Strategy

- `proof fn first_reply_consumes_live_cap` proves a matching reply transition on a live cap makes later matching reply impossible.
- `proof fn cross_pair_reply_rejected` proves mismatched caller/callee identities cannot use or consume a live reply cap.
- `proof fn cancel_and_revoke_invalidate_live_cap` proves both invalidation transitions leave the cap unusable.
- Unit tests cover first reply success, second reply denial, cross-pair rejection, and cancel/revoke invalidation.
- Proptests cover one-shot use, mismatched-pair rejection, and cancelled-cap denial over randomized caller/callee IDs.

## Risks / Trade-offs

**[Cancellation paths can create stale-state bugs]** Keep an explicit `Invalidated` state and prove no invalidation transition reactivates it.
**[Interaction with endpoint semantics]** State only pairing and single-use guarantees here; leave full IPC composition to cross-module properties.
**[Reply creation is more abstract than the real kernel path]** Treat reply-cap creation as a direct constructor in this change so the linear-resource proof stays local.
