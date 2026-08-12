# verification-tooling Specification

## Purpose
Defines deterministic repository verification tooling and fixture-backed regression coverage for checker scripts.

## Requirements

### Requirement: Checker Fixture Regressions
r[verification-tooling.checker-fixture-regressions]
Verification checker scripts MUST have fixture-backed self-tests that cover both accepted inputs and representative rejected inputs for their enforced policy.


#### Scenario: Valid fixtures pass

- GIVEN fixture source files that satisfy coverage, ensures, and iterative-exec policies
- WHEN the checker self-test harness runs
- THEN each checker accepts its valid fixture

#### Scenario: Missing coverage annotation is rejected

- GIVEN a fixture with a non-test public exec function missing an `r[impl]` annotation
- WHEN the coverage checker is run through the self-test harness
- THEN the harness observes a failing exit status and a diagnostic that identifies the missing annotation

#### Scenario: Missing Verus ensures clause is rejected

- GIVEN a fixture with a public exec function inside a `verus!` block that lacks an `ensures` clause
- WHEN the ensures checker is run through the self-test harness
- THEN the harness observes a failing exit status and a diagnostic that identifies the missing `ensures`

#### Scenario: Iterative exec recursion is rejected

- GIVEN a fixture with a self-edge, mutual cycle, or disallowed qualified intra-module exec call
- WHEN the iterative-exec checker is run through the self-test harness
- THEN the harness observes a failing exit status and a diagnostic that identifies the rejected graph shape

#### Scenario: Fixture checks are part of Nix verification

- GIVEN the repository's flake checks are evaluated
- WHEN the checker self-test check is selected
- THEN the fixture-backed self-tests run deterministically without requiring network access or the full repository test suite

### Requirement: JSON Artifact Contract Admission
r[verification-tooling.json-artifact-contract-admission]

Repository-owned JSON artifacts that are checked in for downstream consumption MUST have explicit contract-admission checks for their stable shape when they describe policy, proof evidence, baseline findings, or release/evidence metadata. Nickel contracts MUST validate schema identifiers, required fields, bounded enum values, safe relative paths, digest text formats, and explicit non-claim fields where applicable. Rust generators and semantic validators MUST remain authoritative for repository fact checks that require filesystem, Tracey, Verus, Cargo, or Octet execution.


#### Scenario: Valid JSON artifact is admitted

- GIVEN a checked-in JSON evidence or policy artifact with a supported schema, required fields, bounded enum values, safe relative paths, valid BLAKE3 digest text, and explicit non-claims where required
- WHEN JSON artifact contract admission runs
- THEN the artifact passes shape validation before semantic validators claim success

#### Scenario: Unsupported schema is rejected

- GIVEN a JSON artifact fixture with an unsupported schema identifier or schema version
- WHEN contract admission runs
- THEN validation rejects the fixture without attempting to reinterpret it as a supported schema

#### Scenario: Unsafe path is rejected

- GIVEN a JSON artifact fixture containing an absolute path, parent-directory traversal, or backslash path separator in a contract-governed path field
- WHEN contract admission runs
- THEN validation rejects the fixture with a deterministic diagnostic

#### Scenario: Malformed digest is rejected

- GIVEN a JSON artifact fixture containing a digest field that is not a valid BLAKE3 hex or declared `b3:` digest text for that artifact contract
- WHEN contract admission runs
- THEN validation rejects the fixture before semantic identity checks run

### Requirement: JSON Artifact Contract Fixtures
r[verification-tooling.json-artifact-contract-fixtures]

The JSON artifact contract checker MUST include positive and negative fixtures for each covered artifact family. Negative fixtures MUST cover missing required fields, unsupported schema versions, malformed digest fields, unsafe paths, unsupported enum values, overbroad claim fields, and malformed finding records where the artifact family contains findings.


#### Scenario: Proof-evidence fixture passes

- GIVEN a minimal proof-evidence bundle fixture with required scope, requirement IDs, source paths, proof command, proof outcome, coverage status, content identities, claims, and non-claims
- WHEN the JSON artifact contract fixture checker runs
- THEN the fixture is admitted by the contract

#### Scenario: Proof-evidence missing non-claim fails

- GIVEN a proof-evidence bundle fixture that omits explicit non-claims or adds a release-approval claim
- WHEN the JSON artifact contract fixture checker runs
- THEN the fixture is rejected before downstream consumers treat it as valid evidence

#### Scenario: Octet finding fixture passes

- GIVEN a minimal Octet baseline fixture with supported schema versions, baseline metadata, stable finding identity fields, severity, file path, location, lint, group, and message
- WHEN the JSON artifact contract fixture checker runs
- THEN the fixture is admitted by the contract

#### Scenario: Octet malformed finding fails

- GIVEN an Octet baseline fixture with a malformed finding identity, unsupported severity, unsafe file path, or missing location metadata
- WHEN the JSON artifact contract fixture checker runs
- THEN the fixture is rejected with a deterministic diagnostic

#### Scenario: Contract admission is part of full verification

- GIVEN the repository verification wrapper runs
- WHEN JSON artifact contract checks are configured
- THEN the wrapper runs contract admission before claiming generated evidence and policy artifacts are valid
