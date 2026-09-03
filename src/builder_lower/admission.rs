//! Deterministic orchestration for finite builder-record admission.

fn width(len: usize, table: super::model::Table) -> Result<u32, super::model::Rejection> {
    let count = u32::try_from(len).map_err(|_| super::model::Rejection {
        class: super::model::FailureClass::TableTooWide,
        table,
        index: None,
        subject: None,
    })?;
    if count > super::model::MAX_OPERATIONS {
        Err(super::model::Rejection {
            class: super::model::FailureClass::TableTooWide,
            table,
            index: None,
            subject: None,
        })
    } else {
        Ok(count)
    }
}

/// Admits one complete backend-neutral builder operation record.
// r[impl backend-trace.handles]
// r[impl backend-trace.block-mapping]
// r[impl backend-trace.lowering-trace]
// r[impl backend-trace.handle-validity]
// r[impl backend-trace.terminators]
// r[impl backend-trace.ssa-like-locals]
// r[impl backend-trace.admission]
pub fn admit_record(
    body: &crate::mir_core::Body,
    record: &super::model::Record,
) -> Result<super::model::Facts, super::model::Rejection> {
    width(record.entries.len(), super::model::Table::Entry)?;
    super::layout::check_declarations(record)?;
    let block_count = super::layout::check_mappings(body, record)?;
    let local_count = super::layout::check_locals(body, record)?;
    let value_count = super::events::check_entries(body, record)?;
    Ok(super::model::Facts {
        block_count,
        operation_count: width(record.entries.len(), super::model::Table::Entry)?,
        value_count,
        local_count,
    })
}
