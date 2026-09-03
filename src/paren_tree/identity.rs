//! Finite canonical parenthesized-tree admission and roundtrip checks.
//!
//! The component owns a dialect-neutral atom/list tree, canonical UTF-8 text,
//! bounded parsing, and deterministic failures. It has no evaluator, macro,
//! interner, module loader, bytecode, filesystem, network, or process authority.
// r[impl s-expression.docs]
// r[impl s-expression.evidence.positive]
// r[impl s-expression.evidence.negative]
// r[impl s-expression.final-validation]
// r[impl s-expression.non-claims]

mod admission;
mod emit;
mod equivalence;
mod model;
mod proofs;
mod reader;

pub use admission::admit_atom;
pub use admission::admit_value;
pub use emit::serialize_canonical;
pub use equivalence::check_roundtrip;
pub use model::Atom;
pub use model::Budget;
pub use model::FailureClass;
pub use model::Measures;
pub use model::Rejection;
pub use model::Value;
pub use model::MAX_NESTING_DEPTH;
pub use model::MAX_NODE_COUNT;
pub use model::MAX_TEXT_BYTES;
pub use reader::parse_canonical;

#[cfg(test)]
mod tests;
