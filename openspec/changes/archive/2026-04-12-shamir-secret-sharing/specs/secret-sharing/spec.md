# Secret Sharing Specification

## Purpose

Defines correctness requirements for Shamir's Secret Sharing scheme:
finite field arithmetic, polynomial evaluation, Lagrange interpolation,
share generation, secret reconstruction, and the threshold security
property.

## Requirements

### Field Arithmetic

#### Requirement: Prime Modulus Validation

Shamir field arithmetic MUST operate over GF(p) for a prime modulus p.
Composite moduli MUST be rejected.

##### Scenario: Composite modulus rejected

- GIVEN a composite modulus such as 15
- WHEN validating the modulus for Shamir field operations
- THEN the operation MUST reject it as not being a field modulus

#### Requirement: Closure

Addition and multiplication over GF(p) MUST produce results in the
range [0, p). No operation may return a value outside the field.

##### Scenario: Addition closure

- GIVEN field elements a, b in [0, p)
- WHEN computing field_add(a, b, p)
- THEN the result MUST be in [0, p)

##### Scenario: Multiplication closure

- GIVEN field elements a, b in [0, p)
- WHEN computing field_mul(a, b, p)
- THEN the result MUST be in [0, p)

#### Requirement: Additive Identity

Zero MUST be the additive identity. Adding zero to any element MUST
return that element unchanged.

##### Scenario: Right identity

- GIVEN a field element a
- WHEN computing field_add(a, 0, p)
- THEN the result MUST equal a

##### Scenario: Left identity

- GIVEN a field element a
- WHEN computing field_add(0, a, p)
- THEN the result MUST equal a

#### Requirement: Multiplicative Identity

One MUST be the multiplicative identity. Multiplying any element by one
MUST return that element unchanged.

##### Scenario: Right identity

- GIVEN a field element a
- WHEN computing field_mul(a, 1, p)
- THEN the result MUST equal a

##### Scenario: Left identity

- GIVEN a field element a
- WHEN computing field_mul(1, a, p)
- THEN the result MUST equal a

#### Requirement: Commutativity

Both addition and multiplication MUST be commutative.

##### Scenario: Addition commutes

- GIVEN field elements a and b
- WHEN computing field_add(a, b, p) and field_add(b, a, p)
- THEN both results MUST be equal

##### Scenario: Multiplication commutes

- GIVEN field elements a and b
- WHEN computing field_mul(a, b, p) and field_mul(b, a, p)
- THEN both results MUST be equal

#### Requirement: Multiplicative Inverse

Every non-zero element MUST have a multiplicative inverse. The inverse
MUST satisfy a * inv(a) ≡ 1 (mod p).

##### Scenario: Inverse exists

- GIVEN a non-zero field element a
- WHEN computing field_inv(a, p)
- THEN field_mul(a, field_inv(a, p), p) MUST equal 1

##### Scenario: Zero has no inverse

- GIVEN the element 0
- WHEN attempting field_inv(0, p)
- THEN the operation MUST fail or return None

#### Requirement: Subtraction

Field subtraction MUST satisfy a - b ≡ a + (p - b) (mod p). The result
MUST be in [0, p).

##### Scenario: Subtraction as additive inverse

- GIVEN field elements a and b
- WHEN computing field_sub(a, b, p)
- THEN field_add(result, b, p) MUST equal a

### Polynomial Evaluation

#### Requirement: Horner Correctness

Evaluating a polynomial via Horner's method MUST produce the same
result as the summation definition: f(x) = Σ coeffs[i] * x^i (mod p).

##### Scenario: Constant polynomial

- GIVEN polynomial with coefficients [c]
- WHEN evaluating at any x
- THEN the result MUST equal c mod p

##### Scenario: Linear polynomial

- GIVEN polynomial with coefficients [a0, a1]
- WHEN evaluating at x
- THEN the result MUST equal (a0 + a1 * x) mod p

##### Scenario: Equivalence with summation

- GIVEN polynomial with coefficients [c0, c1, ..., cn]
- WHEN evaluating at x using Horner's method
- THEN the result MUST equal Σ ci * x^i (mod p)

#### Requirement: Degree Bound

A polynomial with k coefficients MUST have degree at most k-1. The
leading coefficient (coeffs[k-1]) MAY be zero.

##### Scenario: Degree

- GIVEN a polynomial with 3 coefficients
- WHEN querying its degree
- THEN the degree MUST be at most 2

### Lagrange Interpolation

#### Requirement: Interpolation Correctness

Given k points (x_i, y_i) with distinct x_i, the Lagrange interpolant
evaluated at any x_j MUST return y_j. The interpolant passes through
all given points.

##### Scenario: Passes through all points

- GIVEN k points (x_1, y_1), ..., (x_k, y_k) with distinct x values
- WHEN computing the Lagrange interpolant L(x)
- THEN L(x_i) MUST equal y_i for all i in 1..k

#### Requirement: Uniqueness

Any two polynomials of degree less than k that agree on k distinct
points MUST be identical. There is exactly one polynomial of degree < k
passing through k points.

##### Scenario: Two polynomials agreeing on k points

- GIVEN polynomials P and Q both of degree < k
- WHEN P(x_i) == Q(x_i) for k distinct x values
- THEN P and Q MUST have identical coefficients

#### Requirement: Basis Partition of Unity

The Lagrange basis polynomials L_i(x) MUST satisfy Σ L_i(x) = 1 for
all x, when evaluated at the interpolation points.

##### Scenario: Sum of basis at interpolation point

- GIVEN basis polynomials L_1, ..., L_k for points x_1, ..., x_k
- WHEN evaluating Σ L_i(x_j) for any j
- THEN the sum MUST equal 1

#### Requirement: Basis Kronecker Delta

Each basis polynomial L_i MUST satisfy L_i(x_j) = δ_{ij}: equals 1
when i = j, equals 0 otherwise.

##### Scenario: Diagonal

- GIVEN basis polynomial L_i for point x_i
- WHEN evaluating L_i(x_i)
- THEN the result MUST equal 1

##### Scenario: Off-diagonal

- GIVEN basis polynomial L_i for point x_i and a different point x_j (j ≠ i)
- WHEN evaluating L_i(x_j)
- THEN the result MUST equal 0

### Share Generation

#### Requirement: Share Count

Splitting a secret with parameters (k, n) MUST produce exactly n
shares. Each share MUST consist of a distinct non-zero x-coordinate
and a y-coordinate in [0, p).

##### Scenario: Correct count

- GIVEN threshold k and total n
- WHEN splitting a secret
- THEN exactly n shares MUST be produced

##### Scenario: Distinct x-coordinates

- GIVEN n shares from a split operation
- WHEN examining x-coordinates
- THEN all x-coordinates MUST be distinct and non-zero

#### Requirement: Parameter Validation

Split MUST require 2 ≤ k ≤ n. k = 1 is degenerate (every share is the
secret). k > n makes reconstruction impossible.

##### Scenario: Valid parameters

- GIVEN k = 3 and n = 5
- WHEN splitting
- THEN the operation MUST succeed

##### Scenario: k greater than n

- GIVEN k = 5 and n = 3
- WHEN attempting to split
- THEN the operation MUST fail

##### Scenario: k equals 1

- GIVEN k = 1
- WHEN attempting to split
- THEN the operation MUST fail (degenerate case)

#### Requirement: Constant Term Preservation

The polynomial used for share generation MUST have the secret as its
constant term (coefficient at index 0). Evaluating at x=0 MUST return
the secret.

##### Scenario: Polynomial at zero

- GIVEN a secret s and a polynomial f constructed for splitting
- WHEN evaluating f(0)
- THEN the result MUST equal s

### Secret Reconstruction

#### Requirement: Reconstruction Correctness

Given any k or more valid shares from the same split operation,
reconstruction MUST return the original secret.

##### Scenario: Exact threshold

- GIVEN k shares from a (k, n) split of secret s
- WHEN reconstructing
- THEN the result MUST equal s

##### Scenario: More than threshold

- GIVEN k+1 shares from a (k, n) split of secret s
- WHEN reconstructing
- THEN the result MUST equal s

##### Scenario: Any k-subset

- GIVEN any k-element subset of n shares from a split of secret s
- WHEN reconstructing from that subset
- THEN the result MUST equal s

#### Requirement: Insufficient Shares

Reconstruction with fewer than k shares MUST NOT produce the correct
secret (except with negligible probability 1/p).

##### Scenario: One share short

- GIVEN k-1 shares from a (k, n) split
- WHEN attempting reconstruction
- THEN the result SHOULD NOT equal the original secret

### Threshold Security

#### Requirement: Information-Theoretic Security

Any set of fewer than k shares MUST be information-theoretically
independent of the secret. For any secret s' and any k-1 shares,
there MUST exist a polynomial of degree k-1 with constant term s'
consistent with those shares.

##### Scenario: Any secret consistent with k-1 shares

- GIVEN k-1 shares (x_1, y_1), ..., (x_{k-1}, y_{k-1})
- WHEN choosing an arbitrary secret s'
- THEN there MUST exist a polynomial f of degree k-1 such that f(0) = s' and f(x_i) = y_i for all i

##### Scenario: No leakage from single share

- GIVEN a single share (x, y) from a (k, n) split where k ≥ 2
- WHEN considering all possible secrets in [0, p)
- THEN every secret value MUST be equally consistent with that share

#### Requirement: Perfect Reconstruction Threshold

Exactly k shares MUST be both necessary and sufficient. Fewer than k
reveal nothing; k or more uniquely determine the secret.

##### Scenario: Sharp threshold

- GIVEN a (k, n) split
- WHEN any k shares are combined
- THEN exactly one polynomial of degree < k fits those points
- AND its constant term is the secret
