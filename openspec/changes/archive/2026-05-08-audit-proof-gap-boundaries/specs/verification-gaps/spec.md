## ADDED Requirements

### Requirement: Source-Derived Proof-Gap Boundary Inventory [r[verification-gaps.boundary-audit.inventory]]
The system MUST maintain a source-derived inventory of remaining trusted or weakened proof boundaries before any proof-gap closeout attempt updates published counts.

#### Scenario: Inventory lists every trusted boundary [r[verification-gaps.boundary-audit.inventory.scenario.lists-every-boundary]]

- GIVEN the audit command scans `src/` for `#[verifier::external_body]`
- WHEN proof-gap inventory evidence is captured
- THEN the evidence lists every matching file, function, and total count
- AND it separately reports counts for `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, and runtime-only exemption markers

### Requirement: Permanent And Closeable Boundary Classification [r[verification-gaps.boundary-audit.classification]]
The system MUST classify each remaining proof boundary as permanent trust or a closeable structural candidate before implementation work removes, rewrites, or re-counts the boundary.

#### Scenario: Cryptographic runtime trust is not overclaimed [r[verification-gaps.boundary-audit.classification.scenario.crypto-not-overclaimed]]

- GIVEN a boundary represents runtime cryptography, digest implementation behavior, codec correctness, or collision resistance
- WHEN the boundary is classified
- THEN the classification records it as permanent trust unless a concrete structural wrapper proof is scoped without claiming cryptographic strength

#### Scenario: Structural candidates include proof plan [r[verification-gaps.boundary-audit.classification.scenario.structural-plan-required]]

- GIVEN a boundary is classified as closeable
- WHEN implementation begins
- THEN the audit records the finite structural property to prove, the touched module, and the focused Verus/Tracey evidence required for closure

### Requirement: Bounded Closeout Slice [r[verification-gaps.boundary-audit.closeout-slice]]
The system MUST drain at most one selected closeable boundary family per proof-gap audit closeout slice, and MUST NOT introduce `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, runtime-only exemption markers, or new `#[verifier::external_body]` sites as replacements.

#### Scenario: Failed closeout is reclassified rather than hidden [r[verification-gaps.boundary-audit.closeout-slice.scenario.failed-closeout-reclassified]]

- GIVEN the selected closeable boundary cannot be closed with a focused proof
- WHEN the closeout attempt is abandoned
- THEN the change records the blocker and classification update
- AND no replacement weakened marker is added

### Requirement: Published Inventory Synchronization [r[verification-gaps.boundary-audit.inventory-sync]]
The system MUST synchronize README, requirements, and canonical OpenSpec proof-gap totals from source-derived count evidence whenever the trusted-boundary inventory changes.

#### Scenario: Published counts match source audit [r[verification-gaps.boundary-audit.inventory-sync.scenario.counts-match-source]]

- GIVEN a closeout changes the number of trusted or weakened proof boundaries
- WHEN the change is verified
- THEN README, `docs/requirements.md`, `openspec/specs/verification-gaps/spec.md`, and captured evidence report the same non-double-counted totals
- AND Tracey uncovered and untested queries remain green
