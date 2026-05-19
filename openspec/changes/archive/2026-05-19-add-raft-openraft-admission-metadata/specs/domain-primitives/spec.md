## MODIFIED Requirements

### Requirement: OpenRaft Log Pointer Admission Metadata

OpenRaft-inspired log-pointer primitives MUST expose deterministic admission metadata for the finite pointer lattice over purged, snapshot, applied, committed, flushed, submitted, and accepted indexes. Admission MUST return OK exactly for the existing valid pointer predicate and MUST otherwise report the first modeled pointer-order violation without claiming disk durability or storage implementation behavior.

r[domain-primitives.raft-consensus.openraft-log-pointers]

#### Scenario: First pointer-order violation is reported

- GIVEN finite OpenRaft-inspired log pointers with more than one ordering violation
- WHEN log-pointer admission metadata is evaluated
- THEN the result reports the first violation in the documented lattice order and does not mutate the pointer record

### Requirement: OpenRaft I/O Progress Admission Metadata

OpenRaft-inspired I/O progress primitives MUST expose deterministic admission metadata for flushed, submitted, and accepted cursor validity plus monotone cursor updates. Admission MUST return OK exactly when the existing I/O progress update predicate holds and MUST otherwise report the first invalid or regressing cursor without claiming fsync, crash recovery, or storage durability.

r[domain-primitives.raft-consensus.openraft-io-progress]

#### Scenario: Regressing progress cursor is rejected deterministically

- GIVEN an old and new finite I/O progress record where a cursor moves backwards
- WHEN I/O progress admission metadata is evaluated
- THEN the result rejects the update with the first regressing-cursor metadata

### Requirement: OpenRaft Vote/Log I/O Admission Metadata

OpenRaft-inspired vote/log I/O ordering primitives MUST expose deterministic admission metadata for the finite rule that log writes are admitted only after term/vote metadata for the log term has been ordered first. Admission MUST return OK exactly when the existing vote/log I/O predicate holds and MUST NOT claim fsync, crash recovery, or storage durability.

r[domain-primitives.raft-consensus.openraft-vote-log-io]

#### Scenario: Log write without prior vote metadata is rejected

- GIVEN a finite log write whose term exceeds the persisted vote/term metadata
- WHEN vote/log I/O admission metadata is evaluated
- THEN the result rejects the write with prior-term-metadata-required metadata

### Requirement: OpenRaft Effective Membership Admission Metadata

OpenRaft-inspired effective-membership primitives MUST expose deterministic admission metadata for committed/effective epoch ordering and a separate finite rollback helper for conflicting uncommitted effective membership. Admission metadata MUST return OK for ordered committed/effective metadata and MUST report older-effective metadata deterministically. Rollback evaluation MUST restore the effective epoch to the committed epoch, and the primitive MUST NOT claim joint-consensus runtime reconfiguration beyond finite metadata ordering.

r[domain-primitives.raft-consensus.openraft-effective-membership]

#### Scenario: Conflicting effective membership rollback reports committed epoch

- GIVEN finite effective-membership metadata whose uncommitted effective epoch must be rolled back
- WHEN conflicting-log rollback is evaluated
- THEN the effective epoch is restored to the committed epoch

### Requirement: OpenRaft Replication Session Admission Metadata

OpenRaft-inspired replication-session primitives MUST expose deterministic admission metadata for leader term, leader node, governing membership log index, monotone matched-index movement, and leader-log bounds. Admission MUST return OK exactly when the existing replication progress predicate holds and MUST otherwise reject stale sessions, backwards matched-index movement, or matched indexes beyond the leader log without claiming network delivery or follower storage behavior.

r[domain-primitives.raft-consensus.openraft-replication-session]

#### Scenario: Stale replication session is rejected before index checks

- GIVEN a replication progress update from a stale leader term, leader node, or membership log index
- WHEN replication-session admission metadata is evaluated
- THEN the result rejects the update as stale before evaluating matched-index movement

### Requirement: OpenRaft Read Linearizer Admission Metadata

OpenRaft-inspired read-linearizer primitives MUST expose deterministic readiness metadata for the finite rule that a read is ready exactly when local last-applied index has caught up to the read log index. Admission MUST return OK exactly when the existing read-linearizer readiness predicate holds and MUST NOT claim quorum RPC behavior, lease timing, clock drift bounds, network delivery, or end-to-end application linearizability.

r[domain-primitives.raft-consensus.openraft-read-linearizer]

#### Scenario: Read waits for local apply catch-up

- GIVEN a finite read-log index greater than the local last-applied index
- WHEN read-linearizer admission metadata is evaluated
- THEN the result rejects readiness with apply-catch-up-required metadata
