## Why

IRQ handler binding and masked delivery are compact, high-value kernel rules. A separate change gives the repo a reusable interrupt-control model without trying to verify hardware controller internals.

## What Changes

- Add IRQ handler binding and release semantics.
- Model pending, masked, and acknowledged IRQ states.
- Prove unique binding and mask-aware delivery eligibility.

## Capabilities

### New Capabilities

- `sel4-irq.binding`: Verified unique live handler binding per IRQ
- `sel4-irq.masking`: Verified masked IRQs are not deliverable
- `sel4-irq.ack`: Verified acknowledgement consumes pending delivery state

## Impact

- **Files**: New `src/sel4_irq.rs` module, `src/lib.rs` export, and this OpenSpec change
- **APIs**: Public spec/exec/proof functions for interrupt delivery control
- **Dependencies**: Standalone for now; later changes may compose it with `sel4_notification` / `sel4_tcb`
- **Testing**: Verus proofs, unit tests, and proptests for bind/mask/unmask/ack cases

## Out of Scope

- Hardware-specific interrupt controller details
- Notification object semantics themselves
- Full scheduler response to interrupts
