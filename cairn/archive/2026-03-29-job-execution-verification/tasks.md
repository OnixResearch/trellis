## Phase 1: Job Retry

- [x] Implement src/job_retry.rs: is_retryable_error spec+exec (range classification) ✅
- [x] Implement attempts_remaining spec+exec (saturating subtraction) ✅
- [x] Implement is_cooldown_elapsed spec+exec (timestamp + int comparison) ✅
- [x] Implement should_retry spec+exec (conjunction of three criteria) ✅
- [x] Implement next_attempt spec+exec (capped increment) ✅
- [x] Prove: non-retryable error → should_retry is false ✅
- [x] Prove: max attempts reached → should_retry is false ✅
- [x] Prove: attempts_remaining is bounded by max_attempts ✅
- [x] Prove: attempts_remaining == 0 iff attempts >= max_attempts ✅
- [x] Prove: next_attempt <= max_attempts (cap invariant) ✅
- [x] Prove: cooldown monotonicity (elapsed at t → elapsed at t' > t) ✅
- [x] Prove: next_attempt decreases remaining by 1 ✅
- [x] Unit tests + proptest for job_retry ✅ (13 tests)
- [x] Verus + tracey clean ✅

## Phase 2: Job Timeout

- [x] Implement src/job_timeout.rs: compute_deadline spec+exec (saturating add) ✅
- [x] Implement is_timed_out spec+exec (now >= deadline) ✅
- [x] Implement time_remaining spec+exec (saturating sub) ✅
- [x] Implement elapsed_time spec+exec (saturating sub) ✅
- [x] Implement extend_deadline spec+exec (grace + max_extension cap) ✅
- [x] Prove: timeout monotonicity (timed out at t → timed out at t' > t) ✅
- [x] Prove: time_remaining + elapsed_time == timeout_ms (when not overflowed) ✅
- [x] Prove: extend_deadline >= original deadline (extension only pushes forward) ✅
- [x] Prove: extend_deadline <= deadline + max_extension (bounded extension) ✅
- [x] Prove: compute_deadline >= start_time (deadline is in the future) ✅
- [x] Prove: not timed out at start (when no overflow) ✅
- [x] Prove: zero grace is identity, grace within limit uses actual grace ✅
- [x] Unit tests + proptest for job_timeout ✅ (14 tests)
- [x] Verus + tracey clean ✅

## Phase 3: Job Result

- [x] Implement src/job_result.rs: is_success_code spec+exec (== 0) ✅
- [x] Implement is_failure_code spec+exec (1-127 range) ✅
- [x] Implement is_error_code spec+exec (>= 128 range) ✅
- [x] Implement output_hash_matches spec+exec (u64 equality) ✅
- [x] Implement output_size_valid spec+exec (min <= actual <= max) ✅
- [x] Implement validate_result spec+exec (first-failure criterion) ✅
- [x] Prove: exit code classification is exhaustive (exactly one category) ✅
- [x] Prove: exit code classification is mutually exclusive ✅
- [x] Prove: validate_result == 0 iff all criteria pass ✅
- [x] Prove: validate_result determinism (same inputs → same output) ✅
- [x] Prove: validate_result order (bad exit checked before hash before size) ✅
- [x] Prove: status always in range {0, 1, 2, 3} ✅
- [x] Unit tests + proptest for job_result ✅ (13 tests)
- [x] Verus + tracey clean ✅

## Phase 4: Cross-Module Properties + Integration

- [x] retry non-retryable → never retries (Verus proof + proptest) ✅
- [x] is_timed_out → job should transition to FAILED (Verus proof + proptest) ✅
- [x] validate_result == 0 → job should transition to COMPLETED (Verus proof + proptest) ✅
- [x] validate_result != 0 → job should transition to FAILED (Verus proof + proptest) ✅
- [x] is_retryable_error range is disjoint from success code (Verus proof + proptest) ✅
- [x] Add all modules to src/lib.rs ✅
- [x] Verus + tracey clean ✅ (4165 total verified, 0 errors)
