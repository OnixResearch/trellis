## Phase 1: Arithmetic Helpers

- [ ] Define `u64_wrapping_add` spec function using nat modular arithmetic
- [ ] Define `u64_saturating_add` spec function with clamp at u64::MAX
- [ ] Define `u64_wrapping_neg` spec function (two's complement negation)
- [ ] Prove wrapping add commutativity via `by(bit_vector)`
- [ ] Prove wrapping add identity (addend 0)
- [ ] Prove wrapping add overflow case (u64::MAX + 1 == 0)
- [ ] Prove saturating add clamp correctness
- [ ] Prove saturating add idempotent at u64::MAX
- [ ] Write exec-level wrapping/saturating add functions with ensures

## Phase 2: Core FAA Operations

- [ ] Define `faa_wrapping_spec` returning `(old, wrapping_add(old, addend))`
- [ ] Define `faa_saturating_spec` returning `(old, saturating_add(old, addend))`
- [ ] Write exec-level `fetch_add_wrapping` and `fetch_add_saturating` functions
- [ ] Prove return semantics: returned value equals old cell content
- [ ] Prove identity: `faa(c, 0) == (c, c)` for both variants

## Phase 3: Algebraic Properties

- [ ] Prove wrapping commutativity: `faa(faa(c,a).1, b).1 == faa(faa(c,b).1, a).1`
- [ ] Prove saturating commutativity
- [ ] Prove wrapping associativity: `faa(faa(c,a).1, b).1 == faa(c, wrapping_add(a,b)).1`
- [ ] Document saturating non-associativity with counterexample proof
- [ ] Prove monotonicity: `a > 0 ∧ c + a ≤ MAX → faa(c,a).1 > c`

## Phase 4: CAS Refinement and Fetch-Sub

- [ ] Define `faa_via_cas` spec using `cas_result_spec` from `cas_loop.rs`
- [ ] Prove `faa_via_cas(c, a) == faa_wrapping_spec(c, a)`
- [ ] Define `fetch_sub_wrapping_spec` as `faa_wrapping_spec(c, wrapping_neg(s))`
- [ ] Prove fetch-sub duality: sub equals add with negation
- [ ] Prove add-then-sub round-trip restores original value
- [ ] Prove sub-then-add round-trip restores original value

## Phase 5: Tests

- [ ] Unit tests for wrapping add edge cases (0, MAX, overflow)
- [ ] Unit tests for saturating add edge cases (0, MAX, clamp)
- [ ] Unit tests for FAA return semantics (wrapping and saturating)
- [ ] Unit tests for fetch-sub round-trip
- [ ] Proptest: wrapping commutativity for random a, b, c
- [ ] Proptest: wrapping associativity for random a, b, c
- [ ] Proptest: add-sub round-trip for random c, n
- [ ] Proptest: CAS refinement equivalence for random c, a
- [ ] Proptest: saturating add never exceeds u64::MAX
