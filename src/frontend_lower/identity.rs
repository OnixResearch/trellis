//! Finite owner-scoped HIR ID lowering admission.
//!
//! The component checks source mappings, generated-ID freshness, created-ID
//! use, uniqueness, and optional definition metadata. It does not model AST or
//! HIR semantics, name resolution, type checking, or a compiler frontend.
// r[impl hir-id.docs]
// r[impl hir-id.evidence]
// r[impl hir-id.final-validation]
// r[impl hir-id.non-claims]

mod admission;
mod catalog;
mod lifecycle;
mod model;
mod proofs;

pub use admission::admit_record;
pub use model::CreatedKey;
pub use model::DefinitionEntry;
pub use model::DefinitionKey;
pub use model::Facts;
pub use model::FailureClass;
pub use model::GeneratedNode;
pub use model::Mapping;
pub use model::OwnerKey;
pub use model::Record;
pub use model::Rejection;
pub use model::ScopedKey;
pub use model::SourceKey;
pub use model::Table;
pub use model::UsePolicy;
pub use model::MAX_RECORDS;

#[cfg(test)]
mod tests;
