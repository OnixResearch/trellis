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
//! - `acid` — ACID transaction proofs: atomicity, consistency, isolation, durability
//! - `arithmetic` — abs diff, saturating ops, checked mul
//! - `backoff` — exponential backoff with cap and jitter bounds
//! - `ballot` — vote tracking for Raft elections with quorum intersection safety
//! - `bitwise` — power-of-two, bit get/set/clear/toggle
//! - `batch` — write batch limit checking and flush decisions
//! - `bitmap` — verified fixed-size bit set using bitwise ops
//! - `bloom` — Bloom filter with no-false-negatives proof
//! - `authorize` — three-factor authorization: subsumption × scope × mode
//! - `approval` — merge gating: approval counting, stale detection, CI checks
//! - `bounded_range` — ranges with optional upper bound (None = unbounded)
//! - `causal_order` — causal delivery and consistent snapshots
//! - `containment` — delegation containment: scope + mode + subsumption
//! - `consistent_hash` — consistent hash ring with complete coverage
//! - `crdt` — G-Counter, PN-Counter, LWW-Register with merge laws
//! - `bounds` — range predicates, clamping
//! - `cas` — compare-and-swap condition evaluation
//! - `chain` — chain hash integrity with append-preserves proof
//! - `circuit_breaker` — three-state circuit breaker: closed/open/half-open
//! - `chunk` — ceiling-division chunk arithmetic, offset↔chunk mapping
//! - `collections` — sorted insert, binary search
//! - `conversion` — checked narrowing (u32/u16/u8), sign conversion (i64↔u64)
//! - `cow` — copy-on-write decision logic, ref counting, snapshot isolation
//! - `delegation` — delegation chain validation: depth bounds, attenuation, Delegate gate
//! - `drain` — drain-state admission protocol for graceful shutdown
//! - `epoch` — lexicographic (term, index) comparison for Raft elections
//! - `eviction` — LRU eviction with protected entries
//! - `fencing` — fencing tokens for stale-write rejection
//! - `glob` — glob pattern matching with `*` wildcards for capabilities
//! - `gossip` — gossip infection bounds and convergence
//! - `expiration` — TTL calculation, lease/key expiration, time remaining
//! - `hash_bucket` — hash-to-bucket assignment, modular and power-of-two
//! - `integer` — parity, alignment, log2, next-power-of-two
//! - `interval` — half-open interval contains, overlap, split, merge
//! - `lease` — time-bounded leases with disjointness, renewal, revocation
//! - `log` — append-only log with Log Matching Property
//! - `merkle` — Merkle proof verification with hash axiom
//! - `monotonic` — advancement predicates for terms, commit indices
//! - `nix_attrset` — Nix attribute set operations: merge, intersect, remove
//! - `nix_closure` — Nix generic closure with fuel-based termination
//! - `nix_list` — Nix list builtins: elem, head, tail, gen_list, concat
//! - `nix_string` — Nix string operations: length, substring, replace
//! - `nix_version` — Nix version comparison with component classification
//! - `option` — verified Option combinators
//! - `ordering` — min, max, median, sign, compare-and-swap
//! - `pattern` — wildcard and set-membership pattern matching for capabilities
//! - `page` — virtual address decomposition: frame number, offset, alignment
//! - `partition` — verified non-overlapping gap-free interval partitions
//! - `permission` — Unix file permission bits: has/set/clear, umask
//! - `pressure` — multi-threshold severity classification
//! - `process_state` — process lifecycle state machine: transitions, terminal, schedulable
//! - `quorum` — majority threshold, quorum predicate, intersection theorem
//! - `rate_limit` — integer token bucket rate limiting
//! - `result` — verified Result combinators
//! - `ring_buffer` — verified FIFO queue with power-of-two capacity
//! - `sequential` — sequential access detection, prefetch triggering
//! - `signal` — signal set algebra: add/remove/contains, union/intersect, deliverable
//! - `scope` — prefix-based scope coverage, compound scoping, optional constraints
//! - `slice` — chunk count, prefix/suffix equality, all-equal, mismatch
//! - `subsumption` — variant subsumption DAG for capability authorization
//! - `token_timestamp` — UCAN token expiration, future rejection, refresh threshold
//! - `two_phase` — two-phase commit decision: unanimity, abort safety
//! - `vector_clock` — causality tracking with partial order and merge lattice

pub mod access_mode;
pub mod acid;
pub mod algebraic;
pub mod approval;
pub mod authorize;
pub mod arithmetic;
pub mod backoff;
pub mod ballot;
pub mod batch;
pub mod bitmap;
pub mod bitwise;
pub mod bloom;
pub mod bounded_range;
pub mod causal_order;
pub mod containment;
pub mod consistent_hash;
pub mod crdt;
pub mod bounds;
pub mod cas;
pub mod chain;
pub mod chunk;
pub mod circuit_breaker;
pub mod collections;
pub mod conversion;
pub mod cost_tracker;
pub mod cow;
pub mod delegation;
pub mod drain;
pub mod env_scrub;
pub mod epoch;
pub mod eviction;
pub mod expiration;
pub mod fencing;
pub mod forest;
pub mod glob;
pub mod gossip;
pub mod hash_bucket;
pub mod integer;
pub mod interval;
pub mod lease;
pub mod log;
pub mod merkle;
pub mod monotonic;
pub mod nix_attrset;
pub mod nix_closure;
pub mod nix_list;
pub mod nix_string;
pub mod nix_version;
pub mod option;
pub mod ordering;
pub mod page;
pub mod partition;
pub mod pattern;
pub mod permission;
pub mod pointwise;
pub mod pressure;
pub mod process_state;
pub mod properties;
pub mod quorum;
pub mod rate_limit;
pub mod result;
pub mod ring_buffer;
pub mod scope;
pub mod seq_fold;
pub mod seq_set;
pub mod sequential;
pub mod session_tree;
pub mod signal;
pub mod slice;
pub mod subsumption;
pub mod token_timestamp;
pub mod two_phase;
pub mod vector_clock;
