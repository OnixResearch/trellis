//! Finite MIR-location lattice analysis and fixed-point admission.
//!
//! The component checks explicit finite tables, semilattice laws, monotone
//! effects, bounded convergence, and pure result inspection. It proves no
//! analysis-specific Rust semantics or compiler behavior.
// r[impl mir-dataflow.docs]
// r[impl mir-dataflow.evidence]
// r[impl mir-dataflow.final-validation]
// r[impl mir-dataflow.non-claims]

mod cursor;
mod engine;
mod lattice;
mod model;
mod proofs;
mod tables;

pub use cursor::inspect_state;
pub use cursor::visit_locations;
pub use engine::run_to_fixed_point;
pub use lattice::admit_domain;
pub use lattice::join_state;
pub use model::AdmissionFacts;
pub use model::Analysis;
pub use model::Direction;
pub use model::Domain;
pub use model::EdgeEffect;
pub use model::EdgeState;
pub use model::FailureClass;
pub use model::LocationEffect;
pub use model::LocationState;
pub use model::Point;
pub use model::Rejection;
pub use model::Results;
pub use model::StateKey;
pub use model::Table;
pub use model::Transfer;
pub use model::MAX_ROUNDS;
pub use model::MAX_ROWS;
pub use model::MAX_STATES;
pub use tables::admit_analysis;
pub use tables::apply_transfer;

#[cfg(test)]
mod tests;
