## MODIFIED Requirements

### Requirement: R1CS Public Input Verification Coverage

The R1CS public-input primitive MUST provide non-vacuous `r[impl]`, `r[verify]`, and runtime coverage for public-input length admission, active binding-index admission, public-value matching, malformed modulus rejection, malformed witness rejection, malformed statement-system rejection, malformed statement-public-input rejection, inactive-slot irrelevance, empty public-input admission, and satisfied-statement decomposition before README/catalog claims are made.

r[rust-primitives.r1cs-public-input-verification-coverage]

#### Scenario: Public helper has traceable coverage

- GIVEN a public helper in `r1cs_public_inputs`
- WHEN repo-wide coverage checks are run
- THEN each helper MUST have a matching implementation or verification reference

#### Scenario: Coverage proof states concrete public-input implications

- GIVEN invalid public-input lengths, invalid moduli, malformed witnesses, out-of-range active bindings, mismatched public values, malformed statement systems, malformed statement public inputs, or satisfied public statements
- WHEN the public-input coverage proof is reviewed
- THEN the proof postconditions MUST state the corresponding validity, rejection, or satisfaction-decomposition implications instead of only `true`
