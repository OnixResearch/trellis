## ADDED Requirements

### Requirement: R1CS Boolean Shape

A bounded R1CS boolean primitive MUST admit only valid witness lengths, valid moduli whose `modulus - 1` coefficient fits the bounded row model, and a nonzero boolean witness index inside the active witness prefix.

r[rust-primitives.r1cs-boolean-shape]

#### Scenario: Malformed boolean shape rejection

- GIVEN an invalid witness length, invalid modulus, overlarge `modulus - 1` coefficient, zero boolean index, or out-of-range boolean index
- WHEN boolean shape admission is checked
- THEN the shape MUST be rejected before satisfaction is claimed

### Requirement: R1CS Boolean Row Encoding

A bounded R1CS boolean primitive MUST encode booleanity as a single R1CS row equivalent to `b * (1 - b) = 0` using the existing constant-one witness slot convention.

r[rust-primitives.r1cs-boolean-row-encoding]

#### Scenario: Boolean row encoding

- GIVEN an admitted boolean index and modulus
- WHEN the booleanity row is encoded
- THEN the row MUST use the active witness length and modulus and MUST be valid under the existing R1CS row-shape predicate

### Requirement: R1CS Boolean Satisfaction

A bounded R1CS boolean satisfaction helper MUST return true exactly when the boolean shape is admitted, the witness is admitted with the same active length, witness slot zero is one, and the encoded row is satisfied.

r[rust-primitives.r1cs-boolean-satisfaction]

#### Scenario: Boolean values accepted and non-boolean values rejected

- GIVEN admitted boolean shapes and witnesses
- WHEN boolean satisfaction is evaluated
- THEN witness values 0 and 1 at the boolean slot MUST be accepted and non-boolean values MUST be rejected

### Requirement: R1CS Boolean Verification Coverage

The R1CS boolean primitive MUST include implementation and verification coverage for shape admission, row encoding, satisfaction, malformed shape rejection, missing one-wire rejection, and non-boolean witness rejection.

r[rust-primitives.r1cs-boolean-verification-coverage]

#### Scenario: Boolean primitive coverage remains catalogued

- GIVEN the R1CS boolean primitive is present
- WHEN repo-wide requirement coverage checks run
- THEN the boolean requirements MUST have implementation and verification references
