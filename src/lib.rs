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
//! - `arithmetic` — abs diff, saturating ops, checked mul
//! - `bitwise` — power-of-two, bit get/set/clear/toggle
//! - `bitmap` — verified fixed-size bit set using bitwise ops
//! - `bounds` — range predicates, clamping
//! - `conversion` — checked narrowing (u32/u16/u8), sign conversion (i64↔u64)
//! - `collections` — sorted insert, binary search
//! - `integer` — parity, alignment, log2, next-power-of-two
//! - `option` — verified Option combinators
//! - `ordering` — min, max, median, sign, compare-and-swap
//! - `quorum` — majority threshold, quorum predicate, intersection theorem
//! - `result` — verified Result combinators
//! - `ring_buffer` — verified FIFO queue with power-of-two capacity
//! - `slice` — chunk count, prefix/suffix equality, all-equal, mismatch

pub mod arithmetic;
pub mod bitmap;
pub mod bitwise;
pub mod bounds;
pub mod conversion;
pub mod collections;
pub mod integer;
pub mod option;
pub mod ordering;
pub mod properties;
pub mod quorum;
pub mod result;
pub mod ring_buffer;
pub mod slice;
