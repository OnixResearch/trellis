# Raft Consensus Domain Primitive Specification

## Purpose

Defines spec-only requirements for a formally verified Raft consensus primitive
family. These primitives MUST use exact finite models for nodes, terms, logs,
messages, quorums, configurations, snapshots, finite traces, deterministic
transition admission, and safety invariants. They MUST avoid network delivery,
wall-clock timing, storage durability, production cluster behavior, performance,
availability, byzantine faults, cryptographic identity, external system
correctness, or liveness claims unless a later focused OpenSpec declares exact
finite assumptions for those models.

## ADDED Requirements

### Requirement: Finite Raft Model Envelope [r[domain-primitives.raft-consensus.model]]
The system MUST define a finite Raft model envelope containing declared node
identities, term bounds, log-index bounds, command identifiers, message kinds,
configuration epochs, and explicit validity predicates before any Raft transition
or invariant references them.

#### Scenario: Unknown node is rejected [r[domain-primitives.raft-consensus.model.scenario.unknown-node-rejected]]

- GIVEN a Raft state or message referencing a node absent from the finite node set
- WHEN model admission is evaluated
- THEN the reference is rejected with unknown-node metadata

### Requirement: Node State Envelope [r[domain-primitives.raft-consensus.node-state]]
The system MUST define follower, candidate, and leader state envelopes whose term,
log, vote, commit, and applied metadata are admitted by the finite Raft model.

#### Scenario: Leader state must belong to current term [r[domain-primitives.raft-consensus.node-state.scenario.leader-state-current-term]]

- GIVEN a node marked as leader for a term different from its current term
- WHEN node-state admission is evaluated
- THEN the state is rejected with inconsistent-term metadata

### Requirement: Term Monotonicity Predicate [r[domain-primitives.raft-consensus.term]]
The system MUST define term predicates that keep current terms monotone across
admitted local transitions and update a node to a larger observed term before
processing term-bearing messages.

#### Scenario: Lower observed term cannot decrease current term [r[domain-primitives.raft-consensus.term.scenario.lower-term-cannot-decrease-current-term]]

- GIVEN a node with current term `t` and an incoming message with term `u` where `u < t`
- WHEN term update admission is evaluated
- THEN the admitted successor keeps current term `t` and marks the message stale

### Requirement: Log Entry Envelope [r[domain-primitives.raft-consensus.log-entry]]
The system MUST define log-entry envelopes containing a finite index, term,
command identifier, and optional client metadata without claiming client I/O or
application semantics.

#### Scenario: Log entry index must be in bounds [r[domain-primitives.raft-consensus.log-entry.scenario.index-in-bounds]]

- GIVEN a log entry whose index is outside the finite model bounds
- WHEN log-entry admission is evaluated
- THEN the entry is rejected with index-bound metadata

### Requirement: Log Prefix and No-Gap Predicate [r[domain-primitives.raft-consensus.log-prefix]]
The system MUST define log predicates for contiguous index ranges, prefix
relations, append-only extension, suffix truncation, and no-gap admission.

#### Scenario: Gap in log is rejected [r[domain-primitives.raft-consensus.log-prefix.scenario.gap-rejected]]

- GIVEN a finite log whose entries skip an admitted index before the last entry
- WHEN log admission is evaluated
- THEN the log is rejected with gap-index metadata

### Requirement: Persistent State Metadata [r[domain-primitives.raft-consensus.persistent-state]]
The system MUST define persisted-state metadata for current term, voted-for, and
log contents as an explicit finite state component without claiming storage
durability, fsync behavior, or crash-proof media.

#### Scenario: Persisted vote must reference admitted node or none [r[domain-primitives.raft-consensus.persistent-state.scenario.vote-reference-admitted]]

- GIVEN persisted voted-for metadata referencing an undeclared node
- WHEN persistent-state admission is evaluated
- THEN the metadata is rejected with vote-reference metadata

### Requirement: Volatile State Metadata [r[domain-primitives.raft-consensus.volatile-state]]
The system MUST define volatile-state metadata for commit index, last applied,
leader next-index vectors, and leader match-index vectors using finite bounds and
admitted node references.

#### Scenario: Applied index cannot exceed commit index [r[domain-primitives.raft-consensus.volatile-state.scenario.applied-not-past-commit]]

- GIVEN a node state with last-applied index greater than its commit index
- WHEN volatile-state admission is evaluated
- THEN the state is rejected with applied-after-commit metadata

### Requirement: Vote Grant Predicate [r[domain-primitives.raft-consensus.vote]]
The system MUST define deterministic vote-grant predicates that enforce one vote
per term, admitted candidate identity, monotone term update, and Raft log
up-to-date comparison over finite log metadata.

#### Scenario: Second vote in same term is rejected [r[domain-primitives.raft-consensus.vote.scenario.second-vote-same-term-rejected]]

- GIVEN a voter that has already granted a vote to candidate `a` in term `t`
- WHEN a vote request from distinct candidate `b` in term `t` is admitted
- THEN the request is rejected with already-voted metadata

### Requirement: RequestVote Admission [r[domain-primitives.raft-consensus.request-vote]]
The system MUST define RequestVote request and response admission envelopes over
finite term, candidate, last-log-index, last-log-term, voter, and grant metadata.

#### Scenario: Candidate with stale log is rejected [r[domain-primitives.raft-consensus.request-vote.scenario.stale-log-rejected]]

- GIVEN a RequestVote request whose candidate log is not at least as up to date as the voter log
- WHEN RequestVote admission is evaluated
- THEN the request is rejected with stale-log metadata

### Requirement: Election Safety Invariant [r[domain-primitives.raft-consensus.election-safety]]
The system MUST define and later prove an election-safety invariant stating that
at most one leader can be admitted for any finite term under the declared quorum
intersection and vote-grant predicates.

#### Scenario: Two leaders in one term violate election safety [r[domain-primitives.raft-consensus.election-safety.scenario.two-leaders-one-term-rejected]]

- GIVEN a finite vote history that would admit two distinct leaders for the same term
- WHEN election-safety admission is evaluated
- THEN the history is rejected with conflicting-leader metadata

### Requirement: AppendEntries Admission [r[domain-primitives.raft-consensus.append-entries]]
The system MUST define AppendEntries request and response admission over finite
leader identity, term, previous-log index/term, entries, leader-commit metadata,
and explicit conflict/rejection metadata.

#### Scenario: Previous log mismatch rejects append [r[domain-primitives.raft-consensus.append-entries.scenario.prev-log-mismatch-rejected]]

- GIVEN an AppendEntries request whose previous-log index is present with a different term in the follower log
- WHEN AppendEntries admission is evaluated
- THEN the request is rejected with previous-log-conflict metadata

### Requirement: Follower Log Update Predicate [r[domain-primitives.raft-consensus.follower-log-update]]
The system MUST define follower log-update predicates that preserve the matching
prefix, truncate only conflicting suffixes, append admitted entries contiguously,
and leave the log unchanged on rejected AppendEntries requests.

#### Scenario: Rejected append does not mutate follower log [r[domain-primitives.raft-consensus.follower-log-update.scenario.rejected-append-no-mutation]]

- GIVEN a follower log and a rejected AppendEntries request
- WHEN the follower log-update predicate is evaluated
- THEN the successor log equals the original log and the rejection metadata identifies the conflict

### Requirement: Log Matching Invariant [r[domain-primitives.raft-consensus.log-matching]]
The system MUST define and later prove the Raft log-matching invariant: if two
admitted logs contain entries with the same index and term, then all earlier
entries in those logs are equal.

#### Scenario: Equal index and term require equal prefix [r[domain-primitives.raft-consensus.log-matching.scenario.equal-index-term-require-equal-prefix]]

- GIVEN two admitted logs with equal entries at index `i` and term `t`
- WHEN log-matching admission is evaluated
- THEN every admitted earlier index has equal log entries in both logs

### Requirement: Commit Index Predicate [r[domain-primitives.raft-consensus.commit]]
The system MUST define commit-index predicates that admit an index only when a
quorum of admitted match indexes covers it and the governing leader-term rule is
satisfied by the finite model.

#### Scenario: Commit without quorum is rejected [r[domain-primitives.raft-consensus.commit.scenario.no-quorum-rejected]]

- GIVEN a proposed commit index not covered by a declared quorum of match indexes
- WHEN commit-index admission is evaluated
- THEN the proposal is rejected with insufficient-quorum metadata

### Requirement: Leader Completeness Invariant [r[domain-primitives.raft-consensus.leader-completeness]]
The system MUST define and later prove leader completeness: every entry committed
in a term is present in the logs of later admitted leaders according to finite
quorum, vote, and log up-to-date predicates.

#### Scenario: Later leader missing committed entry is rejected [r[domain-primitives.raft-consensus.leader-completeness.scenario.later-leader-missing-committed-entry-rejected]]

- GIVEN a committed entry and a later-term leader log missing that entry
- WHEN leader-completeness admission is evaluated
- THEN the later leader history is rejected with missing-committed-entry metadata

### Requirement: State Machine Safety Invariant [r[domain-primitives.raft-consensus.state-machine-safety]]
The system MUST define and later prove state-machine safety: if two nodes apply a
command at the same log index, then the applied command identifiers at that index
are equal.

#### Scenario: Divergent applied commands are rejected [r[domain-primitives.raft-consensus.state-machine-safety.scenario.divergent-applied-commands-rejected]]

- GIVEN two admitted applied-prefix records with different command identifiers at the same index
- WHEN state-machine-safety admission is evaluated
- THEN the pair is rejected with divergent-command metadata

### Requirement: Applied Prefix Predicate [r[domain-primitives.raft-consensus.applied-prefix]]
The system MUST define applied-prefix predicates that apply only committed log
prefixes, preserve prefix order, and never apply entries beyond the admitted
commit index.

#### Scenario: Applying uncommitted entry is rejected [r[domain-primitives.raft-consensus.applied-prefix.scenario.uncommitted-apply-rejected]]

- GIVEN an apply step whose target index is greater than the admitted commit index
- WHEN applied-prefix admission is evaluated
- THEN the step is rejected with uncommitted-apply metadata

### Requirement: Snapshot Metadata Predicate [r[domain-primitives.raft-consensus.snapshot]]
The system MUST define finite snapshot metadata containing last-included index,
last-included term, configuration metadata, and state-machine summary identity
without claiming file transfer, disk durability, or application snapshot
correctness.

#### Scenario: Snapshot must match compacted prefix boundary [r[domain-primitives.raft-consensus.snapshot.scenario.matches-prefix-boundary]]

- GIVEN a snapshot whose last-included index and term do not match the compacted log prefix boundary
- WHEN snapshot admission is evaluated
- THEN the snapshot is rejected with boundary-mismatch metadata

### Requirement: InstallSnapshot Admission [r[domain-primitives.raft-consensus.install-snapshot]]
The system MUST define InstallSnapshot admission that accepts only term-valid,
leader-valid, prefix-compatible snapshot metadata and preserves Raft safety
invariants across finite snapshot installation steps.

#### Scenario: Stale snapshot is rejected [r[domain-primitives.raft-consensus.install-snapshot.scenario.stale-snapshot-rejected]]

- GIVEN an InstallSnapshot request whose last-included index is not newer than the receiver snapshot boundary
- WHEN InstallSnapshot admission is evaluated
- THEN the request is rejected with stale-snapshot metadata

### Requirement: Membership Epoch Predicate [r[domain-primitives.raft-consensus.membership]]
The system MUST define finite membership/configuration epochs, voter sets,
non-voter metadata, and joint-consensus quorum predicates without claiming dynamic
service discovery or production reconfiguration behavior.

#### Scenario: Joint consensus requires old and new quorum [r[domain-primitives.raft-consensus.membership.scenario.joint-consensus-requires-both-quorums]]

- GIVEN a joint-configuration commit candidate
- WHEN joint-consensus quorum admission is evaluated
- THEN the candidate is admitted only if it satisfies quorum predicates for both old and new voter sets

### Requirement: Message Envelope Predicate [r[domain-primitives.raft-consensus.message]]
The system MUST define finite Raft message envelopes for RequestVote,
RequestVoteResponse, AppendEntries, AppendEntriesResponse, InstallSnapshot, and
local timeout/client-proposal events as model inputs without claiming network
transport delivery or ordering.

#### Scenario: Stale message is rejected deterministically [r[domain-primitives.raft-consensus.message.scenario.stale-message-rejected]]

- GIVEN a message whose term is lower than the receiver current term
- WHEN message admission is evaluated
- THEN the message is rejected with stale-term metadata and no unsafe state mutation

### Requirement: Client Proposal Envelope [r[domain-primitives.raft-consensus.client-proposal]]
The system MUST define client-proposal envelopes as finite command identifiers
accepted only through an admitted leader log-append predicate, without claiming
client retry semantics, end-to-end linearizability, or external application
correctness.

#### Scenario: Follower proposal is rejected [r[domain-primitives.raft-consensus.client-proposal.scenario.follower-proposal-rejected]]

- GIVEN a client proposal addressed to a node not admitted as leader for its current term
- WHEN client-proposal admission is evaluated
- THEN the proposal is rejected with not-leader metadata

### Requirement: Transition Trace Predicate [r[domain-primitives.raft-consensus.trace]]
The system MUST define finite transition traces whose adjacent states are related
by one admitted Raft transition kind and whose rejected transition attempts carry
no unsafe mutation.

#### Scenario: Trace step must be admitted transition [r[domain-primitives.raft-consensus.trace.scenario.step-must-be-admitted-transition]]

- GIVEN a finite trace containing a successor state not produced by any admitted transition predicate
- WHEN trace admission is evaluated
- THEN the trace is rejected with invalid-step metadata

### Requirement: Invariant Bundle Preservation [r[domain-primitives.raft-consensus.invariant-bundle]]
The system MUST define a Raft invariant bundle that includes model validity, term
monotonicity, log validity, vote consistency, election safety, log matching,
commit safety, leader completeness, applied-prefix safety, snapshot compatibility,
and membership quorum validity, and later prove each admitted transition preserves
that bundle.

#### Scenario: Unsafe transition violates invariant bundle [r[domain-primitives.raft-consensus.invariant-bundle.scenario.unsafe-transition-rejected]]

- GIVEN a transition candidate that would break any invariant in the declared bundle
- WHEN invariant-preservation admission is evaluated
- THEN the candidate is rejected with the failing invariant identifier

### Requirement: Deterministic Raft Admission [r[domain-primitives.raft-consensus.admission]]
The system MUST define deterministic Raft admission and rejection envelopes for
public exec surfaces that combine finite model, state, message, transition,
quorum, log, snapshot, membership, and invariant checks.

#### Scenario: Admission reports first failing finite check [r[domain-primitives.raft-consensus.admission.scenario.first-failing-check-reported]]

- GIVEN a Raft state or transition candidate failing one or more declared finite checks
- WHEN deterministic Raft admission is evaluated
- THEN the result rejects the candidate with stable failure metadata and does not claim runtime network, clock, storage, or availability behavior

### Requirement: OpenRaft Log Pointer Lattice [r[domain-primitives.raft-consensus.openraft-log-pointers]]
The system MUST define finite OpenRaft-inspired log pointer metadata for purged,
snapshot, applied, committed, flushed, submitted, and accepted indexes, and MUST
reject pointer regressions that violate the ordered pointer lattice without
claiming disk durability or storage implementation behavior.

#### Scenario: Pointer regression is rejected [r[domain-primitives.raft-consensus.openraft-log-pointers.scenario.pointer-regression-rejected]]

- GIVEN finite log pointers where applied exceeds committed or submitted exceeds accepted
- WHEN log-pointer admission is evaluated
- THEN the metadata is rejected with pointer-order failure evidence

### Requirement: OpenRaft I/O Progress [r[domain-primitives.raft-consensus.openraft-io-progress]]
The system MUST define finite flushed, submitted, and accepted I/O progress
cursors, require flushed <= submitted <= accepted, and admit progress updates only
when every cursor moves monotonically.

#### Scenario: Backwards I/O cursor update is rejected [r[domain-primitives.raft-consensus.openraft-io-progress.scenario.backwards-cursor-rejected]]

- GIVEN a finite I/O progress update that moves any cursor backwards
- WHEN progress-update admission is evaluated
- THEN the update is rejected without claiming storage completion behavior

### Requirement: OpenRaft Vote/Log I/O Ordering [r[domain-primitives.raft-consensus.openraft-vote-log-io]]
The system MUST define finite vote/log I/O ordering predicates that admit a log
write only when term/vote metadata for that log term has already been ordered
first. The predicate MUST NOT claim fsync, crash recovery, or storage durability.

#### Scenario: Log write before term metadata is rejected [r[domain-primitives.raft-consensus.openraft-vote-log-io.scenario.log-before-term-rejected]]

- GIVEN a finite log write whose term is newer than the persisted term metadata
- WHEN vote/log I/O admission is evaluated
- THEN the write is rejected with ordering-failure evidence

### Requirement: OpenRaft Effective Membership Rollback [r[domain-primitives.raft-consensus.openraft-effective-membership]]
The system MUST define finite committed/effective membership metadata where the
committed epoch is not newer than the effective epoch, and MUST define rollback
of conflicting uncommitted effective membership to the committed epoch.

#### Scenario: Conflicting effective membership rolls back [r[domain-primitives.raft-consensus.openraft-effective-membership.scenario.rollback-to-committed]]

- GIVEN a committed membership epoch and a newer uncommitted effective epoch
- WHEN conflicting-log rollback is evaluated
- THEN the effective epoch is restored to the committed epoch

### Requirement: OpenRaft Replication Session Isolation [r[domain-primitives.raft-consensus.openraft-replication-session]]
The system MUST define finite replication-session identity from leader term,
leader node, and governing membership log index. Progress updates MUST be
admitted only for the current session, with monotone matched index bounded by the
leader log.

#### Scenario: Stale replication progress is rejected [r[domain-primitives.raft-consensus.openraft-replication-session.scenario.stale-progress-rejected]]

- GIVEN delayed replication progress from an old leader or old membership log session
- WHEN progress-update admission is evaluated
- THEN the update is rejected and cannot mutate the current session progress

### Requirement: OpenRaft Read Linearizer Readiness [r[domain-primitives.raft-consensus.openraft-read-linearizer]]
The system MUST define finite read-linearizer readiness as local last-applied
index catching up to the read log index. The predicate MUST NOT claim quorum RPC,
lease timing, clock-drift, network delivery, or end-to-end application
linearizability behavior.

#### Scenario: Read before apply catch-up is rejected [r[domain-primitives.raft-consensus.openraft-read-linearizer.scenario.apply-catchup-required]]

- GIVEN a read log index greater than the local last-applied index
- WHEN read-linearizer readiness is evaluated
- THEN the read is not ready under the finite predicate
