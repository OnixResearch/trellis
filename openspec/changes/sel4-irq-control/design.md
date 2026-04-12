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

## Risks / Trade-offs

**[Edge vs level semantics could sprawl]** State only the abstract pending/ack contract needed by seL4 primitives.
**[Too much coupling to notifications]** Leave wakeup composition for a later cross-module theorem.
