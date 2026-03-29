## Why

The library has job runner primitives covering lifecycle state machines
(`job_state`), slot allocation (`job_slot`), priority ordering
(`job_priority`), idempotency tokens (`job_idempotency`), cancellation
propagation (`job_cancel`), and DAG dependencies (`job_dag`). These
handle *scheduling* — deciding when and whether to run a job.

The gap is *execution verification* — the decision logic that runs
during and after a job executes. Every production job runner needs:

1. **Retry decisions**: should a failed job be retried? How many
   attempts remain? Is the error type retryable? Without verified
   retry logic, systems either retry forever (resource exhaustion)
   or never retry (transient failures become permanent).

2. **Timeout enforcement**: has a running job exceeded its deadline?
   How much time remains? Can the deadline be extended? Without
   verified timeout logic, jobs hang indefinitely or get killed
   prematurely during legitimate long operations.

3. **Result validation**: did the job produce an acceptable result?
   Does the output hash match the expected value? Is the exit code
   in the success range? Without verified result logic, the system
   accepts corrupted outputs or rejects valid ones.

These are pure decision functions — no I/O, no timers, no process
management. The imperative executor calls them to make decisions.
The existing `retry` module handles generic should_retry with
attempt counting; the existing `expiration` module handles generic
TTL checks. Neither models job-specific concerns: retryable vs
non-retryable error classification, deadline extension with grace
periods, or multi-criteria result validation.

## What Changes

Three new modules covering job execution verification logic:

- **`job_retry`** — Retry eligibility and attempt tracking. Given an
  error code, max attempts, and attempt count, decides whether to
  retry. Classifies errors as retryable (transient) vs non-retryable
  (permanent). Tracks cooldown periods between attempts. Proofs that
  attempts decrease monotonically, non-retryable errors are never
  retried, and max attempts is never exceeded.

- **`job_timeout`** — Deadline enforcement for running jobs. Computes
  deadlines from start time + timeout duration. Checks if a job has
  timed out. Computes remaining time. Supports deadline extension with
  a bounded grace period. Proofs that timeouts are monotonic (once
  timed out, stays timed out), extension only pushes forward, and
  remaining time is bounded by the original timeout.

- **`job_result`** — Result classification and validation. Classifies
  exit codes into success/failure/error ranges. Validates output
  against an expected hash. Combines multiple validation criteria
  (exit code + output hash + size bounds). Proofs that success and
  failure are mutually exclusive, validation is deterministic, and
  partial validation failures identify which criterion failed.

## Capabilities

### New Capabilities
- `job-retry-decision`: Verified retry eligibility with error classification
- `job-timeout-enforcement`: Verified deadline computation and extension
- `job-result-validation`: Verified multi-criteria result checking

### Modified Capabilities
- `cross-module-properties`: Properties linking job_retry to job_state
  (Failed→retry→Ready), job_timeout to job_state (timeout→Failed),
  job_result to job_state (success→Completed, failure→Failed)

## Impact

- **Files**: 3 new src/ modules, src/lib.rs additions, properties.rs additions
- **APIs**: ~25 new pub fns across 3 modules
- **Dependencies**: None — pure logic using existing vstd
- **Testing**: Verus verification + proptest property tests
