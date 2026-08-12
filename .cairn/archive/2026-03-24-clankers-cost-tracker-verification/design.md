## Context

The clankers cost tracker (`clankers-model-selection/src/cost_tracker.rs`)
and routing policy (`policy.rs`) implement budget enforcement and
complexity-based model selection. The cost tracker wraps a
`HashMap<String, ModelUsage>` behind a `RwLock`, computes turn costs
from a pricing table, and emits budget events. The routing policy is
a priority-ordered decision function.

The verified-logic crate already has `arithmetic` (overflow-safe
operations), `bounded_range`, and `circuit_breaker` (state machine
transitions).

## Goals / Non-Goals

**Goals:**
- Verify cost accumulation is monotonically non-decreasing
- Verify threshold events fire exactly once per crossing
- Verify milestone count matches floor-division formula
- Verify budget status exhaustively partitions the cost range
- Verify model selection priority ordering (hard budget > hint > score)
- Verify complexity score is non-negative under non-negative weights

**Non-Goals:**
- Verify floating-point precision (Verus treats f64 axiomatically;
  we prove properties over idealized reals or integers)
- Verify HashMap/RwLock concurrency (runtime concern)
- Verify pricing table loading from JSON (I/O)
- Verify prefix-matching model ID lookup
- Verify TUI status line formatting

## Decisions

### 1. Integer Tokens, Rational Cost

**Choice:** Token counts are `u64`. Prices are represented as
integers in "micro-dollars per token" or similar fixed-point
encoding to avoid floating-point reasoning in proofs.

**Rationale:** The clankers code uses `f64` for prices and costs.
Verus has limited support for floating-point reasoning — Z3 treats
`f64` operations as uninterpreted unless you add axioms. The
correctness properties we care about (monotonicity, threshold
crossing, milestone counting) depend on ordered comparisons, not
precise decimal values.

By encoding costs as `u64` microdollars (1 microdollar = $0.000001),
we keep full integer precision. The conversion is:
`cost_microdollars = tokens * price_microdollars_per_mtok / 1_000_000`.
This is integer arithmetic with a known divisor.

**Alternative:** Use Verus `f64` with explicit rounding axioms.
Rejected — fragile and doesn't buy us anything for the properties
we're proving.

**Implementation:** Spec functions use `int` (Verus arbitrary
precision integers). Exec functions use `u64` with overflow checks.
A separate property test validates that the `u64` encoding matches
the `f64` computation within acceptable tolerance.

### 2. Threshold Detection as Pure Predicate

**Choice:** `crosses_threshold(prev, current, limit) -> bool`
is defined as `prev < limit && current >= limit`. This is a spec
function with no state.

**Rationale:** The clankers code stores `prev_total` in a RwLock
and updates it on each `record_usage`. The stateful tracking is a
runtime concern. The mathematical property (fires exactly once at
the crossing point) is a predicate over three numbers.

**Implementation:** The proof shows: for any sequence of totals
`t0 <= t1 <= t2 <= ...` and a fixed limit `L`, exactly one pair
`(t_{k}, t_{k+1})` satisfies `t_k < L <= t_{k+1}` (if any pair
does). This follows from monotonicity.

### 3. Milestone Count as Floor Division

**Choice:** `milestone_count(prev, current, interval) -> int`
is defined as `floor(current / interval) - floor(prev / interval)`.

**Rationale:** The clankers code iterates from
`(prev / interval).floor() + 1` to `(current / interval).floor()`.
The count is the difference of the two floor values. This is a
closed-form formula that's easy to reason about.

**Implementation:** Spec uses integer division (Verus `int` division
truncates toward zero for non-negative operands). Proof shows the
count is non-negative when `current >= prev` and `interval > 0`.

### 4. Budget Status as State Machine

**Choice:** Budget status is modeled as an enum with four states.
The transition function takes `(soft: Option<u64>, hard: Option<u64>,
total: u64)` and returns the state.

**Rationale:** The clankers code has a 4-arm match on
`(soft_limit, hard_limit)`. Each arm has 2-3 if/else branches
on `total`. Modeling this as a state machine makes the coverage
proof explicit: we show every point on the `total` number line
maps to exactly one state, and increasing `total` only moves
forward (NoBudget is static, Ok → Warning → Exceeded).

**Implementation:** Enum with `NoBudget`, `Ok`, `Warning`, `Exceeded`.
Proof by case analysis on the four `(soft, hard)` combinations,
then ordered comparison within each case.

### 5. Model Selection as Priority Chain

**Choice:** The routing policy's `select_model` is modeled as a
priority chain: check hard budget first, then user hint, then
complexity score. Each check either returns a result or falls
through to the next.

**Rationale:** The clankers code is structured as sequential
if-statements with early returns. The priority ordering is the
key invariant: hard budget must override user hints. Modeling it
as a chain makes the priority proof a sequence of implications.

**Implementation:** Spec function with three nested `if` expressions.
Proof shows: if `cost >= hard_limit`, the result is always "smol"
regardless of the other inputs. If `cost < hard_limit` and hint
is present, the result follows the hint. Otherwise, score thresholds
apply.

### 6. Soft Budget as Score Modifier

**Choice:** Soft budget pressure is modeled as a score multiplier:
if `cost >= soft_limit`, the complexity score is halved before
threshold comparison.

**Rationale:** The clankers code does `score * 0.5` when over the
soft limit. In our integer model, this becomes `score / 2` (integer
division). The effect is that tasks that would normally route to
"slow" get demoted to "default" or "smol".

**Implementation:** The proof shows: soft budget never promotes
(score / 2 <= score for non-negative score), only demotes.

## Risks / Trade-offs

**[Integer cost model diverges from f64 reality]** → For very large
token counts, integer microdollars might overflow `u64`. Mitigation:
`u64` max is ~$18 trillion in microdollars. No session will hit that.
Overflow checks in the exec function catch it.

**[Milestone count with f64 interval has rounding edge cases]** →
The floor-division formula is clean for integers but f64 division
near interval boundaries could yield off-by-one milestones.
Mitigation: we prove the integer version is correct. A proptest
validates that the integer encoding matches the f64 computation.

**[Score non-negativity depends on keyword weights]** → If a user
configures negative keyword weights (e.g., "simple" → -8.0), the
score can go negative. The clankers code allows this (it's
intentional — negative keywords bias toward cheaper models).
Mitigation: we prove non-negativity conditional on non-negative
weights, and document that negative weights are a user-controlled
escape hatch.
