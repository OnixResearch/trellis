## MODIFIED Requirements

### Requirement: R1CS Boolean Verification Coverage

The R1CS boolean primitive MUST include non-vacuous implementation, verification, and runtime coverage for shape admission, row encoding, satisfaction, malformed shape rejection, malformed witness rejection, missing one-wire rejection, non-boolean witness rejection, and valid-but-unsatisfied encoded-row rejection.

r[rust-primitives.r1cs-boolean-verification-coverage]

#### Scenario: Boolean primitive coverage remains catalogued

- GIVEN the R1CS boolean primitive is present
- WHEN repo-wide requirement coverage checks run
- THEN the boolean requirements MUST have implementation and verification references

#### Scenario: Coverage proof states concrete boolean implications

- GIVEN malformed boolean shapes, malformed witnesses, missing one-wire witnesses, or admitted booleans whose encoded row is unsatisfied
- WHEN the boolean coverage proof is reviewed
- THEN the proof postconditions MUST state the corresponding validity or rejection implications instead of only `true`
