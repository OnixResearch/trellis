## ADDED Requirements

### Requirement: R1CS Row Shape

The system MUST expose a finite single-row R1CS envelope with explicit row length, witness length, modulus, three coefficient slots for each of `A`, `B`, and `C`, and three witness-value slots. Row and witness admission MUST reject zero lengths, lengths above the finite slot count, moduli outside the bounded field range, and active coefficient or witness values outside the supported element bound.

r[rust-primitives.r1cs-row-shape]

#### Scenario: Valid row and witness shapes are admitted

- GIVEN a row and witness with matching length in `1..=3`, valid modulus, and bounded active elements
- WHEN row and witness admission are evaluated
- THEN both shapes are admitted

#### Scenario: Length mismatch is rejected by satisfaction

- GIVEN an individually valid row and witness with different active lengths
- WHEN row satisfaction is evaluated
- THEN the satisfaction helper rejects the pair

### Requirement: R1CS Dot Product

The system MUST compute a bounded modular dot product for an admitted row coefficient vector and witness over the active length. The executable dot-product helper MUST refine the spec expression and MUST NOT use unchecked arithmetic outside the finite bounds guaranteed by admission.

r[rust-primitives.r1cs-dot-product]

#### Scenario: Active slots contribute to modular dot product

- GIVEN active coefficient and witness slots within the admitted bounds
- WHEN the dot product is computed
- THEN the result equals the active pairwise sum reduced by the row modulus

#### Scenario: Inactive slots do not affect shorter rows

- GIVEN an admitted row length shorter than the physical slot count
- WHEN inactive coefficient or witness slots differ
- THEN only active slots contribute to the dot-product result

### Requirement: R1CS Row Satisfaction

The system MUST expose a one-row R1CS satisfaction helper that returns true exactly when row and witness shapes are admitted, lengths match, and `<A,w> * <B,w> == <C,w>` modulo the row modulus. The helper MUST reject malformed shapes, mismatched lengths, and unsatisfied equations without claiming multi-row constraint-system or proving-system soundness.

r[rust-primitives.r1cs-row-satisfaction]

#### Scenario: Satisfied row is admitted

- GIVEN an admitted row and matching witness whose modular dot products satisfy the equation
- WHEN row satisfaction is evaluated
- THEN the helper returns true

#### Scenario: Unsatisfied row is rejected

- GIVEN an admitted row and matching witness whose modular dot products do not satisfy the equation
- WHEN row satisfaction is evaluated
- THEN the helper returns false

### Requirement: R1CS Verification Coverage

The R1CS primitive MUST provide explicit `r[impl]` and `r[verify]` coverage for every public helper introduced by this change, plus positive and negative runtime tests before README/catalog claims are made.

r[rust-primitives.r1cs-verification-coverage]

#### Scenario: Public helper has traceable coverage

- GIVEN a public helper in `r1cs_constraint`
- WHEN repo-wide coverage checks are run
- THEN the helper has explicit requirement coverage and corresponding proof or test evidence
