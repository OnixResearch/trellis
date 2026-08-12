## MODIFIED Requirements

### Requirement: R1CS Gate Verification Coverage

The R1CS gate primitive MUST include non-vacuous verification and runtime coverage for gate admission, row encoding, satisfaction, malformed gate rejection, malformed witness rejection, one-wire rejection, and valid-but-unsatisfied encoded-row rejection.

r[rust-primitives.r1cs-gate-verification-coverage]

#### Scenario: Coverage remains catalogued

- GIVEN the R1CS gate primitive is present
- WHEN repo-wide requirement coverage checks run
- THEN the gate requirements MUST have implementation and verification references

#### Scenario: Coverage proof states concrete rejections

- GIVEN malformed gates, malformed witnesses, missing one-wire witnesses, or valid gates whose encoded row is unsatisfied
- WHEN the gate coverage proof is reviewed
- THEN the proof postconditions MUST state the corresponding satisfaction rejection implications instead of only `true`
