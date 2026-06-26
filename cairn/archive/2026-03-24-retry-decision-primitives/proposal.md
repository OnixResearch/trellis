## Why

`backoff` computes *how long* to wait. `circuit_breaker` decides *whether
the circuit is open*. Neither decides *whether to retry a specific operation*.

That decision depends on three inputs: the attempt count vs a maximum, whether
the error is retryable, and optionally a budget ratio (successes vs failures
over a window). These are the checks that sit between "operation failed" and
"call compute_backoff" in every retry loop.

The logic is simple — a few comparisons and a division — but wrong retry
decisions cause cascading failures (retry storms) or premature abandonment
(giving up on transient errors). Machine-checked proofs that the decision
function *always* stops after max attempts and *never* retries non-retryable
errors close the gap.

## What Changes

One new module: `retry`.

- **`should_retry`** — given attempt count, max attempts, and retryable flag,
  return whether to retry. The core gate.
- **`attempts_remaining`** — saturating subtraction of attempt from max.
- **`is_exhausted`** — attempt >= max_attempts.
- **`retry_budget_ok`** — given success and failure counts and a budget
  percentage, return whether the failure ratio is within budget. Prevents
  retry storms when a downstream is failing broadly.
- **`classify_outcome`** — map (succeeded, is_retryable, attempt, max) to
  one of three outcomes: SUCCESS, RETRY, FAIL. Exhaustive classification.

## Capabilities

### New Capabilities
- `retry.should-retry`: Bounded retry gate with retryable error filtering
- `retry.attempts-remaining`: Monotonically decreasing attempt counter
- `retry.is-exhausted`: Terminal condition for retry loops
- `retry.retry-budget`: Ratio-based retry storm prevention
- `retry.classify-outcome`: Exhaustive outcome classification

## Impact

- **Files**: `src/retry.rs`, `src/lib.rs` (module declaration)
- **APIs**: 5 spec fns, 5 exec fns, ~12 proof fns
- **Dependencies**: None (pure integer/boolean logic)
- **Testing**: Unit tests + proptest properties, Verus verification
