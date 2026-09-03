use super::*;

// r[verify rustc-metadata.library-kind]
// r[verify rustc-metadata.admission]
#[test]
fn malformed_artifact_reports_the_same_first_failure() {
    let malformed = Artifact {
        candidate_id: NO_CANDIDATE_ID,
        crate_name_id: 0,
        compiler_version_id: 0,
        metadata: Capability::Absent,
        ..artifact(LibraryKind::Rmeta)
    };
    let first = admit_artifact(malformed);
    let second = admit_artifact(malformed);
    assert_eq!(first, second);
    assert_eq!(first.reason, Reason::MissingCandidateId);
}

// r[verify rustc-metadata.library-kind]
#[test]
fn missing_metadata_and_invalid_kind_capabilities_are_rejected() {
    let missing = Artifact {
        metadata: Capability::Absent,
        ..artifact(LibraryKind::Rlib)
    };
    assert_eq!(admit_artifact(missing).reason, Reason::MissingMetadata);

    let invalid_rmeta = Artifact {
        linkable_code: Capability::Present,
        ..artifact(LibraryKind::Rmeta)
    };
    assert_eq!(admit_artifact(invalid_rmeta).reason, Reason::KindCapabilityMismatch);

    let invalid_rlib = Artifact {
        linkable_code: Capability::Absent,
        ..artifact(LibraryKind::Rlib)
    };
    assert_eq!(admit_artifact(invalid_rlib).reason, Reason::KindCapabilityMismatch);
}

// r[verify rustc-metadata.identity-fields]
// r[verify rustc-metadata.compatibility]
#[test]
fn identity_mismatches_are_rejected_in_stable_order() {
    let candidate = artifact(LibraryKind::Rlib);

    let wrong_name = Request {
        crate_name_id: OTHER_CRATE_NAME_ID,
        ..request(UsageMode::Check)
    };
    assert_eq!(compatible(candidate, wrong_name).reason, Reason::CrateNameMismatch);

    let wrong_version = Request {
        compiler_version_id: OTHER_COMPILER_VERSION_ID,
        ..request(UsageMode::Check)
    };
    assert_eq!(compatible(candidate, wrong_version).reason, Reason::CompilerVersionMismatch);

    let wrong_svh = Request {
        strict_version_hash: OTHER_STRICT_VERSION_HASH,
        ..request(UsageMode::Check)
    };
    assert_eq!(compatible(candidate, wrong_svh).reason, Reason::StrictVersionHashMismatch);

    let wrong_stable_id = Request {
        stable_crate_id: OTHER_STABLE_CRATE_ID,
        ..request(UsageMode::Check)
    };
    assert_eq!(compatible(candidate, wrong_stable_id).reason, Reason::StableCrateIdMismatch);
}

// r[verify rustc-metadata.rmeta-pipelining]
#[test]
fn rmeta_does_not_satisfy_final_linking() {
    let candidate = artifact(LibraryKind::Rmeta);
    assert_eq!(compatible(candidate, request(UsageMode::Link)).reason, Reason::NotLinkable);
}

// r[verify rustc-metadata.admission]
#[test]
fn required_encoded_mir_must_be_present() {
    let candidate = Artifact {
        encoded_mir: Capability::Absent,
        ..artifact(LibraryKind::Rlib)
    };
    let codegen = Request {
        usage: UsageMode::Codegen,
        encoded_mir: Capability::Present,
        ..request(UsageMode::Codegen)
    };
    assert_eq!(compatible(candidate, codegen).reason, Reason::EncodedMirMissing);
}

// r[verify rustc-metadata.dependency-loading]
#[test]
fn no_match_and_ambiguity_are_distinct() {
    let wrong = Artifact {
        candidate_id: OTHER_CANDIDATE_ID,
        strict_version_hash: OTHER_STRICT_VERSION_HASH,
        ..artifact(LibraryKind::Rlib)
    };
    let none = select_candidate(&[wrong], request(UsageMode::Check));
    assert_eq!(none.reason, Reason::NoCompatibleCandidate);

    let first = artifact(LibraryKind::Rlib);
    let second = Artifact {
        candidate_id: OTHER_CANDIDATE_ID,
        ..first
    };
    let ambiguous = select_candidate(&[first, second], request(UsageMode::Check));
    assert_eq!(ambiguous.reason, Reason::AmbiguousCandidates);
    assert_eq!(ambiguous.selected_candidate_id, CANDIDATE_ID);
    assert_eq!(ambiguous.conflicting_candidate_id, OTHER_CANDIDATE_ID);
}

// r[verify rustc-metadata.dependency-loading]
#[test]
fn candidate_limit_is_checked_before_selection() {
    let excessive = vec![artifact(LibraryKind::Rlib); MAX_CANDIDATES + 1];
    let result = select_candidate(&excessive, request(UsageMode::Check));
    assert_eq!(result.reason, Reason::CandidateLimitExceeded);
    assert_eq!(result.selected_candidate_id, NO_CANDIDATE_ID);
}
