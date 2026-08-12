## MODIFIED Requirements

### Requirement: R1CS Verification Coverage

The base R1CS row primitive MUST include non-vacuous implementation, verification, and runtime coverage for length predicates, element predicates, modulus predicates, row/witness shape admission, dot-product helpers, row satisfaction, malformed row rejection, malformed witness rejection, and length mismatch rejection.

r[rust-primitives.r1cs-verification-coverage]

#### Scenario: Coverage remains catalogued

- GIVEN the base R1CS row primitive is present
- WHEN repo-wide requirement coverage checks run
- THEN the row requirements MUST have implementation and verification references

#### Scenario: Coverage proof states concrete row implications

- GIVEN invalid row or witness lengths, malformed rows, malformed witnesses, length mismatches, or satisfied rows
- WHEN the row coverage proof is reviewed
- THEN the proof postconditions MUST state the corresponding validity, rejection, or satisfaction-decomposition implications instead of only `true`
