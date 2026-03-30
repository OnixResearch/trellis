# Proptest Hardening Plan

## Current State

| Metric | Count |
|--------|-------|
| Total cargo tests | 2,727 |
| Unit tests (`fn test_*`) | 1,283 |
| Proptests (`fn prop_*`) | 1,440 (199 cross-module in properties.rs, 1,241 per-module) |
| Modules with proptest blocks | 216/219 (missing: algebraic, pointwise, seq_fold — proof-only) |
| Cross-module properties (properties.rs) | 199 proptests covering 94 modules |
| Modules NOT in properties.rs | 125 |
| `assume()` in proof code | 2 (topo_sort, reachability) |
| `no-ensures` exec fns | 2 (nix_closure, settlement) |
| `external_body` exec fn | 1 (nix_version::compare_versions) |

## Priority Tiers

### P0 — Safety nets for unverified code

These items have verification gaps. Proptests are the only defense.

**1. `assume()` soundness oracles** (2 modules)
- `topo_sort.rs`: `assume(is_topo_order_spec(em, n, om))` — the exec fn produces a topological order but Verus can't prove it matches spec. Add proptests:
  - Output is a permutation of 0..n
  - Every edge (u,v) has order[u] < order[v]
  - Random DAGs of size 2..20
- `reachability.rs`: `assume(reachable_fuel(edges, src, dst, fuel))` — BFS finds path but Verus can't prove it. Add proptests:
  - If reachable returns true, reconstruct path and verify each edge exists
  - If returns false on small graphs, brute-force confirm no path exists
  - Random graphs of size 3..15

**2. `no-ensures` exec-spec agreement** (2 modules)
- `settlement::is_valid_settlement` — exec uses `(usize,usize,u64)`, spec uses `(int,int,u64)`. Add proptests:
  - Generate obligations with party indices in 0..n, amounts in 1..1000
  - Assert `is_valid_settlement(orig, settle, n)` matches hand-computed positions-sum-zero
  - Edge: single-party, self-transfer, all-zero amounts
- `nix_closure::generic_closure` — outside verus!, uses dyn Fn + HashSet. Add proptests:
  - Closure over small sets (adjacency lists of 5..20 nodes)
  - Result is superset of input
  - Result is closed (applying fn to result adds nothing new)
  - Idempotence: closure(closure(x)) == closure(x)

**3. `external_body` exec fn** (1 fn)
- `nix_version::compare_versions` — exec loop vs recursive spec. Already has 4 proptests but add:
  - Transitivity: if cmp(a,b) <= 0 and cmp(b,c) <= 0 then cmp(a,c) <= 0
  - Concrete Nix version corpus (scrape nixpkgs for real version strings)
  - Large version strings (50+ components) — stress the fuel bound
  - Edge: empty strings, all-separator strings, numeric overflow versions

### P1 — Low-coverage modules

40 modules have only 1-3 proptests. Each needs 3-5 more to cover the key properties.

**Batch 1: Raft consensus primitives** (13 modules, 3 proptests each → add ~40)
- `vote_tally`: add quorum threshold sweep, tie-breaking, remove-then-recount
- `pre_vote_tally`: mirror vote_tally proptests for pre-vote phase
- `election_timer`: randomized timeout in range, reset idempotence
- `election_safety_proof`: two-winner impossibility with random vote patterns
- `check_quorum`: active-set size vs quorum threshold boundary
- `follower_commit`: commit advance monotonicity under message reordering
- `follower_redirect`: redirect preserves client request identity
- `leader_append_only`: append-only property under concurrent proposals
- `request_forward`: forward-then-apply == direct-apply
- `compaction_fence`: safe compact index never exceeds applied index
- `commit_advance`: advance monotonicity, no-op advance is identity
- `ghost_history`: append preserves single-vote-per-term across random histories
- `log_invariant_inductive`: LMP preservation under random append sequences

**Batch 2: Job/scheduling primitives** (8 modules, 1-3 proptests each → add ~30)
- `job_cancel`: cancel is idempotent, cancelled + retry = still cancelled
- `priority_inversion`: boosted priority >= original, ceiling protocol bounds
- `fast_backtrack`: backtrack finds same solution as exhaustive search on small inputs
- `layout_constraint`: constraint satisfaction is monotonic in available space
- `entry_type`: type classification is exhaustive (every u8 maps to exactly one type)
- `node_lifecycle`: all terminal states are absorbing, all transitions are reversible or not
- `election_init`: initial state satisfies all invariants
- `noop_commit`: noop commit index advances monotonically

**Batch 3: Data structure modules** (6 modules → add ~25)
- `bloom`: false positive rate within theoretical bounds (for small sets)
- `consistent_hash`: add/remove node affects O(1/n) keys, monotonic
- `gossip`: protocol converges in O(log n) rounds for small clusters
- `joint_consensus`: both old and new quorum must agree
- `session_tree`: insert-lookup roundtrip, prefix queries
- `trace_relation`: composition is associative, identity is neutral

### P2 — Cross-module property expansion

125 modules have no cross-module proptests in properties.rs. Group by domain:

**Group 1: Raft read/write path** (~15 properties)
- follower_read + lease_read: follower read only succeeds with valid lease
- append_entries_build + log: built entries match log contents
- install_snapshot + snapshot_compaction: snapshot install resets compaction state
- config_change + joint_consensus: config transition preserves quorum
- client_session + client_routing: routed request reaches correct handler
- election_jitter + election_timer: jittered timeout stays within [timeout, 2*timeout]

**Group 2: Storage/KV** (~10 properties)
- block_cache + free_space_map: cached blocks not in free map
- wal_recovery + journal: recovered state == last committed state
- write_buffer + block_cache: flushed buffer entries appear in cache
- page_checksum + page: checksum detects single-bit flip
- btree_insert + btree_delete: insert-then-delete is identity (for fresh keys)
- btree_balance + btree_insert: tree stays balanced after random insertions

**Group 3: Concurrency composition** (~10 properties)
- bounded_mpmc + mpsc_bounds: mpmc generalizes mpsc (single producer case)
- ticket_lock + lock_ordering: acquiring in order prevents deadlock (2 locks)
- rcu + epoch_reclaim: RCU grace period implies epoch advance
- hazard_pointer + ref_count: protected + zero refcount → not reclaimable
- barrier + semaphore: barrier(n) == n acquire + n release
- futex_state + spin_backoff: spin then park sequence

**Group 4: Financial** (~8 properties)
- fee + ledger: transfer with fee conserves total (fee goes to fee_account)
- escrow + double_entry: escrow fund creates matching debit/credit entries
- settlement + ledger: settled positions match ledger deltas
- balance + quota: quota check uses balance for limits

**Group 5: Graph/algorithm** (~8 properties)
- topo_sort + cycle_detect: has_cycle <==> topo_sort fails
- reachability + shortest_path: reachable <==> shortest_path < infinity
- graph_color + cycle_detect: 2-colorable ==> no odd cycles

### P3 — Stress testing and regression

**1. Boundary value proptests**
- Every module with u64 arithmetic: test with 0, 1, u64::MAX-1, u64::MAX
- Every module with division: test with divisor = 1, 2, large prime, u64::MAX
- Every overflow guard: test values just above and below the guard threshold

**2. Round-trip / inverse proptests**
Audit every exec fn pair (encode/decode, push/pop, lock/unlock, etc.):
- encode then decode == identity
- serialize then deserialize == identity
- insert then remove == original (where applicable)

**3. Regression proptests from proptest-regressions/**
10 saved regression seeds (1 each): backoff, consensus_path, fair_share,
job_retry, last_applied, mpsc_bounds, properties, rate_limit, tiling, watermark.
- Review each seed comment for the shrunk counterexample
- Add a dedicated unit test encoding the exact failing input (not reliant on seed file)
- Verify the fix is upstream (these should already pass — they caught spec bugs)

## Execution Order

| Phase | Work | Tests Added | Running Total |
|-------|------|-------------|---------------|
| P0 | Safety nets for assume/no-ensures/external_body | ~25 | ~2,752 |
| P1 Batch 1 | Raft consensus modules | ~40 | ~2,792 |
| P1 Batch 2 | Job/scheduling modules | ~30 | ~2,822 |
| P1 Batch 3 | Data structure modules | ~25 | ~2,847 |
| P2 Group 1 | Raft read/write path cross-module | ~15 | ~2,862 |
| P2 Group 2 | Storage/KV cross-module | ~10 | ~2,872 |
| P2 Group 3 | Concurrency cross-module | ~10 | ~2,882 |
| P2 Group 4 | Financial cross-module | ~8 | ~2,890 |
| P2 Group 5 | Graph/algorithm cross-module | ~8 | ~2,898 |
| P3 | Stress/boundary/regression | ~20 | ~2,918 |

**Estimated total: ~190 new proptests, bringing the suite from 2,727 to ~2,918.**

## Conventions

- Proptest functions: `prop_<module>_<property>` in per-module `#[cfg(test)]` blocks
- Cross-module proptests: `prop_<domain>_<property>` in `src/properties.rs`
- Input ranges: use the narrowest range that covers the interesting space (avoid `0..u64::MAX` when `0..10_000` suffices — faster shrinking)
- No format strings in `prop_assert!` (proptest macro limitation)
- Spec-only fns can't be called in proptests — inline the logic or use exec wrappers
