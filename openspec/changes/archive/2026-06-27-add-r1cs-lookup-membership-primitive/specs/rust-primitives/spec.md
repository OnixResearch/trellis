## ADDED Requirements

### Requirement: R1CS Lookup Shape

A bounded R1CS lookup-membership primitive MUST admit only valid witness lengths, a nonzero lookup witness index inside the active witness prefix, a table length from one to three, and bounded R1CS elements in every active table entry.

r[rust-primitives.r1cs-lookup-shape]

#### Scenario: Malformed lookup shape rejection

- GIVEN an invalid witness length, zero lookup index, out-of-range lookup index, zero table length, table length above three, or overlarge active table element
- WHEN lookup shape admission is checked
- THEN the shape MUST be rejected before membership satisfaction is claimed

### Requirement: R1CS Lookup Active Table Membership

A bounded R1CS lookup-membership primitive MUST compare the selected witness-slot value only against active table entries and MUST ignore inactive table entries.

r[rust-primitives.r1cs-lookup-active-membership]

#### Scenario: Active table entries determine membership

- GIVEN an admitted lookup shape, an admitted witness, and a selected witness value
- WHEN lookup membership satisfaction is evaluated
- THEN satisfaction MUST hold exactly when the selected value equals at least one active table entry

### Requirement: R1CS Lookup Satisfaction

A bounded R1CS lookup-membership satisfaction helper MUST return true exactly when the lookup shape and witness are admitted, the witness length matches the lookup length, and the selected witness value is a member of the active table prefix.

r[rust-primitives.r1cs-lookup-satisfaction]

#### Scenario: Lookup accepts members and rejects non-members

- GIVEN an admitted lookup shape and admitted witness
- WHEN lookup satisfaction is evaluated
- THEN active-table member values MUST be accepted and non-member values MUST be rejected

### Requirement: R1CS Lookup Verification Coverage

The R1CS lookup-membership primitive MUST include implementation and verification coverage for shape admission, active table membership, satisfaction, malformed shape rejection, malformed witness rejection, inactive-entry ignoring, and non-member rejection.

r[rust-primitives.r1cs-lookup-verification-coverage]

#### Scenario: Lookup primitive coverage remains catalogued

- GIVEN the R1CS lookup-membership primitive is present
- WHEN repo-wide requirement coverage checks run
- THEN the lookup requirements MUST have implementation and verification references
