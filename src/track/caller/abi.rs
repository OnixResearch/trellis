//! Finite `#[track_caller]` ABI metadata and admission.
//!
//! This facade exposes pure metadata, admission, chain, trace, and proof
//! components. It does not model stack walking, panic formatting, `Location`
//! allocation, ABI lowering, compiler intrinsics, debug information, or
//! whole-codegen correctness.
// r[impl track-caller.evidence]
// r[impl track-caller.non-claims]

pub mod admission;
pub mod chain;
pub mod model;
pub mod proofs;
pub mod trace;

pub use admission::*;
pub use chain::*;
pub use model::*;
pub use proofs::*;
pub use trace::*;

#[cfg(test)]
mod tests;
