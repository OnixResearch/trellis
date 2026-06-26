# Verification Gaps Specification

## ADDED Requirements

### Requirement: Proof Audit Inventory Covers All Trusted Boundaries

The proof-gap inventory MUST track every trusted or weakened proof
boundary in the current tree, not only a historical subset of
`#[verifier::external_body]` items.

Tracked categories MUST include:

- `#[verifier::external_body]`
- `assume(...)` soundness oracles
- public functions exempted with `@verified-logic:no-ensures`
- public verified functions that actually lack an `ensures` clause
- placeholder or simplified specs that weaken exec/spec correspondence

#### Scenario: Mixed trust surfaces exist

- GIVEN the tree contains `external_body`, `assume(...)`, and
  `@verified-logic:no-ensures` sites
- WHEN the proof-gap inventory is refreshed
- THEN every site appears in the inventory with file path,
  classification, and rationale

#### Scenario: Inventory totals are published

- GIVEN the inventory has been regenerated from the current tree
- WHEN `README.md` or the main `verification-gaps` spec reports totals
- THEN the published totals match the regenerated inventory

### Requirement: Proof Status Reports Are Evidence Backed

The project MUST NOT claim a fully green proof posture unless the live
verification commands support that claim.

#### Scenario: Verification pipeline is failing

- GIVEN `./scripts/verify.sh` reports failing Clippy, Verus, or Tracey gates
- WHEN the proof status is documented
- THEN the documentation reports the failing gates and affected modules
  instead of claiming all checks are green

#### Scenario: Verification pipeline is green

- GIVEN `./scripts/verify.sh` passes and Tracey reports zero uncovered
  and zero untested rules
- WHEN the proof status is documented
- THEN the documentation may describe the proof posture as fully green

### Requirement: Proof Audit Tracks Tracey Coverage Drift

Tracey uncovered and untested rules discovered during a proof audit MUST
be treated as remediation items until cleared.

#### Scenario: Uncovered rules are present

- GIVEN `tracey query uncovered` returns one or more rule IDs
- WHEN the proof audit is recorded
- THEN the remediation plan lists those rule IDs and the missing
  implementation references to add

#### Scenario: Untested rules are present

- GIVEN `tracey query untested` returns one or more rule IDs
- WHEN the proof audit is recorded
- THEN the remediation plan lists those rule IDs and the missing
  verification references to add

### Requirement: Ensure Checker Distinguishes Real Gaps From Parser Limits

The contract-checking tooling MUST report real missing postconditions
without flagging long valid signatures as missing `ensures`.

#### Scenario: Long verified signature

- GIVEN a public verified function whose `ensures` clause appears more
  than 15 lines after the `pub fn`
- WHEN `scripts/check-ensures.sh` runs
- THEN the function is accepted as having an `ensures` clause

#### Scenario: Real missing ensures

- GIVEN a public verified function with no `ensures` clause
- WHEN `scripts/check-ensures.sh` runs
- THEN the function is reported as a real contract gap
