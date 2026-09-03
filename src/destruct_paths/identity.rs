//! Finite MIR destruction-decision and replacement metadata.
//!
//! The component checks structural paths, initializedness, obligations,
//! classification, guarded/open expansion, and replacement order. It performs
//! no destructor, unwind, allocation, compiler, filesystem, or process effect.
// r[impl mir-drop.docs]
// r[impl mir-drop.evidence]
// r[impl mir-drop.final-validation]
// r[impl mir-drop.non-claims]

mod decision;
mod facts;
mod model;
mod paths;
mod proofs;

pub use decision::classify;
pub use decision::expand;
pub use decision::split_replace;
pub use facts::admit_state;
pub use facts::apply_action;
pub use model::Action;
pub use model::ActiveVariant;
pub use model::Classification;
pub use model::ConditionalPolicy;
pub use model::ExpansionRequest;
pub use model::Facts;
pub use model::FailureClass;
pub use model::FlagKey;
pub use model::Metadata;
pub use model::Path;
pub use model::PathKey;
pub use model::Record;
pub use model::Rejection;
pub use model::ReplacementRequest;
pub use model::Shape;
pub use model::State;
pub use model::Step;
pub use model::Table;
pub use model::ValueKey;
pub use model::VariantFields;
pub use model::MAX_EXPANSION_STEPS;
pub use model::MAX_PATHS;
pub use paths::admit_record;

#[cfg(test)]
mod tests;
