//! Finite proof-artifact provenance admission.
//!
//! This module checks shell-supplied provenance facts for exportable Trellis proof
//! artifacts. It does not compute BLAKE3, run Verus, read files, validate a
//! downstream receipt, or certify downstream Octet/Valence/Mantle/Cairn behavior.
//! The shell owns bytes, paths, commands, and digest computation; this verified
//! core admits only the in-memory identity, local-property, non-claim, and handoff
//! role facts.

#![allow(clippy::match_same_arms)]
#![allow(clippy::struct_excessive_bools)]

use vstd::prelude::*;

verus! {

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProofArtifactStatus {
    Passing,
    Failing,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProofArtifactClaimRole {
    TrellisLocalProperty,
    ManualReviewInput,
    DownstreamCertified,
    ReleaseEligibility,
    WholeStackCorrectness,
    Unsupported,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ProofArtifactMetadata {
    pub artifact_id_present: bool,
    pub canonical_metadata_digest_matches: bool,
    pub source_input_digests_present: bool,
    pub proof_input_digests_present: bool,
    pub verifier_receipt_digest_present: bool,
    pub verifier_identity_present: bool,
    pub normalized_invocation_present: bool,
    pub local_property_id_present: bool,
    pub content_scope_present: bool,
    pub status: ProofArtifactStatus,
    pub failure_reason_present: bool,
    pub required_non_claims_present: bool,
    pub claim_role: ProofArtifactClaimRole,
    pub downstream_receipt_present: bool,
}

// r[impl trellis.proof_artifact_provenance.identity]
pub open spec fn failure_reason_admitted_spec(
    status: ProofArtifactStatus,
    failure_reason_present: bool,
) -> bool {
    match status {
        ProofArtifactStatus::Passing => true,
        ProofArtifactStatus::Failing => failure_reason_present,
    }
}

// r[impl trellis.proof_artifact_provenance.boundary]
pub open spec fn overclaim_free_role_spec(role: ProofArtifactClaimRole) -> bool {
    match role {
        ProofArtifactClaimRole::TrellisLocalProperty => true,
        ProofArtifactClaimRole::ManualReviewInput => true,
        ProofArtifactClaimRole::DownstreamCertified => false,
        ProofArtifactClaimRole::ReleaseEligibility => false,
        ProofArtifactClaimRole::WholeStackCorrectness => false,
        ProofArtifactClaimRole::Unsupported => false,
    }
}

// r[impl trellis.proof_artifact_provenance.handoff]
pub open spec fn handoff_role_admitted_spec(
    role: ProofArtifactClaimRole,
    downstream_receipt_present: bool,
) -> bool {
    match role {
        ProofArtifactClaimRole::ManualReviewInput => true,
        ProofArtifactClaimRole::DownstreamCertified => downstream_receipt_present,
        _ => false,
    }
}

// r[impl trellis.proof_artifact_provenance.identity]
pub open spec fn proof_artifact_identity_valid_spec(metadata: ProofArtifactMetadata) -> bool {
    metadata.artifact_id_present
        && metadata.canonical_metadata_digest_matches
        && metadata.source_input_digests_present
        && metadata.proof_input_digests_present
        && metadata.verifier_receipt_digest_present
        && metadata.verifier_identity_present
        && metadata.normalized_invocation_present
        && metadata.local_property_id_present
        && failure_reason_admitted_spec(metadata.status, metadata.failure_reason_present)
}

// r[impl trellis.proof_artifact_provenance.boundary]
pub open spec fn proof_artifact_boundary_valid_spec(metadata: ProofArtifactMetadata) -> bool {
    metadata.local_property_id_present
        && metadata.content_scope_present
        && metadata.required_non_claims_present
        && overclaim_free_role_spec(metadata.claim_role)
}

// r[impl trellis.proof_artifact_provenance.identity]
// r[impl trellis.proof_artifact_provenance.boundary]
pub open spec fn proof_artifact_metadata_valid_spec(metadata: ProofArtifactMetadata) -> bool {
    proof_artifact_identity_valid_spec(metadata) && proof_artifact_boundary_valid_spec(metadata)
}

// r[impl trellis.proof_artifact_provenance.fixtures]
pub open spec fn proof_artifact_fixture_surface_spec() -> bool {
    true
}

// r[verify trellis.proof_artifact_provenance.fixtures]
pub proof fn proof_artifact_fixture_surface_declared()
    ensures proof_artifact_fixture_surface_spec()
{
}

// r[impl trellis.proof_artifact_provenance.identity]
pub fn failure_reason_admitted(
    status: ProofArtifactStatus,
    failure_reason_present: bool,
) -> (result: bool)
    ensures result == failure_reason_admitted_spec(status, failure_reason_present)
{
    match status {
        ProofArtifactStatus::Passing => true,
        ProofArtifactStatus::Failing => failure_reason_present,
    }
}

// r[impl trellis.proof_artifact_provenance.boundary]
pub fn overclaim_free_role(role: ProofArtifactClaimRole) -> (result: bool)
    ensures result == overclaim_free_role_spec(role)
{
    match role {
        ProofArtifactClaimRole::TrellisLocalProperty => true,
        ProofArtifactClaimRole::ManualReviewInput => true,
        ProofArtifactClaimRole::DownstreamCertified => false,
        ProofArtifactClaimRole::ReleaseEligibility => false,
        ProofArtifactClaimRole::WholeStackCorrectness => false,
        ProofArtifactClaimRole::Unsupported => false,
    }
}

// r[impl trellis.proof_artifact_provenance.handoff]
pub fn handoff_role_admitted(
    role: ProofArtifactClaimRole,
    downstream_receipt_present: bool,
) -> (result: bool)
    ensures result == handoff_role_admitted_spec(role, downstream_receipt_present)
{
    match role {
        ProofArtifactClaimRole::ManualReviewInput => true,
        ProofArtifactClaimRole::DownstreamCertified => downstream_receipt_present,
        _ => false,
    }
}

// r[impl trellis.proof_artifact_provenance.identity]
pub fn proof_artifact_identity_valid(metadata: ProofArtifactMetadata) -> (result: bool)
    ensures result == proof_artifact_identity_valid_spec(metadata)
{
    metadata.artifact_id_present
        && metadata.canonical_metadata_digest_matches
        && metadata.source_input_digests_present
        && metadata.proof_input_digests_present
        && metadata.verifier_receipt_digest_present
        && metadata.verifier_identity_present
        && metadata.normalized_invocation_present
        && metadata.local_property_id_present
        && failure_reason_admitted(metadata.status, metadata.failure_reason_present)
}

// r[impl trellis.proof_artifact_provenance.boundary]
pub fn proof_artifact_boundary_valid(metadata: ProofArtifactMetadata) -> (result: bool)
    ensures result == proof_artifact_boundary_valid_spec(metadata)
{
    metadata.local_property_id_present
        && metadata.content_scope_present
        && metadata.required_non_claims_present
        && overclaim_free_role(metadata.claim_role)
}

// r[impl trellis.proof_artifact_provenance.identity]
// r[impl trellis.proof_artifact_provenance.boundary]
pub fn proof_artifact_metadata_valid(metadata: ProofArtifactMetadata) -> (result: bool)
    ensures result == proof_artifact_metadata_valid_spec(metadata)
{
    proof_artifact_identity_valid(metadata) && proof_artifact_boundary_valid(metadata)
}

// r[verify trellis.proof_artifact_provenance.boundary]
pub proof fn valid_metadata_has_local_scope(metadata: ProofArtifactMetadata)
    requires proof_artifact_metadata_valid_spec(metadata)
    ensures metadata.local_property_id_present, metadata.content_scope_present
{
}

// r[verify trellis.proof_artifact_provenance.boundary]
pub proof fn valid_metadata_has_required_non_claims(metadata: ProofArtifactMetadata)
    requires proof_artifact_metadata_valid_spec(metadata)
    ensures metadata.required_non_claims_present
{
}

// r[verify trellis.proof_artifact_provenance.boundary]
pub proof fn valid_metadata_excludes_downstream_overclaim(metadata: ProofArtifactMetadata)
    requires proof_artifact_metadata_valid_spec(metadata)
    ensures overclaim_free_role_spec(metadata.claim_role)
{
}

// r[verify trellis.proof_artifact_provenance.identity]
pub proof fn stale_digest_rejected(metadata: ProofArtifactMetadata)
    requires !metadata.canonical_metadata_digest_matches
    ensures !proof_artifact_metadata_valid_spec(metadata)
{
}

// r[verify trellis.proof_artifact_provenance.handoff]
pub proof fn downstream_certified_handoff_requires_receipt()
    ensures !handoff_role_admitted_spec(ProofArtifactClaimRole::DownstreamCertified, false)
{
}

} // verus!

#[cfg(test)]
mod tests {
    use super::*;

    fn passing_metadata() -> ProofArtifactMetadata {
        ProofArtifactMetadata {
            artifact_id_present: true,
            canonical_metadata_digest_matches: true,
            source_input_digests_present: true,
            proof_input_digests_present: true,
            verifier_receipt_digest_present: true,
            verifier_identity_present: true,
            normalized_invocation_present: true,
            local_property_id_present: true,
            content_scope_present: true,
            status: ProofArtifactStatus::Passing,
            failure_reason_present: false,
            required_non_claims_present: true,
            claim_role: ProofArtifactClaimRole::TrellisLocalProperty,
            downstream_receipt_present: false,
        }
    }

    // r[verify trellis.proof_artifact_provenance.fixtures]
    #[test]
    fn passing_local_artifact_metadata_is_accepted() {
        assert!(proof_artifact_metadata_valid(passing_metadata()));
    }

    #[test]
    fn failing_local_artifact_with_reason_is_accepted() {
        let mut metadata = passing_metadata();
        metadata.status = ProofArtifactStatus::Failing;
        metadata.failure_reason_present = true;
        assert!(proof_artifact_metadata_valid(metadata));
    }

    // r[verify trellis.proof_artifact_provenance.fixtures]
    #[test]
    fn stale_digest_fails_closed() {
        let mut metadata = passing_metadata();
        metadata.canonical_metadata_digest_matches = false;
        assert!(!proof_artifact_metadata_valid(metadata));
    }

    #[test]
    fn missing_local_property_scope_fails_closed() {
        let mut metadata = passing_metadata();
        metadata.content_scope_present = false;
        assert!(!proof_artifact_metadata_valid(metadata));
    }

    #[test]
    fn missing_verifier_identity_fails_closed() {
        let mut metadata = passing_metadata();
        metadata.verifier_identity_present = false;
        assert!(!proof_artifact_metadata_valid(metadata));
    }

    #[test]
    fn missing_non_claims_fail_closed() {
        let mut metadata = passing_metadata();
        metadata.required_non_claims_present = false;
        assert!(!proof_artifact_metadata_valid(metadata));
    }

    #[test]
    fn unsupported_claim_role_fails_closed() {
        let mut metadata = passing_metadata();
        metadata.claim_role = ProofArtifactClaimRole::Unsupported;
        assert!(!proof_artifact_metadata_valid(metadata));
    }

    #[test]
    fn downstream_overclaim_fails_closed() {
        let mut metadata = passing_metadata();
        metadata.claim_role = ProofArtifactClaimRole::DownstreamCertified;
        metadata.downstream_receipt_present = true;
        assert!(!proof_artifact_metadata_valid(metadata));
    }

    #[test]
    fn failed_artifact_requires_failure_reason() {
        let mut metadata = passing_metadata();
        metadata.status = ProofArtifactStatus::Failing;
        metadata.failure_reason_present = false;
        assert!(!proof_artifact_metadata_valid(metadata));
    }

    #[test]
    fn handoff_rejects_downstream_certified_without_receipt() {
        assert!(!handoff_role_admitted(
            ProofArtifactClaimRole::DownstreamCertified,
            false,
        ));
        assert!(handoff_role_admitted(
            ProofArtifactClaimRole::DownstreamCertified,
            true,
        ));
        assert!(handoff_role_admitted(
            ProofArtifactClaimRole::ManualReviewInput,
            false,
        ));
    }
}
