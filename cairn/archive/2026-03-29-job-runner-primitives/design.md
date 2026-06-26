## Context

verified-logic has `job_dag` with three operations: `all_deps_satisfied`
(readiness check against a completed set), `unsatisfied_count` (count
pending deps), and `topo_level` (longest path from source). These work
on raw `u64` job IDs and `Seq<u64>` dependency lists.

Related existing modules: `process_state` (Unix process FSM, 5 states),
`retry` (should_retry predicate), `fair_share` (weighted scheduling),
`deadline_queue` (EDF ordering), `work_stealing` (Chase-Lev bounds),
`batch` (flush decisions), `circuit_breaker` (three-state breaker).

The gap: no job-specific state machine (the process states don't match
job lifecycle), no bounded slot pool, no cancellation propagation logic,
no priority comparator for scheduling, no idempotency guard.

## Goals / Non-Goals

**Goals:**
- Verified job state machine with 6 states and guarded transitions
- Verified slot pool with capacity invariant and acquire/release
- Verified cancellation propagation with depth bounding
- Verified total priority order with transitivity/antisymmetry proofs
- Verified idempotency token with monotonicity proof
- All modules follow existing pattern: spec fn + exec fn + proof fn + tests
- Cross-module link: job_state readiness ties to job_dag.all_deps_satisfied

**Non-Goals:**
- Async job executor or scheduler runtime
- Persistent job queue (disk, database)
- Distributed job coordination (locking, leases)
- Time-based scheduling (cron, delays) — covered by deadline_queue
- Retry logic — covered by existing retry module
- Work stealing — covered by existing work_stealing module

## Decisions

### 1. Job states as u8 constants, not enum

**Choice:** `pub const PENDING: u8 = 0; pub const READY: u8 = 1;` etc.
Same encoding pattern as `process_state`, `node_lifecycle`, `trust_level`.

**Rationale:** Verus handles integer constants and match-on-integer well.
The library already has this convention everywhere. Rust enums inside
`verus!` blocks work but add complexity for no verification benefit.

**Alternative:** Rust enum with `#[repr(u8)]`. Rejected — every other
state machine in the library uses u8 constants.

### 2. DAG-aware readiness as a separate predicate, not embedded in transition

**Choice:** `can_become_ready(dep_states: &[u8]) -> bool` is a standalone
function. The transition function `is_valid_transition(from, to)` only
checks the state graph edges. The caller composes them: transition from
Pending to Ready is valid AND `can_become_ready` is true.

**Rationale:** Keeps the state machine pure (no dependency data needed).
The composition proof shows that if `is_valid_transition(PENDING, READY)`
and `can_become_ready(deps)`, then the transition is safe. This matches
how real executors work — they check readiness separately from state
validity.

**Alternative:** Embed deps into the transition function signature.
Rejected — makes the state machine depend on external data structures
and breaks the pattern used by process_state and node_lifecycle.

### 3. Slot pool as pure arithmetic on (in_use, capacity)

**Choice:** `try_acquire_slot(in_use, capacity) -> (bool, u64)` returns
(success, new_in_use). `release_slot(in_use) -> u64` returns new_in_use.
No data structure — just the count.

**Rationale:** The interesting property is the capacity invariant
(`0 <= in_use <= capacity`), not which specific slots are allocated.
Real implementations use atomics or semaphores; the decision logic is
"is there room?" and "did we go negative?". Modeling the count alone
lets Verus prove the bounds without modeling a concurrent data structure.

**Alternative:** Model a set/bitvector of slot IDs. Rejected — adds
complexity without interesting new properties. The count-based approach
matches how semaphores work internally.

### 4. Cancellation propagation as iterative marking with depth bound

**Choice:** `propagate_cancel(cancelled_job, job_deps, job_states, max_depth)`
takes the dependency graph as parallel arrays: `job_deps[i]` is the
dependency list for job `i`, `job_states[i]` is the current state. Returns
a `Seq<bool>` marking which jobs should be cancelled.

The algorithm: mark the source job, then for each depth level up to
max_depth, scan all jobs. A job is marked if it depends on any marked
job AND all its dependencies are either marked-for-cancel or not
Completed (no surviving path).

**Rationale:** Iterative depth-bounded scanning avoids recursion (Verus
handles loops better than deep recursion for this shape). The max_depth
bound makes the loop provably terminating and prevents stack overflow in
deep DAGs. The "no surviving path" check means a job with one cancelled
dep and one completed dep stays alive — only fully blocked jobs cascade.

**Alternative:** Recursive DFS. Rejected — harder to bound in Verus,
and the depth limit is more natural with iterative levels.

### 5. Priority as lexicographic (class, sequence) comparison

**Choice:** `is_higher_priority(class_a, seq_a, class_b, seq_b) -> bool`
returns true if (class_a, seq_a) is strictly higher priority. Higher
class wins. Within the same class, lower sequence number (submitted
earlier) wins. This is strict lexicographic order with sequence inverted.

**Rationale:** Matches how CI systems and job queues work: priority
classes (critical, normal, low) with FIFO within a class. The sequence
number is a monotonic counter, so lower = older = should run first.
Proving transitivity and antisymmetry on lexicographic order is
straightforward for Z3.

**Alternative:** Single numeric priority. Rejected — conflates urgency
(class) with ordering (age). Two-field comparison is what real systems
use.

### 6. Idempotency as membership check on a consumed sequence

**Choice:** `is_consumed(token, consumed: &[u64]) -> bool` does linear
scan. `try_execute(token, consumed: &[u64]) -> (bool, needs_append)`
returns whether to proceed and whether to append the token.

**Rationale:** The proof obligations are membership monotonicity (once
in the set, always in the set) and freshness (not in set → proceed).
Linear scan on `Seq<u64>` is natural for Verus proofs. Real
implementations use HashSets or bloom filters; we verify the decision
logic, not the lookup performance.

**Alternative:** Model as a HashSet. Rejected — Verus doesn't have
HashSet support, and the interesting properties are about set membership,
not lookup speed.

### 7. Phased implementation

**Choice:**
1. job_state + job_slot (independent, foundational)
2. job_priority + job_idempotency (independent, no cross-deps)
3. job_cancel (depends on job_state for terminal/non-terminal checks)
4. cross-module properties (ties everything together)

**Rationale:** Phases 1-2 can be written in any order. Phase 3 needs
the state constants from job_state. Phase 4 links to job_dag.

## Risks / Trade-offs

**[Cancellation graph scan is O(depth × jobs)]** → Acceptable. This is
decision logic, not a hot path. The depth bound keeps it finite. If
someone needs O(n) BFS, they implement it imperatively and call the
verified predicate to check the result.

**[Parallel arrays for DAG representation]** → Less ergonomic than an
adjacency list, but Verus reasons about `Seq<Seq<u64>>` fine. The
existing `job_dag` already uses flat slices. Keeping the same
representation avoids impedance mismatch.

**[No fairness or starvation proofs for priority]** → The priority
module proves ordering properties (total order), not scheduling
properties (every job eventually runs). Starvation prevention requires
a liveness model with time, which is out of scope. The fair_share
module covers weighted fairness separately.
