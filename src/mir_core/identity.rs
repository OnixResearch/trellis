//! Finite structural admission for Rust-shaped MIR core values.
//!
//! This module validates local, place, operand, rvalue, statement, terminator,
//! block, body, and successor shape. It does not parse rustc MIR or establish
//! Rust typing, borrowing, optimization, code generation, or compiler claims.
// r[impl mir-core.docs]
// r[impl mir-core.evidence]
// r[impl mir-core.final-validation]
// r[impl mir-core.non-claims]

mod admission;
mod downstream;
mod model;
mod proofs;

pub use admission::admit_body;
pub use admission::admit_operand;
pub use admission::admit_place;
pub use admission::admit_rvalue;
pub use admission::admit_statement;
pub use admission::admit_terminator;
pub use downstream::extract_adapter_facts;
pub use model::AdapterFacts;
pub use model::BasicBlock;
pub use model::BinaryOperator;
pub use model::Body;
pub use model::Constant;
pub use model::Edge;
pub use model::Failure;
pub use model::FailureClass;
pub use model::LocalDecl;
pub use model::LocalRole;
pub use model::Operand;
pub use model::Place;
pub use model::Projection;
pub use model::Rejection;
pub use model::Rvalue;
pub use model::Statement;
pub use model::Terminator;
pub use model::TypeTag;
pub use model::RETURN_PLACE;

#[cfg(test)]
mod tests;
