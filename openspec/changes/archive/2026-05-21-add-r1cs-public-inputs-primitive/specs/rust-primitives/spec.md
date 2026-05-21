## ADDED Requirements

### Requirement: R1CS Public Input Binding Shape

The system MUST expose a finite public-input binding envelope with explicit active length, witness-index slots, and public-value slots. Binding-shape admission MUST accept only active lengths within the bounded slot count, active indices inside the admitted witness prefix, admitted public values, and an admitted modulus. Inactive binding slots MUST NOT affect binding-shape admission.

r[rust-primitives.r1cs-public-input-binding-shape]

#### Scenario: Matching public inputs are admitted

- GIVEN admitted public bindings whose active indices are inside the witness prefix and whose active public values match the indexed witness values modulo the modulus
- WHEN public-input binding admission is evaluated
- THEN the binding helper returns true

#### Scenario: Out-of-range binding index is rejected

- GIVEN an active public binding whose witness index is outside the active witness prefix
- WHEN public-input binding admission is evaluated
- THEN the binding helper returns false

### Requirement: R1CS Public Statement Satisfaction

The system MUST expose a public statement satisfaction helper that returns true exactly when the R1CS system is satisfied by the witness and the public-input bindings are satisfied against that same witness and system modulus. The helper MUST reject malformed systems, malformed witnesses, unsatisfied systems, mismatched public values, and out-of-range public-input bindings without claiming proving-system soundness.

r[rust-primitives.r1cs-public-statement-satisfaction]

#### Scenario: Satisfied system with matching public inputs is admitted

- GIVEN an admitted R1CS system, an admitted witness satisfying every active row, and public bindings matching selected witness slots
- WHEN public statement satisfaction is evaluated
- THEN the helper returns true

#### Scenario: Mismatched public value rejects statement

- GIVEN an otherwise satisfied R1CS system and admitted witness with one active public value different from the bound witness slot modulo the system modulus
- WHEN public statement satisfaction is evaluated
- THEN the helper returns false

### Requirement: R1CS Public Input Verification Coverage

The R1CS public-input primitive MUST provide explicit `r[impl]` and `r[verify]` coverage for every public helper introduced by this change, plus positive and negative runtime tests before README/catalog claims are made.

r[rust-primitives.r1cs-public-input-verification-coverage]

#### Scenario: Public helper has traceable coverage

- GIVEN a public helper in `r1cs_public_inputs`
- WHEN repo-wide coverage checks are run
- THEN the helper has explicit requirement coverage and corresponding proof or test evidence
