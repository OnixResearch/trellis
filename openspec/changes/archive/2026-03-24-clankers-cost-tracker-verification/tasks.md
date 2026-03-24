## Phase 1: Cost Accumulation

- [x] Add cost-tracker requirements to specs ✅
- [x] Implement `compute_cost_spec` — `(tokens * price_microdollars) / 1_000_000` ✅
- [x] Implement `compute_cost` exec function with overflow check ✅
- [x] Prove cost non-negative: `price >= 0 ==> compute_cost(tokens, price) >= 0` ✅
- [x] Implement `accumulate_spec` — `total_before + turn_cost` ✅
- [x] Implement `accumulate` exec function ✅
- [x] Prove monotonic: `accumulate(total, cost) >= total` when `cost >= 0` ✅
- [x] Prove additive: `accumulate(total, cost) == total + cost` ✅
- [x] Unit tests + proptest (random token counts and prices) ✅
- [x] Verus + tracey clean ✅

## Phase 2: Threshold Crossing Detection

- [x] Implement `crosses_threshold_spec` — `prev < limit && current >= limit` ✅
- [x] Implement `crosses_threshold` exec function ✅
- [x] Prove fires on crossing: `prev < limit && current >= limit ==> fires` ✅
- [x] Prove silent when already crossed: `prev >= limit ==> !fires` ✅
- [x] Prove silent when not reached: `current < limit ==> !fires` ✅
- [x] Prove exactly-once over monotonic sequence: for a sorted sequence and fixed limit, at most one adjacent pair crosses ✅
- [x] Implement `milestone_count_spec` — `floor(current / interval) - floor(prev / interval)` ✅
- [x] Implement `milestone_count` exec function ✅
- [x] Prove count non-negative: `current >= prev && interval > 0 ==> count >= 0` ✅
- [x] Prove count zero when within same interval: `floor(current / interval) == floor(prev / interval) ==> count == 0` ✅
- [x] Prove count matches iteration: milestone_count_additive proves count(a,c) == count(a,b) + count(b,c) ✅
- [x] Unit tests + proptest (random prev/current/interval triples) ✅
- [x] Verus + tracey clean ✅

## Phase 3: Budget Status State Machine

- [x] Define `BudgetStatus` enum: `NoBudget`, `Ok`, `Warning`, `Exceeded` (u8 constants for Verus compat) ✅
- [x] Implement `budget_status_spec` — 4-arm match on `(soft, hard)` × comparison on `total` ✅
- [x] Implement `budget_status` exec function ✅
- [x] Prove exhaustive: every `(soft, hard, total)` maps to exactly one state ✅
- [x] Prove NoBudget: `soft == None && hard == None ==> NoBudget` ✅
- [x] Prove Ok region: `total < min_defined_limit ==> Ok` ✅
- [x] Prove Warning region: `soft <= total < hard ==> Warning` (when both defined) ✅
- [x] Prove Exceeded region: `total >= hard ==> Exceeded` (when hard defined) ✅
- [x] Prove monotone: if status at `t1` is `Warning`, status at `t2 >= t1` is `Warning` or `Exceeded` ✅
- [x] Prove boundary alignment: `Warning` starts exactly at soft, `Exceeded` starts exactly at hard ✅
- [x] Unit tests + proptest (random soft/hard/total triples) ✅
- [x] Verus + tracey clean ✅

## Phase 4: Model Selection Priority

- [x] Implement `select_role_spec` — priority chain: hard budget → hint → score → thresholds ✅
- [x] Implement `select_role` exec function ✅
- [x] Prove hard budget wins: `cost >= hard_limit ==> result == smol` regardless of hint/score ✅
- [x] Prove user hint overrides score: `cost < hard_limit && hint.is_some() ==> result follows hint` ✅
- [x] Prove score thresholds partition: `score < low ==> smol`, `score > high ==> slow`, else `default` ✅
- [x] Prove soft budget demotes: `cost >= soft_limit ==> adjusted_score <= score` ✅
- [x] Prove soft budget never promotes: `adjusted_score <= score` (score / 2 <= score for non-negative) ✅
- [x] Implement `complexity_score_spec` — weighted sum of token contribution + tool weights + keyword weights ✅
- [x] Prove score non-negative: all weights >= 0 && all keyword_weights >= 0 ==> score >= 0 ✅
- [x] Unit tests + proptest (random signals, random configs) ✅
- [x] Verus + tracey clean ✅

## Phase 5: Cross-module and Integration

- [x] Link cost monotonicity to `monotonic` module patterns (accumulate_is_advance) ✅
- [x] Link threshold crossing irreversibility (threshold_crossing_irreversible) ✅
- [x] Link budget status absorbing property (budget_exceeded_stays_exceeded) ✅
- [x] Add cross-module property tests in `src/properties.rs` ✅
- [x] Final `verus` pass on full module — 1047 verified, 0 errors ✅
- [x] Final cargo test — 708 passed, 0 failed ✅
