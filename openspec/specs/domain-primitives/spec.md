# domain-primitives Specification

## Purpose
TBD - created by archiving change add-raft-consensus-primitives. Update Purpose after archive.
## Requirements
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
### Requirement: Bounded Interval [r[domain-primitives.bounded-interval]]
The system MUST provide a reusable bounded-interval primitive for normalized
closed integer ranges, containment, overlap, and clamping.

- **Module:** `src/bounded_interval.rs`
- **Validity boundary:** public constructors MUST reject intervals whose lower
  bound is greater than the upper bound or whose configured endpoints overflow
  the representation.
- **Exported safety:** accepted intervals MUST satisfy `lo <= hi`; containment
  and overlap predicates MUST only report facts consistent with those endpoints;
  clamp results MUST lie inside the accepted interval.

#### Scenario: Normalized interval contains interior value [r[domain-primitives.bounded-interval.scenario.normalized-interval-contains-interior-value]]

- GIVEN an accepted interval and a value between its endpoints
- WHEN containment is checked
- THEN the value is reported contained and the interval validity predicate holds

#### Scenario: Inverted interval is rejected [r[domain-primitives.bounded-interval.scenario.inverted-interval-is-rejected]]

- GIVEN a lower endpoint greater than the upper endpoint
- WHEN the checked interval constructor is evaluated
- THEN it rejects the interval

### Requirement: Rect Grid Index [r[domain-primitives.rect-grid-index]]
The system MUST provide a reusable rectangular grid-index primitive for bounded
2D coordinates and flattened offsets.

- **Module:** `src/rect_grid_index.rs`
- **Validity boundary:** public constructors MUST reject zero dimensions where
  unsupported, dimensions whose area overflows, and coordinates outside
  `0 <= x < width` or `0 <= y < height`.
- **Exported safety:** accepted coordinates MUST flatten to a deterministic
  row-major offset derived from `y * width + x`. The checked public surface does
  not claim a verified inverse/unflatten API for arbitrary offsets.

#### Scenario: Valid coordinate flattens deterministically [r[domain-primitives.rect-grid-index.scenario.valid-coordinate-flattens-deterministically]]

- GIVEN a valid grid and coordinate inside width and height bounds
- WHEN the flattened offset is computed
- THEN the offset equals `y * width + x`

#### Scenario: Out-of-range coordinate is rejected [r[domain-primitives.rect-grid-index.scenario.out-of-range-coordinate-is-rejected]]

- GIVEN an x or y coordinate outside the grid dimensions
- WHEN coordinate admission is checked
- THEN the coordinate is rejected

### Requirement: GIS Coordinate Envelope [r[domain-primitives.gis-coordinate-envelope]]
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

#### Scenario: Valid coordinate is admitted [r[domain-primitives.gis-coordinate-envelope.scenario.valid-coordinate-is-admitted]]

- GIVEN a scaled latitude and longitude inside the configured bounds
- WHEN the coordinate constructor is evaluated
- THEN it returns an admitted coordinate whose spec predicate holds

#### Scenario: Out-of-range coordinate is rejected [r[domain-primitives.gis-coordinate-envelope.scenario.out-of-range-coordinate-is-rejected]]

- GIVEN a latitude, longitude, or projected component outside the configured bounds
- WHEN the coordinate constructor is evaluated
- THEN it rejects the coordinate without producing a valid wrapper

### Requirement: GIS Bounding Box [r[domain-primitives.gis-bounding-box]]
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

#### Scenario: Normalized box contains interior point [r[domain-primitives.gis-bounding-box.scenario.normalized-box-contains-interior-point]]

- GIVEN a valid bounding box and a coordinate inside both axis ranges
- WHEN containment is checked
- THEN the point is reported contained

#### Scenario: Inverted box is rejected [r[domain-primitives.gis-bounding-box.scenario.inverted-box-is-rejected]]

- GIVEN minimum coordinates greater than maximum coordinates on any axis
- WHEN the checked bounding-box constructor is evaluated
- THEN the constructor rejects the box

### Requirement: GIS Tile Index [r[domain-primitives.gis-tile-index]]
The system MUST provide a GIS tile-index primitive for bounded map-tile
addressing and parent/child refinement.

- **Module:** `src/gis_tile_index.rs`
- **Composes:** `src/rect_grid_index.rs` (`rect_grid_valid_spec`, `rect_grid_coord_valid_spec`, `rect_grid_flatten_spec`)
- **Validity boundary:** zoom, x, and y indices MUST be checked against the
  configured maximum zoom and the `0 <= index < 2^zoom` tile range.
- **Exported safety:** accepted child tiles MUST refine to a valid parent tile at
  every lower zoom, and computed parent indices MUST remain in range.

#### Scenario: Child tile refines to parent [r[domain-primitives.gis-tile-index.scenario.child-tile-refines-to-parent]]

- GIVEN a valid tile index at zoom `z > 0`
- WHEN the parent tile is computed
- THEN the parent is valid at zoom `z - 1` and contains the child in the discrete tile hierarchy

#### Scenario: Tile coordinates beyond zoom range are rejected [r[domain-primitives.gis-tile-index.scenario.tile-coordinates-beyond-zoom-range-are-rejected]]

- GIVEN an x or y index outside the range for its zoom level
- WHEN tile admission is checked
- THEN the tile is rejected

### Requirement: GIS Linestring Topology [r[domain-primitives.gis-linestring-topology]]
The system MUST provide a GIS linestring-topology primitive for discrete segment
chains.

- **Module:** `src/gis_linestring_topology.rs`
- **Composes:** `src/gis_coordinate_envelope.rs` (`gis_coordinate_valid_spec`), `src/gis_bounding_box.rs` (`gis_bounding_box_contains_spec`)
- **Validity boundary:** public constructors MUST reject empty chains, degenerate
  consecutive points, and unsupported self-intersections in the first-suite
  discrete model.
- **Exported safety:** accepted linestrings MUST have contiguous non-degenerate
  segments and MUST satisfy the chosen simple-topology predicate.

#### Scenario: Non-degenerate segment chain is accepted [r[domain-primitives.gis-linestring-topology.scenario.non-degenerate-segment-chain-is-accepted]]

- GIVEN a sequence of at least two coordinates with no duplicate consecutive points and no unsupported intersection
- WHEN linestring topology is checked
- THEN the linestring is accepted and every segment is valid

#### Scenario: Degenerate or self-intersecting chain is rejected [r[domain-primitives.gis-linestring-topology.scenario.degenerate-or-self-intersecting-chain-is-rejected]]

- GIVEN duplicate consecutive points or an unsupported crossing segment
- WHEN linestring topology is checked
- THEN the linestring is rejected
### Requirement: ML Tensor Shape [r[domain-primitives.ml-tensor-shape]]
The system MUST provide an ML tensor-shape primitive for exact finite rank-1 dimensions
and flattened-index safety.

#### Scenario: Valid multi-index maps into buffer [r[domain-primitives.ml-tensor-shape.scenario.valid-multi-index-maps-into-buffer]]

- GIVEN a valid finite rank-1 tensor shape and an index within its dimension
- WHEN the flattened offset is computed
- THEN the offset is less than the tensor element count

#### Scenario: Invalid rank or index is rejected [r[domain-primitives.ml-tensor-shape.scenario.invalid-rank-or-index-is-rejected]]

- GIVEN a rank mismatch, zero dimension, or out-of-bounds index
- WHEN tensor-shape admission is checked
- THEN the request is rejected

### Requirement: ML Label Encoding [r[domain-primitives.ml-label-encoding]]
The system MUST provide an ML label-encoding primitive for declared vocabulary
membership and class-index roundtrips.

#### Scenario: Declared label maps to stable index [r[domain-primitives.ml-label-encoding.scenario.declared-label-maps-to-stable-index]]

- GIVEN a finite vocabulary with unique labels
- WHEN a declared label is encoded and decoded
- THEN the decoded label equals the original label

#### Scenario: Unknown label is rejected [r[domain-primitives.ml-label-encoding.scenario.unknown-label-is-rejected]]

- GIVEN a label absent from the declared vocabulary
- WHEN label encoding is requested
- THEN the label is rejected

### Requirement: ML Dataset Split [r[domain-primitives.ml-dataset-split]]
The system MUST provide an ML dataset-split primitive for deterministic disjoint
train, validation, and test partitions over bounded dataset indices.

#### Scenario: Split partitions are disjoint [r[domain-primitives.ml-dataset-split.scenario.split-partitions-are-disjoint]]

- GIVEN a valid dataset size and split plan
- WHEN partitions are produced
- THEN no dataset index appears in more than one partition

#### Scenario: Invalid split bounds are rejected [r[domain-primitives.ml-dataset-split.scenario.invalid-split-bounds-are-rejected]]

- GIVEN split sizes whose sum overflows or exceeds the dataset size
- WHEN split admission is checked
- THEN the split is rejected

### Requirement: ML Batch Iterator [r[domain-primitives.ml-batch-iterator]]
The system MUST provide an ML batch-iterator primitive for bounded contiguous
batch ranges over dataset indices.

#### Scenario: Batches cover dataset range without overlap [r[domain-primitives.ml-batch-iterator.scenario.batches-cover-dataset-range-without-overlap]]

- GIVEN a valid dataset size and positive batch size
- WHEN batches are enumerated
- THEN each index in the dataset range appears in exactly one batch

#### Scenario: Invalid batch size is rejected [r[domain-primitives.ml-batch-iterator.scenario.invalid-batch-size-is-rejected]]

- GIVEN a zero batch size or overflowing range computation
- WHEN batch iteration is requested
- THEN the request is rejected

### Requirement: ML Feature Normalization [r[domain-primitives.ml-feature-normalization]]
The system MUST provide an ML feature-normalization primitive for bounded feature
scaling and missing-value rejection.

#### Scenario: Bounded feature normalizes into target interval [r[domain-primitives.ml-feature-normalization.scenario.bounded-feature-normalizes-into-target-interval]]

- GIVEN valid feature metadata with non-zero span and a feature value inside the metadata range
- WHEN normalization is performed
- THEN the normalized value lies within the configured target interval

#### Scenario: Missing value or invalid span is rejected [r[domain-primitives.ml-feature-normalization.scenario.missing-value-or-invalid-span-is-rejected]]

- GIVEN a missing feature value or equal/inverted min/max metadata
- WHEN normalization is requested
- THEN the request is rejected

### Requirement: ML Top K [r[domain-primitives.ml-top-k]]
The system MUST provide an ML top-k primitive for deterministic bounded selection
from finite score vectors.

#### Scenario: Top k results are ordered and in range [r[domain-primitives.ml-top-k.scenario.top-k-results-are-ordered-and-in-range]]

- GIVEN a finite score vector and valid k
- WHEN top-k is computed
- THEN every returned index is in range and results are ordered by score with deterministic tie-breaking

#### Scenario: Invalid k is rejected [r[domain-primitives.ml-top-k.scenario.invalid-k-is-rejected]]

- GIVEN k equal to zero or greater than the score-vector length
- WHEN top-k admission is checked
- THEN the request is rejected

### Requirement: ML Inference Envelope [r[domain-primitives.ml-inference-envelope]]
The system MUST provide an ML inference-envelope primitive for deterministic
classification or regression result admission bounded by model metadata.

#### Scenario: Declared class result is accepted [r[domain-primitives.ml-inference-envelope.scenario.declared-class-result-is-accepted]]

- GIVEN model metadata with a finite class count and a result whose class index and score are in range
- WHEN result admission is checked
- THEN the classification result is accepted

#### Scenario: Output outside metadata is rejected [r[domain-primitives.ml-inference-envelope.scenario.output-outside-metadata-is-rejected]]

- GIVEN an unknown class index, invalid confidence score, or regression value outside the declared envelope
- WHEN result admission is checked
- THEN the result is rejected

### Requirement: ML Confusion Matrix [r[domain-primitives.ml-confusion-matrix]]
The system MUST provide an ML confusion-matrix primitive for bounded metric counts
and denominator-safe derived metrics.

#### Scenario: Valid prediction updates bounded count [r[domain-primitives.ml-confusion-matrix.scenario.valid-prediction-updates-bounded-count]]

- GIVEN valid expected and predicted labels within the matrix class count
- WHEN the confusion matrix is updated
- THEN exactly the corresponding cell count increases within bounds

#### Scenario: Zero-denominator metric is not produced as numeric [r[domain-primitives.ml-confusion-matrix.scenario.zero-denominator-metric-is-not-produced-as-numeric]]

- GIVEN a class with no positive predictions or no actual positives
- WHEN precision or recall is requested
- THEN the primitive rejects the numeric metric or returns an explicit empty metric result


### Requirement: Physics Units [r[domain-primitives.physics-units]]
The system MUST provide a physics-units primitive for dimension-safe scalar and
vector quantities.

#### Scenario: Compatible quantities combine safely [r[domain-primitives.physics-units.scenario.compatible-quantities-combine-safely]]

- GIVEN two quantities with the same dimension vector
- WHEN addition is requested
- THEN the result keeps that dimension vector

#### Scenario: Incompatible dimensions reject additive operation [r[domain-primitives.physics-units.scenario.incompatible-dimensions-reject-additive-operation]]

- GIVEN quantities with different dimension vectors
- WHEN addition or subtraction is requested
- THEN the operation is rejected

### Requirement: Physics Vector Bounds [r[domain-primitives.physics-vector-bounds]]
The system MUST provide a physics vector-bounds primitive for component-wise and
aggregate envelope admission over exact discrete vectors.

#### Scenario: Vector inside envelope is admitted [r[domain-primitives.physics-vector-bounds.scenario.vector-inside-envelope-is-admitted]]

- GIVEN vector components within every configured bound
- WHEN vector admission is checked
- THEN the vector is accepted

#### Scenario: Component outside envelope is rejected [r[domain-primitives.physics-vector-bounds.scenario.component-outside-envelope-is-rejected]]

- GIVEN any vector component outside its configured bound
- WHEN vector admission is checked
- THEN the vector is rejected

### Requirement: Physics Time Horizon [r[domain-primitives.physics-time-horizon]]
The system MUST provide a physics time-horizon primitive for bounded step-count and positive time-delta admission.

#### Scenario: Valid step count stays inside horizon [r[domain-primitives.physics-time-horizon.scenario.valid-step-count-stays-inside-horizon]]

- GIVEN a positive step size and bounded step horizon
- WHEN the step count is computed
- THEN every admitted step index is inside the configured finite horizon

#### Scenario: Invalid horizon parameters are rejected [r[domain-primitives.physics-time-horizon.scenario.overflowing-horizon-arithmetic-is-rejected]]

- GIVEN zero time-delta, zero step count, or a delta larger than the configured horizon
- WHEN horizon admission is checked
- THEN the horizon is rejected

### Requirement: Physics Integrator Step [r[domain-primitives.physics-integrator-step]]
The system MUST provide a physics integrator-step primitive for bounded-time
state advancement.

#### Scenario: Bounded step advances time [r[domain-primitives.physics-integrator-step.scenario.bounded-step-advances-time]]

- GIVEN a finite modeled state and a valid positive time delta and non-saturated finite timestamp
- WHEN one integration step is admitted
- THEN the output timestamp advances by one modeled tick and state fields remain bounded

#### Scenario: Invalid time step is rejected [r[domain-primitives.physics-integrator-step.scenario.invalid-time-step-is-rejected]]

- GIVEN a zero delta, invalid bounded state, or saturated timestamp
- WHEN integration is requested
- THEN the step is rejected

### Requirement: Physics Collision Envelope [r[domain-primitives.physics-collision-envelope]]
The system MUST provide a physics collision-envelope primitive for conservative
swept-volume overlap checks.

#### Scenario: Overlapping swept envelopes produce candidate [r[domain-primitives.physics-collision-envelope.scenario.overlapping-swept-envelopes-produce-candidate]]

- GIVEN two valid moving envelopes whose conservative swept volumes overlap
- WHEN collision admission is checked
- THEN the pair is returned as a collision candidate

#### Scenario: Separated envelopes are rejected [r[domain-primitives.physics-collision-envelope.scenario.separated-envelopes-are-rejected]]

- GIVEN two valid envelopes whose conservative swept volumes are disjoint
- WHEN collision admission is checked
- THEN the pair is rejected as non-colliding for the step

### Requirement: Physics Contact Manifold [r[domain-primitives.physics-contact-manifold]]
The system MUST provide a physics contact-manifold primitive for bounded contact
point, normal, and penetration records.

#### Scenario: Valid bounded contact record is accepted [r[domain-primitives.physics-contact-manifold.scenario.valid-bounded-contact-record-is-accepted]]

- GIVEN contact points, normals, and penetration values inside configured envelopes
- WHEN contact admission is checked
- THEN the contact manifold is accepted

#### Scenario: Invalid contact record is rejected [r[domain-primitives.physics-contact-manifold.scenario.invalid-contact-record-is-rejected]]

- GIVEN an invalid normal, unsupported contact count, or out-of-envelope penetration value
- WHEN contact admission is checked
- THEN the contact manifold is rejected

### Requirement: Physics Energy Budget [r[domain-primitives.physics-energy-budget]]
The system MUST provide a physics energy-budget primitive for non-negative energy
accounting and bounded drift.

#### Scenario: Energy-preserving transition is accepted [r[domain-primitives.physics-energy-budget.scenario.energy-preserving-transition-is-accepted]]

- GIVEN a valid pre-state and post-state whose energy difference is inside the configured drift budget
- WHEN the energy budget is checked
- THEN the transition is accepted

#### Scenario: Excess drift is rejected [r[domain-primitives.physics-energy-budget.scenario.excess-drift-is-rejected]]

- GIVEN a transition whose energy delta exceeds the configured budget
- WHEN the energy budget is checked
- THEN the transition is rejected

### Requirement: Physics Momentum Budget [r[domain-primitives.physics-momentum-budget]]
The system MUST provide a physics momentum-budget primitive for non-negative mass
and bounded momentum accounting.

#### Scenario: Momentum update inside budget is accepted [r[domain-primitives.physics-momentum-budget.scenario.momentum-update-inside-budget-is-accepted]]

- GIVEN valid masses and momentum values whose delta is inside the configured budget
- WHEN the momentum budget is checked
- THEN the update is accepted

#### Scenario: Invalid mass or momentum drift is rejected [r[domain-primitives.physics-momentum-budget.scenario.invalid-mass-or-momentum-drift-is-rejected]]

- GIVEN negative mass or momentum drift outside the configured budget
- WHEN momentum admission is checked
- THEN the update is rejected

### Requirement: Physics Non-Claims [r[domain-primitives.physics-non-claims]]
The system MUST limit physics primitives to exact discrete admission and budget predicates; it MUST NOT claim floating-point accuracy, continuous dynamics correctness, real-number differential-equation proofs, broad solver completeness, simulator fidelity, collision exactness beyond conservative interval candidates, or production physics-engine correctness.

#### Scenario: Continuous and floating-point claims are excluded [r[domain-primitives.physics-non-claims.scenario.continuous-and-floating-point-claims-are-excluded]]

- GIVEN a physics primitive result
- WHEN the result is interpreted
- THEN it is only evidence for the exact finite predicate and not for continuous or floating-point simulator correctness

### Requirement: Vision Image Buffer [r[domain-primitives.vision-image-buffer]]
The system MUST provide a machine-vision image-buffer primitive for checked
width, height, stride, channel, and backing-buffer layout.

#### Scenario: Valid pixel address is in bounds [r[domain-primitives.vision-image-buffer.scenario.valid-pixel-address-is-in-bounds]]

- GIVEN a valid image buffer and pixel coordinate/channel inside declared bounds
- WHEN the byte offset is computed
- THEN the offset is inside the backing buffer

#### Scenario: Bad stride or buffer length is rejected [r[domain-primitives.vision-image-buffer.scenario.bad-stride-or-buffer-length-is-rejected]]

- GIVEN an image layout whose stride or backing length cannot contain all rows
- WHEN the image-buffer constructor is evaluated
- THEN it rejects the layout

### Requirement: Vision ROI [r[domain-primitives.vision-roi]]
The system MUST provide a machine-vision ROI primitive for bounded regions of
interest and crop-coordinate mapping.

#### Scenario: ROI crop maps into source image [r[domain-primitives.vision-roi.scenario.roi-crop-maps-into-source-image]]

- GIVEN a valid image buffer and ROI inside the image bounds
- WHEN a crop-local coordinate is mapped to source coordinates
- THEN the source coordinate is inside the image buffer

#### Scenario: Out-of-image ROI is rejected [r[domain-primitives.vision-roi.scenario.out-of-image-roi-is-rejected]]

- GIVEN an ROI that extends outside the source image bounds
- WHEN the checked ROI constructor is evaluated
- THEN the ROI is rejected

### Requirement: Vision Convolution Window [r[domain-primitives.vision-convolution-window]]
The system MUST provide a machine-vision convolution-window primitive for safe
kernel indexing under declared border policy.

#### Scenario: Interior kernel samples are in bounds [r[domain-primitives.vision-convolution-window.scenario.interior-kernel-samples-are-in-bounds]]

- GIVEN a valid image buffer, kernel shape, and anchor such that the window lies inside the image
- WHEN window sample offsets are computed
- THEN every sample offset is inside the source image buffer

#### Scenario: Unsupported border sample is rejected [r[domain-primitives.vision-convolution-window.scenario.unsupported-border-sample-is-rejected]]

- GIVEN a kernel window that crosses the image boundary with a reject-border policy
- WHEN sampling is requested
- THEN the window is rejected

### Requirement: Vision Color Layout [r[domain-primitives.vision-color-layout]]
The system MUST provide a machine-vision color-layout primitive for declared
channel count, order, and per-channel byte offset admission.

#### Scenario: Declared channel offset is in bounds [r[domain-primitives.vision-color-layout.scenario.declared-channel-offset-is-in-bounds]]

- GIVEN a valid color layout and declared channel
- WHEN the channel offset is computed
- THEN the offset is inside the pixel stride

#### Scenario: Invalid channel layout is rejected [r[domain-primitives.vision-color-layout.scenario.invalid-channel-layout-is-rejected]]

- GIVEN duplicate channels, unknown channels, or channel offsets outside the pixel stride
- WHEN color-layout admission is checked
- THEN the layout is rejected

### Requirement: Vision Color Transform Envelope [r[domain-primitives.vision-color-transform-envelope]]
The system MUST provide a machine-vision color-transform-envelope primitive for
bounded integer color-space transform outputs.

#### Scenario: Transform output stays inside envelope [r[domain-primitives.vision-color-transform-envelope.scenario.transform-output-stays-inside-envelope]]

- GIVEN valid integer coefficients, input channel bounds, and output bounds
- WHEN the transform envelope is checked
- THEN every admitted output channel lies inside the declared output bounds

#### Scenario: Overflowing transform is rejected [r[domain-primitives.vision-color-transform-envelope.scenario.overflowing-transform-is-rejected]]

- GIVEN coefficients or input bounds whose transform can overflow the representation
- WHEN transform admission is checked
- THEN the transform is rejected

### Requirement: Vision Image Pyramid [r[domain-primitives.vision-image-pyramid]]
The system MUST provide a machine-vision image-pyramid primitive for bounded
multi-level image dimensions and level mapping.

#### Scenario: Pyramid levels decrease monotonically [r[domain-primitives.vision-image-pyramid.scenario.pyramid-levels-decrease-monotonically]]

- GIVEN a valid base image and pyramid scale plan
- WHEN pyramid dimensions are derived
- THEN each admitted level has positive dimensions no larger than the previous level

#### Scenario: Invalid pyramid scale is rejected [r[domain-primitives.vision-image-pyramid.scenario.invalid-pyramid-scale-is-rejected]]

- GIVEN a scale plan that yields zero dimensions or overflowing level metadata
- WHEN pyramid admission is checked
- THEN the pyramid is rejected

### Requirement: Vision Connected Components [r[domain-primitives.vision-connected-components]]
The system MUST provide a machine-vision connected-components primitive for
bounded component labels and merge/equivalence facts.

#### Scenario: Component labels stay in declared range [r[domain-primitives.vision-connected-components.scenario.component-labels-stay-in-declared-range]]

- GIVEN a valid label image and component-count bound
- WHEN labels are admitted
- THEN every non-background label is within the declared component range

#### Scenario: Invalid label or merge is rejected [r[domain-primitives.vision-connected-components.scenario.invalid-label-or-merge-is-rejected]]

- GIVEN a label outside the declared range or a merge referencing an unknown component
- WHEN component admission is checked
- THEN the label or merge is rejected

### Requirement: Vision Detection NMS [r[domain-primitives.vision-detection-nms]]
The system MUST provide a machine-vision detection/NMS primitive for deterministic
score thresholding and non-maximum suppression.

#### Scenario: Higher-scored overlapping box suppresses lower-scored box [r[domain-primitives.vision-detection-nms.scenario.higher-scored-overlapping-box-suppresses-lower-scored-box]]

- GIVEN two valid same-class boxes whose overlap exceeds the suppression threshold and whose scores are ordered
- WHEN NMS is evaluated
- THEN only the higher-scored box remains admitted

#### Scenario: Invalid detection is rejected [r[domain-primitives.vision-detection-nms.scenario.invalid-detection-is-rejected]]

- GIVEN a detection with an invalid box, unknown class, or score outside the configured range
- WHEN detection admission is checked
- THEN the detection is rejected before suppression

### Requirement: Numeric Fixed Point Scale [r[domain-primitives.numeric-fixed-point-scale]]
The system MUST provide a numeric fixed-point scale primitive for checked scaled
integer metadata, conversion envelopes, and range preservation.

#### Scenario: Valid scaled conversion preserves envelope [r[domain-primitives.numeric-fixed-point-scale.scenario.valid-scaled-conversion-preserves-envelope]]

- GIVEN valid scale metadata and a raw integer inside the representable envelope
- WHEN the value is converted to the scaled representation
- THEN the scaled value is accepted and remains inside the declared envelope

#### Scenario: Invalid scale or conversion is rejected [r[domain-primitives.numeric-fixed-point-scale.scenario.invalid-scale-or-conversion-is-rejected]]

- GIVEN zero scale, overflowing scale metadata, or a raw value outside the representable envelope
- WHEN fixed-point admission is checked
- THEN the conversion is rejected

### Requirement: Numeric Checked Arithmetic Envelope [r[domain-primitives.numeric-checked-arithmetic-envelope]]
The system MUST provide a numeric checked-arithmetic-envelope primitive for
bounded add, subtract, multiply, and safe zero-numerator divide-admission operations.

#### Scenario: Accepted arithmetic result stays within bounds [r[domain-primitives.numeric-checked-arithmetic-envelope.scenario.accepted-arithmetic-result-stays-within-bounds]]

- GIVEN operands and an operation whose exact result is representable and inside the declared bounds
- WHEN checked arithmetic is evaluated
- THEN the result is accepted and lies inside the bounds

#### Scenario: Overflowing or invalid arithmetic is rejected [r[domain-primitives.numeric-checked-arithmetic-envelope.scenario.overflowing-or-invalid-arithmetic-is-rejected]]

- GIVEN operands whose operation overflows, leaves the declared bounds, divides by zero, or is outside the safe finite divide-admission surface
- WHEN checked arithmetic is evaluated
- THEN the operation is rejected

### Requirement: Numeric Denominator Safe Ratio [r[domain-primitives.numeric-denominator-safe-ratio]]
The system MUST provide a numeric denominator-safe-ratio primitive for explicit
non-zero denominator admission and empty/reject semantics.

#### Scenario: Non-zero denominator yields numeric ratio [r[domain-primitives.numeric-denominator-safe-ratio.scenario.non-zero-denominator-yields-numeric-ratio]]

- GIVEN a numerator and non-zero denominator whose ratio is representable
- WHEN ratio derivation is requested
- THEN a numeric ratio result is returned

#### Scenario: Zero denominator does not produce numeric ratio [r[domain-primitives.numeric-denominator-safe-ratio.scenario.zero-denominator-does-not-produce-numeric-ratio]]

- GIVEN a zero denominator
- WHEN ratio derivation is requested
- THEN the primitive rejects the request or returns an explicit empty result instead of a numeric ratio

### Requirement: Numeric Monotone Sequence [r[domain-primitives.numeric-monotone-sequence]]
The system MUST provide a numeric monotone-sequence primitive for sortedness,
lower-bound search, and range-query safety over finite three-slot sequences.

#### Scenario: Bound search returns in-range partition point [r[domain-primitives.numeric-monotone-sequence.scenario.bound-search-returns-in-range-partition-point]]

- GIVEN a finite sorted sequence and a query value
- WHEN lower-bound search is evaluated
- THEN the returned index is inside `0..=len` and partitions the sequence according to the query predicate

#### Scenario: Unsorted sequence is rejected [r[domain-primitives.numeric-monotone-sequence.scenario.unsorted-sequence-is-rejected]]

- GIVEN a sequence with an out-of-order adjacent pair
- WHEN monotone sequence admission is checked
- THEN the sequence is rejected

### Requirement: Numeric Histogram Counter [r[domain-primitives.numeric-histogram-counter]]
The system MUST provide a numeric histogram-counter primitive for bounded three-bucket indexing and count-update admission.

#### Scenario: Valid bucket update increments one count [r[domain-primitives.numeric-histogram-counter.scenario.valid-bucket-update-increments-one-count]]

- GIVEN a valid histogram and a value admitted to exactly one bucket
- WHEN the histogram is updated
- THEN exactly that bucket count increases and all other bucket counts are unchanged

#### Scenario: Invalid bucket or overflowing count is rejected [r[domain-primitives.numeric-histogram-counter.scenario.invalid-bucket-or-overflowing-count-is-rejected]]

- GIVEN a value outside all buckets or a bucket count at its maximum
- WHEN histogram update is requested
- THEN the update is rejected

### Requirement: Numeric Affine Transform Discrete [r[domain-primitives.numeric-affine-transform-discrete]]
The system MUST provide a numeric affine-transform primitive for bounded integer
matrix/vector transform envelopes.

#### Scenario: Transform output remains inside declared envelope [r[domain-primitives.numeric-affine-transform-discrete.scenario.transform-output-remains-inside-declared-envelope]]

- GIVEN valid integer coefficients, input bounds, and output bounds
- WHEN the affine transform is admitted
- THEN every produced output component lies inside the declared output envelope

#### Scenario: Overflowing transform envelope is rejected [r[domain-primitives.numeric-affine-transform-discrete.scenario.overflowing-transform-envelope-is-rejected]]

- GIVEN coefficients, offsets, or input bounds whose transform can overflow or leave the declared output envelope
- WHEN affine transform admission is checked
- THEN the transform is rejected

### Requirement: Numeric Prefix Sum [r[domain-primitives.numeric-prefix-sum]]
The system MUST provide a numeric prefix-sum primitive for bounded three-slot cumulative sums and range-sum admission.

#### Scenario: Prefix sums decompose range sum [r[domain-primitives.numeric-prefix-sum.scenario.prefix-sums-decompose-range-sum]]

- GIVEN a finite three-slot prefix tuple whose cumulative sums are representable
- WHEN a valid range sum is requested
- THEN the range sum equals the difference of the corresponding prefix sums

#### Scenario: Prefix sum overflow is rejected [r[domain-primitives.numeric-prefix-sum.scenario.prefix-sum-overflow-is-rejected]]

- GIVEN a sequence whose cumulative sum would overflow the representation
- WHEN prefix-sum construction is requested
- THEN the construction is rejected

### Requirement: Numeric Quantized Threshold [r[domain-primitives.numeric-quantized-threshold]]
The system MUST provide a numeric quantized-threshold primitive for deterministic
threshold and bucket admission over bounded values.

#### Scenario: Value maps to deterministic quantized bucket [r[domain-primitives.numeric-quantized-threshold.scenario.value-maps-to-deterministic-quantized-bucket]]

- GIVEN valid ordered three-bucket thresholds and a value inside the supported range
- WHEN quantization is evaluated
- THEN the value maps to exactly one deterministic bucket

#### Scenario: Invalid thresholds are rejected [r[domain-primitives.numeric-quantized-threshold.scenario.invalid-thresholds-are-rejected]]

- GIVEN unordered three-bucket thresholds, duplicate thresholds where disallowed, or a value outside the supported range
- WHEN threshold admission is checked
- THEN the request is rejected

### Requirement: Data Structure Bounded Array [r[domain-primitives.ds-bounded-array]]
The system MUST provide a bounded-array primitive for finite sequence length,
capacity, index admission, and point-update preservation.

#### Scenario: Valid point update changes only the target index [r[domain-primitives.ds-bounded-array.scenario.valid-point-update-changes-only-target-index]]

- GIVEN a valid bounded array and an index inside its current length
- WHEN a point update is applied
- THEN the resulting array has the same length, stores the new value at that index, and preserves every other index

#### Scenario: Out-of-range index is rejected without mutation [r[domain-primitives.ds-bounded-array.scenario.out-of-range-index-is-rejected-without-mutation]]

- GIVEN a valid bounded array and an index outside its current length
- WHEN a read or point update is requested
- THEN the request is rejected and the array state is unchanged

### Requirement: Data Structure Key Value Map [r[domain-primitives.ds-key-value-map]]
The system MUST provide a finite key/value map primitive for unique keys,
lookup, insertion, replacement, and removal semantics.

#### Scenario: Insert creates or replaces exactly one key [r[domain-primitives.ds-key-value-map.scenario.insert-creates-or-replaces-exactly-one-key]]

- GIVEN a valid finite map and a key/value pair admitted by the map domain
- WHEN the pair is inserted
- THEN lookup for that key returns the inserted value and all other keys keep their previous lookup results

#### Scenario: Remove clears only the target key [r[domain-primitives.ds-key-value-map.scenario.remove-clears-only-target-key]]

- GIVEN a valid finite map and a target key
- WHEN the target key is removed
- THEN lookup for that key is empty and lookup for every other key is unchanged

### Requirement: Data Structure FIFO Queue [r[domain-primitives.ds-fifo-queue]]
The system MUST provide a bounded FIFO queue primitive for enqueue/dequeue order,
capacity admission, and empty/full rejection.

#### Scenario: Dequeue returns oldest enqueued element [r[domain-primitives.ds-fifo-queue.scenario.dequeue-returns-oldest-enqueued-element]]

- GIVEN a valid non-empty FIFO queue built from accepted enqueue operations
- WHEN one element is dequeued
- THEN the returned element is the oldest remaining element and the queue retains the remaining elements in FIFO order

#### Scenario: Queue overflow or underflow is rejected [r[domain-primitives.ds-fifo-queue.scenario.queue-overflow-or-underflow-is-rejected]]

- GIVEN a full queue for enqueue or an empty queue for dequeue
- WHEN the invalid operation is requested
- THEN the operation is rejected and the queue state is unchanged

### Requirement: Data Structure Ring Buffer [r[domain-primitives.ds-ring-buffer]]
The system MUST provide a ring-buffer primitive for bounded circular indexing,
occupied-slot validity, and wraparound push/pop behavior.

#### Scenario: Wraparound preserves logical order [r[domain-primitives.ds-ring-buffer.scenario.wraparound-preserves-logical-order]]

- GIVEN a valid ring buffer whose head or tail is near the physical end of storage
- WHEN accepted push and pop operations wrap around storage
- THEN logical element order is preserved and every occupied logical index maps to a valid physical slot

#### Scenario: Full or empty ring operation is rejected [r[domain-primitives.ds-ring-buffer.scenario.full-or-empty-ring-operation-is-rejected]]

- GIVEN a full ring buffer for push or an empty ring buffer for pop
- WHEN the invalid operation is requested
- THEN the operation is rejected and head, tail, length, and storage are unchanged

### Requirement: Data Structure Priority Queue [r[domain-primitives.ds-priority-queue]]
The system MUST provide a bounded priority-queue primitive for priority-order
validity, deterministic tie handling, bounded push, and pop-min behavior.

#### Scenario: Pop returns deterministic minimum-priority element [r[domain-primitives.ds-priority-queue.scenario.pop-returns-deterministic-minimum-priority-element]]

- GIVEN a valid non-empty priority queue and a declared deterministic tie policy
- WHEN pop-min is requested
- THEN the returned element has minimum priority and satisfies the tie policy among all minimum-priority elements

#### Scenario: Invalid priority queue operation is rejected [r[domain-primitives.ds-priority-queue.scenario.invalid-priority-queue-operation-is-rejected]]

- GIVEN a full priority queue for push or an empty priority queue for pop
- WHEN the invalid operation is requested
- THEN the operation is rejected and the queue state is unchanged

### Requirement: Data Structure Union Find [r[domain-primitives.ds-union-find]]
The system MUST provide a union-find primitive for finite parent/rank arrays,
root discovery, union connectivity, and invalid-index rejection.

#### Scenario: Union connects two valid sets [r[domain-primitives.ds-union-find.scenario.union-connects-two-valid-sets]]

- GIVEN a valid union-find forest and two valid element indices
- WHEN the sets containing those elements are unioned
- THEN the resulting forest remains valid and the two elements have the same representative root

#### Scenario: Invalid union-find index is rejected [r[domain-primitives.ds-union-find.scenario.invalid-union-find-index-is-rejected]]

- GIVEN a valid union-find forest and an index outside the element range
- WHEN find or union is requested with that index
- THEN the operation is rejected and the forest state is unchanged

### Requirement: Finite Principal Envelope [r[domain-primitives.access-control-policy.principal]]
The system MUST define finite principal admission metadata for policy decision
models.

#### Scenario: Principal is declared before rule use [r[domain-primitives.access-control-policy.principal.scenario.declared-before-rule-use]]

- GIVEN finite principal metadata and a policy rule
- WHEN the rule references a principal
- THEN the principal is admitted by the declared principal envelope

### Requirement: Finite Resource Envelope [r[domain-primitives.access-control-policy.resource]]
The system MUST define finite resource admission metadata for policy decision
models.

#### Scenario: Resource is declared before authorization [r[domain-primitives.access-control-policy.resource.scenario.declared-before-authorization]]

- GIVEN finite resource metadata and an authorization request
- WHEN the request references a resource
- THEN the resource is admitted by the declared resource envelope

### Requirement: Finite Action Envelope [r[domain-primitives.access-control-policy.action]]
The system MUST define finite action admission metadata for policy decision
models.

#### Scenario: Action is declared before authorization [r[domain-primitives.access-control-policy.action.scenario.declared-before-authorization]]

- GIVEN finite action metadata and an authorization request
- WHEN the request references an action
- THEN the action is admitted by the declared action envelope

### Requirement: Policy Rule Envelope [r[domain-primitives.access-control-policy.rule]]
The system MUST define policy-rule envelopes over finite principal, resource, and
action scopes with exact effect metadata.

#### Scenario: Rule scope uses admitted values [r[domain-primitives.access-control-policy.rule.scenario.scope-uses-admitted-values]]

- GIVEN a policy-rule envelope
- WHEN the rule scope is evaluated
- THEN every principal, resource, and action referenced by the rule is admitted by its declared envelope

### Requirement: Grant and Denial Decision Metadata [r[domain-primitives.access-control-policy.grant-denial]]
The system MUST define deterministic grant/denial decision metadata for admitted
authorization requests.

#### Scenario: Decision records matched rule metadata [r[domain-primitives.access-control-policy.grant-denial.scenario.records-matched-rule-metadata]]

- GIVEN an admitted authorization request and finite policy rules
- WHEN a grant or denial decision is produced
- THEN the decision metadata identifies the matched admitted rule or the deterministic no-match result

### Requirement: Conflict Resolution Envelope [r[domain-primitives.access-control-policy.conflict]]
The system MUST define deterministic conflict-resolution envelopes for overlapping
finite policy rules.

#### Scenario: Overlapping rules resolve deterministically [r[domain-primitives.access-control-policy.conflict.scenario.overlapping-rules-resolve-deterministically]]

- GIVEN overlapping admitted policy rules for the same principal, resource, and action
- WHEN conflict resolution is evaluated
- THEN the result follows declared priority or tie-break metadata deterministically

### Requirement: Bounded Delegation Envelope [r[domain-primitives.access-control-policy.delegation]]
The system MUST define bounded delegation metadata over finite principals,
resources, actions, scopes, and chain-depth envelopes.

#### Scenario: Delegation states scope and bound [r[domain-primitives.access-control-policy.delegation.scenario.states-scope-and-bound]]

- GIVEN delegation metadata between admitted principals
- WHEN the delegation is used in an authorization model
- THEN it states the admitted resource/action scope and finite depth or chain bound

### Requirement: Authorization Admission Envelope [r[domain-primitives.access-control-policy.authorization]]
The system MUST define deterministic authorization admission results for policy
metadata, including invalid principal, resource, action, rule, conflict, and
delegation shapes.

#### Scenario: Rejects undeclared principal deterministically [r[domain-primitives.access-control-policy.authorization.scenario.rejects-undeclared-principal-deterministically]]

- GIVEN authorization metadata with a request principal outside the declared principal envelope
- WHEN admission is evaluated
- THEN the metadata is rejected deterministically without claiming cryptographic authentication or whole-system security

### Requirement: Config Key Envelope [r[domain-primitives.configuration-feature-flag.key]]
The system MUST provide finite config-key envelopes that record key identifiers,
optional namespaces, and deterministic known/unknown-key admission metadata.

#### Scenario: Unknown key rejected explicitly [r[domain-primitives.configuration-feature-flag.key.scenario.unknown-key-rejected-explicitly]]

- GIVEN a finite admitted config-key set
- WHEN a candidate key is not in the admitted set
- THEN the config-key primitive returns an explicit unknown-key decision without consulting an external store

### Requirement: Typed Value Envelope [r[domain-primitives.configuration-feature-flag.typed-value]]
The system MUST provide finite typed-value envelopes that bind declared type tags
to admitted value shapes and reject type/value mismatches deterministically.

#### Scenario: Type mismatch rejected before use [r[domain-primitives.configuration-feature-flag.typed-value.scenario.type-mismatch-rejected-before-use]]

- GIVEN a declared config value type and candidate finite value envelope
- WHEN the candidate value shape does not match the declared type tag
- THEN the typed-value primitive rejects the value or exposes an explicit type-mismatch decision

### Requirement: Default Value Metadata [r[domain-primitives.configuration-feature-flag.default]]
The system MUST provide default-value metadata that ties each default to a
declared finite key envelope and compatible typed-value envelope.

#### Scenario: Default must match declared key and type [r[domain-primitives.configuration-feature-flag.default.scenario.default-must-match-declared-key-and-type]]

- GIVEN default metadata for a config key
- WHEN the key is unknown or the default value does not match the declared type
- THEN the default primitive rejects the metadata without mutating the candidate config set

### Requirement: Override Precedence Envelope [r[domain-primitives.configuration-feature-flag.override-precedence]]
The system MUST provide deterministic finite override-precedence envelopes for
admitted configuration sources and explicit tie/duplicate-source decisions.

#### Scenario: Highest admitted precedence selected deterministically [r[domain-primitives.configuration-feature-flag.override-precedence.scenario.highest-admitted-precedence-selected-deterministically]]

- GIVEN finite override candidates for the same admitted key
- WHEN the precedence envelope orders the candidate sources without ties
- THEN the override-precedence primitive selects the highest-precedence admitted candidate deterministically

### Requirement: Feature Gate Predicate Envelope [r[domain-primitives.configuration-feature-flag.gate]]
The system MUST provide finite feature-gate predicate envelopes that map admitted
keys, subjects, and typed values to explicit enabled/disabled/rejected decisions.

#### Scenario: Gate rejects unknown subject metadata [r[domain-primitives.configuration-feature-flag.gate.scenario.gate-rejects-unknown-subject-metadata]]

- GIVEN a feature-gate predicate over finite admitted subject metadata
- WHEN a candidate subject lacks the required admitted metadata
- THEN the feature-gate primitive returns an explicit rejection decision rather than claiming runtime rollout behavior

### Requirement: Rollout Bucket Envelope [r[domain-primitives.configuration-feature-flag.rollout-bucket]]
The system MUST provide deterministic rollout-bucket envelopes over finite
subject identifiers, bucket ranges, and declared membership predicates without
claiming randomness quality or distributed rollout consistency.

#### Scenario: Bucket range must be admitted [r[domain-primitives.configuration-feature-flag.rollout-bucket.scenario.bucket-range-must-be-admitted]]

- GIVEN finite rollout bucket metadata and a candidate subject bucket
- WHEN the bucket lies outside the admitted bucket range
- THEN the rollout-bucket primitive rejects the candidate or exposes an explicit out-of-range decision

### Requirement: Validation Decision Metadata [r[domain-primitives.configuration-feature-flag.validation-decision]]
The system MUST provide validation decision metadata that records accepted,
rejected, and reason-coded invalid configuration entries over finite admitted
keys and typed values.

#### Scenario: Invalid entry preserves decision reason [r[domain-primitives.configuration-feature-flag.validation-decision.scenario.invalid-entry-preserves-decision-reason]]

- GIVEN a finite configuration entry with unknown key, mismatched type, or invalid precedence metadata
- WHEN validation is performed by the primitive
- THEN the validation decision records the rejection reason without mutating the input entry

### Requirement: Config Admission Envelope [r[domain-primitives.configuration-feature-flag.admission]]
The system MUST provide deterministic configuration admission/rejection envelopes
that compose key, typed-value, default, override, gate, rollout, and validation
metadata over finite inputs.

#### Scenario: Admission requires all finite checks [r[domain-primitives.configuration-feature-flag.admission.scenario.admission-requires-all-finite-checks]]

- GIVEN finite configuration and feature-flag metadata
- WHEN any required key, type, default, override, gate, rollout, or validation check rejects
- THEN the config-admission primitive returns an explicit rejection decision before any runtime store or distributed flag-service claim is made
### Requirement: Constraint Variable Envelope [r[domain-primitives.constraint-solver.variable]]
The system MUST define finite constraint-variable envelopes for variable
identifiers, declared domains, and variable membership metadata.

#### Scenario: Variable references declared domain [r[domain-primitives.constraint-solver.variable.scenario.references-declared-domain]]

- GIVEN a finite constraint variable and a domain table
- WHEN the variable is admitted by the variable envelope
- THEN its declared domain identifier exists in the finite domain table

### Requirement: Constraint Domain Predicate [r[domain-primitives.constraint-solver.domain]]
The system MUST define finite domain predicates for admitted values, emptiness,
subset relationships, and membership checks.

#### Scenario: Domain membership rejects absent value [r[domain-primitives.constraint-solver.domain.scenario.rejects-absent-value]]

- GIVEN a finite domain and a candidate value
- WHEN the value is not a member of the declared domain set
- THEN the domain predicate rejects the value without claiming theory reasoning

### Requirement: Constraint Relation Envelope [r[domain-primitives.constraint-solver.relation]]
The system MUST define finite constraint-relation envelopes for constraint
identifiers, variable scopes, tuple arity, and admitted tuple metadata.

#### Scenario: Relation tuple arity matches scope [r[domain-primitives.constraint-solver.relation.scenario.tuple-arity-matches-scope]]

- GIVEN a finite relation with a declared variable scope
- WHEN a candidate tuple is checked against the relation
- THEN the relation envelope admits only tuples whose arity matches the declared scope

### Requirement: Constraint Assignment Shape [r[domain-primitives.constraint-solver.assignment]]
The system MUST define assignment predicates for partial and total assignments,
variable uniqueness, and value admission against declared domains.

#### Scenario: Assignment rejects value outside declared domain [r[domain-primitives.constraint-solver.assignment.scenario.rejects-value-outside-domain]]

- GIVEN a finite assignment for a declared variable
- WHEN the assigned value is outside the variable's declared finite domain
- THEN the assignment predicate rejects the assignment

### Requirement: Constraint Consistency Predicate [r[domain-primitives.constraint-solver.consistency]]
The system MUST define deterministic local consistency predicates for assignments
against finite constraint relations.

#### Scenario: Consistency checks assigned relation scope [r[domain-primitives.constraint-solver.consistency.scenario.checks-assigned-relation-scope]]

- GIVEN a finite relation and an assignment covering every variable in the relation scope
- WHEN the assigned tuple is not admitted by the relation envelope
- THEN the consistency predicate reports the assignment as locally inconsistent

### Requirement: Constraint Propagation Envelope [r[domain-primitives.constraint-solver.propagation]]
The system MUST define monotone propagation envelopes for finite domain narrowing,
changed-domain metadata, and contradiction detection.

#### Scenario: Propagation never expands finite domain [r[domain-primitives.constraint-solver.propagation.scenario.never-expands-finite-domain]]

- GIVEN a finite domain state and a propagated domain state
- WHEN propagation metadata is admitted
- THEN every propagated domain is a subset of its corresponding input domain

### Requirement: Constraint Witness Metadata [r[domain-primitives.constraint-solver.witness]]
The system MUST define finite witness metadata for satisfying assignments and
explicit contradictions in declared finite models.

#### Scenario: Satisfying witness carries admitted total assignment [r[domain-primitives.constraint-solver.witness.scenario.carries-admitted-total-assignment]]

- GIVEN a finite constraint model and a candidate satisfying witness
- WHEN the witness is admitted
- THEN it carries a total assignment admitted by every variable domain and locally consistent with every declared relation

### Requirement: Constraint Admission Envelope [r[domain-primitives.constraint-solver.admission]]
The system MUST define deterministic constraint admission envelopes that return
admit/reject/unknown metadata from finite variables, domains, relations,
assignments, propagation, and witness predicates.

#### Scenario: Admission rejects inconsistent assignment deterministically [r[domain-primitives.constraint-solver.admission.scenario.rejects-inconsistent-assignment-deterministically]]

- GIVEN a finite constraint model and a candidate assignment
- WHEN the assignment violates an admitted relation or declared domain
- THEN the admission envelope rejects it with deterministic metadata and does not claim solver optimality or external-engine completeness
