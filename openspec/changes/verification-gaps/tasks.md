## Phase 1: Rate limit nonlinear arithmetic

- [x] Close `tokens_after_refill` — u128 intermediate pattern ✅
- [x] Close `tokens_to_wait_ms` — u128 intermediate pattern ✅
- [x] Close `wait_enables_consumption` — strengthened requires with non-saturation guard ✅
  - Added `tokens_to_wait_ms_spec(tokens, cost, rate_per_sec) < u64::MAX` to requires
  - Proof: ceiling property (fundamental_div_mod) → division monotonicity (div_is_ordered) → refill ≥ shortage
  - Saturation case (wait_ms == u64::MAX) excluded — counterexample: tokens=0, cost=u64::MAX, rate=1
- [x] Verus + cargo test green ✅ (3077 verified, 0 errors; 20/20 rate_limit tests pass)

## Phase 2: Glob recursive spec

- [x] Replace degenerate `glob_match_spec` with recursive definition ✅ (prior session)
- [x] Add `glob_segments_spec` helper for multi-segment matching ✅ (prior session)
- [x] Close `glob_match` exec fn — full rewrite with verified exec helpers ✅
  - 8 new exec helpers: has_star, bytes_eq, segment_end, substr_at, starts_with, ends_with, find_segment_in, slice_to_vec
  - build_segments: loop with deep_view+split_by_star_spec "done+rest=full" invariant
  - segments_match_exec: recursive exec mirroring segments_match_spec via Ghost<Seq<Seq<u8>>>
  - segment_end_bounded proof lemma (inductive, needed for slice bounds)
  - deep_view_push proof lemma (extensional equality for Vec push)
  - Removed find_substr_runtime (replaced by find_segment_in_exec inside verus!)
- [x] Proof fns for key structural cases ✅ (wildcard_matches_all, exact_match, exact_rejects, non_match)
- [x] Verus + cargo test green ✅ (3099 verified, 0 errors; 14/14 glob tests pass)

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
