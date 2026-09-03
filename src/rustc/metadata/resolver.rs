//! Bounded deterministic dependency-candidate selection.

use vstd::prelude::*;

use super::admission::*;
use super::model::*;

verus! {

// r[impl rustc-metadata.dependency-loading]
pub open spec fn candidate_matches_spec(artifact: Artifact, request: Request) -> bool {
    decision_is_accepted_spec(compatibility_spec(artifact, request))
}

// r[impl rustc-metadata.dependency-loading]
pub fn candidate_matches(artifact: Artifact, request: Request) -> (result: bool)
    ensures result == candidate_matches_spec(artifact, request)
{
    decision_is_accepted(compatible(artifact, request))
}

// r[impl rustc-metadata.dependency-loading]
pub open spec fn no_match_in_range_spec(
    candidates: Seq<Artifact>,
    request: Request,
    start: int,
    end: int,
) -> bool {
    forall|index: int| #![trigger candidates[index]]
        start <= index < end
            ==> !candidate_matches_spec(candidates[index], request)
}

// r[impl rustc-metadata.dependency-loading]
pub open spec fn first_match_spec(
    candidates: Seq<Artifact>,
    request: Request,
    index: int,
) -> bool {
    &&& 0 <= index < candidates.len()
    &&& candidate_matches_spec(candidates[index], request)
    &&& no_match_in_range_spec(candidates, request, 0, index)
}

// r[impl rustc-metadata.dependency-loading]
pub open spec fn next_match_spec(
    candidates: Seq<Artifact>,
    request: Request,
    first: int,
    second: int,
) -> bool {
    &&& first_match_spec(candidates, request, first)
    &&& first < second < candidates.len()
    &&& candidate_matches_spec(candidates[second], request)
    &&& no_match_in_range_spec(candidates, request, first + 1, second)
}

// r[impl rustc-metadata.dependency-loading]
pub open spec fn selection_spec(
    candidates: Seq<Artifact>,
    request: Request,
    selection: Selection,
) -> bool {
    if candidates.len() > MAX_CANDIDATES as int {
        &&& selection.reason == Reason::CandidateLimitExceeded
        &&& selection.selected_candidate_id == NO_CANDIDATE_ID
        &&& selection.conflicting_candidate_id == NO_CANDIDATE_ID
    } else {
        match selection.reason {
            Reason::NoCompatibleCandidate => {
                &&& selection.selected_candidate_id == NO_CANDIDATE_ID
                &&& selection.conflicting_candidate_id == NO_CANDIDATE_ID
                &&& no_match_in_range_spec(candidates, request, 0, candidates.len() as int)
            },
            Reason::Accepted => {
                &&& selection.conflicting_candidate_id == NO_CANDIDATE_ID
                &&& exists|index: int| {
                    &&& first_match_spec(candidates, request, index)
                    &&& no_match_in_range_spec(candidates, request, index + 1, candidates.len() as int)
                    &&& selection.selected_candidate_id == candidates[index].candidate_id
                }
            },
            Reason::AmbiguousCandidates => {
                exists|first: int, second: int| {
                    &&& next_match_spec(candidates, request, first, second)
                    &&& selection.selected_candidate_id == candidates[first].candidate_id
                    &&& selection.conflicting_candidate_id == candidates[second].candidate_id
                }
            },
            Reason::MissingCandidateId
            | Reason::MissingCrateName
            | Reason::MissingCompilerVersion
            | Reason::MissingStrictVersionHash
            | Reason::MissingStableCrateId
            | Reason::MissingMetadata
            | Reason::KindCapabilityMismatch
            | Reason::IncompleteRequest
            | Reason::CrateNameMismatch
            | Reason::CompilerVersionMismatch
            | Reason::StrictVersionHashMismatch
            | Reason::StableCrateIdMismatch
            | Reason::NotLinkable
            | Reason::EncodedMirMissing
            | Reason::CandidateLimitExceeded => false,
        }
    }
}

fn ambiguous_result(
    candidates: &[Artifact],
    request: Request,
    first: usize,
    second: usize,
) -> (result: Selection)
    requires
        candidates.len() <= MAX_CANDIDATES,
        first < second < candidates.len(),
        next_match_spec(candidates@, request, first as int, second as int),
    ensures selection_spec(candidates@, request, result)
{
    let result = Selection {
        reason: Reason::AmbiguousCandidates,
        selected_candidate_id: candidates[first].candidate_id,
        conflicting_candidate_id: candidates[second].candidate_id,
    };
    assert(selection_spec(candidates@, request, result)) by {
        assert(exists|prior: int, conflict: int| {
            &&& prior == first as int
            &&& conflict == second as int
            &&& next_match_spec(candidates@, request, prior, conflict)
            &&& result.selected_candidate_id == candidates@[prior].candidate_id
            &&& result.conflicting_candidate_id == candidates@[conflict].candidate_id
        });
    }
    result
}

fn unique_result(candidates: &[Artifact], request: Request, selected: usize) -> (result: Selection)
    requires
        candidates.len() <= MAX_CANDIDATES,
        selected < candidates.len(),
        first_match_spec(candidates@, request, selected as int),
        no_match_in_range_spec(
            candidates@,
            request,
            selected as int + 1,
            candidates@.len() as int,
        ),
    ensures selection_spec(candidates@, request, result)
{
    let result = Selection {
        reason: Reason::Accepted,
        selected_candidate_id: candidates[selected].candidate_id,
        conflicting_candidate_id: NO_CANDIDATE_ID,
    };
    assert(selection_spec(candidates@, request, result)) by {
        assert(exists|index: int| {
            &&& index == selected as int
            &&& first_match_spec(candidates@, request, index)
            &&& no_match_in_range_spec(candidates@, request, index + 1, candidates@.len() as int)
            &&& result.selected_candidate_id == candidates@[index].candidate_id
        });
    }
    result
}

fn select_bounded(candidates: &[Artifact], request: Request) -> (result: Selection)
    requires candidates.len() <= MAX_CANDIDATES
    ensures selection_spec(candidates@, request, result)
{
    let candidate_count = candidates.len();
    let mut first_index: Option<usize> = None;
    let mut index: usize = 0;
    while index < candidate_count
        invariant
            candidate_count == candidates@.len(),
            candidate_count <= MAX_CANDIDATES,
            0 <= index <= candidate_count,
            match first_index {
                None => no_match_in_range_spec(candidates@, request, 0, index as int),
                Some(first) => {
                    &&& first < index
                    &&& first_match_spec(candidates@, request, first as int)
                    &&& no_match_in_range_spec(candidates@, request, first as int + 1, index as int)
                },
            },
        decreases candidate_count - index,
    {
        if candidate_matches(candidates[index], request) {
            match first_index {
                None => {
                    assert(first_match_spec(candidates@, request, index as int));
                    first_index = Some(index);
                },
                Some(first) => {
                    assert(next_match_spec(candidates@, request, first as int, index as int));
                    return ambiguous_result(candidates, request, first, index);
                },
            }
        }
        index = index + 1;
    }

    match first_index {
        None => Selection {
            reason: Reason::NoCompatibleCandidate,
            selected_candidate_id: NO_CANDIDATE_ID,
            conflicting_candidate_id: NO_CANDIDATE_ID,
        },
        Some(first) => {
            unique_result(candidates, request, first)
        },
    }
}

// r[impl rustc-metadata.dependency-loading]
// r[impl rustc-metadata.admission]
pub fn select_candidate(candidates: &[Artifact], request: Request) -> (result: Selection)
    ensures selection_spec(candidates@, request, result)
{
    if candidates.len() > MAX_CANDIDATES {
        Selection {
            reason: Reason::CandidateLimitExceeded,
            selected_candidate_id: NO_CANDIDATE_ID,
            conflicting_candidate_id: NO_CANDIDATE_ID,
        }
    } else {
        select_bounded(candidates, request)
    }
}

} // verus!
