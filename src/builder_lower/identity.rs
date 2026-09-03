//! Finite backend-neutral builder-operation admission.
//!
//! The component checks declared handles, MIR block mappings, source locations,
//! value definition order, control targets, terminators, and local storage
//! classes. It does not establish concrete IR, ABI, machine-code, or linker
//! correctness.
// r[impl backend-trace.docs]
// r[impl backend-trace.evidence]
// r[impl backend-trace.final-validation]
// r[impl backend-trace.non-claims]

mod admission;
mod events;
mod layout;
mod model;
mod proofs;

pub use admission::admit_record;
pub use model::BlockKey;
pub use model::BlockMapping;
pub use model::Declarations;
pub use model::Entry;
pub use model::Facts;
pub use model::FailureClass;
pub use model::FunctionKey;
pub use model::LocalClass;
pub use model::LocalStorage;
pub use model::Operation;
pub use model::PlaceKey;
pub use model::Record;
pub use model::Rejection;
pub use model::Table;
pub use model::TypeKey;
pub use model::ValueKey;
pub use model::MAX_OPERATIONS;

#[cfg(test)]
mod tests;
