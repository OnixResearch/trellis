//! Library-kind and dependency-request compatibility admission.

use vstd::prelude::*;

use super::model::*;

verus! {

// r[impl rustc-metadata.library-kind]
pub open spec fn capability_present_spec(capability: Capability) -> bool {
    match capability {
        Capability::Absent => false,
        Capability::Present => true,
    }
}

// r[impl rustc-metadata.library-kind]
pub fn capability_present(capability: Capability) -> (result: bool)
    ensures result == capability_present_spec(capability)
{
    matches!(capability, Capability::Present)
}

// r[impl rustc-metadata.compatibility]
pub open spec fn decision_is_accepted_spec(decision: Decision) -> bool {
    match decision.reason {
        Reason::Accepted => true,
        _ => false,
    }
}

// r[impl rustc-metadata.compatibility]
pub fn decision_is_accepted(decision: Decision) -> (result: bool)
    ensures result == decision_is_accepted_spec(decision)
{
    matches!(decision.reason, Reason::Accepted)
}

// r[impl rustc-metadata.library-kind]
pub open spec fn kind_capabilities_valid_spec(artifact: Artifact) -> bool {
    capability_present_spec(artifact.metadata)
        && match artifact.kind {
            LibraryKind::Rlib | LibraryKind::Dylib => {
                capability_present_spec(artifact.linkable_code)
            },
            LibraryKind::Rmeta => !capability_present_spec(artifact.linkable_code),
        }
}

// r[impl rustc-metadata.library-kind]
pub fn kind_capabilities_valid(artifact: Artifact) -> (result: bool)
    ensures result == kind_capabilities_valid_spec(artifact)
{
    capability_present(artifact.metadata)
        && match artifact.kind {
            LibraryKind::Rlib | LibraryKind::Dylib => capability_present(artifact.linkable_code),
            LibraryKind::Rmeta => !capability_present(artifact.linkable_code),
        }
}

// r[impl rustc-metadata.library-kind]
// r[impl rustc-metadata.admission]
pub open spec fn artifact_admission_spec(artifact: Artifact) -> Decision {
    let reason = if artifact.candidate_id == NO_CANDIDATE_ID {
        Reason::MissingCandidateId
    } else if artifact.crate_name_id == 0 {
        Reason::MissingCrateName
    } else if artifact.compiler_version_id == 0 {
        Reason::MissingCompilerVersion
    } else if artifact.strict_version_hash == 0 {
        Reason::MissingStrictVersionHash
    } else if artifact.stable_crate_id == 0 {
        Reason::MissingStableCrateId
    } else if !capability_present_spec(artifact.metadata) {
        Reason::MissingMetadata
    } else if !kind_capabilities_valid_spec(artifact) {
        Reason::KindCapabilityMismatch
    } else {
        Reason::Accepted
    };
    Decision { reason, candidate_id: artifact.candidate_id }
}

// r[impl rustc-metadata.library-kind]
// r[impl rustc-metadata.admission]
pub fn admit_artifact(artifact: Artifact) -> (result: Decision)
    ensures result == artifact_admission_spec(artifact)
{
    if artifact.candidate_id == NO_CANDIDATE_ID {
        return Decision { reason: Reason::MissingCandidateId, candidate_id: artifact.candidate_id };
    }
    if artifact.crate_name_id == 0 {
        return Decision { reason: Reason::MissingCrateName, candidate_id: artifact.candidate_id };
    }
    if artifact.compiler_version_id == 0 {
        return Decision { reason: Reason::MissingCompilerVersion, candidate_id: artifact.candidate_id };
    }
    if artifact.strict_version_hash == 0 {
        return Decision { reason: Reason::MissingStrictVersionHash, candidate_id: artifact.candidate_id };
    }
    if artifact.stable_crate_id == 0 {
        return Decision { reason: Reason::MissingStableCrateId, candidate_id: artifact.candidate_id };
    }
    if !capability_present(artifact.metadata) {
        return Decision { reason: Reason::MissingMetadata, candidate_id: artifact.candidate_id };
    }
    if !kind_capabilities_valid(artifact) {
        return Decision { reason: Reason::KindCapabilityMismatch, candidate_id: artifact.candidate_id };
    }
    Decision { reason: Reason::Accepted, candidate_id: artifact.candidate_id }
}

// r[impl rustc-metadata.compatibility]
pub open spec fn request_complete_spec(request: Request) -> bool {
    request.crate_name_id != 0
        && request.compiler_version_id != 0
        && request.strict_version_hash != 0
        && request.stable_crate_id != 0
}

// r[impl rustc-metadata.compatibility]
pub fn request_complete(request: Request) -> (result: bool)
    ensures result == request_complete_spec(request)
{
    request.crate_name_id != 0
        && request.compiler_version_id != 0
        && request.strict_version_hash != 0
        && request.stable_crate_id != 0
}

// r[impl rustc-metadata.rmeta-pipelining]
pub open spec fn usage_requires_link_spec(usage: UsageMode) -> bool {
    match usage {
        UsageMode::Link => true,
        UsageMode::Check | UsageMode::Document | UsageMode::Pipeline | UsageMode::Codegen => false,
    }
}

// r[impl rustc-metadata.rmeta-pipelining]
pub fn usage_requires_link(usage: UsageMode) -> (result: bool)
    ensures result == usage_requires_link_spec(usage)
{
    matches!(usage, UsageMode::Link)
}

// r[impl rustc-metadata.identity-fields]
// r[impl rustc-metadata.compatibility]
// r[impl rustc-metadata.admission]
pub open spec fn compatibility_spec(artifact: Artifact, request: Request) -> Decision {
    let artifact_decision = artifact_admission_spec(artifact);
    if !decision_is_accepted_spec(artifact_decision) {
        artifact_decision
    } else {
        let reason = if !request_complete_spec(request) {
        Reason::IncompleteRequest
    } else if artifact.crate_name_id != request.crate_name_id {
        Reason::CrateNameMismatch
    } else if artifact.compiler_version_id != request.compiler_version_id {
        Reason::CompilerVersionMismatch
    } else if artifact.strict_version_hash != request.strict_version_hash {
        Reason::StrictVersionHashMismatch
    } else if artifact.stable_crate_id != request.stable_crate_id {
        Reason::StableCrateIdMismatch
    } else if usage_requires_link_spec(request.usage)
        && !capability_present_spec(artifact.linkable_code) {
        Reason::NotLinkable
    } else if capability_present_spec(request.encoded_mir)
        && !capability_present_spec(artifact.encoded_mir) {
        Reason::EncodedMirMissing
        } else {
            Reason::Accepted
        };
        Decision { reason, candidate_id: artifact.candidate_id }
    }
}

// r[impl rustc-metadata.identity-fields]
// r[impl rustc-metadata.compatibility]
// r[impl rustc-metadata.admission]
pub fn compatible(artifact: Artifact, request: Request) -> (result: Decision)
    ensures result == compatibility_spec(artifact, request)
{
    let artifact_decision = admit_artifact(artifact);
    if !decision_is_accepted(artifact_decision) {
        return artifact_decision;
    }
    if !request_complete(request) {
        return Decision { reason: Reason::IncompleteRequest, candidate_id: artifact.candidate_id };
    }
    if artifact.crate_name_id != request.crate_name_id {
        return Decision { reason: Reason::CrateNameMismatch, candidate_id: artifact.candidate_id };
    }
    if artifact.compiler_version_id != request.compiler_version_id {
        return Decision { reason: Reason::CompilerVersionMismatch, candidate_id: artifact.candidate_id };
    }
    if artifact.strict_version_hash != request.strict_version_hash {
        return Decision { reason: Reason::StrictVersionHashMismatch, candidate_id: artifact.candidate_id };
    }
    if artifact.stable_crate_id != request.stable_crate_id {
        return Decision { reason: Reason::StableCrateIdMismatch, candidate_id: artifact.candidate_id };
    }
    if usage_requires_link(request.usage) && !capability_present(artifact.linkable_code) {
        return Decision { reason: Reason::NotLinkable, candidate_id: artifact.candidate_id };
    }
    if capability_present(request.encoded_mir) && !capability_present(artifact.encoded_mir) {
        return Decision { reason: Reason::EncodedMirMissing, candidate_id: artifact.candidate_id };
    }
    Decision { reason: Reason::Accepted, candidate_id: artifact.candidate_id }
}

} // verus!
