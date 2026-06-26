# Verification Policy Delta

## ADDED Requirements

### Requirement: Nickel-Authored Verification Policy
Verification checker policy MUST be authored in Nickel when the policy is declarative configuration rather than parser/scanner logic.

r[verification-policy.nickel-authored]

#### Scenario: Nickel policy typechecks

- GIVEN the Nickel policy source files
- WHEN the policy typecheck command runs in the Nix development environment
- THEN Nickel accepts the policy contracts and defaults

#### Scenario: Generated policy is fresh

- GIVEN the checked-in generated JSON policy artifact
- WHEN the policy export freshness check runs
- THEN the exported JSON matches the checked-in artifact byte-for-byte or with deterministic normalization

#### Scenario: Scanner logic remains in Rust

- GIVEN a checker requires source parsing or call-graph analysis
- WHEN the checker runs
- THEN Rust code performs the scanning while Nickel supplies only typed policy/configuration inputs

#### Scenario: Exemptions are bounded

- GIVEN the policy allows an exemption
- WHEN the exemption is authored
- THEN the exemption includes scope, owner, reason, and expiry or equivalent bounded-review metadata
