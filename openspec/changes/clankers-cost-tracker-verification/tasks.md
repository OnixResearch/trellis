## Phase 1: Cost Accumulation

- [ ] Add cost-tracker requirements to specs
- [ ] Implement `compute_cost_spec` — `(tokens * price_microdollars) / 1_000_000`
- [ ] Implement `compute_cost` exec function with overflow check
- [ ] Prove cost non-negative: `price >= 0 ==> compute_cost(tokens, price) >= 0`
- [ ] Implement `accumulate_spec` — `total_before + turn_cost`
- [ ] Implement `accumulate` exec function
- [ ] Prove monotonic: `accumulate(total, cost) >= total` when `cost >= 0`
- [ ] Prove additive: `accumulate(total, cost) == total + cost`
- [ ] Unit tests + proptest (random token counts and prices)
- [ ] Verus + tracey clean

## Phase 2: Threshold Crossing Detection

- [ ] Implement `crosses_threshold_spec` — `prev < limit && current >= limit`
- [ ] Implement `crosses_threshold` exec function
- [ ] Prove fires on crossing: `prev < limit && current >= limit ==> fires`
- [ ] Prove silent when already crossed: `prev >= limit ==> !fires`
- [ ] Prove silent when not reached: `current < limit ==> !fires`
- [ ] Prove exactly-once over monotonic sequence: for a sorted sequence and fixed limit, at most one adjacent pair crosses
- [ ] Implement `milestone_count_spec` — `floor(current / interval) - floor(prev / interval)`
- [ ] Implement `milestone_count` exec function
- [ ] Prove count non-negative: `current >= prev && interval > 0 ==> count >= 0`
- [ ] Prove count zero when within same interval: `floor(current / interval) == floor(prev / interval) ==> count == 0`
- [ ] Prove count matches iteration: count equals number of integers `m` where `prev < m * interval <= current`
- [ ] Unit tests + proptest (random prev/current/interval triples)
- [ ] Verus + tracey clean

## Phase 3: Budget Status State Machine

- [ ] Define `BudgetStatus` enum: `NoBudget`, `Ok`, `Warning`, `Exceeded`
- [ ] Implement `budget_status_spec` — 4-arm match on `(soft, hard)` × comparison on `total`
- [ ] Implement `budget_status` exec function
- [ ] Prove exhaustive: every `(soft, hard, total)` maps to exactly one state
- [ ] Prove NoBudget: `soft == None && hard == None ==> NoBudget`
- [ ] Prove Ok region: `total < min_defined_limit ==> Ok`
- [ ] Prove Warning region: `soft <= total < hard ==> Warning` (when both defined)
- [ ] Prove Exceeded region: `total >= hard ==> Exceeded` (when hard defined)
- [ ] Prove monotone: if status at `t1` is `Warning`, status at `t2 >= t1` is `Warning` or `Exceeded`
- [ ] Prove boundary alignment: `Warning` starts exactly at soft, `Exceeded` starts exactly at hard
- [ ] Unit tests + proptest (random soft/hard/total triples)
- [ ] Verus + tracey clean

## Phase 4: Model Selection Priority

- [ ] Implement `select_role_spec` — priority chain: hard budget → hint → score → thresholds
- [ ] Implement `select_role` exec function
- [ ] Prove hard budget wins: `cost >= hard_limit ==> result == smol` regardless of hint/score
- [ ] Prove user hint overrides score: `cost < hard_limit && hint.is_some() ==> result follows hint`
- [ ] Prove score thresholds partition: `score < low ==> smol`, `score > high ==> slow`, else `default`
- [ ] Prove soft budget demotes: `cost >= soft_limit ==> adjusted_score <= score`
- [ ] Prove soft budget never promotes: `adjusted_score <= score` (score / 2 <= score for non-negative)
- [ ] Implement `complexity_score_spec` — weighted sum of token contribution + tool weights + keyword weights
- [ ] Prove score non-negative: all weights >= 0 && all keyword_weights >= 0 ==> score >= 0
- [ ] Unit tests + proptest (random signals, random configs)
- [ ] Verus + tracey clean

## Phase 5: Cross-module and Integration

- [ ] Link cost monotonicity to `monotonic` module patterns
- [ ] Link threshold crossing to `bounded_range` containment
- [ ] Link budget status to `circuit_breaker` state machine pattern
- [ ] Add cross-module property tests in `src/properties.rs`
- [ ] Final `verus` pass on full module
- [ ] Final `tracey query uncovered` — zero uncovered requirements
