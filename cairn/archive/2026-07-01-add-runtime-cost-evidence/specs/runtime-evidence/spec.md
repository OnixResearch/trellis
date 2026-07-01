## ADDED Requirements

### Requirement: Ghost-Erasure Runtime Evidence

The repository MUST provide reproducible runtime-cost evidence that explains how normal Cargo builds erase proof/ghost code and how users can inspect representative artifacts without treating descriptive measurements as universal performance guarantees.

r[runtime-evidence.ghost-erasure]

#### Scenario: Evidence records command context

- GIVEN a runtime-cost evidence record is documented
- WHEN a user reads the record
- THEN it includes the command, build profile, target or example name, and observed artifact or runtime summary

#### Scenario: Descriptive measurement is not a hard guarantee

- GIVEN the evidence includes an artifact size or runtime observation
- WHEN the evidence is rendered
- THEN it labels the value as an observation unless a named requirement defines a pass/fail bound

#### Scenario: Missing evidence fields are rejected

- GIVEN a checked evidence record omits its command, profile, target, or result field
- WHEN runtime-evidence validation runs
- THEN validation fails with a diagnostic identifying the missing field
