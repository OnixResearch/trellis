## Phase 1: Core model and proofs

- [ ] Create `src/sel4_invocation.rs` with abstract message input and typed invocation outputs.
- [ ] Prove decode accepts only type-correct, rights-authorized requests.
- [ ] Prove wrong-type, malformed, and under-authorized requests are rejected.
- [ ] Prove decode never invents authority beyond the supplied capability context.
- [ ] Add unit tests and proptests for decode success/failure cases.
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean.
