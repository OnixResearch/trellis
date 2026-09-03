//! Frozen import, attenuation, conditional, and header policy admission.

fn position(index: usize) -> Result<u32, super::model::Rejection> {
    u32::try_from(index).map_err(|_| super::model::Rejection {
        class: super::model::FailureClass::TableTooWide,
        table: super::model::Table::Import,
        index: None,
        subject: None,
    })
}

fn reject(class: super::model::FailureClass, index: u32) -> super::model::Rejection {
    super::model::Rejection {
        class,
        table: super::model::Table::Import,
        index: Some(index),
        subject: None,
    }
}

fn headers_are_admitted(edge: super::model::ImportEdge) -> bool {
    match edge.headers {
        super::model::HeaderPolicy::None => true,
        super::model::HeaderPolicy::Forward => matches!(
            (edge.source, edge.target),
            (
                super::model::Origin::Remote { authority: source },
                super::model::Target::Relative { authority: target }
            ) if source == target
        ),
    }
}

fn remote_target_is_attenuated(edge: super::model::ImportEdge) -> bool {
    !matches!(
        (edge.source, edge.target),
        (
            super::model::Origin::Remote { .. },
            super::model::Target::LocalFile | super::model::Target::Home | super::model::Target::Environment
        )
    )
}

/// Admits the complete static frozen import table.
// r[impl domain-primitives.typed-config.import-closure]
pub fn admit_imports(record: &super::model::Record) -> Result<u32, super::model::Rejection> {
    let count = position(record.imports.len())?;
    if count > super::model::MAX_ROWS {
        return Err(super::model::Rejection {
            class: super::model::FailureClass::TableTooWide,
            table: super::model::Table::Import,
            index: None,
            subject: None,
        });
    }
    for (raw_index, edge) in record.imports.iter().copied().enumerate() {
        let index = position(raw_index)?;
        if matches!(edge.target, super::model::Target::Computed) {
            return Err(reject(super::model::FailureClass::ComputedImport, index));
        }
        if edge.selection == super::model::Selection::Conditional {
            return Err(reject(super::model::FailureClass::ConditionalImport, index));
        }
        if !remote_target_is_attenuated(edge) {
            return Err(reject(super::model::FailureClass::RemoteAttenuation, index));
        }
        if !headers_are_admitted(edge) {
            return Err(reject(super::model::FailureClass::CrossOriginHeaders, index));
        }
        if edge.declared != edge.observed {
            return Err(reject(super::model::FailureClass::DigestMismatch, index));
        }
    }
    Ok(count)
}
