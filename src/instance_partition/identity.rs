//! Finite concrete-item collection and partition admission.
//!
//! The component checks declared instances, root reachability, canonical
//! collection, stability classes, and disjoint complete output units. It does
//! not perform trait solving, generic substitution, code generation, or linking.
// r[impl mono.docs]
// r[impl mono.evidence]
// r[impl mono.final-validation]
// r[impl mono.non-claims]

mod admission;
mod collection;
mod model;
mod partition;
mod proofs;

pub use admission::admit_record;
pub use collection::collect_reachable;
pub use model::ConcreteShape;
pub use model::Facts;
pub use model::FailureClass;
pub use model::Item;
pub use model::ItemKey;
pub use model::ItemKind;
pub use model::Origin;
pub use model::Record;
pub use model::Reference;
pub use model::Rejection;
pub use model::Stability;
pub use model::Table;
pub use model::Unit;
pub use model::UnitKey;
pub use model::MAX_ITEMS;

#[cfg(test)]
mod tests;
