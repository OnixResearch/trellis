## Why

The `aspen-cluster-handler` crate has three clusters of pure decision logic
embedded in async handler functions. These are the functions where a bug
causes real damage — a bad membership change breaks Raft quorum safety, an
unbounded peer list causes ticket bloat, a misclassified node misleads
operators during incidents.

The logic is straightforward but sits at the intersection of multiple
invariants that need to hold simultaneously. `handle_promote_learner` must
add the learner, remove the replaced node, preserve all other voters, avoid
duplicates, AND maintain quorum — five properties that are easy to state but
tedious to test exhaustively. The handler code gets this right today, but
nothing prevents a future edit from breaking one of the five while fixing
another.

verified-logic already has `quorum.rs` with majority, intersection, and
`can_remove_safely`. The membership promotion logic is the direct consumer
of those proofs — it's the next link in the chain.

## What Changes

Three new modules extracting the pure decision logic from
`aspen-cluster-handler/src/handler/`:

- **`voter_set`** — Membership promotion set operations from
  `membership.rs:handle_promote_learner`. Given a voter list, a learner to
  promote, and an optional node to replace, compute the new voter list.
  Properties: learner always present, replaced node always absent, other
  voters preserved, no duplicates, quorum preserved when previous voters
  formed a quorum.

- **`bounded_peers`** — Bootstrap peer collection from `tickets.rs`. Build
  a peer list from candidates with a hard capacity bound and self-exclusion.
  Properties: size never exceeds cap, self never appears, fill is monotonic
  (no valid candidate skipped before cap reached), result is never empty
  when at least one candidate exists.

- **`node_classify`** — Node classification from `state.rs`. Given a set of
  member IDs and an optional leader ID, classify each node as voter/learner
  and leader/follower. Properties: at most one leader, leader is always a
  voter, voter iff in member set, learner iff not voter, bounded output size,
  no duplicate node IDs.

## Capabilities

### New Capabilities
- `voter-set-promotion`: Verified voter set add/remove with quorum preservation proof
- `voter-set-replace`: Verified node replacement preserving non-replaced voters
- `bounded-peer-list`: Verified bounded collection with self-exclusion and fill monotonicity
- `node-classification`: Verified node role assignment with at-most-one-leader and voter-member equivalence

### Modified Capabilities
- `quorum.can-remove-safely`: voter_set uses this to guard replacement safety
- `quorum.is-quorum`: voter_set proves quorum preservation across promotion

## Impact

- **Files**: 3 new src/ modules, src/lib.rs additions, delta specs for consensus domain
- **APIs**: ~18 new pub fns across 3 modules
- **Dependencies**: None — uses existing `quorum.rs` proofs, pure vstd logic
- **Testing**: Verus verification + proptest property tests + oracle tests
  matching aspen-cluster-handler's existing test expectations
