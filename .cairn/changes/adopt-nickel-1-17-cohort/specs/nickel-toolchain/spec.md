# Trellis Nickel Toolchain Delta

## ADDED Requirements

### Requirement: Trellis pins Nickel 1.17 exactly

r[trellis.nickel_toolchain.cohort] Trellis MUST use Nickel `1.17.0` from upstream commit `1320a983e6c3d1e2fb53dd2464b084b4903b1426` for proof configuration and evidence contracts.

#### Scenario: Proof tools resolve

- GIVEN Trellis builds proof-profile and evidence checks
- WHEN cohort admission runs
- THEN every Nickel command MUST use the exact reviewed evaluator

#### Scenario: An older or floating evaluator resolves

- GIVEN an ambient package, branch, or older pin supplies Nickel
- WHEN admission runs
- THEN admission MUST fail

### Requirement: Proof authority remains outside Nickel

r[trellis.nickel_toolchain.boundary] Nickel MUST validate profile and evidence shape only. Trellis proof tools and checked Rust MUST retain proof authority.

#### Scenario: A profile passes

- GIVEN Nickel accepts a proof profile
- WHEN the proof rail runs
- THEN the named proof tool MUST still establish the scoped result

### Requirement: Compatibility includes negative proof configuration

r[trellis.nickel_toolchain.compatibility] Tests MUST cover valid profiles and rejected malformed, missing-import, contract, unknown-field, identity-mismatch, and cohort fixtures.

#### Scenario: The fixture matrix runs

- GIVEN representative valid and invalid proof configurations
- WHEN Nickel `1.17.0` evaluates them
- THEN valid configurations MUST retain supported outcomes
- AND invalid configurations MUST fail closed

### Requirement: Evidence records the evaluator identity

r[trellis.nickel_toolchain.evidence] Nickel-dependent evidence MUST record the exact evaluator version and source commit while preserving proof scope and non-claims.

#### Scenario: Proof evidence is generated

- GIVEN a Nickel-backed proof profile passes
- WHEN evidence is emitted
- THEN the evaluator identity MUST be explicit
- AND the evidence MUST NOT claim downstream correctness

### Requirement: The repository validation rail passes

r[trellis.nickel_toolchain.validation] The change MUST pass profile fixtures, proof checks, formatting, Clippy, Cairn gates, and relevant Nix checks.

#### Scenario: Validation completes

- GIVEN the pin, fixtures, and evidence are current
- WHEN validation runs
- THEN every required check MUST pass or report one exact blocker
