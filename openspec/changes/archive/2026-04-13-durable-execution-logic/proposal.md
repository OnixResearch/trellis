## Why

The library already covers scheduling and post-run decision logic:
`job_state`, `job_slot`, `job_idempotency`, `job_retry`,
`job_timeout`, `job_result`, plus lower-level durability building
blocks like `persistent_state` and `journal`. Missing piece is
*durable execution logic* for crash-recoverable workflows and
state machines.

A durable executor does more than decide whether a job may run. It must
reconstruct progress from durable history, replay deterministic steps,
suppress duplicate side effects, advance checkpoints only when durable
history is contiguous, and refuse to resurrect terminal executions after
recovery. Bugs here cause classic failure modes: duplicated side effects,
skipped steps after crash, replay reading beyond persisted history, and
completed runs reopening after restart.

This change scopes that logic as pure functions only. No storage engine,
network I/O, timers, or actual effect execution. The imperative runtime
calls these functions to decide what it may append, replay, resume, and
execute.

## What Changes

- **`durable_history`** — append-only execution history rules: event
  sequence monotonicity, contiguous durable prefix, and checkpoint
  advancement only when prior steps are durable.
- **`durable_replay`** — replay decisions over durable history:
  determine replayable prefix, reject reads past the durable boundary,
  and distinguish "replay recorded result" from "execute now".
- **`durable_resume`** — crash recovery cursor logic: derive the next
  step to run from the durable checkpoint and history, bounded so resume
  never skips gaps and never re-enters already durable work.
- **`durable_terminal`** — terminal-run semantics: once a terminal event
  is durable, the execution stays terminal and no new commands may be
  scheduled for that run.

## Capabilities

### New Capabilities

- `durable-execution.history-prefix`: Verified append-only durable history
- `durable-execution.checkpoint`: Verified monotonic checkpoint advance
- `durable-execution.replay-gate`: Verified replay-vs-execute decision logic
- `durable-execution.resume-point`: Verified crash recovery resume cursor
- `durable-execution.terminality`: Verified durable terminal-run fence

## Impact

- **Files**: new modules under `src/` plus `src/lib.rs` and likely
  `src/properties.rs` updates for cross-module properties
- **APIs**: public spec/exec fns for history validity, checkpoint
  advance, replay gating, resume-point derivation, and terminal checks
- **Dependencies**: none beyond existing Verus/vstd patterns; composes
  with `persistent_state`, `journal`, `job_idempotency`, and job runner logic
- **Testing**: Verus proofs for prefix/terminality properties and
  proptest or unit tests for random histories, replay decisions, and
  crash-recovery scenarios
