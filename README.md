# verified-logic

Reusable logic building blocks with machine-checked correctness.

Every public function has a `spec fn` (mathematical definition) and an
`exec fn` whose `ensures result == spec(args)` clause is verified by
[Verus](https://verus-lang.github.io/verus/guide/). The specs are embedded
directly in `src/` — drift between spec and implementation is structurally
impossible. Requirement coverage is tracked by [Tracey](https://tracey.bearcove.eu/).

**3182 Verus proofs · 1918 tests · 913 requirements · 163 modules · 13 trusted axioms**

## How it works

```rust
verus! {

// Mathematical definition (ghost code, erased at compile time)
pub open spec fn abs_diff_spec(a: u64, b: u64) -> u64 {
    if a >= b { (a - b) as u64 } else { (b - a) as u64 }
}

// Runtime implementation — Verus checks `ensures` against the body
pub fn abs_diff(a: u64, b: u64) -> (result: u64)
    ensures result == abs_diff_spec(a, b)    // ← mechanically checked
{
    if a >= b { a - b } else { b - a }
}

// Machine-checked proof of a property
proof fn abs_diff_symmetric(a: u64, b: u64)
    ensures abs_diff_spec(a, b) == abs_diff_spec(b, a)
{}

} // verus!
```

## Modules

### Core arithmetic and data structures

| Module | What it verifies |
|---|---|
| `arithmetic` | abs diff, saturating add/sub, checked mul, gcd, div ceil |
| `bitwise` | power-of-two, bit get/set/clear/toggle |
| `bitfield` | multi-bit field extraction and insertion in u64 words |
| `bitmap` | fixed-size bit set over u64 words |
| `bounds` | range predicates, clamping |
| `collections` | sorted insert, binary search, merge sorted, partitioning, find, count |
| `conversion` | checked narrowing (u32/u16/u8), signed↔unsigned |
| `integer` | parity, alignment, log2, next-power-of-two |
| `interval` | half-open interval contains, overlap, split, adjacent, merge |
| `option` | verified Option combinators (map, and_then, or_else, flatten, zip, filter) |
| `ordering` | min, max, median, sign, compare-and-swap |
| `result` | verified Result combinators (map_ok, map_err, and_then, or_else) |
| `ring_buffer` | FIFO queue with power-of-two capacity |
| `sequence` | monotonic sequence generator with batch reservation |
| `slice` | chunk count, prefix/suffix equality, all-equal, mismatch |

### Raft consensus

| Module | What it verifies |
|---|---|
| `election` | leader election state machine |
| `election_init` | candidate election initiation |
| `election_jitter` | election jitter via Knuth multiplicative hash |
| `election_timer` | election timer reset conditions |
| `election_safety_proof` | at most one leader per term (system-level) |
| `pre_vote` | pre-vote protocol |
| `pre_vote_tally` | pre-vote response counting |
| `pre_vote_lease` | PreVote + CheckQuorum interaction |
| `priority_election` | priority-based leader election with delay and yield |
| `vote_tally` | vote response counting |
| `ballot` | vote tracking |
| `epoch` | lexicographic (term, index) comparison |
| `quorum` | majority threshold, intersection theorem, fault tolerance |
| `heartbeat` | heartbeat send/receive primitives |
| `check_quorum` | leader liveness via active peer counting |
| `term_authority` | step-down on higher term |
| `log` | append-only log with Log Matching Property |
| `log_conflict` | log conflict detection and resolution |
| `log_compaction` | prefix truncation for snapshot-based compaction |
| `log_invariant_inductive` | inductive proof of LMP preservation across transitions |
| `append_entries_build` | AppendEntries RPC construction |
| `append_entries_check` | AppendEntries consistency check |
| `append_response` | AppendEntries response dispatch |
| `match_index` | match/next index tracking for replication |
| `commit_advance` | commit index advancement |
| `commit_notify` | commit notification dispatch |
| `fast_backtrack` | accelerated log backtracking |
| `replication_mode` | per-peer FSM (Probe/Replicate/Snapshot) |
| `replication_pipeline` | pipelined replication flow control |
| `follower_commit` | follower commit index update |
| `follower_read` | follower linearizable reads via leader confirmation |
| `follower_redirect` | follower redirect primitives |
| `request_forward` | follower→leader write forwarding |
| `read_index` | read index protocol for linearizable reads |
| `lease_read` | lease-based reads |
| `noop_commit` | no-op entry for leader commitment |
| `install_snapshot` | snapshot installation |
| `snapshot_compaction` | snapshot creation and compaction trigger |
| `snapshot_transfer` | chunked snapshot transfer |
| `snapshot_integrity` | dual-hash integrity scheme |
| `config_change` | configuration change state machine |
| `joint_consensus` | joint consensus for membership changes |
| `voter_set` | voter set promotion |
| `learner_change` | learner add/remove primitives |
| `learner_readiness` | learner catch-up readiness |
| `leadership_transfer` | leadership transfer protocol |
| `witness` | witness replica primitives (vote without full log) |
| `bounded_peers` | bounded peer collection for bootstrap |
| `node_classify` | node classification for cluster reporting |
| `persistent_state` | crash recovery state management |
| `client_routing` | client request routing |
| `client_session` | exactly-once session tracking |
| `entry_type` | log entry type routing |
| `last_applied` | state machine application tracking |
| `linearizability` | linearizability specification |
| `state_machine_safety` | state machine safety theorem |
| `leader_append_only` | leader append-only invariant |
| `leader_completeness` | leader completeness theorem |
| `leader_completeness_ghost` | leader completeness with ghost state |
| `ghost_history` | ghost vote/provenance records for inductive proofs |
| `raft_step` | step decomposition for modular invariant proofs |
| `raft_timing` | timeout configuration validators |
| `trace_relation` | Raft internal state → client-visible trace |

### Capability authorization

| Module | What it verifies |
|---|---|
| `access_mode` | access mode partial order (Read ⊂ Full) |
| `authorize` | authorization composition |
| `containment` | delegation containment |
| `delegation` | delegation chain validation |
| `glob` | glob pattern matching with wildcards |
| `pattern` | set-membership pattern matching |
| `scope` | scope coverage, compound scoping |
| `subsumption` | variant subsumption |
| `token_timestamp` | UCAN token timestamp validity |

### Memory management

| Module | What it verifies |
|---|---|
| `buddy` | block sizing, buddy addressing, alignment, coalescing, order-for-size |
| `page` | virtual address decomposition, frame numbers, page alignment |
| `region` | address region contains, disjoint, adjacent, merge, split |
| `slab` | slot index/offset/address, generation handles, slab capacity |
| `watermark` | bump allocation with remaining/fits/alloc/reset |

### Financial accounting

| Module | What it verifies |
|---|---|
| `balance` | debit/credit with overflow guards |
| `double_entry` | double-entry bookkeeping invariants |
| `escrow` | fund/release/refund/dispute state machine with conservation |
| `fee` | basis-point fee computation, monotonicity, conservation |
| `journal` | append-only transaction journal with running balances |
| `ledger` | multi-account ledger with total conservation theorem |
| `settlement` | bilateral netting with positions-sum-zero theorem |
| `transfer` | value transfer between accounts, duplicate detection |

### Distributed systems

| Module | What it verifies |
|---|---|
| `causal_order` | vector-clock-based causal ordering |
| `circuit_breaker` | circuit breaker state machine |
| `consistent_hash` | consistent hash ring lookup |
| `crdt` | G-Counter, PN-Counter, LWW-Register |
| `distributed_lock` | distributed lock protocol |
| `fencing` | fencing tokens for stale-write rejection |
| `gossip` | gossip infection bounds |
| `markov` | Markov chain prediction |
| `merkle` | Merkle proof verification |
| `rwlock` | read-write lock mode transitions, mutual exclusion |
| `vector_clock` | vector clock merge, happens-before, concurrency |

### Nix package manager

| Module | What it verifies |
|---|---|
| `nix_attrset` | attribute set operations (get, has, merge, intersect, remove) |
| `nix_closure` | generic closure computation |
| `nix_list` | list operations (head, tail, elem, concat, gen_list) |
| `nix_string` | string operations (substring, length, replace) |
| `nix_version` | version comparison (same algorithm as Nix's compareVersions) |

### Systems decision logic

| Module | What it verifies |
|---|---|
| `approval` | merge gating: approval counting, stale detection |
| `backoff` | exponential backoff with cap and jitter bounds |
| `batch` | write batch limit checking and flush decisions |
| `bloom` | Bloom filter insert/query monotonicity, no false negatives |
| `bounded_range` | ranges with optional upper bound |
| `cas` | compare-and-swap condition evaluation |
| `chunk` | ceiling-division chunk arithmetic |
| `constant_time_eq` | constant-time 32-byte equality |
| `cost_tracker` | cost accumulation, budget enforcement, model selection |
| `cow` | copy-on-write ref counting, snapshot isolation |
| `drain` | graceful shutdown admission protocol |
| `drain_protocol` | drain with double-check admission |
| `env_scrub` | environment variable scrubbing for sandbox isolation |
| `eviction` | LRU cache eviction with protected entries |
| `eviction_score` | multi-factor eviction scoring |
| `expiration` | TTL calculation, lease/key expiration |
| `hash_bucket` | hash-to-bucket assignment (modular and power-of-two) |
| `kv_version` | KV entry version computation |
| `layout_constraint` | layout constraint resolution |
| `lease` | lease management with disjointness and renewal |
| `monotonic` | advancement predicates for terms, indices |
| `node_lifecycle` | node lifecycle state machine |
| `partition` | non-overlapping gap-free interval partitions |
| `peer_spec` | peer specification parsing |
| `permission` | Unix file permission bit manipulation |
| `pressure` | multi-threshold severity classification |
| `process_state` | process lifecycle state machine |
| `rate_limit` | integer token bucket rate limiting |
| `retry` | retry decision logic with budget tracking |
| `sequential` | sequential access detection, prefetch triggering |
| `signal` | signal set operations |
| `tier` | tier classification for tiered caching |
| `tiered_rate_limit` | two-tier rate limit (global + per-peer) |
| `tiling` | tiling layout: disjoint rects, containment, scroll bounds |
| `trust_level` | trust level resolution and access control |
| `two_phase` | two-phase commit decision logic |
| `type_lattice` | type conversion lattice |

### Generic proof infrastructure

| Module | What it verifies |
|---|---|
| `algebraic` | commutative, associative, idempotent, semilattice, partial order predicates |
| `pointwise` | scalar operation lifting to sequences |
| `seq_fold` | recursive counting and folding over sequences |
| `seq_set` | set-theoretic operations over integer sequences |
| `forest` | parent-pointer forest: ancestors, LCA, subtree walks |
| `session_tree` | session tree as forest + seq_set composition |
| `chain` | chain hash integrity with append-preserves proof |
| `acid` | atomicity, consistency, isolation, durability over state vectors |

### Cross-module properties

| Module | What it verifies |
|---|---|
| `properties` | 80+ proofs linking modules: signal↔bitwise agreement, permission independence, page↔arithmetic, lease↔monotonic, tier↔score monotonicity, escrow↔balance, ledger↔transfer, slab↔region, bloom↔insert monotonicity, gossip↔infection bounds, and more |

## Usage

```toml
[dependencies]
verified-logic = { git = "https://github.com/yourname/verified-logic" }
```

```rust
use verified_logic::arithmetic::abs_diff;
use verified_logic::bounds::clamp;
use verified_logic::quorum::majority;
use verified_logic::drain::can_start_op;

let d = abs_diff(10, 3);       // 7
let c = clamp(15, 0, 10);      // 10
let m = majority(5);           // 3
let ok = can_start_op(false);  // true (not draining)
```

For normal `cargo build`, the `vstd` crate erases all ghost code. No
runtime cost from specs, proofs, requires, or ensures.

## Development

```bash
nix develop                # enter devshell
cargo check                # compile (ghost code erased)
cargo test                 # 1918 tests (unit + property)
cargo clippy -- -D warnings
```

## Verification

```bash
./scripts/verify.sh        # all gates: clippy → tests → verus → coverage
./scripts/verus.sh         # verus only (3182 proofs)
./scripts/check-coverage.sh   # every pub fn has r[impl] annotation
./scripts/check-ensures.sh    # every pub fn in verus! has ensures
```

## Structure

```
src/lib.rs              ← crate root, 163 module declarations
src/<module>.rs         ← spec fn + exec fn + proof fn + #[cfg(test)] tests
docs/requirements.md    ← 913 tracey requirements
scripts/verus.sh        ← verus wrapper with nix store paths
scripts/verify.sh       ← full verification pipeline
scripts/check-*.sh      ← annotation enforcement
```

Each module follows the same pattern:

1. `pub open spec fn *_spec(...)` — mathematical definition with `r[depends]` annotation
2. `pub fn *(...)` with `ensures result == *_spec(args)` — runtime code with `r[impl]` annotation
3. `proof fn` — properties of the spec with `r[verify]` annotation
4. `#[cfg(test)]` — unit tests and proptest property tests

## Trusted items

13 functions are marked `#[verifier::external_body]` — Verus trusts the
ensures clause without checking the body. All are permanent:

- **6 cryptographic axioms** (`chain.rs`, `snapshot_integrity.rs`) — Blake3
  hash modeled as uninterpreted function with determinism and collision
  resistance axioms. This is the irreducible trust boundary between formal
  verification and cryptographic assumptions.
- **4 version comparison** (`nix_version.rs`) — the `cv_fuel` spec has 8+
  levels of nested let/if/else that exceed Z3's quantifier instantiation
  budget even at rlimit(2000). Proof bodies are preserved as documentation;
  proptest oracles verify runtime correctness.
- **3 log matching inductive** (`log_invariant_inductive.rs`) — the ensures
  clauses are only sound under the system-level Raft invariant that all
  logs were built from the same leader chain. These are trust boundaries
  encoding the global safety argument, not proof gaps.

All trusted items have proptest oracles verifying the same property at runtime.
