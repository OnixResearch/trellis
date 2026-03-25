## Why

The library has verified building blocks for Raft — quorum arithmetic,
epoch comparison, monotonic advancement, append-only logs, ballot voting,
election state machines, voter set changes, timing validation, fencing
tokens, and snapshot integrity. These are the mathematical foundations.

What's missing is everything between "a leader is elected" and "entries
are committed." The log replication machinery. A leader tracks per-follower
progress with matchIndex and nextIndex arrays. Followers validate
AppendEntries by checking prevLogIndex/prevLogTerm consistency. Conflicts
are resolved by truncating the follower's log and appending the leader's
entries. The leader advances its commitIndex by finding the largest index
replicated to a quorum *in the current term* (the §5.4.2 "Figure 8" rule
that prevents ghost commits from prior terms).

On top of that, several protocol-level rules use the existing primitives
but aren't stated as composed properties: step-down on higher term, the
leader append-only invariant, the no-op entry after election, pre-vote
protocol, linearizable read confirmation, and joint consensus for
multi-node membership changes.

These are the functions where bugs cause split-brain, data loss, or stale
reads. A bad commitIndex calculation lets uncommitted entries become
visible. A missing step-down on higher term lets two leaders coexist.
A broken conflict resolution corrupts follower logs. Formal verification
of the decision logic prevents these failure modes.

## What Changes

Thirteen new verified modules covering log replication mechanics,
protocol-level rules, composed safety properties, and Raft extensions.

**Log Replication Mechanics (core):**

- **`append_entries_check`** — Follower-side AppendEntries consistency
  validation. Check that log[prevLogIndex].term == prevLogTerm before
  accepting entries. This is the runtime enforcement of the Log Matching
  Property.

- **`log_conflict`** — Find the divergence point between leader and
  follower entries, truncate the follower's log, then append new entries.
  The truncate-then-append composition that resolves conflicts.

- **`match_index`** — Per-follower matchIndex and nextIndex tracking.
  Advance matchIndex on success, backtrack nextIndex on rejection.
  Monotonicity invariants on matchIndex.

- **`commit_advance`** — Commit index advancement from match indices.
  Find the largest N where matchIndex[i] >= N for a quorum AND
  log[N].term == currentTerm. The §5.4.2 rule.

**Protocol-Level Rules:**

- **`term_authority`** — Step-down on higher term. Any server receiving
  a message with term > currentTerm must adopt the new term, revert to
  follower, and clear votedFor. The universal "all servers" rule.

- **`leader_append_only`** — A leader never overwrites or deletes
  entries in its own log. Formalized as: if state == Leader and log
  has length L, after any leader operation the log length is >= L and
  all entries at indices [0, L) are unchanged.

- **`noop_commit`** — No-op entry commitment after election. A new
  leader appends a no-op for its current term to force commitment of
  prior-term entries. Without this, entries from previous terms may
  remain uncommitted indefinitely.

**Composed Safety Properties:**

- **`leader_completeness`** — If an entry is committed in term T, it
  appears in every leader's log for terms > T. Composes vote-granting
  (ballot.can_grant_vote requires epoch_ge) with quorum intersection.

- **`state_machine_safety`** — No two servers apply different entries
  at the same log index. Follows from commit_advance (only commits at
  indices with quorum replication) plus log prefix matching.

**Raft Extensions:**

- **`pre_vote`** — Pre-vote protocol. A candidate collects pre-votes
  without incrementing its term. Prevents disruptive term inflation
  from partitioned nodes rejoining the cluster.

- **`read_index`** — Linearizable read confirmation. A leader confirms
  it still holds authority by collecting heartbeat acknowledgments
  from a quorum before serving a read. Without this, a deposed leader
  serves stale data.

- **`joint_consensus`** — Joint consensus for multi-node membership
  changes. During the transition, both old and new configurations must
  independently form quorums. Extends voter_set.rs which covers
  single-node changes only.

- **`install_snapshot`** — Snapshot installation trigger. When a
  leader's nextIndex for a follower falls behind the compacted log
  prefix, it sends a snapshot instead of AppendEntries. The follower
  replaces its log prefix with the snapshot.

## Capabilities

### New Capabilities
- `append-entries-check`: Verified prevLogIndex/prevLogTerm consistency check
- `log-conflict-resolution`: Verified find-divergence + truncate + append composition
- `match-index-tracking`: Verified matchIndex/nextIndex with monotonicity proofs
- `commit-advance`: Verified commit index from quorum of match indices with current-term guard
- `term-step-down`: Verified higher-term step-down rule
- `leader-append-only`: Verified leader log immutability invariant
- `noop-commit`: Verified no-op entry for prior-term commitment
- `leader-completeness`: Composed proof from ballot + quorum + epoch
- `state-machine-safety`: Composed proof from commit + log matching
- `pre-vote`: Verified pre-vote protocol without term inflation
- `read-index`: Verified quorum read confirmation
- `joint-consensus`: Verified dual-quorum membership changes
- `install-snapshot`: Verified snapshot trigger and log replacement

### Modified Capabilities
- `cross-module-properties`: Properties linking replication modules to
  existing quorum, epoch, log, ballot, monotonic, election, voter_set

## Impact

- **Files**: 13 new src/ modules, src/lib.rs additions, docs/requirements.md
  additions, src/properties.rs additions, delta specs for consensus domain
- **APIs**: ~80 new pub fns across 13 modules
- **Dependencies**: None — uses existing library modules and vstd
- **Testing**: Verus verification + proptest property tests + model oracles
