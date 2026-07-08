# Proof Artifact Provenance Specification

## Purpose

Defines the `proof-artifact-provenance` capability.

## Requirements

### Requirement: Proof artifact canonical identity
r[trellis.proof_artifact_provenance.identity] Trellis proof artifacts that are exportable to downstream evidence chains MUST carry canonical content identity using BLAKE3 over deterministic metadata plus BLAKE3 digests of source/proof inputs and produced verifier receipts.

#### Scenario: Valid proof artifact identity is accepted
r[trellis.proof_artifact_provenance.identity.valid]
- GIVEN proof artifact metadata with a stable artifact ID, canonical metadata digest, input digests, verifier identity, normalized invocation, local property ID, status, and required non-claims
- WHEN Trellis validates the proof artifact provenance metadata
- THEN validation MUST accept the artifact identity and report the canonical BLAKE3 digest.

#### Scenario: Stale digest fails closed
r[trellis.proof_artifact_provenance.identity.stale_digest]
- GIVEN proof artifact metadata whose stated BLAKE3 digest does not match the canonical metadata or input bytes
- WHEN Trellis validates the metadata
- THEN validation MUST fail with a deterministic stale-digest diagnostic.

#### Scenario: Missing verifier identity fails closed
r[trellis.proof_artifact_provenance.identity.missing_verifier]
- GIVEN proof artifact metadata without verifier or toolchain identity
- WHEN Trellis validates the metadata
- THEN validation MUST fail with a deterministic missing-verifier diagnostic.

### Requirement: Proof artifact boundary
r[trellis.proof_artifact_provenance.boundary] Trellis proof artifact provenance MUST state that the artifact proves only the declared Trellis-local property for the exact recorded content and MUST NOT claim downstream certification, release eligibility, or whole-stack correctness.

#### Scenario: Local property scope is explicit
r[trellis.proof_artifact_provenance.boundary.local_scope]
- GIVEN an exportable Trellis proof artifact
- WHEN its provenance metadata is inspected
- THEN it MUST name the declared local property and the exact content scope that was checked.

#### Scenario: Missing non-claims fail closed
r[trellis.proof_artifact_provenance.boundary.missing_non_claims]
- GIVEN proof artifact metadata without the required non-claim boundary
- WHEN Trellis validates the metadata
- THEN validation MUST fail with a deterministic missing-non-claim diagnostic.

#### Scenario: Downstream overclaim fails closed
r[trellis.proof_artifact_provenance.boundary.downstream_overclaim]
- GIVEN proof artifact metadata that treats a Trellis-local proof as downstream Octet certification, Mantle release eligibility, or Cairn gate satisfaction
- WHEN Trellis validates the metadata
- THEN validation MUST fail with a deterministic overclaim diagnostic.

### Requirement: Proof artifact handoff role
r[trellis.proof_artifact_provenance.handoff] Trellis MUST document that exported proof artifact identities are downstream manual-review or reference inputs until the downstream repo reruns and validates its own proof receipt.

#### Scenario: Handoff role is visible
r[trellis.proof_artifact_provenance.handoff.visible]
- GIVEN Octet or Valence consumes a Trellis proof artifact identity
- WHEN the Trellis handoff documentation is read
- THEN it MUST state that the artifact is a manual-review or reference input unless the downstream repo records its own verifier receipt.

#### Scenario: Downstream-certified role is rejected without receipt
r[trellis.proof_artifact_provenance.handoff.no_implicit_certification]
- GIVEN metadata marks a Trellis proof artifact as downstream-certified without a downstream verifier receipt
- WHEN Trellis validates the handoff metadata
- THEN validation MUST reject the role and identify the missing downstream receipt.

### Requirement: Proof artifact fixtures
r[trellis.proof_artifact_provenance.fixtures] Trellis MUST include positive and negative fixtures for proof artifact provenance validation.

#### Scenario: Positive fixtures cover pass and fail statuses
r[trellis.proof_artifact_provenance.fixtures.positive]
- GIVEN valid proof artifact metadata for a passing proof and a failed proof with a failure reason
- WHEN the fixture suite runs
- THEN both fixtures MUST validate as well-formed metadata with their exact statuses preserved.

#### Scenario: Negative fixtures cover broken chains
r[trellis.proof_artifact_provenance.fixtures.negative]
- GIVEN fixtures with stale digests, missing property scope, missing verifier identity, missing non-claims, unsupported claim role, or downstream overclaim
- WHEN the fixture suite runs
- THEN each fixture MUST fail closed with a deterministic diagnostic.
