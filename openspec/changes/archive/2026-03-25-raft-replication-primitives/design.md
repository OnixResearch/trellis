## Context

The library's Raft coverage so far:

```
quorum (majority, intersection, fault tolerance)
  → ballot (vote granting, single-vote-per-term, election safety)
    → election (state machine, fencing tokens, timing)
      → election_jitter, raft_timing (operational config)
epoch (lexicographic term,index comparison)
  → ballot (log freshness check for vote granting)
monotonic (is_advance, is_strict_advance)
  → used by terms, commit indices, fencing tokens
log (agree_at, prefix matching, append, truncate)
  → log matching property, append-preserves-prefix
voter_set (promote, replace, quorum preserved)
fencing (issue, valid write, stale rejection)
snapshot_integrity (dual-hash, chain hash, tamper detection)
```

The gap is the operational layer between election and commitment.
Raft's safety depends on a chain:

```
quorum intersection
  → election safety (at most one leader per term)
    → leader completeness (committed entries survive elections)
      → state machine safety (all replicas converge)
```

The bottom two links exist. The top two don't. And the replication
machinery that makes commitment work — AppendEntries validation,
conflict resolution, matchIndex tracking, commit advancement — is
entirely absent.

## Goals / Non-Goals

**Goals:**
- Verified AppendEntries consistency check (prevLogIndex/prevLogTerm)
- Verified log conflict resolution (find divergence, truncate, append)
- Verified matchIndex/nextIndex tracking with monotonicity invariants
- Verified commit index advancement with current-term guard (§5.4.2)
- Verified step-down on higher term
- Verified leader append-only invariant
- Verified no-op entry for prior-term commitment
- Composed safety proofs: leader completeness, state machine safety
- Verified pre-vote, read index, joint consensus, install snapshot
- All modules follow existing pattern: spec fn + exec fn + proof fn + tests
- Cross-module properties linking to quorum, epoch, log, ballot, monotonic

**Non-Goals:**
- Full Raft protocol state machine (async, messages, timers)
- Network modeling or message reordering
- Liveness proofs (depend on timing assumptions)
- RPC serialization or transport
- Snapshot data transfer mechanics (only the trigger logic)

## Decisions

### 1. AppendEntries check as a pure predicate on log state

**Choice:** `check_append_entries(log, prev_log_index, prev_log_term) -> bool`
that returns true when the log has an entry at prev_log_index with matching
term, or when prev_log_index is 0 (empty prefix). No message types.

**Rationale:** The check is a predicate on local log state plus two
integers from the RPC. Modeling it as a pure function separates the
verification target from transport concerns. The existing `log::agree_at`
already checks term equality at an index — this builds on it.

**Alternative:** Model AppendEntries as a message struct with all fields.
Rejected — adds type ceremony without strengthening the proof. The fields
we don't check (leaderId, leaderCommit, entries[]) are irrelevant to
the consistency predicate.

### 2. Log conflict resolution as a three-step composition

**Choice:** Three functions composed:
1. `find_conflict(follower_log, leader_entries, start_index) -> Option<usize>`
   — first index where terms disagree
2. `resolve_conflict(follower_log, leader_entries, start_index) -> Vec<LogEntry>`
   — truncate at conflict point, append leader entries
3. Proof that the result satisfies `logs_match_prefix` with the leader's
   log through start_index + leader_entries.len()

**Rationale:** Breaking conflict resolution into find + resolve mirrors
the Raft paper's description and makes each step independently verifiable.
The composition proof chains: find_conflict identifies where to cut,
truncate preserves the matching prefix, append adds the leader's entries.
The existing `log_truncate` and `log_append` proofs are reused directly.

**Alternative:** Single monolithic function. Rejected — harder to verify
and reuse. The find step is useful independently for fast-backtrack
optimizations.

### 3. Match index as parallel arrays with monotonicity invariant

**Choice:** Model matchIndex and nextIndex as `Seq<u64>` indexed by
server ID. `update_match_index(matches, server, new_match)` requires
`new_match >= matches[server]` (monotonic). `backtrack_next_index(nexts,
server)` decrements by 1 (or to a conflict hint). `compute_commit_index`
scans matchIndex for the quorum threshold.

**Rationale:** Raft's matchIndex is monotonically non-decreasing per
follower. This is exactly `monotonic::is_advance`. nextIndex may decrease
on rejection but has a floor at 1. The parallel array representation
matches the Raft paper and avoids HashMap modeling in Verus.

**Alternative:** Use a Map<ServerId, u64>. Rejected — Verus Map
reasoning is heavier than Seq indexing for fixed-size arrays.

### 4. Commit advancement with current-term guard as a separate predicate

**Choice:** `can_commit_at(log, match_indices, index, current_term,
cluster_size) -> bool` checks three conditions:
1. `index > current_commit_index`
2. `log[index].term == current_term`
3. count of `match_indices[i] >= index` forms a quorum

`advance_commit_index` finds the largest such index.

**Rationale:** The current-term guard is the critical safety rule from
§5.4.2. Without it, a leader in term 4 could "commit" an entry from
term 2 by counting replicas — but if a different leader in term 3
already overwrote that entry on some replicas, the "committed" entry
isn't safe. Separating the guard as a predicate makes it explicit and
independently testable.

**Alternative:** Fold the term check into the quorum scan. Rejected —
the term check is the subtle part; making it a separate predicate
with its own proof forces attention to it.

### 5. Term authority as a pure state transition function

**Choice:** `apply_higher_term(current_term, current_state, voted_for,
message_term) -> (new_term, new_state, new_voted_for)` that returns
updated state. When message_term > current_term: adopt message_term,
become follower, clear voted_for. Otherwise: no change.

**Rationale:** This is the "all servers" rule from the Raft paper.
It's a pure function from (state, message_term) to new state. No
message routing or RPC handling needed. The proof shows that after
applying a higher term, the server's term is >= the message term
(monotonic) and the server is a follower.

### 6. Leader append-only as an invariant predicate, not a wrapper

**Choice:** `leader_log_invariant(old_log, new_log) -> bool` checks
that new_log.len() >= old_log.len() and all entries in [0, old_log.len())
are unchanged. This is checked after any leader operation.

**Rationale:** A wrapper type that prevents truncation would need unsafe
interior mutability or a restricted API. An invariant predicate is
simpler: the proof obligation is that every leader-side function
preserves it. The existing `log::append_preserves_prefix` already
proves half of this.

### 7. Composed proofs reference existing modules, don't duplicate

**Choice:** Leader completeness and state machine safety are proof
modules that import specs from ballot, quorum, epoch, log, and
commit_advance. They state the composed theorem and prove it by chaining
existing lemmas.

**Rationale:** The whole point of the building-block approach is
composition. Duplicating quorum intersection inside a leader completeness
proof wastes effort and creates maintenance burden. The proof should
cite `quorum::prove_quorum_intersection` directly.

**Alternative:** Self-contained proofs that re-derive everything.
Rejected — defeats the library's purpose.

### 8. Joint consensus as dual-quorum predicate

**Choice:** `is_joint_quorum(count_old, total_old, count_new, total_new) -> bool`
requires `is_quorum(count_old, total_old) && is_quorum(count_new, total_new)`.
During the transition phase, every decision requires approval from both
configurations independently.

**Rationale:** Joint consensus is a quorum policy, not a new data
structure. The existing `quorum::is_quorum` handles each configuration.
The joint predicate composes them with AND. The intersection proof
extends: two joint quorums overlap in BOTH configurations.

### 9. Pre-vote as a ballot variant without term increment

**Choice:** `can_grant_pre_vote(state, candidate_term, candidate_last,
my_last, my_term) -> bool` — same log freshness check as
`ballot::can_grant_vote` but the voter does NOT update its voted_term.
The candidate does NOT increment its term until pre-votes succeed.

**Rationale:** Pre-vote is isomorphic to the regular vote except for
state mutation. The log comparison is identical (`epoch_ge`). The
safety argument is that a partitioned node's pre-vote requests don't
disrupt the cluster because they don't advance any term.

### 10. Phased implementation following dependency order

**Choice:** Build in six phases:

1. **AppendEntries + Conflict Resolution** — Foundation for replication.
   Uses existing log module.
2. **Match Index + Commit Advance** — Core replication tracking. Uses
   existing quorum and monotonic modules.
3. **Term Authority + Leader Append-Only + No-Op** — Protocol rules.
   Uses existing election and ballot modules.
4. **Leader Completeness + State Machine Safety** — Composed proofs.
   Chains everything from phases 1-3 plus existing modules.
5. **Pre-Vote + Read Index** — Extensions that reuse ballot and quorum.
6. **Joint Consensus + Install Snapshot** — Membership and compaction.
   Extends voter_set and log.

Each phase ends with verus + tracey clean. Phases 1 and 3 can overlap
(no code dependency). Phase 4 depends on 1-3 completing.

## Risks / Trade-offs

**[Composed proofs may need intermediate lemmas]** → Leader completeness
chains ballot (vote granting requires up-to-date log) + quorum
intersection (two quorums overlap) + epoch comparison (log freshness is
transitive). Z3 may not connect these across module boundaries without
explicit intermediate assertions. Mitigation: write step-by-step proof
with one assertion per logical step, citing each lemma explicitly.

**[Commit advancement scan over matchIndex is O(n²)]** → Finding the
largest index where a quorum of matchIndex values are >= N requires
checking each candidate N. Verus loops need explicit invariants at
each level. Mitigation: structure as an outer loop over candidate
indices (descending) and inner quorum count, with the invariant that
all indices above the current candidate have been checked.

**[Joint consensus intersection proof]** → Two joint quorums must
overlap in both old and new configurations. This requires four
quorum applications of the intersection theorem (old-old, old-new,
new-old, new-new). The proof is mechanical but verbose. Mitigation:
factor into a lemma that calls prove_quorum_intersection twice.

**[Pre-vote state tracking]** → Pre-vote responses don't update
voted_term, so the voter state type diverges from regular ballots.
This means pre_vote can't reuse VoteState directly. Mitigation:
pre_vote module defines its own PreVoteState or works with raw
parameters (no struct).

**[Install snapshot log replacement]** → Replacing a log prefix with
a snapshot conceptually sets all entries below the snapshot index to
"compacted." Verus Seq doesn't support sparse representation.
Mitigation: model as `apply_snapshot(log, snapshot_index, snapshot_term)
-> Seq<LogEntry>` that returns a fresh log starting from the snapshot
point. Prove that the suffix (entries after snapshot_index) is preserved.
