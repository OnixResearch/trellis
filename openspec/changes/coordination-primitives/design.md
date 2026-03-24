## Context

aspen-coordination's `verified/` module has 12 submodules of pure functions
totaling ~9,500 lines. verified-logic already has the mathematical
foundations: `fencing` (token issuance), `expiration` (TTL/expired checks),
`lease` (grant/renew/revoke), `backoff` (exponential with cap), `quorum`
(majority), `cas` (CAS conditions), `two_phase` (unanimity), `drain`
(admission). The gap is the coordination protocol logic that composes
these into election transitions, lock protocols, sequence generation, and
rwlock mode management.

Four modules are high-value targets: election (state machine bugs cause
dual leaders), lock (token bugs cause stale writes), sequence (overflow
bugs cause duplicate IDs), rwlock (mode bugs break mutual exclusion). The
remaining eight (counter, barrier, semaphore, queue, registry, strategies,
worker, fencing-cross-primitive) are lower priority — counter and barrier
overlap more with existing modules, and queue/registry/strategies/worker
are larger and more domain-specific.

## Goals / Non-Goals

**Goals:**
- Verified election state machine with transition guard proofs
- Verified lock token monotonicity composing existing fencing + expiration
- Verified sequence monotonicity with overflow protection
- Verified rwlock mutual exclusion invariant
- All modules follow existing combined pattern: `verus!` block with
  spec fn + exec fn + proof in the same file
- Cross-module properties linking to existing library primitives
- Pure functions only: no I/O, no async, time as explicit u64 parameter

**Non-Goals:**
- Porting aspen-coordination's imperative shell (async CAS loops, retries)
- Counter, barrier, semaphore (simpler; barrier phases overlap two_phase)
- Queue submodules (1,200 lines of domain-specific queue protocol)
- Registry (mostly TTL patterns already in expiration.rs)
- Strategies + Worker (2,100 lines of scheduling heuristics — hard to state
  invariants for load balancing)
- Cross-primitive fencing (aspen's fencing.rs overlaps our existing fencing.rs
  + quorum.rs; the HashMap-based split-brain check needs String which Verus
  handles poorly)

## Decisions

### 1. Election as self-contained enum state machine

**Choice:** Define `LeadershipState` as an enum with Follower, Transitioning,
Leader(u64). Transition functions take `&LeadershipState` and return a new
state or bool. No reference to aspen-coordination's `FencingToken` wrapper —
use raw u64 for the fencing token to keep Verus happy.

**Rationale:** Verus handles enum matching well. The state machine is small
(3 states, 5 transitions) and the invariants are expressible as spec fns
over the enum. Using u64 instead of a newtype avoids the need to teach
Verus about wrapper methods.

**Alternative:** Import aspen-coordination's types. Rejected — verified-logic
has no dependency on aspen-coordination and shouldn't. The types are simple
enough to redefine.

### 2. Lock module composes existing verified primitives

**Choice:** `distributed_lock` module imports and uses:
- `fencing::issue_token` for the token progression proof
- `expiration::is_expired` for the deadline check
- `backoff::compute_backoff` for the retry delay

The lock-specific functions (`compute_next_fencing_token`,
`compute_lock_deadline`, `is_lock_expired`, `remaining_ttl_ms`,
`compute_backoff_with_jitter`) call through to these existing functions
where possible, adding lock-specific preconditions (token >= 1, deadline
>= acquired_at).

**Rationale:** Demonstrates the composability thesis of verified-logic.
The lock module is thin glue with lock-specific invariants proved on top
of already-verified building blocks.

**Alternative:** Reimplement the arithmetic independently. Rejected — that
duplicates proofs and misses the composition story.

### 3. Sequence uses Option for overflow signaling

**Choice:** `compute_batch_end`, `compute_next_after_refill`,
`compute_range_start` all return `Option<u64>`. None means overflow.
`compute_new_sequence_value` returns a `SequenceResult` enum with
`Ok { new_value, range_end }` and `Overflow` variants.

**Rationale:** Verus proves the None/Overflow case is returned exactly
when addition would exceed u64::MAX. This is stronger than saturating
arithmetic (which silently caps) — callers know overflow happened.
Matches aspen-coordination's existing approach.

**Alternative:** Saturating arithmetic everywhere. Rejected — for
sequence generators, silently capping means returning duplicate IDs, which
is the exact bug we're trying to prevent.

### 4. RWLock uses mode enum, not reader/writer flags

**Choice:** Three-state enum `RWLockMode { Free, Read, Write }`. Mode
transitions are pure functions: `mode_after_read_acquire(current) -> mode`,
`mode_after_read_release(remaining_readers) -> mode`, etc. Mutual exclusion
is a spec fn: `mutual_exclusion_holds(mode, active_readers, has_writer)`.

**Rationale:** The enum makes illegal states unrepresentable at the type
level (can't be Read and Write simultaneously just by looking at the mode).
The mutual exclusion spec fn then additionally proves that the reader count
and writer state are consistent with the mode.

**Alternative:** Separate `has_readers: bool, has_writer: bool` booleans.
Rejected — this allows the (true, true) state that we're trying to prove
impossible.

### 5. Reader/writer expiry uses existing expiration pattern

**Choice:** `is_reader_expired(deadline, now)` and `is_writer_expired(deadline, now)`
delegate to the same `now > deadline` pattern as `expiration::is_expired`.
`filter_expired_readers` and `count_active_readers` build on top.

**Rationale:** Same arithmetic, same proof structure. Cross-module
property: `is_reader_expired(d, n) == expiration::is_expired(d, n)`.

### 6. Phased implementation: election → lock → sequence → rwlock

**Choice:** Build in dependency order:
1. Election — standalone, no deps on other new modules
2. Lock — composes fencing + expiration + backoff (all existing)
3. Sequence — standalone arithmetic, no deps
4. RWLock — standalone state machine, can reference fencing for writer tokens

**Rationale:** Election and sequence are independent. Lock depends on
existing modules (tests composition). RWLock is the largest and benefits
from patterns established in the election module.

## Risks / Trade-offs

**[Enum matching in Verus]** → Verus handles basic enum matching but
nested patterns (matching on tuples of enums) can be slow. Mitigation:
use helper predicates (`is_leader`, `is_follower`) instead of deep
pattern matches in spec fns.

**[String parameters in rwlock]** → `has_read_lock` takes `holder_id: &str`.
Verus has limited string support. Mitigation: the rwlock module's verified
functions focus on the mode transitions and count arithmetic (u32/u64).
String-keyed lookups (`has_read_lock`, `has_write_lock`) are exec-only with
runtime assertions, not full Verus spec proofs.

**[Cross-module composition proofs]** → Proving that `distributed_lock`
correctly uses `fencing::issue_token` requires Verus to see both modules.
Mitigation: cross-module properties go in `src/properties.rs` which already
imports all modules. If Verus struggles with cross-module reasoning, fall
back to proptest-level cross-checks.

**[Scope creep into queue/registry/worker]** → The four chosen modules
cover the highest-value coordination primitives. Queue, registry, and
worker are larger, more domain-specific, and have weaker invariants
(hard to state "the queue is correct" as a simple predicate). Guard rail:
if a function needs HashMap, Vec<String>, or domain-specific types (QueueItem,
ServiceInstance), it's out of scope for this change.
