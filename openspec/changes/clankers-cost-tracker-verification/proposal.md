## Why

Clankers tracks per-model token usage and enforces budget limits via
`CostTracker`. Users set soft limits (warn) and hard limits (force
downgrade to cheaper model). The routing policy consults
`budget_status()` on every turn to decide whether to override the
complexity-based model selection.

Three pieces of logic are correctness-critical:

1. **Cost accumulation** — `record_usage` computes
   `(input_tokens / 1M) × input_price + (output_tokens / 1M) × output_price`,
   adds it to a running total, and returns the cumulative cost.
   The total must be monotonically non-decreasing. A bug here means
   the budget status bar lies to the user about their spend.

2. **Threshold crossing detection** — budget events (Warning, Exceeded,
   Milestone) must fire exactly once when the total crosses a boundary.
   The current implementation tracks `prev_total` and compares against
   the new total. A double-fire means duplicate warnings in the TUI.
   A missed fire means the user hits a hard limit without warning.

3. **Budget status state machine** — `compute_budget_status` maps
   `(soft_limit, hard_limit, total)` to one of four states:
   `NoBudget`, `Ok`, `Warning`, `Exceeded`. The four arms of the
   match must partition the cost number line correctly. A gap means
   a total in the "between soft and hard" range returns `Ok` instead
   of `Warning`.

The routing policy also has a `select_model` function that is a
priority-ordered decision chain: hard budget → user hint → complexity
score → threshold bucketing. The priority ordering has a key invariant:
hard budget overrides user hints (you can't `use opus` when you're
over budget).

All of this is pure arithmetic and comparisons. No I/O, no async,
no string processing beyond model ID lookup. The cost calculation is
fixed-point-style (integer token counts × floating-point price per
million), and the threshold logic is ordered comparisons with a
`prev_total` latch.

## What Changes

A new `budget` module in verified-logic that extracts the cost
accumulation, threshold detection, budget status, and model selection
priority logic from `clankers-model-selection`.

**Cost accumulation**

- `compute_cost_spec` — `(tokens / 1M) × price`
- `accumulate_spec` — add turn cost to running total
- Proofs: result is non-negative when price is non-negative,
  total is monotonically non-decreasing, cost is additive

**Threshold crossing**

- `crosses_threshold_spec` — `prev < limit ≤ current`
- `milestone_count_spec` — number of milestones crossed in one step:
  `floor(current / interval) - floor(prev / interval)`
- Proofs: each threshold fires exactly once (if `prev < limit` and
  `current >= limit`), milestone count is non-negative, zero
  milestones when current < interval

**Budget status**

- `budget_status_spec` — maps `(soft, hard, total)` to status enum
- Proofs: exhaustive coverage (every total maps to exactly one
  status), monotone transitions (increasing total never moves
  from Exceeded back to Ok), status boundary alignment (Warning
  starts exactly at soft, Exceeded starts exactly at hard)

**Model selection priority**

- `select_role_spec` — priority chain: hard budget → user hint →
  score → threshold
- Proofs: hard budget always wins (if cost ≥ hard, result is smol
  regardless of hint or score), user hint overrides score, score
  thresholds partition the number line

## Capabilities

### New Capabilities

- `cost-monotonic`: Running total never decreases
- `cost-non-negative`: Cost is non-negative when prices are
- `threshold-exactly-once`: Budget events fire exactly once per crossing
- `milestone-count-correct`: Milestone count matches floor division
- `budget-status-exhaustive`: Every total maps to exactly one status
- `budget-status-monotone`: Status transitions follow total ordering
- `select-priority-order`: Hard budget > user hint > score > default

## Impact

- **Files**: 1 new `src/budget.rs` module
- **APIs**: ~12–15 pub fns (spec + exec pairs)
- **Dependencies**: None — pure arithmetic on `int`, `u64`, `f64`
  (Verus integer reasoning for thresholds, axiomatized float for
  cost calculation where exact precision matters less)
- **Testing**: Verus verification + proptest with random usage sequences
