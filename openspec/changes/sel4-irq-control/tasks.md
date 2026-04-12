## Phase 1: Core model and proofs

- [ ] Create `src/sel4_irq.rs` with binding, mask/unmask, ack, and deliverability predicates.
- [ ] Prove each IRQ has at most one live binding and masked IRQs are not deliverable.
- [ ] Prove acknowledgement consumes pending delivery state under the abstract IRQ model.
- [ ] Add unit tests and proptests for bind/mask/unmask/ack scenarios.
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean.
