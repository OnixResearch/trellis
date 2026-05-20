## Phase 1: Spec and Skeleton

- [x] [serial] Add docs/requirements anchors and OpenSpec requirement coverage for `r[rust-primitives.async-task-state]`, `r[rust-primitives.task-waker]`, `r[rust-primitives.no-lost-wake]`, `r[rust-primitives.oneshot-channel]`, `r[rust-primitives.oneshot-readiness]`, `r[rust-primitives.drop-guard]`, and `r[rust-primitives.verification-coverage]`.
- [x] [depends:rust-primitives.verification-coverage] Add module skeletons, `src/lib.rs` exports, README catalog entries, and no-op compile coverage for `async_task_state`, `task_waker`, `oneshot_channel`, and `drop_guard`.

## Phase 2: Async State and Waker Core

- [x] [depends:rust-primitives.async-task-state] Implement finite async task-state spec predicates, executable admission helpers, proof bridges, and positive/negative tests for ready/completed/cancelled transitions. ✅ 6m 22s (started: 2026-05-20T03:15:37Z → completed: 2026-05-20T03:21:59Z; evidence: `nix develop -c cargo test --quiet async_task_state`, `nix develop -c cargo clippy --all-targets -- -D warnings`, `nix develop -c ./scripts/verus.sh src/lib.rs`, `nix develop -c rustfmt --check src/async_task_state.rs`, `openspec validate add-rust-async-primitives --strict --json`)
- [x] [depends:rust-primitives.task-waker] Implement finite waker-slot spec predicates, executable helpers, proof bridges, and positive/negative tests for register/replace/wake/take/idempotent wake behavior. ✅ 8m (started: 2026-05-20T03:22:41Z → completed: 2026-05-20T03:30:41Z; evidence: `nix develop -c cargo test --quiet task_waker`, `nix develop -c cargo clippy --all-targets -- -D warnings`, `nix develop -c ./scripts/verus.sh src/lib.rs`, `nix develop -c rustfmt --check src/task_waker.rs`, `openspec validate add-rust-async-primitives --strict --json`)
- [ ] [depends:rust-primitives.no-lost-wake] Implement no-lost-wake bridge predicates/proofs between task readiness and waker-slot state without claiming callback execution or scheduler fairness.

## Phase 3: Oneshot and Drop Guard

- [ ] [depends:rust-primitives.oneshot-channel] Implement single-slot oneshot channel state, send/receive/close/cancel helpers, proof bridges, and tests for send-once, receive-once, closed-send rejection, and cancellation behavior.
- [ ] [depends:rust-primitives.oneshot-readiness] Compose successful oneshot sends with async task readiness and waker notification; add negative tests proving rejected sends do not create readiness evidence.
- [ ] [depends:rust-primitives.drop-guard] Implement armed/disarmed/cleaned drop-guard state helpers, exactly-once cleanup proofs, and positive/negative tests for cleanup, disarm, and repeated cleanup attempts.

## Phase 4: Cross-Module and Documentation

- [ ] [depends:phase-2-3] Add focused cross-module properties for task-state ↔ waker and oneshot ↔ waker readiness composition, keeping any broader async-runtime properties out of scope.
- [ ] [depends:rust-primitives.verification-coverage] Update README/module catalog and generated requirement/docs references so public claims match implemented proof/test coverage.

## Phase 5: Validation and Evidence

- [ ] [serial] Run and capture focused module tests, `cargo test --quiet` or the smallest relevant test rail, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested checks, and `openspec validate add-rust-async-primitives --strict --json`.
- [ ] [serial] Run final `./scripts/verify.rs`, reconcile README metrics/catalog drift, save validation transcripts under the change evidence directory, then sync/archive only after all tasks are complete.
