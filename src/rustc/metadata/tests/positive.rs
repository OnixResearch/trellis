use super::*;

// r[verify rustc-metadata.library-kind]
// r[verify rustc-metadata.compatibility]
#[test]
fn rlib_and_dylib_support_metadata_and_link_use() {
    for kind in [LibraryKind::Rlib, LibraryKind::Dylib] {
        let candidate = artifact(kind);
        assert_eq!(admit_artifact(candidate).reason, Reason::Accepted);
        assert_eq!(compatible(candidate, request(UsageMode::Link)).reason, Reason::Accepted);
    }
}

// r[verify rustc-metadata.library-kind]
// r[verify rustc-metadata.rmeta-pipelining]
#[test]
fn rmeta_supports_metadata_only_modes() {
    let candidate = artifact(LibraryKind::Rmeta);
    for usage in [UsageMode::Check, UsageMode::Document, UsageMode::Pipeline] {
        assert_eq!(compatible(candidate, request(usage)).reason, Reason::Accepted);
    }
}

// r[verify rustc-metadata.admission]
#[test]
fn codegen_use_accepts_declared_encoded_mir() {
    let candidate = artifact(LibraryKind::Rmeta);
    let codegen = Request {
        usage: UsageMode::Codegen,
        encoded_mir: Capability::Present,
        ..request(UsageMode::Codegen)
    };
    assert_eq!(compatible(candidate, codegen).reason, Reason::Accepted);
}

// r[verify rustc-metadata.identity-fields]
#[test]
fn exact_rustc_identity_fields_are_compatible() {
    let candidate = artifact(LibraryKind::Rlib);
    let decision = compatible(candidate, request(UsageMode::Check));
    assert_eq!(decision.reason, Reason::Accepted);
    assert_eq!(decision.candidate_id, CANDIDATE_ID);
}

// r[verify rustc-metadata.dependency-loading]
#[test]
fn unique_compatible_candidate_is_selected() {
    let incompatible = Artifact {
        candidate_id: OTHER_CANDIDATE_ID,
        compiler_version_id: OTHER_COMPILER_VERSION_ID,
        ..artifact(LibraryKind::Rlib)
    };
    let compatible_candidate = Artifact {
        candidate_id: THIRD_CANDIDATE_ID,
        ..artifact(LibraryKind::Rlib)
    };
    let candidates = vec![incompatible, compatible_candidate];
    let selected = select_candidate(&candidates, request(UsageMode::Link));
    assert_eq!(selected.reason, Reason::Accepted);
    assert_eq!(selected.selected_candidate_id, THIRD_CANDIDATE_ID);
    assert_eq!(selected.conflicting_candidate_id, NO_CANDIDATE_ID);
}

// r[verify rustc-metadata.evidence]
// r[verify rustc-metadata.non-claims]
#[test]
fn model_uses_declared_rustc_fields_without_rehashing() {
    let candidate = artifact(LibraryKind::Rlib);
    assert_eq!(candidate.strict_version_hash, STRICT_VERSION_HASH);
    assert_eq!(candidate.stable_crate_id, STABLE_CRATE_ID);
    assert_eq!(compatible(candidate, request(UsageMode::Check)).reason, Reason::Accepted);
}
