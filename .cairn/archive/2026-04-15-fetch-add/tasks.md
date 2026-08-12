## Phase 1: Arithmetic Helpers

- [x] Define `u64_wrapping_add` spec function using nat modular arithmetic ✅
- [x] Define `u64_saturating_add` spec function with clamp at u64::MAX ✅
- [x] Define `u64_wrapping_neg` spec function (two's complement negation) ✅
- [x] Prove wrapping add commutativity via `by(bit_vector)` ✅
- [x] Prove wrapping add identity (addend 0) ✅
- [x] Prove wrapping add overflow case (u64::MAX + 1 == 0) ✅
- [x] Prove saturating add clamp correctness ✅
- [x] Prove saturating add idempotent at u64::MAX ✅
- [x] Write exec-level wrapping/saturating add functions with ensures ✅

## Phase 2: Core FAA Operations

- [x] Define `faa_wrapping_spec` returning `(old, wrapping_add(old, addend))` ✅
- [x] Define `faa_saturating_spec` returning `(old, saturating_add(old, addend))` ✅
- [x] Write exec-level `fetch_add_wrapping` and `fetch_add_saturating` functions ✅
- [x] Prove return semantics: returned value equals old cell content ✅
- [x] Prove identity: `faa(c, 0) == (c, c)` for both variants ✅

## Phase 3: Algebraic Properties

- [x] Prove wrapping commutativity: `faa(faa(c,a).1, b).1 == faa(faa(c,b).1, a).1` ✅
- [x] Prove saturating commutativity ✅
- [x] Prove wrapping associativity: `faa(faa(c,a).1, b).1 == faa(c, wrapping_add(a,b)).1` ✅
- [x] Prove saturating associativity: `faa(faa(c,a).1, b).1 == faa(c, saturating_add(a,b)).1` ✅
- [x] Prove monotonicity: `a > 0 ∧ c + a ≤ MAX → faa(c,a).1 > c` ✅

## Phase 4: CAS Refinement and Fetch-Sub

- [x] Define `faa_via_cas` spec using `cas_result_spec` from `cas_loop.rs` ✅
- [x] Prove `faa_via_cas(c, a) == faa_wrapping_spec(c, a)` ✅
- [x] Define `fetch_sub_wrapping_spec` as `faa_wrapping_spec(c, wrapping_neg(s))` ✅
- [x] Prove fetch-sub duality: sub equals add with negation ✅
- [x] Prove add-then-sub round-trip restores original value ✅
- [x] Prove sub-then-add round-trip restores original value ✅

## Phase 5: Tests

- [x] Unit tests for wrapping add edge cases (0, MAX, overflow) ✅
- [x] Unit tests for saturating add edge cases (0, MAX, clamp) ✅
- [x] Unit tests for FAA return semantics (wrapping and saturating) ✅
- [x] Unit tests for fetch-sub round-trip ✅
- [x] Proptest: wrapping commutativity for random a, b, c ✅
- [x] Proptest: wrapping associativity for random a, b, c ✅
- [x] Proptest: add-sub round-trip for random c, n ✅
- [x] Proptest: CAS refinement equivalence for random c, a ✅
- [x] Proptest: saturating add never exceeds u64::MAX ✅
