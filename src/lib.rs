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
//! - `acid` — ACID transaction proofs: atomicity, consistency, isolation, durability
//! - `arithmetic` — abs diff, saturating ops, checked mul
//! - `bitwise` — power-of-two, bit get/set/clear/toggle
//! - `batch` — write batch limit checking and flush decisions
//! - `bitmap` — verified fixed-size bit set using bitwise ops
//! - `approval` — merge gating: approval counting, stale detection, CI checks
//! - `bounded_range` — ranges with optional upper bound (None = unbounded)
//! - `bounds` — range predicates, clamping
//! - `cas` — compare-and-swap condition evaluation
//! - `chain` — chain hash integrity with append-preserves proof
//! - `chunk` — ceiling-division chunk arithmetic, offset↔chunk mapping
//! - `collections` — sorted insert, binary search
//! - `conversion` — checked narrowing (u32/u16/u8), sign conversion (i64↔u64)
//! - `cow` — copy-on-write decision logic, ref counting, snapshot isolation
//! - `drain` — drain-state admission protocol for graceful shutdown
//! - `epoch` — lexicographic (term, index) comparison for Raft elections
//! - `eviction` — LRU eviction with protected entries
//! - `expiration` — TTL calculation, lease/key expiration, time remaining
//! - `hash_bucket` — hash-to-bucket assignment, modular and power-of-two
//! - `integer` — parity, alignment, log2, next-power-of-two
//! - `interval` — half-open interval contains, overlap, split, merge
//! - `log` — append-only log with Log Matching Property
//! - `monotonic` — advancement predicates for terms, commit indices
//! - `option` — verified Option combinators
//! - `ordering` — min, max, median, sign, compare-and-swap
//! - `partition` — verified non-overlapping gap-free interval partitions
//! - `pressure` — multi-threshold severity classification
//! - `quorum` — majority threshold, quorum predicate, intersection theorem
//! - `rate_limit` — integer token bucket rate limiting
//! - `result` — verified Result combinators
//! - `ring_buffer` — verified FIFO queue with power-of-two capacity
//! - `sequential` — sequential access detection, prefetch triggering
//! - `slice` — chunk count, prefix/suffix equality, all-equal, mismatch

pub mod acid;
pub mod approval;
pub mod arithmetic;
pub mod batch;
pub mod bitmap;
pub mod bitwise;
pub mod bounded_range;
pub mod bounds;
pub mod cas;
pub mod chain;
pub mod chunk;
pub mod collections;
pub mod conversion;
pub mod cow;
pub mod drain;
pub mod epoch;
pub mod eviction;
pub mod expiration;
pub mod hash_bucket;
pub mod integer;
pub mod interval;
pub mod log;
pub mod monotonic;
pub mod option;
pub mod ordering;
pub mod partition;
pub mod pressure;
pub mod properties;
pub mod quorum;
pub mod rate_limit;
pub mod result;
pub mod ring_buffer;
pub mod sequential;
pub mod slice;
