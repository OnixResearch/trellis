# Job Runner Primitives Specification

## Purpose

Defines correctness properties for job runner decision logic: job lifecycle
state machines, concurrency slot allocation, cancellation propagation,
priority ordering, and idempotency tokens. These are pure functions with
no I/O — the imperative executor lives elsewhere.

## Requirements

### Job State Machine

#### Requirement: Valid States

A job state MUST be one of: Pending (0), Ready (1), Running (2),
Completed (3), Failed (4), Cancelled (5). Any other value MUST be
rejected by `is_valid_state`.

##### Scenario: All valid states

- GIVEN state in {0, 1, 2, 3, 4, 5}
- WHEN checking is_valid_state
- THEN MUST return true

##### Scenario: Invalid state

- GIVEN state = 6
- WHEN checking is_valid_state
- THEN MUST return false

#### Requirement: Valid Transitions

The job state machine MUST permit only these transitions:
Pending→Ready, Ready→Running, Running→Completed, Running→Failed,
Pending→Cancelled, Ready→Cancelled, Running→Cancelled. All other
transitions MUST be rejected.

##### Scenario: Pending to Ready

- GIVEN state=Pending
- WHEN transitioning to Ready
- THEN MUST be valid

##### Scenario: Running to Completed

- GIVEN state=Running
- WHEN transitioning to Completed
- THEN MUST be valid

##### Scenario: Completed to Running (invalid)

- GIVEN state=Completed
- WHEN transitioning to Running
- THEN MUST be invalid

##### Scenario: Failed to Ready (invalid)

- GIVEN state=Failed
- WHEN transitioning to Ready
- THEN MUST be invalid

#### Requirement: Terminal States

Completed, Failed, and Cancelled MUST be terminal. No transition from
any terminal state to any other state is valid.

##### Scenario: Completed is terminal

- GIVEN state=Completed
- WHEN transitioning to any state
- THEN MUST be invalid

##### Scenario: Failed is terminal

- GIVEN state=Failed
- WHEN transitioning to any state
- THEN MUST be invalid

##### Scenario: Cancelled is terminal

- GIVEN state=Cancelled
- WHEN transitioning to any state
- THEN MUST be invalid

#### Requirement: Cancellation from Non-Terminal

Any non-terminal state (Pending, Ready, Running) MUST permit transition
to Cancelled. This is the only transition available from all non-terminal
states.

##### Scenario: Pending cancellation

- GIVEN state=Pending
- WHEN transitioning to Cancelled
- THEN MUST be valid

##### Scenario: Running cancellation

- GIVEN state=Running
- WHEN transitioning to Cancelled
- THEN MUST be valid

#### Requirement: DAG-Aware Readiness

`can_become_ready` MUST return true only when all dependencies of the
job are in the Completed state. A job with a Failed or Cancelled
dependency MUST NOT become Ready.

##### Scenario: All deps completed

- GIVEN dep_states = [Completed, Completed, Completed]
- WHEN checking can_become_ready
- THEN MUST return true

##### Scenario: One dep still running

- GIVEN dep_states = [Completed, Running, Completed]
- WHEN checking can_become_ready
- THEN MUST return false

##### Scenario: One dep failed

- GIVEN dep_states = [Completed, Failed]
- WHEN checking can_become_ready
- THEN MUST return false

##### Scenario: No dependencies

- GIVEN dep_states = []
- WHEN checking can_become_ready
- THEN MUST return true

### Job Slot Allocation

#### Requirement: Acquire Within Capacity

`try_acquire_slot` MUST return true when `in_use < capacity`.
After a successful acquire, `in_use` MUST increase by 1.

##### Scenario: Slots available

- GIVEN in_use=3, capacity=8
- WHEN acquiring a slot
- THEN MUST return true, in_use becomes 4

##### Scenario: Empty pool

- GIVEN in_use=0, capacity=8
- WHEN acquiring a slot
- THEN MUST return true, in_use becomes 1

#### Requirement: Acquire at Capacity Fails

`try_acquire_slot` MUST return false when `in_use >= capacity`.
The `in_use` count MUST NOT change on failure.

##### Scenario: At capacity

- GIVEN in_use=8, capacity=8
- WHEN acquiring a slot
- THEN MUST return false, in_use stays 8

#### Requirement: Release Never Underflows

`release_slot` MUST decrement `in_use` by 1 when `in_use > 0`.
If `in_use == 0`, `release_slot` MUST be a no-op (saturating).
The count MUST never go negative.

##### Scenario: Normal release

- GIVEN in_use=3
- WHEN releasing a slot
- THEN in_use becomes 2

##### Scenario: Release at zero

- GIVEN in_use=0
- WHEN releasing a slot
- THEN in_use stays 0

#### Requirement: Slot Count Bounded

The `in_use` count MUST always satisfy `0 <= in_use <= capacity`.
This invariant MUST hold after any sequence of acquire and release
operations.

##### Scenario: Invariant after acquire

- GIVEN in_use < capacity
- WHEN acquiring a slot
- THEN 0 <= new_in_use <= capacity

##### Scenario: Invariant after release

- GIVEN in_use > 0
- WHEN releasing a slot
- THEN 0 <= new_in_use <= capacity

#### Requirement: Has Available Predicate

`has_available_slot` MUST return true iff `in_use < capacity`.
This is equivalent to `try_acquire_slot` succeeding, without
performing the acquisition.

##### Scenario: Slots free

- GIVEN in_use=5, capacity=8
- WHEN checking has_available_slot
- THEN MUST return true

##### Scenario: Full

- GIVEN in_use=8, capacity=8
- WHEN checking has_available_slot
- THEN MUST return false

### Job Cancellation Propagation

#### Requirement: Direct Dependents Cancelled

When a job is cancelled, every job that lists the cancelled job as a
dependency and has no other satisfied (Completed) path MUST be marked
for cancellation.

##### Scenario: Single dependent

- GIVEN job A cancelled, job B depends only on A
- WHEN propagating cancellation
- THEN B MUST be in the cancellation set

##### Scenario: Dependent with alternative

- GIVEN job A cancelled, job B depends on A and C, C is Completed
- WHEN propagating cancellation
- THEN B MUST NOT be in the cancellation set (C satisfies it)

#### Requirement: Cascade Through Chain

Cancellation MUST propagate transitively. If A is cancelled, B depends
on A and gets cancelled, and C depends on B, then C MUST also be
cancelled (assuming no alternative paths).

##### Scenario: Chain A→B→C

- GIVEN A cancelled, B depends only on A, C depends only on B
- WHEN propagating cancellation from A
- THEN both B and C MUST be in the cancellation set

#### Requirement: Already Terminal Unaffected

Jobs that are already Completed, Failed, or Cancelled MUST NOT appear
in the cancellation propagation set. Only Pending, Ready, or Running
jobs are candidates.

##### Scenario: Completed dependent

- GIVEN job A cancelled, job B depends on A, B is already Completed
- WHEN propagating cancellation
- THEN B MUST NOT be in the cancellation set

#### Requirement: Cascade Depth Bounded

The propagation MUST accept a `max_depth` parameter. Propagation MUST
NOT recurse beyond `max_depth` levels from the originally cancelled job.
This prevents unbounded recursion in deep DAGs.

##### Scenario: Depth limit reached

- GIVEN chain A→B→C→D, A cancelled, max_depth=2
- WHEN propagating cancellation
- THEN B and C MUST be cancelled, D MUST NOT (depth 3)

##### Scenario: Depth zero

- GIVEN A cancelled, B depends on A, max_depth=0
- WHEN propagating cancellation
- THEN cancellation set MUST be empty (no propagation)

### Job Priority Ordering

#### Requirement: Total Order

`is_higher_priority` MUST define a total order on
(priority_class, sequence_number) pairs. For any two distinct pairs,
exactly one MUST be higher priority than the other.

##### Scenario: Different priority class

- GIVEN job A with class=2, job B with class=1
- WHEN comparing priority
- THEN A MUST be higher priority (higher class wins)

##### Scenario: Same class, different sequence

- GIVEN job A with class=1 seq=5, job B with class=1 seq=10
- WHEN comparing priority
- THEN A MUST be higher priority (lower sequence = older = first)

#### Requirement: Transitivity

If job A has higher priority than B, and B has higher priority than C,
then A MUST have higher priority than C.

##### Scenario: Transitive chain

- GIVEN A=(class=3, seq=1), B=(class=2, seq=1), C=(class=1, seq=1)
- WHEN A > B and B > C
- THEN A > C MUST hold

#### Requirement: Antisymmetry

If A has higher priority than B, then B MUST NOT have higher priority
than A.

##### Scenario: Antisymmetric

- GIVEN A=(class=2, seq=1), B=(class=1, seq=5)
- WHEN A is higher priority than B
- THEN B MUST NOT be higher priority than A

#### Requirement: Equal Priority

Two jobs with identical (priority_class, sequence_number) MUST be
considered equal priority. `is_higher_priority` MUST return false
in both directions.

##### Scenario: Equal pairs

- GIVEN A=(class=1, seq=5), B=(class=1, seq=5)
- WHEN comparing
- THEN is_higher_priority(A, B) MUST be false
- AND is_higher_priority(B, A) MUST be false

#### Requirement: Select Highest from Set

`select_highest_priority` over a non-empty sequence of
(priority_class, sequence_number) pairs MUST return the index of the
pair with the highest priority. The result MUST be a valid index.

##### Scenario: Mixed priorities

- GIVEN jobs [(1,10), (3,5), (2,1)]
- WHEN selecting highest
- THEN index 1 MUST be returned (class 3 wins)

### Job Idempotency

#### Requirement: Fresh Token Not Consumed

A token that has never been recorded in the consumed set MUST be
reported as not consumed by `is_consumed`.

##### Scenario: New token

- GIVEN consumed_set = [100, 200, 300], token = 400
- WHEN checking is_consumed
- THEN MUST return false

#### Requirement: Consumed Token Stays Consumed

Once a token is added to the consumed set, `is_consumed` MUST return
true for that token. Adding more tokens MUST NOT affect the consumed
status of previously added tokens (monotonicity).

##### Scenario: After recording

- GIVEN token 42 added to consumed set
- WHEN checking is_consumed(42)
- THEN MUST return true

##### Scenario: Monotonicity

- GIVEN consumed_set contains token 42
- WHEN token 99 is added to consumed_set
- THEN is_consumed(42) MUST still return true

#### Requirement: Try Execute Once

`try_execute` takes a token and the consumed set. If the token is not
consumed, it MUST return true (proceed) and the token MUST be added to
the consumed set. If already consumed, it MUST return false (skip) and
the set MUST NOT change.

##### Scenario: First execution

- GIVEN consumed_set = [], token = 1
- WHEN try_execute is called
- THEN MUST return true, consumed_set becomes [1]

##### Scenario: Duplicate execution

- GIVEN consumed_set = [1], token = 1
- WHEN try_execute is called
- THEN MUST return false, consumed_set stays [1]

#### Requirement: Consumed Count Monotonic

The size of the consumed set MUST never decrease. After `try_execute`
with a fresh token, the size MUST increase by exactly 1. After
`try_execute` with an already-consumed token, the size MUST stay the
same.

##### Scenario: Size after fresh

- GIVEN consumed_set has 5 entries, token is fresh
- WHEN try_execute succeeds
- THEN consumed_set has 6 entries

##### Scenario: Size after duplicate

- GIVEN consumed_set has 5 entries, token is consumed
- WHEN try_execute is skipped
- THEN consumed_set has 5 entries
