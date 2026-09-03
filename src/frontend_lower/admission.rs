//! Deterministic orchestration for finite HIR-ID lowering admission.

fn reject(table: super::model::Table) -> super::model::Rejection {
    super::model::Rejection {
        class: super::model::FailureClass::TableTooWide,
        table,
        index: None,
        related: None,
    }
}

fn table_width(len: usize, table: super::model::Table) -> Result<u32, super::model::Rejection> {
    let width = u32::try_from(len).map_err(|_| reject(table))?;
    if width > super::model::MAX_RECORDS {
        Err(reject(table))
    } else {
        Ok(width)
    }
}

fn check_widths(record: &super::model::Record) -> Result<(), super::model::Rejection> {
    table_width(record.source_nodes.len(), super::model::Table::Source)?;
    table_width(record.mappings.len(), super::model::Table::Mapping)?;
    table_width(record.created_ids.len(), super::model::Table::Created)?;
    table_width(record.used_ids.len(), super::model::Table::Used)?;
    table_width(record.generated_nodes.len(), super::model::Table::Generated)?;
    table_width(record.definition_requirements.len(), super::model::Table::DefinitionRequirement)?;
    table_width(record.definitions.len(), super::model::Table::Definition)?;
    Ok(())
}

/// Admits one complete finite owner-scoped HIR-ID lowering record.
// r[impl hir-id.owner-scope]
// r[impl hir-id.lowering-map]
// r[impl hir-id.fresh-generated]
// r[impl hir-id.uniqueness]
// r[impl hir-id.def-correspondence]
// r[impl hir-id.admission]
pub fn admit_record(record: &super::model::Record) -> Result<super::model::Facts, super::model::Rejection> {
    check_widths(record)?;
    super::catalog::check_sources(record)?;
    super::catalog::check_mappings(record)?;
    super::catalog::check_generated(record)?;
    super::lifecycle::check_usage(record)?;
    super::lifecycle::check_definitions(record)?;

    Ok(super::model::Facts {
        source_count: table_width(record.source_nodes.len(), super::model::Table::Source)?,
        mapping_count: table_width(record.mappings.len(), super::model::Table::Mapping)?,
        generated_count: table_width(record.generated_nodes.len(), super::model::Table::Generated)?,
        used_count: table_width(record.used_ids.len(), super::model::Table::Used)?,
        definition_count: table_width(record.definitions.len(), super::model::Table::Definition)?,
    })
}
