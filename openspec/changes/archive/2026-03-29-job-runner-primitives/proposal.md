## Why

The library has `job_dag` covering dependency satisfaction, unsatisfied
counts, and topological levels. But a job runner needs more than a DAG
checker — it needs a state machine for individual jobs, slot allocation
to bound concurrency, cancellation propagation through the dependency
graph, priority comparison for scheduling order, and idempotency tokens
to prevent duplicate execution.

These are the pure decision functions that sit between the DAG structure
(`job_dag`) and the imperative executor. Every real job runner —
CI pipelines, build systems, workflow engines — has this layer, and
bugs here cause double-execution, orphaned jobs, priority inversion,
and slot leaks. The logic is small, pure, and benefits from machine-checked
proofs.

The existing modules cover adjacent ground: `process_state` models Unix
process lifecycle (Created→Ready→Running→Blocked→Terminated), `retry`
decides whether to retry, `fair_share` does weighted scheduling,
`deadline_queue` handles EDF ordering. None of these model the specific
job runner concerns: a job that transitions through
Pending→Ready→Running→Completed/Failed/Cancelled with DAG-aware
readiness, bounded execution slots, and cancellation that cascades to
dependents.

## What Changes

Five new modules covering job runner decision logic. Each is
self-contained with spec fns, exec fns, proofs, and property tests.

- **`job_state`** — Job lifecycle state machine:
  Pending→Ready→Running→Completed/Failed/Cancelled. Transition guards
  encode DAG readiness (Pending→Ready only when all deps satisfied),
  terminal states (Completed/Failed/Cancelled have no successors), and
  cancellation from any non-terminal state.

- **`job_slot`** — Concurrency slot allocation: acquire/release with a
  fixed capacity. Proofs that acquire fails at capacity, release never
  underflows, and the in-use count stays within [0, capacity]. Slot
  occupancy predicate for scheduling decisions.

- **`job_cancel`** — Cancellation propagation through a dependency graph.
  Given a cancelled job and the set of jobs that depend on it, determines
  which dependents should also be cancelled (those with no alternative
  path to completion). Cascade depth bounding.

- **`job_priority`** — Priority comparison for scheduling. Total order
  on (priority_class, sequence_number) pairs. Higher priority class wins;
  within the same class, lower sequence number (older job) wins. Proofs
  of transitivity, antisymmetry, and totality.

- **`job_idempotency`** — Idempotency token matching: a job execution
  request carries a token, and the system checks whether that token has
  already been consumed. Proofs that consumed tokens stay consumed
  (monotonicity) and that a fresh token is never marked consumed.

## Capabilities

### New Capabilities
- `job-state-machine`: Verified job lifecycle with DAG-aware readiness
- `job-slot-allocation`: Verified bounded concurrency slots
- `job-cancel-propagation`: Verified cascading cancellation
- `job-priority-order`: Verified total ordering for scheduling
- `job-idempotency-token`: Verified duplicate execution prevention

### Modified Capabilities
- `cross-module-properties`: Properties linking job_state to job_dag
  (readiness predicate), job_slot to job_state (Running requires slot),
  job_cancel to job_state (cancelled is terminal)

## Impact

- **Files**: 5 new src/ modules, src/lib.rs additions, tracey annotations
- **APIs**: ~30 new pub fns across 5 modules
- **Dependencies**: None — pure logic using existing vstd
- **Testing**: Verus verification + proptest property tests
