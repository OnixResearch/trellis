## Phase 1: Core retry gate

- [x] Add `should_retry_spec` — bool: is_retryable && attempt < max_attempts ✅
- [x] Add `should_retry` exec fn with ensures ✅
- [x] Proof: `never_retries_non_retryable` — !is_retryable ⟹ !should_retry ✅
- [x] Proof: `never_retries_exhausted` — attempt >= max ⟹ !should_retry ✅
- [x] Proof: `first_attempt_retries` — is_retryable && max > 0 ⟹ should_retry(0, max, true) ✅
- [x] Verus + cargo test green ✅

## Phase 2: Attempt counting

- [x] Add `attempts_remaining_spec` — saturating sub: if attempt < max { max - attempt } else { 0 } ✅
- [x] Add `attempts_remaining` exec fn with ensures ✅
- [x] Add `is_exhausted_spec` — attempt >= max_attempts ✅
- [x] Add `is_exhausted` exec fn with ensures ✅
- [x] Proof: `exhausted_iff_zero_remaining` — is_exhausted ⟺ remaining == 0 ✅
- [x] Proof: `remaining_monotonic` — attempt+1 ⟹ remaining decreases or stays 0 ✅
- [x] Proof: `should_retry_iff_not_exhausted` — should_retry(a, m, true) ⟺ !is_exhausted(a, m) ✅
- [x] Verus + cargo test green ✅

## Phase 3: Retry budget

- [x] Add `retry_budget_ok_spec` — failures * 100 <= total * budget_pct (empty = ok) ✅
- [x] Add `retry_budget_ok` exec fn with overflow-guarded ensures ✅
- [x] Proof: `empty_window_always_ok` — successes == 0 && failures == 0 ⟹ budget ok ✅
- [x] Proof: `zero_failures_always_ok` — failures == 0 ⟹ budget ok ✅
- [x] Proof: `full_budget_always_ok` — budget_pct >= 100 ⟹ budget ok ✅
- [x] Verus + cargo test green ✅

## Phase 4: Outcome classification

- [x] Add SUCCESS/RETRY/FAIL constants ✅
- [x] Add `classify_outcome_spec` — exhaustive mapping to 0/1/2 ✅
- [x] Add `classify_outcome` exec fn with ensures ✅
- [x] Proof: `classify_is_valid` — result is always 0, 1, or 2 ✅
- [x] Proof: `success_always_success` — succeeded ⟹ classify == SUCCESS ✅
- [x] Proof: `non_retryable_never_retry` — !is_retryable ⟹ classify != RETRY ✅
- [x] Proof: `classify_agrees_with_should_retry` — classify == RETRY ⟺ should_retry ✅
- [x] Verus + cargo test green ✅

## Phase 5: Tests and cross-module

- [x] Unit tests for all 5 exec fns ✅
- [x] Proptest: should_retry agrees with is_exhausted ✅
- [x] Proptest: classify exhaustive (every input maps to valid outcome) ✅
- [x] Proptest: budget monotonic (fewer failures ⟹ still ok) ✅
- [x] Cross-module property: should_retry && circuit CLOSED ⟹ allow_request ✅
- [x] Cross-module property: should_retry ⟹ backoff bounded by cap ✅
- [x] Cross-module property: classify == RETRY ⟹ attempt < max_attempts ✅
- [x] Verus + cargo test green ✅
