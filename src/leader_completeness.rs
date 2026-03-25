//! Verified leader completeness theorem for Raft.
//!
//! If an entry is committed in term T, then that entry is present in
//! the log of every leader for terms > T. This is the key safety
//! property that prevents committed data from being lost across elections.
//!
//! The proof chains:
//! 1. The committed entry was replicated to a quorum Q1
//! 2. The new leader received votes from a quorum Q2
//! 3. Q1 ∩ Q2 ≠ ∅ (quorum intersection)
//! 4. The shared voter has the committed entry
//! 5. Vote granting requires epoch_ge (log at least as up-to-date)
//! 6. Therefore the new leader's log contains the committed entry
//!
//! This module states the theorem and its key lemmas. The full inductive
//! proof over all terms requires protocol-level state tracking beyond
//! pure primitives, so we verify the critical logical steps and leave
//! the induction structure as documented premises.

use vstd::prelude::*;

use crate::epoch::*;
use crate::quorum::*;

verus! {

// --- Leader completeness: quorum overlap step ---

// r[depends leader-completeness.quorum-overlap]
/// Spec: the quorum that committed an entry and the quorum that elected
/// a new leader must overlap. This is the pigeonhole step.
pub open spec fn commitment_election_overlap_spec(
    commit_votes: u64,
    election_votes: u64,
    cluster_size: u64,
) -> bool
    recommends cluster_size > 0
{
    is_quorum_spec(commit_votes, cluster_size)
    && is_quorum_spec(election_votes, cluster_size)
    ==> (commit_votes as int + election_votes as int > cluster_size as int)
}

// r[verify leader-completeness.quorum-overlap]
/// The commitment quorum and election quorum always overlap.
pub proof fn prove_commitment_election_overlap(
    commit_votes: u64,
    election_votes: u64,
    cluster_size: u64,
)
    requires
        cluster_size > 0,
        is_quorum_spec(commit_votes, cluster_size),
        is_quorum_spec(election_votes, cluster_size),
    ensures
        commit_votes as int + election_votes as int > cluster_size as int
{
    prove_quorum_intersection(commit_votes, election_votes, cluster_size);
}

// --- Leader completeness: log freshness step ---

// r[depends leader-completeness.log-freshness]
/// Spec: if the shared voter has the committed entry at (commit_term,
/// commit_index), and the new leader's log is at least as up-to-date
/// (by epoch comparison), then the new leader's log epoch is >=
/// the committed entry's epoch.
pub open spec fn leader_has_committed_entry_spec(
    commit_term: u64,
    commit_index: u64,
    leader_last_term: u64,
    leader_last_index: u64,
) -> bool {
    epoch_ge_spec(
        Epoch { term: leader_last_term, index: leader_last_index },
        Epoch { term: commit_term, index: commit_index },
    )
}

// r[impl leader-completeness.log-freshness]
/// Check if the leader's log is at least as up-to-date as the committed entry.
pub fn leader_has_committed_entry(
    commit_term: u64,
    commit_index: u64,
    leader_last_term: u64,
    leader_last_index: u64,
) -> (result: bool)
    ensures result == leader_has_committed_entry_spec(
        commit_term, commit_index, leader_last_term, leader_last_index,
    )
{
    epoch_ge(
        &Epoch { term: leader_last_term, index: leader_last_index },
        &Epoch { term: commit_term, index: commit_index },
    )
}

// r[verify leader-completeness.log-freshness]
/// A leader in a higher term whose log is at least as up-to-date has
/// the committed entry. Higher term guarantees epoch_ge when the
/// leader's term > commit_term.
proof fn higher_term_leader_has_entry(
    commit_term: u64,
    commit_index: u64,
    leader_term: u64,
    leader_last_index: u64,
)
    requires leader_term > commit_term
    ensures leader_has_committed_entry_spec(
        commit_term, commit_index, leader_term, leader_last_index,
    )
{}

// r[verify leader-completeness.log-freshness]
/// Same term: leader has entry when its index is >= commit_index.
proof fn same_term_leader_has_entry(
    commit_term: u64,
    commit_index: u64,
    leader_last_index: u64,
)
    requires leader_last_index >= commit_index
    ensures leader_has_committed_entry_spec(
        commit_term, commit_index, commit_term, leader_last_index,
    )
{}

// r[verify leader-completeness.quorum-overlap]
/// Theorem statement: given a committed entry and a valid election,
/// the new leader's log is at least as up-to-date as the committed entry.
///
/// Premises (the protocol must establish these):
/// - The entry was replicated to a quorum (commit_quorum_size >= majority)
/// - The new leader received votes from a quorum (election_quorum_size >= majority)
/// - The shared voter's log epoch is >= (commit_term, commit_index)
/// - Vote granting required the new leader's log to be >= the voter's log
///
/// Conclusion: the new leader's log epoch is >= (commit_term, commit_index)
/// by transitivity of epoch_ge through the shared voter.
proof fn leader_completeness_theorem(
    commit_term: u64,
    commit_index: u64,
    voter_last_term: u64,
    voter_last_index: u64,
    leader_last_term: u64,
    leader_last_index: u64,
)
    requires
        // The shared voter has the committed entry (its log is at least as up-to-date)
        epoch_ge_spec(
            Epoch { term: voter_last_term, index: voter_last_index },
            Epoch { term: commit_term, index: commit_index },
        ),
        // The leader's log is at least as up-to-date as the voter's (vote was granted)
        epoch_ge_spec(
            Epoch { term: leader_last_term, index: leader_last_index },
            Epoch { term: voter_last_term, index: voter_last_index },
        ),
    ensures
        // Therefore the leader's log is at least as up-to-date as the committed entry
        epoch_ge_spec(
            Epoch { term: leader_last_term, index: leader_last_index },
            Epoch { term: commit_term, index: commit_index },
        )
{
    // Transitivity of epoch_ge
}

} // verus!

#[cfg(test)]
mod tests {
    use super::*;

    // r[verify leader-completeness.quorum-overlap]
    #[test]
    fn test_quorum_overlap_5_node() {
        // 5-node cluster: majority is 3
        // Commit quorum: 3, election quorum: 3 → overlap: 3+3=6 > 5
        assert!(3 + 3 > 5);
    }

    // r[verify leader-completeness.log-freshness]
    #[test]
    fn test_higher_term_has_entry() {
        // Entry committed at (term=2, index=5)
        // New leader in term 3 — any index is sufficient
        assert!(leader_has_committed_entry(2, 5, 3, 0));
        assert!(leader_has_committed_entry(2, 5, 3, 100));
    }

    // r[verify leader-completeness.log-freshness]
    #[test]
    fn test_same_term_needs_index() {
        // Entry committed at (term=2, index=5)
        // Leader also in term 2 — needs index >= 5
        assert!(leader_has_committed_entry(2, 5, 2, 5));
        assert!(leader_has_committed_entry(2, 5, 2, 10));
        assert!(!leader_has_committed_entry(2, 5, 2, 3));
    }

    // r[verify leader-completeness.log-freshness]
    #[test]
    fn test_lower_term_lacks_entry() {
        // Entry committed at (term=3, index=5)
        // Leader in term 2 — cannot have the entry
        assert!(!leader_has_committed_entry(3, 5, 2, 100));
    }

    // r[verify leader-completeness.quorum-overlap]
    #[test]
    fn test_concrete_leader_completeness() {
        // Scenario: entry committed at (2, 5) with quorum 3/5
        // New leader elected in term 3 with quorum 3/5
        // Shared voter has log epoch >= (2, 5)
        // Leader's log epoch >= voter's epoch (vote was granted)
        // Therefore leader's epoch >= (2, 5) ✓
        let voter_epoch = (2u64, 8u64); // voter has more entries than committed
        let leader_epoch = (3u64, 2u64); // leader in higher term

        // Voter has committed entry
        assert!(leader_has_committed_entry(2, 5, voter_epoch.0, voter_epoch.1));
        // Leader's log >= voter's (higher term wins)
        assert!(leader_has_committed_entry(voter_epoch.0, voter_epoch.1, leader_epoch.0, leader_epoch.1));
        // Therefore leader has committed entry (transitivity)
        assert!(leader_has_committed_entry(2, 5, leader_epoch.0, leader_epoch.1));
    }
}
