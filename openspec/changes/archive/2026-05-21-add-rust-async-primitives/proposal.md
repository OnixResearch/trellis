## Why

Trellis already has strong verified coverage for blocking concurrency, job-runner state, memory ordering, and finite protocol queues, but it lacks Rust-native async primitives that model the correctness edges most likely to cause production bugs: lost wakes, cancellation races, send-once delivery, and RAII cleanup. Adding a small async primitive slice gives later runtimes and durable executors reusable verified building blocks without claiming a full executor implementation.

## What Changes

- **Async task state**: Add a finite task/future lifecycle model covering pending, ready, completed, cancelled, and invalid post-terminal transitions.
- **Waker registration**: Add a waker-slot primitive for register/replace/wake/take behavior with explicit lost-wake prevention.
- **Oneshot channel**: Add send-once/receive-once/close semantics that compose task readiness with one-value delivery.
- **Drop guard**: Add a Rust RAII cleanup primitive that proves cleanup is admitted exactly once across normal drop and explicit disarm paths.

## Capabilities

### New Capabilities

- `rust-primitives.async-task-state`: Finite async task lifecycle predicates and runtime helpers.
- `rust-primitives.task-waker`: Waker-slot admission, replacement, wake idempotence, and no-lost-wake bridge proofs.
- `rust-primitives.oneshot-channel`: Send-once channel state, receive readiness, close, and cancellation behavior.
- `rust-primitives.drop-guard`: Exactly-once cleanup/disarm guard model.

### Modified Capabilities

- `documentation-catalog`: README/module catalog entries must include the new Rust primitive modules once implemented.
- `verification-tooling`: Public functions in the new modules must carry `r[impl]`/`r[verify]` coverage and pass the existing repo-wide gates.

## Impact

- **Files**: Expected implementation under `src/async_task_state.rs`, `src/task_waker.rs`, `src/oneshot_channel.rs`, `src/drop_guard.rs`, `src/lib.rs`, `README.md`, `docs/requirements.md`, and the new/archived OpenSpec domain.
- **APIs**: New small public pure helpers and proof functions; no full async runtime, executor, OS parking, or unsafe pin projection API in this change.
- **Dependencies**: No new external dependencies expected.
- **Testing**: Focused positive/negative unit tests per module, Verus proofs for transition/refinement properties, Tracey coverage checks, strict OpenSpec validation, and final `./scripts/verify.rs` evidence.
