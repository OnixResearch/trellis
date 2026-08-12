# Proof Artifact Manifest Specification

## Purpose

Adds optional Preserves carriers for Trellis proof-manifest snapshots.

## Requirements

### Requirement: Preserves proof-manifest envelope contract
r[trellis.proof_artifact_manifest.preserves_envelope.contract] Trellis MUST support an optional Preserves proof-manifest envelope contract that links canonical Preserves envelopes to Trellis source manifest digests, selected primitive/proof rows, schema labels, allowed consumers, and non-claims.

#### Scenario: Source-linked Preserves manifest envelope passes
r[trellis.proof_artifact_manifest.preserves_envelope.fixtures.positive]
- GIVEN an envelope row records a source manifest digest, selected primitive/proof rows, canonical Preserves digest, schema label, allowed consumers, and required non-claims
- WHEN envelope validation runs
- THEN validation MUST pass and preserve the source manifest linkage.

#### Scenario: Invalid Preserves manifest envelope fails
r[trellis.proof_artifact_manifest.preserves_envelope.fixtures.negative]
- GIVEN an envelope row has stale source identity, missing primitive/proof rows, unsupported schema, overbroad verified claims, or missing non-claims
- WHEN envelope validation runs
- THEN validation MUST fail with deterministic diagnostics.

### Requirement: Envelope validation boundary
r[trellis.proof_artifact_manifest.preserves_envelope.validation] Trellis MUST treat Preserves envelopes as carriers for proof-manifest identity only, not as proof artifacts or downstream authorization semantics.

#### Scenario: Carrier boundary is visible
r[trellis.proof_artifact_manifest.preserves_envelope.docs]
- GIVEN a Preserves proof-manifest envelope validates
- WHEN the supported claim is rendered
- THEN it MUST state that Verus/spec artifacts and Trellis manifest rows remain authoritative for proof identity.

### Requirement: Final validation
r[trellis.proof_artifact_manifest.preserves_envelope.final_validation] The change MUST include positive and negative fixtures plus focused validation evidence before archive.

#### Scenario: Fixture suite covers envelope health
r[trellis.proof_artifact_manifest.preserves_envelope.final_validation.fixtures]
- GIVEN valid and invalid Preserves proof-manifest envelope fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass and invalid fixtures MUST fail closed.
