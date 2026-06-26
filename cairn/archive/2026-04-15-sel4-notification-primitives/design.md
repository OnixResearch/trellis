## Context

Notifications are separate from endpoints in seL4 and have their own sharp rule: pending badges accumulate by OR until delivery. This change isolates signal accumulation, wait/poll consumption, and TCB binding behavior.

## Goals / Non-Goals

**Goals:**
- Accumulate pending badges by OR.
- Deliver exactly the pending badge state.
- Wake the bound waiter when eligible.

**Non-Goals:**
- Endpoint sender/receiver queues
- Reply-cap creation
- Global scheduler proofs

## Data Model

The notification state is:
- `Notification { pending_badge: u64, bound_tcb: Option<usize>, waiting_tcb: Option<usize> }`

`pending_badge` is the undelivered OR-accumulated badge word. `bound_tcb` is optional local binding. `waiting_tcb` tracks one locally modeled waiter.

## Public API Surface

- `new_notification(bound_tcb) -> Notification`
- `bind_tcb(&mut Notification, tcb)` / `unbind_tcb(&mut Notification)`
- `signal(&mut Notification, badge) -> Option<usize>` returns the woken TCB when one eligible waiter exists
- `wait(&mut Notification, waiter) -> Option<u64>` returns the exact pending badge when one is available, otherwise records an eligible waiter
- `poll(&mut Notification) -> Option<u64>` returns and consumes the exact pending badge without registering a waiter

## Constructor vs requires policy

Public helpers use constructor-normalized inputs instead of proof-heavy `requires`:
- `Notification` values are well formed by construction.
- `wait` accepts any `waiter` ID; if a binding exists and `waiter` is not the bound TCB, the call is a no-op registration-wise.
- `signal`, `wait`, and `poll` are total over all notification states.

## Trusted boundary expectation

This design expects `0` new `#[verifier::external_body]` or `assume_specification[...]` bridges.

## Decisions

### 1. Pending state is badge-valued

**Choice:** Pending state is badge-valued
**Rationale:** The key semantic object is the accumulated badge word, not a generic queue.

### 2. Keep binding explicit

**Choice:** Keep binding explicit
**Rationale:** Bound-TCB wakeup should be visible in the notification state machine.

### 3. Separate signal and consume steps

**Choice:** Separate signal and consume steps
**Rationale:** This keeps OR-accumulation and delivery proofs small and compositional.

### 4. Mirror Tracey anchors into `docs/requirements.md`

**Choice:** Add `r[sel4-notification.pending-badge]`, `r[sel4-notification.consume]`, and `r[sel4-notification.binding]` to `docs/requirements.md` and tag the defining code artifacts with `r[impl]` / `r[verify]`.
**Rationale:** Tracey coverage in this repo is driven by `docs/requirements.md`.

## Verification Strategy

- `proof fn repeated_signal_accumulates_by_or` proves repeated `signal` steps accumulate badges by OR.
- `proof fn delivery_returns_exact_pending` proves a nonzero pending state is returned exactly by `wait`/`poll`.
- `proof fn eligible_bound_waiter_wakes` proves `signal` returns the bound waiter when `waiting_tcb == bound_tcb`.
- Unit tests cover two-signal accumulation, wait consume, poll consume, bound wake, and ineligible wait behavior.
- Proptests cover randomized OR accumulation, exact wait delivery, and bound-waiter wakeups.

## Risks / Trade-offs

**[Consumption semantics can blur wait vs poll]** Specify both against the same delivered-pending abstraction.
**[Binding could overreach into scheduler theory]** State only local wake eligibility here and leave ready-queue composition to later changes.
**[Single-waiter model is narrower than kernel queues]** Keep this change local to notification semantics; endpoint/ready-queue composition stays in later seL4 changes.
