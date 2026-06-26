## Context

The verified-logic library proves algebraic properties with Verus. Shamir's
Secret Sharing requires finite field arithmetic, polynomial evaluation, and
Lagrange interpolation — all deterministic, bounded, and well-suited to SMT
reasoning. The scheme works over GF(p) for a prime p. No floating point, no
unbounded allocation, no external dependencies.

## Goals / Non-Goals

**Goals:**
- Prime-field arithmetic over a checked prime modulus
- Polynomial evaluation matching the summation definition
- Lagrange interpolation passing through given points
- Split/reconstruct round-trip
- Threshold security property (k-1 shares → all secrets consistent)
- Explicit accounting of any remaining trusted Verus bridges in the proof-gap inventory

**Non-Goals:**
- Runtime random polynomial generation (caller supplies coefficients)
- Specific prime selection or parameter negotiation
- Serialization or wire format for shares
- Side-channel resistance (constant-time is out of scope for logic proofs)
- Arbitrary-precision integers (use u64 with prime < 2^32 to avoid overflow)

## Decisions

### 1. Field Representation

**Choice:** `u64` values with a `u64` prime modulus, prime constrained to fit
in `u32` so that `a * b` never overflows `u64`.

**Rationale:** Verus reasons about `u64` arithmetic natively. Keeping the
prime under 2^32 means multiplication of two field elements (each < 2^32)
fits in 64 bits without widening. This avoids the need for multi-word
arithmetic while still supporting primes large enough for meaningful
secret sharing (e.g., Mersenne prime 2^31 - 1).

**Alternative:** `u128` with larger primes — rejected because Verus support
for `u128` is less mature and the proof overhead isn't justified for a
logic library.

**Implementation:** All field functions take an explicit `p: u64` parameter
with `requires valid_modulus_spec(p)` where `valid_modulus_spec` means prime
and `< 0x1_0000_0000`. The runtime validator checks primality by trial
division and rejects composite moduli. Results carry `ensures result < p`.

### 2. Polynomial Representation

**Choice:** `Vec<u64>` of coefficients in ascending degree order:
`coeffs[i]` is the coefficient of x^i. The length determines the degree
bound.

**Rationale:** Ascending order matches the mathematical summation
Σ c_i * x^i directly. Verus can index into `Vec` and reason about length.

**Implementation:** A polynomial of degree k-1 has `coeffs.len() == k`.
The constant term (secret) is `coeffs[0]`.

### 3. Evaluation Strategy

**Choice:** Horner's method with a loop, plus a ghost proof that it equals
the summation form.

**Rationale:** Horner's method is O(n) multiplications with no
exponentiation. The equivalence proof to the summation form is a
standard induction on coefficient count that Verus handles well.

**Implementation:**
```
result = 0
for i in (0..coeffs.len()).rev() {
    result = (result * x + coeffs[i]) % p
}
```
Ghost spec function `poly_eval_sum` defines the summation form. The exec
function's ensures clause states equality.

### 4. Lagrange Interpolation

**Choice:** Direct Lagrange formula evaluated at x=0 only (for
reconstruction). A general-evaluation form is provided as a spec function
for proof purposes.

**Rationale:** Reconstruction only needs the interpolant at x=0, which
simplifies the formula. The basis polynomial L_i(0) = Π_{j≠i} (0 - x_j) /
(x_i - x_j) = Π_{j≠i} (-x_j) / (x_i - x_j). Proving correctness of the
full Lagrange formula and then specializing to x=0 keeps the proof modular.

**Implementation:** `reconstruct` takes a slice of `(u64, u64)` pairs
(x, y) and the prime p. Computes Σ y_i * L_i(0) mod p.

### 5. Multiplicative Inverse

**Choice:** Extended Euclidean algorithm (iterative) in runtime code.

**Rationale:** Fermat's little theorem (a^{p-2} mod p) requires
exponentiation with a loop and exponent reasoning. Extended GCD is a
standard loop with a well-known termination argument (gcd decreases).
The current implementation uses the runtime algorithm directly and keeps
any remaining proof debt explicit in the verification-gap inventory.

**Alternative:** Fermat's little theorem — rejected due to more complex
loop invariant for modular exponentiation.

### 6. Module Structure

**Choice:** Six modules, one per concern, mirroring the spec sections.

```
shamir_field.rs       — field ops: add, sub, mul, inv
shamir_poly.rs        — polynomial eval (Horner), spec eval (summation)
shamir_lagrange.rs    — basis functions, interpolation, uniqueness proof
shamir_split.rs       — split(secret, k, coefficients, xs, p) → shares
shamir_reconstruct.rs — reconstruct(shares, p) → secret
shamir_threshold.rs   — proof that k-1 shares leave secret unconstrained
```

**Rationale:** Matches the project's one-module-per-concept pattern. Each
module's proof obligations are self-contained. `shamir_split` and
`shamir_reconstruct` compose the lower-level modules.

### 7. Share Type

**Choice:** A share is a `(u64, u64)` tuple — (x-coordinate, y-coordinate).
No wrapper struct.

**Rationale:** Keeps the API minimal. Verus reasons about tuples without
needing custom struct invariants. The x-coordinate is the evaluation point
(non-zero, distinct); the y-coordinate is the polynomial value at that point.

## Risks / Trade-offs

**[Prime size limits practical use]** → The u32 prime constraint caps
secrets at ~31 bits. Acceptable for a logic library — real deployments
would use big-integer fields, but the algebraic properties proved here
transfer.

**[No randomness in split]** → The caller must supply random coefficients.
This is deliberate: verified-logic is a pure logic library, not a
cryptographic implementation. The spec still proves that *if* coefficients
are chosen, the scheme is correct.

**[Trusted proof bridges remain]** → The shipped implementation is
behaviorally green but still carries trusted `external_body` bridges in the
Shamir modules. Those sites are listed explicitly in
`openspec/specs/verification-gaps/spec.md` and remain closeable proof debt.
