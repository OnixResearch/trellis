//! Verified logic building blocks.
//!
//! Every public function in this crate has machine-checked pre/postconditions
//! verified by [Verus](https://verus-lang.github.io/verus/guide/). Requirements
//! are tracked by [Tracey](https://tracey.bearcove.eu/) in `docs/requirements.md`.
//!
//! ## Modules
//!
//! - `arithmetic` — abs diff, saturating ops, checked mul
//! - `bounds` — range predicates, clamping
//! - `collections` — sorted insert, binary search
//! - `option` — verified Option combinators

pub mod arithmetic;
pub mod bounds;
pub mod collections;
pub mod option;
