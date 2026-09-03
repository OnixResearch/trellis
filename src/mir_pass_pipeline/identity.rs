//! Finite structural admission for MIR pass-pipeline traces.
//!
//! The core validates declared phase order, pass registrations, validation
//! gates, optimization gates, and read-before-steal ordering. It does not run
//! rustc queries or prove transformation semantics.
// r[impl mir-pass.docs]
// r[impl mir-pass.evidence]
// r[impl mir-pass.non-claims]

mod admission;
mod model;
mod proofs;
mod registration;

pub use admission::MAX_TRACE_STEPS;
pub use admission::admit_trace;
pub use admission::apply_step;
pub use admission::phase_rank;
pub use model::ArtifactState;
pub use model::Descriptor;
pub use model::FailureClass;
pub use model::Kind;
pub use model::MutationKind;
pub use model::OptimizationLevel;
pub use model::Phase;
pub use model::Rejection;
pub use model::Settings;
pub use model::State;
pub use model::StealPolicy;
pub use model::Step;
pub use model::StructureEffect;
pub use model::ValidationKind;
pub use registration::registered_descriptor;

#[cfg(test)]
mod tests;
