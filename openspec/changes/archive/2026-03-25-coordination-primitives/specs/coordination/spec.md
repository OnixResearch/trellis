# Coordination Primitives Specification

## Purpose

Defines correctness properties for distributed coordination primitives:
leader election state machines, distributed lock protocols, monotonic
sequence generators, and read-write lock mutual exclusion. These are
pure decision functions — no I/O, no async, time passed explicitly.

## Requirements

### Leader Election

#### Requirement: Valid State Transitions

The election state machine MUST permit only these transitions:
Follower→Transitioning (start election), Transitioning→Leader (win),
Transitioning→Follower (lose), Leader→Follower (stepdown or lost
leadership). Self-transitions (same state to same state) MUST be
permitted. All other transitions MUST be rejected.

##### Scenario: Start election

- GIVEN state=Follower
- WHEN transitioning to Transitioning
- THEN MUST be valid

##### Scenario: Win election

- GIVEN state=Transitioning
- WHEN transitioning to Leader with fencing token
- THEN MUST be valid

##### Scenario: Lose election

- GIVEN state=Transitioning
- WHEN transitioning to Follower
- THEN MUST be valid

##### Scenario: Stepdown

- GIVEN state=Leader
- WHEN transitioning to Follower
- THEN MUST be valid

##### Scenario: Invalid skip to Leader

- GIVEN state=Follower
- WHEN transitioning directly to Leader
- THEN MUST be invalid

##### Scenario: Invalid Leader to Transitioning

- GIVEN state=Leader
- WHEN transitioning to Transitioning
- THEN MUST be invalid

##### Scenario: Self-transition Follower

- GIVEN state=Follower
- WHEN transitioning to Follower
- THEN MUST be valid

##### Scenario: Leader self-transition requires same token

- GIVEN state=Leader with fencing_token=5
- WHEN transitioning to Leader with fencing_token=5
- THEN MUST be valid

##### Scenario: Leader self-transition rejects changed token

- GIVEN state=Leader with fencing_token=5
- WHEN transitioning to Leader with fencing_token=6
- THEN MUST be invalid (token cannot change while leader)

#### Requirement: Election Preconditions

`can_start_election` MUST require Follower state and running=true.
`can_win_election` MUST require Transitioning state and new_token >
max_fencing_token. `can_lose_election` MUST require Transitioning state.
`can_stepdown` MUST require Leader state. `can_lose_leadership` MUST
require Leader state.

##### Scenario: Can start from Follower

- GIVEN state=Follower, running=true
- WHEN checking can_start_election
- THEN MUST return true

##### Scenario: Cannot start when not running

- GIVEN state=Follower, running=false
- WHEN checking can_start_election
- THEN MUST return false

##### Scenario: Cannot start from Leader

- GIVEN state=Leader, running=true
- WHEN checking can_start_election
- THEN MUST return false

##### Scenario: Can win from Transitioning

- GIVEN state=Transitioning, max_fencing_token=5, new_token=6
- WHEN checking can_win_election
- THEN MUST return true

##### Scenario: Cannot win with stale token

- GIVEN state=Transitioning, max_fencing_token=5, new_token=3
- WHEN checking can_win_election
- THEN MUST return false

##### Scenario: Cannot win from Follower

- GIVEN state=Follower, max_fencing_token=5, new_token=6
- WHEN checking can_win_election
- THEN MUST return false

#### Requirement: Token Monotonicity

`compute_next_election_token` MUST return current_max_token + 1
(saturating). The token after winning MUST be >= the token used to win.
Tokens MUST never decrease across election cycles.

##### Scenario: Next token

- GIVEN current_max_token=5
- WHEN computing next election token
- THEN MUST return 6

##### Scenario: Saturating at max

- GIVEN current_max_token=u64::MAX
- WHEN computing next election token
- THEN MUST return u64::MAX (saturated)

##### Scenario: Token after win

- GIVEN new_token=6
- WHEN computing max_token_after_win
- THEN MUST return a value >= 6

#### Requirement: State After Transitions

`get_state_after_start_election` MUST return Transitioning.
`get_state_after_win_election(token)` MUST return Leader with the given
token. `get_state_after_lose_election` MUST return Follower.
`get_state_after_stepdown` MUST return Follower.
`get_state_after_lose_leadership` MUST return Follower.

##### Scenario: After start

- WHEN getting state after starting election
- THEN MUST be Transitioning

##### Scenario: After win

- GIVEN fencing_token=7
- WHEN getting state after winning
- THEN MUST be Leader with fencing_token=7

##### Scenario: After lose

- WHEN getting state after losing election
- THEN MUST be Follower

##### Scenario: After stepdown

- WHEN getting state after stepdown
- THEN MUST be Follower

#### Requirement: Leader Well-Formedness

A leader state is well-formed if and only if the fencing token is > 0
and <= max_fencing_token. A non-leader state is always well-formed.

##### Scenario: Valid leader

- GIVEN is_leader=true, fencing_token=5, max_fencing_token=10
- WHEN checking well-formedness
- THEN MUST be true

##### Scenario: Zero token leader

- GIVEN is_leader=true, fencing_token=0, max_fencing_token=10
- WHEN checking well-formedness
- THEN MUST be false

##### Scenario: Token exceeds max

- GIVEN is_leader=true, fencing_token=15, max_fencing_token=10
- WHEN checking well-formedness
- THEN MUST be false

##### Scenario: Non-leader always well-formed

- GIVEN is_leader=false, fencing_token=0, max_fencing_token=0
- WHEN checking well-formedness
- THEN MUST be true

#### Requirement: Renewal Timing

`is_renewal_time` MUST return true when enough time has elapsed since
the last renewal. `should_maintain_leadership` MUST return true when
the leader's lease is still valid. `should_start_election` MUST return
true when the follower has not heard from the leader within the
election timeout.

##### Scenario: Renewal due

- GIVEN last_renewed_ms=1000, now_ms=6000, interval_ms=5000
- WHEN checking is_renewal_time
- THEN MUST return true

##### Scenario: Renewal not due

- GIVEN last_renewed_ms=1000, now_ms=3000, interval_ms=5000
- WHEN checking is_renewal_time
- THEN MUST return false

#### Requirement: Election Timeout with Jitter

`calculate_election_timeout` MUST return a value in the range
[base_timeout_ms, base_timeout_ms + jitter_range_ms]. The jitter MUST
be deterministic given the same seed. The result MUST use saturating
arithmetic.

##### Scenario: Base timeout with jitter

- GIVEN base_timeout_ms=1500, jitter_seed=42, jitter_range_ms=500
- WHEN calculating election timeout
- THEN result MUST be in [1500, 2000]

##### Scenario: Zero jitter range

- GIVEN base_timeout_ms=1500, jitter_seed=42, jitter_range_ms=0
- WHEN calculating election timeout
- THEN result MUST be 1500

### Distributed Lock

#### Requirement: Token Always Positive

`compute_next_fencing_token` MUST return a value >= 1. When no previous
lock entry exists, the token MUST be 1. When a previous entry exists,
the token MUST be previous_token + 1 (saturating).

##### Scenario: First lock

- GIVEN no previous lock entry
- WHEN computing next fencing token
- THEN MUST return 1

##### Scenario: Subsequent lock

- GIVEN previous entry with fencing_token=5
- WHEN computing next fencing token
- THEN MUST return 6

##### Scenario: Saturating at max

- GIVEN previous entry with fencing_token=u64::MAX
- WHEN computing next fencing token
- THEN MUST return u64::MAX

#### Requirement: Token Monotonicity

The fencing token produced by `compute_next_fencing_token` MUST be
greater than or equal to the previous token. Tokens MUST never decrease
across lock acquisitions.

##### Scenario: Monotonic progression

- GIVEN previous fencing_token=N for any N
- WHEN computing next token
- THEN result MUST be >= N

#### Requirement: Lock Expiry

`is_lock_expired` MUST return true when now_ms > deadline_ms.
`remaining_ttl_ms` MUST return deadline_ms - now_ms when the lock is
not expired, and 0 when expired. `compute_lock_deadline` MUST return
acquired_at_ms + ttl_ms (saturating).

##### Scenario: Not expired

- GIVEN deadline_ms=10000, now_ms=5000
- WHEN checking expiry
- THEN is_lock_expired MUST be false
- AND remaining_ttl_ms MUST be 5000

##### Scenario: Expired

- GIVEN deadline_ms=5000, now_ms=10000
- WHEN checking expiry
- THEN is_lock_expired MUST be true
- AND remaining_ttl_ms MUST be 0

##### Scenario: Exact deadline

- GIVEN deadline_ms=5000, now_ms=5000
- WHEN checking expiry
- THEN is_lock_expired MUST be false (deadline is inclusive)

##### Scenario: Deadline computation

- GIVEN acquired_at_ms=1000, ttl_ms=5000
- WHEN computing deadline
- THEN MUST return 6000

##### Scenario: Deadline overflow

- GIVEN acquired_at_ms=u64::MAX - 100, ttl_ms=200
- WHEN computing deadline
- THEN MUST return u64::MAX (saturated, not wrapped)

#### Requirement: Backoff with Jitter

`compute_backoff_with_jitter` MUST return a delay in the range
[0, min(current_backoff * 2, max_backoff)]. The next backoff base
MUST be min(current_backoff * 2, max_backoff). Jitter MUST be
deterministic given the same seed.

##### Scenario: Normal backoff

- GIVEN current_backoff_ms=100, max_backoff_ms=10000, jitter_seed=42
- WHEN computing backoff
- THEN delay MUST be in [0, 200]
- AND next base MUST be 200

##### Scenario: Capped backoff

- GIVEN current_backoff_ms=8000, max_backoff_ms=10000
- WHEN computing backoff
- THEN next base MUST be 10000 (capped)

### Sequence Generator

#### Requirement: Monotonic Values

`compute_new_sequence_value` MUST return a value strictly greater than
the current value. Sequence values MUST never decrease. If addition
would overflow, MUST return an overflow indicator rather than wrapping.

##### Scenario: Normal increment

- GIVEN current=100, count=10
- WHEN computing new sequence value
- THEN result MUST be > 100
- AND result MUST encode a range of 10 IDs

##### Scenario: Overflow

- GIVEN current=u64::MAX - 5, count=10
- WHEN computing new sequence value
- THEN MUST indicate overflow (not wrap to a small number)

#### Requirement: Batch State

`should_refill_batch` MUST return true when next >= batch_end.
`batch_remaining` MUST return batch_end - next when next < batch_end,
and 0 otherwise.

##### Scenario: Batch exhausted

- GIVEN next=100, batch_end=100
- WHEN checking should_refill
- THEN MUST return true

##### Scenario: Batch has capacity

- GIVEN next=50, batch_end=100
- WHEN checking should_refill
- THEN MUST return false
- AND batch_remaining MUST be 50

##### Scenario: Next beyond batch_end

- GIVEN next=150, batch_end=100
- WHEN checking
- THEN should_refill MUST be true
- AND batch_remaining MUST be 0

#### Requirement: Batch Computation

`compute_batch_end` MUST return batch_start + batch_size. If the
addition would overflow, MUST return None. `compute_next_after_refill`
MUST return batch_start + 1. If the addition would overflow, MUST
return None.

##### Scenario: Normal batch

- GIVEN batch_start=100, batch_size=50
- WHEN computing batch_end
- THEN MUST return Some(150)

##### Scenario: Batch overflow

- GIVEN batch_start=u64::MAX - 10, batch_size=20
- WHEN computing batch_end
- THEN MUST return None

##### Scenario: After refill

- GIVEN batch_start=100
- WHEN computing next after refill
- THEN MUST return Some(101)

#### Requirement: Range Start

`compute_range_start` MUST return current + 1. If the addition would
overflow, MUST return None. `is_initial_reservation` MUST return true
if and only if current equals start_value.

##### Scenario: Normal range

- GIVEN current=50
- WHEN computing range start
- THEN MUST return Some(51)

##### Scenario: Initial reservation

- GIVEN current=0, start_value=0
- WHEN checking is_initial_reservation
- THEN MUST return true

##### Scenario: Non-initial

- GIVEN current=50, start_value=0
- WHEN checking is_initial_reservation
- THEN MUST return false

#### Requirement: CAS Expected Value

`compute_cas_expected` MUST return the expected value for the
compare-and-swap operation. For initial reservations (current ==
start_value), MUST return None (key doesn't exist yet). For subsequent
reservations, MUST return Some(current).

##### Scenario: Initial CAS

- GIVEN current=0, start_value=0
- WHEN computing CAS expected
- THEN MUST return None

##### Scenario: Subsequent CAS

- GIVEN current=100, start_value=0
- WHEN computing CAS expected
- THEN MUST return Some(100)

### Read-Write Lock

#### Requirement: Mutual Exclusion

Read mode and Write mode MUST NOT be simultaneously active. If the
mode is Write, the active reader count MUST be 0. If the mode is Read
with active readers, no writer MUST hold the lock.

##### Scenario: Write excludes readers

- GIVEN mode=Write, readers=[(alice, deadline)]
- WHEN checking mutual exclusion (with non-expired readers)
- THEN check_mutual_exclusion MUST report a violation

##### Scenario: Read excludes writers

- GIVEN mode=Read, active_readers=2, writer_holds_lock=true
- WHEN checking mutual exclusion
- THEN MUST report a violation

##### Scenario: Free is always valid

- GIVEN mode=Free
- WHEN checking mutual exclusion
- THEN MUST report no violation regardless of reader/writer state

#### Requirement: Mode Transitions

`can_acquire_read` MUST return true when mode is Free, or mode is Read
with no pending writers (or writer expired). MUST return false when
mode is Write (unless writer expired). `can_acquire_write` MUST return
true only when mode is Free (or all readers expired and no active
writer), or when the writer lock is expired.

##### Scenario: Read in Free mode

- GIVEN mode=Free
- WHEN checking can_acquire_read
- THEN MUST return true

##### Scenario: Read in Read mode no pending writers

- GIVEN mode=Read, pending_writers=0
- WHEN checking can_acquire_read
- THEN MUST return true

##### Scenario: Read blocked by writer

- GIVEN mode=Write, writer_expired=false
- WHEN checking can_acquire_read
- THEN MUST return false

##### Scenario: Read when writer expired

- GIVEN mode=Write, writer_expired=true
- WHEN checking can_acquire_read
- THEN MUST return true

##### Scenario: Write in Free mode

- GIVEN mode=Free, active_readers=0
- WHEN checking can_acquire_write
- THEN MUST return true

##### Scenario: Write blocked by readers

- GIVEN mode=Read, active_readers=3, writer_expired=false
- WHEN checking can_acquire_write
- THEN MUST return false

##### Scenario: Write when readers expired

- GIVEN mode=Read, active_readers=0, writer_expired=true
- WHEN checking can_acquire_write
- THEN MUST return true

#### Requirement: Mode After Operations

`mode_after_read_acquire` on Free or Read MUST return Read.
`mode_after_read_release` MUST return Free when reader count drops to
0, Read otherwise. `mode_after_write_release` MUST always return Free.

##### Scenario: Read acquire from Free

- GIVEN mode=Free
- WHEN mode_after_read_acquire
- THEN MUST be Read

##### Scenario: Read acquire from Read

- GIVEN mode=Read
- WHEN mode_after_read_acquire
- THEN MUST be Read

##### Scenario: Last reader releases

- GIVEN reader_count_after=0
- WHEN mode_after_read_release
- THEN MUST be Free

##### Scenario: Readers remain

- GIVEN reader_count_after=2
- WHEN mode_after_read_release
- THEN MUST be Read

##### Scenario: Write release

- WHEN mode_after_write_release
- THEN MUST be Free (unconditionally)

#### Requirement: Reader Count Arithmetic

`increment_reader_count` MUST return current + 1 (saturating at u32::MAX).
`decrement_reader_count` MUST return current - 1 (saturating at 0).
The reader count MUST be bounded by a maximum (checked by
`check_readers_bounded`).

##### Scenario: Increment

- GIVEN current_count=5
- WHEN incrementing
- THEN MUST return 6

##### Scenario: Decrement

- GIVEN current_count=5
- WHEN decrementing
- THEN MUST return 4

##### Scenario: Decrement at zero

- GIVEN current_count=0
- WHEN decrementing
- THEN MUST return 0 (saturated)

##### Scenario: Increment at max

- GIVEN current_count=u32::MAX
- WHEN incrementing
- THEN MUST return u32::MAX (saturated)

#### Requirement: Writer Fencing Token

`compute_next_write_token` MUST return current_token + 1 (saturating).
Writer tokens MUST be monotonically increasing. Each write acquisition
MUST produce a token strictly greater than the previous (unless
saturated).

##### Scenario: Token progression

- GIVEN current_token=5
- WHEN computing next write token
- THEN MUST return 6

##### Scenario: Token after acquire

- GIVEN current_token=5
- WHEN computing fencing_token_after_write_acquire
- THEN MUST return 6

##### Scenario: Saturating at max

- GIVEN current_token=u64::MAX
- WHEN computing next write token
- THEN MUST return u64::MAX

#### Requirement: Reader/Writer Expiry

`is_reader_expired` MUST return true when now_ms > deadline_ms.
`is_writer_expired` MUST return true when now_ms > deadline_ms.
`filter_expired_readers` MUST return only readers whose deadline >=
now_ms. `count_active_readers` MUST count only non-expired readers.

##### Scenario: Active reader

- GIVEN deadline_ms=10000, now_ms=5000
- WHEN checking expiry
- THEN is_reader_expired MUST be false

##### Scenario: Expired reader

- GIVEN deadline_ms=5000, now_ms=10000
- WHEN checking expiry
- THEN is_reader_expired MUST be true

##### Scenario: Filter preserves active

- GIVEN readers=[(a, 10000), (b, 3000), (c, 8000)], now_ms=5000
- WHEN filtering expired
- THEN result MUST contain a and c but not b

##### Scenario: Count active

- GIVEN readers=[(a, 10000), (b, 3000), (c, 8000)], now_ms=5000
- WHEN counting active
- THEN count MUST be 2

#### Requirement: Downgrade

`can_downgrade_lock` MUST return true only when the caller holds the
write lock. Downgrading converts Write mode to Read mode, adding the
former writer as a reader.

##### Scenario: Writer can downgrade

- GIVEN mode=Write, caller holds write lock
- WHEN checking can_downgrade
- THEN MUST return true

##### Scenario: Reader cannot downgrade

- GIVEN mode=Read
- WHEN checking can_downgrade
- THEN MUST return false

#### Requirement: Release Preconditions

`can_release_read_lock` MUST return true only when the caller holds a
read lock. `can_release_write_lock` MUST return true only when the
caller holds the write lock.

##### Scenario: Valid read release

- GIVEN caller has an active read lock
- WHEN checking can_release_read_lock
- THEN MUST return true

##### Scenario: Invalid read release

- GIVEN caller does not have a read lock
- WHEN checking can_release_read_lock
- THEN MUST return false

##### Scenario: Valid write release

- GIVEN caller holds the write lock
- WHEN checking can_release_write_lock
- THEN MUST return true
