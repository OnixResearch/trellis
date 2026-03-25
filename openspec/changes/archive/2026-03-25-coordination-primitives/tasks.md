## Phase 1: Leader Election

- [x] Add coordination requirements to docs/requirements.md ✅
- [x] Implement src/election.rs: state constants (STATE_FOLLOWER/TRANSITIONING/LEADER as u8) ✅
- [x] State predicates: is_leader, is_follower, is_transitioning (spec+exec) ✅
- [x] Transition validation: is_valid_state_transition (spec+exec) ✅
- [x] Prove exhaustiveness: every (from, to) pair classified as valid or invalid ✅
- [x] Prove Leader self-transition requires same token ✅
- [x] Election preconditions: can_start_election, can_win_election, can_lose_election, can_stepdown, can_lose_leadership ✅
- [x] Prove can_win requires Transitioning AND new_token > max ✅
- [x] State-after functions: get_state_after_{start,win,lose,stepdown} ✅
- [x] Prove each state_after result satisfies is_valid_state_transition from its precondition state ✅
- [x] Token progression: compute_next_election_token, compute_max_token_after_win ✅
- [x] Prove monotonicity: next_token >= current_max_token ✅
- [x] Prove saturating: no overflow/wrap ✅
- [x] Well-formedness: is_leader_state_wellformed (spec+exec) ✅
- [x] Prove: leader → token > 0 ∧ token <= max ✅
- [x] Timing: calculate_election_timeout, is_renewal_time, should_start_election, should_maintain_leadership ✅
- [x] Prove: timeout result in [base, base + jitter_range] ✅
- [x] Prove: jitter_range=0 → result = base ✅
- [x] Unit tests + proptest for all election functions ✅
- [x] Verus clean (no new errors) ✅

## Phase 2: Distributed Lock

- [x] Implement src/distributed_lock.rs: compute_next_fencing_token ✅
- [x] Prove: result >= 1 for all inputs (None and Some) ✅
- [x] Prove: result >= previous token (monotonicity) ✅
- [x] Prove: result == 1 when no previous entry ✅
- [x] Deadline: compute_lock_deadline ✅
- [x] Prove: deadline >= acquired_at (saturating) ✅
- [x] Expiry: is_lock_expired, remaining_ttl ✅
- [x] Prove: expired implies remaining == 0 ✅
- [x] Prove: positive remaining implies not expired ✅
- [x] Backoff: compute_doubled_backoff, compute_jittered_delay ✅
- [x] Prove: doubled bounded by max ✅
- [x] Prove: jitter bounded by cap ✅
- [x] Unit tests + proptest ✅
- [x] Verus clean ✅

## Phase 3: Sequence Generator

- [x] Implement src/sequence.rs: batch state predicates ✅
- [x] should_refill_batch: Prove true iff batch_remaining == 0 ✅
- [x] batch_remaining: Prove == batch_end - next when next < batch_end, else 0 ✅
- [x] Batch computation: compute_batch_end, compute_next_after_refill ✅
- [x] Prove: batch_end returns None on overflow, Some(start + size) otherwise ✅
- [x] Prove: next_after_refill > start ✅
- [x] Range: compute_range_start, is_initial_reservation ✅
- [x] Prove: range_start > current ✅
- [x] CAS: compute_cas_expected ✅
- [x] Prove: None iff initial reservation ✅
- [x] Sequence value: compute_new_sequence_value (status + value tuple) ✅
- [x] Prove: Ok result > current input (strict monotonicity) ✅
- [x] Prove: Overflow returned iff current + count > u64::MAX ✅
- [x] Unit tests + proptest ✅
- [x] Verus clean ✅

## Phase 4: Read-Write Lock

- [x] Implement src/rwlock.rs: MODE_FREE/READ/WRITE constants ✅
- [x] Mutual exclusion: check_mutual_exclusion spec+exec ✅
- [x] Prove: mode=Write → active_readers == 0 ✅
- [x] Prove: mode=Read → no writer holds lock ✅
- [x] Prove: mode=Free → always valid ✅
- [x] Acquisition: can_acquire_read, can_acquire_write ✅
- [x] Mode transitions: mode_after_read_acquire, mode_after_read_release, mode_after_write_release ✅
- [x] Prove: write_release → Free unconditionally ✅
- [x] Prove: last reader → Free, non-last → Read ✅
- [x] Reader count: increment_reader_count, decrement_reader_count ✅
- [x] Prove: increment saturates at u32::MAX ✅
- [x] Prove: decrement saturates at 0 ✅
- [x] Prove: inc-dec roundtrip ✅
- [x] Writer token: compute_next_write_token ✅
- [x] Prove: monotonic, strictly increasing when not saturated ✅
- [x] Expiry: is_reader_expired, is_writer_expired ✅
- [x] Prove: reader and writer expiry are identical predicates ✅
- [x] Unit tests + proptest ✅
- [x] Verus clean ✅

## Phase 5: Cross-Module Properties + Integration

- [x] election token monotonicity → fencing::issue_token monotonicity (same value for non-MAX) ✅
- [x] distributed_lock::is_lock_expired → expiration::is_expired (identical predicate) ✅
- [x] lock first token always 1 ✅
- [x] rwlock writer token → fencing::issue_token (same value for non-MAX) ✅
- [x] rwlock reader/writer expiry → expiration::is_expired (identical predicate) ✅
- [x] sequence overflow → None exactly when addition overflows u64 ✅
- [x] election and rwlock tokens agree (same function) ✅
- [x] Verus proofs: 6 cross-module proofs, all automatic ✅
- [x] Property tests: 7 proptest properties ✅
- [x] Verus clean ✅
