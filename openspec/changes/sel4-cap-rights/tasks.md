## Phase 1: Core model and proofs

- [ ] Create `src/sel4_cap_rights.rs` with subset, attenuation, mask, and operation-gating predicates.
- [ ] Prove derived rights never exceed parent rights and masking is idempotent and monotone.
- [ ] Add unit tests and proptests for send/receive/map/control gate checks.
- [ ] Add Tracey annotations and export the module from `src/lib.rs` when implementation lands.
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean.
