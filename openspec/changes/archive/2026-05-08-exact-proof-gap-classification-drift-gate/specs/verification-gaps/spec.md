## ADDED Requirements

### Requirement: Proof-Gap Classification Drift Gate

The system MUST validate the classification attached to every expected proof-gap site, so permanent trust and closeable-candidate labels cannot silently drift independently of the exact site list.

r[verification-gaps.inventory-classification-drift-gate]

#### Scenario: Current classifications pass

- GIVEN the current audited 16-site proof-gap baseline with classifications
- WHEN the Rust inventory checker validates classifications
- THEN every live site matches its expected classification and the checker exits successfully

#### Scenario: Classification substitution fails closed

- GIVEN a controlled fixture that preserves the exact site list but changes one classification
- WHEN the Rust checker evaluates classification drift
- THEN it exits non-zero and reports the changed classification without mutating source files

#### Scenario: Full verification enforces classifications

- GIVEN `./scripts/verify.sh` runs
- WHEN it invokes the proof-gap inventory drift stage
- THEN classification drift is checked before full verification can pass
