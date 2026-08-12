## Phase 1: Core model and proofs

- [x] Create `src/sel4_ready_queue.rs` with per-priority queues and iterative selection/remove logic, export it from `src/lib.rs`, add `r[sel4-ready-queue.consistency]`, `r[sel4-ready-queue.selection]`, and `r[sel4-ready-queue.order]` to `docs/requirements.md`, and tag the defining spec/exec items with `r[impl]`.
- [x] Add real `proof fn` artifacts for ready-queue consistency: every runnable TCB appears in exactly one queue at its priority, and every queued TCB is runnable.
- [x] Prove `select_highest` returns a TCB from a highest-priority non-empty queue and prove enqueue/dequeue/remove preserve unaffected order within one priority.
- [x] Add unit tests and proptests for enqueue/dequeue/remove/priority cases; captured targeted module evidence in `evidence/cargo-test-sel4-ready-queue.txt` and `evidence/verus-sel4-ready-queue.txt`.
- [x] Sync `openspec/specs/sel4-ready-queue/spec.md` from the change-local spec.
- [x] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean; captured repo-level evidence under `evidence/`.
