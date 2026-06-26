# Job Execution Verification Specification

## Purpose

Defines correctness properties for job execution verification logic:
retry decisions with error classification, deadline enforcement with
grace periods, and multi-criteria result validation. These are pure
functions with no I/O — the imperative executor calls them for
decisions during and after job execution.

## Requirements

### Job Retry

#### Requirement: Retry Eligibility

`should_retry` MUST return true only when ALL of these hold:
1. The error code is classified as retryable
2. The attempt count is less than max_attempts
3. The cooldown period has elapsed since the last attempt

##### Scenario: Retryable error with attempts remaining

- GIVEN error_code is retryable, attempts=2, max_attempts=5, cooldown elapsed
- WHEN checking should_retry
- THEN MUST return true

##### Scenario: Non-retryable error

- GIVEN error_code is non-retryable, attempts=0, max_attempts=5
- WHEN checking should_retry
- THEN MUST return false

##### Scenario: Max attempts reached

- GIVEN error_code is retryable, attempts=5, max_attempts=5
- WHEN checking should_retry
- THEN MUST return false

##### Scenario: Cooldown not elapsed

- GIVEN error_code is retryable, attempts=1, max_attempts=5, cooldown NOT elapsed
- WHEN checking should_retry
- THEN MUST return false

#### Requirement: Error Classification

`is_retryable_error` MUST classify error codes into retryable (transient)
and non-retryable (permanent) categories. Error codes 0-99 are
non-retryable (permanent failures). Error codes 100-199 are retryable
(transient failures). Error codes >= 200 are non-retryable (unknown).

##### Scenario: Permanent failure

- GIVEN error_code = 42
- WHEN classifying
- THEN MUST return false (non-retryable)

##### Scenario: Transient failure

- GIVEN error_code = 150
- WHEN classifying
- THEN MUST return true (retryable)

##### Scenario: Unknown error

- GIVEN error_code = 250
- WHEN classifying
- THEN MUST return false (non-retryable)

#### Requirement: Attempts Remaining

`attempts_remaining` MUST return max_attempts - attempts when
attempts < max_attempts, and 0 otherwise. The result MUST never
be negative.

##### Scenario: Some attempts left

- GIVEN attempts=2, max_attempts=5
- WHEN computing remaining
- THEN MUST return 3

##### Scenario: No attempts left

- GIVEN attempts=5, max_attempts=5
- WHEN computing remaining
- THEN MUST return 0

##### Scenario: Over max (defensive)

- GIVEN attempts=7, max_attempts=5
- WHEN computing remaining
- THEN MUST return 0

#### Requirement: Cooldown Check

`is_cooldown_elapsed` MUST return true when `now >= last_attempt_time + cooldown_ms`.
The addition MUST NOT overflow — use saturating arithmetic.

##### Scenario: Cooldown passed

- GIVEN last_attempt=1000, cooldown=500, now=1600
- WHEN checking cooldown
- THEN MUST return true

##### Scenario: Still in cooldown

- GIVEN last_attempt=1000, cooldown=500, now=1200
- WHEN checking cooldown
- THEN MUST return false

##### Scenario: Exactly at boundary

- GIVEN last_attempt=1000, cooldown=500, now=1500
- WHEN checking cooldown
- THEN MUST return true (>= boundary)

#### Requirement: Next Attempt Number

`next_attempt` MUST return attempts + 1 when retry is eligible,
capped at max_attempts. MUST NOT overflow.

##### Scenario: Normal increment

- GIVEN attempts=2, max_attempts=5
- WHEN computing next attempt
- THEN MUST return 3

##### Scenario: At max

- GIVEN attempts=5, max_attempts=5
- WHEN computing next attempt
- THEN MUST return 5 (capped)

### Job Timeout

#### Requirement: Deadline Computation

`compute_deadline` MUST return start_time + timeout_ms. The addition
MUST use saturating arithmetic — if the sum overflows u64, return
u64::MAX.

##### Scenario: Normal deadline

- GIVEN start_time=1000, timeout_ms=5000
- WHEN computing deadline
- THEN MUST return 6000

##### Scenario: Overflow saturation

- GIVEN start_time=u64::MAX - 100, timeout_ms=200
- WHEN computing deadline
- THEN MUST return u64::MAX

#### Requirement: Timeout Detection

`is_timed_out` MUST return true when `now >= deadline`. Once a job
times out, it stays timed out for all future time values (monotonic).

##### Scenario: Not yet timed out

- GIVEN now=5000, deadline=6000
- WHEN checking timeout
- THEN MUST return false

##### Scenario: At deadline

- GIVEN now=6000, deadline=6000
- WHEN checking timeout
- THEN MUST return true

##### Scenario: Past deadline

- GIVEN now=7000, deadline=6000
- WHEN checking timeout
- THEN MUST return true

#### Requirement: Time Remaining

`time_remaining` MUST return deadline - now when now < deadline,
and 0 when now >= deadline. The result MUST never be negative.

##### Scenario: Time left

- GIVEN now=4000, deadline=6000
- WHEN computing remaining
- THEN MUST return 2000

##### Scenario: At deadline

- GIVEN now=6000, deadline=6000
- WHEN computing remaining
- THEN MUST return 0

##### Scenario: Past deadline

- GIVEN now=7000, deadline=6000
- WHEN computing remaining
- THEN MUST return 0

#### Requirement: Deadline Extension

`extend_deadline` MUST add a grace period to an existing deadline,
returning the new deadline. The extension MUST be bounded: the new
deadline MUST NOT exceed original_deadline + max_extension. Uses
saturating arithmetic.

##### Scenario: Normal extension

- GIVEN deadline=6000, grace=1000, max_extension=5000
- WHEN extending
- THEN MUST return 7000

##### Scenario: Extension capped

- GIVEN deadline=6000, grace=8000, max_extension=5000
- WHEN extending
- THEN MUST return 11000 (6000 + 5000, capped by max)

##### Scenario: Overflow saturation

- GIVEN deadline=u64::MAX - 100, grace=200, max_extension=u64::MAX
- WHEN extending
- THEN MUST return u64::MAX

#### Requirement: Elapsed Time

`elapsed_time` MUST return now - start_time when now >= start_time,
and 0 otherwise. The result MUST never be negative.

##### Scenario: Normal elapsed

- GIVEN start_time=1000, now=3500
- WHEN computing elapsed
- THEN MUST return 2500

##### Scenario: Clock not advanced

- GIVEN start_time=1000, now=1000
- WHEN computing elapsed
- THEN MUST return 0

### Job Result

#### Requirement: Exit Code Classification

`is_success_code` MUST return true for exit code 0 only.
`is_failure_code` MUST return true for exit codes 1-127.
`is_error_code` MUST return true for exit codes >= 128.
These three ranges MUST be mutually exclusive and exhaustive.

##### Scenario: Success

- GIVEN exit_code = 0
- WHEN classifying
- THEN is_success=true, is_failure=false, is_error=false

##### Scenario: Failure

- GIVEN exit_code = 1
- WHEN classifying
- THEN is_success=false, is_failure=true, is_error=false

##### Scenario: Signal/Error

- GIVEN exit_code = 137
- WHEN classifying
- THEN is_success=false, is_failure=false, is_error=true

#### Requirement: Output Hash Validation

`output_hash_matches` MUST return true when the computed hash of
the output matches the expected hash. Both are u64 values (pre-hashed).

##### Scenario: Matching hash

- GIVEN computed=0xABCD, expected=0xABCD
- WHEN validating
- THEN MUST return true

##### Scenario: Mismatched hash

- GIVEN computed=0xABCD, expected=0x1234
- WHEN validating
- THEN MUST return false

#### Requirement: Output Size Validation

`output_size_valid` MUST return true when
`min_size <= actual_size <= max_size`.

##### Scenario: Within bounds

- GIVEN actual=500, min=100, max=1000
- WHEN validating
- THEN MUST return true

##### Scenario: Too small

- GIVEN actual=50, min=100, max=1000
- WHEN validating
- THEN MUST return false

##### Scenario: Too large

- GIVEN actual=1500, min=100, max=1000
- WHEN validating
- THEN MUST return false

#### Requirement: Combined Validation

`validate_result` MUST check all three criteria: exit code is success,
output hash matches, and output size is within bounds. MUST return a
status indicating which criterion failed (if any): 0=all pass,
1=bad exit code, 2=bad hash, 3=bad size. Returns the FIRST failing
criterion.

##### Scenario: All pass

- GIVEN exit_code=0, hash matches, size valid
- WHEN validating
- THEN MUST return 0

##### Scenario: Bad exit code

- GIVEN exit_code=1, hash matches, size valid
- WHEN validating
- THEN MUST return 1

##### Scenario: Good exit but bad hash

- GIVEN exit_code=0, hash mismatch, size valid
- WHEN validating
- THEN MUST return 2

##### Scenario: Good exit and hash but bad size

- GIVEN exit_code=0, hash matches, size too large
- WHEN validating
- THEN MUST return 3

#### Requirement: Validation Determinism

For the same inputs, `validate_result` MUST always return the same
status code. Two calls with identical arguments MUST produce
identical results.

##### Scenario: Repeated validation

- GIVEN identical inputs
- WHEN calling validate_result twice
- THEN both MUST return the same status
