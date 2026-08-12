## ADDED Requirements

### Requirement: Proof-Gap Site-List Drift Gate

The system MUST validate the exact current `#[verifier::external_body]` site list, not only the aggregate count, so same-count proof-gap substitutions fail closed.

r[verification-gaps.inventory-site-list-drift-gate]

#### Scenario: Current external-body site list passes

- GIVEN the current audited external-body baseline of 16 file/function sites
- WHEN `scripts/check-proof-gap-inventory.rs` scans `src/`
- THEN the script reports the expected 16 sites
- AND it exits successfully when every live site matches the expected file/function list

#### Scenario: Same-count site substitution fails closed

- GIVEN the Rust inventory script runs in a controlled same-count site-substitution fixture mode
- WHEN the simulated live inventory preserves the external-body count but replaces one expected site with an unexpected site
- THEN the script exits non-zero
- AND the evidence records the missing expected site and unexpected simulated site without mutating source files

#### Scenario: Full verification enforces exact sites

- GIVEN the repository verification script is run
- WHEN `./scripts/verify.sh` invokes the proof-gap inventory drift stage
- THEN the Rust checker validates both marker counts and the exact external-body site list before full verification can pass
