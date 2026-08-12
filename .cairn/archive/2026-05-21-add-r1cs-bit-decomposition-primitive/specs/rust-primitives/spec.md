## ADDED Requirements

### Requirement: R1CS Bit Decomposition Shape

A bounded R1CS bit-decomposition primitive MUST admit only valid witness lengths, valid boolean-compatible moduli, a bounded value below four, and two distinct nonzero bit witness indices inside the active witness prefix.

r[rust-primitives.r1cs-bit-decomposition-shape]

#### Scenario: Malformed bit-decomposition shape rejection

- GIVEN an invalid witness length, invalid boolean-compatible modulus, value outside the two-bit range, zero bit index, out-of-range bit index, or duplicate bit indices
- WHEN bit-decomposition shape admission is checked
- THEN the shape MUST be rejected before satisfaction is claimed

### Requirement: R1CS Bit Boolean Composition

A bounded R1CS bit-decomposition primitive MUST require each active bit witness slot to satisfy the existing R1CS boolean primitive under the same active witness length and modulus.

r[rust-primitives.r1cs-bit-decomposition-boolean-composition]

#### Scenario: Non-boolean bit rejection

- GIVEN an admitted bit-decomposition shape and a witness with a non-boolean value in either bit slot
- WHEN bit-decomposition satisfaction is evaluated
- THEN satisfaction MUST be rejected

### Requirement: R1CS Bit Recomposition Satisfaction

A bounded R1CS bit-decomposition satisfaction helper MUST return true exactly when the shape and witness are admitted, both bit slots are boolean, witness slot zero is one, and `value = bit0 + 2 * bit1`.

r[rust-primitives.r1cs-bit-decomposition-satisfaction]

#### Scenario: Recomposition accepts matching bits and rejects mismatches

- GIVEN an admitted bit-decomposition shape and boolean bit witness slots
- WHEN bit-decomposition satisfaction is evaluated
- THEN matching two-bit recompositions MUST be accepted and mismatched recompositions MUST be rejected

### Requirement: R1CS Bit Decomposition Verification Coverage

The R1CS bit-decomposition primitive MUST include implementation and verification coverage for shape admission, boolean-bit composition, recomposition satisfaction, malformed shape rejection, non-boolean bit rejection, and mismatched value rejection.

r[rust-primitives.r1cs-bit-decomposition-verification-coverage]

#### Scenario: Bit-decomposition primitive coverage remains catalogued

- GIVEN the R1CS bit-decomposition primitive is present
- WHEN repo-wide requirement coverage checks run
- THEN the bit-decomposition requirements MUST have implementation and verification references
