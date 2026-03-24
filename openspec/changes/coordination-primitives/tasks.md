## Phase 1: Leader Election

- [ ] Add coordination requirements to docs/requirements.md
- [ ] Implement src/election.rs: LeadershipState enum (Follower/Transitioning/Leader(u64))
- [ ] State predicates: is_leader, is_follower, is_transitioning (spec+exec)
- [ ] Transition validation: is_valid_state_transition (spec+exec)
- [ ] Prove exhaustiveness: every (from, to) pair classified as valid or invalid
- [ ] Prove Leader self-transition requires same token
- [ ] Election preconditions: can_start_election, can_win_election, can_lose_election, can_stepdown, can_lose_leadership
- [ ] Prove can_win requires Transitioning AND new_token > max
- [ ] State-after functions: get_state_after_{start,win,lose,stepdown,lose_leadership}
- [ ] Prove each state-after result satisfies is_valid_state_transition from its precondition state
- [ ] Token progression: compute_next_election_token, compute_max_token_after_win
- [ ] Prove monotonicity: next_token >= current_max_token
- [ ] Prove saturating: no overflow/wrap
- [ ] Well-formedness: is_leader_state_wellformed (spec+exec)
- [ ] Prove: leader → token > 0 ∧ token <= max
- [ ] Timing: calculate_election_timeout, is_renewal_time, should_start_election, should_maintain_leadership
- [ ] Prove: timeout result in [base, base + jitter_range]
- [ ] Prove: jitter_range=0 → result = base
- [ ] Unit tests + proptest for all election functions
- [ ] Verus + tracey clean

## Phase 2: Distributed Lock

- [ ] Implement src/distributed_lock.rs: compute_next_fencing_token
- [ ] Prove: result >= 1 for all inputs (None and Some)
- [ ] Prove: result >= previous token (monotonicity)
- [ ] Prove: result == 1 when no previous entry
- [ ] Deadline: compute_lock_deadline, compute_acquire_deadline
- [ ] Prove: deadline >= acquired_at (saturating)
- [ ] Prove: deadline == acquired_at + ttl when no overflow
- [ ] Expiry: is_lock_expired, remaining_ttl_ms
- [ ] Prove: !expired ↔ remaining > 0
- [ ] Prove: expired ↔ now > deadline
- [ ] Cross-module: link is_lock_expired to expiration::is_expired
- [ ] Backoff: compute_backoff_with_jitter (BackoffResult struct)
- [ ] Prove: delay in [0, min(current*2, max)]
- [ ] Prove: next_base == min(current*2, max)
- [ ] Cross-module: link to backoff::compute_backoff
- [ ] Unit tests + proptest
- [ ] Verus + tracey clean

## Phase 3: Sequence Generator

- [ ] Implement src/sequence.rs: batch state predicates
- [ ] should_refill_batch: Prove true iff next >= batch_end
- [ ] batch_remaining: Prove == batch_end - next when next < batch_end, else 0
- [ ] Batch computation: compute_batch_end, compute_next_after_refill
- [ ] Prove: batch_end returns None on overflow, Some(start + size) otherwise
- [ ] Prove: next_after_refill returns None on overflow, Some(start + 1) otherwise
- [ ] Range: compute_range_start, is_initial_reservation, compute_initial_current
- [ ] Prove: range_start returns None on overflow
- [ ] Prove: is_initial iff current == start_value
- [ ] Sequence value: compute_new_sequence_value (SequenceResult enum)
- [ ] Prove: Ok result > current input (strict monotonicity)
- [ ] Prove: Overflow returned iff current + count > u64::MAX
- [ ] CAS: compute_cas_expected
- [ ] Prove: None iff initial reservation, Some(current) otherwise
- [ ] Parsing: parse_sequence_value
- [ ] Unit tests + proptest
- [ ] Verus + tracey clean

## Phase 4: Read-Write Lock

- [ ] Implement src/rwlock.rs: RWLockMode enum (Free/Read/Write)
- [ ] Mutual exclusion: check_mutual_exclusion spec+exec
- [ ] Prove: mode=Write → active_readers == 0
- [ ] Prove: mode=Read with active_readers > 0 → no writer holds lock
- [ ] Prove: mode=Free → always valid
- [ ] Acquisition: can_acquire_read, can_acquire_write
- [ ] Prove: can_acquire_read when Free or (Read and no pending writers or writer expired)
- [ ] Prove: can_acquire_write only when Free (or all expired)
- [ ] Prove: cannot acquire_write when active readers > 0 and not expired
- [ ] Mode transitions: mode_after_read_acquire, mode_after_read_release, mode_after_write_release
- [ ] Prove: read_acquire → mode is Read
- [ ] Prove: read_release with 0 remaining → Free
- [ ] Prove: read_release with >0 remaining → Read
- [ ] Prove: write_release → Free unconditionally
- [ ] Reader count: increment_reader_count, decrement_reader_count
- [ ] Prove: increment saturates at u32::MAX
- [ ] Prove: decrement saturates at 0
- [ ] Prove: check_readers_bounded with configurable max
- [ ] Writer token: compute_next_write_token, compute_fencing_token_after_write_acquire
- [ ] Prove: monotonic (result >= current)
- [ ] Prove: saturating at u64::MAX
- [ ] Cross-module: link to fencing::issue_token
- [ ] Expiry: is_reader_expired, is_writer_expired, filter_expired_readers, count_active_readers
- [ ] Prove: filter preserves only non-expired entries
- [ ] Prove: count_active == length of filtered list
- [ ] Cross-module: link to expiration::is_expired
- [ ] Downgrade: can_downgrade_lock
- [ ] Release preconditions: can_release_read_lock, can_release_write_lock
- [ ] Pending writers: increment_pending_writers, decrement_pending_writers
- [ ] Unit tests + proptest
- [ ] Verus + tracey clean

## Phase 5: Cross-Module Properties + Integration

- [ ] election token monotonicity → fencing::issue_token monotonicity (shared pattern)
- [ ] distributed_lock::is_lock_expired → expiration::is_expired (equivalence)
- [ ] distributed_lock::compute_backoff_with_jitter → backoff::compute_backoff (bounded by cap)
- [ ] rwlock writer token → fencing::issue_token (same progression)
- [ ] rwlock reader/writer expiry → expiration::is_expired (same predicate)
- [ ] sequence overflow → None signals align with checked_add semantics
- [ ] Property tests for all cross-module relationships
- [ ] Verus + tracey clean
