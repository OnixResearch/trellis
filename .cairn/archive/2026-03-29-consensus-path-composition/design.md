## Context

20+ individual Raft modules, each with spec fns, exec fns, and
intra-module proofs. Pairwise cross-module properties in properties.rs.
No end-to-end path proofs.

Key specs to chain:

**Write path:**
- `leader_append_only::leader_log_invariant_spec` — log only grows
- `append_entries_build::build_append_entries_spec` — constructs RPC
- `append_entries_check::check_append_entries_spec` — validates prev
- `log_conflict::find_conflict_spec` + `resolve_conflict` — repair follower
- `match_index::update_match_index_spec` — leader tracks replication
- `commit_advance::advance_commit_index_spec` — leader commits at quorum
- `follower_commit::follower_update_commit_spec` — follower learns commit
- `last_applied::advance_last_applied_spec` — apply to state machine

**Read path:**
- `read_index::confirm_read_authority_spec` — quorum ack
- `read_index::read_safe_at_spec` — commit covers read
- `lease_read::is_lease_valid_spec` — alternative authority
- `lease_read::lease_read_safe_spec` — lease + commit
- `state_machine_safety::commit_safety_spec` — same entry at same index

## Goals / Non-Goals

**Goals:**
- Prove write path stages compose: output of stage N satisfies input of stage N+1
- Prove read path stages compose: authority + commit → safe read
- Prove write-then-read: committed write visible to subsequent read
- All proofs use existing spec fns — no new decision logic

**Non-Goals:**
- Liveness (eventually committed, eventually applied)
- Concurrent write ordering (that's linearizability, needs a system model)
- Network failure handling (message loss, reordering)
- Config change interaction (joint consensus adds complexity)

## Decisions

### 1. Proof-only module, no exec fns

**Choice:** `consensus_path.rs` contains only `proof fn`s inside
`verus!` blocks. No runtime code.

**Rationale:** The individual modules already have exec fns. The
composition is purely about proving that the spec-level preconditions
chain. Adding exec fns would duplicate existing code.

### 2. Chain proofs as implication sequences

**Choice:** Each proof takes the output of one stage and proves it
satisfies the precondition of the next. For example:

```
proof fn append_then_check(log, entry, prev_index, prev_term)
    requires
        leader_log_invariant_spec(old_log, log),  // stage 1 output
        log[prev_index - 1].term == prev_term,     // log has the entry
    ensures
        check_append_entries_spec(log, prev_index, prev_term)  // stage 2 input
```

**Rationale:** Implication chaining is the natural proof structure.
Each proof is small and focused. The caller (or a top-level theorem)
chains them with `proof1(); proof2(); ...`.

### 3. Concrete scenario proptests alongside formal proofs

**Choice:** Proptests that simulate the full path with random data
and check invariants at each stage.

**Rationale:** The formal proofs work at the spec level. Proptests
exercise the exec fns end-to-end, catching any spec/exec mismatch
that wasn't caught by individual module tests.

### 4. Phased: write path first, read path second

**Choice:**
1. Write path stages 1-5 (leader append → apply)
2. Read path stages 1-3 (authority → serve)
3. Write-then-read theorem
4. Proptest scenarios

**Rationale:** Write path is more complex (5 stages) and provides
the foundation. Read path builds on commit_advance output.
