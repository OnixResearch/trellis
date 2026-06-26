# Retry Decision Specification

## Purpose

Defines the pure decision logic for operation retry: whether to retry,
how many attempts remain, whether attempts are exhausted, budget-based
retry storm prevention, and exhaustive outcome classification.

## Requirements

### Requirement: Should Retry

The system MUST return true iff the error is retryable AND the attempt
count is strictly less than max_attempts.

#### Scenario: Retryable within budget
- GIVEN attempt = 2, max_attempts = 5, is_retryable = true
- WHEN should_retry is called
- THEN result is true

#### Scenario: Non-retryable error
- GIVEN attempt = 0, max_attempts = 5, is_retryable = false
- WHEN should_retry is called
- THEN result is false

#### Scenario: Exhausted attempts
- GIVEN attempt = 5, max_attempts = 5, is_retryable = true
- WHEN should_retry is called
- THEN result is false

### Requirement: Attempts Remaining

The system MUST return max_attempts - attempt when attempt < max_attempts,
and 0 otherwise (saturating subtraction).

#### Scenario: Some remaining
- GIVEN attempt = 3, max_attempts = 10
- WHEN attempts_remaining is called
- THEN result is 7

#### Scenario: Exhausted
- GIVEN attempt = 10, max_attempts = 10
- WHEN attempts_remaining is called
- THEN result is 0

### Requirement: Is Exhausted

The system MUST return true iff attempt >= max_attempts.

#### Scenario: Not exhausted
- GIVEN attempt = 2, max_attempts = 5
- WHEN is_exhausted is called
- THEN result is false

#### Scenario: Exactly at max
- GIVEN attempt = 5, max_attempts = 5
- WHEN is_exhausted is called
- THEN result is true

### Requirement: Retry Budget

The system MUST return true iff the failure count is within the budget
percentage of total operations. When total is zero, the budget is ok
(no evidence of overload).

#### Scenario: Within budget
- GIVEN successes = 90, failures = 10, budget_pct = 20
- WHEN retry_budget_ok is called
- THEN result is true (10% failures < 20% budget)

#### Scenario: Over budget
- GIVEN successes = 50, failures = 50, budget_pct = 20
- WHEN retry_budget_ok is called
- THEN result is false (50% failures > 20% budget)

#### Scenario: Empty window
- GIVEN successes = 0, failures = 0, budget_pct = 20
- WHEN retry_budget_ok is called
- THEN result is true

### Requirement: Classify Outcome

The system MUST map every (succeeded, is_retryable, attempt, max_attempts)
tuple to exactly one of SUCCESS (0), RETRY (1), or FAIL (2).

#### Scenario: Success
- GIVEN succeeded = true
- WHEN classify_outcome is called
- THEN result is SUCCESS (0)

#### Scenario: Retryable failure with budget
- GIVEN succeeded = false, is_retryable = true, attempt = 2, max = 5
- WHEN classify_outcome is called
- THEN result is RETRY (1)

#### Scenario: Non-retryable failure
- GIVEN succeeded = false, is_retryable = false, attempt = 0, max = 5
- WHEN classify_outcome is called
- THEN result is FAIL (2)

#### Scenario: Exhausted retryable
- GIVEN succeeded = false, is_retryable = true, attempt = 5, max = 5
- WHEN classify_outcome is called
- THEN result is FAIL (2)
