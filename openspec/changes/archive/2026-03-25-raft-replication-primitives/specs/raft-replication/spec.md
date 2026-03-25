# Raft Replication Primitives Specification

## Purpose

Defines the correctness requirements for Raft log replication machinery,
protocol-level rules, composed safety properties, and Raft extensions.
These build on the existing consensus primitives (quorum, epoch, log,
ballot, election, monotonic) to cover everything between leader election
and entry commitment.

## Requirements

### AppendEntries Consistency Check

#### Requirement: PrevLog Validation

`check_append_entries(log, prev_log_index, prev_log_term)` MUST return
true if and only if one of:
1. `prev_log_index == 0` (empty prefix — first AppendEntries), OR
2. The log has an entry at `prev_log_index - 1` whose term equals
   `prev_log_term`.

MUST return false when `prev_log_index > 0` and the log has no entry
at that position (log too short) or the entry's term does not match.

##### Scenario: Empty prefix

- GIVEN an empty log and prev_log_index=0
- WHEN checking AppendEntries consistency
- THEN MUST return true

##### Scenario: Matching entry

- GIVEN a log [(term=1), (term=2), (term=3)] and prev_log_index=2, prev_log_term=2
- WHEN checking
- THEN MUST return true (log[1].term == 2)

##### Scenario: Term mismatch

- GIVEN a log [(term=1), (term=2), (term=3)] and prev_log_index=2, prev_log_term=5
- WHEN checking
- THEN MUST return false (log[1].term == 2 ≠ 5)

##### Scenario: Log too short

- GIVEN a log of length 2 and prev_log_index=5, prev_log_term=1
- WHEN checking
- THEN MUST return false (no entry at index 4)

#### Requirement: Consistency Implies Prefix Agreement

If `check_append_entries` returns true with prev_log_index=k and two
logs both pass the check for the same (k, term), then the two logs
MUST agree at all indices in [0, k). This connects the runtime check
to the Log Matching Property.

##### Scenario: Two logs pass same check

- GIVEN log_a and log_b both pass check with prev_log_index=3, prev_log_term=2
- WHEN comparing prefixes
- THEN logs_match_prefix(log_a, log_b, 3) holds (assuming inductive log matching)

#### Requirement: Rejection Means Divergence

If `check_append_entries` returns false for prev_log_index > 0, then
either the log is too short or the entry at that position has a
different term. In either case, the follower's log diverges from the
leader's at or before that index.

##### Scenario: Short log rejection

- GIVEN log length 2, prev_log_index=5
- WHEN check returns false
- THEN the log cannot match the leader's prefix through index 5

##### Scenario: Term mismatch rejection

- GIVEN log[prev_log_index-1].term ≠ prev_log_term
- WHEN check returns false
- THEN the follower has a conflicting entry from a different term

### Log Conflict Resolution

#### Requirement: Find Conflict Index

`find_conflict(follower_log, leader_entries, start_index)` MUST return
the first index at or after `start_index` where the follower's log and
leader's entries disagree on term. MUST return None if all overlapping
entries match (no conflict).

##### Scenario: Conflict at start

- GIVEN follower log [(t=1),(t=2),(t=3)] and leader entries [(t=1),(t=4)] starting at index 0
- WHEN finding conflict
- THEN MUST return Some(1) (terms 2 vs 4 at index 1)

##### Scenario: No conflict

- GIVEN follower log [(t=1),(t=2)] and leader entries [(t=1),(t=2)] starting at index 0
- WHEN finding conflict
- THEN MUST return None

##### Scenario: Leader extends beyond follower

- GIVEN follower log [(t=1)] and leader entries [(t=1),(t=2),(t=3)] starting at index 0
- WHEN finding conflict
- THEN MUST return None (no conflict in overlapping portion)

#### Requirement: Resolve Conflict

`resolve_conflict(follower_log, leader_entries, start_index)` MUST:
1. Find the conflict point (if any)
2. Truncate the follower log at the conflict point
3. Append all leader entries from the conflict point onward

If no conflict exists and the leader has entries beyond the follower's
log, those entries MUST be appended. The resulting log MUST match the
leader's entries at all indices in [start_index, start_index + leader_entries.len()).

##### Scenario: Truncate and append

- GIVEN follower [(t=1),(t=2),(t=3)] and leader entries [(t=1),(t=4),(t=4)] at start_index=0
- WHEN resolving conflict
- THEN result MUST be [(t=1),(t=4),(t=4)]

##### Scenario: Append only

- GIVEN follower [(t=1),(t=2)] and leader entries [(t=1),(t=2),(t=3)] at start_index=0
- WHEN resolving conflict
- THEN result MUST be [(t=1),(t=2),(t=3)]

##### Scenario: No change needed

- GIVEN follower [(t=1),(t=2)] and leader entries [(t=1),(t=2)] at start_index=0
- WHEN resolving conflict
- THEN result MUST equal follower (unchanged)

#### Requirement: Prefix Preservation

After conflict resolution, the matching prefix between the follower's
original log and the result MUST be preserved. Entries before the
conflict point MUST be identical to the original.

##### Scenario: Prefix untouched

- GIVEN conflict at index 2 in a 5-entry log
- WHEN resolving
- THEN entries at indices 0 and 1 MUST be unchanged

#### Requirement: Result Matches Leader

After conflict resolution, the result log MUST agree with the leader's
entries at all indices covered by leader_entries. `agree_at(result,
leader_log, i)` MUST hold for all i in [start_index, start_index +
leader_entries.len()).

### Match Index Tracking

#### Requirement: Match Index Monotonicity

`update_match_index(matches, server, new_match)` MUST set
matches[server] to new_match. new_match MUST be >= matches[server]
(monotonic — match index never decreases). The update MUST NOT modify
any other server's match index.

##### Scenario: Advance

- GIVEN matches=[0, 0, 0, 0, 0] and server=2, new_match=5
- WHEN updating
- THEN matches becomes [0, 0, 5, 0, 0]

##### Scenario: Monotonic violation rejected

- GIVEN matches=[0, 0, 5, 0, 0] and server=2, new_match=3
- WHEN attempting update
- THEN MUST be rejected (3 < 5 violates monotonicity)

#### Requirement: Next Index Backtrack

`backtrack_next_index(nexts, server)` MUST decrease nexts[server] by 1,
with a floor of 1 (next index never goes below 1). The update MUST NOT
modify any other server's next index.

##### Scenario: Normal backtrack

- GIVEN nexts=[5, 5, 5, 5, 5] and server=2
- WHEN backtracking
- THEN nexts becomes [5, 5, 4, 5, 5]

##### Scenario: Floor at 1

- GIVEN nexts=[5, 5, 1, 5, 5] and server=2
- WHEN backtracking
- THEN nexts stays [5, 5, 1, 5, 5] (cannot go below 1)

#### Requirement: Initialize Indices

`init_match_indices(cluster_size)` MUST return a sequence of
`cluster_size` zeros. `init_next_indices(cluster_size, last_log_index)`
MUST return a sequence where every element is `last_log_index + 1`
(leader initializes nextIndex to its log end).

##### Scenario: Fresh leader

- GIVEN cluster_size=5, last_log_index=10
- WHEN initializing
- THEN matchIndex=[0,0,0,0,0] and nextIndex=[11,11,11,11,11]

#### Requirement: Match Index Bounded by Log Length

A server's matchIndex MUST never exceed the leader's log length.
After any update, `matches[server] <= leader_log_length` MUST hold.

### Commit Index Advancement

#### Requirement: Current Term Guard

`can_commit_at(log, index, current_term)` MUST return false when
`log[index].term != current_term`. A leader MUST NOT advance its commit
index to an entry from a prior term by counting replicas alone. This
is the §5.4.2 "Figure 8" safety rule.

##### Scenario: Current term entry

- GIVEN log[(t=1),(t=2),(t=3)] and current_term=3
- WHEN checking can_commit_at index=2
- THEN MUST return true (log[2].term == 3 == current_term)

##### Scenario: Prior term entry

- GIVEN log[(t=1),(t=2),(t=3)] and current_term=4
- WHEN checking can_commit_at index=1
- THEN MUST return false (log[1].term == 2 ≠ 4)

##### Scenario: Prior term committed indirectly

- GIVEN log[(t=1),(t=2),(t=3)] and current_term=3
- WHEN committing index=2 (term 3 entry)
- THEN indices 0 and 1 (terms 1 and 2) are also committed (prefix)

#### Requirement: Quorum Replication Check

`has_quorum_at(match_indices, index, cluster_size)` MUST return true
if and only if the count of servers where `match_indices[i] >= index`
constitutes a quorum. The leader counts itself (its matchIndex is
always equal to its log length).

##### Scenario: Majority replicated

- GIVEN match_indices=[5, 3, 5, 2, 5] and index=5, cluster_size=5
- WHEN checking quorum
- THEN MUST return true (3 out of 5 have matchIndex >= 5)

##### Scenario: Not enough replicas

- GIVEN match_indices=[5, 1, 1, 1, 1] and index=5, cluster_size=5
- WHEN checking quorum
- THEN MUST return false (only 1 out of 5)

#### Requirement: Advance Commit Index

`advance_commit_index(log, match_indices, current_commit, current_term,
cluster_size)` MUST return the largest index N where:
1. N > current_commit
2. log[N-1].term == current_term (current term guard)
3. has_quorum_at(match_indices, N, cluster_size) is true

If no such N exists, MUST return current_commit (no advancement).

##### Scenario: Advance

- GIVEN log=[(t=1),(t=2),(t=2),(t=2)], match_indices=[4,2,3,1,4],
  current_commit=1, current_term=2, cluster_size=5
- WHEN advancing
- THEN commit index MUST advance to 3 (index 3 has term 2, 3 servers have matchIndex >= 3)

##### Scenario: No advance (wrong term)

- GIVEN log=[(t=1),(t=1),(t=1)], match_indices=[3,3,3,3,3],
  current_commit=0, current_term=2
- WHEN advancing
- THEN commit index stays 0 (all entries are term 1, not current term 2)

#### Requirement: Commit Monotonicity

The commit index MUST never decrease. `advance_commit_index` MUST
always return a value >= current_commit.

#### Requirement: Committed Prefix

If commit index is N, all entries at indices [0, N) are committed.
Advancing the commit index from N to M (M > N) commits entries at
[N, M). Previously committed entries MUST remain committed.

### Term Authority

#### Requirement: Step Down on Higher Term

`apply_higher_term(current_term, current_state, message_term)` MUST:
1. When message_term > current_term: return (message_term, Follower, None)
   — adopt the new term, become follower, clear votedFor.
2. When message_term <= current_term: return unchanged state.

##### Scenario: Higher term received

- GIVEN current_term=3, state=Leader
- WHEN receiving message with term=5
- THEN new_term=5, new_state=Follower, votedFor=None

##### Scenario: Same term

- GIVEN current_term=3, state=Leader
- WHEN receiving message with term=3
- THEN state unchanged

##### Scenario: Lower term

- GIVEN current_term=5, state=Follower
- WHEN receiving message with term=3
- THEN state unchanged (stale message)

#### Requirement: Term Monotonicity

After `apply_higher_term`, the resulting term MUST be >= current_term.
Terms MUST never decrease. This is `monotonic::is_advance` applied to
terms.

#### Requirement: Leader Demotion

If a leader receives a higher term, it MUST become a follower. There
MUST NOT exist a state where current_term < message_term and the server
remains leader.

##### Scenario: Leader demoted

- GIVEN state=Leader, current_term=3, message_term=4
- WHEN applying
- THEN state MUST be Follower (not Leader, not Transitioning)

#### Requirement: VotedFor Cleared

When adopting a higher term, votedFor MUST be cleared (set to None).
The server has not voted in the new term yet. This prevents carrying
a stale vote into the new term.

### Leader Append-Only

#### Requirement: Log Extension Only

While a server is leader, its log MUST only grow. After any leader
operation, `leader_log_invariant(old_log, new_log)` MUST hold:
1. `new_log.len() >= old_log.len()`
2. For all i in [0, old_log.len()): `new_log[i] == old_log[i]`

##### Scenario: Append preserves

- GIVEN a leader with log [(t=1),(t=2)]
- WHEN appending entry (t=3, v=99)
- THEN log becomes [(t=1),(t=2),(t=3)] and old entries unchanged

##### Scenario: Truncation forbidden

- GIVEN a leader with log [(t=1),(t=2),(t=3)]
- WHEN any leader operation runs
- THEN log length MUST be >= 3 and entries at [0,3) unchanged

#### Requirement: Leader Append Term

A leader MUST only append entries with term == currentTerm. A leader
MUST NOT append entries with a term different from its own.

##### Scenario: Correct term

- GIVEN a leader in term 5
- WHEN appending a new entry
- THEN entry.term MUST be 5

### No-Op Commitment

#### Requirement: No-Op After Election

`make_noop_entry(current_term)` MUST return a log entry with the
current term and a distinguished no-op value. A new leader MUST append
this entry to commit pending entries from prior terms.

##### Scenario: New leader no-op

- GIVEN a leader just elected in term 5
- WHEN creating a no-op
- THEN entry MUST have term=5

#### Requirement: No-Op Commits Prior Entries

Once the no-op entry is committed (replicated to a quorum and its
term matches the current term), all entries preceding it in the log
are also committed. The no-op acts as a commitment barrier.

##### Scenario: Prior entries committed

- GIVEN log [(t=1),(t=2),(t=3,noop)] and the no-op at index 2 is committed
- WHEN checking entries 0 and 1
- THEN they are committed (prefix of a committed entry)

#### Requirement: No-Op Is Harmless

The no-op entry MUST NOT affect state machine output. Applying a
no-op to the state machine MUST be a no-op (identity operation).

### Leader Completeness

#### Requirement: Committed Entries Survive Elections

If an entry is committed at index i in term T, and a leader is elected
in term T' > T, then that leader's log MUST contain the same entry at
index i.

This follows from:
1. The entry was replicated to a quorum Q1 in term T
2. The new leader received votes from a quorum Q2 in term T'
3. Q1 ∩ Q2 ≠ ∅ (quorum intersection)
4. The shared voter has the entry and voted for the new leader
5. Vote granting requires epoch_ge (the new leader's log is at least
   as up-to-date), so the new leader has the entry

##### Scenario: Entry persists across elections

- GIVEN entry committed at index 3 in term 2, new leader elected in term 3
- WHEN checking new leader's log
- THEN log[3] MUST have term 2 (same entry)

#### Requirement: Quorum Intersection Dependency

The leader completeness proof MUST depend on
`quorum::prove_quorum_intersection` — two quorums of the same cluster
always share a member. This is the pigeonhole argument.

#### Requirement: Vote Freshness Dependency

The leader completeness proof MUST depend on
`ballot::can_grant_vote_spec` — specifically the `epoch_ge` requirement
that ensures the winning candidate's log is at least as up-to-date as
any voter's.

### State Machine Safety

#### Requirement: Index Uniqueness

If server A has applied entry E_a at log index i and server B has
applied entry E_b at log index i, then E_a == E_b (same term and
same value).

This follows from:
1. Both entries were committed (only committed entries are applied)
2. Committed entries have quorum replication
3. Log Matching Property: if two logs agree at index i, they agree
   at all j < i
4. Leader completeness: committed entries survive elections

##### Scenario: Two servers agree

- GIVEN servers A and B both committed through index 5
- WHEN comparing log[5]
- THEN A.log[5].term == B.log[5].term AND A.log[5].value == B.log[5].value

#### Requirement: Commit Index Dependency

State machine safety MUST depend on the commit advancement rule — only
entries with quorum replication at the current term can be committed.
This prevents applying entries that might be overwritten.

#### Requirement: Log Matching Dependency

State machine safety MUST depend on `log::logs_match_prefix_spec` —
agreement at an index implies agreement at all prior indices.

### Pre-Vote Protocol

#### Requirement: Pre-Vote Without Term Increment

`can_grant_pre_vote(my_term, candidate_term, candidate_last, my_last)` MUST
check that candidate_term >= my_term AND `epoch_ge(candidate_last, my_last)`.
The voter MUST NOT update its voted_term or voted_for. The candidate
MUST NOT increment its term.

##### Scenario: Grant pre-vote

- GIVEN my_term=3, candidate_term=4, candidate log more up-to-date
- WHEN evaluating pre-vote
- THEN MUST return true AND my_term remains 3

##### Scenario: Stale candidate

- GIVEN my_term=5, candidate_term=3
- WHEN evaluating pre-vote
- THEN MUST return false (candidate_term < my_term)

#### Requirement: Pre-Vote Preserves Term

After evaluating a pre-vote request (regardless of the decision), the
voter's current_term MUST be unchanged. Pre-votes MUST NOT cause term
advancement.

##### Scenario: Rejected pre-vote no side effect

- GIVEN my_term=5
- WHEN rejecting a pre-vote from term 3
- THEN my_term still 5

##### Scenario: Granted pre-vote no side effect

- GIVEN my_term=3
- WHEN granting a pre-vote for term 4
- THEN my_term still 3

#### Requirement: Pre-Vote Quorum Starts Election

A candidate that collects pre-votes from a quorum MUST then begin a
real election (increment term, request actual votes). `should_start_real_election(pre_votes, cluster_size)` MUST
return true when `is_quorum(pre_votes, cluster_size)`.

##### Scenario: Quorum pre-votes

- GIVEN 3 pre-votes in a 5-node cluster
- WHEN checking
- THEN MUST return true (3 >= majority(5) = 3)

#### Requirement: Partition Safety

A partitioned node that repeatedly fails pre-votes MUST NOT inflate
its term. Since pre-votes don't increment the term, the node's term
stays at its last known value. When the partition heals, the node's
term is not artificially high.

##### Scenario: Partitioned node

- GIVEN a node partitioned for 1000 pre-vote attempts
- WHEN partition heals
- THEN node's term MUST equal its term before the partition

### Read Index

#### Requirement: Quorum Confirmation

`confirm_read_authority(ack_count, cluster_size)` MUST return true
if and only if `is_quorum(ack_count, cluster_size)`. The leader
MUST collect heartbeat acknowledgments from a quorum before serving
a linearizable read.

##### Scenario: Quorum acks

- GIVEN 3 acks in a 5-node cluster
- WHEN checking
- THEN MUST return true

##### Scenario: Insufficient acks

- GIVEN 2 acks in a 5-node cluster
- WHEN checking
- THEN MUST return false

#### Requirement: Read After Commit

A linearizable read MUST NOT return data that precedes the leader's
commit index at the time the read was registered. `read_safe_at(read_index,
commit_index)` MUST return true when `commit_index >= read_index`.

##### Scenario: Committed past read

- GIVEN read registered at commit_index=5, current commit_index=7
- WHEN checking
- THEN MUST return true

##### Scenario: Not yet committed

- GIVEN read registered at commit_index=5, current commit_index=3
- WHEN checking
- THEN MUST return false

#### Requirement: Deposed Leader Rejection

If a leader cannot confirm quorum authority (heartbeat acks < majority),
it MUST NOT serve reads. A deposed leader that doesn't know it's been
replaced MUST fail the quorum check and reject the read.

##### Scenario: Deposed leader

- GIVEN a leader that only receives 1 ack (itself) in a 5-node cluster
- WHEN attempting a read
- THEN MUST reject (1 < majority(5) = 3)

### Joint Consensus

#### Requirement: Dual Quorum

`is_joint_quorum(count_old, total_old, count_new, total_new)` MUST
return true if and only if `is_quorum(count_old, total_old) AND
is_quorum(count_new, total_new)`. During a membership transition,
every decision MUST be approved by both configurations independently.

##### Scenario: Both quorums met

- GIVEN old config: 3/5, new config: 2/3
- WHEN checking joint quorum
- THEN MUST return true

##### Scenario: Old quorum not met

- GIVEN old config: 2/5, new config: 2/3
- WHEN checking joint quorum
- THEN MUST return false (2 < majority(5) = 3)

##### Scenario: New quorum not met

- GIVEN old config: 3/5, new config: 1/3
- WHEN checking joint quorum
- THEN MUST return false (1 < majority(3) = 2)

#### Requirement: Joint Intersection

If two joint quorums both hold for the same (total_old, total_new),
then the quorums overlap in BOTH configurations. Formally:
`a_old + b_old > total_old AND a_new + b_new > total_new`.

##### Scenario: Overlap in both

- GIVEN two joint quorums of old=5, new=3
- WHEN checking intersection
- THEN overlap in old config AND overlap in new config

#### Requirement: Transition Safety

During the transition from C_old to C_new, the system MUST use joint
consensus (C_old,new). After the joint configuration entry is committed,
the system transitions to C_new only. At no point can a decision be
made without approval from one of the active configurations.

##### Scenario: No gap in coverage

- GIVEN transition from [A,B,C,D,E] to [A,B,C,F,G]
- WHEN joint consensus is active
- THEN both {A,B,C,D,E} majority AND {A,B,C,F,G} majority required

#### Requirement: Single Server Change Subsumption

When adding or removing exactly one server, joint consensus reduces to
requiring a quorum of both old and new configurations. The existing
`voter_set::promote_voters` handles the set construction.
`is_joint_quorum` handles the quorum check.

### Install Snapshot

#### Requirement: Snapshot Trigger

`should_install_snapshot(next_index, first_log_index)` MUST return
true when `next_index <= first_log_index`. The leader sends a snapshot
when the follower needs entries that have been compacted.

##### Scenario: Follower behind compaction point

- GIVEN next_index=5, first_log_index=10 (entries [1,9] compacted)
- WHEN checking
- THEN MUST return true

##### Scenario: Follower within log

- GIVEN next_index=15, first_log_index=10
- WHEN checking
- THEN MUST return false

#### Requirement: Apply Snapshot to Log

`apply_snapshot(log, snapshot_index, snapshot_term)` MUST:
1. Discard all log entries at indices < snapshot_index
2. If the log has an entry at snapshot_index with matching term,
   preserve entries from snapshot_index onward
3. If the log does not have a matching entry at snapshot_index,
   replace the entire log with a single entry (snapshot_index, snapshot_term)

##### Scenario: Matching entry preserved

- GIVEN log [(t=1),(t=2),(t=3),(t=4)] and snapshot at index=2, term=3
- WHEN applying
- THEN result starts from index 2: [(t=3),(t=4)]

##### Scenario: No matching entry

- GIVEN log [(t=1),(t=2),(t=5)] and snapshot at index=2, term=3
- WHEN applying (log[2].term=5 ≠ 3)
- THEN result is [(t=3)] (snapshot entry only, log replaced)

#### Requirement: Snapshot Advances State

After applying a snapshot, the log's effective first index MUST be >=
snapshot_index. The snapshot MUST NOT cause the log to move backward.
`snapshot_index >= current_first_index` MUST hold as a precondition.

#### Requirement: Commit Index After Snapshot

After applying a snapshot with snapshot_index S, the commit index MUST
be >= S. All entries included in the snapshot are committed by definition.
`max(current_commit, snapshot_index)` MUST be the new commit index.

##### Scenario: Snapshot advances commit

- GIVEN current_commit=3, snapshot_index=10
- WHEN applying snapshot
- THEN commit index becomes 10
