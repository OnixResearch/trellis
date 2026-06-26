## ADDED Requirements

### Requirement: R1CS Gate Shape

A bounded R1CS gate adapter MUST admit only supported finite gate kinds, valid witness lengths, valid moduli, in-range active witness indices, and bounded constant values.

r[rust-primitives.r1cs-gate-shape]

#### Scenario: Malformed gate rejection

- GIVEN a gate with an unsupported shape, out-of-range witness index, invalid witness length, invalid modulus, or overlarge constant value
- WHEN gate admission is checked
- THEN the gate MUST be rejected before satisfaction is claimed

### Requirement: R1CS Gate Row Encoding

A bounded R1CS gate adapter MUST encode multiplication, addition, and constant gates into a single R1CS row with the same witness length and modulus as the admitted gate.

r[rust-primitives.r1cs-gate-row-encoding]

#### Scenario: Multiplication row encoding

- GIVEN an admitted multiplication gate over witness indices x, y, and z
- WHEN the gate is encoded to a row
- THEN the row MUST enforce `w[x] * w[y] = w[z]` through the existing R1CS row satisfaction predicate

#### Scenario: Addition and constant row encoding

- GIVEN an admitted addition or constant gate and a witness whose slot zero is the constant-one wire
- WHEN the gate is encoded to a row
- THEN addition MUST enforce `(w[x] + w[y]) * 1 = w[z]` and constant MUST enforce `w[x] * 1 = c` through the existing R1CS row satisfaction predicate

### Requirement: R1CS Gate Satisfaction

A bounded R1CS gate satisfaction helper MUST return true exactly when the gate is admitted, the witness is admitted with the same active length, any required constant-one wire is present, and the encoded R1CS row is satisfied.

r[rust-primitives.r1cs-gate-satisfaction]

#### Scenario: Gate satisfaction and rejection

- GIVEN admitted multiplication, addition, or constant gates and a witness
- WHEN the gate satisfaction helper is evaluated
- THEN matching witnesses MUST be accepted and mismatched witness values MUST be rejected

### Requirement: R1CS Gate Verification Coverage

The R1CS gate primitive MUST include verification and runtime coverage for gate admission, row encoding, satisfaction, malformed gate rejection, and mismatched witness rejection.

r[rust-primitives.r1cs-gate-verification-coverage]

#### Scenario: Coverage remains catalogued

- GIVEN the R1CS gate primitive is present
- WHEN repo-wide requirement coverage checks run
- THEN the gate requirements MUST have implementation and verification references
