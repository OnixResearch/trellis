## ADDED Requirements

### Requirement: R1CS System Shape

The system MUST expose a finite multi-row R1CS envelope with explicit system length, shared modulus, and three physical row slots. System admission MUST reject zero lengths, lengths above the finite row-slot count, invalid system moduli, invalid active rows, and active rows whose modulus differs from the system modulus. Inactive rows MUST NOT affect system-shape admission.

r[rust-primitives.r1cs-system-shape]

#### Scenario: Valid active rows are admitted

- GIVEN a system with length in `1..=3`, a valid modulus, and active rows using that modulus
- WHEN system-shape admission is evaluated
- THEN the system shape is admitted

#### Scenario: Modulus mismatch rejects active row

- GIVEN an otherwise valid system with an active row over a different modulus
- WHEN system-shape admission is evaluated
- THEN the system is rejected

### Requirement: R1CS System Satisfaction

The system MUST expose a satisfaction helper that returns true exactly when the system shape is admitted, the witness is admitted, and every active row is satisfied by the same witness. The helper MUST reject malformed witnesses, failed active rows, and malformed active rows without claiming proving-system soundness.

r[rust-primitives.r1cs-system-satisfaction]

#### Scenario: All active rows satisfied

- GIVEN an admitted system and admitted witness where every active row satisfies the R1CS equation
- WHEN system satisfaction is evaluated
- THEN the helper returns true

#### Scenario: One failed active row rejects the system

- GIVEN an admitted system and admitted witness where one active row fails the R1CS equation
- WHEN system satisfaction is evaluated
- THEN the helper returns false

### Requirement: R1CS System Verification Coverage

The R1CS system primitive MUST provide explicit `r[impl]` and `r[verify]` coverage for every public helper introduced by this change, plus positive and negative runtime tests before README/catalog claims are made.

r[rust-primitives.r1cs-system-verification-coverage]

#### Scenario: Public helper has traceable coverage

- GIVEN a public helper in `r1cs_system`
- WHEN repo-wide coverage checks are run
- THEN the helper has explicit requirement coverage and corresponding proof or test evidence
