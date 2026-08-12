## MODIFIED Requirements

### Requirement: R1CS System Verification Coverage

The R1CS system primitive MUST provide non-vacuous `r[impl]`, `r[verify]`, and runtime coverage for system length admission, shared-modulus row admission, malformed system rejection, malformed witness rejection, failed active-row rejection, inactive-row irrelevance, and satisfied-system decomposition before README/catalog claims are made.

r[rust-primitives.r1cs-system-verification-coverage]

#### Scenario: Public helper has traceable coverage

- GIVEN a public helper in `r1cs_system`
- WHEN repo-wide coverage checks are run
- THEN each helper MUST have a matching implementation or verification reference

#### Scenario: Coverage proof states concrete system implications

- GIVEN invalid system lengths, active modulus mismatches, malformed systems, malformed witnesses, failed active rows, or satisfied systems
- WHEN the system coverage proof is reviewed
- THEN the proof postconditions MUST state the corresponding validity, rejection, or satisfaction-decomposition implications instead of only `true`
