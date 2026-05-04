## Coverage Index

| Requirement ID | Future module focus |
|---|---|
| `domain-primitives.raft-consensus.model` | `src/raft_*`, finite node/term/log/message/config universes |
| `domain-primitives.raft-consensus.node-state` | follower/candidate/leader state envelopes |
| `domain-primitives.raft-consensus.term` | term ordering and current-term update predicates |
| `domain-primitives.raft-consensus.log-entry` | log-entry identity and command metadata |
| `domain-primitives.raft-consensus.log-prefix` | prefix, append, truncation, and no-gap predicates |
| `domain-primitives.raft-consensus.persistent-state` | persisted term/vote/log metadata without durability claims |
| `domain-primitives.raft-consensus.volatile-state` | commit index, applied index, next/match index metadata |
| `domain-primitives.raft-consensus.vote` | one-vote-per-term and grant predicates |
| `domain-primitives.raft-consensus.request-vote` | RequestVote request/response admission |
| `domain-primitives.raft-consensus.election-safety` | at-most-one leader per term |
| `domain-primitives.raft-consensus.append-entries` | AppendEntries request/response admission |
| `domain-primitives.raft-consensus.log-matching` | equal index/term prefix agreement |
| `domain-primitives.raft-consensus.commit` | committed-index and quorum metadata |
| `domain-primitives.raft-consensus.leader-completeness` | committed entries preserved in later leaders |
| `domain-primitives.raft-consensus.state-machine-safety` | applied command agreement |
| `domain-primitives.raft-consensus.snapshot` | snapshot metadata and prefix compaction |
| `domain-primitives.raft-consensus.membership` | configuration and joint-consensus quorum metadata |
| `domain-primitives.raft-consensus.message` | finite message envelope and stale-message rejection |
| `domain-primitives.raft-consensus.trace` | finite trace transition/invariant preservation |
| `domain-primitives.raft-consensus.admission` | deterministic Raft admission/rejection composition |

## Phase 1: Spec foundation

- [x] Create spec-only Raft consensus primitive requirements and umbrella taxonomy entry. `[covers=domain-primitives.raft-consensus.model,domain-primitives.raft-consensus.node-state,domain-primitives.raft-consensus.term,domain-primitives.raft-consensus.log-entry,domain-primitives.raft-consensus.log-prefix,domain-primitives.raft-consensus.persistent-state,domain-primitives.raft-consensus.volatile-state,domain-primitives.raft-consensus.vote,domain-primitives.raft-consensus.request-vote,domain-primitives.raft-consensus.election-safety,domain-primitives.raft-consensus.append-entries,domain-primitives.raft-consensus.log-matching,domain-primitives.raft-consensus.commit,domain-primitives.raft-consensus.leader-completeness,domain-primitives.raft-consensus.state-machine-safety,domain-primitives.raft-consensus.snapshot,domain-primitives.raft-consensus.membership,domain-primitives.raft-consensus.message,domain-primitives.raft-consensus.trace,domain-primitives.raft-consensus.admission]`

## Phase 2: Future implementation

- [x] Add docs/requirements entries and Tracey IDs for the implemented finite term/vote/election-safety model slice.
- [x] Implement and verify term-ordering and current-term update primitives.
- [x] Implement and verify log-entry, log-prefix, append, truncation, and no-gap predicates.
- [x] Implement and verify persistent-state and volatile-state metadata predicates without durability claims.
- [x] Implement and verify vote-grant and RequestVote admission predicates.
- [x] Prove election safety over finite vote histories and quorum intersections.
- [x] Implement and verify AppendEntries admission, conflict metadata, and follower log-update predicates.
- [x] Prove log matching and prefix preservation across admitted transitions.
- [x] Implement and verify commit-index and quorum commit predicates.
- [x] Prove leader completeness and state-machine safety from the committed-log model.
- [x] Implement and verify snapshot metadata, snapshot installation admission, and prefix-compaction preservation.
- [x] Implement and verify configuration/membership epoch and joint-consensus quorum metadata.
- [x] Implement and verify finite message envelopes, stale-message rejection, and transition-trace admission.
- [x] Compose the Raft invariant bundle and prove preservation for every admitted transition kind.
- [x] Implement and verify deterministic Raft admission/rejection API with stable failure metadata.
- [x] Add focused positive, negative, and mutation tests for each Raft primitive and invariant.
- [x] Implement and verify OpenRaft-inspired finite operational-safety metadata primitives.
- [x] Run focused Verus, Tracey uncovered/untested, strict OpenSpec validation, and final repo validation.
- [x] Capture reviewable evidence and archive the change after all implementation tasks complete.
