//! Verified core Raft consensus primitives for the domain-primitives Raft OpenSpec.
//!
//! This module intentionally stays at the exact finite model boundary: term
//! update, vote/request-vote admission, and election-safety composition over
//! existing verified quorum/ballot/ghost-history primitives. It does not claim
//! networking, storage durability, wall-clock timing, byzantine tolerance, or
//! liveness.

#![allow(clippy::wildcard_imports)]

use vstd::prelude::*;

use crate::append_entries_check::*;
use crate::ballot::*;
use crate::election_safety_proof::*;
use crate::epoch::*;
use crate::ghost_history::*;
use crate::log::*;
use crate::quorum::*;

verus! {

/// Minimal term-bearing local state used by the finite Raft model slice.
pub struct RaftTermState {
    /// The node's current term.
    pub current_term: u64,
}

/// Explicit finite Raft log-entry envelope for the domain primitive model.
#[derive(Copy, Clone, Debug)]
pub struct RaftIndexedLogEntry {
    /// One-based Raft log index.
    pub index: u64,
    /// Election term that produced the entry.
    pub term: u64,
    /// Finite command identifier; this model does not claim client/application semantics.
    pub command: u64,
}

/// Finite persistent-state envelope; durability/fsync is intentionally out of scope.
#[derive(Copy, Clone, Debug)]
pub struct RaftPersistentMetadata {
    /// Persisted current term.
    pub current_term: u64,
    /// Persisted voted-for node; zero means no vote recorded in this finite model.
    pub voted_for: u64,
}

/// Finite volatile-state envelope for commit/applied/log indexes.
#[derive(Copy, Clone, Debug)]
pub struct RaftVolatileMetadata {
    /// Highest committed one-based log index known to the node.
    pub commit_index: u64,
    /// Highest one-based log index applied to the local state-machine model.
    pub last_applied: u64,
    /// Highest one-based log index present in the local log metadata.
    pub last_log_index: u64,
}

/// Finite AppendEntries conflict metadata; zero index/term means no concrete conflicting entry.
#[derive(Copy, Clone, Debug)]
pub struct RaftAppendConflictMetadata {
    /// One-based index of the conflicting follower entry, or zero when absent.
    pub conflict_index: u64,
    /// Term at the conflict index, or zero when no concrete conflicting entry is identified.
    pub conflict_term: u64,
    /// One-based follower log length metadata, used to bound conflict indexes.
    pub follower_log_len: u64,
}

// r[depends domain-primitives.raft-consensus.persistent-state]
/// Spec: finite persisted metadata is bounded to the modeled node universe.
pub open spec fn raft_persistent_metadata_valid_spec(meta: RaftPersistentMetadata, max_node: u64) -> bool {
    meta.voted_for <= max_node
}

// r[depends domain-primitives.raft-consensus.persistent-state]
/// Spec: observing a higher term updates the persisted term and clears the recorded vote.
pub open spec fn raft_persistent_observe_term_spec(
    meta: RaftPersistentMetadata,
    observed_term: u64,
) -> RaftPersistentMetadata {
    if observed_term > meta.current_term {
        RaftPersistentMetadata { current_term: observed_term, voted_for: 0 }
    } else {
        meta
    }
}

// r[depends domain-primitives.raft-consensus.volatile-state]
/// Spec: finite volatile metadata keeps applied, committed, and log indexes ordered.
pub open spec fn raft_volatile_metadata_valid_spec(meta: RaftVolatileMetadata) -> bool {
    meta.last_applied <= meta.commit_index && meta.commit_index <= meta.last_log_index
}

// r[depends domain-primitives.raft-consensus.volatile-state]
/// Spec: a commit-index advance is admitted only within the local log and never backwards.
pub open spec fn raft_commit_advance_admit_spec(meta: RaftVolatileMetadata, new_commit: u64) -> bool {
    raft_volatile_metadata_valid_spec(meta)
        && meta.commit_index <= new_commit
        && new_commit <= meta.last_log_index
}

// r[impl domain-primitives.raft-consensus.persistent-state]
/// Return whether finite persisted Raft metadata is inside the modeled node bound.
pub fn raft_persistent_metadata_valid(meta: &RaftPersistentMetadata, max_node: u64) -> (result: bool)
    ensures result == raft_persistent_metadata_valid_spec(*meta, max_node)
{
    meta.voted_for <= max_node
}

// r[impl domain-primitives.raft-consensus.persistent-state]
/// Update finite persisted metadata for an observed term, clearing votes only on term advance.
pub fn raft_persistent_observe_term(
    meta: &RaftPersistentMetadata,
    observed_term: u64,
) -> (result: RaftPersistentMetadata)
    ensures
        result == raft_persistent_observe_term_spec(*meta, observed_term),
        result.current_term >= meta.current_term,
        observed_term > meta.current_term ==> result.voted_for == 0,
{
    if observed_term > meta.current_term {
        RaftPersistentMetadata { current_term: observed_term, voted_for: 0 }
    } else {
        *meta
    }
}

// r[impl domain-primitives.raft-consensus.volatile-state]
/// Return whether finite volatile Raft metadata has ordered applied/commit/log indexes.
pub fn raft_volatile_metadata_valid(meta: &RaftVolatileMetadata) -> (result: bool)
    ensures result == raft_volatile_metadata_valid_spec(*meta)
{
    meta.last_applied <= meta.commit_index && meta.commit_index <= meta.last_log_index
}

// r[impl domain-primitives.raft-consensus.volatile-state]
/// Return whether a finite commit-index advance is inside the local log and monotone.
pub fn raft_commit_advance_admit(meta: &RaftVolatileMetadata, new_commit: u64) -> (result: bool)
    ensures result == raft_commit_advance_admit_spec(*meta, new_commit)
{
    raft_volatile_metadata_valid(meta) && meta.commit_index <= new_commit && new_commit <= meta.last_log_index
}

// r[verify domain-primitives.raft-consensus.persistent-state]
/// Observing a higher term keeps persisted terms monotone and clears the prior vote.
pub proof fn persistent_observe_higher_term_clears_vote(meta: RaftPersistentMetadata, observed_term: u64)
    requires observed_term > meta.current_term
    ensures
        raft_persistent_observe_term_spec(meta, observed_term).current_term == observed_term,
        raft_persistent_observe_term_spec(meta, observed_term).voted_for == 0,
        raft_persistent_observe_term_spec(meta, observed_term).current_term >= meta.current_term,
{}

// r[verify domain-primitives.raft-consensus.persistent-state]
/// Observing a non-higher term leaves persisted metadata unchanged.
pub proof fn persistent_observe_non_higher_term_unchanged(meta: RaftPersistentMetadata, observed_term: u64)
    requires observed_term <= meta.current_term
    ensures raft_persistent_observe_term_spec(meta, observed_term) == meta
{}

// r[verify domain-primitives.raft-consensus.volatile-state]
/// Admitted commit advances never move commit index backwards and never exceed the local log.
pub proof fn admitted_commit_advance_is_ordered(meta: RaftVolatileMetadata, new_commit: u64)
    requires raft_commit_advance_admit_spec(meta, new_commit)
    ensures
        meta.commit_index <= new_commit,
        new_commit <= meta.last_log_index,
        meta.last_applied <= new_commit,
{}

// r[depends domain-primitives.raft-consensus.append-entries]
/// Spec: AppendEntries conflict metadata is bounded by the finite follower-log length.
pub open spec fn raft_append_conflict_metadata_valid_spec(meta: RaftAppendConflictMetadata) -> bool {
    meta.conflict_index <= meta.follower_log_len
        && (meta.conflict_index == 0 ==> meta.conflict_term == 0)
}

// r[depends domain-primitives.raft-consensus.append-entries]
/// Spec: a finite follower log update truncates after the admitted previous-log prefix and appends entries.
pub open spec fn raft_follower_log_update_spec(
    follower_log: Seq<LogEntry>,
    prev_log_index: u64,
    entries: Seq<LogEntry>,
) -> Seq<LogEntry>
    recommends prev_log_index as int <= follower_log.len()
{
    follower_log.subrange(0, prev_log_index as int) + entries
}

// r[impl domain-primitives.raft-consensus.append-entries]
/// Return whether finite AppendEntries conflict metadata is internally bounded.
pub fn raft_append_conflict_metadata_valid(meta: &RaftAppendConflictMetadata) -> (result: bool)
    ensures result == raft_append_conflict_metadata_valid_spec(*meta)
{
    meta.conflict_index <= meta.follower_log_len
        && (meta.conflict_index != 0 || meta.conflict_term == 0)
}

// r[verify domain-primitives.raft-consensus.append-entries]
/// Concrete conflict indexes are always inside the follower log metadata.
pub proof fn concrete_append_conflict_is_bounded(meta: RaftAppendConflictMetadata)
    requires
        raft_append_conflict_metadata_valid_spec(meta),
        meta.conflict_index > 0,
    ensures
        meta.conflict_index <= meta.follower_log_len,
        meta.conflict_term == meta.conflict_term,
{}

// r[verify domain-primitives.raft-consensus.append-entries]
/// An admitted follower update preserves every entry before the previous-log index.
pub proof fn follower_update_preserves_admitted_prefix(
    follower_log: Seq<LogEntry>,
    prev_log_index: u64,
    entries: Seq<LogEntry>,
    i: int,
)
    requires
        prev_log_index as int <= follower_log.len(),
        0 <= i < prev_log_index as int,
    ensures raft_follower_log_update_spec(follower_log, prev_log_index, entries)[i] == follower_log[i]
{}

// r[verify domain-primitives.raft-consensus.append-entries]
/// A follower update length is the retained prefix plus the appended entry count.
pub proof fn follower_update_length_matches_prefix_plus_entries(
    follower_log: Seq<LogEntry>,
    prev_log_index: u64,
    entries: Seq<LogEntry>,
)
    requires prev_log_index as int <= follower_log.len()
    ensures raft_follower_log_update_spec(follower_log, prev_log_index, entries).len()
        == prev_log_index as int + entries.len()
{}

// r[depends domain-primitives.raft-consensus.log-matching]
/// Spec: Raft log matching at a one-based index requires equal terms there and matching prior terms.
pub open spec fn raft_log_matching_at_spec(a: Seq<LogEntry>, b: Seq<LogEntry>, index: u64) -> bool {
    index > 0
        && index as int <= a.len()
        && index as int <= b.len()
        && a[(index - 1) as int].term == b[(index - 1) as int].term
        && logs_match_prefix_spec(a, b, index as int)
}

// r[impl domain-primitives.raft-consensus.log-matching]
/// Return whether two finite Raft logs match through the given one-based index.
pub fn raft_log_matching_at(a: &[LogEntry], b: &[LogEntry], index: usize) -> (result: bool)
    ensures result == raft_log_matching_at_spec(a@, b@, index as u64)
{
    if index == 0 || index > a.len() || index > b.len() {
        return false;
    }
    if a[index - 1].term != b[index - 1].term {
        return false;
    }
    logs_match_prefix(a, b, index)
}

// r[verify domain-primitives.raft-consensus.log-matching]
/// Log matching at an index implies all prior terms match as a prefix.
pub proof fn log_matching_implies_prefix_match(a: Seq<LogEntry>, b: Seq<LogEntry>, index: u64)
    requires raft_log_matching_at_spec(a, b, index)
    ensures logs_match_prefix_spec(a, b, index as int)
{}

// r[verify domain-primitives.raft-consensus.log-matching]
/// Follower updates preserve the admitted matching prefix before appended entries.
pub proof fn follower_update_preserves_matching_prefix(
    leader_log: Seq<LogEntry>,
    follower_log: Seq<LogEntry>,
    prev_log_index: u64,
    entries: Seq<LogEntry>,
)
    requires
        prev_log_index as int <= follower_log.len(),
        logs_match_prefix_spec(leader_log, follower_log, prev_log_index as int),
    ensures logs_match_prefix_spec(
        leader_log,
        raft_follower_log_update_spec(follower_log, prev_log_index, entries),
        prev_log_index as int,
    )
{
    assert forall|i: int| 0 <= i < prev_log_index as int implies
        leader_log[i].term == raft_follower_log_update_spec(follower_log, prev_log_index, entries)[i].term by {
            assert(leader_log[i].term == follower_log[i].term);
            assert(raft_follower_log_update_spec(follower_log, prev_log_index, entries)[i] == follower_log[i]);
        }
}

// r[depends domain-primitives.raft-consensus.commit]
/// Spec: quorum commit metadata admits an index only when enough replicas match it.
pub open spec fn raft_quorum_commit_admit_spec(
    match_count: u64,
    cluster_size: u64,
    commit_index: u64,
    candidate_index: u64,
    last_log_index: u64,
) -> bool
    recommends cluster_size > 0
{
    is_quorum_spec(match_count, cluster_size)
        && commit_index <= candidate_index
        && candidate_index <= last_log_index
}

// r[depends domain-primitives.raft-consensus.commit]
/// Spec: admitted quorum commit advances to the candidate index and otherwise stays unchanged.
pub open spec fn raft_commit_index_update_spec(
    match_count: u64,
    cluster_size: u64,
    commit_index: u64,
    candidate_index: u64,
    last_log_index: u64,
) -> u64
    recommends cluster_size > 0
{
    if raft_quorum_commit_admit_spec(match_count, cluster_size, commit_index, candidate_index, last_log_index) {
        candidate_index
    } else {
        commit_index
    }
}

// r[impl domain-primitives.raft-consensus.commit]
/// Return whether a finite candidate commit index has quorum support and local-log bounds.
pub fn raft_quorum_commit_admit(
    match_count: u64,
    cluster_size: u64,
    commit_index: u64,
    candidate_index: u64,
    last_log_index: u64,
) -> (result: bool)
    requires cluster_size > 0
    ensures result == raft_quorum_commit_admit_spec(
        match_count,
        cluster_size,
        commit_index,
        candidate_index,
        last_log_index,
    )
{
    is_quorum(match_count, cluster_size) && commit_index <= candidate_index && candidate_index <= last_log_index
}

// r[impl domain-primitives.raft-consensus.commit]
/// Return the finite commit index after applying quorum-commit admission.
pub fn raft_commit_index_update(
    match_count: u64,
    cluster_size: u64,
    commit_index: u64,
    candidate_index: u64,
    last_log_index: u64,
) -> (result: u64)
    requires cluster_size > 0
    ensures
        result == raft_commit_index_update_spec(
            match_count,
            cluster_size,
            commit_index,
            candidate_index,
            last_log_index,
        ),
        result >= commit_index,
        result <= last_log_index || commit_index > last_log_index,
{
    if raft_quorum_commit_admit(match_count, cluster_size, commit_index, candidate_index, last_log_index) {
        candidate_index
    } else {
        commit_index
    }
}

// r[verify domain-primitives.raft-consensus.commit]
/// Admitted quorum commits are monotone and bounded by the local log.
pub proof fn admitted_quorum_commit_is_ordered(
    match_count: u64,
    cluster_size: u64,
    commit_index: u64,
    candidate_index: u64,
    last_log_index: u64,
)
    requires
        cluster_size > 0,
        raft_quorum_commit_admit_spec(match_count, cluster_size, commit_index, candidate_index, last_log_index),
    ensures
        commit_index <= candidate_index,
        candidate_index <= last_log_index,
        is_quorum_spec(match_count, cluster_size),
{}

// r[verify domain-primitives.raft-consensus.commit]
/// Rejected quorum commits leave the commit index unchanged.
pub proof fn rejected_quorum_commit_stays_put(
    match_count: u64,
    cluster_size: u64,
    commit_index: u64,
    candidate_index: u64,
    last_log_index: u64,
)
    requires
        cluster_size > 0,
        !raft_quorum_commit_admit_spec(match_count, cluster_size, commit_index, candidate_index, last_log_index),
    ensures raft_commit_index_update_spec(match_count, cluster_size, commit_index, candidate_index, last_log_index) == commit_index
{}

// r[depends domain-primitives.raft-consensus.leader-completeness]
/// Spec: a later leader preserves the committed entry at the same finite log position.
pub open spec fn raft_leader_contains_committed_entry_spec(
    committed_log: Seq<RaftIndexedLogEntry>,
    leader_log: Seq<RaftIndexedLogEntry>,
    committed_pos: int,
) -> bool {
    0 <= committed_pos
        && committed_pos < committed_log.len()
        && committed_pos < leader_log.len()
        && leader_log[committed_pos].index == committed_log[committed_pos].index
        && leader_log[committed_pos].term == committed_log[committed_pos].term
        && leader_log[committed_pos].command == committed_log[committed_pos].command
}

// r[impl domain-primitives.raft-consensus.leader-completeness]
/// Return whether a later leader log contains the finite committed entry at `committed_pos`.
pub fn raft_leader_contains_committed_entry(
    committed_log: &[RaftIndexedLogEntry],
    leader_log: &[RaftIndexedLogEntry],
    committed_pos: usize,
) -> (result: bool)
    ensures result == raft_leader_contains_committed_entry_spec(
        committed_log@,
        leader_log@,
        committed_pos as int,
    )
{
    committed_pos < committed_log.len()
        && committed_pos < leader_log.len()
        && leader_log[committed_pos].index == committed_log[committed_pos].index
        && leader_log[committed_pos].term == committed_log[committed_pos].term
        && leader_log[committed_pos].command == committed_log[committed_pos].command
}

// r[verify domain-primitives.raft-consensus.leader-completeness]
/// Leader-completeness admission exposes same index, term, and command for the committed position.
pub proof fn admitted_leader_completeness_preserves_entry(
    committed_log: Seq<RaftIndexedLogEntry>,
    leader_log: Seq<RaftIndexedLogEntry>,
    committed_pos: int,
)
    requires raft_leader_contains_committed_entry_spec(committed_log, leader_log, committed_pos)
    ensures
        leader_log[committed_pos].index == committed_log[committed_pos].index,
        leader_log[committed_pos].term == committed_log[committed_pos].term,
        leader_log[committed_pos].command == committed_log[committed_pos].command,
{}

// r[depends domain-primitives.raft-consensus.state-machine-safety]
/// Spec: applied prefixes agree on the command at a shared applied position.
pub open spec fn raft_state_machine_safety_at_spec(
    a: Seq<RaftIndexedLogEntry>,
    b: Seq<RaftIndexedLogEntry>,
    applied_a: int,
    applied_b: int,
    pos: int,
) -> bool {
    0 <= pos
        && pos < applied_a
        && pos < applied_b
        && pos < a.len()
        && pos < b.len()
        && a[pos].command == b[pos].command
}

// r[impl domain-primitives.raft-consensus.state-machine-safety]
/// Return whether two finite applied prefixes agree on a command at `pos`.
pub fn raft_state_machine_safety_at(
    a: &[RaftIndexedLogEntry],
    b: &[RaftIndexedLogEntry],
    applied_a: usize,
    applied_b: usize,
    pos: usize,
) -> (result: bool)
    ensures result == raft_state_machine_safety_at_spec(
        a@,
        b@,
        applied_a as int,
        applied_b as int,
        pos as int,
    )
{
    pos < applied_a && pos < applied_b && pos < a.len() && pos < b.len() && a[pos].command == b[pos].command
}

// r[verify domain-primitives.raft-consensus.state-machine-safety]
/// Admitted state-machine safety gives equal command identifiers at the shared applied position.
pub proof fn admitted_state_machine_safety_commands_equal(
    a: Seq<RaftIndexedLogEntry>,
    b: Seq<RaftIndexedLogEntry>,
    applied_a: int,
    applied_b: int,
    pos: int,
)
    requires raft_state_machine_safety_at_spec(a, b, applied_a, applied_b, pos)
    ensures a[pos].command == b[pos].command
{}

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

// r[depends domain-primitives.raft-consensus.log-entry]
/// Spec: an indexed Raft log entry is inside the declared finite bounds.
pub open spec fn raft_log_entry_in_bounds_spec(
    entry: RaftIndexedLogEntry,
    max_index: u64,
    max_term: u64,
) -> bool {
    1 <= entry.index && entry.index <= max_index && entry.term <= max_term
}

// r[depends domain-primitives.raft-consensus.log-prefix]
/// Spec: an indexed Raft log has no gaps exactly when entry indexes are 1-based and contiguous.
pub open spec fn raft_log_no_gap_spec(log: Seq<RaftIndexedLogEntry>) -> bool {
    log.len() <= u64::MAX as int
        && forall|i: int| #![trigger log[i]]
            0 <= i < log.len() ==> log[i].index == (i + 1) as u64
}

// r[depends domain-primitives.raft-consensus.log-prefix]
/// Spec: `prefix` is an exact entry prefix of `log`.
pub open spec fn raft_log_prefix_spec(
    prefix: Seq<RaftIndexedLogEntry>,
    log: Seq<RaftIndexedLogEntry>,
) -> bool {
    prefix.len() <= log.len()
        && forall|i: int| #![trigger log[i]]
            0 <= i < prefix.len() ==> log[i] == prefix[i]
}

// r[depends domain-primitives.raft-consensus.log-prefix]
/// Spec: appending a single entry to the finite indexed log.
pub open spec fn raft_log_append_spec(
    log: Seq<RaftIndexedLogEntry>,
    entry: RaftIndexedLogEntry,
) -> Seq<RaftIndexedLogEntry> {
    log.push(entry)
}

// r[impl domain-primitives.raft-consensus.log-entry]
/// Return whether an explicit Raft log-entry envelope is inside finite bounds.
pub fn raft_log_entry_in_bounds(
    entry: &RaftIndexedLogEntry,
    max_index: u64,
    max_term: u64,
) -> (result: bool)
    ensures result == raft_log_entry_in_bounds_spec(*entry, max_index, max_term)
{
    1 <= entry.index && entry.index <= max_index && entry.term <= max_term
}

// r[impl domain-primitives.raft-consensus.log-prefix]
/// Return whether indexed entries form the contiguous one-based Raft log range `1..=len`.
pub fn raft_log_no_gap(log: &[RaftIndexedLogEntry]) -> (result: bool)
    requires log@.len() <= u64::MAX as int
    ensures result == raft_log_no_gap_spec(log@)
{
    let mut i: usize = 0;
    while i < log.len()
        invariant
            0 <= i <= log@.len(),
            log@.len() <= u64::MAX as int,
            forall|j: int| #![trigger log@[j]]
                0 <= j < i as int ==> log@[j].index == (j + 1) as u64,
        decreases log@.len() - i
    {
        if log[i].index != i as u64 + 1 {
            assert(0 <= i as int);
            assert((i as int) < log@.len());
            assert(log@[i as int].index != (i as int + 1) as u64);
            assert(!raft_log_no_gap_spec(log@));
            return false;
        }
        i = i + 1;
    }
    true
}

// r[impl domain-primitives.raft-consensus.log-prefix]
/// Return whether `prefix` is an exact entry prefix of `log`.
pub fn raft_log_prefix(
    prefix: &[RaftIndexedLogEntry],
    log: &[RaftIndexedLogEntry],
) -> (result: bool)
    ensures result == raft_log_prefix_spec(prefix@, log@)
{
    if prefix.len() > log.len() {
        return false;
    }
    let mut i: usize = 0;
    while i < prefix.len()
        invariant
            0 <= i <= prefix@.len(),
            prefix@.len() <= log@.len(),
            forall|j: int| #![trigger log@[j]]
                0 <= j < i as int ==> log@[j] == prefix@[j],
        decreases prefix@.len() - i
    {
        if log[i].index != prefix[i].index
            || log[i].term != prefix[i].term
            || log[i].command != prefix[i].command
        {
            assert(0 <= i as int);
            assert((i as int) < prefix@.len());
            assert(log@[i as int] != prefix@[i as int]);
            assert(!raft_log_prefix_spec(prefix@, log@));
            return false;
        }
        i = i + 1;
    }
    true
}

// r[impl domain-primitives.raft-consensus.log-prefix]
/// Append a contiguous entry to an explicit indexed Raft log.
pub fn raft_log_append(
    log: &mut Vec<RaftIndexedLogEntry>,
    entry: RaftIndexedLogEntry,
)
    requires
        old(log)@.len() < u64::MAX as int,
        entry.index == old(log)@.len() as u64 + 1,
    ensures
        log@ == raft_log_append_spec(old(log)@, entry),
        log@.len() == old(log)@.len() + 1,
        log@[old(log)@.len() as int] == entry,
        forall|i: int| 0 <= i < old(log)@.len() ==> log@[i] == old(log)@[i],
        raft_log_no_gap_spec(old(log)@) ==> raft_log_no_gap_spec(log@),
{
    log.push(entry);
}

// r[verify domain-primitives.raft-consensus.log-entry]
/// A valid entry always has a nonzero one-based index.
pub proof fn admitted_log_entry_has_nonzero_index(
    entry: RaftIndexedLogEntry,
    max_index: u64,
    max_term: u64,
)
    requires raft_log_entry_in_bounds_spec(entry, max_index, max_term)
    ensures entry.index > 0
{}

// r[verify domain-primitives.raft-consensus.log-prefix]
/// Empty indexed logs are gap-free.
pub proof fn empty_raft_log_has_no_gaps()
    ensures raft_log_no_gap_spec(Seq::<RaftIndexedLogEntry>::empty())
{}

// r[verify domain-primitives.raft-consensus.log-prefix]
/// Contiguous append preserves the no-gap predicate.
pub proof fn contiguous_append_preserves_no_gap(log: Seq<RaftIndexedLogEntry>, entry: RaftIndexedLogEntry)
    requires
        log.len() < u64::MAX as int,
        raft_log_no_gap_spec(log),
        entry.index == log.len() as u64 + 1,
    ensures raft_log_no_gap_spec(raft_log_append_spec(log, entry))
{}

// r[verify domain-primitives.raft-consensus.log-prefix]
/// A prefix of a no-gap log is also no-gap.
pub proof fn prefix_of_no_gap_is_no_gap(
    prefix: Seq<RaftIndexedLogEntry>,
    log: Seq<RaftIndexedLogEntry>,
)
    requires
        raft_log_no_gap_spec(log),
        raft_log_prefix_spec(prefix, log),
    ensures raft_log_no_gap_spec(prefix)
{
    assert(prefix.len() <= log.len());
    assert(log.len() <= u64::MAX as int);
    assert(prefix.len() <= u64::MAX as int);
    assert forall|i: int| #![trigger prefix[i]]
        0 <= i < prefix.len() implies prefix[i].index == (i + 1) as u64 by {
            assert(log[i] == prefix[i]);
            assert(log[i].index == (i + 1) as u64);
        }
}

// r[depends domain-primitives.raft-consensus.append-entries]
/// Spec: this finite AppendEntries admission slice combines non-stale term and prev-log check.
pub open spec fn raft_append_entries_admit_spec(
    current_term: u64,
    leader_term: u64,
    follower_log: Seq<LogEntry>,
    prev_log_index: u64,
    prev_log_term: u64,
) -> bool {
    !stale_term_message_spec(current_term, leader_term)
        && check_append_entries_spec(follower_log, prev_log_index, prev_log_term)
}

// r[impl domain-primitives.raft-consensus.append-entries]
/// Return whether this finite AppendEntries request is admitted by term and prev-log checks.
pub fn raft_append_entries_admit(
    current_term: u64,
    leader_term: u64,
    follower_log: &[LogEntry],
    prev_log_index: u64,
    prev_log_term: u64,
) -> (result: bool)
    requires prev_log_index as int <= usize::MAX as int
    ensures result == raft_append_entries_admit_spec(
        current_term,
        leader_term,
        follower_log@,
        prev_log_index,
        prev_log_term,
    )
{
    if stale_term_message(current_term, leader_term) {
        false
    } else {
        check_append_entries(follower_log, prev_log_index, prev_log_term)
    }
}

// r[verify domain-primitives.raft-consensus.append-entries]
/// Stale AppendEntries requests are rejected before any prev-log admission can accept them.
pub proof fn stale_append_entries_rejected(
    current_term: u64,
    leader_term: u64,
    follower_log: Seq<LogEntry>,
    prev_log_index: u64,
    prev_log_term: u64,
)
    requires leader_term < current_term
    ensures !raft_append_entries_admit_spec(
        current_term,
        leader_term,
        follower_log,
        prev_log_index,
        prev_log_term,
    )
{}

// r[verify domain-primitives.raft-consensus.append-entries]
/// Previous-log mismatch rejects AppendEntries in the non-stale term case.
pub proof fn append_entries_prev_log_mismatch_rejected(
    current_term: u64,
    leader_term: u64,
    follower_log: Seq<LogEntry>,
    prev_log_index: u64,
    prev_log_term: u64,
)
    requires
        leader_term >= current_term,
        !check_append_entries_spec(follower_log, prev_log_index, prev_log_term),
    ensures !raft_append_entries_admit_spec(
        current_term,
        leader_term,
        follower_log,
        prev_log_index,
        prev_log_term,
    )
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
    use crate::log::LogEntry;

    fn epoch(term: u64, index: u64) -> Epoch {
        Epoch { term, index }
    }

    fn indexed(index: u64, term: u64, command: u64) -> RaftIndexedLogEntry {
        RaftIndexedLogEntry { index, term, command }
    }

    fn log_entry(term: u64, value: u64) -> LogEntry {
        LogEntry { term, value }
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

    // r[verify domain-primitives.raft-consensus.persistent-state]
    #[test]
    fn persistent_metadata_observed_term_clears_vote_only_on_advance() {
        let meta = RaftPersistentMetadata {
            current_term: 3,
            voted_for: 9,
        };
        assert!(raft_persistent_metadata_valid(&meta, 10));
        assert!(!raft_persistent_metadata_valid(&meta, 8));

        let unchanged = raft_persistent_observe_term(&meta, 3);
        assert_eq!(unchanged.current_term, 3);
        assert_eq!(unchanged.voted_for, 9);

        let advanced = raft_persistent_observe_term(&meta, 4);
        assert_eq!(advanced.current_term, 4);
        assert_eq!(advanced.voted_for, 0);
    }

    // r[verify domain-primitives.raft-consensus.volatile-state]
    #[test]
    fn volatile_metadata_orders_commit_and_log_indexes() {
        let meta = RaftVolatileMetadata {
            last_applied: 2,
            commit_index: 4,
            last_log_index: 6,
        };
        assert!(raft_volatile_metadata_valid(&meta));
        assert!(raft_commit_advance_admit(&meta, 4));
        assert!(raft_commit_advance_admit(&meta, 6));
        assert!(!raft_commit_advance_admit(&meta, 3));
        assert!(!raft_commit_advance_admit(&meta, 7));

        let invalid = RaftVolatileMetadata {
            last_applied: 5,
            commit_index: 4,
            last_log_index: 6,
        };
        assert!(!raft_volatile_metadata_valid(&invalid));
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

    // r[verify domain-primitives.raft-consensus.log-entry]
    #[test]
    fn log_entry_bounds_reject_zero_and_out_of_range() {
        assert!(raft_log_entry_in_bounds(&indexed(1, 2, 99), 10, 5));
        assert!(!raft_log_entry_in_bounds(&indexed(0, 2, 99), 10, 5));
        assert!(!raft_log_entry_in_bounds(&indexed(11, 2, 99), 10, 5));
        assert!(!raft_log_entry_in_bounds(&indexed(1, 6, 99), 10, 5));
    }

    // r[verify domain-primitives.raft-consensus.log-prefix]
    #[test]
    fn no_gap_rejects_skipped_index() {
        let contiguous = vec![indexed(1, 1, 10), indexed(2, 1, 20), indexed(3, 2, 30)];
        let gap = vec![indexed(1, 1, 10), indexed(3, 2, 30)];
        assert!(raft_log_no_gap(&contiguous));
        assert!(!raft_log_no_gap(&gap));
    }

    // r[verify domain-primitives.raft-consensus.log-prefix]
    #[test]
    fn prefix_and_append_preserve_contiguous_log() {
        let mut log = vec![indexed(1, 1, 10), indexed(2, 1, 20)];
        let prefix = vec![indexed(1, 1, 10)];
        assert!(raft_log_prefix(&prefix, &log));
        raft_log_append(&mut log, indexed(3, 2, 30));
        assert!(raft_log_no_gap(&log));
        assert!(raft_log_prefix(&prefix, &log));
    }

    // r[verify domain-primitives.raft-consensus.append-entries]
    #[test]
    fn append_entries_admission_checks_term_and_prev_log() {
        let follower_log = vec![log_entry(1, 10), log_entry(2, 20)];
        assert!(raft_append_entries_admit(2, 2, &follower_log, 2, 2));
        assert!(!raft_append_entries_admit(3, 2, &follower_log, 2, 2));
        assert!(!raft_append_entries_admit(2, 2, &follower_log, 2, 9));
    }

    // r[verify domain-primitives.raft-consensus.append-entries]
    #[test]
    fn append_entries_conflict_metadata_is_bounded() {
        let bounded = RaftAppendConflictMetadata {
            conflict_index: 2,
            conflict_term: 5,
            follower_log_len: 3,
        };
        assert!(raft_append_conflict_metadata_valid(&bounded));

        let absent = RaftAppendConflictMetadata {
            conflict_index: 0,
            conflict_term: 0,
            follower_log_len: 3,
        };
        assert!(raft_append_conflict_metadata_valid(&absent));

        let bad_absent = RaftAppendConflictMetadata {
            conflict_index: 0,
            conflict_term: 5,
            follower_log_len: 3,
        };
        assert!(!raft_append_conflict_metadata_valid(&bad_absent));

        let out_of_bounds = RaftAppendConflictMetadata {
            conflict_index: 4,
            conflict_term: 5,
            follower_log_len: 3,
        };
        assert!(!raft_append_conflict_metadata_valid(&out_of_bounds));
    }

    // r[verify domain-primitives.raft-consensus.log-matching]
    #[test]
    fn log_matching_requires_equal_prefix_terms() {
        let leader_log = vec![log_entry(1, 10), log_entry(2, 20), log_entry(3, 30)];
        let matching = vec![log_entry(1, 90), log_entry(2, 80), log_entry(4, 70)];
        let divergent = vec![log_entry(1, 90), log_entry(5, 80), log_entry(3, 70)];

        assert!(raft_log_matching_at(&leader_log, &matching, 2));
        assert!(!raft_log_matching_at(&leader_log, &matching, 3));
        assert!(!raft_log_matching_at(&leader_log, &divergent, 3));
        assert!(!raft_log_matching_at(&leader_log, &matching, 0));
    }

    // r[verify domain-primitives.raft-consensus.commit]
    #[test]
    fn quorum_commit_admission_is_monotone_and_bounded() {
        assert!(raft_quorum_commit_admit(3, 5, 2, 4, 6));
        assert_eq!(raft_commit_index_update(3, 5, 2, 4, 6), 4);

        assert!(!raft_quorum_commit_admit(2, 5, 2, 4, 6));
        assert_eq!(raft_commit_index_update(2, 5, 2, 4, 6), 2);

        assert!(!raft_quorum_commit_admit(3, 5, 4, 3, 6));
        assert_eq!(raft_commit_index_update(3, 5, 4, 3, 6), 4);

        assert!(!raft_quorum_commit_admit(3, 5, 2, 7, 6));
        assert_eq!(raft_commit_index_update(3, 5, 2, 7, 6), 2);
    }

    // r[verify domain-primitives.raft-consensus.leader-completeness]
    #[test]
    fn leader_completeness_requires_committed_entry_presence() {
        let committed = vec![indexed(1, 1, 10), indexed(2, 2, 20), indexed(3, 2, 30)];
        let complete_leader = vec![indexed(1, 1, 10), indexed(2, 2, 20), indexed(3, 3, 99)];
        let missing = vec![indexed(1, 1, 10), indexed(2, 3, 21), indexed(3, 3, 99)];

        assert!(raft_leader_contains_committed_entry(&committed, &complete_leader, 1));
        assert!(!raft_leader_contains_committed_entry(&committed, &missing, 1));
        assert!(!raft_leader_contains_committed_entry(&committed, &complete_leader, 4));
    }

    // r[verify domain-primitives.raft-consensus.state-machine-safety]
    #[test]
    fn state_machine_safety_rejects_divergent_applied_commands() {
        let a = vec![indexed(1, 1, 10), indexed(2, 2, 20)];
        let same = vec![indexed(1, 1, 10), indexed(2, 3, 20)];
        let divergent = vec![indexed(1, 1, 10), indexed(2, 3, 21)];

        assert!(raft_state_machine_safety_at(&a, &same, 2, 2, 1));
        assert!(!raft_state_machine_safety_at(&a, &divergent, 2, 2, 1));
        assert!(!raft_state_machine_safety_at(&a, &same, 1, 2, 1));
    }
}
