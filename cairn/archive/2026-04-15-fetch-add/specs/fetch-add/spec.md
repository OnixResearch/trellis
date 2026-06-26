# Fetch-and-Add Specification

## Purpose

Defines requirements for verified atomic fetch-and-add (FAA) operations,
covering wrapping and saturating variants, algebraic properties, CAS
refinement, and fetch-sub duality. FAA is the unconditional atomic
read-modify-write primitive: it always succeeds, returns the old value,
and stores old + addend.

## Requirements

### Requirement: Return Semantics

FAA MUST return the value that was in the cell before the addition and
store the sum in the cell.

#### Scenario: Wrapping add

- GIVEN a cell containing value `c` and an addend `a`
- WHEN `fetch_add_wrapping(c, a)` is called
- THEN the returned value is `c` and the new cell content is `(c + a) mod 2^64`

#### Scenario: Saturating add

- GIVEN a cell containing value `c` and an addend `a`
- WHEN `fetch_add_saturating(c, a)` is called
- THEN the returned value is `c` and the new cell content is `min(c + a, u64::MAX)`

### Requirement: Identity

FAA with addend zero MUST be a no-op.

#### Scenario: Wrapping zero

- GIVEN any cell value `c`
- WHEN `fetch_add_wrapping(c, 0)` is called
- THEN the returned value is `c` and the new cell content is `c`

#### Scenario: Saturating zero

- GIVEN any cell value `c`
- WHEN `fetch_add_saturating(c, 0)` is called
- THEN the returned value is `c` and the new cell content is `c`

### Requirement: Commutativity

Two sequential FAAs MUST produce the same final cell value regardless of
the order the addends are applied.

#### Scenario: Wrapping commutativity

- GIVEN a cell containing `c`, addends `a` and `b`
- WHEN `fetch_add_wrapping(c, a)` then `fetch_add_wrapping(result, b)` is performed
- THEN the final cell value equals that of `fetch_add_wrapping(c, b)` then `fetch_add_wrapping(result, a)`

#### Scenario: Saturating commutativity

- GIVEN a cell containing `c`, addends `a` and `b`
- WHEN `fetch_add_saturating(c, a)` then `fetch_add_saturating(result, b)` is performed
- THEN the final cell value equals that of `fetch_add_saturating(c, b)` then `fetch_add_saturating(result, a)`

### Requirement: Associativity

Sequential `faa(a)` then `faa(b)` MUST produce the same final cell value
as a single `faa(a + b)` for both wrapping and saturating arithmetic.

#### Scenario: Wrapping associativity

- GIVEN a cell containing `c`, addends `a` and `b`
- WHEN `fetch_add_wrapping(c, a)` then `fetch_add_wrapping(mid, b)` is performed
- THEN the final cell value equals that of `fetch_add_wrapping(c, wrapping_add(a, b))`

#### Scenario: Saturating associativity

- GIVEN a cell containing `c`, addends `a` and `b`
- WHEN `fetch_add_saturating(c, a)` then `fetch_add_saturating(mid, b)` is performed
- THEN the final cell value equals that of `fetch_add_saturating(c, saturating_add(a, b))`

### Requirement: Monotonicity

When the addend is positive and no overflow occurs, the new cell value
MUST be strictly greater than the old value.

#### Scenario: No overflow

- GIVEN a cell containing `c` and addend `a > 0` where `c + a <= u64::MAX`
- WHEN `fetch_add_wrapping(c, a)` is called
- THEN the new cell value is strictly greater than `c`

#### Scenario: Overflow wraps

- GIVEN a cell containing `c` and addend `a` where `c + a > u64::MAX`
- WHEN `fetch_add_wrapping(c, a)` is called
- THEN the new cell value MAY be less than `c` (wrapping occurred)

### Requirement: CAS Refinement

FAA MUST be expressible as a CAS loop where the desired value is
`current + addend`, and the CAS always succeeds when expected equals
current.

#### Scenario: Single-step CAS equivalence

- GIVEN a cell containing `c` and addend `a`
- WHEN a CAS is attempted with expected=`c`, desired=`wrapping_add(c, a)`
- THEN the CAS succeeds and the result matches `fetch_add_wrapping(c, a)`

### Requirement: Wrapping Arithmetic Correctness

The wrapping add helper MUST satisfy modular arithmetic identities.

#### Scenario: Wrapping is commutative

- GIVEN any `a` and `b`
- WHEN `wrapping_add(a, b)` and `wrapping_add(b, a)` are computed
- THEN the results are equal

#### Scenario: Wrapping identity

- GIVEN any `a`
- WHEN `wrapping_add(a, 0)` is computed
- THEN the result is `a`

#### Scenario: Wrapping mod 2^64

- GIVEN `a = u64::MAX` and `b = 1`
- WHEN `wrapping_add(a, b)` is computed
- THEN the result is `0`

### Requirement: Saturating Arithmetic Correctness

The saturating add helper MUST clamp at `u64::MAX`.

#### Scenario: No saturation

- GIVEN `a` and `b` where `a + b <= u64::MAX`
- WHEN `saturating_add(a, b)` is computed
- THEN the result equals `a + b`

#### Scenario: Saturation clamp

- GIVEN `a` and `b` where `a + b > u64::MAX`
- WHEN `saturating_add(a, b)` is computed
- THEN the result is `u64::MAX`

#### Scenario: Idempotent at max

- GIVEN `a = u64::MAX` and any `b`
- WHEN `saturating_add(a, b)` is computed
- THEN the result is `u64::MAX`

### Requirement: Fetch-Sub Duality

Fetch-sub MUST be equivalent to FAA with the two's complement negation
of the subtrahend.

#### Scenario: Sub as negated add

- GIVEN a cell containing `c` and subtrahend `s`
- WHEN `fetch_sub_wrapping(c, s)` is called
- THEN the result equals `fetch_add_wrapping(c, wrapping_neg(s))`

#### Scenario: Add-sub round-trip

- GIVEN a cell containing `c` and a value `n` where `c + n <= u64::MAX`
- WHEN `fetch_add_wrapping(c, n)` then `fetch_sub_wrapping(result, n)` is performed
- THEN the final cell value equals the original `c`

#### Scenario: Sub-add round-trip

- GIVEN a cell containing `c` and a value `n` where `c >= n`
- WHEN `fetch_sub_wrapping(c, n)` then `fetch_add_wrapping(result, n)` is performed
- THEN the final cell value equals the original `c`
