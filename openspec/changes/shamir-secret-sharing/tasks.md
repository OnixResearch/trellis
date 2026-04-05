## Phase 1: Field Arithmetic

- [ ] Implement `shamir_field.rs` — field_add, field_sub, field_mul with closure proofs (result < p)
- [ ] Implement extended GCD for field_inv with loop invariant and termination proof
- [ ] Prove commutativity and identity laws for add and mul
- [ ] Prove inverse correctness: a * inv(a) ≡ 1 (mod p) for non-zero a
- [ ] Add proptest checks against known field arithmetic test vectors

## Phase 2: Polynomial Evaluation

- [ ] Implement `shamir_poly.rs` — spec fn poly_eval_sum (summation definition)
- [ ] Implement exec fn poly_eval_horner with ensures matching poly_eval_sum
- [ ] Prove constant and linear polynomial base cases
- [ ] Prove Horner-summation equivalence by induction on coefficient count
- [ ] Add proptest checks: evaluate random polynomials both ways, compare results

## Phase 3: Lagrange Interpolation

- [ ] Implement `shamir_lagrange.rs` — Lagrange basis function L_i(x) as spec fn
- [ ] Prove Kronecker delta property: L_i(x_j) = δ_{ij}
- [ ] Prove interpolation correctness: interpolant passes through all given points
- [ ] Prove uniqueness: two degree-<k polynomials agreeing on k points are identical
- [ ] Implement exec fn lagrange_at_zero for reconstruction use

## Phase 4: Split and Reconstruct

- [ ] Implement `shamir_split.rs` — generate n shares from secret + coefficients + evaluation points
- [ ] Prove share count equals n and all x-coordinates are distinct and non-zero
- [ ] Prove constant term preservation: polynomial at x=0 equals the secret
- [ ] Implement `shamir_reconstruct.rs` — reconstruct secret from k shares via lagrange_at_zero
- [ ] Prove round-trip: reconstruct(split(s, k, ...)) == s for any k-subset of shares
- [ ] Add parameter validation: require 2 ≤ k ≤ n, reject degenerate cases

## Phase 5: Threshold Security and Integration

- [ ] Implement `shamir_threshold.rs` — proof that for any k-1 shares and any secret s', a consistent polynomial exists
- [ ] Prove the construction: given k-1 points and a target constant term, exhibit the polynomial
- [ ] Wire all modules into lib.rs with module declarations
- [ ] Run full verus verification pass — all proofs green
- [ ] Run tracey query uncovered — all requirements annotated
