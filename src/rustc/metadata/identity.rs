//! Finite rustc crate-artifact compatibility and dependency selection.
//!
//! This facade checks declared fields only. It does not parse archives, compute
//! rustc hashes, prove collision resistance, link code, validate LTO, or prove
//! whole build-system correctness.
// r[impl rustc-metadata.evidence]
// r[impl rustc-metadata.non-claims]

pub mod admission;
pub mod model;
pub mod proofs;
pub mod resolver;

pub use admission::*;
pub use model::*;
pub use proofs::*;
pub use resolver::*;

#[cfg(test)]
mod tests;
