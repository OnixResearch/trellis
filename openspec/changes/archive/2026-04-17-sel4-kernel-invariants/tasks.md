## Phase 1: Core model and proofs

- [x] Create `src/sel4_invariants.rs` with `KernelState { tcbs, ready_queues, endpoints, notification_count, reply_count, cdt, cspace, untypeds, asid_pool, vspaces, irqs }`, export it from `src/lib.rs`, tag the public defining helpers with `r[impl sel4-invariants.*]`, and keep helper-only routines private.
- [x] Add named sub-invariant spec fns `object_refs_valid_spec`, `cdt_wf_spec`, `cspace_valid_spec`, `tcb_legal_spec`, `scheduler_consistent_spec`, `untyped_ownership_spec`, `asid_pool_wf_spec`, `vspace_consistent_spec`, and `irq_unique_spec`, plus the composed `kernel_invariant_spec` conjunction.
- [x] Prove the global invariant decomposes into the named local obligations using explicit public `proof fn` artifacts.
- [x] Prove the initial preservation-step set with explicit exec wrappers and named `proof fn` artifacts: `step_endpoint_send_block`, `step_endpoint_send_rendezvous`, and `step_untyped_revoke`.
- [x] Add exact `docs/requirements.md` entries for `r[sel4-invariants.compose]`, `r[sel4-invariants.decomposition]`, and `r[sel4-invariants.step-preservation]`, plus unit tests or model checks for the initial preservation-step set.
- [x] Add `src/properties.rs` helpers only if proof factoring requires them; otherwise keep the invariant story local to `src/sel4_invariants.rs` and record that choice in code comments.
- [x] Run `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, and `./scripts/verify.sh`.
- [x] Confirm Tracey reports `0 uncovered` and `0 untested` for the finished change.
