//! Finite typed configuration normal-form and import-policy admission.
//!
//! The component owns in-memory type, normalization, digest-input, import, and
//! refactor decisions. It parses no Dhall or Nickel, performs no import or hash
//! effect, and claims no external language compatibility.
// r[impl domain-primitives.typed-config.docs]
// r[impl domain-primitives.typed-config.positive]
// r[impl domain-primitives.typed-config.negative]
// r[impl domain-primitives.typed-config.final-validation]
// r[impl domain-primitives.typed-config.non-claims]

mod canonical;
mod graph;
mod model;
mod policy;
mod proofs;
mod schema;
pub use canonical::admit_refactor;
pub use canonical::normalize;
pub use canonical::select_digest_input;
pub use model::Compatibility;
pub use model::Digest;
pub use model::DigestInput;
pub use model::Facts;
pub use model::FailureClass;
pub use model::Field;
pub use model::HeaderPolicy;
pub use model::ImportEdge;
pub use model::Kind;
pub use model::Node;
pub use model::NodeKey;
pub use model::NormalChild;
pub use model::NormalEntry;
pub use model::NormalForm;
pub use model::Origin;
pub use model::Record;
pub use model::Refactor;
pub use model::Rejection;
pub use model::RequiredField;
pub use model::Scalar;
pub use model::Selection;
pub use model::Table;
pub use model::Target;
pub use model::TypeTag;
pub use model::DIGEST_BYTES;
pub use model::MAX_ROWS;
pub use policy::admit_imports;

fn table_width(len: usize, table: model::Table) -> Result<u32, model::Rejection> {
    let count = u32::try_from(len).map_err(|_| model::Rejection {
        class: model::FailureClass::TableTooWide,
        table,
        index: None,
        subject: None,
    })?;
    if count > model::MAX_ROWS {
        Err(model::Rejection {
            class: model::FailureClass::TableTooWide,
            table,
            index: None,
            subject: None,
        })
    } else {
        Ok(count)
    }
}

/// Admits one complete finite typed configuration record.
// r[impl domain-primitives.typed-config.expression-envelope]
// r[impl domain-primitives.typed-config.type-admission]
// r[impl domain-primitives.typed-config.explicit-failure]
pub fn admit_record(record: &model::Record) -> Result<model::Facts, model::Rejection> {
    let node_count = graph::check_structure(record)?;
    let field_count = schema::check_required_fields(record)?;
    let import_count = policy::admit_imports(record)?;
    table_width(record.dependency_digests.len(), model::Table::Import)?;
    Ok(model::Facts {
        node_count,
        field_count,
        import_count,
    })
}

#[cfg(test)]
mod tests;
