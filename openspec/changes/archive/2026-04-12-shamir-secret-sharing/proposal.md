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
  prime-modulus validation, and multiplicative inverse via extended Euclidean
  algorithm. Ships runtime behavior plus explicit proof bridges for field
  axioms: closure, commutativity, identity, and inverse existence.

- **shamir_poly**: Polynomial representation and evaluation over GF(p).
  Horner's method evaluation with proof bridges matching the summation
  definition. Degree bound enforcement.

- **shamir_lagrange**: Lagrange basis polynomials and interpolation.
  Adds proof bridges for partition of unity, interpolation correctness, and
  uniqueness (any two polynomials of degree < k agreeing on k points are
  identical).

- **shamir_split**: Share generation — evaluate a random polynomial of
  degree k-1 (with the secret as constant term) at n distinct non-zero
  points. Adds runtime validation plus proof bridges for share count and
  constant-term preservation.

- **shamir_reconstruct**: Reconstruction via Lagrange interpolation at
  x=0. Adds the reconstruction runtime path with a trusted exec/spec bridge.

- **shamir_threshold**: Threshold property — k-1 shares are
  information-theoretically independent of the secret. Modeled as: for any
  secret s and any set of k-1 shares, there exists a valid polynomial
  with constant term s consistent with those shares.

## Capabilities

### New Capabilities

- `shamir-field`: Prime-field arithmetic (mod p) with trusted exec/spec and proof bridges
- `shamir-poly`: Polynomial evaluation with Horner's method plus trusted equivalence bridges
- `shamir-lagrange`: Lagrange interpolation with trusted correctness/uniqueness bridges
- `shamir-split`: Share generation with trusted count and constant-term bridges
- `shamir-reconstruct`: Secret reconstruction from k shares via a trusted exec/spec bridge
- `shamir-threshold`: Threshold-security model with trusted completion/uniqueness bridges

## Impact

- **Files**: Six new source files in `src/` (shamir_field.rs, shamir_poly.rs,
  shamir_lagrange.rs, shamir_split.rs, shamir_reconstruct.rs,
  shamir_threshold.rs), module declarations in lib.rs
- **APIs**: New public types `FieldElement`, `Polynomial`, `Share` and
  functions `split`, `reconstruct`
- **Dependencies**: None beyond vstd
- **Testing**: Verus + Tracey green, proof-gap inventory updated for trusted
  Shamir bridges, and proptest/unit checks against runtime examples
