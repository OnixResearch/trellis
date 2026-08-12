# Cost Tracker Specification

## Purpose

Defines the correctness properties for per-model cost accumulation,
budget threshold detection, budget status classification, and model
selection priority ordering. Extracted from `clankers-model-selection`.

## Definitions

- **Turn cost**: `(input_tokens / 1,000,000) × input_price +
  (output_tokens / 1,000,000) × output_price` for a single API call.
- **Running total**: Cumulative sum of all turn costs across all models.
- **Soft limit**: A cost threshold that triggers a warning but does
  not restrict model selection.
- **Hard limit**: A cost threshold that forces downgrade to the
  cheapest model.
- **Milestone**: A cost interval (e.g., every $1.00) that triggers
  a notification when crossed.
- **Previous total**: The running total before the most recent
  `record_usage` call, used for threshold crossing detection.

## Requirements

### Requirement: Cost Non-negative

`compute_cost(tokens, price)` MUST return a non-negative value when
`price >= 0`.

#### Scenario: Zero tokens

- GIVEN input_tokens = 0 and any non-negative price
- WHEN computing cost
- THEN the result is 0

#### Scenario: Positive tokens and price

- GIVEN input_tokens = 1,000,000 and price = 3.0
- WHEN computing cost
- THEN the result is 3.0

### Requirement: Cost Monotonically Non-decreasing

The running total after `record_usage` MUST be greater than or equal
to the running total before the call.

`total_after >= total_before` for all non-negative prices.

#### Scenario: Sequential usage

- GIVEN a running total of $5.00
- WHEN recording 100K tokens at $3/MTok
- THEN the new total is >= $5.00

### Requirement: Cost Additive

The running total after recording usage MUST equal the previous total
plus the turn cost.

`total_after == total_before + turn_cost`

#### Scenario: Two recordings

- GIVEN initial total 0, first turn costs $1.00, second costs $2.00
- WHEN both are recorded
- THEN the final total is $3.00

### Requirement: Threshold Fires Exactly Once

A budget event (Warning or Exceeded) MUST fire exactly once: when
`prev_total < limit` and `new_total >= limit`.

It MUST NOT fire if `prev_total >= limit` (already crossed).

It MUST NOT fire if `new_total < limit` (not yet crossed).

#### Scenario: First crossing

- GIVEN soft_limit = $5.00, prev_total = $4.00, new_total = $6.00
- WHEN checking thresholds
- THEN a Warning event fires

#### Scenario: Already crossed

- GIVEN soft_limit = $5.00, prev_total = $6.00, new_total = $8.00
- WHEN checking thresholds
- THEN no Warning event fires

#### Scenario: Not yet crossed

- GIVEN soft_limit = $5.00, prev_total = $2.00, new_total = $3.00
- WHEN checking thresholds
- THEN no Warning event fires

### Requirement: Milestone Count Correct

The number of Milestone events in one `record_usage` call MUST
equal `floor(new_total / interval) - floor(prev_total / interval)`.

#### Scenario: Crosses three milestones

- GIVEN interval = $1.00, prev_total = $0.50, new_total = $3.20
- WHEN checking thresholds
- THEN 3 Milestone events fire (at $1, $2, $3)

#### Scenario: No milestone crossed

- GIVEN interval = $1.00, prev_total = $1.20, new_total = $1.80
- WHEN checking thresholds
- THEN 0 Milestone events fire

### Requirement: Milestone Non-negative

The milestone count MUST be non-negative (zero or more events).

#### Scenario: Monotonic total

- GIVEN any non-negative prev_total and new_total >= prev_total
- WHEN computing milestone count
- THEN the count is >= 0

### Requirement: Budget Status Exhaustive

`compute_budget_status(soft, hard, total)` MUST return exactly one
of four states for any combination of `(soft, hard, total)`:

- `NoBudget` — when both soft and hard are None
- `Ok` — when total is below the first defined limit
- `Warning` — when total is at or above soft but below hard
- `Exceeded` — when total is at or above hard

#### Scenario: No limits set

- GIVEN soft = None, hard = None
- WHEN computing status for any total
- THEN the result is NoBudget

#### Scenario: Between soft and hard

- GIVEN soft = Some(5.0), hard = Some(10.0), total = 7.0
- WHEN computing status
- THEN the result is Warning

#### Scenario: Over hard

- GIVEN soft = Some(5.0), hard = Some(10.0), total = 12.0
- WHEN computing status
- THEN the result is Exceeded

#### Scenario: Under soft

- GIVEN soft = Some(5.0), hard = Some(10.0), total = 3.0
- WHEN computing status
- THEN the result is Ok

#### Scenario: Only hard set, under it

- GIVEN soft = None, hard = Some(10.0), total = 5.0
- WHEN computing status
- THEN the result is Ok

#### Scenario: Only hard set, over it

- GIVEN soft = None, hard = Some(10.0), total = 15.0
- WHEN computing status
- THEN the result is Exceeded

### Requirement: Budget Status Monotone

Increasing the total MUST NOT move the status backward. If status
at total `t1` is Warning, then status at total `t2 >= t1` MUST
be Warning or Exceeded, not Ok.

#### Scenario: Warning stays Warning or advances

- GIVEN status is Warning at total $7.00
- WHEN total increases to $8.00
- THEN status is Warning or Exceeded

### Requirement: Hard Budget Overrides User Hint

Model selection MUST return the cheapest role when `cost >= hard_limit`,
regardless of any user hint.

#### Scenario: User requests expensive model but over budget

- GIVEN hard_limit = $5.00, current_cost = $6.00, user_hint = Thorough
- WHEN selecting model
- THEN the result is the cheapest role ("smol")

### Requirement: User Hint Overrides Complexity Score

When cost is under hard limit, a user hint MUST override the
complexity score.

#### Scenario: Low complexity but user wants thorough

- GIVEN token_count = 50 (low), user_hint = Thorough, no hard limit hit
- WHEN selecting model
- THEN the result is "slow" (thorough)

### Requirement: Score Thresholds Partition Number Line

The three roles (smol, default, slow) MUST partition the score
range with no gaps:
- `score < low_threshold` → smol
- `low_threshold <= score <= high_threshold` → default
- `score > high_threshold` → slow

#### Scenario: Score at boundary

- GIVEN low_threshold = 20, score = 20
- WHEN selecting by score
- THEN the result is "default" (not "smol")

### Requirement: Complexity Score Non-negative

When all weights are non-negative and all keyword weights are
non-negative, the complexity score MUST be non-negative.

#### Scenario: All positive inputs

- GIVEN positive token_weight, tool_weight, and keyword weights
- WHEN computing complexity score
- THEN the result is >= 0
