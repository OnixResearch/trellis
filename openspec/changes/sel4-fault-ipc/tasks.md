## Phase 1: Core model and proofs

- [ ] Create `src/sel4_fault_ipc.rs` with structured fault records and encode/decode helpers.
- [ ] Prove encode/decode preserves fault class and payload fields.
- [ ] Model handler delivery and prove the handler sees the original fault metadata.
- [ ] Add unit tests and proptests for page-fault and generic-fault roundtrips.
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean.
