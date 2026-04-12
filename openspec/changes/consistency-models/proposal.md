## Why

verified-logic already has specs and verified primitives for quorums, vector
clocks, leases, MVCC, snapshot cuts, and causal delivery, but it does not yet
have a single OpenSpec domain for the consistency models those building blocks
are meant to support. The Jepsen consistency-models tree gives a clean scope:
it names the common models, shows the refinement edges between them, and
separates transactional, object, and session guarantees.

Adding this change gives later Verus work a shared vocabulary for histories,
sessions, visibility, arbitration, and real-time order. It also keeps the repo
from having to rediscover the differences between close neighbors like
repeatable read vs snapshot isolation, or PRAM vs causal consistency.

## What Changes

- Add a new `consistency-models` OpenSpec domain rooted in the Jepsen tree at
  `https://jepsen.io/consistency/models`.
- Capture the transactional family: strict serializability, serializability,
  repeatable read, snapshot isolation, monotonic atomic view, cursor stability,
  read committed, and read uncommitted.
- Capture the single-object and session family: linearizability, sequential
  consistency, causal consistency, PRAM, writes follow reads, monotonic reads,
  monotonic writes, and read your writes.
- Record the implication graph and availability envelope so later Rust modules
  can prove refinement relationships instead of re-documenting them ad hoc.
- Define the shared history vocabulary and source-level split that future Rust
  modules should implement for operations, transactions, sessions,
  visibility, arbitration, and real-time order.

## Capabilities

### New Capabilities

- `consistency.transactional-models`: OpenSpec requirements for 8
  transactional models
- `consistency.object-models`: OpenSpec requirements for linearizability and
  sequential consistency
- `consistency.session-models`: OpenSpec requirements for causal, PRAM, and
  session guarantees
- `consistency.shared-history-vocabulary`: Shared operation, transaction,
  session, visibility, arbitration, and real-time vocabulary plus the planned
  source/Tracey layout for implementing it
- `consistency.implication-graph`: Named refinement edges across all modeled
  levels
- `consistency.availability-envelope`: Availability classification aligned
  with Jepsen's tree

## Impact

- **Files**: `openspec/changes/consistency-models/{proposal,design,tasks}.md`,
  `.openspec.yaml`, and `specs/consistency-models/spec.md`
- **APIs**: No Rust API changes in this artifact-only step; the next
  implementation slice is planned around shared history/relations modules plus
  family-specific transactional, object, session, and refinement modules
- **Dependencies**: Uses Jepsen's published taxonomy as the source for names
  and hierarchy
- **Testing**: No build impact yet; later implementation should use Verus
  proofs plus example histories and model-level regression cases
