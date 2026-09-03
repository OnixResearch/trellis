//! Deterministic visitor coverage and traversal over admitted MIR-core bodies.
//!
//! The component checks explicit locations, exactly-once traces, reachability,
//! block order, and shape-preserving mutation authority. It does not implement
//! rustc visitor macros or prove the semantics of any transformation.
// r[impl mir-visitor.docs]
// r[impl mir-visitor.evidence]
// r[impl mir-visitor.final-validation]
// r[impl mir-visitor.non-claims]

mod admission;
mod bounds;
mod model;
mod proofs;
mod rewrite;
mod traversal;

pub use admission::admit_trace;
pub use bounds::admit_location;
pub use model::CoverageMode;
pub use model::Entry;
pub use model::FailureClass;
pub use model::Location;
pub use model::MutationClass;
pub use model::Order;
pub use model::Owner;
pub use model::ReachabilityPolicy;
pub use model::Rejection;
pub use model::Shape;
pub use model::Trace;
pub use model::TraceFacts;
pub use rewrite::admit_mutation;
pub use rewrite::body_shape;
pub use traversal::preorder_blocks;
pub use traversal::reverse_postorder_blocks;

#[cfg(test)]
mod tests;
