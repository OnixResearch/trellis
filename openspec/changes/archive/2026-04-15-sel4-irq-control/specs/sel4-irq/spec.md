# seL4 IRQ Control Specification

## Purpose

Defines requirements for verified `sel4_irq` semantics in this repo. Scope is interrupt delivery control behavior and preserved invariants, not raw ABI details.

## Requirements

### Requirement: IRQ Binding Is Unique

r[sel4-irq.binding]
Each IRQ line MUST have at most one live handler binding at a time.

#### Scenario: Second live bind rejected

- GIVEN IRQ `I` already bound to handler `H1`
- WHEN binding `I` to `H2` is attempted without releasing `H1`
- THEN the bind is invalid

### Requirement: Masked IRQs Are Not Deliverable

r[sel4-irq.masking]
An IRQ that is masked MUST NOT be deliverable even if it is pending.

#### Scenario: Mask blocks delivery

- GIVEN IRQ `I` is pending and masked
- WHEN delivery is checked
- THEN `I` is not deliverable

### Requirement: IRQ Acknowledge Consumes Pending State

r[sel4-irq.ack]
Acknowledgement MUST clear or consume the abstract pending delivery state for the IRQ being acknowledged.

#### Scenario: Ack clears pending delivery

- GIVEN an unmasked pending IRQ with a live binding
- WHEN it is acknowledged
- THEN the IRQ is no longer pending for immediate delivery
