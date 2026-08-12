## ADDED Requirements

### Requirement: JSON Artifact Contract Admission

Repository-owned JSON artifacts that are checked in for downstream consumption MUST have explicit contract-admission checks for their stable shape when they describe policy, proof evidence, baseline findings, or release/evidence metadata. Nickel contracts MUST validate schema identifiers, required fields, bounded enum values, safe relative paths, digest text formats, and explicit non-claim fields where applicable. Rust generators and semantic validators MUST remain authoritative for repository fact checks that require filesystem, Tracey, Verus, Cargo, or Octet execution.

r[verification-tooling.json-artifact-contract-admission]

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

The JSON artifact contract checker MUST include positive and negative fixtures for each covered artifact family. Negative fixtures MUST cover missing required fields, unsupported schema versions, malformed digest fields, unsafe paths, unsupported enum values, overbroad claim fields, and malformed finding records where the artifact family contains findings.

r[verification-tooling.json-artifact-contract-fixtures]

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
