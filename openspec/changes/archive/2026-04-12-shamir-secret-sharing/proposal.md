## Why

Shamir's Secret Sharing is a foundational cryptographic primitive: split a
secret into n shares such that any k of them reconstruct the secret, but
k-1 reveal nothing. It underpins key escrow, multi-party computation,
threshold signatures, and distributed key management. The math is
polynomial interpolation over a finite field — self-contained, algebraic,
and a natural fit for Verus verification.

The existing verified-logic library has no secret sharing or finite field
polynomial modules. Adding Shamir's scheme fills that gap and provides
reusable building blocks (field arithmetic, polynomial evaluation, Lagrange
interpolation) that other cryptographic modules can use.

## What Changes

- **shamir_field**: Modular arithmetic over GF(p) — addition, multiplication,
  and multiplicative inverse via extended Euclidean algorithm. Proves field
  axioms: closure, associativity, commutativity, identity, inverse existence.

- **shamir_poly**: Polynomial representation and evaluation over GF(p).
  Horner's method evaluation with a proof that it matches the summation
  definition. Degree bound enforcement.

- **shamir_lagrange**: Lagrange basis polynomials and interpolation.
  Proves the partition-of-unity property, interpolation correctness (the
  interpolated polynomial passes through all given points), and uniqueness
  (any two polynomials of degree < k agreeing on k points are identical).

- **shamir_split**: Share generation — evaluate a random polynomial of
  degree k-1 (with the secret as constant term) at n distinct non-zero
  points. Proves share count correctness and that the constant term is
  preserved.

- **shamir_reconstruct**: Reconstruction via Lagrange interpolation at
  x=0. Proves that any k valid shares recover the original secret.

- **shamir_threshold**: Threshold property — k-1 shares are
  information-theoretically independent of the secret. Modeled as: for any
  secret s and any set of k-1 shares, there exists a valid polynomial
  with constant term s consistent with those shares.

## Capabilities

### New Capabilities

- `shamir-field`: Verified finite field arithmetic (mod p)
- `shamir-poly`: Verified polynomial evaluation with Horner's method
- `shamir-lagrange`: Verified Lagrange interpolation with uniqueness proof
- `shamir-split`: Verified share generation with count and constant-term preservation
- `shamir-reconstruct`: Verified secret reconstruction from k shares
- `shamir-threshold`: Verified threshold security property

## Impact

- **Files**: Six new source files in `src/` (shamir_field.rs, shamir_poly.rs,
  shamir_lagrange.rs, shamir_split.rs, shamir_reconstruct.rs,
  shamir_threshold.rs), module declarations in lib.rs
- **APIs**: New public types `FieldElement`, `Polynomial`, `Share` and
  functions `split`, `reconstruct`
- **Dependencies**: None beyond vstd
- **Testing**: Verus proofs for all correctness properties, proptest for
  runtime sanity checks against known test vectors
