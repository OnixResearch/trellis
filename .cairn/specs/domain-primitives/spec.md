# domain-primitives Specification

## Purpose
This specification defines reusable finite domain primitive requirements for protocol, data, scheduling, storage, parsing, numeric, and related verified-logic families. It records the admitted model envelopes, validity boundaries, and scenario-level obligations that implementation slices compose into machine-checked Rust/Verus predicates and proofs.

## Requirements

### Requirement: Finite Raft Model Envelope
r[domain-primitives.raft-consensus.model]
The system MUST expose finite Raft model envelope predicates over a non-empty
one-based node universe, term bounds, log-index bounds, command identifiers,
message kinds, and configuration epochs. Model admission MUST reject unknown
nodes or out-of-bound references with deterministic first-failure metadata before
any higher-level transition predicate relies on those values.

#### Scenario: Unknown node is rejected
r[r[domain-primitives.raft-consensus.model.scenario.unknown-node-rejected]]

- GIVEN a Raft state or message referencing a node absent from the finite node set
- WHEN model admission is evaluated
- THEN the reference is rejected with unknown-node metadata

### Requirement: Node State Envelope
r[domain-primitives.raft-consensus.node-state]
The system MUST expose follower, candidate, and leader state envelopes whose
node identity, term, role term, vote, log, commit, applied, and configuration
metadata are admitted by the finite Raft model. Node-state admission MUST reject
invalid roles, leader role-term drift, candidate non-self-votes, and
applied/commit/log ordering violations with deterministic metadata.

#### Scenario: Leader state must belong to current term
r[r[domain-primitives.raft-consensus.node-state.scenario.leader-state-current-term]]

- GIVEN a node marked as leader for a term different from its current term
- WHEN node-state admission is evaluated
- THEN the state is rejected with inconsistent-leader-term metadata

### Requirement: Term Monotonicity Predicate
r[domain-primitives.raft-consensus.term]
The system MUST define term predicates that keep current terms monotone across
admitted local transitions and update a node to a larger observed term before
processing term-bearing messages.

#### Scenario: Lower observed term cannot decrease current term
r[r[domain-primitives.raft-consensus.term.scenario.lower-term-cannot-decrease-current-term]]

- GIVEN a node with current term `t` and an incoming message with term `u` where `u < t`
- WHEN term update admission is evaluated
- THEN the admitted successor keeps current term `t` and marks the message stale

### Requirement: Log Entry Envelope
r[domain-primitives.raft-consensus.log-entry]
The system MUST define log-entry envelopes containing a finite index, term,
command identifier, and optional client metadata without claiming client I/O or
application semantics.

#### Scenario: Log entry index must be in bounds
r[r[domain-primitives.raft-consensus.log-entry.scenario.index-in-bounds]]

- GIVEN a log entry whose index is outside the finite model bounds
- WHEN log-entry admission is evaluated
- THEN the entry is rejected with index-bound metadata

### Requirement: Log Prefix and No-Gap Predicate
r[domain-primitives.raft-consensus.log-prefix]
The system MUST define log predicates for contiguous index ranges, prefix
relations, append-only extension, suffix truncation, and no-gap admission.

#### Scenario: Gap in log is rejected
r[r[domain-primitives.raft-consensus.log-prefix.scenario.gap-rejected]]

- GIVEN a finite log whose entries skip an admitted index before the last entry
- WHEN log admission is evaluated
- THEN the log is rejected with gap-index metadata

### Requirement: Persistent State Metadata
r[domain-primitives.raft-consensus.persistent-state]
The system MUST define persisted-state metadata for current term, voted-for, and
log contents as an explicit finite state component without claiming storage
durability, fsync behavior, or crash-proof media.

#### Scenario: Persisted vote must reference admitted node or none
r[r[domain-primitives.raft-consensus.persistent-state.scenario.vote-reference-admitted]]

- GIVEN persisted voted-for metadata referencing an undeclared node
- WHEN persistent-state admission is evaluated
- THEN the metadata is rejected with vote-reference metadata

### Requirement: Volatile State Metadata
r[domain-primitives.raft-consensus.volatile-state]
The system MUST define volatile-state metadata for commit index, last applied,
leader next-index vectors, and leader match-index vectors using finite bounds and
admitted node references.

#### Scenario: Applied index cannot exceed commit index
r[r[domain-primitives.raft-consensus.volatile-state.scenario.applied-not-past-commit]]

- GIVEN a node state with last-applied index greater than its commit index
- WHEN volatile-state admission is evaluated
- THEN the state is rejected with applied-after-commit metadata

### Requirement: Vote Grant Predicate
r[domain-primitives.raft-consensus.vote]
The system MUST define deterministic vote-grant predicates that enforce one vote
per term, admitted candidate identity, monotone term update, and Raft log
up-to-date comparison over finite log metadata.

#### Scenario: Second vote in same term is rejected
r[r[domain-primitives.raft-consensus.vote.scenario.second-vote-same-term-rejected]]

- GIVEN a voter that has already granted a vote to candidate `a` in term `t`
- WHEN a vote request from distinct candidate `b` in term `t` is admitted
- THEN the request is rejected with already-voted metadata

### Requirement: RequestVote Admission
r[domain-primitives.raft-consensus.request-vote]
The system MUST define RequestVote request and response admission envelopes over
finite term, candidate, last-log-index, last-log-term, voter, and grant metadata.

#### Scenario: Candidate with stale log is rejected
r[r[domain-primitives.raft-consensus.request-vote.scenario.stale-log-rejected]]

- GIVEN a RequestVote request whose candidate log is not at least as up to date as the voter log
- WHEN RequestVote admission is evaluated
- THEN the request is rejected with stale-log metadata

### Requirement: Election Safety Invariant
r[domain-primitives.raft-consensus.election-safety]
The system MUST define and later prove an election-safety invariant stating that
at most one leader can be admitted for any finite term under the declared quorum
intersection and vote-grant predicates.

#### Scenario: Two leaders in one term violate election safety
r[r[domain-primitives.raft-consensus.election-safety.scenario.two-leaders-one-term-rejected]]

- GIVEN a finite vote history that would admit two distinct leaders for the same term
- WHEN election-safety admission is evaluated
- THEN the history is rejected with conflicting-leader metadata

### Requirement: AppendEntries Admission
r[domain-primitives.raft-consensus.append-entries]
The system MUST define AppendEntries request and response admission over finite
leader identity, term, previous-log index/term, entries, leader-commit metadata,
and explicit conflict/rejection metadata.

#### Scenario: Previous log mismatch rejects append
r[r[domain-primitives.raft-consensus.append-entries.scenario.prev-log-mismatch-rejected]]

- GIVEN an AppendEntries request whose previous-log index is present with a different term in the follower log
- WHEN AppendEntries admission is evaluated
- THEN the request is rejected with previous-log-conflict metadata

### Requirement: Follower Log Update Predicate
r[domain-primitives.raft-consensus.follower-log-update]

The system MUST expose a finite follower log-update implementation surface for
AppendEntries outcomes that leaves the follower log unchanged on rejection,
preserves the matching prefix on accepted requests, truncates only conflicting
suffix entries, appends admitted entries contiguously, and records deterministic
conflict/update metadata.

#### Scenario: Rejected append does not mutate follower log
r[r[domain-primitives.raft-consensus.follower-log-update.scenario.rejected-append-no-mutation]]

- GIVEN a follower log and AppendEntries request fail previous-log admission
- WHEN the follower log-update predicate is evaluated
- THEN the successor log equals the original log and rejection metadata identifies the first conflict

#### Scenario: Accepted append preserves matching prefix
r[r[domain-primitives.raft-consensus.follower-log-update.scenario.accepted-append-preserves-prefix]]

- GIVEN an AppendEntries request has a matching previous-log boundary and admitted entries
- WHEN the follower log-update predicate is evaluated
- THEN every entry at or before the previous-log index is preserved in the successor log

#### Scenario: Accepted append is contiguous after truncation
r[r[domain-primitives.raft-consensus.follower-log-update.scenario.accepted-append-contiguous]]

- GIVEN an AppendEntries request replaces a conflicting suffix with admitted entries
- WHEN the follower log-update predicate is evaluated
- THEN the successor log has no index gaps and appended entries begin exactly after the retained prefix

### Requirement: Log Matching Invariant
r[domain-primitives.raft-consensus.log-matching]
The system MUST define and later prove the Raft log-matching invariant: if two
admitted logs contain entries with the same index and term, then all earlier
entries in those logs are equal.

#### Scenario: Equal index and term require equal prefix
r[r[domain-primitives.raft-consensus.log-matching.scenario.equal-index-term-require-equal-prefix]]

- GIVEN two admitted logs with equal entries at index `i` and term `t`
- WHEN log-matching admission is evaluated
- THEN every admitted earlier index has equal log entries in both logs

### Requirement: Commit Index Predicate
r[domain-primitives.raft-consensus.commit]
The system MUST define commit-index predicates that admit an index only when a
quorum of admitted match indexes covers it and the governing leader-term rule is
satisfied by the finite model.

#### Scenario: Commit without quorum is rejected
r[r[domain-primitives.raft-consensus.commit.scenario.no-quorum-rejected]]

- GIVEN a proposed commit index not covered by a declared quorum of match indexes
- WHEN commit-index admission is evaluated
- THEN the proposal is rejected with insufficient-quorum metadata

### Requirement: Leader Completeness Invariant
r[domain-primitives.raft-consensus.leader-completeness]
The system MUST define and later prove leader completeness: every entry committed
in a term is present in the logs of later admitted leaders according to finite
quorum, vote, and log up-to-date predicates.

#### Scenario: Later leader missing committed entry is rejected
r[r[domain-primitives.raft-consensus.leader-completeness.scenario.later-leader-missing-committed-entry-rejected]]

- GIVEN a committed entry and a later-term leader log missing that entry
- WHEN leader-completeness admission is evaluated
- THEN the later leader history is rejected with missing-committed-entry metadata

### Requirement: State Machine Safety Invariant
r[domain-primitives.raft-consensus.state-machine-safety]
The system MUST define and later prove state-machine safety: if two nodes apply a
command at the same log index, then the applied command identifiers at that index
are equal.

#### Scenario: Divergent applied commands are rejected
r[r[domain-primitives.raft-consensus.state-machine-safety.scenario.divergent-applied-commands-rejected]]

- GIVEN two admitted applied-prefix records with different command identifiers at the same index
- WHEN state-machine-safety admission is evaluated
- THEN the pair is rejected with divergent-command metadata

### Requirement: Applied Prefix Predicate
r[domain-primitives.raft-consensus.applied-prefix]
The system MUST expose applied-prefix predicates that admit only monotone
contiguous state-machine application metadata. Non-empty steps MUST start at the
next unapplied index, the new last-applied index MUST be committed and present
in the local log, and no-op steps MUST carry no first-applied index.

#### Scenario: Applying uncommitted entry is rejected
r[r[domain-primitives.raft-consensus.applied-prefix.scenario.uncommitted-apply-rejected]]

- GIVEN an apply step whose target index is greater than the admitted commit index
- WHEN applied-prefix admission is evaluated
- THEN the step is rejected with uncommitted-apply metadata before state-machine execution

### Requirement: Snapshot Metadata Predicate
r[domain-primitives.raft-consensus.snapshot]
The system MUST define finite snapshot metadata containing last-included index,
last-included term, configuration metadata, and state-machine summary identity
without claiming file transfer, disk durability, or application snapshot
correctness.

#### Scenario: Snapshot must match compacted prefix boundary
r[r[domain-primitives.raft-consensus.snapshot.scenario.matches-prefix-boundary]]

- GIVEN a snapshot whose last-included index and term do not match the compacted log prefix boundary
- WHEN snapshot admission is evaluated
- THEN the snapshot is rejected with boundary-mismatch metadata

### Requirement: InstallSnapshot Admission
r[domain-primitives.raft-consensus.install-snapshot]
The system MUST define InstallSnapshot admission that accepts only term-valid,
leader-valid, prefix-compatible snapshot metadata and preserves Raft safety
invariants across finite snapshot installation steps.

#### Scenario: Stale snapshot is rejected
r[r[domain-primitives.raft-consensus.install-snapshot.scenario.stale-snapshot-rejected]]

- GIVEN an InstallSnapshot request whose last-included index is not newer than the receiver snapshot boundary
- WHEN InstallSnapshot admission is evaluated
- THEN the request is rejected with stale-snapshot metadata

### Requirement: Membership Epoch Predicate
r[domain-primitives.raft-consensus.membership]
The system MUST define finite membership/configuration epochs, voter sets,
non-voter metadata, and joint-consensus quorum predicates without claiming dynamic
service discovery or production reconfiguration behavior.

#### Scenario: Joint consensus requires old and new quorum
r[r[domain-primitives.raft-consensus.membership.scenario.joint-consensus-requires-both-quorums]]

- GIVEN a joint-configuration commit candidate
- WHEN joint-consensus quorum admission is evaluated
- THEN the candidate is admitted only if it satisfies quorum predicates for both old and new voter sets

### Requirement: Message Envelope Predicate
r[domain-primitives.raft-consensus.message]
The system MUST define finite Raft message envelopes for RequestVote,
RequestVoteResponse, AppendEntries, AppendEntriesResponse, InstallSnapshot, and
local timeout/client-proposal events as model inputs without claiming network
transport delivery or ordering.

#### Scenario: Stale message is rejected deterministically
r[r[domain-primitives.raft-consensus.message.scenario.stale-message-rejected]]

- GIVEN a message whose term is lower than the receiver current term
- WHEN message admission is evaluated
- THEN the message is rejected with stale-term metadata and no unsafe state mutation

### Requirement: Client Proposal Envelope
r[domain-primitives.raft-consensus.client-proposal]
The system MUST define client-proposal envelopes as finite command identifiers
accepted only through an admitted leader log-append predicate, without claiming
client retry semantics, end-to-end linearizability, or external application
correctness.

#### Scenario: Follower proposal is rejected
r[r[domain-primitives.raft-consensus.client-proposal.scenario.follower-proposal-rejected]]

- GIVEN a client proposal addressed to a node not admitted as leader for its current term
- WHEN client-proposal admission is evaluated
- THEN the proposal is rejected with not-leader metadata before append eligibility

### Requirement: Transition Trace Predicate
r[domain-primitives.raft-consensus.trace]
The system MUST define finite transition traces whose adjacent states are related
by one admitted Raft transition kind and whose rejected transition attempts carry
no unsafe mutation.

#### Scenario: Trace step must be admitted transition
r[r[domain-primitives.raft-consensus.trace.scenario.step-must-be-admitted-transition]]

- GIVEN a finite trace containing a successor state not produced by any admitted transition predicate
- WHEN trace admission is evaluated
- THEN the trace is rejected with invalid-step metadata

### Requirement: Invariant Bundle Preservation
r[domain-primitives.raft-consensus.invariant-bundle]
The system MUST define a Raft invariant bundle that includes model validity, term
monotonicity, log validity, vote consistency, election safety, log matching,
commit safety, leader completeness, applied-prefix safety, snapshot compatibility,
and membership quorum validity, and later prove each admitted transition preserves
that bundle.

#### Scenario: Unsafe transition violates invariant bundle
r[r[domain-primitives.raft-consensus.invariant-bundle.scenario.unsafe-transition-rejected]]

- GIVEN a transition candidate that would break any invariant in the declared bundle
- WHEN invariant-preservation admission is evaluated
- THEN the candidate is rejected with the failing invariant identifier

### Requirement: Deterministic Raft Admission
r[domain-primitives.raft-consensus.admission]
The system MUST define deterministic Raft admission and rejection envelopes for
public exec surfaces that combine finite model, state, message, transition,
quorum, log, snapshot, membership, and invariant checks.

#### Scenario: Admission reports first failing finite check
r[r[domain-primitives.raft-consensus.admission.scenario.first-failing-check-reported]]

- GIVEN a Raft state or transition candidate failing one or more declared finite checks
- WHEN deterministic Raft admission is evaluated
- THEN the result rejects the candidate with stable failure metadata and does not claim runtime network, clock, storage, or availability behavior

### Requirement: OpenRaft Log Pointer Lattice
r[domain-primitives.raft-consensus.openraft-log-pointers]
The system MUST define finite OpenRaft-inspired log pointer metadata for purged,
snapshot, applied, committed, flushed, submitted, and accepted indexes, and MUST
reject pointer regressions that violate the ordered pointer lattice without
claiming disk durability or storage implementation behavior. Admission metadata
MUST return OK exactly for the valid pointer predicate and otherwise report the
first pointer-order violation in stable lattice order.

#### Scenario: Pointer regression is rejected
r[r[domain-primitives.raft-consensus.openraft-log-pointers.scenario.pointer-regression-rejected]]

- GIVEN finite log pointers where applied exceeds committed or submitted exceeds accepted
- WHEN log-pointer admission is evaluated
- THEN the metadata is rejected with pointer-order failure evidence

#### Scenario: First pointer-order violation is reported
r[r[domain-primitives.raft-consensus.openraft-log-pointers.scenario.first-pointer-violation-reported]]

- GIVEN finite log pointers with multiple lattice violations
- WHEN log-pointer admission metadata is evaluated
- THEN the result reports the first violation in deterministic pointer-lattice order

### Requirement: OpenRaft I/O Progress
r[domain-primitives.raft-consensus.openraft-io-progress]
The system MUST define finite flushed, submitted, and accepted I/O progress
cursors, require flushed <= submitted <= accepted, and admit progress updates only
when every cursor moves monotonically. Admission metadata MUST return OK exactly
for the valid monotone-update predicate and otherwise report invalid old metadata,
invalid new metadata, or the first regressing cursor without claiming fsync,
crash recovery, or storage durability.

#### Scenario: Backwards I/O cursor update is rejected
r[r[domain-primitives.raft-consensus.openraft-io-progress.scenario.backwards-cursor-rejected]]

- GIVEN a finite I/O progress update that moves any cursor backwards
- WHEN progress-update admission is evaluated
- THEN the update is rejected without claiming storage completion behavior

#### Scenario: Invalid old progress is reported before regressions
r[r[domain-primitives.raft-consensus.openraft-io-progress.scenario.invalid-old-progress-first]]

- GIVEN invalid old progress metadata and a regressing update
- WHEN progress-update admission metadata is evaluated
- THEN invalid old progress is reported before cursor-regression metadata

### Requirement: OpenRaft Vote/Log I/O Ordering
r[domain-primitives.raft-consensus.openraft-vote-log-io]
The system MUST define finite vote/log I/O ordering predicates that admit a log
write only when term/vote metadata for that log term has already been ordered
first. Admission metadata MUST return OK exactly for the ordering predicate and
MUST NOT claim fsync, crash recovery, or storage durability.

#### Scenario: Log write before term metadata is rejected
r[r[domain-primitives.raft-consensus.openraft-vote-log-io.scenario.log-before-term-rejected]]

- GIVEN a finite log write whose term is newer than the persisted term metadata
- WHEN vote/log I/O admission is evaluated
- THEN the write is rejected with ordering-failure evidence

### Requirement: OpenRaft Effective Membership Rollback
r[domain-primitives.raft-consensus.openraft-effective-membership]
The system MUST define finite committed/effective membership metadata where the
committed epoch is not newer than the effective epoch, and MUST define rollback
of conflicting uncommitted effective membership to the committed epoch. Admission
metadata MUST return OK exactly for ordered committed/effective metadata and MUST
report older-effective metadata deterministically without claiming runtime
joint-consensus reconfiguration behavior.

#### Scenario: Conflicting effective membership rolls back
r[r[domain-primitives.raft-consensus.openraft-effective-membership.scenario.rollback-to-committed]]

- GIVEN a committed membership epoch and a newer uncommitted effective epoch
- WHEN conflicting-log rollback is evaluated
- THEN the effective epoch is restored to the committed epoch

### Requirement: OpenRaft Replication Session Isolation
r[domain-primitives.raft-consensus.openraft-replication-session]
The system MUST define finite replication-session identity from leader term,
leader node, and governing membership log index. Progress updates MUST be
admitted only for the current session, with monotone matched index bounded by the
leader log. Admission metadata MUST return OK exactly for the current-session
bounded-update predicate and otherwise reject stale sessions before matched-index
movement or leader-log-bound checks.

#### Scenario: Stale replication progress is rejected
r[r[domain-primitives.raft-consensus.openraft-replication-session.scenario.stale-progress-rejected]]

- GIVEN delayed replication progress from an old leader or old membership log session
- WHEN progress-update admission is evaluated
- THEN the update is rejected and cannot mutate the current session progress

#### Scenario: Stale session is reported before index checks
r[r[domain-primitives.raft-consensus.openraft-replication-session.scenario.stale-session-first]]

- GIVEN a stale replication session and a backwards matched-index update
- WHEN replication progress admission metadata is evaluated
- THEN stale-session metadata is reported before matched-index metadata

### Requirement: OpenRaft Read Linearizer Readiness
r[domain-primitives.raft-consensus.openraft-read-linearizer]
The system MUST define finite read-linearizer readiness as local last-applied
index catching up to the read log index. Admission metadata MUST return OK exactly
for the local-apply catch-up predicate and MUST NOT claim quorum RPC, lease
timing, clock-drift, network delivery, or end-to-end application linearizability
behavior.

#### Scenario: Read before apply catch-up is rejected
r[r[domain-primitives.raft-consensus.openraft-read-linearizer.scenario.apply-catchup-required]]

- GIVEN a read log index greater than the local last-applied index
- WHEN read-linearizer readiness is evaluated
- THEN the read is not ready under the finite predicate

### Requirement: Bounded Interval
r[domain-primitives.bounded-interval]
The system MUST provide a reusable bounded-interval primitive for normalized
closed integer ranges, containment, overlap, and clamping.

- **Module:** `src/bounded_interval.rs`
- **Validity boundary:** public constructors MUST reject intervals whose lower
  bound is greater than the upper bound or whose configured endpoints overflow
  the representation.
- **Exported safety:** accepted intervals MUST satisfy `lo <= hi`; containment
  and overlap predicates MUST only report facts consistent with those endpoints;
  clamp results MUST lie inside the accepted interval.

#### Scenario: Normalized interval contains interior value
r[r[domain-primitives.bounded-interval.scenario.normalized-interval-contains-interior-value]]

- GIVEN an accepted interval and a value between its endpoints
- WHEN containment is checked
- THEN the value is reported contained and the interval validity predicate holds

#### Scenario: Inverted interval is rejected
r[r[domain-primitives.bounded-interval.scenario.inverted-interval-is-rejected]]

- GIVEN a lower endpoint greater than the upper endpoint
- WHEN the checked interval constructor is evaluated
- THEN it rejects the interval

### Requirement: Rect Grid Index
r[domain-primitives.rect-grid-index]
The system MUST provide a reusable rectangular grid-index primitive for bounded
2D coordinates and flattened offsets.

- **Module:** `src/rect_grid_index.rs`
- **Validity boundary:** public constructors MUST reject zero dimensions where
  unsupported, dimensions whose area overflows, and coordinates outside
  `0 <= x < width` or `0 <= y < height`.
- **Exported safety:** accepted coordinates MUST flatten to a deterministic
  row-major offset derived from `y * width + x`. The checked public surface does
  not claim a verified inverse/unflatten API for arbitrary offsets.

#### Scenario: Valid coordinate flattens deterministically
r[r[domain-primitives.rect-grid-index.scenario.valid-coordinate-flattens-deterministically]]

- GIVEN a valid grid and coordinate inside width and height bounds
- WHEN the flattened offset is computed
- THEN the offset equals `y * width + x`

#### Scenario: Out-of-range coordinate is rejected
r[r[domain-primitives.rect-grid-index.scenario.out-of-range-coordinate-is-rejected]]

- GIVEN an x or y coordinate outside the grid dimensions
- WHEN coordinate admission is checked
- THEN the coordinate is rejected

### Requirement: GIS Coordinate Envelope
r[domain-primitives.gis-coordinate-envelope]
The system MUST provide a GIS coordinate-envelope primitive for checked scaled
latitude/longitude and projected-coordinate ranges.

- **Module:** `src/gis_coordinate_envelope.rs`
- **Composes:** `src/bounded_interval.rs` (`bounded_interval_valid_spec`, `bounded_interval_contains_spec`)
- **Validity boundary:** public checked constructors MUST reject coordinates
  outside the selected scaled-integer range; proof-only helpers MAY require the
  constructor-valid predicate.
- **Exported safety:** accepted coordinates MUST be within the configured domain
  bounds and conversions between stored coordinate wrappers and raw scaled values
  MUST preserve those bounds.

#### Scenario: Valid coordinate is admitted
r[r[domain-primitives.gis-coordinate-envelope.scenario.valid-coordinate-is-admitted]]

- GIVEN a scaled latitude and longitude inside the configured bounds
- WHEN the coordinate constructor is evaluated
- THEN it returns an admitted coordinate whose spec predicate holds

#### Scenario: Out-of-range coordinate is rejected
r[r[domain-primitives.gis-coordinate-envelope.scenario.out-of-range-coordinate-is-rejected]]

- GIVEN a latitude, longitude, or projected component outside the configured bounds
- WHEN the coordinate constructor is evaluated
- THEN it rejects the coordinate without producing a valid wrapper

### Requirement: GIS Bounding Box
r[domain-primitives.gis-bounding-box]
The system MUST provide a GIS bounding-box primitive for normalized rectangular
spatial envelopes.

- **Module:** `src/gis_bounding_box.rs`
- **Composes:** `src/bounded_interval.rs` (`bounded_interval_valid_spec`, `bounded_interval_contains_spec`, `bounded_interval_overlaps_spec`), `src/gis_coordinate_envelope.rs` (`gis_coordinate_valid_spec`)
- **Validity boundary:** public constructors MUST reject inverted boxes unless a
  named normalization helper is explicitly used; normalization MUST preserve the
  set of represented corner coordinates.
- **Exported safety:** accepted boxes MUST satisfy minimum-coordinate <=
  maximum-coordinate on every axis, and containment/intersection predicates MUST
  only return true for points or boxes inside the normalized bounds.

#### Scenario: Normalized box contains interior point
r[r[domain-primitives.gis-bounding-box.scenario.normalized-box-contains-interior-point]]

- GIVEN a valid bounding box and a coordinate inside both axis ranges
- WHEN containment is checked
- THEN the point is reported contained

#### Scenario: Inverted box is rejected
r[r[domain-primitives.gis-bounding-box.scenario.inverted-box-is-rejected]]

- GIVEN minimum coordinates greater than maximum coordinates on any axis
- WHEN the checked bounding-box constructor is evaluated
- THEN the constructor rejects the box

### Requirement: GIS Tile Index
r[domain-primitives.gis-tile-index]
The system MUST provide a GIS tile-index primitive for bounded map-tile
addressing and parent/child refinement.

- **Module:** `src/gis_tile_index.rs`
- **Composes:** `src/rect_grid_index.rs` (`rect_grid_valid_spec`, `rect_grid_coord_valid_spec`, `rect_grid_flatten_spec`)
- **Validity boundary:** zoom, x, and y indices MUST be checked against the
  configured maximum zoom and the `0 <= index < 2^zoom` tile range.
- **Exported safety:** accepted child tiles MUST refine to a valid parent tile at
  every lower zoom, and computed parent indices MUST remain in range.

#### Scenario: Child tile refines to parent
r[r[domain-primitives.gis-tile-index.scenario.child-tile-refines-to-parent]]

- GIVEN a valid tile index at zoom `z > 0`
- WHEN the parent tile is computed
- THEN the parent is valid at zoom `z - 1` and contains the child in the discrete tile hierarchy

#### Scenario: Tile coordinates beyond zoom range are rejected
r[r[domain-primitives.gis-tile-index.scenario.tile-coordinates-beyond-zoom-range-are-rejected]]

- GIVEN an x or y index outside the range for its zoom level
- WHEN tile admission is checked
- THEN the tile is rejected

### Requirement: GIS Linestring Topology
r[domain-primitives.gis-linestring-topology]
The system MUST provide a GIS linestring-topology primitive for discrete segment
chains.

- **Module:** `src/gis_linestring_topology.rs`
- **Composes:** `src/gis_coordinate_envelope.rs` (`gis_coordinate_valid_spec`), `src/gis_bounding_box.rs` (`gis_bounding_box_contains_spec`)
- **Validity boundary:** public constructors MUST reject empty chains, degenerate
  consecutive points, and unsupported self-intersections in the first-suite
  discrete model.
- **Exported safety:** accepted linestrings MUST have contiguous non-degenerate
  segments and MUST satisfy the chosen simple-topology predicate.

#### Scenario: Non-degenerate segment chain is accepted
r[r[domain-primitives.gis-linestring-topology.scenario.non-degenerate-segment-chain-is-accepted]]

- GIVEN a sequence of at least two coordinates with no duplicate consecutive points and no unsupported intersection
- WHEN linestring topology is checked
- THEN the linestring is accepted and every segment is valid

#### Scenario: Degenerate or self-intersecting chain is rejected
r[r[domain-primitives.gis-linestring-topology.scenario.degenerate-or-self-intersecting-chain-is-rejected]]

- GIVEN duplicate consecutive points or an unsupported crossing segment
- WHEN linestring topology is checked
- THEN the linestring is rejected

### Requirement: ML Tensor Shape
r[domain-primitives.ml-tensor-shape]
The system MUST provide an ML tensor-shape primitive for exact finite rank-1 dimensions
and flattened-index safety.

#### Scenario: Valid multi-index maps into buffer
r[r[domain-primitives.ml-tensor-shape.scenario.valid-multi-index-maps-into-buffer]]

- GIVEN a valid finite rank-1 tensor shape and an index within its dimension
- WHEN the flattened offset is computed
- THEN the offset is less than the tensor element count

#### Scenario: Invalid rank or index is rejected
r[r[domain-primitives.ml-tensor-shape.scenario.invalid-rank-or-index-is-rejected]]

- GIVEN a rank mismatch, zero dimension, or out-of-bounds index
- WHEN tensor-shape admission is checked
- THEN the request is rejected

### Requirement: ML Label Encoding
r[domain-primitives.ml-label-encoding]
The system MUST provide an ML label-encoding primitive for declared vocabulary
membership and class-index roundtrips.

#### Scenario: Declared label maps to stable index
r[r[domain-primitives.ml-label-encoding.scenario.declared-label-maps-to-stable-index]]

- GIVEN a finite vocabulary with unique labels
- WHEN a declared label is encoded and decoded
- THEN the decoded label equals the original label

#### Scenario: Unknown label is rejected
r[r[domain-primitives.ml-label-encoding.scenario.unknown-label-is-rejected]]

- GIVEN a label absent from the declared vocabulary
- WHEN label encoding is requested
- THEN the label is rejected

### Requirement: ML Dataset Split
r[domain-primitives.ml-dataset-split]
The system MUST provide an ML dataset-split primitive for deterministic disjoint
train, validation, and test partitions over bounded dataset indices.

#### Scenario: Split partitions are disjoint
r[r[domain-primitives.ml-dataset-split.scenario.split-partitions-are-disjoint]]

- GIVEN a valid dataset size and split plan
- WHEN partitions are produced
- THEN no dataset index appears in more than one partition

#### Scenario: Invalid split bounds are rejected
r[r[domain-primitives.ml-dataset-split.scenario.invalid-split-bounds-are-rejected]]

- GIVEN split sizes whose sum overflows or exceeds the dataset size
- WHEN split admission is checked
- THEN the split is rejected

### Requirement: ML Batch Iterator
r[domain-primitives.ml-batch-iterator]
The system MUST provide an ML batch-iterator primitive for bounded contiguous
batch ranges over dataset indices.

#### Scenario: Batches cover dataset range without overlap
r[r[domain-primitives.ml-batch-iterator.scenario.batches-cover-dataset-range-without-overlap]]

- GIVEN a valid dataset size and positive batch size
- WHEN batches are enumerated
- THEN each index in the dataset range appears in exactly one batch

#### Scenario: Invalid batch size is rejected
r[r[domain-primitives.ml-batch-iterator.scenario.invalid-batch-size-is-rejected]]

- GIVEN a zero batch size or overflowing range computation
- WHEN batch iteration is requested
- THEN the request is rejected

### Requirement: ML Feature Normalization
r[domain-primitives.ml-feature-normalization]
The system MUST provide an ML feature-normalization primitive for bounded feature
scaling and missing-value rejection.

#### Scenario: Bounded feature normalizes into target interval
r[r[domain-primitives.ml-feature-normalization.scenario.bounded-feature-normalizes-into-target-interval]]

- GIVEN valid feature metadata with non-zero span and a feature value inside the metadata range
- WHEN normalization is performed
- THEN the normalized value lies within the configured target interval

#### Scenario: Missing value or invalid span is rejected
r[r[domain-primitives.ml-feature-normalization.scenario.missing-value-or-invalid-span-is-rejected]]

- GIVEN a missing feature value or equal/inverted min/max metadata
- WHEN normalization is requested
- THEN the request is rejected

### Requirement: ML Top K
r[domain-primitives.ml-top-k]
The system MUST provide an ML top-k primitive for deterministic bounded selection
from finite score vectors.

#### Scenario: Top k results are ordered and in range
r[r[domain-primitives.ml-top-k.scenario.top-k-results-are-ordered-and-in-range]]

- GIVEN a finite score vector and valid k
- WHEN top-k is computed
- THEN every returned index is in range and results are ordered by score with deterministic tie-breaking

#### Scenario: Invalid k is rejected
r[r[domain-primitives.ml-top-k.scenario.invalid-k-is-rejected]]

- GIVEN k equal to zero or greater than the score-vector length
- WHEN top-k admission is checked
- THEN the request is rejected

### Requirement: ML Inference Envelope
r[domain-primitives.ml-inference-envelope]
The system MUST provide an ML inference-envelope primitive for deterministic
classification or regression result admission bounded by model metadata.

#### Scenario: Declared class result is accepted
r[r[domain-primitives.ml-inference-envelope.scenario.declared-class-result-is-accepted]]

- GIVEN model metadata with a finite class count and a result whose class index and score are in range
- WHEN result admission is checked
- THEN the classification result is accepted

#### Scenario: Output outside metadata is rejected
r[r[domain-primitives.ml-inference-envelope.scenario.output-outside-metadata-is-rejected]]

- GIVEN an unknown class index, invalid confidence score, or regression value outside the declared envelope
- WHEN result admission is checked
- THEN the result is rejected

### Requirement: ML Confusion Matrix
r[domain-primitives.ml-confusion-matrix]
The system MUST provide an ML confusion-matrix primitive for bounded metric counts
and denominator-safe derived metrics.

#### Scenario: Valid prediction updates bounded count
r[r[domain-primitives.ml-confusion-matrix.scenario.valid-prediction-updates-bounded-count]]

- GIVEN valid expected and predicted labels within the matrix class count
- WHEN the confusion matrix is updated
- THEN exactly the corresponding cell count increases within bounds

#### Scenario: Zero-denominator metric is not produced as numeric
r[r[domain-primitives.ml-confusion-matrix.scenario.zero-denominator-metric-is-not-produced-as-numeric]]

- GIVEN a class with no positive predictions or no actual positives
- WHEN precision or recall is requested
- THEN the primitive rejects the numeric metric or returns an explicit empty metric result

### Requirement: Physics Units
r[domain-primitives.physics-units]
The system MUST provide a physics-units primitive for dimension-safe scalar and
vector quantities.

#### Scenario: Compatible quantities combine safely
r[r[domain-primitives.physics-units.scenario.compatible-quantities-combine-safely]]

- GIVEN two quantities with the same dimension vector
- WHEN addition is requested
- THEN the result keeps that dimension vector

#### Scenario: Incompatible dimensions reject additive operation
r[r[domain-primitives.physics-units.scenario.incompatible-dimensions-reject-additive-operation]]

- GIVEN quantities with different dimension vectors
- WHEN addition or subtraction is requested
- THEN the operation is rejected

### Requirement: Physics Vector Bounds
r[domain-primitives.physics-vector-bounds]
The system MUST provide a physics vector-bounds primitive for component-wise and
aggregate envelope admission over exact discrete vectors.

#### Scenario: Vector inside envelope is admitted
r[r[domain-primitives.physics-vector-bounds.scenario.vector-inside-envelope-is-admitted]]

- GIVEN vector components within every configured bound
- WHEN vector admission is checked
- THEN the vector is accepted

#### Scenario: Component outside envelope is rejected
r[r[domain-primitives.physics-vector-bounds.scenario.component-outside-envelope-is-rejected]]

- GIVEN any vector component outside its configured bound
- WHEN vector admission is checked
- THEN the vector is rejected

### Requirement: Physics Time Horizon
r[domain-primitives.physics-time-horizon]
The system MUST provide a physics time-horizon primitive for bounded step-count and positive time-delta admission.

#### Scenario: Valid step count stays inside horizon
r[r[domain-primitives.physics-time-horizon.scenario.valid-step-count-stays-inside-horizon]]

- GIVEN a positive step size and bounded step horizon
- WHEN the step count is computed
- THEN every admitted step index is inside the configured finite horizon

#### Scenario: Invalid horizon parameters are rejected
r[r[domain-primitives.physics-time-horizon.scenario.overflowing-horizon-arithmetic-is-rejected]]

- GIVEN zero time-delta, zero step count, or a delta larger than the configured horizon
- WHEN horizon admission is checked
- THEN the horizon is rejected

### Requirement: Physics Integrator Step
r[domain-primitives.physics-integrator-step]
The system MUST provide a physics integrator-step primitive for bounded-time
state advancement.

#### Scenario: Bounded step advances time
r[r[domain-primitives.physics-integrator-step.scenario.bounded-step-advances-time]]

- GIVEN a finite modeled state and a valid positive time delta and non-saturated finite timestamp
- WHEN one integration step is admitted
- THEN the output timestamp advances by one modeled tick and state fields remain bounded

#### Scenario: Invalid time step is rejected
r[r[domain-primitives.physics-integrator-step.scenario.invalid-time-step-is-rejected]]

- GIVEN a zero delta, invalid bounded state, or saturated timestamp
- WHEN integration is requested
- THEN the step is rejected

### Requirement: Physics Collision Envelope
r[domain-primitives.physics-collision-envelope]
The system MUST provide a physics collision-envelope primitive for conservative
swept-volume overlap checks.

#### Scenario: Overlapping swept envelopes produce candidate
r[r[domain-primitives.physics-collision-envelope.scenario.overlapping-swept-envelopes-produce-candidate]]

- GIVEN two valid moving envelopes whose conservative swept volumes overlap
- WHEN collision admission is checked
- THEN the pair is returned as a collision candidate

#### Scenario: Separated envelopes are rejected
r[r[domain-primitives.physics-collision-envelope.scenario.separated-envelopes-are-rejected]]

- GIVEN two valid envelopes whose conservative swept volumes are disjoint
- WHEN collision admission is checked
- THEN the pair is rejected as non-colliding for the step

### Requirement: Physics Contact Manifold
r[domain-primitives.physics-contact-manifold]
The system MUST provide a physics contact-manifold primitive for bounded contact
point, normal, and penetration records.

#### Scenario: Valid bounded contact record is accepted
r[r[domain-primitives.physics-contact-manifold.scenario.valid-bounded-contact-record-is-accepted]]

- GIVEN contact points, normals, and penetration values inside configured envelopes
- WHEN contact admission is checked
- THEN the contact manifold is accepted

#### Scenario: Invalid contact record is rejected
r[r[domain-primitives.physics-contact-manifold.scenario.invalid-contact-record-is-rejected]]

- GIVEN an invalid normal, unsupported contact count, or out-of-envelope penetration value
- WHEN contact admission is checked
- THEN the contact manifold is rejected

### Requirement: Physics Energy Budget
r[domain-primitives.physics-energy-budget]
The system MUST provide a physics energy-budget primitive for non-negative energy
accounting and bounded drift.

#### Scenario: Energy-preserving transition is accepted
r[r[domain-primitives.physics-energy-budget.scenario.energy-preserving-transition-is-accepted]]

- GIVEN a valid pre-state and post-state whose energy difference is inside the configured drift budget
- WHEN the energy budget is checked
- THEN the transition is accepted

#### Scenario: Excess drift is rejected
r[r[domain-primitives.physics-energy-budget.scenario.excess-drift-is-rejected]]

- GIVEN a transition whose energy delta exceeds the configured budget
- WHEN the energy budget is checked
- THEN the transition is rejected

### Requirement: Physics Momentum Budget
r[domain-primitives.physics-momentum-budget]
The system MUST provide a physics momentum-budget primitive for non-negative mass
and bounded momentum accounting.

#### Scenario: Momentum update inside budget is accepted
r[r[domain-primitives.physics-momentum-budget.scenario.momentum-update-inside-budget-is-accepted]]

- GIVEN valid masses and momentum values whose delta is inside the configured budget
- WHEN the momentum budget is checked
- THEN the update is accepted

#### Scenario: Invalid mass or momentum drift is rejected
r[r[domain-primitives.physics-momentum-budget.scenario.invalid-mass-or-momentum-drift-is-rejected]]

- GIVEN negative mass or momentum drift outside the configured budget
- WHEN momentum admission is checked
- THEN the update is rejected

### Requirement: Physics Non-Claims
r[domain-primitives.physics-non-claims]
The system MUST limit physics primitives to exact discrete admission and budget predicates; it MUST NOT claim floating-point accuracy, continuous dynamics correctness, real-number differential-equation proofs, broad solver completeness, simulator fidelity, collision exactness beyond conservative interval candidates, or production physics-engine correctness.

#### Scenario: Continuous and floating-point claims are excluded
r[r[domain-primitives.physics-non-claims.scenario.continuous-and-floating-point-claims-are-excluded]]

- GIVEN a physics primitive result
- WHEN the result is interpreted
- THEN it is only evidence for the exact finite predicate and not for continuous or floating-point simulator correctness

### Requirement: Vision Image Buffer
r[domain-primitives.vision-image-buffer]
The system MUST provide a machine-vision image-buffer primitive for checked
width, height, stride, channel, and backing-buffer layout.

#### Scenario: Valid pixel address is in bounds
r[r[domain-primitives.vision-image-buffer.scenario.valid-pixel-address-is-in-bounds]]

- GIVEN a valid image buffer and pixel coordinate/channel inside declared bounds
- WHEN the byte offset is computed
- THEN the offset is inside the backing buffer

#### Scenario: Bad stride or buffer length is rejected
r[r[domain-primitives.vision-image-buffer.scenario.bad-stride-or-buffer-length-is-rejected]]

- GIVEN an image layout whose stride or backing length cannot contain all rows
- WHEN the image-buffer constructor is evaluated
- THEN it rejects the layout

### Requirement: Vision ROI
r[domain-primitives.vision-roi]
The system MUST provide a machine-vision ROI primitive for bounded regions of
interest and crop-coordinate mapping.

#### Scenario: ROI crop maps into source image
r[r[domain-primitives.vision-roi.scenario.roi-crop-maps-into-source-image]]

- GIVEN a valid image buffer and ROI inside the image bounds
- WHEN a crop-local coordinate is mapped to source coordinates
- THEN the source coordinate is inside the image buffer

#### Scenario: Out-of-image ROI is rejected
r[r[domain-primitives.vision-roi.scenario.out-of-image-roi-is-rejected]]

- GIVEN an ROI that extends outside the source image bounds
- WHEN the checked ROI constructor is evaluated
- THEN the ROI is rejected

### Requirement: Vision Convolution Window
r[domain-primitives.vision-convolution-window]
The system MUST provide a machine-vision convolution-window primitive for safe
kernel indexing under declared border policy.

#### Scenario: Interior kernel samples are in bounds
r[r[domain-primitives.vision-convolution-window.scenario.interior-kernel-samples-are-in-bounds]]

- GIVEN a valid image buffer, kernel shape, and anchor such that the window lies inside the image
- WHEN window sample offsets are computed
- THEN every sample offset is inside the source image buffer

#### Scenario: Unsupported border sample is rejected
r[r[domain-primitives.vision-convolution-window.scenario.unsupported-border-sample-is-rejected]]

- GIVEN a kernel window that crosses the image boundary with a reject-border policy
- WHEN sampling is requested
- THEN the window is rejected

### Requirement: Vision Color Layout
r[domain-primitives.vision-color-layout]
The system MUST provide a machine-vision color-layout primitive for declared
channel count, order, and per-channel byte offset admission.

#### Scenario: Declared channel offset is in bounds
r[r[domain-primitives.vision-color-layout.scenario.declared-channel-offset-is-in-bounds]]

- GIVEN a valid color layout and declared channel
- WHEN the channel offset is computed
- THEN the offset is inside the pixel stride

#### Scenario: Invalid channel layout is rejected
r[r[domain-primitives.vision-color-layout.scenario.invalid-channel-layout-is-rejected]]

- GIVEN duplicate channels, unknown channels, or channel offsets outside the pixel stride
- WHEN color-layout admission is checked
- THEN the layout is rejected

### Requirement: Vision Color Transform Envelope
r[domain-primitives.vision-color-transform-envelope]
The system MUST provide a machine-vision color-transform-envelope primitive for
bounded integer color-space transform outputs.

#### Scenario: Transform output stays inside envelope
r[r[domain-primitives.vision-color-transform-envelope.scenario.transform-output-stays-inside-envelope]]

- GIVEN valid integer coefficients, input channel bounds, and output bounds
- WHEN the transform envelope is checked
- THEN every admitted output channel lies inside the declared output bounds

#### Scenario: Overflowing transform is rejected
r[r[domain-primitives.vision-color-transform-envelope.scenario.overflowing-transform-is-rejected]]

- GIVEN coefficients or input bounds whose transform can overflow the representation
- WHEN transform admission is checked
- THEN the transform is rejected

### Requirement: Vision Image Pyramid
r[domain-primitives.vision-image-pyramid]
The system MUST provide a machine-vision image-pyramid primitive for bounded
multi-level image dimensions and level mapping.

#### Scenario: Pyramid levels decrease monotonically
r[r[domain-primitives.vision-image-pyramid.scenario.pyramid-levels-decrease-monotonically]]

- GIVEN a valid base image and pyramid scale plan
- WHEN pyramid dimensions are derived
- THEN each admitted level has positive dimensions no larger than the previous level

#### Scenario: Invalid pyramid scale is rejected
r[r[domain-primitives.vision-image-pyramid.scenario.invalid-pyramid-scale-is-rejected]]

- GIVEN a scale plan that yields zero dimensions or overflowing level metadata
- WHEN pyramid admission is checked
- THEN the pyramid is rejected

### Requirement: Vision Connected Components
r[domain-primitives.vision-connected-components]
The system MUST provide a machine-vision connected-components primitive for
bounded component labels and merge/equivalence facts.

#### Scenario: Component labels stay in declared range
r[r[domain-primitives.vision-connected-components.scenario.component-labels-stay-in-declared-range]]

- GIVEN a valid label image and component-count bound
- WHEN labels are admitted
- THEN every non-background label is within the declared component range

#### Scenario: Invalid label or merge is rejected
r[r[domain-primitives.vision-connected-components.scenario.invalid-label-or-merge-is-rejected]]

- GIVEN a label outside the declared range or a merge referencing an unknown component
- WHEN component admission is checked
- THEN the label or merge is rejected

### Requirement: Vision Detection NMS
r[domain-primitives.vision-detection-nms]
The system MUST provide a machine-vision detection/NMS primitive for deterministic
score thresholding and non-maximum suppression.

#### Scenario: Higher-scored overlapping box suppresses lower-scored box
r[r[domain-primitives.vision-detection-nms.scenario.higher-scored-overlapping-box-suppresses-lower-scored-box]]

- GIVEN two valid same-class boxes whose overlap exceeds the suppression threshold and whose scores are ordered
- WHEN NMS is evaluated
- THEN only the higher-scored box remains admitted

#### Scenario: Invalid detection is rejected
r[r[domain-primitives.vision-detection-nms.scenario.invalid-detection-is-rejected]]

- GIVEN a detection with an invalid box, unknown class, or score outside the configured range
- WHEN detection admission is checked
- THEN the detection is rejected before suppression

### Requirement: Numeric Fixed Point Scale
r[domain-primitives.numeric-fixed-point-scale]
The system MUST provide a numeric fixed-point scale primitive for checked scaled
integer metadata, conversion envelopes, and range preservation.

#### Scenario: Valid scaled conversion preserves envelope
r[r[domain-primitives.numeric-fixed-point-scale.scenario.valid-scaled-conversion-preserves-envelope]]

- GIVEN valid scale metadata and a raw integer inside the representable envelope
- WHEN the value is converted to the scaled representation
- THEN the scaled value is accepted and remains inside the declared envelope

#### Scenario: Invalid scale or conversion is rejected
r[r[domain-primitives.numeric-fixed-point-scale.scenario.invalid-scale-or-conversion-is-rejected]]

- GIVEN zero scale, overflowing scale metadata, or a raw value outside the representable envelope
- WHEN fixed-point admission is checked
- THEN the conversion is rejected

### Requirement: Numeric Checked Arithmetic Envelope
r[domain-primitives.numeric-checked-arithmetic-envelope]
The system MUST provide a numeric checked-arithmetic-envelope primitive for
bounded add, subtract, multiply, and safe zero-numerator divide-admission operations.

#### Scenario: Accepted arithmetic result stays within bounds
r[r[domain-primitives.numeric-checked-arithmetic-envelope.scenario.accepted-arithmetic-result-stays-within-bounds]]

- GIVEN operands and an operation whose exact result is representable and inside the declared bounds
- WHEN checked arithmetic is evaluated
- THEN the result is accepted and lies inside the bounds

#### Scenario: Overflowing or invalid arithmetic is rejected
r[r[domain-primitives.numeric-checked-arithmetic-envelope.scenario.overflowing-or-invalid-arithmetic-is-rejected]]

- GIVEN operands whose operation overflows, leaves the declared bounds, divides by zero, or is outside the safe finite divide-admission surface
- WHEN checked arithmetic is evaluated
- THEN the operation is rejected

### Requirement: Numeric Denominator Safe Ratio
r[domain-primitives.numeric-denominator-safe-ratio]
The system MUST provide a numeric denominator-safe-ratio primitive for explicit
non-zero denominator admission and empty/reject semantics.

#### Scenario: Non-zero denominator yields numeric ratio
r[r[domain-primitives.numeric-denominator-safe-ratio.scenario.non-zero-denominator-yields-numeric-ratio]]

- GIVEN a numerator and non-zero denominator whose ratio is representable
- WHEN ratio derivation is requested
- THEN a numeric ratio result is returned

#### Scenario: Zero denominator does not produce numeric ratio
r[r[domain-primitives.numeric-denominator-safe-ratio.scenario.zero-denominator-does-not-produce-numeric-ratio]]

- GIVEN a zero denominator
- WHEN ratio derivation is requested
- THEN the primitive rejects the request or returns an explicit empty result instead of a numeric ratio

### Requirement: Numeric Monotone Sequence
r[domain-primitives.numeric-monotone-sequence]
The system MUST provide a numeric monotone-sequence primitive for sortedness,
lower-bound search, and range-query safety over finite three-slot sequences.

#### Scenario: Bound search returns in-range partition point
r[r[domain-primitives.numeric-monotone-sequence.scenario.bound-search-returns-in-range-partition-point]]

- GIVEN a finite sorted sequence and a query value
- WHEN lower-bound search is evaluated
- THEN the returned index is inside `0..=len` and partitions the sequence according to the query predicate

#### Scenario: Unsorted sequence is rejected
r[r[domain-primitives.numeric-monotone-sequence.scenario.unsorted-sequence-is-rejected]]

- GIVEN a sequence with an out-of-order adjacent pair
- WHEN monotone sequence admission is checked
- THEN the sequence is rejected

### Requirement: Numeric Histogram Counter
r[domain-primitives.numeric-histogram-counter]
The system MUST provide a numeric histogram-counter primitive for bounded three-bucket indexing and count-update admission.

#### Scenario: Valid bucket update increments one count
r[r[domain-primitives.numeric-histogram-counter.scenario.valid-bucket-update-increments-one-count]]

- GIVEN a valid histogram and a value admitted to exactly one bucket
- WHEN the histogram is updated
- THEN exactly that bucket count increases and all other bucket counts are unchanged

#### Scenario: Invalid bucket or overflowing count is rejected
r[r[domain-primitives.numeric-histogram-counter.scenario.invalid-bucket-or-overflowing-count-is-rejected]]

- GIVEN a value outside all buckets or a bucket count at its maximum
- WHEN histogram update is requested
- THEN the update is rejected

### Requirement: Numeric Affine Transform Discrete
r[domain-primitives.numeric-affine-transform-discrete]
The system MUST provide a numeric affine-transform primitive for bounded integer
matrix/vector transform envelopes.

#### Scenario: Transform output remains inside declared envelope
r[r[domain-primitives.numeric-affine-transform-discrete.scenario.transform-output-remains-inside-declared-envelope]]

- GIVEN valid integer coefficients, input bounds, and output bounds
- WHEN the affine transform is admitted
- THEN every produced output component lies inside the declared output envelope

#### Scenario: Overflowing transform envelope is rejected
r[r[domain-primitives.numeric-affine-transform-discrete.scenario.overflowing-transform-envelope-is-rejected]]

- GIVEN coefficients, offsets, or input bounds whose transform can overflow or leave the declared output envelope
- WHEN affine transform admission is checked
- THEN the transform is rejected

### Requirement: Numeric Prefix Sum
r[domain-primitives.numeric-prefix-sum]
The system MUST provide a numeric prefix-sum primitive for bounded three-slot cumulative sums and range-sum admission.

#### Scenario: Prefix sums decompose range sum
r[r[domain-primitives.numeric-prefix-sum.scenario.prefix-sums-decompose-range-sum]]

- GIVEN a finite three-slot prefix tuple whose cumulative sums are representable
- WHEN a valid range sum is requested
- THEN the range sum equals the difference of the corresponding prefix sums

#### Scenario: Prefix sum overflow is rejected
r[r[domain-primitives.numeric-prefix-sum.scenario.prefix-sum-overflow-is-rejected]]

- GIVEN a sequence whose cumulative sum would overflow the representation
- WHEN prefix-sum construction is requested
- THEN the construction is rejected

### Requirement: Numeric Quantized Threshold
r[domain-primitives.numeric-quantized-threshold]
The system MUST provide a numeric quantized-threshold primitive for deterministic
threshold and bucket admission over bounded values.

#### Scenario: Value maps to deterministic quantized bucket
r[r[domain-primitives.numeric-quantized-threshold.scenario.value-maps-to-deterministic-quantized-bucket]]

- GIVEN valid ordered three-bucket thresholds and a value inside the supported range
- WHEN quantization is evaluated
- THEN the value maps to exactly one deterministic bucket

#### Scenario: Invalid thresholds are rejected
r[r[domain-primitives.numeric-quantized-threshold.scenario.invalid-thresholds-are-rejected]]

- GIVEN unordered three-bucket thresholds, duplicate thresholds where disallowed, or a value outside the supported range
- WHEN threshold admission is checked
- THEN the request is rejected

### Requirement: Data Structure Bounded Array
r[domain-primitives.ds-bounded-array]
The system MUST provide a bounded-array primitive for finite sequence length,
capacity, index admission, and point-update preservation.

#### Scenario: Valid point update changes only the target index
r[r[domain-primitives.ds-bounded-array.scenario.valid-point-update-changes-only-target-index]]

- GIVEN a valid bounded array and an index inside its current length
- WHEN a point update is applied
- THEN the resulting array has the same length, stores the new value at that index, and preserves every other index

#### Scenario: Out-of-range index is rejected without mutation
r[r[domain-primitives.ds-bounded-array.scenario.out-of-range-index-is-rejected-without-mutation]]

- GIVEN a valid bounded array and an index outside its current length
- WHEN a read or point update is requested
- THEN the request is rejected and the array state is unchanged

### Requirement: Data Structure Key Value Map
r[domain-primitives.ds-key-value-map]
The system MUST provide a finite key/value map primitive for unique keys,
lookup, insertion, replacement, and removal semantics.

#### Scenario: Insert creates or replaces exactly one key
r[r[domain-primitives.ds-key-value-map.scenario.insert-creates-or-replaces-exactly-one-key]]

- GIVEN a valid finite map and a key/value pair admitted by the map domain
- WHEN the pair is inserted
- THEN lookup for that key returns the inserted value and all other keys keep their previous lookup results

#### Scenario: Remove clears only the target key
r[r[domain-primitives.ds-key-value-map.scenario.remove-clears-only-target-key]]

- GIVEN a valid finite map and a target key
- WHEN the target key is removed
- THEN lookup for that key is empty and lookup for every other key is unchanged

### Requirement: Data Structure FIFO Queue
r[domain-primitives.ds-fifo-queue]
The system MUST provide a bounded FIFO queue primitive for enqueue/dequeue order,
capacity admission, and empty/full rejection.

#### Scenario: Dequeue returns oldest enqueued element
r[r[domain-primitives.ds-fifo-queue.scenario.dequeue-returns-oldest-enqueued-element]]

- GIVEN a valid non-empty FIFO queue built from accepted enqueue operations
- WHEN one element is dequeued
- THEN the returned element is the oldest remaining element and the queue retains the remaining elements in FIFO order

#### Scenario: Queue overflow or underflow is rejected
r[r[domain-primitives.ds-fifo-queue.scenario.queue-overflow-or-underflow-is-rejected]]

- GIVEN a full queue for enqueue or an empty queue for dequeue
- WHEN the invalid operation is requested
- THEN the operation is rejected and the queue state is unchanged

### Requirement: Data Structure Ring Buffer
r[domain-primitives.ds-ring-buffer]
The system MUST provide a ring-buffer primitive for bounded circular indexing,
occupied-slot validity, and wraparound push/pop behavior.

#### Scenario: Wraparound preserves logical order
r[r[domain-primitives.ds-ring-buffer.scenario.wraparound-preserves-logical-order]]

- GIVEN a valid ring buffer whose head or tail is near the physical end of storage
- WHEN accepted push and pop operations wrap around storage
- THEN logical element order is preserved and every occupied logical index maps to a valid physical slot

#### Scenario: Full or empty ring operation is rejected
r[r[domain-primitives.ds-ring-buffer.scenario.full-or-empty-ring-operation-is-rejected]]

- GIVEN a full ring buffer for push or an empty ring buffer for pop
- WHEN the invalid operation is requested
- THEN the operation is rejected and head, tail, length, and storage are unchanged

### Requirement: Data Structure Priority Queue
r[domain-primitives.ds-priority-queue]
The system MUST provide a bounded priority-queue primitive for priority-order
validity, deterministic tie handling, bounded push, and pop-min behavior.

#### Scenario: Pop returns deterministic minimum-priority element
r[r[domain-primitives.ds-priority-queue.scenario.pop-returns-deterministic-minimum-priority-element]]

- GIVEN a valid non-empty priority queue and a declared deterministic tie policy
- WHEN pop-min is requested
- THEN the returned element has minimum priority and satisfies the tie policy among all minimum-priority elements

#### Scenario: Invalid priority queue operation is rejected
r[r[domain-primitives.ds-priority-queue.scenario.invalid-priority-queue-operation-is-rejected]]

- GIVEN a full priority queue for push or an empty priority queue for pop
- WHEN the invalid operation is requested
- THEN the operation is rejected and the queue state is unchanged

### Requirement: Data Structure Union Find
r[domain-primitives.ds-union-find]
The system MUST provide a union-find primitive for finite parent/rank arrays,
root discovery, union connectivity, and invalid-index rejection.

#### Scenario: Union connects two valid sets
r[r[domain-primitives.ds-union-find.scenario.union-connects-two-valid-sets]]

- GIVEN a valid union-find forest and two valid element indices
- WHEN the sets containing those elements are unioned
- THEN the resulting forest remains valid and the two elements have the same representative root

#### Scenario: Invalid union-find index is rejected
r[r[domain-primitives.ds-union-find.scenario.invalid-union-find-index-is-rejected]]

- GIVEN a valid union-find forest and an index outside the element range
- WHEN find or union is requested with that index
- THEN the operation is rejected and the forest state is unchanged

### Requirement: Finite Principal Envelope
r[domain-primitives.access-control-policy.principal]
The system MUST define finite principal admission metadata for policy decision
models.

#### Scenario: Principal is declared before rule use
r[r[domain-primitives.access-control-policy.principal.scenario.declared-before-rule-use]]

- GIVEN finite principal metadata and a policy rule
- WHEN the rule references a principal
- THEN the principal is admitted by the declared principal envelope

### Requirement: Finite Resource Envelope
r[domain-primitives.access-control-policy.resource]
The system MUST define finite resource admission metadata for policy decision
models.

#### Scenario: Resource is declared before authorization
r[r[domain-primitives.access-control-policy.resource.scenario.declared-before-authorization]]

- GIVEN finite resource metadata and an authorization request
- WHEN the request references a resource
- THEN the resource is admitted by the declared resource envelope

### Requirement: Finite Action Envelope
r[domain-primitives.access-control-policy.action]
The system MUST define finite action admission metadata for policy decision
models.

#### Scenario: Action is declared before authorization
r[r[domain-primitives.access-control-policy.action.scenario.declared-before-authorization]]

- GIVEN finite action metadata and an authorization request
- WHEN the request references an action
- THEN the action is admitted by the declared action envelope

### Requirement: Policy Rule Envelope
r[domain-primitives.access-control-policy.rule]
The system MUST define policy-rule envelopes over finite principal, resource, and
action scopes with exact effect metadata.

#### Scenario: Rule scope uses admitted values
r[r[domain-primitives.access-control-policy.rule.scenario.scope-uses-admitted-values]]

- GIVEN a policy-rule envelope
- WHEN the rule scope is evaluated
- THEN every principal, resource, and action referenced by the rule is admitted by its declared envelope

### Requirement: Grant and Denial Decision Metadata
r[domain-primitives.access-control-policy.grant-denial]
The system MUST define deterministic grant/denial decision metadata for admitted
authorization requests.

#### Scenario: Decision records matched rule metadata
r[r[domain-primitives.access-control-policy.grant-denial.scenario.records-matched-rule-metadata]]

- GIVEN an admitted authorization request and finite policy rules
- WHEN a grant or denial decision is produced
- THEN the decision metadata identifies the matched admitted rule or the deterministic no-match result

### Requirement: Conflict Resolution Envelope
r[domain-primitives.access-control-policy.conflict]
The system MUST define deterministic conflict-resolution envelopes for overlapping
finite policy rules.

#### Scenario: Overlapping rules resolve deterministically
r[r[domain-primitives.access-control-policy.conflict.scenario.overlapping-rules-resolve-deterministically]]

- GIVEN overlapping admitted policy rules for the same principal, resource, and action
- WHEN conflict resolution is evaluated
- THEN the result follows declared priority or tie-break metadata deterministically

### Requirement: Bounded Delegation Envelope
r[domain-primitives.access-control-policy.delegation]
The system MUST define bounded delegation metadata over finite principals,
resources, actions, scopes, and chain-depth envelopes.

#### Scenario: Delegation states scope and bound
r[r[domain-primitives.access-control-policy.delegation.scenario.states-scope-and-bound]]

- GIVEN delegation metadata between admitted principals
- WHEN the delegation is used in an authorization model
- THEN it states the admitted resource/action scope and finite depth or chain bound

### Requirement: Authorization Admission Envelope
r[domain-primitives.access-control-policy.authorization]
The system MUST define deterministic authorization admission results for policy
metadata, including invalid principal, resource, action, rule, conflict, and
delegation shapes.

#### Scenario: Rejects undeclared principal deterministically
r[r[domain-primitives.access-control-policy.authorization.scenario.rejects-undeclared-principal-deterministically]]

- GIVEN authorization metadata with a request principal outside the declared principal envelope
- WHEN admission is evaluated
- THEN the metadata is rejected deterministically without claiming cryptographic authentication or whole-system security

### Requirement: Config Key Envelope
r[domain-primitives.configuration-feature-flag.key]
The system MUST provide finite config-key envelopes that record key identifiers,
optional namespaces, and deterministic known/unknown-key admission metadata.

#### Scenario: Unknown key rejected explicitly
r[r[domain-primitives.configuration-feature-flag.key.scenario.unknown-key-rejected-explicitly]]

- GIVEN a finite admitted config-key set
- WHEN a candidate key is not in the admitted set
- THEN the config-key primitive returns an explicit unknown-key decision without consulting an external store

### Requirement: Typed Value Envelope
r[domain-primitives.configuration-feature-flag.typed-value]
The system MUST provide finite typed-value envelopes that bind declared type tags
to admitted value shapes and reject type/value mismatches deterministically.

#### Scenario: Type mismatch rejected before use
r[r[domain-primitives.configuration-feature-flag.typed-value.scenario.type-mismatch-rejected-before-use]]

- GIVEN a declared config value type and candidate finite value envelope
- WHEN the candidate value shape does not match the declared type tag
- THEN the typed-value primitive rejects the value or exposes an explicit type-mismatch decision

### Requirement: Default Value Metadata
r[domain-primitives.configuration-feature-flag.default]
The system MUST provide default-value metadata that ties each default to a
declared finite key envelope and compatible typed-value envelope.

#### Scenario: Default must match declared key and type
r[r[domain-primitives.configuration-feature-flag.default.scenario.default-must-match-declared-key-and-type]]

- GIVEN default metadata for a config key
- WHEN the key is unknown or the default value does not match the declared type
- THEN the default primitive rejects the metadata without mutating the candidate config set

### Requirement: Override Precedence Envelope
r[domain-primitives.configuration-feature-flag.override-precedence]
The system MUST provide deterministic finite override-precedence envelopes for
admitted configuration sources and explicit tie/duplicate-source decisions.

#### Scenario: Highest admitted precedence selected deterministically
r[r[domain-primitives.configuration-feature-flag.override-precedence.scenario.highest-admitted-precedence-selected-deterministically]]

- GIVEN finite override candidates for the same admitted key
- WHEN the precedence envelope orders the candidate sources without ties
- THEN the override-precedence primitive selects the highest-precedence admitted candidate deterministically

### Requirement: Feature Gate Predicate Envelope
r[domain-primitives.configuration-feature-flag.gate]
The system MUST provide finite feature-gate predicate envelopes that map admitted
keys, subjects, and typed values to explicit enabled/disabled/rejected decisions.

#### Scenario: Gate rejects unknown subject metadata
r[r[domain-primitives.configuration-feature-flag.gate.scenario.gate-rejects-unknown-subject-metadata]]

- GIVEN a feature-gate predicate over finite admitted subject metadata
- WHEN a candidate subject lacks the required admitted metadata
- THEN the feature-gate primitive returns an explicit rejection decision rather than claiming runtime rollout behavior

### Requirement: Rollout Bucket Envelope
r[domain-primitives.configuration-feature-flag.rollout-bucket]
The system MUST provide deterministic rollout-bucket envelopes over finite
subject identifiers, bucket ranges, and declared membership predicates without
claiming randomness quality or distributed rollout consistency.

#### Scenario: Bucket range must be admitted
r[r[domain-primitives.configuration-feature-flag.rollout-bucket.scenario.bucket-range-must-be-admitted]]

- GIVEN finite rollout bucket metadata and a candidate subject bucket
- WHEN the bucket lies outside the admitted bucket range
- THEN the rollout-bucket primitive rejects the candidate or exposes an explicit out-of-range decision

### Requirement: Validation Decision Metadata
r[domain-primitives.configuration-feature-flag.validation-decision]
The system MUST provide validation decision metadata that records accepted,
rejected, and reason-coded invalid configuration entries over finite admitted
keys and typed values.

#### Scenario: Invalid entry preserves decision reason
r[r[domain-primitives.configuration-feature-flag.validation-decision.scenario.invalid-entry-preserves-decision-reason]]

- GIVEN a finite configuration entry with unknown key, mismatched type, or invalid precedence metadata
- WHEN validation is performed by the primitive
- THEN the validation decision records the rejection reason without mutating the input entry

### Requirement: Config Admission Envelope
r[domain-primitives.configuration-feature-flag.admission]
The system MUST provide deterministic configuration admission/rejection envelopes
that compose key, typed-value, default, override, gate, rollout, and validation
metadata over finite inputs.

#### Scenario: Admission requires all finite checks
r[r[domain-primitives.configuration-feature-flag.admission.scenario.admission-requires-all-finite-checks]]

- GIVEN finite configuration and feature-flag metadata
- WHEN any required key, type, default, override, gate, rollout, or validation check rejects
- THEN the config-admission primitive returns an explicit rejection decision before any runtime store or distributed flag-service claim is made

### Requirement: Constraint Variable Envelope
r[domain-primitives.constraint-solver.variable]
The system MUST define finite constraint-variable envelopes for variable
identifiers, declared domains, and variable membership metadata.

#### Scenario: Variable references declared domain
r[r[domain-primitives.constraint-solver.variable.scenario.references-declared-domain]]

- GIVEN a finite constraint variable and a domain table
- WHEN the variable is admitted by the variable envelope
- THEN its declared domain identifier exists in the finite domain table

### Requirement: Constraint Domain Predicate
r[domain-primitives.constraint-solver.domain]
The system MUST define finite domain predicates for admitted values, emptiness,
subset relationships, and membership checks.

#### Scenario: Domain membership rejects absent value
r[r[domain-primitives.constraint-solver.domain.scenario.rejects-absent-value]]

- GIVEN a finite domain and a candidate value
- WHEN the value is not a member of the declared domain set
- THEN the domain predicate rejects the value without claiming theory reasoning

### Requirement: Constraint Relation Envelope
r[domain-primitives.constraint-solver.relation]
The system MUST define finite constraint-relation envelopes for constraint
identifiers, variable scopes, tuple arity, and admitted tuple metadata.

#### Scenario: Relation tuple arity matches scope
r[r[domain-primitives.constraint-solver.relation.scenario.tuple-arity-matches-scope]]

- GIVEN a finite relation with a declared variable scope
- WHEN a candidate tuple is checked against the relation
- THEN the relation envelope admits only tuples whose arity matches the declared scope

### Requirement: Constraint Assignment Shape
r[domain-primitives.constraint-solver.assignment]
The system MUST define assignment predicates for partial and total assignments,
variable uniqueness, and value admission against declared domains.

#### Scenario: Assignment rejects value outside declared domain
r[r[domain-primitives.constraint-solver.assignment.scenario.rejects-value-outside-domain]]

- GIVEN a finite assignment for a declared variable
- WHEN the assigned value is outside the variable's declared finite domain
- THEN the assignment predicate rejects the assignment

### Requirement: Constraint Consistency Predicate
r[domain-primitives.constraint-solver.consistency]
The system MUST define deterministic local consistency predicates for assignments
against finite constraint relations.

#### Scenario: Consistency checks assigned relation scope
r[r[domain-primitives.constraint-solver.consistency.scenario.checks-assigned-relation-scope]]

- GIVEN a finite relation and an assignment covering every variable in the relation scope
- WHEN the assigned tuple is not admitted by the relation envelope
- THEN the consistency predicate reports the assignment as locally inconsistent

### Requirement: Constraint Propagation Envelope
r[domain-primitives.constraint-solver.propagation]
The system MUST define monotone propagation envelopes for finite domain narrowing,
changed-domain metadata, and contradiction detection.

#### Scenario: Propagation never expands finite domain
r[r[domain-primitives.constraint-solver.propagation.scenario.never-expands-finite-domain]]

- GIVEN a finite domain state and a propagated domain state
- WHEN propagation metadata is admitted
- THEN every propagated domain is a subset of its corresponding input domain

### Requirement: Constraint Witness Metadata
r[domain-primitives.constraint-solver.witness]
The system MUST define finite witness metadata for satisfying assignments and
explicit contradictions in declared finite models.

#### Scenario: Satisfying witness carries admitted total assignment
r[r[domain-primitives.constraint-solver.witness.scenario.carries-admitted-total-assignment]]

- GIVEN a finite constraint model and a candidate satisfying witness
- WHEN the witness is admitted
- THEN it carries a total assignment admitted by every variable domain and locally consistent with every declared relation

### Requirement: Constraint Admission Envelope
r[domain-primitives.constraint-solver.admission]
The system MUST define deterministic constraint admission envelopes that return
admit/reject/unknown metadata from finite variables, domains, relations,
assignments, propagation, and witness predicates.

#### Scenario: Admission rejects inconsistent assignment deterministically
r[r[domain-primitives.constraint-solver.admission.scenario.rejects-inconsistent-assignment-deterministically]]

- GIVEN a finite constraint model and a candidate assignment
- WHEN the assignment violates an admitted relation or declared domain
- THEN the admission envelope rejects it with deterministic metadata and does not claim solver optimality or external-engine completeness

### Requirement: Control Setpoint
r[domain-primitives.control-setpoint]
The system MUST provide a control setpoint primitive for bounded target values,
setpoint update admission, and invalid-bound rejection.

#### Scenario: Valid setpoint update stays within bounds
r[r[domain-primitives.control-setpoint.scenario.valid-setpoint-update-stays-within-bounds]]

- GIVEN valid setpoint bounds and a target value inside those bounds
- WHEN the setpoint is updated
- THEN the new setpoint is accepted and remains inside the declared bounds

#### Scenario: Invalid setpoint update is rejected
r[r[domain-primitives.control-setpoint.scenario.invalid-setpoint-update-is-rejected]]

- GIVEN reversed setpoint bounds or a target value outside the declared bounds
- WHEN setpoint admission is checked
- THEN the update is rejected and the prior setpoint is unchanged

### Requirement: Control Error Signal
r[domain-primitives.control-error-signal]
The system MUST provide a control error-signal primitive for checked bounded
difference between target and measurement values.

#### Scenario: Valid error signal lies within envelope
r[r[domain-primitives.control-error-signal.scenario.valid-error-signal-lies-within-envelope]]

- GIVEN target and measurement values whose checked difference is representable and inside the declared error envelope
- WHEN error derivation is evaluated
- THEN the error signal is accepted and lies inside the envelope

#### Scenario: Overflowing error signal is rejected
r[r[domain-primitives.control-error-signal.scenario.overflowing-error-signal-is-rejected]]

- GIVEN target and measurement values whose difference overflows or leaves the declared error envelope
- WHEN error derivation is requested
- THEN the request is rejected

### Requirement: Control Actuator Saturation
r[domain-primitives.control-actuator-saturation]
The system MUST provide an actuator-saturation primitive for clamp bounds,
command limiting, and saturation flag semantics.

#### Scenario: Command is clamped inside actuator bounds
r[r[domain-primitives.control-actuator-saturation.scenario.command-is-clamped-inside-actuator-bounds]]

- GIVEN valid actuator bounds and an input command
- WHEN saturation is applied
- THEN the output command lies inside the actuator bounds and the saturation flag states whether clamping occurred

#### Scenario: Invalid actuator bounds are rejected
r[r[domain-primitives.control-actuator-saturation.scenario.invalid-actuator-bounds-are-rejected]]

- GIVEN reversed or otherwise malformed actuator bounds
- WHEN saturation is requested
- THEN the request is rejected instead of producing a command

### Requirement: Control Deadband
r[domain-primitives.control-deadband]
The system MUST provide a deadband primitive for deterministic no-action/action
decisions around a declared threshold interval.

#### Scenario: In-band error yields no-action decision
r[r[domain-primitives.control-deadband.scenario.in-band-error-yields-no-action-decision]]

- GIVEN a valid deadband threshold policy and an error value inside the no-action band
- WHEN the deadband decision is evaluated
- THEN the decision is no-action

#### Scenario: Invalid deadband is rejected
r[r[domain-primitives.control-deadband.scenario.invalid-deadband-is-rejected]]

- GIVEN negative width, reversed thresholds, or unsupported boundary semantics
- WHEN deadband admission is checked
- THEN the deadband request is rejected

### Requirement: Control Hysteresis
r[domain-primitives.control-hysteresis]
The system MUST provide a hysteresis primitive for two-threshold ordering,
finite-mode transitions, and hold-band state preservation.

#### Scenario: Hysteresis transitions only across declared thresholds
r[r[domain-primitives.control-hysteresis.scenario.hysteresis-transitions-only-across-declared-thresholds]]

- GIVEN valid low/high thresholds and a current finite mode
- WHEN an input value is evaluated
- THEN the next mode changes only when the input crosses the declared transition threshold and otherwise preserves the prior mode

#### Scenario: Invalid hysteresis thresholds are rejected
r[r[domain-primitives.control-hysteresis.scenario.invalid-hysteresis-thresholds-are-rejected]]

- GIVEN thresholds that are reversed, equal where separation is required, or outside declared input bounds
- WHEN hysteresis admission is checked
- THEN the request is rejected

### Requirement: Control Rate Limit
r[domain-primitives.control-rate-limit]
The system MUST provide a rate-limit primitive for bounded step-to-step changes,
accepted output envelopes, and invalid-rate rejection.

#### Scenario: Output step respects rate limit
r[r[domain-primitives.control-rate-limit.scenario.output-step-respects-rate-limit]]

- GIVEN a previous output, requested output, and valid maximum step delta
- WHEN rate limiting is applied
- THEN the accepted output differs from the previous output by no more than the maximum step delta and remains inside the output envelope

#### Scenario: Invalid or overflowing rate limit is rejected
r[r[domain-primitives.control-rate-limit.scenario.invalid-or-overflowing-rate-limit-is-rejected]]

- GIVEN a negative maximum step, malformed output envelope, or delta arithmetic that would overflow
- WHEN rate limiting is requested
- THEN the request is rejected

### Requirement: Control Safety Envelope
r[domain-primitives.control-safety-envelope]
The system MUST provide a safety-envelope primitive for conservative state/input
guard decisions over bounded discrete values.

#### Scenario: State and input inside envelope are accepted
r[r[domain-primitives.control-safety-envelope.scenario.state-and-input-inside-envelope-are-accepted]]

- GIVEN valid state bounds, input bounds, and state/input values inside those bounds
- WHEN the safety envelope guard is evaluated
- THEN the guard accepts the state/input pair

#### Scenario: Envelope violation is rejected without mutation
r[r[domain-primitives.control-safety-envelope.scenario.envelope-violation-is-rejected-without-mutation]]

- GIVEN a state or input outside the declared envelope, or malformed envelope bounds
- WHEN the safety envelope guard is evaluated
- THEN the guard rejects the request and the controlled state is unchanged

### Requirement: Schema Predicate Envelope
r[domain-primitives.data-validation.schema-predicate]
The system MUST provide finite schema predicate envelopes that declare admitted
field identifiers, value-shape predicates, and deterministic predicate IDs over
finite candidate records.

#### Scenario: Unknown schema predicate rejected explicitly
r[r[domain-primitives.data-validation.schema-predicate.scenario.unknown-schema-predicate-rejected-explicitly]]

- GIVEN a finite admitted schema predicate set
- WHEN a candidate predicate ID is absent from that set
- THEN the schema-predicate primitive returns an explicit unknown-predicate decision without invoking an external validator

### Requirement: Field Presence Envelope
r[domain-primitives.data-validation.field-presence]
The system MUST provide required/optional field-presence envelopes that check
finite candidate records against declared field metadata and expose explicit
missing, present, or duplicate-field decisions.

#### Scenario: Required field absence preserves input record
r[r[domain-primitives.data-validation.field-presence.scenario.required-field-absence-preserves-input-record]]

- GIVEN a finite candidate record and required field metadata
- WHEN a required field is absent
- THEN the field-presence primitive returns a missing-field decision without mutating the candidate record

### Requirement: Enum Constraint Envelope
r[domain-primitives.data-validation.enum-constraint]
The system MUST provide finite enum-constraint envelopes that admit only values
contained in a declared finite member set and expose explicit out-of-enum
rejection decisions.

#### Scenario: Enum candidate must be declared
r[r[domain-primitives.data-validation.enum-constraint.scenario.enum-candidate-must-be-declared]]

- GIVEN a finite enum member set and candidate value
- WHEN the candidate value is not a member of the declared set
- THEN the enum-constraint primitive rejects the value with explicit out-of-enum metadata

### Requirement: Range Constraint Envelope
r[domain-primitives.data-validation.range-constraint]
The system MUST provide bounded range-constraint envelopes over admitted scalar
values, inclusive/exclusive bound metadata, and deterministic out-of-range
rejection decisions.

#### Scenario: Range bounds are checked before scalar use
r[r[domain-primitives.data-validation.range-constraint.scenario.range-bounds-are-checked-before-scalar-use]]

- GIVEN finite scalar metadata and declared lower/upper range bounds
- WHEN the candidate scalar lies outside the admitted bound envelope
- THEN the range-constraint primitive rejects the scalar before any downstream validation composition uses it

### Requirement: Cross-Field Predicate Envelope
r[domain-primitives.data-validation.cross-field]
The system MUST provide finite cross-field predicate envelopes that relate a
bounded set of admitted fields and return explicit satisfied, unsatisfied, or
inapplicable decisions.

#### Scenario: Cross-field predicate rejects missing dependency
r[r[domain-primitives.data-validation.cross-field.scenario.cross-field-predicate-rejects-missing-dependency]]

- GIVEN a cross-field predicate requiring two admitted fields
- WHEN either required field is absent from the candidate record
- THEN the cross-field primitive returns an explicit inapplicable or rejected decision rather than claiming business-rule completeness

### Requirement: Error Accumulation Envelope
r[domain-primitives.data-validation.error-accumulation]
The system MUST provide deterministic error-accumulation envelopes that collect
finite validation error codes, field references, and stable ordering metadata.

#### Scenario: Error accumulation order is deterministic
r[r[domain-primitives.data-validation.error-accumulation.scenario.error-accumulation-order-is-deterministic]]

- GIVEN a finite set or sequence of validation decisions
- WHEN multiple decisions reject
- THEN the error-accumulation primitive records the resulting error metadata in a deterministic order declared by the envelope

### Requirement: Normalization Decision Envelope
r[domain-primitives.data-validation.normalization-decision]
The system MUST provide normalization decision envelopes that record accepted,
rejected, unchanged, or transformed shape metadata for deterministic finite
normalization steps without claiming semantic repair correctness.

#### Scenario: Normalization rejection keeps original value metadata
r[r[domain-primitives.data-validation.normalization-decision.scenario.normalization-rejection-keeps-original-value-metadata]]

- GIVEN a finite value envelope and deterministic normalization metadata
- WHEN normalization is not admitted
- THEN the normalization-decision primitive records rejection metadata while preserving the original value envelope

### Requirement: Validation Admission Envelope
r[domain-primitives.data-validation.admission]
The system MUST provide deterministic validation admission/rejection envelopes
that compose schema predicate, field presence, enum, range, cross-field, error
accumulation, and normalization decisions over finite candidate records.

#### Scenario: Admission requires all finite validation checks
r[r[domain-primitives.data-validation.admission.scenario.admission-requires-all-finite-validation-checks]]

- GIVEN finite validation metadata and a candidate record
- WHEN any schema predicate, field-presence, enum, range, cross-field, error, or normalization check rejects
- THEN the validation-admission primitive returns an explicit rejection decision before any external-validator or business-rule claim is made

### Requirement: Finite Objective Envelope
r[domain-primitives.discrete-optimization.objective]
The system MUST define objective metadata over a finite candidate universe and
exact discrete score representation.

#### Scenario: Objective references admitted candidate
r[r[domain-primitives.discrete-optimization.objective.scenario.references-admitted-candidate]]

- GIVEN objective metadata for finite candidate IDs and exact scores
- WHEN a score is associated with a candidate
- THEN the candidate is in the declared finite candidate universe

### Requirement: Finite Feasible Set Envelope
r[domain-primitives.discrete-optimization.feasible-set]
The system MUST define finite feasibility predicates that admit or reject
candidate IDs with deterministic metadata.

#### Scenario: Rejects absent candidate
r[r[domain-primitives.discrete-optimization.feasible-set.scenario.rejects-absent-candidate]]

- GIVEN a finite feasible-set envelope
- WHEN a candidate ID is outside the declared candidate universe
- THEN the candidate is rejected deterministically

### Requirement: Candidate Ranking Envelope
r[domain-primitives.discrete-optimization.ranking]
The system MUST define deterministic candidate-ranking envelopes over admitted
candidates and exact comparison metadata.

#### Scenario: Ranking compares only admitted candidates
r[r[domain-primitives.discrete-optimization.ranking.scenario.compares-only-admitted-candidates]]

- GIVEN a deterministic ranking envelope
- WHEN two candidates are compared
- THEN both candidates are admitted by the declared feasible-set envelope

### Requirement: Dominance Predicate
r[domain-primitives.discrete-optimization.dominance]
The system MUST define pairwise dominance predicates over exact finite candidate
attributes or objective tuples.

#### Scenario: Dominance result names compared candidates
r[r[domain-primitives.discrete-optimization.dominance.scenario.result-names-compared-candidates]]

- GIVEN a dominance predicate over finite candidates
- WHEN it returns a dominance decision
- THEN the decision metadata identifies the admitted candidates and exact attributes used

### Requirement: Objective Bound Metadata
r[domain-primitives.discrete-optimization.bound]
The system MUST define objective-bound metadata with explicit lower/upper bound
direction and validity predicates.

#### Scenario: Bound states direction and scope
r[r[domain-primitives.discrete-optimization.bound.scenario.states-direction-and-scope]]

- GIVEN objective-bound metadata
- WHEN it is attached to an optimization envelope
- THEN it states the bound direction, objective scope, and finite candidate universe it applies to

### Requirement: Optimum Witness Envelope
r[domain-primitives.discrete-optimization.witness]
The system MUST define optimum witness metadata for an admitted candidate that is
best only relative to a declared finite feasible set and exact objective model.

#### Scenario: Witness carries finite-scope bestness
r[r[domain-primitives.discrete-optimization.witness.scenario.carries-finite-scope-bestness]]

- GIVEN an optimum witness envelope
- WHEN it claims a candidate is best
- THEN the witness identifies the declared finite feasible set, exact objective model, and admitted candidate

### Requirement: Frontier Membership Envelope
r[domain-primitives.discrete-optimization.frontier]
The system MUST define Pareto/frontier membership metadata over finite candidates
and exact dominance predicates.

#### Scenario: Frontier member is nondominated in declared set
r[r[domain-primitives.discrete-optimization.frontier.scenario.member-is-nondominated-in-declared-set]]

- GIVEN a frontier membership envelope
- WHEN it marks a candidate as a frontier member
- THEN the candidate is admitted and the dominance predicate scope is the declared finite candidate set

### Requirement: Optimization Admission Envelope
r[domain-primitives.discrete-optimization.admission]
The system MUST define deterministic admission results for optimization problem
metadata, including invalid objective, candidate, bound, witness, and frontier
shapes.

#### Scenario: Rejects malformed witness deterministically
r[r[domain-primitives.discrete-optimization.admission.scenario.rejects-malformed-witness-deterministically]]

- GIVEN optimization problem metadata with a witness for a candidate outside the declared feasible set
- WHEN admission is evaluated
- THEN the metadata is rejected deterministically without claiming external solver correctness

### Requirement: Path Segment Envelope
r[domain-primitives.filesystem-path.segment]
The system MUST provide finite path-segment envelopes that record segment text or
bytes, separator policy metadata, empty/current/parent segment classifiers, and
deterministic segment admission results.

#### Scenario: Rejected segment is explicit
r[r[domain-primitives.filesystem-path.segment.scenario.rejected-segment-is-explicit]]

- GIVEN finite path-segment admission metadata
- WHEN a candidate segment is empty, forbidden, or outside the declared segment envelope
- THEN the path-segment primitive returns an explicit rejected-segment decision without consulting an OS filesystem

### Requirement: Path Form Envelope
r[domain-primitives.filesystem-path.form]
The system MUST provide finite path-form envelopes for absolute, relative,
rooted, empty, and scoped path forms over declared segment sequences.

#### Scenario: Path form is admitted before containment
r[r[domain-primitives.filesystem-path.form.scenario.path-form-is-admitted-before-containment]]

- GIVEN finite path form metadata and a candidate segment sequence
- WHEN the candidate form is not admitted by the envelope
- THEN the path-form primitive rejects it before containment or lookup decisions are composed

### Requirement: Path Normalization Decision
r[domain-primitives.filesystem-path.normalization]
The system MUST provide deterministic path-normalization decision envelopes that
record accepted, rejected, unchanged, or rewritten finite segment sequences under
declared normalization rules.

#### Scenario: Normalization rejection preserves original segments
r[r[domain-primitives.filesystem-path.normalization.scenario.normalization-rejection-preserves-original-segments]]

- GIVEN a finite candidate path and normalization metadata
- WHEN normalization is not admitted
- THEN the normalization primitive records rejection metadata while preserving the original segment sequence

### Requirement: Root Scope Containment Predicate
r[domain-primitives.filesystem-path.containment]
The system MUST provide finite root/scope containment predicates over declared
root segment prefixes and candidate normalized segment sequences.

#### Scenario: Candidate outside root is rejected
r[r[domain-primitives.filesystem-path.containment.scenario.candidate-outside-root-is-rejected]]

- GIVEN a finite root scope and candidate normalized path segments
- WHEN the candidate does not have the declared root/scope prefix
- THEN the containment primitive returns an explicit outside-scope decision without claiming sandbox security

### Requirement: Relative Traversal Rejection
r[domain-primitives.filesystem-path.traversal]
The system MUST provide deterministic relative traversal rejection metadata for
finite path segments that represent parent/current traversal under declared
segment classifier rules.

#### Scenario: Parent traversal is rejected by policy
r[r[domain-primitives.filesystem-path.traversal.scenario.parent-traversal-is-rejected-by-policy]]

- GIVEN finite path segments and a policy rejecting parent traversal segments
- WHEN a parent traversal segment appears before normalization or admission completes
- THEN the traversal primitive returns explicit rejection metadata without claiming exploit prevention beyond the declared finite predicate

### Requirement: Extension Type Metadata
r[domain-primitives.filesystem-path.extension-type]
The system MUST provide finite extension/type metadata envelopes that classify
admitted final segments by declared suffix, extension, or type tags without
claiming MIME, content-sniffing, or OS file-type correctness.

#### Scenario: Extension metadata does not imply file contents
r[r[domain-primitives.filesystem-path.extension-type.scenario.extension-metadata-does-not-imply-file-contents]]

- GIVEN a finite path with admitted extension metadata
- WHEN the extension/type primitive classifies the final segment
- THEN the result records only declared segment metadata and does not claim runtime file contents or MIME correctness

### Requirement: Path Join Split Envelope
r[domain-primitives.filesystem-path.join-split]
The system MUST provide deterministic path join/split shape envelopes over
finite segment sequences and separator metadata.

#### Scenario: Join split preserves admitted segment order
r[r[domain-primitives.filesystem-path.join-split.scenario.join-split-preserves-admitted-segment-order]]

- GIVEN finite admitted segment sequences and separator metadata
- WHEN a path join or split shape operation is modeled by the primitive
- THEN the resulting envelope records deterministic segment order without consulting host path APIs

### Requirement: Path Admission Envelope
r[domain-primitives.filesystem-path.admission]
The system MUST provide deterministic path admission/rejection envelopes that
compose segment, form, normalization, containment, traversal, extension/type, and
join/split decisions over finite path metadata.

#### Scenario: Admission requires all finite path checks
r[r[domain-primitives.filesystem-path.admission.scenario.admission-requires-all-finite-path-checks]]

- GIVEN finite path metadata and a candidate path
- WHEN any segment, form, normalization, containment, traversal, extension/type, or join/split check rejects
- THEN the path-admission primitive returns an explicit rejection decision before any OS filesystem or security claim is made

### Requirement: Point Vector Envelope
r[domain-primitives.geometry-foundation.point-vector]
The system MUST provide finite point/vector envelopes that record coordinate
count, coordinate bounds, scale metadata, dimension tags, and deterministic
point/vector admission results.

#### Scenario: Dimension mismatch is rejected explicitly
r[r[domain-primitives.geometry-foundation.point-vector.scenario.dimension-mismatch-is-rejected-explicitly]]

- GIVEN finite point/vector metadata and a declared dimension
- WHEN a candidate point/vector has a different coordinate count
- THEN the point/vector primitive returns explicit dimension-mismatch rejection metadata

### Requirement: Axis-Aligned Bounds Envelope
r[domain-primitives.geometry-foundation.bounds]
The system MUST provide finite axis-aligned bounds and extent envelopes that
record lower/upper coordinate bounds, emptiness metadata, extent metadata, and
deterministic bounds admission results.

#### Scenario: Inverted bound is rejected
r[r[domain-primitives.geometry-foundation.bounds.scenario.inverted-bound-is-rejected]]

- GIVEN finite axis-aligned bounds metadata
- WHEN any lower coordinate is greater than its upper coordinate under the declared order
- THEN the bounds primitive returns explicit inverted-bound rejection metadata

### Requirement: Orientation Metadata
r[domain-primitives.geometry-foundation.orientation]
The system MUST provide finite orientation/ordering metadata for declared point
or vector tuples under exact finite arithmetic envelopes.

#### Scenario: Degenerate orientation is explicit
r[r[domain-primitives.geometry-foundation.orientation.scenario.degenerate-orientation-is-explicit]]

- GIVEN finite orientation metadata for a declared tuple
- WHEN the tuple is degenerate under the exact finite predicate
- THEN the orientation primitive records explicit degenerate metadata rather than inventing a strict orientation

### Requirement: Containment Predicate Envelope
r[domain-primitives.geometry-foundation.containment]
The system MUST provide finite containment predicate envelopes for point-in-bounds,
bounds-in-bounds, and candidate-within-extent checks over admitted finite
geometry metadata.

#### Scenario: Outside point is rejected by containment
r[r[domain-primitives.geometry-foundation.containment.scenario.outside-point-is-rejected-by-containment]]

- GIVEN finite point and bounds metadata
- WHEN the point is outside the admitted bounds under declared inclusive/exclusive edge policy
- THEN the containment primitive returns explicit outside metadata

### Requirement: Intersection Touch Metadata
r[domain-primitives.geometry-foundation.intersection]
The system MUST provide finite intersection/touch metadata envelopes for admitted
axis-aligned bounds and finite primitive shapes without claiming complete
computational-geometry behavior.

#### Scenario: Disjoint bounds produce no-intersection metadata
r[r[domain-primitives.geometry-foundation.intersection.scenario.disjoint-bounds-produce-no-intersection-metadata]]

- GIVEN two finite admitted axis-aligned bounds envelopes
- WHEN the bounds are disjoint under declared edge policy
- THEN the intersection primitive records explicit no-intersection metadata

### Requirement: Transform Shape Envelope
r[domain-primitives.geometry-foundation.transform]
The system MUST provide finite transform-shape envelopes for translation,
scaling, permutation, and affine-like metadata under declared exact arithmetic
bounds.

#### Scenario: Transform overflow is rejected
r[r[domain-primitives.geometry-foundation.transform.scenario.transform-overflow-is-rejected]]

- GIVEN finite transform metadata and coordinate bounds
- WHEN applying the declared transform would exceed the output envelope
- THEN the transform primitive returns explicit overflow rejection metadata

### Requirement: Grid Cell Envelope
r[domain-primitives.geometry-foundation.grid-cell]
The system MUST provide finite grid-cell and index envelopes that record origin,
cell size metadata, grid extents, cell coordinates, and deterministic grid-cell
admission results.

#### Scenario: Cell index must fit grid extent
r[r[domain-primitives.geometry-foundation.grid-cell.scenario.cell-index-must-fit-grid-extent]]

- GIVEN finite grid-cell metadata and declared grid extents
- WHEN a candidate cell index is outside the grid extent
- THEN the grid-cell primitive returns explicit out-of-grid rejection metadata

### Requirement: Geometry Admission Envelope
r[domain-primitives.geometry-foundation.admission]
The system MUST provide deterministic geometry admission/rejection envelopes that
compose point/vector, bounds, orientation, containment, intersection, transform,
and grid-cell metadata over finite geometry data.

#### Scenario: Admission requires all finite geometry checks
r[r[domain-primitives.geometry-foundation.admission.scenario.admission-requires-all-finite-geometry-checks]]

- GIVEN finite geometry metadata and a candidate geometry envelope
- WHEN any point/vector, bounds, orientation, containment, intersection, transform, or grid-cell check rejects
- THEN the geometry-admission primitive returns an explicit rejection before any floating-point, topology, rendering, physics, or external geometry claim is made

### Requirement: Graph Adjacency
r[domain-primitives.graph-adjacency]
The system MUST provide a finite graph-adjacency primitive for node-count
validity, edge endpoint admission, and adjacency lookup.

#### Scenario: Valid edge is admitted into adjacency
r[r[domain-primitives.graph-adjacency.scenario.valid-edge-is-admitted-into-adjacency]]

- GIVEN a finite node count and an edge whose source and destination are valid node indices
- WHEN adjacency is constructed or queried
- THEN the edge is accepted and appears only in the source node adjacency relation

#### Scenario: Malformed edge is rejected
r[r[domain-primitives.graph-adjacency.scenario.malformed-edge-is-rejected]]

- GIVEN an edge with a source or destination outside the finite node range
- WHEN graph adjacency admission is checked
- THEN the edge is rejected and the graph state is unchanged

### Requirement: Graph Reachability
r[domain-primitives.graph-reachability]
The system MUST provide a finite graph-reachability primitive for path-witness
validity, reachability extension, and explicit unreachable results.

#### Scenario: Valid path witness proves reachability
r[r[domain-primitives.graph-reachability.scenario.valid-path-witness-proves-reachability]]

- GIVEN a valid finite graph and a path witness whose adjacent nodes follow admitted edges
- WHEN reachability for the path endpoints is checked
- THEN the destination is reachable from the source

#### Scenario: Invalid or absent path yields explicit non-reachability result
r[r[domain-primitives.graph-reachability.scenario.invalid-or-absent-path-yields-explicit-non-reachability-result]]

- GIVEN invalid endpoints, a malformed path witness, or a bounded search that finds no path
- WHEN reachability is checked
- THEN the primitive rejects the request or returns an explicit unreachable/empty result

### Requirement: Graph Topological Order
r[domain-primitives.graph-topological-order]
The system MUST provide a graph topological-order primitive for DAG admission,
permutation validity, and edge-forward ordering.

#### Scenario: Valid topological order places every edge forward
r[r[domain-primitives.graph-topological-order.scenario.valid-topological-order-places-every-edge-forward]]

- GIVEN a finite directed graph and an order containing every node exactly once
- WHEN topological-order validity is checked
- THEN every admitted edge points from an earlier node to a later node in the order

#### Scenario: Cycle or malformed order is rejected
r[r[domain-primitives.graph-topological-order.scenario.cycle-or-malformed-order-is-rejected]]

- GIVEN a graph with a directed cycle, duplicate order entries, missing nodes, or out-of-range nodes
- WHEN topological-order admission is checked
- THEN the order is rejected

### Requirement: Graph Component Partition
r[domain-primitives.graph-component-partition]
The system MUST provide an undirected component-partition primitive for complete,
disjoint grouping of finite graph nodes by connectivity.

#### Scenario: Partition covers each node exactly once
r[r[domain-primitives.graph-component-partition.scenario.partition-covers-each-node-exactly-once]]

- GIVEN a valid finite undirected graph and a component partition
- WHEN partition validity is checked
- THEN every node appears in exactly one component and all nodes inside a component are connected

#### Scenario: Invalid component partition is rejected
r[r[domain-primitives.graph-component-partition.scenario.invalid-component-partition-is-rejected]]

- GIVEN a partition with missing nodes, duplicate nodes, out-of-range nodes, or a component containing disconnected nodes
- WHEN component-partition admission is checked
- THEN the partition is rejected

### Requirement: Graph Shortest Path Envelope
r[domain-primitives.graph-shortest-path-envelope]
The system MUST provide a bounded shortest-path-envelope primitive for
non-negative weights, path-cost accumulation, and candidate-cost comparison.

#### Scenario: Candidate path cost is bounded and comparable
r[r[domain-primitives.graph-shortest-path-envelope.scenario.candidate-path-cost-is-bounded-and-comparable]]

- GIVEN a valid finite weighted graph with non-negative bounded edge weights and a valid path witness
- WHEN the path cost is accumulated
- THEN the resulting cost is accepted, bounded, and comparable against other accepted candidate costs

#### Scenario: Invalid or overflowing path cost is rejected
r[r[domain-primitives.graph-shortest-path-envelope.scenario.invalid-or-overflowing-path-cost-is-rejected]]

- GIVEN a negative weight, malformed edge, invalid path witness, or path whose accumulated cost would overflow the bound
- WHEN path-cost admission is checked
- THEN the cost request is rejected

### Requirement: Graph Frontier Cut
r[domain-primitives.graph-frontier-cut]
The system MUST provide a graph frontier/cut primitive for traversal state,
visited/frontier disjointness, and expansion preservation.

#### Scenario: Valid expansion preserves visited frontier cut invariant
r[r[domain-primitives.graph-frontier-cut.scenario.valid-expansion-preserves-visited-frontier-cut-invariant]]

- GIVEN a valid traversal state with disjoint visited and frontier sets
- WHEN one valid frontier node is expanded through admitted outgoing edges
- THEN the resulting visited and frontier sets remain within the node range and preserve the declared cut invariant

#### Scenario: Invalid frontier expansion is rejected without mutation
r[r[domain-primitives.graph-frontier-cut.scenario.invalid-frontier-expansion-is-rejected-without-mutation]]

- GIVEN a traversal state with an out-of-range frontier node, overlapping visited/frontier sets, or malformed outgoing edge
- WHEN expansion is requested
- THEN the operation is rejected and the traversal state is unchanged

### Requirement: Identifier Envelope
r[domain-primitives.identifier-namespace.identifier]
The system MUST provide finite identifier envelopes that record identifier bytes
or symbols, declared kind metadata, and deterministic ID-shape admission results.

#### Scenario: Malformed identifier rejected explicitly
r[r[domain-primitives.identifier-namespace.identifier.scenario.malformed-identifier-rejected-explicitly]]

- GIVEN finite identifier shape metadata
- WHEN a candidate identifier does not match the admitted shape
- THEN the identifier primitive returns an explicit malformed-ID decision without consulting an external identity service

### Requirement: Namespace Scope Envelope
r[domain-primitives.identifier-namespace.namespace]
The system MUST provide finite namespace scope envelopes that define admitted
scope identifiers, parent/child scope metadata, and deterministic namespace
membership decisions.

#### Scenario: Unknown namespace rejected before lookup
r[r[domain-primitives.identifier-namespace.namespace.scenario.unknown-namespace-rejected-before-lookup]]

- GIVEN a finite admitted namespace set
- WHEN a candidate namespace is absent from that set
- THEN the namespace primitive rejects the namespace before any lookup or registration decision is made

### Requirement: Scoped Uniqueness Predicate
r[domain-primitives.identifier-namespace.scoped-uniqueness]
The system MUST provide scoped uniqueness predicates over finite registry
snapshots and declared namespace scopes, returning explicit unique or duplicate
decisions.

#### Scenario: Duplicate only applies within declared scope
r[r[domain-primitives.identifier-namespace.scoped-uniqueness.scenario.duplicate-only-applies-within-declared-scope]]

- GIVEN two finite registry entries and a declared namespace scope
- WHEN the entries share an identifier inside that same scope
- THEN the scoped-uniqueness primitive returns a duplicate decision for that scope without claiming global uniqueness

### Requirement: Alias Map Envelope
r[domain-primitives.identifier-namespace.alias-map]
The system MUST provide finite alias-map envelopes that map admitted aliases to
admitted canonical identifiers and expose explicit missing-target or cycle
metadata.

#### Scenario: Alias target must be admitted
r[r[domain-primitives.identifier-namespace.alias-map.scenario.alias-target-must-be-admitted]]

- GIVEN finite alias metadata and admitted canonical identifiers
- WHEN an alias points to a target outside the admitted identifier set
- THEN the alias-map primitive rejects the alias or records explicit missing-target metadata

### Requirement: Canonicalization Decision Envelope
r[domain-primitives.identifier-namespace.canonicalization]
The system MUST provide canonicalization decision envelopes that record accepted,
rejected, unchanged, or mapped identifier forms without claiming semantic
equivalence beyond the declared finite map.

#### Scenario: Canonicalization decision preserves original identifier
r[r[domain-primitives.identifier-namespace.canonicalization.scenario.canonicalization-decision-preserves-original-identifier]]

- GIVEN a finite identifier and canonicalization metadata
- WHEN canonicalization maps or rejects the identifier
- THEN the canonicalization primitive records the original identifier and explicit decision metadata

### Requirement: Collision Metadata Envelope
r[domain-primitives.identifier-namespace.collision]
The system MUST provide deterministic collision metadata for finite registry
snapshots, candidate identifiers, and declared namespace scopes without claiming
hash collision resistance or randomness quality.

#### Scenario: Collision metadata is deterministic
r[r[domain-primitives.identifier-namespace.collision.scenario.collision-metadata-is-deterministic]]

- GIVEN the same finite registry snapshot, candidate identifier, and namespace scope
- WHEN collision metadata is computed by the primitive
- THEN the primitive returns the same collision or no-collision decision metadata

### Requirement: Lookup Result Envelope
r[domain-primitives.identifier-namespace.lookup]
The system MUST provide deterministic lookup result envelopes over finite
namespace scopes, admitted identifiers, aliases, and registry snapshots.

#### Scenario: Lookup distinguishes absent and rejected identifiers
r[r[domain-primitives.identifier-namespace.lookup.scenario.lookup-distinguishes-absent-and-rejected-identifiers]]

- GIVEN a finite registry snapshot and candidate identifier
- WHEN the identifier is malformed, outside the namespace, or absent from the registry
- THEN the lookup primitive returns distinct explicit decision metadata for the corresponding case

### Requirement: Registration Admission Envelope
r[domain-primitives.identifier-namespace.registration]
The system MUST provide deterministic registration admission/rejection envelopes
that compose identifier-shape, namespace, uniqueness, alias, canonicalization,
collision, and lookup decisions over finite registry snapshots.

#### Scenario: Registration requires all finite namespace checks
r[r[domain-primitives.identifier-namespace.registration.scenario.registration-requires-all-finite-namespace-checks]]

- GIVEN finite identifier, namespace, alias, canonicalization, and registry metadata
- WHEN any identifier-shape, namespace, uniqueness, alias, collision, or lookup check rejects
- THEN the registration primitive returns an explicit rejection decision before any distributed-identity or security claim is made

### Requirement: Endpoint Address Metadata
r[domain-primitives.network-protocol.endpoint-address]
The system MUST provide finite endpoint/address metadata envelopes that record
address-family tags, host/address identifiers, port/service metadata, scope tags,
and deterministic endpoint admission results without consulting DNS, sockets, or
network interfaces.

#### Scenario: Unknown endpoint family is rejected explicitly
r[r[domain-primitives.network-protocol.endpoint-address.scenario.unknown-endpoint-family-is-rejected-explicitly]]

- GIVEN finite endpoint/address metadata and a declared set of admitted address-family tags
- WHEN a candidate endpoint uses an unknown address-family tag
- THEN the endpoint primitive returns an explicit rejection without consulting DNS or the OS network stack

### Requirement: Packet Frame Envelope
r[domain-primitives.network-protocol.packet-frame]
The system MUST provide finite packet/frame envelopes that record byte-span
bounds, frame-kind tags, declared header ranges, payload ranges, and deterministic
frame admission results.

#### Scenario: Payload range must fit inside frame
r[r[domain-primitives.network-protocol.packet-frame.scenario.payload-range-must-fit-inside-frame]]

- GIVEN a finite packet/frame envelope with header and payload ranges
- WHEN the payload range is outside the declared frame bounds
- THEN the packet-frame primitive rejects the envelope before parser admission is composed

### Requirement: Header Field Metadata
r[domain-primitives.network-protocol.header-field]
The system MUST provide finite header-field metadata envelopes for field names or
IDs, field ranges, required/optional status, duplicate policy, and deterministic
header admission results.

#### Scenario: Missing required header is reported
r[r[domain-primitives.network-protocol.header-field.scenario.missing-required-header-is-reported]]

- GIVEN finite header-field metadata and required header definitions
- WHEN a required header field is absent
- THEN the header-field primitive returns explicit missing-header metadata

### Requirement: State-Free Parser Admission
r[domain-primitives.network-protocol.parser-admission]
The system MUST provide deterministic state-free parser admission envelopes over
one finite packet, frame, or message under declared endpoint, header, payload,
and checksum-shape metadata.

#### Scenario: Parser admission does not claim stream completeness
r[r[domain-primitives.network-protocol.parser-admission.scenario.parser-admission-does-not-claim-stream-completeness]]

- GIVEN one finite packet/frame/message envelope
- WHEN parser admission accepts the envelope under declared finite rules
- THEN the result admits only that envelope and does not claim streaming parser completeness, reassembly, delivery, or session correctness

### Requirement: Message Sequencing Metadata
r[domain-primitives.network-protocol.sequence]
The system MUST provide finite message sequencing metadata for declared sequence
numbers, acknowledgement-like fields, monotone/window bounds, duplicate markers,
and deterministic sequencing admission results.

#### Scenario: Sequence metadata does not imply delivery ordering
r[r[domain-primitives.network-protocol.sequence.scenario.sequence-metadata-does-not-imply-delivery-ordering]]

- GIVEN finite message sequencing metadata
- WHEN a candidate sequence field is admitted
- THEN the sequencing primitive records only declared metadata and does not claim transport delivery, ordering, replay protection, or liveness

### Requirement: Checksum Shape Metadata
r[domain-primitives.network-protocol.checksum-shape]
The system MUST provide finite checksum-shape metadata envelopes for checksum
field location, width, covered ranges, stored value shape, and deterministic
shape admission without claiming checksum strength or corruption detection.

#### Scenario: Checksum shape does not imply integrity
r[r[domain-primitives.network-protocol.checksum-shape.scenario.checksum-shape-does-not-imply-integrity]]

- GIVEN finite checksum-shape metadata and a candidate packet/frame
- WHEN the checksum field shape is admitted
- THEN the checksum-shape primitive records field-shape metadata only and does not claim integrity, authentication, or corruption detection

### Requirement: Payload Bound Metadata
r[domain-primitives.network-protocol.payload-bound]
The system MUST provide finite payload-bound metadata for payload byte ranges,
length-prefix agreement, payload-kind tags, empty-payload policy, and
deterministic payload admission results.

#### Scenario: Payload length mismatch is rejected
r[r[domain-primitives.network-protocol.payload-bound.scenario.payload-length-mismatch-is-rejected]]

- GIVEN finite payload-bound metadata and a declared payload length
- WHEN the payload range length does not match the declared payload length
- THEN the payload-bound primitive returns explicit length-mismatch rejection metadata

### Requirement: Protocol Admission Envelope
r[domain-primitives.network-protocol.admission]
The system MUST provide deterministic protocol admission/rejection envelopes that
compose endpoint/address metadata, packet/frame envelopes, header fields,
state-free parser admission, sequencing metadata, checksum-shape metadata, and
payload bounds over finite protocol metadata.

#### Scenario: Admission requires all finite protocol checks
r[r[domain-primitives.network-protocol.admission.scenario.admission-requires-all-finite-protocol-checks]]

- GIVEN finite protocol metadata and a candidate packet/frame/message
- WHEN any endpoint, frame, header, parser, sequence, checksum-shape, or payload-bound check rejects
- THEN the protocol-admission primitive returns an explicit rejection decision before any transport, security, delivery, or external protocol claim is made

### Requirement: Knowledge Graph Term Metadata
r[domain-primitives.knowledge-graph.term]
The system MUST provide finite knowledge-graph term metadata envelopes for entity
identifiers, relation identifiers, literal values, blank-node-like local
identifiers, labels, property keys, namespace scopes, and deterministic term
admission results.

#### Scenario: Unknown predicate term is rejected
r[r[domain-primitives.knowledge-graph.term.scenario.unknown-predicate-term-is-rejected]]

- GIVEN a finite term registry and a candidate fact referencing a predicate absent from that registry
- WHEN term admission is evaluated
- THEN the term primitive returns explicit unknown-predicate rejection metadata

### Requirement: RDF-Like Fact Triple Envelope
r[domain-primitives.knowledge-graph.triple]
The system MUST provide finite subject/predicate/object fact triple envelopes
that admit only declared subject terms, predicate terms, and object terms or
literal envelopes.

#### Scenario: Fact triple requires admitted endpoints
r[r[domain-primitives.knowledge-graph.triple.scenario.fact-triple-requires-admitted-endpoints]]

- GIVEN finite term metadata and a candidate subject/predicate/object fact
- WHEN the subject or object is outside the admitted term set
- THEN the triple primitive rejects the fact with endpoint metadata before any RDF semantics claim is made

### Requirement: Property Graph Metadata Envelope
r[domain-primitives.knowledge-graph.property]
The system MUST provide finite property-graph metadata envelopes for node labels,
edge labels, node properties, edge properties, property key admission, and
property value envelopes.

#### Scenario: Edge property key must be declared
r[r[domain-primitives.knowledge-graph.property.scenario.edge-property-key-must-be-declared]]

- GIVEN finite property metadata and a candidate edge property
- WHEN the property key is outside the declared property-key envelope
- THEN the property primitive rejects the edge property with unknown-property-key metadata

### Requirement: Ontology Constraint Envelope
r[domain-primitives.knowledge-graph.ontology]
The system MUST provide finite ontology/schema constraint envelopes for classes,
relations, domains, ranges, subtype edges, cardinality metadata, and deterministic
constraint admission results.

#### Scenario: Relation range violation is rejected
r[r[domain-primitives.knowledge-graph.ontology.scenario.relation-range-violation-is-rejected]]

- GIVEN finite ontology metadata declaring a relation range class
- WHEN a fact uses an object term outside the admitted range envelope
- THEN the ontology primitive rejects the fact with range-violation metadata

### Requirement: Provenance Metadata Envelope
r[domain-primitives.knowledge-graph.provenance]
The system MUST provide finite provenance metadata envelopes for source
identifiers, extraction method identifiers, confidence envelopes, timestamps or
version counters, derivation links, and deterministic provenance admission.

#### Scenario: Fact without required source is rejected
r[r[domain-primitives.knowledge-graph.provenance.scenario.fact-without-required-source-is-rejected]]

- GIVEN a graph admission policy requiring source provenance
- WHEN a candidate fact lacks admitted source metadata
- THEN the provenance primitive rejects the fact with missing-source metadata

### Requirement: Entity Alignment Envelope
r[domain-primitives.knowledge-graph.alignment]
The system MUST provide deterministic entity-alignment envelopes for aliases,
equivalence candidates, merge witnesses, canonical-term selection, conflict
metadata, and non-merge decisions.

#### Scenario: Conflicting merge witness prevents canonicalization
r[r[domain-primitives.knowledge-graph.alignment.scenario.conflicting-merge-witness-prevents-canonicalization]]

- GIVEN two entity terms with incompatible declared class or key metadata
- WHEN entity alignment is evaluated
- THEN the alignment primitive rejects the merge and returns explicit conflict metadata

### Requirement: Traversal and Query Shape Envelope
r[domain-primitives.knowledge-graph.query-shape]
The system MUST provide finite traversal and graph-pattern query-shape envelopes
for start terms, edge or predicate filters, path-length bounds, variable slots,
projection slots, and deterministic query-shape admission.

#### Scenario: Query shape with unbounded path is rejected
r[r[domain-primitives.knowledge-graph.query-shape.scenario.query-shape-with-unbounded-path-is-rejected]]

- GIVEN finite query-shape metadata requiring explicit traversal bounds
- WHEN a candidate graph pattern contains an unbounded path expression
- THEN the query-shape primitive rejects the pattern before any query-engine or SPARQL/Cypher claim is made

### Requirement: Bounded Inference Witness Envelope
r[domain-primitives.knowledge-graph.inference]
The system MUST provide bounded inference-witness envelopes for declared rules,
input fact identifiers, derived fact identifiers, closure-depth bounds, and
explicit inferred-fact admission metadata.

#### Scenario: Derived fact requires admitted rule witness
r[r[domain-primitives.knowledge-graph.inference.scenario.derived-fact-requires-admitted-rule-witness]]

- GIVEN finite rule metadata and candidate inferred fact metadata
- WHEN the inferred fact lacks an admitted rule witness or exceeds the declared closure-depth bound
- THEN the inference primitive rejects the fact with missing-or-overbound witness metadata

### Requirement: Embedding Index Metadata Envelope
r[domain-primitives.knowledge-graph.embedding-index]
The system MUST provide finite embedding-index metadata envelopes for embedding
identifiers, vector dimension bounds, indexed term or fact identifiers, distance
metric identifiers, version metadata, and deterministic index admission without
claiming vector-search quality.

#### Scenario: Embedding dimension mismatch is rejected
r[r[domain-primitives.knowledge-graph.embedding-index.scenario.embedding-dimension-mismatch-is-rejected]]

- GIVEN embedding-index metadata declaring a vector dimension
- WHEN a candidate embedding vector envelope has a different dimension
- THEN the embedding-index primitive rejects the vector with dimension-mismatch metadata

### Requirement: Knowledge Graph Admission Envelope
r[domain-primitives.knowledge-graph.admission]
The system MUST provide deterministic knowledge-graph admission/rejection
envelopes that compose term, triple, property, ontology, provenance, alignment,
query-shape, inference-witness, and embedding-index metadata over finite
knowledge-graph data.

#### Scenario: Admission requires all finite KG checks
r[r[domain-primitives.knowledge-graph.admission.scenario.admission-requires-all-finite-kg-checks]]

- GIVEN finite knowledge-graph metadata and a candidate fact, entity merge, query shape, inferred fact, or embedding record
- WHEN any term, triple, property, ontology, provenance, alignment, query-shape, inference, or embedding-index check rejects
- THEN the knowledge-graph-admission primitive returns explicit rejection before any graph database, standards-conformance, reasoning-completeness, LLM, RAG, or vector-search claim is made

### Requirement: Time-Series Timestamp Index
r[domain-primitives.timeseries-timestamp-index]
The system MUST provide a time-series timestamp-index primitive for finite
integer timestamps, monotone ordering, duplicate policy, and lookup/partition
bounds.

#### Scenario: Monotone timestamp lookup returns valid partition
r[r[domain-primitives.timeseries-timestamp-index.scenario.monotone-timestamp-lookup-returns-valid-partition]]

- GIVEN a finite timestamp index admitted under its monotone and duplicate policy
- WHEN lookup or bound search is evaluated for a query timestamp
- THEN the returned position is inside `0..=len` and partitions timestamps according to the query predicate

#### Scenario: Unsorted or invalid timestamp index is rejected
r[r[domain-primitives.timeseries-timestamp-index.scenario.unsorted-or-invalid-timestamp-index-is-rejected]]

- GIVEN timestamps that violate the monotone policy, duplicate policy, or declared timestamp bounds
- WHEN timestamp-index admission is checked
- THEN the index is rejected

### Requirement: Time-Series Sample Series
r[domain-primitives.timeseries-sample-series]
The system MUST provide a time-series sample-series primitive for aligned
timestamp/value sequences, length preservation, point updates, and invalid-sample
rejection.

#### Scenario: Valid point update preserves sample alignment
r[r[domain-primitives.timeseries-sample-series.scenario.valid-point-update-preserves-sample-alignment]]

- GIVEN a valid sample series and a valid sample index
- WHEN the value at that index is updated within the declared value bounds
- THEN timestamps remain unchanged, lengths remain aligned, and only the target value changes

#### Scenario: Malformed sample series is rejected
r[r[domain-primitives.timeseries-sample-series.scenario.malformed-sample-series-is-rejected]]

- GIVEN mismatched timestamp/value lengths, an invalid sample index, or a value outside declared bounds
- WHEN sample-series admission or update is requested
- THEN the request is rejected and the series state is unchanged

### Requirement: Time-Series Window
r[domain-primitives.timeseries-window]
The system MUST provide a time-series window primitive for declared boundary
semantics, selected-index bounds, and complete finite in-window coverage.

#### Scenario: Valid window selects exactly in-range samples
r[r[domain-primitives.timeseries-window.scenario.valid-window-selects-exactly-in-range-samples]]

- GIVEN a valid timestamp index and a valid declared window boundary policy
- WHEN window selection is evaluated
- THEN every selected sample satisfies the boundary policy and every sample satisfying the boundary policy is selected

#### Scenario: Invalid window is rejected
r[r[domain-primitives.timeseries-window.scenario.invalid-window-is-rejected]]

- GIVEN reversed window bounds, bounds outside the supported timestamp range, or an unsupported boundary policy
- WHEN window admission is checked
- THEN the window request is rejected

### Requirement: Time-Series Resample Envelope
r[domain-primitives.timeseries-resample-envelope]
The system MUST provide a time-series resample-envelope primitive for
deterministic bucket assignment over finite monotone timestamps.

#### Scenario: Sample maps to deterministic resample bucket
r[r[domain-primitives.timeseries-resample-envelope.scenario.sample-maps-to-deterministic-resample-bucket]]

- GIVEN valid monotone timestamps, a valid period, and declared bucket bounds
- WHEN resample bucket assignment is evaluated
- THEN each admitted sample maps to exactly one deterministic bucket inside the bucket range

#### Scenario: Invalid resample parameters are rejected
r[r[domain-primitives.timeseries-resample-envelope.scenario.invalid-resample-parameters-are-rejected]]

- GIVEN zero period, overflowing bucket arithmetic, reversed bucket bounds, or timestamps outside the declared resample range
- WHEN resample-envelope admission is checked
- THEN the request is rejected

### Requirement: Time-Series Rolling Aggregate
r[domain-primitives.timeseries-rolling-aggregate]
The system MUST provide a time-series rolling-aggregate primitive for bounded
count, sum, minimum, and maximum envelopes over finite windows.

#### Scenario: Rolling aggregate stays within accumulator bounds
r[r[domain-primitives.timeseries-rolling-aggregate.scenario.rolling-aggregate-stays-within-accumulator-bounds]]

- GIVEN a valid sample series, valid rolling windows, and representable aggregate results
- WHEN rolling aggregates are evaluated
- THEN each aggregate result corresponds to its selected window and stays within the declared accumulator bounds

#### Scenario: Rolling aggregate overflow is rejected
r[r[domain-primitives.timeseries-rolling-aggregate.scenario.rolling-aggregate-overflow-is-rejected]]

- GIVEN a rolling window whose count or accumulated value would exceed the declared accumulator bounds
- WHEN rolling aggregation is requested
- THEN the aggregate request is rejected

### Requirement: Time-Series Gap Detection
r[domain-primitives.timeseries-gap-detection]
The system MUST provide a time-series gap-detection primitive for finite monotone
indexes, maximum-gap thresholds, and missing-interval reporting.

#### Scenario: Gap detector reports intervals exceeding threshold
r[r[domain-primitives.timeseries-gap-detection.scenario.gap-detector-reports-intervals-exceeding-threshold]]

- GIVEN a valid monotone timestamp index and a positive maximum-gap threshold
- WHEN gap detection is evaluated
- THEN every reported gap has adjacent timestamps separated by more than the threshold and every such adjacent separation is reported

#### Scenario: Invalid gap detection request is rejected
r[r[domain-primitives.timeseries-gap-detection.scenario.invalid-gap-detection-request-is-rejected]]

- GIVEN an unsorted timestamp index, zero threshold, or overflowing timestamp difference
- WHEN gap detection is requested
- THEN the request is rejected

### Requirement: Byte Buffer Envelope
r[domain-primitives.serialization-codec.byte-buffer]
The system MUST provide a finite byte-buffer envelope that records byte length,
capacity bounds, and admitted byte ranges without claiming ownership of any
external buffer implementation.

#### Scenario: Buffer range must fit finite length
r[r[domain-primitives.serialization-codec.byte-buffer.scenario.buffer-range-must-fit-finite-length]]

- GIVEN a finite byte buffer envelope and a candidate half-open byte range
- WHEN the range end exceeds the admitted buffer length
- THEN the byte-buffer primitive rejects the range or requires a proven bound before use

### Requirement: Framed Record Envelope
r[domain-primitives.serialization-codec.frame]
The system MUST provide framed-record envelopes whose header, payload, and
optional trailer ranges are finite, nonnegative, and contained within the
admitted byte-buffer envelope.

#### Scenario: Frame sections must stay within buffer
r[r[domain-primitives.serialization-codec.frame.scenario.frame-sections-must-stay-within-buffer]]

- GIVEN a frame envelope with header, payload, and trailer metadata
- WHEN any section range falls outside the admitted buffer length
- THEN the frame primitive rejects the envelope or requires a proof that the section is in bounds

### Requirement: Length Prefix Envelope
r[domain-primitives.serialization-codec.length-prefix]
The system MUST provide checked length-prefix metadata that ties a declared
finite payload length to the remaining admitted byte window.

#### Scenario: Declared length must fit remaining payload
r[r[domain-primitives.serialization-codec.length-prefix.scenario.declared-length-must-fit-remaining-payload]]

- GIVEN a length prefix and a remaining finite payload window
- WHEN the declared length is greater than the remaining window length
- THEN the length-prefix primitive rejects the envelope or exposes an explicit invalid-length decision

### Requirement: Schema Version Admission
r[domain-primitives.serialization-codec.schema-version]
The system MUST provide deterministic schema/version admission metadata that
records finite schema identifiers, supported version ranges, and explicit
unknown-version rejection.

#### Scenario: Unknown version rejected deterministically
r[r[domain-primitives.serialization-codec.schema-version.scenario.unknown-version-rejected-deterministically]]

- GIVEN finite supported schema/version metadata
- WHEN a record declares an unknown schema or unsupported version
- THEN the schema-version primitive returns an explicit rejection decision without mutating the input envelope

### Requirement: Checksum Shape Metadata
r[domain-primitives.serialization-codec.checksum-shape]
The system MUST provide checksum shape metadata for algorithm identifiers,
checksum byte ranges, and covered byte ranges without claiming checksum
integrity, corruption detection, collision resistance, or cryptographic
security.

#### Scenario: Checksum field range must be structural only
r[r[domain-primitives.serialization-codec.checksum-shape.scenario.checksum-field-range-must-be-structural-only]]

- GIVEN checksum metadata for an admitted finite buffer
- WHEN the checksum field range or covered range is outside the buffer
- THEN the checksum-shape primitive rejects the metadata without claiming whether any checksum value is correct

### Requirement: Encoder Shape Predicate
r[domain-primitives.serialization-codec.encoder-shape]
The system MUST provide deterministic encoder shape predicates that relate an
admitted finite model value to finite output-buffer shape metadata.

#### Scenario: Encoder output must satisfy declared shape
r[r[domain-primitives.serialization-codec.encoder-shape.scenario.encoder-output-must-satisfy-declared-shape]]

- GIVEN an admitted finite model value and encoder shape predicate
- WHEN an encoder output length or section layout violates the declared shape
- THEN the encoder-shape primitive rejects the output metadata or requires a proof of shape preservation

### Requirement: Decoder Shape Predicate
r[domain-primitives.serialization-codec.decoder-shape]
The system MUST provide deterministic decoder shape predicates that accept or
reject finite byte buffers based on declared byte-level envelopes before any
format-specific semantic claim is made.

#### Scenario: Decoder rejects malformed finite buffer shape
r[r[domain-primitives.serialization-codec.decoder-shape.scenario.decoder-rejects-malformed-finite-buffer-shape]]

- GIVEN a finite byte buffer and decoder shape predicate
- WHEN required frame, length, schema, or section metadata is malformed
- THEN the decoder-shape primitive returns an explicit rejection decision without claiming external format conformance

### Requirement: Bounded Round-Trip Envelope
r[domain-primitives.serialization-codec.round-trip]
The system MUST provide bounded round-trip envelopes only for explicitly
admitted finite model values, schemas, versions, output buffers, and decoder
shape predicates.

#### Scenario: Round trip requires admitted finite model
r[r[domain-primitives.serialization-codec.round-trip.scenario.round-trip-requires-admitted-finite-model]]

- GIVEN a candidate model value and serialization/codec round-trip envelope
- WHEN the value, schema, version, or output shape is not admitted by the finite model
- THEN the round-trip primitive rejects the claim or requires the missing admission proof before asserting a bounded round trip

### Requirement: Grammar Symbol Admission
r[domain-primitives.parsing-format.grammar-symbol]
The system MUST define finite grammar-symbol admission for terminal and
nonterminal identifiers without claiming grammar completeness or recognition of
an external language.

#### Scenario: Symbol admission rejects malformed symbol metadata
r[r[domain-primitives.parsing-format.grammar-symbol.scenario.rejects-malformed-symbol-metadata]]

- GIVEN a grammar symbol with a kind tag and bounded identifier metadata
- WHEN the kind tag or identifier bounds are invalid
- THEN the checked admission predicate rejects the symbol without mutating caller-owned token or grammar state

### Requirement: Token Stream Shape
r[domain-primitives.parsing-format.token-stream]
The system MUST define finite token-stream shape checks for token kinds, source
spans, and monotone ordering over a finite token sequence.

#### Scenario: Token stream admits monotone non-overlapping spans
r[r[domain-primitives.parsing-format.token-stream.scenario.admits-monotone-nonoverlapping-spans]]

- GIVEN a finite token sequence with half-open source spans
- WHEN every token span is valid and ordered after the prior token span
- THEN the stream shape predicate admits the sequence as structurally well-formed

### Requirement: Source Span Predicate
r[domain-primitives.parsing-format.source-span]
The system MUST define half-open source span predicates for bounded start/end
offsets, containment, overlap, and disjointness over a finite source length.

#### Scenario: Source span rejects out-of-bounds ranges
r[r[domain-primitives.parsing-format.source-span.scenario.rejects-out-of-bounds-ranges]]

- GIVEN a finite source length and a candidate half-open span
- WHEN the start is greater than the end or the end exceeds the source length
- THEN the span predicate rejects the candidate

### Requirement: AST Shape Envelope
r[domain-primitives.parsing-format.ast-shape]
The system MUST define finite AST shape envelopes for node kind metadata, child
range bounds, arity limits, root admission, and acyclic child references.

#### Scenario: AST shape rejects dangling child ranges
r[r[domain-primitives.parsing-format.ast-shape.scenario.rejects-dangling-child-ranges]]

- GIVEN a finite AST node table and a node with a declared child range
- WHEN the child range exceeds the node table bounds or violates the arity limit
- THEN the AST shape envelope rejects the node table

### Requirement: Parse Error Envelope
r[domain-primitives.parsing-format.error-envelope]
The system MUST define deterministic parse-error metadata envelopes containing a
bounded source position, error kind, and optional expected-symbol metadata.

#### Scenario: Error envelope admits bounded diagnostic metadata
r[r[domain-primitives.parsing-format.error-envelope.scenario.admits-bounded-diagnostic-metadata]]

- GIVEN a finite source length and deterministic parser decision metadata
- WHEN the error position is within bounds and the error kind is admitted
- THEN the error envelope admits the diagnostic metadata without claiming user-facing message quality

### Requirement: Format Version Admission
r[domain-primitives.parsing-format.version-admission]
The system MUST define exact format-version admission for finite version tags,
accepted version sets, and closed-open supported version ranges.

#### Scenario: Version admission rejects unsupported versions
r[r[domain-primitives.parsing-format.version-admission.scenario.rejects-unsupported-versions]]

- GIVEN a finite accepted version set or supported version range
- WHEN an input version is outside the accepted metadata
- THEN the version admission predicate rejects the input before format-specific decoding proceeds

### Requirement: Serializer Shape Envelope
r[domain-primitives.parsing-format.serialize-shape]
The system MUST define serializer output-shape envelopes for finite emitted
segments, length bounds, ordering metadata, and deterministic failure admission.

#### Scenario: Serializer shape preserves declared segment order
r[r[domain-primitives.parsing-format.serialize-shape.scenario.preserves-declared-segment-order]]

- GIVEN a finite list of serializer output segments with declared offsets and lengths
- WHEN the segment metadata is admitted
- THEN the output-shape envelope states that segment offsets are monotone and within the declared output length

### Requirement: Deserializer Shape Envelope
r[domain-primitives.parsing-format.deserialize-shape]
The system MUST define deserializer input-shape and structural result envelopes
for finite fields, consumed ranges, remaining ranges, and deterministic
admission or rejection.

#### Scenario: Deserializer shape separates consumed and remaining input
r[r[domain-primitives.parsing-format.deserialize-shape.scenario.separates-consumed-and-remaining-input]]

- GIVEN a finite input and a structural deserializer result
- WHEN the result is admitted
- THEN the consumed range and remaining range are bounded, non-overlapping, and cover only admitted portions of the input

### Requirement: Bounded Round-Trip Envelope
r[domain-primitives.parsing-format.roundtrip-envelope]
The system MUST define bounded canonical round-trip predicates only for
explicitly admitted finite values, versions, and shape envelopes.

#### Scenario: Round-trip envelope stays canonical and bounded
r[r[domain-primitives.parsing-format.roundtrip-envelope.scenario.stays-canonical-and-bounded]]

- GIVEN an admitted finite value, an accepted format version, and a declared canonical serializer/deserializer envelope
- WHEN a later implementation claims round-trip preservation
- THEN the claim is scoped to the admitted canonical value set and does not imply arbitrary external-format compatibility

### Requirement: Statistics Count Table
r[domain-primitives.stats-count-table]
The system MUST provide a statistics count-table primitive for finite category
admission, bounded count updates, and overflow rejection.

#### Scenario: Valid category update increments one count
r[r[domain-primitives.stats-count-table.scenario.valid-category-update-increments-one-count]]

- GIVEN a valid finite count table and an admitted category
- WHEN the count table is updated for that category
- THEN exactly that category count increases and all other category counts are unchanged

#### Scenario: Invalid category or overflowing count is rejected
r[r[domain-primitives.stats-count-table.scenario.invalid-category-or-overflowing-count-is-rejected]]

- GIVEN an unknown category or a category count at its declared maximum
- WHEN a count update is requested
- THEN the update is rejected and the table state is unchanged

### Requirement: Statistics Frequency Table
r[domain-primitives.stats-frequency-table]
The system MUST provide a frequency-table primitive for denominator-safe
count-to-frequency derivation over finite non-negative counts.

#### Scenario: Non-zero total count yields frequencies
r[r[domain-primitives.stats-frequency-table.scenario.non-zero-total-count-yields-frequencies]]

- GIVEN finite non-negative counts whose total count is non-zero and representable
- WHEN frequencies are derived
- THEN each frequency is admitted under the declared ratio envelope and uses the total count as denominator

#### Scenario: Zero or overflowing total count is rejected
r[r[domain-primitives.stats-frequency-table.scenario.zero-or-overflowing-total-count-is-rejected]]

- GIVEN all-zero counts or counts whose total would overflow the declared accumulator
- WHEN frequency derivation is requested
- THEN the request is rejected or returns an explicit empty result instead of numeric frequencies

### Requirement: Statistics Discrete Distribution
r[domain-primitives.stats-discrete-distribution]
The system MUST provide a discrete-distribution primitive for finite
non-negative weights, positive total weight admission, and normalization
envelopes.

#### Scenario: Valid weights form admitted finite distribution
r[r[domain-primitives.stats-discrete-distribution.scenario.valid-weights-form-admitted-finite-distribution]]

- GIVEN finite non-negative weights with a positive representable total weight
- WHEN distribution admission is checked
- THEN the distribution is accepted and every normalized weight lies inside the declared probability envelope

#### Scenario: Invalid distribution weights are rejected
r[r[domain-primitives.stats-discrete-distribution.scenario.invalid-distribution-weights-are-rejected]]

- GIVEN a negative weight, all-zero weights, or weights whose total overflows the accumulator
- WHEN distribution admission is checked
- THEN the distribution is rejected

### Requirement: Statistics CDF Quantile
r[domain-primitives.stats-cdf-quantile]
The system MUST provide a CDF/quantile primitive for monotone cumulative weights,
partition lookup, and deterministic tie policy.

#### Scenario: Quantile lookup returns deterministic bucket
r[r[domain-primitives.stats-cdf-quantile.scenario.quantile-lookup-returns-deterministic-bucket]]

- GIVEN a valid monotone cumulative distribution and an admitted query threshold
- WHEN quantile lookup is evaluated
- THEN the lookup returns exactly one bucket according to the declared boundary and tie policy

#### Scenario: Invalid CDF or threshold is rejected
r[r[domain-primitives.stats-cdf-quantile.scenario.invalid-cdf-or-threshold-is-rejected]]

- GIVEN a non-monotone CDF, zero final cumulative weight, overflowing cumulative weight, or query threshold outside the admitted range
- WHEN CDF or quantile admission is checked
- THEN the request is rejected

### Requirement: Statistics Bounded Moments
r[domain-primitives.stats-bounded-moments]
The system MUST provide a bounded-moments primitive for finite count, sum, mean,
and variance-envelope derivation with checked accumulators.

#### Scenario: Valid sample set yields bounded moments
r[r[domain-primitives.stats-bounded-moments.scenario.valid-sample-set-yields-bounded-moments]]

- GIVEN finite bounded samples whose count and accumulators are representable
- WHEN moment envelopes are derived
- THEN count, sum, mean envelope, and variance envelope are accepted within declared bounds

#### Scenario: Moment derivation rejects empty or overflowing samples
r[r[domain-primitives.stats-bounded-moments.scenario.moment-derivation-rejects-empty-or-overflowing-samples]]

- GIVEN an empty sample set where a mean is required or samples whose accumulators overflow
- WHEN moment derivation is requested
- THEN the request is rejected or returns an explicit empty result instead of numeric moments

### Requirement: Statistics Deterministic Sampler
r[domain-primitives.stats-deterministic-sampler]
The system MUST provide a deterministic sampler-admission primitive for mapping
admitted integer thresholds or sample indices into finite distribution buckets.

#### Scenario: Admitted threshold maps to exactly one bucket
r[r[domain-primitives.stats-deterministic-sampler.scenario.admitted-threshold-maps-to-exactly-one-bucket]]

- GIVEN a valid finite cumulative distribution and an admitted deterministic threshold
- WHEN sample admission is evaluated
- THEN the threshold maps to exactly one bucket according to the declared cumulative partition

#### Scenario: Out-of-range deterministic sample is rejected
r[r[domain-primitives.stats-deterministic-sampler.scenario.out-of-range-deterministic-sample-is-rejected]]

- GIVEN a threshold outside the admitted range or a malformed cumulative distribution
- WHEN deterministic sample admission is checked
- THEN the sample request is rejected

### Requirement: Statistics Confidence Interval Envelope
r[domain-primitives.stats-confidence-interval-envelope]
The system MUST provide a confidence-interval-envelope primitive for ordered
interval endpoints and bounded confidence metadata without coverage-probability
claims.

#### Scenario: Valid interval metadata is admitted
r[r[domain-primitives.stats-confidence-interval-envelope.scenario.valid-interval-metadata-is-admitted]]

- GIVEN ordered interval endpoints and confidence metadata inside declared bounds
- WHEN confidence-interval envelope admission is checked
- THEN the interval metadata is accepted as a bounded interval record

#### Scenario: Malformed interval metadata is rejected
r[r[domain-primitives.stats-confidence-interval-envelope.scenario.malformed-interval-metadata-is-rejected]]

- GIVEN reversed endpoints, confidence metadata outside declared bounds, or non-representable endpoint arithmetic
- WHEN confidence-interval envelope admission is checked
- THEN the interval metadata is rejected

### Requirement: Resource Budget Envelope
r[domain-primitives.resource-accounting.budget]
The system MUST define finite resource-budget envelopes for unit metadata,
limits, consumed amounts, remaining amounts, and no-overflow arithmetic.

#### Scenario: Budget rejects consumption beyond limit
r[r[domain-primitives.resource-accounting.budget.scenario.rejects-consumption-beyond-limit]]

- GIVEN a finite budget with declared unit metadata and limit
- WHEN a candidate consumed amount exceeds the declared limit
- THEN the budget envelope rejects the state without claiming runtime enforcement

### Requirement: Resource Counter Predicate
r[domain-primitives.resource-accounting.counter]
The system MUST define bounded usage-counter predicates for finite counter
values, monotone increments, reset metadata, and maximum bounds.

#### Scenario: Counter increment stays within bound
r[r[domain-primitives.resource-accounting.counter.scenario.increment-stays-within-bound]]

- GIVEN a bounded counter value and a finite increment amount
- WHEN the increment result is inside the declared maximum bound
- THEN the counter predicate admits the increment and records deterministic usage metadata

### Requirement: Resource Quota Envelope
r[domain-primitives.resource-accounting.quota]
The system MUST define quota envelopes for finite subjects, scopes, limits,
current usage, and deterministic admission metadata.

#### Scenario: Quota admission rejects scoped overuse
r[r[domain-primitives.resource-accounting.quota.scenario.rejects-scoped-overuse]]

- GIVEN a finite quota for a subject and scope
- WHEN a candidate usage amount would exceed the quota limit for that subject and scope
- THEN the quota envelope rejects the candidate with deterministic overage metadata

### Requirement: Resource Lease Envelope
r[domain-primitives.resource-accounting.lease]
The system MUST define finite lease envelopes for holders, resource identifiers,
half-open validity intervals, renewal metadata, and expiry predicates.

#### Scenario: Lease expires outside validity interval
r[r[domain-primitives.resource-accounting.lease.scenario.expires-outside-validity-interval]]

- GIVEN a finite lease with a holder, resource identifier, and half-open validity interval
- WHEN a candidate bucket is outside the lease interval
- THEN the lease envelope treats the lease as expired for that bucket

### Requirement: Resource Usage Window
r[domain-primitives.resource-accounting.usage-window]
The system MUST define bounded usage-window predicates for finite aggregation
windows, admitted samples, sum/count metadata, and no-overflow envelopes.

#### Scenario: Usage window aggregates admitted samples
r[r[domain-primitives.resource-accounting.usage-window.scenario.aggregates-admitted-samples]]

- GIVEN a finite usage window and usage samples admitted by the window predicate
- WHEN the aggregate amount is computed in the declared bounded domain
- THEN the usage-window predicate admits only aggregate metadata that stays within the no-overflow envelope

### Requirement: Resource Reservation Envelope
r[domain-primitives.resource-accounting.reservation]
The system MUST define reservation envelopes for finite resource identifiers,
reserved amounts, capacity limits, and admitted reservation sets.

#### Scenario: Reservation rejects capacity overflow
r[r[domain-primitives.resource-accounting.reservation.scenario.rejects-capacity-overflow]]

- GIVEN a finite resource capacity and admitted reservations
- WHEN adding a candidate reservation would make the reserved amount exceed capacity
- THEN the reservation envelope rejects the candidate without claiming scheduling optimality

### Requirement: Resource Overage Classification
r[domain-primitives.resource-accounting.overage]
The system MUST define deterministic overage classifications for finite limits,
usage amounts, margins, and severity metadata.

#### Scenario: Overage classification records exact margin
r[r[domain-primitives.resource-accounting.overage.scenario.records-exact-margin]]

- GIVEN a finite limit and usage amount in the same unit domain
- WHEN usage exceeds the limit
- THEN the overage classification records the exact finite margin and deterministic severity metadata

### Requirement: Resource Admission Envelope
r[domain-primitives.resource-accounting.admission]
The system MUST define deterministic resource-admission envelopes that return
admit/reject metadata from finite budgets, quotas, leases, usage windows,
reservations, and overage predicates.

#### Scenario: Admission rejects exhausted budget deterministically
r[r[domain-primitives.resource-accounting.admission.scenario.rejects-exhausted-budget-deterministically]]

- GIVEN a finite accounting state and a candidate resource request
- WHEN the request would exceed an admitted budget, quota, lease, usage-window, or reservation predicate
- THEN the admission envelope rejects the request with deterministic metadata and does not claim fairness or runtime enforcement

### Requirement: Calendar Field Admission
r[domain-primitives.scheduling-calendar.calendar-field]
The system MUST define bounded calendar-field admission for finite field kinds,
minimum/maximum values, and closed or closed-open value ranges.

#### Scenario: Calendar field rejects out-of-range value
r[r[domain-primitives.scheduling-calendar.calendar-field.scenario.rejects-out-of-range-value]]

- GIVEN a finite calendar field with declared minimum and maximum bounds
- WHEN a candidate value falls outside the declared bounds
- THEN the field admission predicate rejects the value without claiming timezone or locale-calendar correctness

### Requirement: Time Bucket Metadata
r[domain-primitives.scheduling-calendar.time-bucket]
The system MUST define discrete time-bucket metadata for finite bucket indexes,
positive bucket durations, and bounded bucket ranges.

#### Scenario: Time bucket admits positive bounded duration
r[r[domain-primitives.scheduling-calendar.time-bucket.scenario.admits-positive-bounded-duration]]

- GIVEN a bucket index, bucket duration, and maximum bucket count
- WHEN the duration is positive and the index is inside the finite bucket range
- THEN the bucket metadata predicate admits the bucket

### Requirement: Schedule Interval Predicate
r[domain-primitives.scheduling-calendar.interval]
The system MUST define half-open schedule interval predicates for bounded start
and end buckets, containment, overlap, and disjointness.

#### Scenario: Interval conflict uses half-open overlap
r[r[domain-primitives.scheduling-calendar.interval.scenario.uses-half-open-overlap]]

- GIVEN two admitted half-open intervals over the same finite bucket domain
- WHEN the first interval end equals the second interval start
- THEN the interval predicate treats the intervals as non-overlapping

### Requirement: Recurrence Window Envelope
r[domain-primitives.scheduling-calendar.recurrence-window]
The system MUST define finite recurrence-window envelopes for candidate schedule
buckets, horizon bounds, stride metadata, and deterministic enumeration limits.

#### Scenario: Recurrence window stays within horizon
r[r[domain-primitives.scheduling-calendar.recurrence-window.scenario.stays-within-horizon]]

- GIVEN a recurrence start bucket, stride, maximum candidates, and finite horizon
- WHEN candidate buckets are admitted by the recurrence envelope
- THEN every candidate bucket is within the horizon and respects the declared stride metadata

### Requirement: Deadline Order Predicate
r[domain-primitives.scheduling-calendar.deadline-order]
The system MUST define deterministic deadline and priority ordering predicates
for finite jobs or reservations.

#### Scenario: Deadline order uses stable tie-break metadata
r[r[domain-primitives.scheduling-calendar.deadline-order.scenario.uses-stable-tie-break-metadata]]

- GIVEN two finite scheduling items with equal deadlines and priorities
- WHEN the ordering predicate compares them
- THEN the predicate uses declared stable tie-break metadata rather than nondeterministic runtime order

### Requirement: Schedule Conflict Predicate
r[domain-primitives.scheduling-calendar.conflict]
The system MUST define conflict predicates over admitted schedule intervals and
finite resource identifiers.

#### Scenario: Conflict requires shared resource and overlapping interval
r[r[domain-primitives.scheduling-calendar.conflict.scenario.requires-shared-resource-and-overlap]]

- GIVEN two admitted reservations with resource identifiers and half-open intervals
- WHEN the reservations use different resources or non-overlapping intervals
- THEN the conflict predicate reports no conflict

### Requirement: Resource Slot Envelope
r[domain-primitives.scheduling-calendar.resource-slot]
The system MUST define finite resource-slot envelopes for resource identifiers,
capacity bounds, reservation counts, and admitted occupancy.

#### Scenario: Resource slot rejects capacity overflow
r[r[domain-primitives.scheduling-calendar.resource-slot.scenario.rejects-capacity-overflow]]

- GIVEN a finite resource slot with declared capacity and admitted reservations
- WHEN the reservation count or declared occupancy exceeds capacity
- THEN the resource-slot envelope rejects the slot

### Requirement: Schedule Admission Envelope
r[domain-primitives.scheduling-calendar.admission]
The system MUST define deterministic schedule admission envelopes that return
admit/reject metadata from finite intervals, deadlines, priorities, resources,
and conflict predicates.

#### Scenario: Admission rejects conflicting reservation deterministically
r[r[domain-primitives.scheduling-calendar.admission.scenario.rejects-conflicting-reservation-deterministically]]

- GIVEN a finite schedule state and a candidate reservation
- WHEN the candidate conflicts with an admitted reservation under the declared resource and interval predicates
- THEN the admission envelope rejects the candidate with deterministic conflict metadata and does not claim optimal rescheduling

### Requirement: Finite Sample Sequence
r[signal.finite-sample-sequence]
The system MUST define a finite sample sequence primitive with explicit length,
channel/layout metadata, sample-domain bounds, and valid index admission.

#### Scenario: Checked sample sequence accepts bounded finite metadata
r[r[signal.finite-sample-sequence.scenario.checked-sequence-accepts-bounded-finite-metadata]]

- GIVEN finite length, channel count, layout metadata, and sample-domain bounds inside representable limits
- WHEN a checked sample sequence constructor or checker evaluates the metadata
- THEN it accepts the sequence metadata and exposes only indices within the finite length and channel layout

#### Scenario: Invalid sample metadata is rejected without mutation
r[r[signal.finite-sample-sequence.scenario.invalid-sample-metadata-rejected-without-mutation]]

- GIVEN a previously valid sample sequence value
- WHEN a public checked operation receives invalid length, channel, layout, or sample-domain metadata
- THEN it rejects the input explicitly and leaves the previously valid value unchanged

### Requirement: FIR Filter Envelope
r[signal.fir-filter-envelope]
The system MUST define a finite FIR filter envelope primitive for kernel length,
coefficient bounds, input sample bounds, and conservative accumulator/output
bounds over exact discrete values.

#### Scenario: FIR envelope bounds every admitted tap accumulation
r[r[signal.fir-filter-envelope.scenario.fir-envelope-bounds-every-admitted-tap-accumulation]]

- GIVEN an admitted finite kernel, coefficient bounds, sample bounds, and accumulator envelope
- WHEN a tap index is within the kernel length and an input sample is within the sample bounds
- THEN the primitive states that the tap contribution and accumulated envelope remain within the declared conservative bounds

#### Scenario: FIR envelope avoids floating-point frequency-response claims
r[r[signal.fir-filter-envelope.scenario.fir-envelope-avoids-floating-point-frequency-response-claims]]

- GIVEN a finite FIR filter envelope requirement or implementation task
- WHEN it states exported guarantees
- THEN it does not claim floating-point filter accuracy, frequency response, phase response, or runtime DSP-library correctness

### Requirement: Discrete Convolution Window
r[signal.convolution-window]
The system MUST define a discrete convolution window primitive that maps finite
input length, finite kernel length, padding/admission policy, and output index to
an exact support interval and conservative output envelope.

#### Scenario: Convolution window maps output index to finite support
r[r[signal.convolution-window.scenario.convolution-window-maps-output-index-to-finite-support]]

- GIVEN admitted finite input length, finite kernel length, padding policy, and output index
- WHEN the convolution window primitive evaluates the output index
- THEN it returns or specifies only support positions that are within admitted input/kernel bounds or explicitly marked as padded positions

#### Scenario: Invalid convolution dimensions are rejected
r[r[signal.convolution-window.scenario.invalid-convolution-dimensions-rejected]]

- GIVEN zero, overflowing, or otherwise invalid input/kernel/output dimensions
- WHEN a checked convolution-window operation evaluates those dimensions
- THEN it rejects the dimensions without constructing an invalid support mapping

### Requirement: Decimation Admission
r[signal.decimation-admission]
The system MUST define a decimation admission primitive for finite input length,
positive down-sampling factor, phase/offset, output length, and deterministic
output-to-input index mapping.

#### Scenario: Decimation maps each output index to one admitted input index
r[r[signal.decimation-admission.scenario.decimation-maps-each-output-index-to-one-admitted-input-index]]

- GIVEN finite input length, positive decimation factor, admitted phase, and output index within the computed output length
- WHEN the decimation primitive maps the output index
- THEN the mapped input index is within the input length and follows the declared factor/phase formula

#### Scenario: Invalid decimation factor or phase is rejected
r[r[signal.decimation-admission.scenario.invalid-decimation-factor-or-phase-rejected]]

- GIVEN a zero factor, overflowing length computation, or phase outside the admitted factor/input range
- WHEN a checked decimation operation evaluates the metadata
- THEN it rejects the metadata and does not expose an output index mapping

### Requirement: Interpolation Admission
r[signal.interpolation-admission]
The system MUST define an interpolation admission primitive for finite input
length, positive up-sampling factor, output length, gap-fill policy, and
deterministic output/input correspondence metadata.

#### Scenario: Interpolation classifies source-aligned and gap outputs
r[r[signal.interpolation-admission.scenario.interpolation-classifies-source-aligned-and-gap-outputs]]

- GIVEN finite input length, positive interpolation factor, admitted output length, and output index
- WHEN the interpolation primitive classifies the output index
- THEN it marks the output as either corresponding to an in-range source sample or to an admitted gap-fill position according to the declared factor policy

#### Scenario: Invalid interpolation metadata is rejected
r[r[signal.interpolation-admission.scenario.invalid-interpolation-metadata-rejected]]

- GIVEN a zero factor, overflowing output length, or gap-fill policy outside the supported finite model
- WHEN a checked interpolation operation evaluates the metadata
- THEN it rejects the metadata without exposing inconsistent output/input correspondence

### Requirement: Peak Candidate Detection
r[signal.peak-candidate]
The system MUST define a peak-candidate primitive over finite sequences using a
bounded neighborhood, threshold/envelope metadata, and deterministic tie handling.

#### Scenario: Peak candidate satisfies local finite-neighborhood predicate
r[r[signal.peak-candidate.scenario.peak-candidate-satisfies-local-finite-neighborhood-predicate]]

- GIVEN a finite sequence, candidate index, admitted neighborhood radius, threshold metadata, and tie policy
- WHEN the peak-candidate predicate accepts the candidate index
- THEN every compared neighbor is within the finite sequence and the candidate satisfies the declared threshold and ordering relation against those neighbors

#### Scenario: Peak tie policy is deterministic
r[r[signal.peak-candidate.scenario.peak-tie-policy-is-deterministic]]

- GIVEN equal-valued samples inside a peak neighborhood
- WHEN the peak-candidate primitive evaluates candidates under the declared tie policy
- THEN it admits candidates according to that deterministic policy rather than relying on iteration-order ambiguity

### Requirement: Spectral Bin Metadata
r[signal.spectral-bin-metadata]
The system MUST define spectral-bin metadata primitives for finite bin count,
bin index admission, range metadata, and bounded magnitude/energy envelopes over
exact discrete or fixed-point values.

#### Scenario: Spectral bin metadata admits only finite in-range bins
r[r[signal.spectral-bin-metadata.scenario.spectral-bin-metadata-admits-only-finite-in-range-bins]]

- GIVEN finite bin count, range metadata, and a candidate bin index
- WHEN the spectral-bin metadata primitive evaluates the bin index
- THEN it admits only indices within the finite bin count and associates them with the declared range metadata

#### Scenario: Spectral metadata avoids FFT numeric correctness claims
r[r[signal.spectral-bin-metadata.scenario.spectral-metadata-avoids-fft-numeric-correctness-claims]]

- GIVEN a spectral-bin metadata requirement or implementation task
- WHEN it states exported guarantees
- THEN it does not claim FFT algorithm correctness, floating-point spectral accuracy, leakage/noise behavior, or runtime DSP-library correctness

### Requirement: State Set Envelope
r[domain-primitives.state-machine-workflow.state]
The system MUST define finite state-set envelopes that admit declared states and
record initial and terminal-state metadata before transitions or traces reference
them.

#### Scenario: Initial state must be declared
r[r[domain-primitives.state-machine-workflow.state.scenario.initial-state-must-be-declared]]

- GIVEN a finite state-machine declaration with an initial state
- WHEN state-set admission is evaluated
- THEN the declaration is admitted only if the initial state belongs to the finite declared state set

### Requirement: Transition Relation Envelope
r[domain-primitives.state-machine-workflow.transition]
The system MUST define transition relation envelopes over finite source state,
event, target state, and optional guard metadata.

#### Scenario: Transition endpoints must be declared states
r[r[domain-primitives.state-machine-workflow.transition.scenario.endpoints-must-be-declared-states]]

- GIVEN a transition whose source or target state is absent from the declared state set
- WHEN transition admission is evaluated
- THEN the transition is rejected with endpoint metadata

### Requirement: Event Envelope
r[domain-primitives.state-machine-workflow.event]
The system MUST define finite event envelopes that admit declared event symbols
before transitions or workflow steps reference them.

#### Scenario: Unknown event is rejected
r[r[domain-primitives.state-machine-workflow.event.scenario.unknown-event-rejected]]

- GIVEN a transition referencing an event absent from the finite event set
- WHEN event admission is evaluated
- THEN the event reference is rejected with unknown-event metadata

### Requirement: Guard Predicate Envelope
r[domain-primitives.state-machine-workflow.guard]
The system MUST define deterministic guard predicate envelopes over admitted
transition context without claiming external side effects or runtime execution.

#### Scenario: Failed guard blocks transition admission
r[r[domain-primitives.state-machine-workflow.guard.scenario.failed-guard-blocks-transition-admission]]

- GIVEN an admitted source state, event, target state, and guard predicate metadata
- WHEN the guard predicate evaluates to false in the finite transition context
- THEN transition admission rejects the guarded transition with failed-guard metadata

### Requirement: Transition Admission Envelope
r[domain-primitives.state-machine-workflow.admission]
The system MUST define deterministic transition admission and rejection envelopes
for public exec surfaces that combine state, transition, event, and guard checks.

#### Scenario: Undeclared transition is rejected deterministically
r[r[domain-primitives.state-machine-workflow.admission.scenario.undeclared-transition-rejected-deterministically]]

- GIVEN a transition request that is absent from the finite transition relation
- WHEN transition admission is evaluated
- THEN the request is rejected with undeclared-transition metadata

### Requirement: Workflow Step Envelope
r[domain-primitives.state-machine-workflow.step]
The system MUST define finite workflow step envelopes that reference declared
states, events, and transition metadata without claiming runtime step execution.

#### Scenario: Workflow step references declared transition metadata
r[r[domain-primitives.state-machine-workflow.step.scenario.references-declared-transition-metadata]]

- GIVEN a workflow step with source state, event, and target state references
- WHEN workflow step admission is evaluated
- THEN the step is admitted only if those references match declared transition metadata

### Requirement: Workflow Dependency Envelope
r[domain-primitives.state-machine-workflow.dependency]
The system MUST define finite workflow dependency envelopes over declared steps
with deterministic dependency-shape admission metadata.

#### Scenario: Dependency references declared workflow steps
r[r[domain-primitives.state-machine-workflow.dependency.scenario.references-declared-workflow-steps]]

- GIVEN a workflow dependency edge between two step identifiers
- WHEN dependency admission is evaluated
- THEN the dependency is admitted only if both step identifiers are declared in the finite workflow step set

### Requirement: Reachability Path Envelope
r[domain-primitives.state-machine-workflow.reachability]
The system MUST define bounded reachability/path envelopes over finite admitted
transition systems and explicit path-witness metadata.

#### Scenario: Reachability witness follows admitted transitions
r[r[domain-primitives.state-machine-workflow.reachability.scenario.witness-follows-admitted-transitions]]

- GIVEN a finite path witness from one state to another
- WHEN reachability admission is evaluated
- THEN each adjacent state pair in the witness follows an admitted transition relation entry

### Requirement: Terminal State Predicate Envelope
r[domain-primitives.state-machine-workflow.terminal]
The system MUST define terminal-state predicate envelopes over finite declared
states without claiming liveness, eventual completion, or runtime scheduling.

#### Scenario: Terminal predicate only accepts declared terminal state
r[r[domain-primitives.state-machine-workflow.terminal.scenario.only-accepts-declared-terminal-state]]

- GIVEN a state candidate
- WHEN terminal-state predicate evaluation is performed
- THEN the predicate matches only if the candidate is declared as terminal in the finite state metadata

### Requirement: Transition Trace Envelope
r[domain-primitives.state-machine-workflow.trace]
The system MUST define finite transition trace envelopes whose consecutive trace
items are stepwise admitted by the transition relation and guard metadata.

#### Scenario: Trace rejects invalid step
r[r[domain-primitives.state-machine-workflow.trace.scenario.rejects-invalid-step]]

- GIVEN a finite transition trace containing one step absent from the admitted transition relation
- WHEN trace admission is evaluated
- THEN the trace is rejected with the invalid step index and transition metadata

### Requirement: Page Range
r[storage.page-range]
The system MUST define a finite page-range primitive with checked page
identifier bounds, span length metadata, and half-open page interval admission.

#### Scenario: Checked page range admits finite half-open span
r[r[storage.page-range.scenario.checked-page-range-admits-finite-half-open-span]]

- GIVEN finite page-count metadata, a start page, and an end page with start less than or equal to end
- WHEN a checked page-range constructor evaluates the span
- THEN it admits only page identifiers within the half-open interval and within the finite page count

#### Scenario: Invalid page range is rejected without mutation
r[r[storage.page-range.scenario.invalid-page-range-rejected-without-mutation]]

- GIVEN a previously valid page-range value
- WHEN a public checked operation receives overflowing, reversed, or out-of-bounds page metadata
- THEN it rejects the input explicitly and leaves the previously valid value unchanged

### Requirement: Record Slot Table
r[storage.record-slot-table]
The system MUST define a finite record-slot table primitive with slot-count
bounds, occupied/free state metadata, and valid slot-index admission.

#### Scenario: Slot table admits only in-range occupied slots
r[r[storage.record-slot-table.scenario.slot-table-admits-only-in-range-occupied-slots]]

- GIVEN finite slot-count metadata and occupied/free state for each admitted slot
- WHEN a slot lookup evaluates a candidate slot index
- THEN it admits occupied-slot access only for indices within the finite slot count whose state is occupied

#### Scenario: Free slot remains explicit state
r[r[storage.record-slot-table.scenario.free-slot-remains-explicit-state]]

- GIVEN a finite slot table with a free slot
- WHEN the slot-state primitive evaluates that slot
- THEN it exposes the free state explicitly rather than treating the slot as an occupied record

### Requirement: Key Range
r[storage.key-range]
The system MUST define an ordered finite key-range primitive with lower and upper
bounds, half-open membership, and overlap/disjointness metadata.

#### Scenario: Key range membership follows half-open bounds
r[r[storage.key-range.scenario.key-range-membership-follows-half-open-bounds]]

- GIVEN an admitted ordered key range with lower bound less than or equal to upper bound
- WHEN membership is evaluated for a candidate key
- THEN the key is admitted exactly when it is greater than or equal to the lower bound and strictly less than the upper bound

#### Scenario: Invalid key range is rejected
r[r[storage.key-range.scenario.invalid-key-range-rejected]]

- GIVEN unordered or non-representable key bounds
- WHEN a checked key-range operation evaluates the bounds
- THEN it rejects them without exposing inconsistent membership metadata

### Requirement: Cursor Bounds
r[storage.cursor-bounds]
The system MUST define a cursor-bounds primitive for finite scan ranges, current
cursor position, end state, and monotone cursor movement.

#### Scenario: Cursor movement stays within admitted scan range
r[r[storage.cursor-bounds.scenario.cursor-movement-stays-within-admitted-scan-range]]

- GIVEN an admitted finite scan range and cursor position within or at the end of that range
- WHEN the cursor advances under the declared movement policy
- THEN the resulting cursor is either within the same range at a later position or explicitly at the end state

#### Scenario: Cursor rejects movement from invalid position
r[r[storage.cursor-bounds.scenario.cursor-rejects-movement-from-invalid-position]]

- GIVEN a cursor position outside the admitted scan range and not equal to the declared end state
- WHEN a checked cursor operation evaluates the position
- THEN it rejects the operation without constructing an invalid next cursor

### Requirement: Segment Map
r[storage.segment-map]
The system MUST define a finite segment-map primitive from logical key/page
ranges to physical segment/page spans with explicit coverage and disjointness
metadata.

#### Scenario: Segment map associates logical range with finite physical span
r[r[storage.segment-map.scenario.segment-map-associates-logical-range-with-finite-physical-span]]

- GIVEN an admitted logical range and finite physical segment/page span
- WHEN the segment-map primitive evaluates the mapping
- THEN it exposes only physical positions within the declared segment/page bounds for that logical range

#### Scenario: Segment map rejects overlapping inconsistent spans
r[r[storage.segment-map.scenario.segment-map-rejects-overlapping-inconsistent-spans]]

- GIVEN segment-map metadata with overlapping logical ranges that map to inconsistent physical spans under the declared policy
- WHEN a checked segment-map constructor evaluates the metadata
- THEN it rejects the map instead of exposing ambiguous logical-to-physical lookup metadata

### Requirement: Lookup Result Admission
r[storage.lookup-result]
The system MUST define a lookup-result primitive that admits present, absent, and
out-of-range outcomes according to finite table/range metadata and deterministic
decision rules.

#### Scenario: Present lookup result references admitted record
r[r[storage.lookup-result.scenario.present-lookup-result-references-admitted-record]]

- GIVEN finite table/range metadata and a lookup result marked present
- WHEN the lookup-result primitive validates the result
- THEN the referenced key, slot, page, or record metadata is within the admitted finite bounds and matches the declared lookup key/range relation

#### Scenario: Absent and out-of-range results are distinguished
r[r[storage.lookup-result.scenario.absent-and-out-of-range-results-are-distinguished]]

- GIVEN a lookup key and admitted key-range metadata
- WHEN the lookup-result primitive validates an unsuccessful result
- THEN it distinguishes an in-range absent result from an out-of-range rejected result according to the declared bounds

### Requirement: Compaction Envelope
r[storage.compaction-envelope]
The system MUST define a finite compaction envelope primitive that preserves live
logical records and admitted key/slot membership while allowing physical page or
segment spans to be remapped.

#### Scenario: Compaction preserves live logical record identity
r[r[storage.compaction-envelope.scenario.compaction-preserves-live-logical-record-identity]]

- GIVEN pre-compaction and post-compaction finite metadata plus a declared live-record set
- WHEN the compaction envelope validates the remap
- THEN every live logical record admitted before compaction is admitted after compaction with the same logical identity and allowed physical span remapping

#### Scenario: Compaction may drop non-live records only by explicit policy
r[r[storage.compaction-envelope.scenario.compaction-may-drop-non-live-records-only-by-explicit-policy]]

- GIVEN a record not included in the declared live-record set
- WHEN the compaction envelope omits that record from post-compaction metadata
- THEN the omission is valid only when the declared compaction policy explicitly allows dropping non-live records

### Requirement: Versioned Slot
r[storage.versioned-slot]
The system MUST define a versioned-slot primitive over finite version metadata,
slot state, visibility bounds, and deterministic latest-visible selection.

#### Scenario: Latest-visible selection chooses maximal admitted version
r[r[storage.versioned-slot.scenario.latest-visible-selection-chooses-maximal-admitted-version]]

- GIVEN a finite set of versions for a slot and an admitted read/version bound
- WHEN the versioned-slot primitive selects a visible entry
- THEN the selected entry is visible under the bound and no other admitted visible entry has a greater version under the declared ordering

#### Scenario: Versioned slot rejects ambiguous latest entries
r[r[storage.versioned-slot.scenario.versioned-slot-rejects-ambiguous-latest-entries]]

- GIVEN two distinct entries that are both maximal under the declared version ordering without a deterministic tie policy
- WHEN a checked versioned-slot operation evaluates latest-visible selection
- THEN it rejects the metadata instead of exposing an ambiguous latest-visible result

### Requirement: Table Schema Metadata
r[domain-primitives.table-relational.table-schema]
The system MUST provide finite table/schema metadata envelopes that record table
identifiers, declared columns, row-count bounds, schema-version metadata, and
deterministic table admission results.

#### Scenario: Unknown table schema is rejected explicitly
r[r[domain-primitives.table-relational.table-schema.scenario.unknown-table-schema-is-rejected-explicitly]]

- GIVEN finite table/schema metadata and a declared schema registry envelope
- WHEN a candidate table references an unknown schema identifier
- THEN the table-schema primitive returns explicit rejection metadata without consulting a database engine

### Requirement: Column Metadata Envelope
r[domain-primitives.table-relational.column]
The system MUST provide finite column metadata envelopes for column identifiers,
declared value envelopes, nullability or presence policy, ordering position, and
deterministic column admission results.

#### Scenario: Column type mismatch is rejected before row admission
r[r[domain-primitives.table-relational.column.scenario.column-type-mismatch-is-rejected-before-row-admission]]

- GIVEN finite column metadata and a candidate cell value envelope
- WHEN the value envelope does not match the declared column envelope
- THEN the column primitive rejects the cell before row admission is composed

### Requirement: Row Shape Envelope
r[domain-primitives.table-relational.row]
The system MUST provide finite row shape envelopes that record row identifiers,
cell positions, required/optional cell presence, and deterministic row admission
results.

#### Scenario: Missing required cell is reported
r[r[domain-primitives.table-relational.row.scenario.missing-required-cell-is-reported]]

- GIVEN finite row shape metadata and required column positions
- WHEN a required cell is absent
- THEN the row primitive returns explicit missing-cell metadata

### Requirement: Key Metadata Envelope
r[domain-primitives.table-relational.key]
The system MUST provide finite key metadata envelopes for primary-key-like,
foreign-key-like, compound-key, and uniqueness predicates over declared row and
column metadata without claiming database enforcement.

#### Scenario: Duplicate finite key is rejected
r[r[domain-primitives.table-relational.key.scenario.duplicate-finite-key-is-rejected]]

- GIVEN finite key metadata and admitted row envelopes
- WHEN two rows violate a declared finite uniqueness predicate
- THEN the key primitive returns explicit duplicate-key metadata without claiming database uniqueness enforcement

### Requirement: Projection Filter Shape
r[domain-primitives.table-relational.projection-filter]
The system MUST provide deterministic projection/filter shape envelopes over
finite column sets, row predicates, and admitted table metadata.

#### Scenario: Projection references admitted columns only
r[r[domain-primitives.table-relational.projection-filter.scenario.projection-references-admitted-columns-only]]

- GIVEN finite table metadata and a projection envelope
- WHEN the projection references a column outside the admitted schema
- THEN the projection/filter primitive rejects the operation shape

### Requirement: Join Shape Metadata
r[domain-primitives.table-relational.join]
The system MUST provide finite join-shape metadata for declared input tables,
join-key metadata, cardinality bounds, unmatched-row policy, and deterministic
join-shape admission results.

#### Scenario: Join key must be present in both inputs
r[r[domain-primitives.table-relational.join.scenario.join-key-must-be-present-in-both-inputs]]

- GIVEN finite join metadata and two admitted table schemas
- WHEN a declared join key is absent from either input schema
- THEN the join primitive returns explicit join-key rejection metadata

### Requirement: Grouping Metadata Envelope
r[domain-primitives.table-relational.grouping]
The system MUST provide finite grouping/aggregate-shape metadata for group keys,
aggregate input columns, aggregate result metadata, and deterministic grouping
admission without claiming aggregate numeric accuracy beyond declared exact
models.

#### Scenario: Grouping references admitted columns only
r[r[domain-primitives.table-relational.grouping.scenario.grouping-references-admitted-columns-only]]

- GIVEN finite grouping metadata and an admitted table schema
- WHEN a group key or aggregate input references an unknown column
- THEN the grouping primitive returns explicit unknown-column rejection metadata

### Requirement: Relational Admission Envelope
r[domain-primitives.table-relational.admission]
The system MUST provide deterministic relational admission/rejection envelopes
that compose table/schema, column, row, key, projection/filter, join, and grouping
metadata over finite relational data.

#### Scenario: Admission requires all finite relational checks
r[r[domain-primitives.table-relational.admission.scenario.admission-requires-all-finite-relational-checks]]

- GIVEN finite relational metadata and candidate table or operation shape
- WHEN any schema, column, row, key, projection/filter, join, or grouping check rejects
- THEN the relational-admission primitive returns an explicit rejection before any database engine, SQL, transaction, or query-planner claim is made

### Requirement: Finite Byte String
r[text.finite-byte-string]
The system MUST define a finite byte-string primitive with explicit length
bounds, valid byte-index admission, and no-mutation rejection semantics for
checked public operations.

#### Scenario: Checked byte string admits finite length and indices
r[r[text.finite-byte-string.scenario.checked-byte-string-admits-finite-length-and-indices]]

- GIVEN a finite byte sequence and representable length metadata
- WHEN a checked byte-string constructor or checker evaluates the sequence
- THEN it admits only byte indices within the finite length and exposes the length as representable metadata

#### Scenario: Invalid byte string metadata is rejected without mutation
r[r[text.finite-byte-string.scenario.invalid-byte-string-metadata-rejected-without-mutation]]

- GIVEN a previously valid byte-string value
- WHEN a public checked operation receives overflowing length, invalid slice, or out-of-bounds index metadata
- THEN it rejects the input explicitly and leaves the previously valid value unchanged

### Requirement: UTF-8 Boundary Range
r[text.utf8-boundary-range]
The system MUST define UTF-8 boundary/range metadata over finite byte strings for
admitted character-boundary indices and half-open character/byte spans.

#### Scenario: UTF-8 boundary range admits only character boundaries
r[r[text.utf8-boundary-range.scenario.utf8-boundary-range-admits-only-character-boundaries]]

- GIVEN a finite byte string and a candidate byte range
- WHEN the UTF-8 boundary-range primitive admits the range
- THEN the range start and end are in bounds and occur on declared UTF-8 character boundaries

#### Scenario: Invalid UTF-8 range is rejected
r[r[text.utf8-boundary-range.scenario.invalid-utf8-range-rejected]]

- GIVEN a byte range that is out of bounds, reversed, or not aligned to admitted UTF-8 character boundaries
- WHEN a checked UTF-8 range operation evaluates the range
- THEN it rejects the range without exposing character-index metadata for it

### Requirement: Token Span
r[text.token-span]
The system MUST define token-span metadata over finite text with token index,
byte span, optional character span, and deterministic span-order constraints.

#### Scenario: Token span maps token to admitted byte and character ranges
r[r[text.token-span.scenario.token-span-maps-token-to-admitted-byte-and-character-ranges]]

- GIVEN finite text, token metadata, and an admitted token index
- WHEN the token-span primitive evaluates the token
- THEN it exposes a finite half-open byte span and, when UTF-8 metadata is present, a corresponding admitted character span

#### Scenario: Token spans are ordered or explicitly overlapping by policy
r[r[text.token-span.scenario.token-spans-are-ordered-or-explicitly-overlapping-by-policy]]

- GIVEN multiple token spans in finite text
- WHEN the token-span metadata declares a non-overlap policy
- THEN each later token span starts at or after the previous token span end

### Requirement: Prefix and Suffix Predicate
r[text.prefix-suffix]
The system MUST define finite prefix and suffix predicates with length bounds,
byte/character layer declaration, and exact sequence equality over admitted
positions.

#### Scenario: Prefix predicate compares only admitted prefix positions
r[r[text.prefix-suffix.scenario.prefix-predicate-compares-only-admitted-prefix-positions]]

- GIVEN finite haystack and prefix sequences with admitted byte or character layer metadata
- WHEN the prefix predicate accepts the prefix
- THEN the prefix length is less than or equal to the haystack length and every admitted prefix position equals the corresponding haystack position

#### Scenario: Suffix predicate compares only admitted suffix positions
r[r[text.prefix-suffix.scenario.suffix-predicate-compares-only-admitted-suffix-positions]]

- GIVEN finite haystack and suffix sequences with admitted byte or character layer metadata
- WHEN the suffix predicate accepts the suffix
- THEN the suffix length is less than or equal to the haystack length and every admitted suffix position equals the corresponding haystack position at the declared offset

### Requirement: Substring Span
r[text.substring-span]
The system MUST define substring containment and match-span admission over finite
text with deterministic span metadata for accepted matches.

#### Scenario: Substring match span is finite and in bounds
r[r[text.substring-span.scenario.substring-match-span-is-finite-and-in-bounds]]

- GIVEN finite haystack and needle sequences plus a declared byte or character layer
- WHEN the substring-span primitive admits a match
- THEN the match span is within the haystack bounds and every admitted needle position equals the corresponding haystack position inside that span

#### Scenario: Substring absence does not expose match span
r[r[text.substring-span.scenario.substring-absence-does-not-expose-match-span]]

- GIVEN finite haystack and needle sequences
- WHEN the substring-span primitive reports no admitted match
- THEN it exposes absence metadata without constructing an out-of-bounds or inconsistent match span

### Requirement: Edit Distance Envelope
r[text.edit-distance-envelope]
The system MUST define a bounded edit-distance envelope primitive for finite
source/target lengths, operation-budget metadata, and conservative lower/upper
bounds on accepted edit operations.

#### Scenario: Edit-distance envelope admits only within declared budget
r[r[text.edit-distance-envelope.scenario.edit-distance-envelope-admits-only-within-declared-budget]]

- GIVEN finite source and target lengths plus an edit-operation budget
- WHEN the edit-distance envelope accepts a candidate edit path or summary
- THEN the accepted operation count is representable, within the declared budget, and consistent with the finite source/target length envelope

#### Scenario: Edit-distance envelope avoids unproved optimality claims
r[r[text.edit-distance-envelope.scenario.edit-distance-envelope-avoids-unproved-optimality-claims]]

- GIVEN an edit-distance envelope requirement or implementation task
- WHEN it states exported guarantees
- THEN it does not claim algorithmic optimality or Levenshtein distance equality unless a dedicated proof for that algorithm is also specified

### Requirement: Normalization Envelope
r[text.normalization-envelope]
The system MUST define normalization envelope metadata for finite source/output
spans, length-growth bounds, supported normalization form tags, and explicit
unsupported-form rejection.

#### Scenario: Normalization envelope bounds output span metadata
r[r[text.normalization-envelope.scenario.normalization-envelope-bounds-output-span-metadata]]

- GIVEN finite source text, a supported normalization form tag, and declared output length bounds
- WHEN the normalization-envelope primitive admits normalized output metadata
- THEN the output span is finite, representable, and within the declared conservative length-growth envelope

#### Scenario: Normalization envelope avoids Unicode table correctness claims
r[r[text.normalization-envelope.scenario.normalization-envelope-avoids-unicode-table-correctness-claims]]

- GIVEN a normalization-envelope requirement or implementation task
- WHEN it states exported guarantees
- THEN it does not claim full Unicode normalization table correctness, locale collation, or semantic text equivalence unless a dedicated model owns those claims

### Requirement: Delimiter Split Join Shape
r[text.delimiter-split-join]
The system MUST define delimiter split/join shape metadata over finite text,
finite delimiter spans, segment bounds, and deterministic empty-segment policy.

#### Scenario: Delimiter split exposes finite ordered segments
r[r[text.delimiter-split-join.scenario.delimiter-split-exposes-finite-ordered-segments]]

- GIVEN finite text, finite delimiter metadata, and a declared empty-segment policy
- WHEN the delimiter split-shape primitive accepts the metadata
- THEN it exposes finite ordered segment spans that are in bounds and consistent with the delimiter positions and empty-segment policy

#### Scenario: Join shape accounts for delimiters between segments
r[r[text.delimiter-split-join.scenario.join-shape-accounts-for-delimiters-between-segments]]

- GIVEN finite segment lengths and finite delimiter length metadata
- WHEN the delimiter join-shape primitive computes or validates output length metadata
- THEN the output length envelope accounts for every segment and every inserted delimiter between adjacent segments without overflow

### Requirement: Content-Addressed Definition Identity
r[domain-primitives.content-addressed-code.definition-identity]
The system MUST define finite content-addressed definition identity envelopes where a definition hash is admitted only for a declared normalized definition kind, dependency-hash sequence, and metadata boundary, while names remain separate aliases and do not affect definition identity.

#### Scenario: Name change preserves definition identity

- GIVEN the same normalized definition and dependency hashes with two different human-readable names
- WHEN definition identity admission is evaluated
- THEN both aliases point at the same admitted definition hash envelope

#### Scenario: Dependency change changes identity input

- GIVEN two otherwise equal definition envelopes with different dependency-hash sequences
- WHEN definition identity admission is evaluated
- THEN the envelopes are distinguished by dependency identity metadata

#### Scenario: Unknown definition hash is rejected

- GIVEN a name binding that points to a hash absent from the finite codebase definition set
- WHEN name-binding admission is evaluated
- THEN the binding is rejected with missing-definition metadata

### Requirement: Dependency Closure Admission
r[domain-primitives.content-addressed-code.dependency-closure]
The system MUST define finite dependency-closure admission for a root definition hash, reporting whether every transitively referenced dependency hash is present in the declared codebase and producing deterministic missing-dependency metadata without claiming runtime linking or execution.

#### Scenario: Complete closure is admitted

- GIVEN a root definition whose transitive dependency hashes are all present in the finite codebase
- WHEN dependency-closure admission is evaluated
- THEN the root is admitted as closure-complete

#### Scenario: Missing dependency is reported

- GIVEN a root definition whose transitive dependency closure references an absent hash
- WHEN dependency-closure admission is evaluated
- THEN the root is rejected with the missing dependency hash metadata

#### Scenario: Adding missing dependency can close the root

- GIVEN a rejected root whose only missing dependency hash is added to the finite codebase with its own complete closure
- WHEN dependency-closure admission is reevaluated
- THEN the root is admitted as closure-complete

### Requirement: Recursive Definition Group Canonical Envelope
r[domain-primitives.content-addressed-code.recursive-group]
The system MUST define finite recursive definition group envelopes whose members are ordered canonically by declared member identity, whose intra-group references use stable member positions, and whose group hash metadata is independent of source-file order.

#### Scenario: Source order does not affect group envelope

- GIVEN two recursive group declarations with the same members in different source order
- WHEN canonical group admission is evaluated
- THEN both declarations produce the same admitted member-order metadata

#### Scenario: Out-of-range recursive reference is rejected

- GIVEN a recursive group member referencing a member position outside the canonical group bounds
- WHEN group admission is evaluated
- THEN the group is rejected with out-of-range-recursive-reference metadata

### Requirement: Ability Requirement Set Admission
r[domain-primitives.ability-effect.requirement-set]
The system MUST define finite ability requirement sets where a computation is admitted in a context only when every required ability is provided by that context, and an empty requirement set is treated as pure without claiming runtime side-effect absence beyond the finite model.

#### Scenario: Provided context admits requirements

- GIVEN a computation requiring abilities `A` and `B`
- WHEN the provided context contains `A`, `B`, and possibly additional abilities
- THEN ability admission accepts the computation

#### Scenario: Missing ability is rejected

- GIVEN a computation requiring abilities `A` and `B`
- WHEN the provided context lacks `B`
- THEN ability admission rejects the computation with missing-ability metadata

#### Scenario: Empty requirement is pure in the model

- GIVEN a computation with an empty ability requirement set
- WHEN ability admission is evaluated against any context
- THEN the computation is admitted as requiring no modeled abilities

### Requirement: Ability Handler Discharge
r[domain-primitives.ability-effect.handler-discharge]
The system MUST define finite ability handler discharge where a handler for an ability removes exactly the handled ability from the required set when the handler is declared available, leaving all unhandled abilities explicit in the resulting requirement set.

#### Scenario: Handler removes declared ability

- GIVEN a computation requiring `Store` and `Exception`
- WHEN a handler for `Store` is applied
- THEN the resulting modeled requirement set still contains `Exception` and no longer contains `Store`

#### Scenario: Handler cannot discharge absent declaration

- GIVEN a computation requiring `Remote`
- WHEN a handler declaration does not include `Remote`
- THEN the handler discharge check does not remove `Remote`

### Requirement: Ability Sandbox Attenuation
r[domain-primitives.ability-effect.sandbox-attenuation]
The system MUST define sandbox ability attenuation where a sandboxed context can provide only a subset of its parent context and remote or untrusted computations are rejected when their requirements exceed the sandbox-provided ability set.

#### Scenario: Sandbox subset is admitted

- GIVEN a parent context with `IO`, `Remote`, and `Exception`
- WHEN a sandbox provides only `Exception`
- THEN the sandbox context is admitted as an attenuation of the parent

#### Scenario: Sandbox cannot add ability

- GIVEN a parent context lacking `IO`
- WHEN a sandbox declaration attempts to provide `IO`
- THEN sandbox attenuation rejects the declaration

#### Scenario: Computation exceeding sandbox is rejected

- GIVEN a sandbox providing only `Exception`
- WHEN a computation requiring `IO` is evaluated for sandbox admission
- THEN the computation is rejected with missing-ability metadata

### Requirement: Algebraic Effect Signature Admission
r[domain-primitives.algebraic-effect.signature]
The system MUST define finite algebraic-effect signatures whose operation declarations admit calls only by exact `(effect, operation)` identity, without claiming runtime stack capture, typechecker correctness, or handler execution semantics.

#### Scenario: Declared operation is admitted

- GIVEN a signature declaring operation `Read` for effect `Store`
- WHEN a call references `(Store, Read)`
- THEN signature admission accepts the call

#### Scenario: Undeclared operation is rejected

- GIVEN a signature declaring operation `Read` for effect `Store`
- WHEN a call references `(Store, Write)` or `(Console, Read)` without a matching declaration
- THEN signature admission rejects the call

### Requirement: Algebraic Effect Invocation Arity
r[domain-primitives.algebraic-effect.arity]
The system MUST define finite invocation admission where an invocation is accepted only when the signature declares the same `(effect, operation)` identity and the declared input and output arities exactly match the invocation counts, while preserving a separate call-identity projection and without claiming runtime argument typechecking.

#### Scenario: Matching arity is admitted

- GIVEN a signature declaring `State.Put` with two inputs and one output
- WHEN an invocation references `State.Put` with two input values and one expected output
- THEN invocation admission accepts the invocation and its call identity is `(State, Put)`

#### Scenario: Arity mismatch is rejected

- GIVEN a signature declaring `State.Put` with two inputs and one output
- WHEN an invocation references `State.Put` with one input or two expected outputs
- THEN invocation admission rejects the invocation

### Requirement: Algebraic Effect Row
r[domain-primitives.algebraic-effect.effect-row]
The system MUST define finite effect rows as permitted effect identifiers, admit calls only when their effect is present in the row, admit handlers only when they are available and target a row effect, and remove handled effects while preserving the order of remaining row entries without claiming type inference or runtime effect tracking.

#### Scenario: Row admits permitted calls

- GIVEN an effect row containing `State` and `Console`
- WHEN calls for `State` and `Async` are checked against the row
- THEN the `State` call is admitted and the `Async` call is rejected

#### Scenario: Handler target must be available in the row

- GIVEN an effect row containing `State`
- WHEN an available `State` handler and an unavailable `State` handler are checked
- THEN only the available handler is admitted as targeting a row effect

#### Scenario: Handled effect is removed from residual row

- GIVEN an effect row containing `State`, `Console`, and another `State` entry
- WHEN `State` is removed as handled
- THEN the residual row preserves only the non-`State` effects in their original order

### Requirement: Algebraic Effect Summary
r[domain-primitives.algebraic-effect.effect-summary]
The system MUST define finite effect summaries with required and residual rows, admit summaries only when every required effect is present in the ambient row, and admit sequential summary composition only when the left residual row admits the right summary requirements, producing the left required row and right residual row without claiming full effect inference.

#### Scenario: Summary requires ambient effects

- GIVEN an ambient row containing `State`, `Console`, and `Async`
- WHEN a summary requiring `State` and `Async` is checked against that row
- THEN summary admission accepts it

#### Scenario: Missing required effect rejects summary

- GIVEN an ambient row containing `State` and `Console`
- WHEN a summary requiring `State` and `Async` is checked against that row
- THEN summary admission rejects it

#### Scenario: Sequential summaries compose through residual row

- GIVEN a left summary whose residual row contains `Async` and a right summary requiring `Async`
- WHEN summary composition is evaluated
- THEN composition is admitted and the composed summary preserves the left required row and right residual row

#### Scenario: Sequential summary composition rejects missing residual effects

- GIVEN a left summary whose residual row lacks `Async` and a right summary requiring `Async`
- WHEN summary composition is evaluated
- THEN composition is rejected

### Requirement: Algebraic Effect Summary Handler Application
r[domain-primitives.algebraic-effect.summary-handler-application]
The system MUST define finite summary handler application that admits only available handlers whose handled effect appears in the summary required row, and applying an admitted handler MUST preserve the summary required row while removing that handled effect from the residual row without claiming handler execution semantics.

#### Scenario: Available required handler applies to summary

- GIVEN a summary whose required and residual rows include `State`
- WHEN an available `State` handler is checked and applied
- THEN handler admission accepts it and the applied summary preserves the required row while removing `State` from the residual row

#### Scenario: Unavailable or unrequired handler is rejected

- GIVEN a summary whose required row includes `State` but not `Async`
- WHEN an unavailable `State` handler or an available `Async` handler is checked
- THEN summary handler admission rejects the handler

### Requirement: Algebraic Effect Handler Coverage
r[domain-primitives.algebraic-effect.handler-coverage]
The system MUST define finite handler coverage where an available handler for one effect covers exactly the declared operations for that effect that have matching handler clauses, leaving other effects outside the coverage claim.

#### Scenario: Complete handler covers its handled effect

- GIVEN a signature with operations `Ask` and `Tell` for effect `Log`
- WHEN an available handler for `Log` declares clauses for both operations
- THEN handler coverage accepts the handler for that effect

#### Scenario: Missing handler clause rejects coverage

- GIVEN a signature with operations `Ask` and `Tell` for effect `Log`
- WHEN a handler for `Log` lacks a `Tell` clause
- THEN handler coverage rejects the handler for that effect

### Requirement: Algebraic Effect Handler Discharge
r[domain-primitives.algebraic-effect.handler-discharge]
The system MUST define finite residual-call discharge where a handler removes only calls for its handled effect and declared operation clauses, preserving calls for other effects or missing clauses in order.

#### Scenario: Matching calls are discharged

- GIVEN a call sequence containing handled `State.Get` calls and unhandled `Console.Print` calls
- WHEN an available `State` handler with a `Get` clause is applied
- THEN the residual call sequence excludes the handled `State.Get` calls and preserves the `Console.Print` calls in order

#### Scenario: Missing clause remains residual

- GIVEN a call sequence containing `State.Put`
- WHEN a `State` handler lacks a `Put` clause
- THEN `State.Put` remains in the residual call sequence

### Requirement: Algebraic Effect Handler Composition
r[domain-primitives.algebraic-effect.handler-composition]
The system MUST define finite handler composition that admits only available handlers for the same effect with disjoint operation clauses, rejects duplicate cross-handler clauses as ambiguous, and produces a composed handler whose clauses are the left handler clauses followed by the right handler clauses while preserving the handled effect.

#### Scenario: Disjoint handlers compose

- GIVEN two available handlers for effect `State` with disjoint `Get` and `Put` clauses
- WHEN handler composition is evaluated
- THEN composition is admitted and the composed handler handles both operations for `State`

#### Scenario: Duplicate clause is ambiguous

- GIVEN two available handlers for effect `State` that both declare a `Get` clause
- WHEN handler composition is evaluated
- THEN composition is rejected as ambiguous

#### Scenario: Different effects do not compose

- GIVEN one handler for effect `State` and another handler for effect `Console`
- WHEN handler composition is evaluated
- THEN composition is rejected without producing a combined handler

### Requirement: Algebraic Effect Continuation Token
r[domain-primitives.algebraic-effect.continuation-token]
The system MUST define finite continuation tokens that can be resumed only by an available handler for the token's effect and become unusable after consumption, without claiming runtime continuation capture or re-entry semantics.

#### Scenario: Matching unused token can resume

- GIVEN an unused continuation token for effect `Async`
- WHEN an available `Async` handler checks resume admission
- THEN the token is admitted for resume

#### Scenario: Consumed token cannot resume again

- GIVEN a continuation token that has been consumed
- WHEN any handler checks resume admission
- THEN the token is rejected as already used

### Requirement: Algebraic Effect Continuation Resume Mode
r[domain-primitives.algebraic-effect.continuation-resume-mode]
The system MUST define finite operation resume modes that distinguish resumable from non-resumable operations, admit continuation resume only when the token and handler match an explicitly resumable operation mode, and reject handler clauses that attempt to resume non-resumable operations without claiming runtime continuation capture or re-entry semantics.

#### Scenario: Resumable operation admits matching continuation resume

- GIVEN an unused continuation token for effect `Async`, an available `Async` handler, and a resumable operation mode for `Async.Await`
- WHEN continuation resume-mode admission is evaluated
- THEN the resume is admitted

#### Scenario: Non-resumable operation rejects continuation resume

- GIVEN an unused continuation token for effect `Log` and a non-resumable operation mode for `Log.Emit`
- WHEN continuation resume-mode admission is evaluated
- THEN the resume is rejected even if the token and handler effects match

#### Scenario: Resuming clause must match operation mode

- GIVEN a handler clause that resumes operation `State.Get`
- WHEN it is checked against a non-resumable `State.Get` operation mode or a different operation mode
- THEN the clause is rejected as incompatible

### Requirement: Typed Durable Reference Envelope
r[domain-primitives.typed-durable-reference.envelope]
The system MUST define finite typed durable reference envelopes containing value identity, expected type identity, schema/version metadata, and dependency-closure identity, and MUST reject references whose declared metadata is inconsistent or outside finite model bounds.

#### Scenario: Well-formed typed reference is admitted

- GIVEN a durable reference with admitted value hash, type hash, schema version, and dependency closure identity
- WHEN typed durable reference admission is evaluated
- THEN the reference is admitted

#### Scenario: Missing dependency closure is rejected

- GIVEN a durable reference whose declared dependency closure identity is absent from the finite codebase model
- WHEN typed durable reference admission is evaluated
- THEN the reference is rejected with missing-closure metadata

### Requirement: Unpersist Type Match Admission
r[domain-primitives.typed-durable-reference.unpersist-type-match]
The system MUST define unpersist admission that succeeds only when the caller's expected type identity equals the persisted reference's declared type identity and the referenced dependency closure is available, without claiming runtime byte decoding or storage retrieval correctness.

#### Scenario: Matching type identity admits unpersist

- GIVEN an admitted durable reference with type hash `T`
- WHEN a caller requests unpersist with expected type hash `T` and the dependency closure is available
- THEN unpersist admission accepts the request

#### Scenario: Mismatched type identity rejects unpersist

- GIVEN an admitted durable reference with type hash `T`
- WHEN a caller requests unpersist with a different expected type hash `U`
- THEN unpersist admission rejects the request with type-mismatch metadata

#### Scenario: Available type is not enough without closure

- GIVEN matching type identity but an unavailable dependency closure
- WHEN unpersist admission is evaluated
- THEN the request is rejected with missing-closure metadata

### Requirement: Explicit Durable Reference Migration Boundary
r[domain-primitives.typed-durable-reference.migration-boundary]
The system MUST define schema/type migration admission as an explicit conversion witness from an old typed durable reference identity to a new typed durable reference identity, and MUST reject implicit migration when no conversion witness is declared.

#### Scenario: Explicit conversion admits migration

- GIVEN an old typed reference and a declared conversion witness to a new type/schema identity
- WHEN migration admission is evaluated
- THEN the migrated reference identity is admitted according to the witness metadata

#### Scenario: Implicit type change is rejected

- GIVEN a persisted reference whose type identity differs from the caller's expected type
- WHEN no conversion witness is declared
- THEN migration admission rejects the request

### Requirement: Remote Computation Envelope
r[domain-primitives.remote-computation.envelope]
The system MUST define finite remote computation envelopes containing computation identity, dependency-closure requirements, ability requirement sets, target node identity, and declared resource bounds, and MUST reject unknown targets or malformed bounds without claiming real network transport or execution.

#### Scenario: Well-formed envelope is admitted

- GIVEN a remote computation envelope whose computation, target node, dependency set, ability set, and resource bounds are all declared in the finite model
- WHEN envelope admission is evaluated
- THEN the envelope is admitted

#### Scenario: Unknown target node is rejected

- GIVEN a remote computation envelope targeting a node absent from the finite node set
- WHEN envelope admission is evaluated
- THEN the envelope is rejected with unknown-target metadata

#### Scenario: Malformed resource bounds are rejected

- GIVEN a remote computation envelope with zero declared memory or step bounds
- WHEN envelope admission is evaluated
- THEN the envelope is rejected with malformed-bound metadata

### Requirement: Remote Dependency Availability Admission
r[domain-primitives.remote-computation.dependency-availability]
The system MUST define remote dependency availability admission that accepts a computation only when every dependency hash is already present on the target node or is declared fetchable, and MUST deterministically report ready, fetch-pending, or rejected state with the first missing dependency hash.

#### Scenario: Present closure admits dependency availability

- GIVEN a target node whose cache contains every dependency hash required by the computation
- WHEN dependency availability is evaluated
- THEN the computation is admitted as dependency-ready

#### Scenario: Fetchable missing dependency is pending

- GIVEN a required dependency hash absent from the target node but declared fetchable from the source
- WHEN dependency availability is evaluated
- THEN the computation is admitted only as fetch-pending with the missing dependency metadata

#### Scenario: Unfetchable missing dependency is rejected

- GIVEN a required dependency hash absent from the target node and not declared fetchable
- WHEN dependency availability is evaluated
- THEN remote admission rejects the computation with unfetchable-dependency metadata

### Requirement: Remote Ability And Resource Admission
r[domain-primitives.remote-computation.ability-resource]
The system MUST define remote ability and resource admission where a target node may run a computation only when dependencies are ready, the sandbox-provided abilities cover the computation's required abilities, and declared resource bounds fit within the target's finite resource limits.

#### Scenario: Ability and resource bounds admit execution eligibility

- GIVEN a computation whose dependencies are ready, required abilities are provided by the target sandbox, and declared resource bounds fit the target limits
- WHEN remote execution eligibility is evaluated
- THEN the computation is admitted as eligible in the finite model

#### Scenario: Missing remote ability rejects execution eligibility

- GIVEN a computation requiring `Remote` or `IO`
- WHEN the target sandbox does not provide that ability
- THEN remote execution eligibility rejects the computation with missing-ability metadata

#### Scenario: Resource bound overflow rejects execution eligibility

- GIVEN a computation declaring memory or step bounds above the target's finite limits
- WHEN remote execution eligibility is evaluated
- THEN the computation is rejected with resource-bound metadata

### Requirement: Immutable Definition Binding Refactor
r[domain-primitives.semantic-refactoring.immutable-binding]
The system MUST define finite refactoring-session metadata where moving a name from an old definition hash to a new definition hash never mutates the old definition identity and always records both old and new bindings explicitly.

#### Scenario: Rename does not mutate old definition

- GIVEN a name binding initially pointing to old definition hash `H1`
- WHEN the name is moved to new definition hash `H2`
- THEN the old definition `H1` remains present and unchanged in the finite codebase model

#### Scenario: Unknown replacement is rejected

- GIVEN a refactoring step moving a name to a definition hash absent from the finite codebase model
- WHEN refactoring admission is evaluated
- THEN the step is rejected with missing-new-definition metadata

### Requirement: Compatible Replacement Propagation
r[domain-primitives.semantic-refactoring.compatible-propagation]
The system MUST define compatible replacement propagation where dependents of an old definition may be moved to a new definition only when declared type/interface compatibility metadata admits the replacement, and propagation preserves dependent validity in the finite model.

#### Scenario: Compatible replacement propagates to dependent

- GIVEN a dependent definition referencing old hash `H1` and a compatible replacement hash `H2`
- WHEN propagation admission is evaluated
- THEN the dependent may be admitted with the reference updated to `H2`

#### Scenario: Incompatible replacement cannot propagate

- GIVEN a dependent definition referencing old hash `H1` and a replacement hash `H2` whose type/interface metadata is incompatible
- WHEN propagation admission is evaluated
- THEN the dependent is not rewritten and is recorded as unresolved

### Requirement: Refactoring Session Validity
r[gpu.block-dim]
The system MUST define finite refactoring session validity where every step either preserves admitted bindings/dependents or records unresolved dependents explicitly, so the codebase model remains queryable and no name points to an absent definition.

#### Scenario: Session with unresolved dependent remains explicit

- GIVEN an incompatible replacement with at least one dependent that cannot be propagated
- WHEN session validity is evaluated
- THEN the session is admitted only if that dependent appears in the unresolved-dependent set

#### Scenario: Dangling name rejects session

- GIVEN a refactoring session whose resulting name table points to an absent definition hash
- WHEN session validity is evaluated
- THEN the session is rejected with dangling-name metadata

#### Scenario: Fully propagated session has no unresolved dependents

- GIVEN every dependent of an old definition is compatible with the replacement
- WHEN session validity is evaluated after propagation
- THEN the unresolved-dependent set is empty

**GPU Programming Primitives**

GPU block dimension admission MUST require positive x/y/z dimensions and MUST reject blocks whose finite thread product exceeds the declared maximum threads per block.

r[gpu.grid-dim]
GPU grid dimension admission MUST require positive x/y/z dimensions and MUST reject grids whose finite block product exceeds the declared maximum block count.

r[gpu.thread-index]
GPU thread-index admission MUST require an admitted block and grid, local thread coordinates strictly within the block dimensions, and block coordinates strictly within the grid dimensions.

r[gpu.coalesced-access]
GPU coalesced memory-access admission MUST require a positive lane count, positive element size, positive segment size, a lane-span no larger than the segment, and a base/span range contained in the buffer without relying on wrapping arithmetic.

r[gpu.shared-tile]
GPU shared-tile admission MUST require positive rows, columns, and element size, and MUST reject tiles whose finite byte footprint exceeds the declared shared-memory capacity.

r[gpu.launch-envelope]
GPU launch-envelope admission MUST compose admitted block dimensions, grid dimensions, and shared-tile metadata, and MUST reject launches whose shared-tile capacity exceeds the launch-level maximum shared memory per block.

r[gpu.pitched-buffer]
GPU pitched-buffer admission MUST require positive width, height, and element size, MUST require each row byte footprint to fit in the pitch, and MUST require the finite pitch-by-height backing footprint to fit in the buffer.

r[gpu.pitched-address]
GPU pitched-address admission MUST compose an admitted pitched buffer with in-bounds x/y coordinates and MUST require the addressed element span to fit inside the backing buffer.

r[gpu.tensor3d-layout]
GPU tensor3D layout admission MUST require positive width, height, depth, and element size, and MUST reject tensors whose finite row-major byte footprint exceeds the declared buffer size.

r[gpu.tensor3d-index]
GPU tensor3D index admission MUST compose an admitted tensor3D layout with x/y/z coordinates strictly inside the finite tensor shape.

r[gpu.tile-projection]
GPU tile-projection admission MUST compose an admitted pitched buffer with positive tile rows/columns and MUST reject tile origins whose finite rectangle extends past the buffer width or height.

r[gpu.elementwise-program]
GPU elementwise-program admission MUST require a positive element count, positive thread count, and positive elements-per-thread, and MUST reject programs whose finite thread capacity cannot cover all elements.

r[gpu.grid-stride-loop]
GPU grid-stride-loop admission MUST require positive length, stride, and iteration count, MUST require the start index to be in bounds, and MUST reject loop shapes whose last visited index would exceed the finite input length.

r[gpu.tiled-kernel2d]
GPU tiled-kernel2D admission MUST require positive output dimensions, tile dimensions, and grid dimensions, and MUST require the finite tile grid to cover the output domain in both axes.

r[gpu.block-reduction-program]
GPU block-reduction-program admission MUST require positive input, thread, per-thread, block, and element sizes, MUST require finite work capacity to cover the input, and MUST require per-block shared-memory bytes to fit the declared shared capacity.

### Requirement: GPU Reduction Tree Program Semantics
r[domain-primitives.gpu.reduction-tree-program]

The system MUST define finite GPU reduction-tree program semantics where admitted reduction stages have positive input cardinality, bounded active lanes, monotonically non-increasing output cardinality, and a final stage that produces the declared finite number of reduction outputs without claiming operator algebra or numerical accuracy.


#### Scenario: Stage reduces active cardinality

- GIVEN an admitted reduction stage with positive input count and bounded active lanes
- WHEN stage admission is evaluated
- THEN the stage output count is positive and no larger than the input count

#### Scenario: Tree reaches declared outputs

- GIVEN a finite sequence of admitted reduction stages
- WHEN tree admission is evaluated against a declared output count
- THEN the final stage cardinality equals the declared output count

#### Scenario: Invalid expanding stage is rejected

- GIVEN a reduction stage whose output count exceeds its input count
- WHEN stage admission is evaluated
- THEN the stage is rejected

### Requirement: GPU Stencil Boundary Program Semantics
r[domain-primitives.gpu.stencil-boundary-program]

The system MUST define finite GPU stencil-program boundary semantics where admitted output coordinates, stencil radii, and read windows guarantee that every modeled input read remains inside the declared finite input domain before any kernel computation claim is made.


#### Scenario: Interior stencil reads are admitted

- GIVEN an output coordinate whose full radius window lies inside the finite input domain
- WHEN stencil read-window admission is evaluated
- THEN every read offset in the modeled window is admitted as in bounds

#### Scenario: Boundary without guard is rejected

- GIVEN an output coordinate near an edge and a stencil radius that would read outside the input domain
- WHEN unguarded stencil admission is evaluated
- THEN the read window is rejected

#### Scenario: Guarded boundary narrows reads

- GIVEN a boundary coordinate with an explicit guard that skips out-of-domain offsets
- WHEN guarded stencil admission is evaluated
- THEN only in-domain read coordinates are admitted and no out-of-domain read is claimed

### Requirement: GPU Write-Disjoint Program Semantics
r[domain-primitives.gpu.write-disjoint-program]

The system MUST define finite GPU program write-region semantics where admitted output writes have positive byte spans, remain inside the declared output buffer, and pairwise distinct writes in the modeled program domain are disjoint unless explicitly admitted as the same logical output element.


#### Scenario: Elementwise writes are disjoint

- GIVEN an admitted elementwise GPU program with one output element per logical index
- WHEN write-region admission is evaluated for two distinct logical indices
- THEN the two byte ranges are admitted only when their output spans do not overlap

#### Scenario: Tiled output writes stay inside buffer

- GIVEN an admitted tiled 2D GPU program and admitted output layout
- WHEN a tile-local output coordinate is projected to a write region
- THEN the region is rejected if its byte span exceeds the output buffer

#### Scenario: Aliasing writes are rejected

- GIVEN two distinct program indices project to overlapping output byte ranges
- WHEN write-disjointness admission is evaluated
- THEN the pair is rejected with no CUDA scheduling or ordering claim

### Requirement: Parking-Lot Core Queue Validity
r[domain-primitives.parking-lot-core.queue-validity]

The system MUST define a finite parking-lot core model where lock keys map to waiter queues, each waiter appears at most once across all queues, each queued waiter records the key it is parked on, and table/bucket metadata remains valid across queue operations.


#### Scenario: Distinct waiters form a valid queue

- GIVEN a parking-lot table with one key mapped to a queue of distinct waiter identities
- WHEN queue validity is evaluated
- THEN the table is accepted and every waiter is associated with the queue key

#### Scenario: Duplicate waiter is rejected

- GIVEN a waiter identity appears twice in one queue or across two queues
- WHEN queue validity is evaluated
- THEN the table is rejected with duplicate-waiter evidence

#### Scenario: Waiter key mismatch is rejected

- GIVEN a waiter is stored under key `K1` but records parked key `K2`
- WHEN queue validity is evaluated
- THEN the table is rejected with key-mismatch evidence

### Requirement: Parking-Lot Core Operations
r[domain-primitives.parking-lot-core.operations]

The system MUST define finite park, unpark-one, unpark-all, and requeue operations that preserve queue validity, preserve waiter identity except for explicit removal/wake results, and never lose a waiter during key-to-key transfer.


#### Scenario: Park inserts one waiter

- GIVEN a valid parking-lot table and a waiter absent from all queues
- WHEN the waiter parks on key `K`
- THEN the resulting table is valid and the waiter appears exactly once under `K`

#### Scenario: Unpark one returns a queued waiter

- GIVEN a valid queue under key `K` with at least one waiter
- WHEN unpark-one is evaluated for `K`
- THEN exactly one waiter is removed from `K` and returned as the wake result

#### Scenario: Unpark all removes only matching-key waiters

- GIVEN a valid table with waiters under keys `K1` and `K2`
- WHEN unpark-all is evaluated for `K1`
- THEN all and only waiters under `K1` are returned and queues for other keys are unchanged

#### Scenario: Requeue preserves non-woken waiters

- GIVEN a valid source queue under key `FROM` and destination key `TO`
- WHEN one waiter is woken and remaining source waiters are requeued to `TO`
- THEN the woken waiter is removed, every non-woken source waiter appears under `TO`, and no waiter is lost or duplicated

### Requirement: Raw Mutex Parking Bits
r[domain-primitives.parking-lot-raw-mutex.state-machine]

The system MUST define a compact raw-mutex state machine with locked and parked bits where uncontended acquisition uses a direct free-to-locked transition, contended acquisition sets the parked bit before sleeping, and unlocking preserves the invariant that queued waiters are represented by the parked bit until they are woken, handed off, or proven absent.


#### Scenario: Uncontended lock takes fast path

- GIVEN a raw mutex state with neither locked nor parked bit set
- WHEN lock acquisition is evaluated
- THEN the resulting state has the locked bit set and no waiter is introduced

#### Scenario: Contended lock records parked waiter

- GIVEN a raw mutex state with locked bit set
- WHEN a new waiter attempts blocking acquisition
- THEN the parked bit is set before the waiter is admitted to the parking-lot queue

#### Scenario: Unlock without waiters clears locked bit

- GIVEN a raw mutex state with locked bit set, parked bit clear, and no queued waiters
- WHEN unlock is evaluated
- THEN the resulting state has both locked and parked bits clear

#### Scenario: Waiters cannot be stranded without parked bit

- GIVEN a raw mutex has one or more queued waiters
- WHEN state validity is evaluated
- THEN a state with parked bit clear is rejected unless an explicit wake, timeout, abort, or handoff result accounts for every waiter

### Requirement: Condition Variable Requeue Without Spurious Wakeup
r[domain-primitives.parking-lot-condvar.requeue-no-spurious]

The system MUST define condition-variable semantics where a condvar is associated with at most one mutex while waiters are present, notify-one wakes or transfers at most one waiter, notify-all wakes at most one waiter and requeues the rest to the associated mutex, and non-timeout wake results are produced only by explicit notifications.


#### Scenario: Condvar binds to one mutex while waiting

- GIVEN a condvar has waiters associated with mutex `M1`
- WHEN another waiter attempts to wait using mutex `M2`
- THEN the wait is rejected unless the condvar has no remaining waiters for `M1`

#### Scenario: Notify one wakes one waiter

- GIVEN a condvar queue with multiple waiters associated with mutex `M`
- WHEN notify-one is evaluated
- THEN at most one waiter receives a notification and all remaining waiters stay associated with the condvar or are explicitly requeued

#### Scenario: Notify all requeues non-woken waiters

- GIVEN a condvar queue with multiple waiters and associated mutex `M`
- WHEN notify-all is evaluated
- THEN at most one waiter is woken immediately and every other waiter is transferred to the mutex wait queue

#### Scenario: No spurious non-timeout wakeup

- GIVEN a waiter returns from condvar wait without a timeout result
- WHEN wake-result validity is evaluated
- THEN the result is accepted only if a matching notify-one or notify-all event exists

### Requirement: Packed Fair RwLock State
r[domain-primitives.parking-lot-rwlock.packed-fair-state]

The system MUST define a parking-lot-style RwLock state model with packed parked, writer-parked, upgradable, writer, and reader-count fields, admitting readers and writers only when mutual exclusion, writer waiting, downgrade, and upgradable-to-writer constraints are preserved.


#### Scenario: Multiple readers without writer are admitted

- GIVEN an RwLock state with reader count below capacity, writer bit clear, and no pending fairness block
- WHEN shared acquisition is evaluated
- THEN the reader count increases and no writer ownership is introduced

#### Scenario: Pending writer blocks new readers

- GIVEN an RwLock state with a pending writer represented by writer or writer-parked metadata
- WHEN a new reader attempts acquisition outside an admitted read phase
- THEN the reader is rejected or queued so writer starvation is not introduced

#### Scenario: Write downgrades to read

- GIVEN an RwLock state with writer bit set and no active readers
- WHEN downgrade is evaluated
- THEN writer ownership clears, reader count becomes one, and queued shared waiters may be admitted according to the parking-lot queue model

#### Scenario: Upgradable read is unique

- GIVEN an RwLock state with upgradable bit set
- WHEN another upgradable reader attempts acquisition
- THEN the acquisition is rejected or queued so at most one upgradable reader is active

### Requirement: Fair Unlock and Direct Handoff
r[domain-primitives.parking-lot-fairness.eventual-handoff]

The system MUST define fair unlock and direct handoff semantics where normal unlock may clear ownership before waking a waiter, fair unlock selects an eligible queued waiter, and handoff transfers ownership to that waiter without exposing an unowned-but-stealable interval in the finite model.


#### Scenario: Fair unlock selects queued waiter

- GIVEN a lock with a valid non-empty wait queue
- WHEN fair unlock is evaluated
- THEN the result identifies an eligible queued waiter and removes or marks that waiter according to the handoff mode

#### Scenario: Direct handoff preserves ownership continuity

- GIVEN a locked mutex and a selected queued waiter
- WHEN direct handoff is evaluated
- THEN the lock remains logically owned before and after the transition, with ownership transferred from the old holder to the selected waiter

#### Scenario: Empty queue fair unlock behaves as normal unlock

- GIVEN a lock with no queued waiters
- WHEN fair unlock is evaluated
- THEN the result matches normal unlock and does not fabricate a waiter

### Requirement: Timed Wait and Timeout Admission
r[domain-primitives.parking-lot-timeout.timed-wait]

The system MUST define timed wait semantics where deadlines and timeout results are explicit, timeout cannot be reported before the modeled deadline, a notify result cannot be fabricated without a matching wake event, and timeout/notify races resolve to exactly one accepted outcome.


#### Scenario: Deadline not reached cannot time out

- GIVEN a waiter with deadline `D` and modeled time `T` where `T < D`
- WHEN timeout admission is evaluated
- THEN a timeout result is rejected

#### Scenario: Deadline reached may time out

- GIVEN a waiter with deadline `D` and modeled time `T` where `T >= D`
- WHEN timeout admission is evaluated
- THEN a timeout result is accepted and the waiter is removed from the wait queue

#### Scenario: Notify beats timeout once

- GIVEN a waiter has both an eligible notify event and an eligible timeout event
- WHEN wake resolution is evaluated
- THEN exactly one outcome is selected and the waiter is not both woken and timed out

### Requirement: Reentrant Mutex Owner Count
r[domain-primitives.parking-lot-reentrant-mutex.owner-count]

The system MUST define reentrant mutex semantics with explicit owner identity and recursion count where the owner may reacquire, non-owners are excluded while the mutex is owned, only the owner may release, and ownership clears exactly when the recursion count returns to zero.


#### Scenario: Owner reacquires

- GIVEN a reentrant mutex owned by thread `T` with recursion count `N`
- WHEN thread `T` acquires it again
- THEN ownership remains `T` and recursion count becomes `N + 1` unless the count is saturated or rejected by a capacity rule

#### Scenario: Non-owner cannot acquire while owned

- GIVEN a reentrant mutex owned by thread `T1`
- WHEN thread `T2` attempts immediate acquisition
- THEN the acquisition is rejected or queued and ownership remains `T1`

#### Scenario: Final owner release clears ownership

- GIVEN a reentrant mutex owned by thread `T` with recursion count `1`
- WHEN `T` releases it
- THEN the mutex becomes unowned and queued waiters may be considered for wake or handoff

#### Scenario: Non-owner release is invalid

- GIVEN a reentrant mutex owned by thread `T1`
- WHEN thread `T2` attempts release
- THEN the release is rejected and owner/count state is unchanged

### Requirement: Once and OnceLock Exactly-Once Initialization
r[domain-primitives.parking-lot-once.exactly-once]

The system MUST define Once and OnceLock initialization states where at most one initializer runs at a time, completion publishes exactly one initialized value or completion marker, readers observe values only after completion, and retry/poison behavior is represented explicitly when initialization fails.


#### Scenario: First initializer enters running state

- GIVEN a Once value in incomplete state
- WHEN initialization begins
- THEN the state becomes running and no second initializer is admitted concurrently

#### Scenario: Completed value is stable

- GIVEN a OnceLock has completed with value identity `V`
- WHEN additional initialization attempts occur
- THEN the completed value identity remains `V` and no replacement value is admitted

#### Scenario: Reader before completion is rejected

- GIVEN a OnceLock is incomplete or running
- WHEN a reader attempts to observe the value
- THEN value observation is rejected or blocked rather than returning uninitialized data

#### Scenario: Failed initialization records retry policy

- GIVEN an initializer fails or panics under a modeled retry/poison policy
- WHEN state transition is evaluated
- THEN the next state explicitly records whether retry is permitted or future callers observe poison

### Requirement: Raw Lock Trait Contracts
r[domain-primitives.parking-lot-raw-lock-contracts.contracts]

The system MUST define reusable raw-lock contract predicates for mutex and RwLock surfaces, including timed, fair, downgrade, and upgrade extensions, so higher-level lock facades can require contracts instead of relying on implementation-specific state assumptions.


#### Scenario: Raw mutex contract preserves mutual exclusion

- GIVEN a raw mutex implementation satisfies the raw mutex contract
- WHEN two acquisition results are evaluated for the same logical instant
- THEN the contract does not admit two distinct active owners

#### Scenario: Timed extension refines raw acquisition

- GIVEN a timed raw mutex or RwLock extension reports successful timed acquisition
- WHEN contract refinement is evaluated
- THEN the corresponding untimed ownership predicate is also true

#### Scenario: Fair extension selects only queued waiters

- GIVEN a fair raw lock extension performs fair unlock or bump
- WHEN contract validity is evaluated
- THEN any selected successor was present in the modeled wait queue

#### Scenario: Downgrade and upgrade preserve RwLock invariants

- GIVEN an RwLock contract admits downgrade or upgrade
- WHEN the transition is evaluated
- THEN packed-state mutual exclusion, reader count, writer bit, and upgradable uniqueness invariants remain valid

### Requirement: Deadlock Detector Cycle Soundness
r[domain-primitives.parking-lot-deadlock-detector.cycle-soundness]

The system MUST define a wait-for graph detector where reported deadlocks correspond to real nontrivial cycles in the finite graph, acyclic graphs produce no deadlock reports, and detected cycles include enough thread/resource identities for reviewable evidence.


#### Scenario: Three-thread cycle is reported

- GIVEN a wait-for graph with edges `T1 -> R2`, `R2 -> T2`, `T2 -> R3`, `R3 -> T3`, `T3 -> R1`, and `R1 -> T1`
- WHEN deadlock detection is evaluated
- THEN a nontrivial cycle involving those identities is reported

#### Scenario: Acyclic wait graph has no report

- GIVEN a wait-for graph whose edges form a finite acyclic order
- WHEN deadlock detection is evaluated
- THEN no deadlock cycle is reported

#### Scenario: Reported cycle is real

- GIVEN a deadlock detector output containing a cycle
- WHEN report soundness is evaluated
- THEN every adjacent pair in the reported cycle corresponds to an edge in the input wait-for graph and the cycle length is greater than one

#### Scenario: Self-wait policy is explicit

- GIVEN a thread waits on a resource it already owns
- WHEN deadlock detection is evaluated
- THEN the behavior is accepted only if the self-cycle policy explicitly classifies it as reportable or ignored

### Requirement: Spreadsheet Address Envelope
r[domain-primitives.spreadsheet.address]

The system MUST provide finite spreadsheet sheet-bound and cell-address envelopes that record row bounds, column bounds, cell coordinates, and deterministic address-admission results. A checked cell address MUST be admitted only when both coordinates are inside the declared finite sheet bounds, and rejected address admission MUST leave the prior admitted sheet metadata unchanged.

#### Scenario: Cell address must fit sheet bounds

- GIVEN finite spreadsheet sheet metadata and a candidate cell coordinate
- WHEN the coordinate row or column is outside the declared sheet bounds
- THEN the address primitive returns explicit out-of-bounds rejection metadata without constructing an admitted cell reference

#### Scenario: In-bounds cell address is admitted deterministically

- GIVEN finite spreadsheet sheet metadata and a candidate cell coordinate inside the declared row and column bounds
- WHEN address admission is evaluated repeatedly
- THEN the same admitted cell-address metadata is returned each time

### Requirement: Spreadsheet Range Envelope
r[domain-primitives.spreadsheet.range]

The system MUST provide finite rectangular-range envelopes that normalize two admitted endpoint addresses into a canonical rectangle, expose deterministic range membership, and expand only to admitted cell coordinates inside the declared sheet bounds.

#### Scenario: Reversed range endpoints normalize to the same rectangle

- GIVEN two admitted cell addresses in the same finite sheet
- WHEN a rectangular range is admitted using the endpoints in either order
- THEN both admissions produce the same canonical rectangle membership set

#### Scenario: Range rejects invalid endpoints

- GIVEN finite spreadsheet sheet metadata and a range candidate with an out-of-bounds endpoint
- WHEN range admission is evaluated
- THEN the range primitive rejects the range before any membership or expansion result is exposed

### Requirement: Spreadsheet Formula Envelope
r[domain-primitives.spreadsheet.formula]

The system MUST provide a finite spreadsheet formula and value model covering blank values, exact numeric values, booleans, text, explicit errors, constants, cell references, rectangular ranges, arithmetic operations, and basic finite aggregates over admitted ranges. Formula admission MUST reject syntactically or structurally invalid formulas before dependency extraction or evaluation.

#### Scenario: Formula references admitted cells and ranges only

- GIVEN finite sheet metadata and a formula containing cell and range references
- WHEN formula admission is evaluated
- THEN admission succeeds only if every referenced address and range is admitted for the sheet

#### Scenario: Invalid formula shape is rejected before evaluation

- GIVEN a candidate formula with malformed structure or an unsupported operation envelope
- WHEN formula admission is evaluated
- THEN the formula primitive returns explicit rejection metadata without attempting dependency extraction or evaluation

### Requirement: Spreadsheet Dependency Extraction
r[domain-primitives.spreadsheet.dependency]

The system MUST provide deterministic dependency-extraction predicates that return exactly the finite set of precedent cell coordinates named by an admitted formula, including all cells contained in admitted referenced ranges. Dependency extraction MUST be pure over formula structure and declared sheet bounds, and MUST NOT depend on current formatted values, cached results, external workbook files, clocks, or volatile runtime state.

#### Scenario: Range dependency expands to contained cells

- GIVEN an admitted formula that references an admitted rectangular range
- WHEN dependency extraction runs
- THEN every cell coordinate in the canonical range is included in the precedent set and no out-of-range coordinate is included

#### Scenario: Constants have no cell precedents

- GIVEN an admitted constant-only formula
- WHEN dependency extraction runs
- THEN the precedent set is empty

### Requirement: Spreadsheet Cycle Rejection
r[domain-primitives.spreadsheet.cycle]

The system MUST reject spreadsheet formula sets whose dependency graph contains a cycle before formula evaluation begins. An admitted formula set MUST have an acyclic dependency graph over the finite formula-cell coordinates and their extracted precedents.

#### Scenario: Direct circular reference is rejected

- GIVEN a formula cell whose admitted formula directly references the same cell
- WHEN spreadsheet formula-set admission is evaluated
- THEN admission returns explicit cycle rejection metadata and no evaluation order is exposed

#### Scenario: Indirect circular reference is rejected

- GIVEN admitted formula cells whose extracted dependencies form an indirect cycle
- WHEN spreadsheet formula-set admission is evaluated
- THEN admission returns explicit cycle rejection metadata before any formula value is evaluated

### Requirement: Spreadsheet Recalculation Order
r[domain-primitives.spreadsheet.recalc]

The system MUST provide recalculation-order predicates that admit only finite orders where each selected formula cell appears after every selected formula precedent required by its extracted dependency set. Successful recalculation-order admission MUST refine the acyclic dependency graph and MUST NOT introduce formula cells outside the declared finite sheet metadata.

#### Scenario: Topological recalculation order is admitted

- GIVEN an acyclic admitted spreadsheet dependency graph and a candidate order containing selected formula cells
- WHEN every formula cell appears after its selected precedents
- THEN recalculation-order admission succeeds

#### Scenario: Precedent-after-dependent order is rejected

- GIVEN an admitted spreadsheet dependency graph and a candidate order that places a formula cell before one of its selected formula precedents
- WHEN recalculation-order admission is evaluated
- THEN the order is rejected and no evaluation result is exposed for that order

### Requirement: Spreadsheet Dirty Propagation
r[domain-primitives.spreadsheet.dirty-propagation]

The system MUST provide dirty-cell propagation predicates that include each changed cell and every formula cell reachable from the changed cells through reverse spreadsheet dependencies, while excluding formula cells that are not reachable dependents under the admitted finite dependency graph.

#### Scenario: Dependent formulas become dirty

- GIVEN an admitted acyclic spreadsheet dependency graph and a changed input cell
- WHEN dirty propagation runs
- THEN every formula cell that transitively depends on the changed cell is included in the dirty set

#### Scenario: Unrelated formulas stay clean

- GIVEN an admitted acyclic spreadsheet dependency graph, a changed cell, and a formula cell that is not reachable through reverse dependencies from that changed cell
- WHEN dirty propagation runs
- THEN the unrelated formula cell is not included in the dirty set

### Requirement: Spreadsheet Error Envelope
r[domain-primitives.spreadsheet.error]

The system MUST provide explicit deterministic spreadsheet error envelopes for bad references, division by zero, value-shape mismatches, and cycle rejection. Error-producing checks MUST return the same error metadata for the same admitted inputs, MUST NOT panic, and MUST NOT mutate admitted sheet, formula, dependency, or recalculation metadata.

#### Scenario: Division by zero returns explicit error

- GIVEN an admitted arithmetic formula whose denominator evaluates to exact zero
- WHEN formula evaluation is checked
- THEN the result is the explicit division-by-zero error envelope

#### Scenario: Bad reference returns explicit error

- GIVEN a formula reference that no longer resolves to an admitted cell under the declared sheet metadata
- WHEN formula admission or evaluation is checked
- THEN the result is the explicit bad-reference rejection or error envelope

### Requirement: Spreadsheet Admission Composition
r[domain-primitives.spreadsheet.admission]

The system MUST provide a deterministic spreadsheet-admission composition that requires address admission, range admission, formula admission, dependency extraction, cycle rejection, recalculation-order admission, dirty-propagation admission, and error-envelope determinism before any spreadsheet evaluation claim is exposed.

#### Scenario: Admission requires every spreadsheet check

- GIVEN finite spreadsheet metadata and candidate formula/recalculation inputs
- WHEN any address, range, formula, dependency, cycle, recalculation-order, dirty-propagation, or error-envelope check rejects
- THEN the spreadsheet-admission primitive returns an explicit rejection before claiming any evaluated spreadsheet value

### Requirement: Distinct Durable Reference Identity Types
r[newtypes.durable-identities.types] The typed durable reference model MUST use distinct structures for reference, value, type, codec, closure, and schema identities in every envelope, request, catalog, result, and conversion witness.

#### Scenario: Correct identity domains are accepted
r[newtypes.durable-identities.types.correct]
- GIVEN a durable reference whose fields use their declared identity types
- WHEN finite metadata and unpersist admission are evaluated
- THEN each comparison occurs only within the matching identity domain.

#### Scenario: Cross-domain substitution does not compile
r[newtypes.durable-identities.types.exchanged]
- GIVEN a program that supplies a closure identity as a type identity, or another mismatched domain
- WHEN Rust or Verus type checking evaluates the program
- THEN the program is rejected before semantic admission runs.

### Requirement: Invalid Raw Identities Remain Expressible
r[newtypes.durable-identities.invalid-inputs] Each identity structure MUST retain the complete raw `u64` value domain, including zero, so malformed finite metadata remains expressible and rejectable by explicit admission predicates.

#### Scenario: Zero identity is represented and rejected
r[newtypes.durable-identities.invalid-inputs.zero]
- GIVEN a durable reference with a zero raw value in a required identity field
- WHEN finite metadata admission is evaluated
- THEN the value is represented successfully and the reference is rejected by the existing validity rule.

#### Scenario: Constructor does not imply validity
r[newtypes.durable-identities.invalid-inputs.constructor]
- GIVEN any raw `u64` value
- WHEN its declared identity structure is constructed
- THEN construction alone makes no nonzero, cryptographic, storage, or migration claim.

### Requirement: Durable Reference Semantics Are Preserved
r[newtypes.durable-identities.preservation] For equivalent raw field values, the typed implementation MUST preserve finite metadata, closure availability, missing-closure reporting, unpersist type matching, and conversion-witness admission results.

#### Scenario: Matching type remains admitted
r[newtypes.durable-identities.preservation.type-match]
- GIVEN an admitted reference and request with equal `TypeHash` values and an available `ClosureHash`
- WHEN unpersist admission is evaluated
- THEN the request is accepted as before the migration.

#### Scenario: Incomplete witness remains rejected
r[newtypes.durable-identities.preservation.witness]
- GIVEN a conversion witness whose same-domain old or new identity differs from the referenced envelope
- WHEN migration admission is evaluated
- THEN the witness is rejected as before the migration.

### Requirement: Durable Identity Evidence
r[newtypes.durable-identities.evidence] The change MUST include positive semantic tests, negative malformed-input tests, and compile-fail evidence for representative cross-domain substitutions.

#### Scenario: Evidence covers semantic and structural behavior
r[newtypes.durable-identities.evidence.coverage]
- GIVEN the focused durable-reference evidence suite
- WHEN it runs
- THEN valid typed flows pass, malformed same-domain values fail admission, and mismatched domains fail to compile.

### Requirement: Durable Identity Traceability
r[newtypes.durable-identities.docs] The change MUST update canonical requirements, source and test annotations, public documentation, and accepted Cairn text for every new identity boundary.

#### Scenario: Identity requirements remain linked
r[newtypes.durable-identities.docs.traceable]
- GIVEN the completed migration
- WHEN Tracey and Cairn validation run
- THEN type, invalid-input, preservation, evidence, validation, and non-claim requirements have complete links.

### Requirement: Durable Identity Validation
r[newtypes.durable-identities.validation] The migration MUST pass focused and full Cargo tests, compile-fail checks, Clippy, full-library Verus, Tracey, proof-gap inventory, and Cairn validation without adding trusted proof boundaries.

#### Scenario: Full proof strength remains intact
r[newtypes.durable-identities.validation.verus]
- GIVEN the completed identity migration
- WHEN full-library Verus verification and proof-gap checks run
- THEN existing durable-reference properties verify and the trusted-boundary inventory does not increase.

### Requirement: Durable Identity Non-Claims
r[newtypes.durable-identities.non-claims] Identity wrappers MUST NOT be presented as cryptographic digests, collision-resistance evidence, persistence evidence, codec correctness, type-checker correctness, dependency retrieval, or migration execution.

#### Scenario: Wrapper claim stays structural
r[newtypes.durable-identities.non-claims.structural]
- GIVEN a consumer cites the typed identity evidence
- WHEN the claim is reviewed
- THEN the claim is limited to domain separation and preserved finite-model admission behavior.

### Requirement: Typed Config Expression Envelope
r[domain-primitives.typed-config.expression-envelope] The system MUST define finite typed configuration expression envelopes for literals, records, variants/options, lists, references, and import metadata with explicit node identifiers, type identifiers, and bounded child references.

#### Scenario: Finite expression graph is admitted
r[domain-primitives.typed-config.expression-envelope.finite]
- GIVEN a finite expression envelope whose child references point to declared nodes and whose list/record metadata is representable
- WHEN expression-envelope admission is evaluated
- THEN the expression is accepted as a finite candidate for type admission.

#### Scenario: Dangling expression reference is rejected
r[domain-primitives.typed-config.expression-envelope.dangling-reference]
- GIVEN an expression envelope with a child, reference, or import node identifier absent from the finite expression set
- WHEN expression-envelope admission is evaluated
- THEN the expression is rejected with deterministic missing-node metadata before normalization is attempted.

### Requirement: Typed Config Type Admission
r[domain-primitives.typed-config.type-admission] The system MUST define type-admission predicates that accept only expressions whose value shape matches the declared finite schema/type envelope and reject malformed, absent, or unsupported values before normalization or downstream use.

#### Scenario: Required fields type-check before normalization
r[domain-primitives.typed-config.type-admission.required-fields]
- GIVEN a record expression containing every required field with values admitted by the corresponding field type
- WHEN type admission is evaluated
- THEN the record is accepted for normalization.

#### Scenario: Type mismatch is rejected before use
r[domain-primitives.typed-config.type-admission.type-mismatch]
- GIVEN an expression whose declared type tag requires one value shape but whose finite value envelope carries a different shape
- WHEN type admission is evaluated
- THEN the expression is rejected with deterministic type-mismatch metadata before normalization or digest input selection.

#### Scenario: Missing required field is rejected explicitly
r[domain-primitives.typed-config.type-admission.missing-field]
- GIVEN a schema requiring a field that is absent from the finite record expression
- WHEN type admission is evaluated
- THEN the expression is rejected with deterministic missing-field metadata.

### Requirement: Explicit Config Failure Metadata
r[domain-primitives.typed-config.explicit-failure] The system MUST represent absence, malformed values, unsupported expression forms, digest mismatches, and import-policy failures as explicit decision metadata rather than panics, unchecked unwraps, or implicit default success.

#### Scenario: Unsupported expression form fails closed
r[domain-primitives.typed-config.explicit-failure.unsupported-form]
- GIVEN a candidate expression whose kind is outside the supported finite typed-config subset
- WHEN admission is evaluated
- THEN the decision rejects the expression with unsupported-form metadata.

#### Scenario: Optional absence cannot satisfy required value
r[domain-primitives.typed-config.explicit-failure.optional-required]
- GIVEN an optional or absent value where the schema requires a present non-optional value
- WHEN type admission is evaluated
- THEN the decision rejects the expression instead of fabricating a default.

### Requirement: Deterministic Config Normal Form
r[domain-primitives.typed-config.normal-form] The system MUST define deterministic normal-form metadata for admitted typed configuration expressions, including canonical record-field ordering, canonical variant/optional representation, canonical list order preservation, and source-name/comment independence.

#### Scenario: Record normalizes to canonical field order
r[domain-primitives.typed-config.normal-form.record-order]
- GIVEN two admitted record expressions with the same typed fields presented in different source orders
- WHEN normal-form metadata is computed
- THEN both records produce the same canonical field sequence.

#### Scenario: Source-only refactor preserves normal form
r[domain-primitives.typed-config.normal-form.source-refactor]
- GIVEN two admitted expressions that differ only by local source names, comments, whitespace, or non-semantic ordering admitted by the normalizer
- WHEN normal-form metadata is computed
- THEN the normal-form metadata is equal.

#### Scenario: Normalization only follows type admission
r[domain-primitives.typed-config.normal-form.requires-type-admission]
- GIVEN a malformed expression that fails type admission
- WHEN normalization is requested
- THEN normalization returns explicit rejection metadata and no successful normal-form claim is exported.

### Requirement: Semantic Digest Input Selection
r[domain-primitives.typed-config.semantic-digest] The system MUST define semantic digest input selection over canonical normal-form metadata and declared dependency digest sequences, rather than source text, comments, whitespace, local names, or unresolved import syntax.

#### Scenario: Textual refactor keeps digest input stable
r[domain-primitives.typed-config.semantic-digest.refactor-stable]
- GIVEN two admitted expressions with equal canonical normal-form metadata and equal dependency digest sequences
- WHEN semantic digest input selection is evaluated
- THEN the selected digest input is equal for both expressions.

#### Scenario: Dependency digest drift changes digest input
r[domain-primitives.typed-config.semantic-digest.dependency-drift]
- GIVEN two admitted expressions with equal local normal-form metadata but different declared dependency digest sequences
- WHEN semantic digest input selection is evaluated
- THEN the selected digest input differs or reports dependency-drift metadata.

### Requirement: Typed Config Import Closure
r[domain-primitives.typed-config.import-closure] The system MUST define finite import-closure admission for frozen imports, declared semantic digest checks, no computed imports, no conditional import leakage, remote import attenuation, and same-origin header forwarding metadata.

#### Scenario: Frozen import admits matching digest
r[domain-primitives.typed-config.import-closure.frozen-match]
- GIVEN an import edge whose fetched normal-form digest metadata matches the declared frozen digest
- WHEN import-closure admission is evaluated
- THEN the import edge is accepted as digest-aligned within the finite model.

#### Scenario: Frozen import mismatch fails closed
r[domain-primitives.typed-config.import-closure.frozen-mismatch]
- GIVEN an import edge whose fetched normal-form digest metadata differs from the declared frozen digest
- WHEN import-closure admission is evaluated
- THEN the import edge is rejected with digest-mismatch metadata.

#### Scenario: Computed import is rejected
r[domain-primitives.typed-config.import-closure.no-computed-import]
- GIVEN an import target whose path, origin, or header scope depends on a normalized configuration value
- WHEN import-closure admission is evaluated
- THEN the import edge is rejected with computed-import metadata.

#### Scenario: Conditional import set cannot leak state
r[domain-primitives.typed-config.import-closure.no-conditional-leakage]
- GIVEN an expression that would fetch different import sets depending on a configuration branch or value
- WHEN import-closure admission is evaluated
- THEN the expression is rejected or the full static import set is required independently of branch selection.

#### Scenario: Remote import cannot access local or environment source
r[domain-primitives.typed-config.import-closure.remote-attenuation]
- GIVEN an expression imported from a remote origin whose transitive import edge targets a local file, home path, or environment variable source
- WHEN import-closure admission is evaluated
- THEN the transitive import is rejected with remote-to-local metadata.

#### Scenario: Headers forward only within same origin
r[domain-primitives.typed-config.import-closure.same-origin-headers]
- GIVEN an import edge with header metadata and a transitive relative import within the same origin
- WHEN import-closure admission is evaluated
- THEN header forwarding is admitted only for the same-origin relative target and rejected for cross-origin targets.

### Requirement: Typed Config Semantic Refactoring
r[domain-primitives.typed-config.semantic-refactoring] The system MUST define refactoring admission for typed config expressions where equal normal-form digest inputs admit behavior-preserving source refactors, and digest-changing refactors require explicit compatibility or unresolved-dependent metadata.

#### Scenario: Equal digest input admits source refactor
r[domain-primitives.typed-config.semantic-refactoring.equal-digest]
- GIVEN a refactor step whose before and after expressions have equal semantic digest input metadata
- WHEN refactor admission is evaluated
- THEN the step is admitted as behavior-preserving within the finite typed-config model.

#### Scenario: Digest-changing refactor requires compatibility metadata
r[domain-primitives.typed-config.semantic-refactoring.digest-change]
- GIVEN a refactor step whose before and after expressions have different semantic digest input metadata
- WHEN no compatibility witness or unresolved-dependent record is declared
- THEN the step is rejected with digest-changing-refactor metadata.

### Requirement: Typed Config Evidence Coverage
r[domain-primitives.typed-config.positive] The implementation MUST include positive evidence for typed record normalization, source refactor with stable digest input, matching frozen import, and same-origin relative import header metadata.

#### Scenario: Positive fixtures cover accepted behavior
r[domain-primitives.typed-config.positive.fixtures]
- GIVEN the typed-config primitive test suite
- WHEN positive fixtures run
- THEN they cover accepted type admission, canonical normalization, stable digest input, frozen-import match, and same-origin header forwarding.

### Requirement: Typed Config Negative Coverage
r[domain-primitives.typed-config.negative] The implementation MUST include negative evidence for type mismatch, missing required field, unsupported expression form, forbidden computed import, conditional import leakage, remote-to-local or remote-to-environment import, cross-origin header forwarding, frozen digest mismatch, and digest-changing refactor without compatibility.

#### Scenario: Negative fixtures cover failure modes
r[domain-primitives.typed-config.negative.fixtures]
- GIVEN the typed-config primitive test suite
- WHEN negative fixtures run
- THEN they reject every declared failure mode with deterministic metadata rather than panicking or silently accepting malformed input.

### Requirement: Typed Config Documentation Sync
r[domain-primitives.typed-config.docs] The implementation MUST update Trellis requirement, README/module-catalog, and accepted Cairn spec documentation for any public typed-config module or requirement IDs that land in source.

#### Scenario: Requirement IDs are traceable
r[domain-primitives.typed-config.docs.traceable]
- GIVEN the typed-config module lands in `src/`
- WHEN Tracey and documentation catalog checks run
- THEN the relevant `docs/requirements.md`, README/catalog, and Cairn spec entries expose matching requirement IDs and public module descriptions.

### Requirement: Typed Config Validation Evidence
r[domain-primitives.typed-config.final-validation] The change MUST capture focused validation evidence for Cargo tests, Verus verification as needed for the new module or full library, Tracey uncovered/untested checks, and Cairn validation/gates before archive.

#### Scenario: Final validation names commands and outcomes
r[domain-primitives.typed-config.final-validation.evidence]
- GIVEN the change is ready to archive
- WHEN final validation evidence is inspected
- THEN it names the commands run, their outcomes, and any scoped limitations or blockers.

### Requirement: Typed Config Non-Claim Boundary
r[domain-primitives.typed-config.non-claims] The typed-config primitive MUST state that it does not prove Dhall parser correctness, full Dhall language compatibility, network security, filesystem security, environment secrecy, Nickel evaluation, cryptographic hash collision resistance, runtime import fetching, cache correctness, or downstream system correctness.

#### Scenario: Digest boundary is explicit
r[domain-primitives.typed-config.non-claims.digest-boundary]
- GIVEN semantic digest metadata is accepted by the typed-config primitive
- WHEN a reviewer reads the exported claim
- THEN the claim is limited to deterministic digest input selection and does not assert collision resistance or hash implementation correctness.

#### Scenario: Import policy is a finite model
r[domain-primitives.typed-config.non-claims.import-boundary]
- GIVEN import-closure admission succeeds in the typed-config primitive
- WHEN a reviewer reads the exported claim
- THEN the claim is limited to the declared finite import metadata and does not assert actual HTTP, filesystem, environment, CORS, credential, or cache behavior.
