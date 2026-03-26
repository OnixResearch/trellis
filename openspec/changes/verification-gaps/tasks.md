## Phase 1: Rate limit nonlinear arithmetic

- [x] Close `tokens_after_refill` — u128 intermediate pattern ✅
- [x] Close `tokens_to_wait_ms` — u128 intermediate pattern ✅
- [x] Close `wait_enables_consumption` — strengthened requires with non-saturation guard ✅
  - Added `tokens_to_wait_ms_spec(tokens, cost, rate_per_sec) < u64::MAX` to requires
  - Proof: ceiling property (fundamental_div_mod) → division monotonicity (div_is_ordered) → refill ≥ shortage
  - Saturation case (wait_ms == u64::MAX) excluded — counterexample: tokens=0, cost=u64::MAX, rate=1
- [x] Verus + cargo test green ✅ (3077 verified, 0 errors; 20/20 rate_limit tests pass)

## Phase 2: Glob recursive spec

- [ ] Replace degenerate `glob_match_spec` with recursive definition
- [ ] Add `glob_segments_spec` helper for multi-segment matching
- [ ] Close `glob_match` exec fn with loop invariant
- [ ] Add proof fns for key structural cases
- [ ] Verus + cargo test green

## Phase 3: Replace strings

- [ ] Open `replace_strings_spec` — recursive left-to-right scan
- [ ] Add `first_match_spec` helper for pattern priority
- [ ] Close `replace_strings` exec fn
- [ ] Verus + cargo test green

## Phase 4: Document permanent gaps

- [ ] Add `@verified-logic:permanent-axiom` markers to crypto items (6)
- [ ] Add `@verified-logic:permanent-z3-limit` markers to version items (4)
- [ ] Add `@verified-logic:permanent-theory-bridge` marker to buddy (1)
- [ ] Update docs/requirements.md with verification gap inventory
- [ ] Archive change
