//! Verified core Raft consensus primitives for the domain-primitives Raft OpenSpec.
//!
//! This module intentionally stays at the exact finite model boundary: term
//! update, vote/request-vote admission, and election-safety composition over
//! existing verified quorum/ballot/ghost-history primitives. It does not claim
//! networking, storage durability, wall-clock timing, byzantine tolerance, or
//! liveness.

#![allow(clippy::wildcard_imports)]

use vstd::prelude::*;

use crate::ballot::*;
use crate::election_safety_proof::*;
use crate::epoch::*;
use crate::ghost_history::*;

verus! {

/// Minimal term-bearing local state used by the finite Raft model slice.
pub struct RaftTermState {
    /// The node's current term.
    pub current_term: u64,
}

// r[depends domain-primitives.raft-consensus.term]
/// Spec: processing an observed term never decreases the current term.
pub open spec fn observed_term_update_spec(current_term: u64, observed_term: u64) -> u64 {
    if observed_term > current_term { observed_term } else { current_term }
}

// r[depends domain-primitives.raft-consensus.message]
/// Spec: a term-bearing message is stale exactly when its term is below the receiver term.
pub open spec fn stale_term_message_spec(current_term: u64, observed_term: u64) -> bool {
    observed_term < current_term
}

// r[impl domain-primitives.raft-consensus.term]
/// Return the successor current term after observing a term-bearing Raft input.
pub fn observed_term_update(current_term: u64, observed_term: u64) -> (result: u64)
    ensures
        result == observed_term_update_spec(current_term, observed_term),
        result >= current_term,
        result >= observed_term || observed_term < current_term,
{
    if observed_term > current_term { observed_term } else { current_term }
}

// r[impl domain-primitives.raft-consensus.message]
/// Return whether a term-bearing Raft input is stale for the receiver.
pub fn stale_term_message(current_term: u64, observed_term: u64) -> (result: bool)
    ensures result == stale_term_message_spec(current_term, observed_term)
{
    observed_term < current_term
}

// r[verify domain-primitives.raft-consensus.term]
/// Observing any term is a monotone local-term transition.
pub proof fn observed_term_update_monotone(current_term: u64, observed_term: u64)
    ensures
        observed_term_update_spec(current_term, observed_term) >= current_term,
        observed_term_update_spec(current_term, observed_term) == current_term
            || observed_term_update_spec(current_term, observed_term) == observed_term,
{}

// r[verify domain-primitives.raft-consensus.message]
/// Lower observed terms are rejected as stale and cannot decrease the receiver term.
pub proof fn stale_observed_term_keeps_current(current_term: u64, observed_term: u64)
    requires observed_term < current_term
    ensures
        stale_term_message_spec(current_term, observed_term),
        observed_term_update_spec(current_term, observed_term) == current_term,
{}

// r[depends domain-primitives.raft-consensus.vote]
/// Spec: finite Raft vote admission delegates to the verified ballot predicate.
pub open spec fn raft_vote_grant_spec(
    state: VoteState,
    candidate: u64,
    candidate_term: u64,
    candidate_last: Epoch,
    voter_last: Epoch,
) -> bool {
    can_grant_vote_spec(state, candidate, candidate_term, candidate_last, voter_last)
}

// r[depends domain-primitives.raft-consensus.request-vote]
/// Spec: RequestVote admission for this slice is the vote-grant predicate plus non-stale term.
pub open spec fn request_vote_admit_spec(
    state: VoteState,
    candidate: u64,
    candidate_term: u64,
    candidate_last: Epoch,
    voter_last: Epoch,
) -> bool {
    !stale_term_message_spec(state.voted_term, candidate_term)
        && raft_vote_grant_spec(state, candidate, candidate_term, candidate_last, voter_last)
}

// r[impl domain-primitives.raft-consensus.vote]
/// Return whether the finite vote-grant predicate admits this candidate.
pub fn raft_vote_grant(
    state: &VoteState,
    candidate: u64,
    candidate_term: u64,
    candidate_last: &Epoch,
    voter_last: &Epoch,
) -> (result: bool)
    ensures result == raft_vote_grant_spec(*state, candidate, candidate_term, *candidate_last, *voter_last)
{
    can_grant_vote(state, candidate, candidate_term, candidate_last, voter_last)
}

// r[impl domain-primitives.raft-consensus.request-vote]
/// Return whether a finite RequestVote input is admitted by term and log freshness checks.
pub fn request_vote_admit(
    state: &VoteState,
    candidate: u64,
    candidate_term: u64,
    candidate_last: &Epoch,
    voter_last: &Epoch,
) -> (result: bool)
    ensures result == request_vote_admit_spec(*state, candidate, candidate_term, *candidate_last, *voter_last)
{
    if stale_term_message(state.voted_term, candidate_term) {
        false
    } else {
        raft_vote_grant(state, candidate, candidate_term, candidate_last, voter_last)
    }
}

// r[verify domain-primitives.raft-consensus.vote]
// r[verify domain-primitives.raft-consensus.request-vote]
/// Once a voter records a vote for one candidate, a distinct candidate is rejected in the same term.
pub proof fn second_vote_same_term_rejected(
    candidate_a: u64,
    candidate_b: u64,
    term: u64,
    candidate_b_last: Epoch,
    voter_last: Epoch,
)
    requires candidate_a != candidate_b
    ensures !request_vote_admit_spec(
        grant_vote_spec(candidate_a, term),
        candidate_b,
        term,
        candidate_b_last,
        voter_last,
    )
{}

// r[depends domain-primitives.raft-consensus.election-safety]
/// Spec: two admitted leaders for the same term are compatible only if they are the same node.
pub open spec fn same_term_leaders_compatible_spec(candidate_a: u64, candidate_b: u64) -> bool {
    candidate_a == candidate_b
}

// r[impl domain-primitives.raft-consensus.election-safety]
/// Return whether two same-term leader witnesses are compatible with election safety.
pub fn same_term_leaders_compatible(candidate_a: u64, candidate_b: u64) -> (result: bool)
    ensures result == same_term_leaders_compatible_spec(candidate_a, candidate_b)
{
    candidate_a == candidate_b
}

// r[verify domain-primitives.raft-consensus.election-safety]
/// Distinct same-term leader witnesses violate election safety.
pub proof fn distinct_same_term_leaders_incompatible(candidate_a: u64, candidate_b: u64)
    requires candidate_a != candidate_b
    ensures !same_term_leaders_compatible_spec(candidate_a, candidate_b)
{}

// r[depends domain-primitives.raft-consensus.election-safety]
/// Spec: the finite vote history satisfies Raft election safety for a term.
pub open spec fn raft_election_safe_spec(
    vote_history: Seq<VoteRecord>,
    term: u64,
    cluster_size: u64,
) -> bool
    recommends cluster_size > 0
{
    election_safe_spec(vote_history, term, cluster_size)
}

// r[verify domain-primitives.raft-consensus.election-safety]
/// Single-vote-per-term histories compose with quorum intersection into election safety.
pub proof fn prove_raft_election_safety(
    vote_history: Seq<VoteRecord>,
    term: u64,
    cluster_size: u64,
)
    requires
        cluster_size > 0,
        single_vote_per_term_spec(vote_history),
    ensures raft_election_safe_spec(vote_history, term, cluster_size)
{
    election_safety_theorem(vote_history, term, cluster_size);
}

} // verus!

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ballot::VoteState;
    use crate::epoch::Epoch;

    fn epoch(term: u64, index: u64) -> Epoch {
        Epoch { term, index }
    }

    // r[verify domain-primitives.raft-consensus.term]
    #[test]
    fn observed_term_update_is_monotone() {
        assert_eq!(observed_term_update(5, 3), 5);
        assert_eq!(observed_term_update(5, 5), 5);
        assert_eq!(observed_term_update(5, 8), 8);
    }

    // r[verify domain-primitives.raft-consensus.message]
    #[test]
    fn stale_term_detection_matches_ordering() {
        assert!(stale_term_message(5, 4));
        assert!(!stale_term_message(5, 5));
        assert!(!stale_term_message(5, 6));
    }

    // r[verify domain-primitives.raft-consensus.election-safety]
    #[test]
    fn same_term_leader_witnesses_must_match() {
        assert!(same_term_leaders_compatible(7, 7));
        assert!(!same_term_leaders_compatible(7, 8));
    }

    // r[verify domain-primitives.raft-consensus.vote]
    // r[verify domain-primitives.raft-consensus.request-vote]
    #[test]
    fn request_vote_rejects_second_candidate_same_term() {
        let voted = VoteState {
            voted_term: 7,
            voted_for: 11,
        };
        assert!(!request_vote_admit(&voted, 12, 7, &epoch(7, 10), &epoch(7, 9)));
        assert!(request_vote_admit(&voted, 11, 7, &epoch(7, 10), &epoch(7, 9)));
    }

    // r[verify domain-primitives.raft-consensus.request-vote]
    #[test]
    fn request_vote_rejects_stale_log() {
        let fresh = VoteState {
            voted_term: 0,
            voted_for: 0,
        };
        assert!(!request_vote_admit(&fresh, 12, 1, &epoch(1, 3), &epoch(2, 0)));
    }
}
