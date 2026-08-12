## Context

IRQ handler binding and masked delivery are compact, high-value kernel rules. A separate change gives the repo a reusable interrupt-control model without trying to verify hardware controller internals.

## Goals / Non-Goals

**Goals:**
- Bind at most one live handler per IRQ.
- Block delivery when masked.
- Consume pending state on ack according to the abstract IRQ model.

**Non-Goals:**
- Device-driver logic
- Hardware register protocols
- Endpoint IPC

## Data Model

The core state is one abstract IRQ line:
- `IrqLine { irq: usize, handler: Option<usize>, pending: bool, masked: bool }`

This is intentionally per-line, not a global table. The uniqueness requirement is local to a single IRQ line object: one line stores at most one live handler binding.

## Public API Surface

- `new_irq_line(irq) -> IrqLine`
- `bind_handler(&mut IrqLine, handler) -> bool`
- `unbind_handler(&mut IrqLine)`
- `mask(&mut IrqLine)` / `unmask(&mut IrqLine)`
- `set_pending(&mut IrqLine)`
- `acknowledge(&mut IrqLine)`
- `is_deliverable(&IrqLine) -> bool`

`bind_handler` succeeds only when the line is currently unbound. `is_deliverable` is true exactly when the line is pending, unmasked, and bound. `acknowledge` clears `pending` regardless of current mask state.

## Constructor vs requires policy

Public helpers are total and use constructor-normalized inputs:
- `new_irq_line` constructs a well-formed line.
- `bind_handler`, `mask`, `unmask`, `set_pending`, `acknowledge`, and `is_deliverable` have no `requires` clauses.
- Invalid rebinding is represented by a `false` return from `bind_handler`, not a precondition failure.

## Trusted boundary expectation

This design expects `0` new `#[verifier::external_body]` or `assume_specification[...]` bridges.

## Decisions

### 1. Keep an abstract pending/masked model

**Choice:** Keep an abstract pending/masked model
**Rationale:** The kernel invariants care about deliverability, not device-controller register details.

### 2. Separate binding from delivery

**Choice:** Separate binding from delivery
**Rationale:** Unique ownership and deliverability are different proof obligations.

### 3. Ack is a state transition

**Choice:** Ack is a state transition
**Rationale:** Acknowledgement should explicitly consume pending delivery state in the model.

### 4. Mirror Tracey anchors into `docs/requirements.md`

**Choice:** Add `r[sel4-irq.binding]`, `r[sel4-irq.masking]`, and `r[sel4-irq.ack]` to `docs/requirements.md` and tag the defining code artifacts with `r[impl]` / `r[verify]`.
**Rationale:** Tracey coverage in this repo is driven by `docs/requirements.md`.

## Verification Strategy

- `proof fn second_live_bind_rejected` proves a second bind on an already-bound line leaves the binding unchanged.
- `proof fn masked_irq_not_deliverable` proves masked lines are never deliverable.
- `proof fn acknowledge_consumes_pending` proves `acknowledge` clears pending state and therefore clears immediate deliverability.
- Unit tests cover second-bind rejection, mask blocking, acknowledge consumption, and unmask restoring delivery on a bound pending line.
- Proptests cover first-bind/second-bind behavior, masked pending non-delivery, and acknowledge clearing pending state.

## Risks / Trade-offs

**[Edge vs level semantics could sprawl]** State only the abstract pending/ack contract needed by seL4 primitives.
**[Too much coupling to notifications]** Leave wakeup composition for a later cross-module theorem.
**[Per-line model is narrower than a global IRQ table]** Keep v1 local to one IRQ line so the uniqueness and deliverability rules stay simple and mechanically checked.
