## ADDED Requirements

### Requirement: Proof-Gap Inventory Drift Gate

The system MUST provide a Rust-script inventory gate that scans source files for trusted or weakened proof-gap markers and fails when live counts drift from the published expected inventory.

r[verification-gaps.inventory-drift-gate]

#### Scenario: Current source inventory passes

- GIVEN the current source tree and expected inventory counts
- WHEN `scripts/check-proof-gap-inventory.rs` scans `src/`
- THEN it reports 16 `#[verifier::external_body]` sites
- AND it reports zero `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, and `@verified-logic:exempt` markers
- AND it exits successfully

#### Scenario: Added trusted marker fails closed

- GIVEN the Rust inventory script runs in its controlled extra-marker fixture mode
- WHEN the simulated live inventory contains one additional `#[verifier::external_body]` marker
- THEN the script exits non-zero
- AND the evidence records the expected-count mismatch without mutating source files

#### Scenario: Full verification includes the drift gate

- GIVEN the repository verification script is run
- WHEN `./scripts/verify.sh` reaches the proof-gap inventory stage
- THEN it invokes the Rust inventory script before cargo, Verus, and Tracey checks claim success
