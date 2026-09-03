//! Finite inert HIR attribute parsing and policy admission.
//!
//! This facade composes descriptor, occurrence, phase-agreement, proof, and
//! test components. It does not model active expansion, proc macros, token
//! parsing, rendered diagnostics, feature stabilization, or the whole frontend.
// r[impl hir-attr.evidence]
// r[impl hir-attr.non-claims]

pub mod admission;
pub mod agreement;
pub mod model;
pub mod proofs;
pub mod registry;
pub mod rules;

pub use admission::*;
pub use agreement::*;
pub use model::*;
pub use proofs::*;
pub use registry::*;
pub use rules::*;

#[cfg(test)]
mod tests;
