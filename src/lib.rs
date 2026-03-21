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
//! - `bounds` — range predicates, clamping
//! - `collections` — sorted insert, binary search
//! - `option` — verified Option combinators
//! - `ordering` — min, max, median, sign, compare-and-swap
//! - `result` — verified Result combinators

pub mod arithmetic;
pub mod bitwise;
pub mod bounds;
pub mod collections;
pub mod option;
pub mod ordering;
pub mod properties;
pub mod result;
