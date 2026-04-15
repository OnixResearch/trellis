## Phase 1: Core model and proofs

- [x] Create `src/sel4_tcb.rs` with legal states and blocking-reason variants, export it from `src/lib.rs`, add `r[sel4-tcb.state]`, `r[sel4-tcb.blocking-object]`, and `r[sel4-tcb.transitions]` to `docs/requirements.md`, and tag the defining spec/exec items with `r[impl]`.
- [x] Add real `proof fn` artifacts showing every blocked-state constructor carries the object responsible for the block.
- [x] Prove legal transitions for block, wake, restart, and deactivate operations with a named `legal_transition_spec` contract and matching exec postconditions.
- [x] Add unit tests and proptests for state-transition scenarios; capture module evidence in `evidence/cargo-test-sel4-tcb.txt` and `evidence/verus-sel4-tcb.txt`.
- [x] Sync `openspec/specs/sel4-tcb/spec.md` from the change-local spec.
- [x] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean; captured repo-level evidence under `evidence/`.
