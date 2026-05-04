## Why

Raft is a useful next target for `verified-logic`, but it is too broad to treat
as an ordinary distributed-system implementation task. A focused spec-only Raft
child OpenSpec lets the repository define the exact finite models, safety
invariants, transition predicates, and evidence plan needed for formally verified
Raft before any runtime code or proof work begins.

This change scopes Raft to finite traces and deterministic admission predicates:
terms, nodes, logs, elections, votes, AppendEntries, RequestVote, commit index,
applied state-machine prefixes, membership epochs, snapshots, and recovery
metadata. It explicitly avoids network delivery, timing, persistence durability,
performance, real cluster operation, byzantine faults, cryptographic identity,
client linearizability beyond the modeled log/state-machine safety boundary, and
liveness except where a later change declares precise fairness assumptions.

## What Changes

- Add a spec-only `add-raft-consensus-primitives` child OpenSpec under the
  domain-primitives umbrella.
- Define exhaustive finite Raft model requirements for node state, terms, logs,
  voting, election/leader safety, AppendEntries, log matching, commit rules,
  leader completeness, state-machine safety, snapshots, membership epochs,
  crash/restart metadata, message envelopes, traces, invariant bundles, and
  deterministic admission/rejection.
- Leave implementation, Verus proofs, Tracey coverage, tests, and evidence as
  future tasks.

## Capabilities

### New Capabilities

- `domain-primitives.raft-consensus.model`: finite Raft universe and validity
  model.
- `domain-primitives.raft-consensus.node-state`: follower/candidate/leader state
  envelopes.
- `domain-primitives.raft-consensus.term`: monotone term and current-term update
  predicates.
- `domain-primitives.raft-consensus.log-entry`: log-entry identity and ordering
  predicates.
- `domain-primitives.raft-consensus.log-prefix`: prefix, append, truncation, and
  no-gap log predicates.
- `domain-primitives.raft-consensus.vote`: one-vote-per-term and vote-grant
  predicates.
- `domain-primitives.raft-consensus.request-vote`: RequestVote admission.
- `domain-primitives.raft-consensus.election-safety`: at-most-one leader per
  term.
- `domain-primitives.raft-consensus.append-entries`: AppendEntries admission and
  conflict metadata.
- `domain-primitives.raft-consensus.log-matching`: equal index/term implies
  equal prefix.
- `domain-primitives.raft-consensus.commit`: commit-index admission.
- `domain-primitives.raft-consensus.leader-completeness`: committed entries are
  present in later leaders.
- `domain-primitives.raft-consensus.state-machine-safety`: applied command
  agreement.
- `domain-primitives.raft-consensus.snapshot`: finite snapshot installation and
  log-prefix compaction metadata.
- `domain-primitives.raft-consensus.membership`: finite configuration and joint
  consensus quorum metadata.
- `domain-primitives.raft-consensus.trace`: finite transition trace and invariant
  preservation envelopes.
- `domain-primitives.raft-consensus.admission`: deterministic Raft admission and
  rejection envelopes.

## Impact

- **Files**: OpenSpec artifacts now; future implementation may add `src/raft_*`,
  `src/consensus_*`, `docs/requirements.md`, tests, proof modules, and evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new dependencies.
- **Testing**: strict OpenSpec validation now; future implementation must run
  focused tests, Verus, Tracey, and final repo validation.

## Out of Scope

- implementing Raft modules now
- proving wall-clock election timeouts, fairness, availability, throughput,
  performance, scheduling, network delivery, transport ordering, storage
  durability, fsync semantics, real cluster operation, client retry behavior,
  byzantine behavior, cryptographic authentication, side-channel resistance, or
  external system correctness
- claiming full linearizability beyond finite modeled log/state-machine safety
  without a later focused client-operation OpenSpec
- liveness proofs except under a later OpenSpec that declares exact finite
  fairness and delivery assumptions
