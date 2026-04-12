## Phase 1: Field Arithmetic

- [x] Implement `shamir_field.rs` — field_add, field_sub, field_mul with closure proofs (result < p)
- [x] Implement extended GCD for field_inv with loop invariant and termination proof
- [x] Prove commutativity and identity laws for add and mul
- [x] Prove inverse correctness: a * inv(a) ≡ 1 (mod p) for non-zero a
- [x] Add proptest checks against known field arithmetic test vectors

## Phase 2: Polynomial Evaluation

- [x] Implement `shamir_poly.rs` — spec fn poly_eval_sum (summation definition)
- [x] Implement exec fn poly_eval_horner with ensures matching poly_eval_sum
- [x] Prove constant and linear polynomial base cases
- [x] Prove Horner-summation equivalence by induction on coefficient count
- [x] Add proptest checks: evaluate random polynomials both ways, compare results

## Phase 3: Lagrange Interpolation

- [x] Implement `shamir_lagrange.rs` — Lagrange basis function L_i(x) as spec fn
- [x] Prove Kronecker delta property: L_i(x_j) = δ_{ij}
- [x] Prove interpolation correctness: interpolant passes through all given points
- [x] Prove uniqueness: two degree-<k polynomials agreeing on k points are identical
- [x] Implement exec fn lagrange_at_zero for reconstruction use

## Phase 4: Split and Reconstruct

- [x] Implement `shamir_split.rs` — generate n shares from secret + coefficients + evaluation points
- [x] Prove share count equals n and all x-coordinates are distinct and non-zero
- [x] Prove constant term preservation: polynomial at x=0 equals the secret
- [x] Implement `shamir_reconstruct.rs` — reconstruct secret from k shares via lagrange_at_zero
- [x] Prove round-trip: reconstruct(split(s, k, ...)) == s for any k-subset of shares
- [x] Add parameter validation: require 2 ≤ k ≤ n, reject degenerate cases

## Phase 5: Threshold Security and Integration

- [x] Implement `shamir_threshold.rs` — proof that for any k-1 shares and any secret s', a consistent polynomial exists
- [x] Prove the construction: given k-1 points and a target constant term, exhibit the polynomial
- [x] Wire all modules into lib.rs with module declarations
- [x] Run full verus verification pass — all proofs green
- [x] Run tracey query uncovered — all requirements annotated
