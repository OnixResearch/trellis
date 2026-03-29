## Why

The library has 20+ Raft consensus modules, each verified in isolation:
election, ballot, term_authority, leader_append_only, append_entries_check,
log_conflict, match_index, commit_advance, follower_commit, last_applied,
noop_commit, read_index, lease_read, heartbeat, check_quorum,
state_machine_safety, leader_completeness, ghost_history, etc.

Cross-module properties exist but are pairwise — they link adjacent
modules (e.g., last_applied bounded by commit, heartbeat resets prevent
election, lease_read implies read_index safe). No proof chains the
full write path (client write → leader append → replicate → commit →
apply) or the full read path (client read → authority check → commit
check → serve). These end-to-end compositions are where the real
safety guarantees live.

Without path composition proofs, you can verify each step is correct
but you can't mechanically confirm the steps compose — that the output
of one step satisfies the input preconditions of the next. A bug at
a module boundary (mismatched invariants, dropped preconditions) would
be invisible.

## What Changes

A new module `consensus_path.rs` that chains existing specs into
end-to-end composition proofs for the write path and read path.
No new exec functions — only proof fns and cross-module properties
connecting existing specs.

**Write path composition** (5 stages):
1. Leader appends entry → leader_append_only invariant holds
2. Leader builds AppendEntries → append_entries_build produces valid RPC
3. Follower checks + resolves conflicts → append_entries_check + log_conflict
4. Leader advances commit → commit_advance with quorum
5. Follower applies → last_applied advances through committed entries

**Read path composition** (3 stages):
1. Leader confirms authority → read_index quorum check OR lease_read validity
2. Commit check → read_safe_at confirms commit index covers read index
3. Serve → state_machine_safety guarantees all servers agree on applied entries

## Capabilities

### New Capabilities
- `write-path-composition`: End-to-end proof that a client write, once
  committed, is applied identically on all servers
- `read-path-composition`: End-to-end proof that a linearizable read
  sees all committed writes

### Modified Capabilities
- None — uses only existing module specs

## Impact

- **Files**: 1 new src/ module, src/lib.rs addition, properties.rs additions
- **APIs**: ~10 proof fns, 0 exec fns
- **Dependencies**: Existing consensus module specs only
- **Testing**: Verus verification + proptest for concrete scenarios
