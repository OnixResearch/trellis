//! Proof artifacts for compatibility and deterministic selection.

use vstd::prelude::*;

use super::admission::*;
use super::model::*;
use super::resolver::*;

verus! {

// r[verify rustc-metadata.compatibility]
// r[verify rustc-metadata.identity-fields]
pub proof fn accepted_candidate_matches_requested_fields(artifact: Artifact, request: Request)
    requires compatibility_spec(artifact, request).reason == Reason::Accepted
    ensures
        artifact.crate_name_id == request.crate_name_id,
        artifact.compiler_version_id == request.compiler_version_id,
        artifact.strict_version_hash == request.strict_version_hash,
        artifact.stable_crate_id == request.stable_crate_id,
{}

// r[verify rustc-metadata.rmeta-pipelining]
pub proof fn rmeta_cannot_satisfy_link_use(
    candidate_id: u64,
    crate_name_id: u64,
    compiler_version_id: u64,
    strict_version_hash: u64,
    stable_crate_id: u64,
)
    requires
        candidate_id > 0,
        crate_name_id > 0,
        compiler_version_id > 0,
        strict_version_hash > 0,
        stable_crate_id > 0,
    ensures compatibility_spec(
        Artifact {
            candidate_id,
            crate_name_id,
            compiler_version_id,
            strict_version_hash,
            stable_crate_id,
            kind: LibraryKind::Rmeta,
            metadata: Capability::Present,
            linkable_code: Capability::Absent,
            encoded_mir: Capability::Absent,
        },
        Request {
            crate_name_id,
            compiler_version_id,
            strict_version_hash,
            stable_crate_id,
            usage: UsageMode::Link,
            encoded_mir: Capability::Absent,
        },
    ).reason == Reason::NotLinkable
{}

// r[verify rustc-metadata.dependency-loading]
pub proof fn accepted_selection_names_one_compatible_candidate(
    candidates: Seq<Artifact>,
    request: Request,
    selection: Selection,
)
    requires
        selection_spec(candidates, request, selection),
        selection.reason == Reason::Accepted,
    ensures exists|index: int| {
        &&& 0 <= index < candidates.len()
        &&& candidate_matches_spec(candidates[index], request)
        &&& selection.selected_candidate_id == candidates[index].candidate_id
        &&& forall|other: int| #![trigger candidates[other]]
            0 <= other < candidates.len() && other != index
                ==> !candidate_matches_spec(candidates[other], request)
    }
{}

// r[verify rustc-metadata.dependency-loading]
pub proof fn ambiguous_selection_names_two_compatible_candidates(
    candidates: Seq<Artifact>,
    request: Request,
    selection: Selection,
)
    requires
        selection_spec(candidates, request, selection),
        selection.reason == Reason::AmbiguousCandidates,
    ensures exists|first: int, second: int| {
        &&& 0 <= first < second < candidates.len()
        &&& candidate_matches_spec(candidates[first], request)
        &&& candidate_matches_spec(candidates[second], request)
        &&& selection.selected_candidate_id == candidates[first].candidate_id
        &&& selection.conflicting_candidate_id == candidates[second].candidate_id
    }
{}

} // verus!
