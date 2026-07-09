# Proof Artifact Manifest Specification

## Purpose

Defines the `proof-artifact-manifest` capability for Trellis-owned exported primitive and proof-artifact identity.

## Requirements

### Requirement: Verified primitive manifest contract
r[trellis.proof_artifact_manifest.contract] Trellis MUST define a reviewed manifest contract for exported verified primitives, proof artifacts, verifier commands, trusted-boundary classifications, evidence-tier recommendations, and non-claim boundaries.

#### Scenario: Exported primitive row is well formed
r[trellis.proof_artifact_manifest.positive]
- GIVEN a manifest row names a stable primitive ID, module path, public API surface, source identity, verifier identity, requirement IDs, and required non-claims
- WHEN the manifest validator evaluates the row
- THEN the row MUST pass and preserve the primitive ID for downstream references.

#### Scenario: Malformed primitive row fails closed
r[trellis.proof_artifact_manifest.negative]
- GIVEN a manifest row omits source identity, verifier identity, trusted-boundary classification, or required non-claim text
- WHEN the manifest validator evaluates the row
- THEN validation MUST fail with deterministic diagnostics naming the invalid field.

### Requirement: Manifest freshness check
r[trellis.proof_artifact_manifest.freshness] Trellis MUST provide a freshness check that compares manifest source and proof artifact identities against measured repository bytes.

#### Scenario: Fresh artifact identities pass
r[trellis.proof_artifact_manifest.freshness.pass]
- GIVEN manifest rows whose source and proof BLAKE3 identities match measured files
- WHEN the freshness check runs
- THEN it MUST report the manifest as fresh and include the manifest digest.

#### Scenario: Stale artifact identities fail
r[trellis.proof_artifact_manifest.freshness.stale]
- GIVEN a source or proof file changes without updating the manifest identity
- WHEN the freshness check runs
- THEN it MUST fail closed with a stale-artifact diagnostic.

### Requirement: Evidence-tier boundaries
r[trellis.proof_artifact_manifest.tiers] The manifest MUST prevent trusted-boundary, manual-review, or recorded-only rows from being represented as downstream formal-proof evidence without a downstream-owned verifier receipt.

#### Scenario: Trusted boundary cannot promote itself
r[trellis.proof_artifact_manifest.tiers.no_promotion]
- GIVEN a row includes a trusted runtime or proof-gap boundary
- WHEN the manifest is exported
- THEN its evidence-tier recommendation MUST NOT be `formal_proof` unless a scoped verifier receipt and proof artifact support that exact property.

#### Scenario: Non-claims remain visible
r[trellis.proof_artifact_manifest.tiers.non_claims]
- GIVEN a downstream consumer renders or imports a primitive manifest row
- WHEN the supported claim is displayed
- THEN it MUST state that Trellis manifest identity does not prove downstream integration correctness, consumer release eligibility, or verifier soundness outside the declared proof boundary.

### Requirement: Downstream consumption guidance
r[trellis.proof_artifact_manifest.downstream] Trellis MUST document how UCAN, Octet, Valence, Cairn, Basalt, and Molten consume primitive manifest identity without treating vendored crate bytes as automatic proof certification.

#### Scenario: UCAN vendor sync records manifest identity
r[trellis.proof_artifact_manifest.downstream.ucan]
- GIVEN UCAN refreshes `vendor/verified-logic` from Trellis
- WHEN the sync records Trellis revision metadata
- THEN it SHOULD also record the consumed primitive manifest digest or explicitly state why the manifest was not used.

#### Scenario: Octet proof imports stay bounded
r[trellis.proof_artifact_manifest.downstream.octet]
- GIVEN Octet imports a Trellis primitive row
- WHEN Octet emits proof-card or provenance evidence before rerunning the proof
- THEN the imported evidence MUST remain external or manual-review unless Octet has an Octet-owned verifier receipt.

### Requirement: Validation evidence
r[trellis.proof_artifact_manifest.validation] The change MUST include positive and negative fixtures plus focused validation evidence before archive.

#### Scenario: Fixture matrix covers success and failure
r[trellis.proof_artifact_manifest.validation.fixtures]
- GIVEN valid, stale, missing-command, missing-non-claim, and tier-promotion fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass, invalid fixtures MUST fail closed, and the validation receipt MUST bind input and policy identities.
