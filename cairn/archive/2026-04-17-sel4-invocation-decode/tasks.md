## Phase 1: Core model and proofs

- [x] Create `src/sel4_invocation.rs` with `InvocationMessage`, `Capability`, `DecodeContext`, and a typed `Invocation` enum for exactly `EndpointSend`, `EndpointReceive`, `FrameMap`, and `UntypedRetype`; export it from `src/lib.rs`; tag the public defining helpers and exec surface with `r[impl sel4-invocation.*]`; keep helper-only routines private.
- [x] Prove successful decode returns only type-correct, rights-authorized starter-set requests, using both the exact `decode`/`decode_spec` postcondition and named `proof fn` artifacts.
- [x] Prove malformed messages, wrong-type capabilities, missing required secondary capabilities, and under-authorized requests are rejected, using a named `proof fn` artifact over the exact `decode_spec` rejection surface.
- [x] Prove successful decode never invents authority beyond the supplied capability context, using the structural containment rule from `design.md`, the exact `decode`/`decode_spec` postcondition, and a named `proof fn` artifact.
- [x] Add exact `docs/requirements.md` entries for `r[sel4-invocation.decode-soundness]`, `r[sel4-invocation.reject]`, and `r[sel4-invocation.no-invented-authority]`, plus unit tests and proptests for decode success/failure cases.
- [x] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean.
