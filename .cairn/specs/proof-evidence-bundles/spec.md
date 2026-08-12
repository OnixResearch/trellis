# Proof Evidence Bundles Specification

## Purpose

Defines the `proof-evidence-bundles` capability.

## Requirements

### Requirement: Exportable Proof-Evidence Bundles

The repository SHOULD provide deterministic proof-evidence bundles that summarize Trellis proof facts, requirement coverage, trusted-boundary references, and content identities for downstream consumers without granting release approval by themselves.

r[proof-evidence-bundles.export]

#### Scenario: Bundle record names proof scope

- GIVEN a proof-evidence bundle record is generated
- WHEN a downstream consumer inspects the record
- THEN it includes schema version, bundle scope, requirement IDs, module/source paths, proof or verifier command, proof outcome, Tracey coverage status, trusted-boundary or proof-gap references, BLAKE3 content identities, and explicit non-claims.

#### Scenario: Bundle remains deterministic

- GIVEN the same repository content and supported policy inputs
- WHEN bundle generation runs twice in the same environment
- THEN the generated bundle content is deterministic and does not depend on network access, wall-clock time, random numbers, or external service state.

#### Scenario: Bundle does not approve a release

- GIVEN a proof-evidence bundle is valid
- WHEN downstream tooling consumes it
- THEN Trellis only asserts the recorded proof facts and boundaries.
- AND downstream release admission, artifact provenance, and policy decisions remain owned by the consuming gate.

### Requirement: Proof-Evidence Bundle Validation

The repository MUST validate proof-evidence bundles fail-closed for stale identities, missing evidence fields, unsupported schemas, missing coverage, missing proof-gap references, overbroad claims, and malformed consumer-facing shape. Shape admission SHOULD be captured in Nickel contracts for supported schema identifiers, required fields, BLAKE3 content identities, safe relative source paths, proof outcomes, coverage status, claims, and explicit non-claims, while Rust validation remains authoritative for recomputing identities and checking repository-local proof facts.

r[proof-evidence-bundles.validation]

#### Scenario: Valid bundle passes

- GIVEN a bundle record includes all required fields, current BLAKE3 identities, supported schema version, covered requirement IDs, proof-gap references for tracked trusted boundaries, scoped claim text, and explicit non-claims
- WHEN bundle validation runs
- THEN validation accepts the bundle and reports the validated scope

#### Scenario: Missing required field fails

- GIVEN a bundle record omits a required field such as requirement IDs, proof command, proof outcome, content identity, or explicit non-claim
- WHEN bundle validation runs
- THEN validation fails with a diagnostic identifying the missing field

#### Scenario: Stale identity fails

- GIVEN a bundle record contains a BLAKE3 identity that no longer matches the referenced local file content
- WHEN bundle validation runs
- THEN validation fails with a diagnostic identifying the stale reference

#### Scenario: Unsupported schema fails

- GIVEN a bundle record declares an unsupported schema version
- WHEN bundle validation runs
- THEN validation fails without attempting to reinterpret the record as a different schema

#### Scenario: Missing coverage or proof-gap reference fails

- GIVEN a bundle record names requirement IDs without coverage or a trusted boundary without a proof-gap reference
- WHEN bundle validation runs
- THEN validation fails with a diagnostic identifying the uncovered requirement or missing boundary reference

#### Scenario: Overbroad claim fails

- GIVEN a bundle record claims whole-program correctness, release approval, certification, third-party conformance, or production safety beyond the recorded proof scope
- WHEN bundle validation runs
- THEN validation fails with a diagnostic identifying the unsupported claim

#### Scenario: Contract shape fails before semantic validation

- GIVEN a bundle record has malformed BLAKE3 digest text, unsafe source paths, unsupported proof outcome, unsupported coverage status, or missing non-claim fields
- WHEN bundle validation runs
- THEN contract admission rejects the record before downstream consumers treat it as valid proof evidence
