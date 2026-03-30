//! Verified logic building blocks.
//!
//! Every public function in this crate has machine-checked pre/postconditions
//! verified by [Verus](https://verus-lang.github.io/verus/guide/). Requirements
//! are tracked by [Tracey](https://tracey.bearcove.eu/) in `docs/requirements.md`.
//!
//! The `verus!` blocks are embedded directly in the source — Verus verifies
//! the actual implementations, not separate spec files. For normal `cargo build`,
//! the `vstd` crate erases all ghost code (specs, proofs, requires, ensures).
//!
//! ## Modules
//!
//! - `access_mode` — access mode partial order: read/write/delete/full with no-escalation
//! - `append_entries_build` — Raft leader-side AppendEntries construction: prev_log, entries, batching
//! - `append_entries_check` — Raft AppendEntries consistency: prevLogIndex/prevLogTerm validation
//! - `acid` — ACID transaction proofs: atomicity, consistency, isolation, durability
//! - `arithmetic` — abs diff, saturating ops, checked mul
//! - `backoff` — exponential backoff with cap and jitter bounds
//! - `balance` — credit/debit with overflow protection and conservation proofs
//! - `ballot` — vote tracking for Raft elections with quorum intersection safety
//! - `barrier` — thread barrier: arrive, release on Nth arrival, generation counter
//! - `bitwise` — power-of-two, bit get/set/clear/toggle
//! - `batch` — write batch limit checking and flush decisions
//! - `bitfield` — multi-bit field extraction and insertion with round-trip proofs
//! - `btree_balance` — B-tree node balance: fill factor, split/merge triggers, redistribution
//! - `bitmap` — verified fixed-size bit set using bitwise ops
//! - `bloom` — Bloom filter with no-false-negatives proof
//! - `authorize` — three-factor authorization: subsumption × scope × mode
//! - `approval` — merge gating: approval counting, stale detection, CI checks
//! - `bounded_mpmc` — bounded MPMC queue: head/tail occupancy, enqueue/dequeue, slot generation, capacity conservation
//! - `bounded_peers` — bounded peer collection: capacity cap, self-exclusion, order preservation
//! - `bounded_range` — ranges with optional upper bound (None = unbounded)
//! - `buddy` — buddy system: block sizes, buddy addresses, coalescing, parent blocks
//! - `causal_order` — causal delivery and consistent snapshots
//! - `consistent_snapshot` — consistent cuts with lattice operations (join/meet) and message consistency
//! - `containment` — delegation containment: scope + mode + subsumption
//! - `consistent_hash` — consistent hash ring with complete coverage
//! - `crdt` — G-Counter, PN-Counter, LWW-Register with merge laws
//! - `bounds` — range predicates, clamping
//! - `cas` — compare-and-swap condition evaluation
//! - `cas_loop` — CAS retry loop modeling: success/failure, ABA detection via generation counters, retry bounds
//! - `chain` — chain hash integrity with append-preserves proof
//! - `check_quorum` — leader liveness: step down when quorum contact lost
//! - `circuit_breaker` — three-state circuit breaker: closed/open/half-open
//! - `chunk` — ceiling-division chunk arithmetic, offset↔chunk mapping
//! - `client_routing` — Raft client request routing: accept/redirect/reject by node role
//! - `client_session` — Raft client session deduplication for exactly-once semantics (§8)
//! - `config_change` — Raft configuration change protocol FSM: stable↔joint transitions (§6)
//! - `collections` — sorted insert, binary search
//! - `commit_advance` — Raft commit index advancement with §5.4.2 current-term guard
//! - `commit_notify` — commit notification dispatch: newly committed ranges, proposal status
//! - `compaction_fence` — log compaction coordination with replication: safe compact index
//! - `constant_time_eq` — fixed-iteration XOR-fold comparison for 32-byte hashes
//! - `conversion` — checked narrowing (u32/u16/u8), sign conversion (i64↔u64)
//! - `cow` — copy-on-write decision logic, ref counting, snapshot isolation
//! - `delegation` — delegation chain validation: depth bounds, attenuation, Delegate gate
//! - `double_entry` — double-entry bookkeeping: balanced debits/credits, trial balance
//! - `distributed_lock` — distributed lock tokens, deadlines, expiry, backoff
//! - `drain` — drain-state admission protocol for graceful shutdown
//! - `drain_protocol` — double-check admission with recheck and notification
//! - `epoch_reclaim` — epoch-based memory reclamation: pin/unpin, three-epoch cycling, safe-to-reclaim
//! - `election` — leader election state machine: transitions, tokens, timing
//! - `election_jitter` — deterministic per-node jitter via Knuth multiplicative hash
//! - `election_safety_proof` — system-level election safety: at most one leader per term
//! - `epoch` — lexicographic (term, index) comparison for Raft elections
//! - `escrow` — conditional value holding: fund, release, refund, dispute with conservation
//! - `eviction` — LRU eviction with protected entries
//! - `eviction_score` — multi-factor eviction scoring: weighted factors × priority × tier
//! - `fast_backtrack` — Raft accelerated log backtracking: skip conflicting terms in one RPC (§5.3)
//! - `fee` — proportional fee computation: basis points, tiered, conservation
//! - `fencing` — fencing tokens for stale-write rejection
//! - `follower_read` — follower-side linearizable reads via leader commit confirmation
//! - `futex_state` — futex wait/wake state machine: idle/waiting/notified transitions
//! - `ghost_history` — ghost state for Raft inductive proofs: vote history, leader logs, entry provenance
//! - `glob` — glob pattern matching with `*` wildcards for capabilities
//! - `gossip` — gossip infection bounds and convergence
//! - `expiration` — TTL calculation, lease/key expiration, time remaining
//! - `hash_bucket` — hash-to-bucket assignment, modular and power-of-two
//! - `hazard_pointer` — hazard pointer protocol: protect, unprotect, reclaim, no-use-after-free proof
//! - `heap_index` — binary heap index arithmetic: parent/child navigation, leaf detection, roundtrip proofs
//! - `install_snapshot` — snapshot trigger and commit advancement for log compaction
//! - `integer` — parity, alignment, log2, next-power-of-two
//! - `interval` — half-open interval contains, overlap, split, merge
//! - `joint_consensus` — dual-quorum membership changes with intersection proof
//! - `journal` — append-only transaction journal with running balances and seq num monotonicity
//! - `kv_version` — KV entry version computation: create/update with saturation
//! - `layout_constraint` — tiling layout constraint resolution: satisfaction, clamping, conservation
//! - `leader_append_only` — leader log immutability invariant: only grows, never overwrites
//! - `leader_completeness` — committed entries survive elections (quorum intersection + epoch_ge)
//! - `leadership_transfer` — voluntary leadership transfer: catch-up, `TimeoutNow`, deadline
//! - `learner_change` — learner add/remove primitives: no quorum impact, idempotent ops
//! - `learner_readiness` — learner catch-up readiness for safe voter promotion
//! - `lease` — time-bounded leases with disjointness, renewal, revocation
//! - `ledger` — multi-account ledger: transfers, batch, fee routing, total conservation
//! - `settlement` — bilateral obligation netting: net positions, sum-to-zero, valid settlement
//! - `lease_read` — lease-based linearizable reads with clock drift compensation
//! - `lock_ordering` — lock acquisition ordering for deadlock prevention: strict total order, cycle freedom
//! - `lsm_level` — LSM-tree level management: target sizing, compaction triggers, amplification
//! - `log` — append-only log with Log Matching Property
//! - `match_index` — per-follower matchIndex/nextIndex tracking with monotonicity
//! - `log_conflict` — log conflict resolution: find divergence, truncate, append leader entries
//! - `markov` — Markov chain prediction: transition counting, confidence thresholds
//! - `memory_order` — memory ordering lattice: Relaxed/Acquire/Release/AcqRel/SeqCst, fence composition, load/store validity
//! - `merkle` — Merkle proof verification with hash axiom
//! - `monotonic` — advancement predicates for terms, commit indices
//! - `mpsc_bounds` — MPSC channel bounds: occupancy, capacity, send/receive, close/drain
//! - `nix_attrset` — Nix attribute set operations: merge, intersect, remove
//! - `noop_commit` — no-op entry for committing prior-term entries after election
//! - `node_classify` — node classification: voter/learner, leader/follower, batch with bounds
//! - `nix_closure` — Nix generic closure with fuel-based termination
//! - `node_lifecycle` — node status FSM: Starting/Online/Degraded/Offline
//! - `nix_list` — Nix list builtins: elem, head, tail, `gen_list`, concat
//! - `nix_string` — Nix string operations: length, substring, replace
//! - `nix_version` — Nix version comparison with component classification
//! - `option` — verified Option combinators
//! - `ordering` — min, max, median, sign, compare-and-swap
//! - `pattern` — wildcard and set-membership pattern matching for capabilities
//! - `page` — virtual address decomposition: frame number, offset, alignment
//! - `partition` — verified non-overlapping gap-free interval partitions
//! - `peer_spec` — peer specification parsing: `node_id@endpoint` format
//! - `permission` — Unix file permission bits: has/set/clear, umask
//! - `persistent_state` — Raft crash recovery: durable state, persist-before-respond, vote safety
//! - `pre_vote` — pre-vote protocol without term inflation for partitioned nodes
//! - `pre_vote_lease` — PreVote + CheckQuorum interaction: reject pre-votes when leader is active
//! - `pre_vote_tally` — pre-vote response tally: counting and quorum check for pre-election phase
//! - `pressure` — multi-threshold severity classification
//! - `priority_election` — priority-based leader election: delay, yield, candidate comparison
//! - `process_state` — process lifecycle state machine: transitions, terminal, schedulable
//! - `quota` — resource quota enforcement: remaining capacity, admission, consume/release, utilization
//! - `quorum` — majority threshold, quorum predicate, intersection theorem
//! - `raft_step` — Raft step decomposition for modular invariant proofs
//! - `raft_timing` — Raft timeout configuration validators with fail-fast composition
//! - `rate_limit` — integer token bucket rate limiting
//! - `rcu` — read-copy-update: grace periods, quiescent state detection, safe reclamation
//! - `read_index` — linearizable read confirmation via quorum heartbeat acks
//! - `ref_count` — reference counting: increment/decrement, weak upgrade, dealloc trigger
//! - `region` — address region arithmetic: contains, disjoint, adjacent, merge, split
//! - `replication_mode` — per-peer replication FSM: Probe → Replicate → Snapshot transitions
//! - `replication_pipeline` — flow control for pipelined `AppendEntries`
//! - `request_forward` — client request forwarding: follower-to-leader proxy with pending tracking
//! - `result` — verified Result combinators
//! - `ring_buffer` — verified FIFO queue with power-of-two capacity
//! - `rwlock` — read-write lock modes, mutual exclusion, reader/writer tokens
//! - `sequence` — monotonic ID generation with batch reservation and overflow protection
//! - `sequential` — sequential access detection, prefetch triggering
//! - `signal` — signal set algebra: add/remove/contains, union/intersect, deliverable
//! - `snapshot_compaction` — Raft log compaction trigger: when to snapshot, prefix discard, metadata
//! - `spin_backoff` — spin-wait backoff: two-phase (spin then yield), exponential spin count
//! - `snapshot_integrity` — dual-hash snapshot verification with round-trip and tamper proofs
//! - `snapshot_transfer` — chunked snapshot streaming: validation, ordering, progress
//! - `state_machine_safety` — no two servers apply different entries at the same index
//! - `slab` — slab allocation: slot index/address, capacity, generation counters, handle packing
//! - `scope` — prefix-based scope coverage, compound scoping, optional constraints
//! - `semaphore` — counting semaphore: acquire/release, capacity bounds, conservation, binary mutex
//! - `seqlock` — sequence lock protocol: even/odd counter, valid read detection, writer versioning
//! - `slice` — chunk count, prefix/suffix equality, all-equal, mismatch
//! - `subsumption` — variant subsumption DAG for capability authorization
//! - `term_authority` — step-down on higher term: adopt, revert to follower, clear vote
//! - `ticket_lock` — FIFO-fair ticket lock: take/release, waiter counting, mutual exclusion, queue position
//! - `tier` — hot/warm/cold tier classification with promote/demote
//! - `tiered_rate_limit` — global-first two-tier rate limiting with capacity invariant
//! - `tiling` — 2D rect non-overlap, containment, strip partitioning for layout engines
//! - `type_lattice` — type conversion reachability: reflexive, ANY top, STRING source
//! - `token_timestamp` — UCAN token expiration, future rejection, refresh threshold
//! - `transfer` — value transfer validation: conservation, idempotency, status codes
//! - `trust_level` — trust resolution with block precedence and access control
//! - `two_phase` — two-phase commit decision: unanimity, abort safety
//! - `vector_clock` — causality tracking with partial order and merge lattice
//! - `view_change` — view-based reconfiguration: view monotonicity, leader rotation, quorum-gated transitions
//! - `quorum_cert` — quorum certificate: vote counting, QC ordering, commit rule via consecutive certificates
//! - `voter_set` — voter set promotion: add learner, replace node, quorum preservation
//! - `wal_recovery` — WAL crash recovery: replay range, idempotent replay, checkpoint advancement
//! - `wal_segment` — WAL segment management: contiguity, space, truncation safety, replay range
//! - `watermark` — bump allocation: remaining, fits, alloc, reset, non-overlap
//! - `write_buffer` — memtable flush thresholds, buffer swap, write amplification, flush counting
//! - `block_cache` — cache slot states: insert, evict, mark-dirty, write-back, clock-hand sweep
//! - `free_space_map` — bitmap free-space: alloc/free conservation, first-free, contiguous runs
//! - `page_checksum` — Fletcher-style checksum accumulation, pack/unpack roundtrip, torn-write detection
//! - `witness` — witness replicas: vote without full log, heartbeat acks, role taxonomy
//! - `work_stealing` — work-stealing deque: push/pop/steal, size tracking, circular buffer index
//!
//! ### Memory allocator verification
//!
//! - `alloc_token` — allocation lifecycle FSM: free → allocated → freed, double-free prevention
//! - `size_class` — size class bucketing: power-of-two classes, waste bounds, monotonicity
//! - `free_list` — intrusive free list: LIFO push/pop, no-cycle (all-distinct), containment
//! - `split_coalesce` — buddy split/coalesce sequences: block conservation, recursive split trees
//! - `alloc_bitmap` — typed allocation bitmap: generational handles, double-free detection
//! - `arena` — arena allocation: bump alloc, collective free via generation reset
//! - `pool` — fixed-size object pool: slab + bitmap composition, slot alignment
//! - `segregated_free` — segregated free lists by size class: cross-class uniqueness
//! - `coalesce_policy` — immediate vs deferred coalescing: convergence, total preservation
//! - `fragmentation` — fragmentation metrics: external/internal bounds, can-satisfy

pub mod access_mode;
pub mod append_entries_build;
pub mod append_entries_check;
pub mod append_response;
pub mod acid;
pub mod algebraic;
pub mod approval;
pub mod authorize;
pub mod arithmetic;
pub mod backoff;
pub mod balance;
pub mod ballot;
pub mod barrier;
pub mod batch;
pub mod bitfield;
pub mod bitmap;
pub mod bitwise;
pub mod bloom;
pub mod bounded_range;
pub mod btree_balance;
pub mod buddy;
pub mod causal_order;
pub mod containment;
pub mod btree_delete;
pub mod btree_insert;
pub mod concurrent_composition;
pub mod consensus_path;
pub mod consistent_hash;
pub mod consistent_snapshot;

pub mod crdt;
pub mod bounded_mpmc;
pub mod bounded_peers;
pub mod bounds;
pub mod cas;
pub mod cas_loop;
pub mod chain;
pub mod check_quorum;
pub mod chunk;
pub mod client_routing;
pub mod client_session;
pub mod config_change;
pub mod circuit_breaker;
pub mod collections;
pub mod commit_advance;
pub mod commit_notify;
pub mod compaction_fence;
pub mod constant_time_eq;
pub mod conversion;
pub mod cost_tracker;
pub mod cow;
pub mod deadline_queue;
pub mod delegation;
pub mod double_entry;
pub mod distributed_lock;
pub mod drain;
pub mod drain_protocol;
pub mod env_scrub;
pub mod epoch_reclaim;
pub mod election;
pub mod escrow;
pub mod fair_share;
pub mod fee;
pub mod election_init;
pub mod election_jitter;
pub mod election_safety_proof;
pub mod election_timer;
pub mod entry_type;
pub mod epoch;
pub mod eviction;
pub mod eviction_score;
pub mod expiration;
pub mod fast_backtrack;
pub mod fencing;
pub mod follower_commit;
pub mod follower_redirect;
pub mod follower_read;
pub mod futex_state;
pub mod forest;
pub mod ghost_history;
pub mod glob;
pub mod gossip;
pub mod hash_bucket;
pub mod hazard_pointer;
pub mod heap_index;
pub mod heartbeat;
pub mod install_snapshot;
pub mod integer;
pub mod interval;
pub mod job_cancel;
pub mod job_dag;
pub mod job_idempotency;
pub mod job_priority;
pub mod job_result;
pub mod job_retry;
pub mod job_slot;
pub mod job_state;
pub mod job_timeout;
pub mod joint_consensus;
pub mod journal;
pub mod kv_version;
pub mod last_applied;
pub mod layout_constraint;
pub mod leader_append_only;
pub mod leader_completeness;
pub mod leader_completeness_ghost;
pub mod leadership_transfer;
pub mod learner_change;
pub mod learner_readiness;
pub mod lease;
pub mod lease_mutex;
pub mod ledger;
pub mod lease_read;
pub mod linearizability;
pub mod lock_ordering;
pub mod log;
pub mod log_compaction;
pub mod lsm_level;
pub mod log_conflict;
pub mod log_invariant_inductive;
pub mod markov;
pub mod match_index;
pub mod memory_order;
pub mod merkle;
pub mod monotonic;
pub mod mpsc_bounds;
pub mod nix_attrset;
pub mod noop_commit;
pub mod node_classify;
pub mod nix_closure;
pub mod node_lifecycle;
pub mod nix_list;
pub mod nix_string;
pub mod nix_version;
pub mod option;
pub mod ordering;
pub mod page;
pub mod paxos;
pub mod partition;
pub mod pattern;
pub mod peer_spec;
pub mod permission;
pub mod persistent_state;
pub mod pointwise;
pub mod pre_vote;
pub mod pre_vote_lease;
pub mod pre_vote_tally;
pub mod pressure;
pub mod priority_election;
pub mod priority_inversion;
pub mod process_state;
pub mod properties;
pub mod quota;
pub mod quorum;
pub mod quorum_cert;
pub mod raft_step;
pub mod raft_timing;
pub mod rate_limit;
pub mod rcu;
pub mod read_index;
pub mod ref_count;
pub mod region;
pub mod replication_mode;
pub mod replication_pipeline;
pub mod request_forward;
pub mod result;
pub mod retry;
pub mod ring_buffer;
pub mod rwlock;
pub mod scope;
pub mod semaphore;
pub mod seqlock;
pub mod seq_fold;
pub mod seq_set;
pub mod sequence;
pub mod sequential;
pub mod session_tree;
pub mod serialize_inj;
pub mod settlement;
pub mod signal;
pub mod snapshot_compaction;
pub mod spin_backoff;
pub mod snapshot_integrity;
pub mod snapshot_transfer;
pub mod state_machine_safety;
pub mod slab;
pub mod slice;
pub mod subsumption;
pub mod tcp_state;
pub mod term_authority;
pub mod ticket_lock;
pub mod tier;
pub mod tiered_rate_limit;
pub mod tiling;
pub mod token_timestamp;
pub mod trace_relation;
pub mod transfer;
pub mod trust_level;
pub mod type_lattice;
pub mod two_phase;
pub mod two_phase_lock;
pub mod typed_state;
pub mod vector_clock;
pub mod vote_tally;
pub mod view_change;
pub mod voter_set;
pub mod wal_segment;
pub mod watermark;
pub mod weighted_round_robin;
pub mod witness;
pub mod block_cache;
pub mod free_space_map;
pub mod page_checksum;
pub mod wal_recovery;
pub mod write_buffer;
pub mod work_stealing;
pub mod topo_sort;
pub mod cycle_detect;
pub mod reachability;
pub mod graph_color;
pub mod shortest_path;
pub mod alloc_token;
pub mod size_class;
pub mod free_list;
pub mod split_coalesce;
pub mod alloc_bitmap;
pub mod arena;
pub mod pool;
pub mod segregated_free;
pub mod coalesce_policy;
pub mod fragmentation;
pub mod txn_isolation;
pub mod mvcc;
pub mod snapshot_isolation;
pub mod query_cost;
pub mod index_select;
pub mod txn_dependency;
