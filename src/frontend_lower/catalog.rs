//! Source mapping, owner scope, uniqueness, and generated freshness checks.

fn position(index: usize, table: super::model::Table) -> Result<u32, super::model::Rejection> {
    u32::try_from(index).map_err(|_| super::model::Rejection {
        class: super::model::FailureClass::TableTooWide,
        table,
        index: None,
        related: None,
    })
}

fn reject(
    class: super::model::FailureClass,
    table: super::model::Table,
    index: u32,
    related: Option<u32>,
) -> super::model::Rejection {
    super::model::Rejection {
        class,
        table,
        index: Some(index),
        related,
    }
}

pub(super) fn source_is_declared(record: &super::model::Record, source: super::model::SourceKey) -> bool {
    record.source_nodes.contains(&source)
}

pub(super) fn target_is_mapped(record: &super::model::Record, target: super::model::ScopedKey) -> bool {
    record.mappings.iter().any(|mapping| mapping.target == target)
}

pub(super) fn check_sources(record: &super::model::Record) -> Result<(), super::model::Rejection> {
    for (raw_index, source) in record.source_nodes.iter().copied().enumerate() {
        let index = position(raw_index, super::model::Table::Source)?;
        if let Some(raw_prior) = record.source_nodes[..raw_index].iter().position(|prior| *prior == source) {
            return Err(reject(
                super::model::FailureClass::DuplicateSource,
                super::model::Table::Source,
                index,
                Some(position(raw_prior, super::model::Table::Source)?),
            ));
        }
    }
    Ok(())
}

pub(super) fn check_mappings(record: &super::model::Record) -> Result<(), super::model::Rejection> {
    for (raw_index, mapping) in record.mappings.iter().copied().enumerate() {
        let index = position(raw_index, super::model::Table::Mapping)?;
        if !source_is_declared(record, mapping.source) {
            return Err(reject(
                super::model::FailureClass::UnknownSource,
                super::model::Table::Mapping,
                index,
                Some(mapping.source.0),
            ));
        }
        if !super::proofs::owner_admitted(mapping.target.owner.0, record.owner.0) {
            return Err(reject(
                super::model::FailureClass::WrongOwner,
                super::model::Table::Mapping,
                index,
                Some(mapping.target.owner.0),
            ));
        }
        if let Some(raw_prior) = record.mappings[..raw_index].iter().position(|prior| prior.source == mapping.source) {
            return Err(reject(
                super::model::FailureClass::DuplicateSource,
                super::model::Table::Mapping,
                index,
                Some(position(raw_prior, super::model::Table::Mapping)?),
            ));
        }
        if let Some(raw_prior) = record.mappings[..raw_index].iter().position(|prior| prior.target == mapping.target) {
            return Err(reject(
                super::model::FailureClass::DuplicateHirId,
                super::model::Table::Mapping,
                index,
                Some(position(raw_prior, super::model::Table::Mapping)?),
            ));
        }
    }

    for (raw_index, source) in record.source_nodes.iter().copied().enumerate() {
        if !record.mappings.iter().any(|mapping| mapping.source == source) {
            return Err(reject(
                super::model::FailureClass::UnloweredSource,
                super::model::Table::Source,
                position(raw_index, super::model::Table::Source)?,
                Some(source.0),
            ));
        }
    }
    Ok(())
}

// r[impl hir-id.fresh-generated]
// r[impl hir-id.uniqueness]
pub(super) fn check_generated(record: &super::model::Record) -> Result<(), super::model::Rejection> {
    for (raw_index, generated) in record.generated_nodes.iter().copied().enumerate() {
        let index = position(raw_index, super::model::Table::Generated)?;
        if !super::proofs::owner_admitted(generated.target.owner.0, record.owner.0) {
            return Err(reject(
                super::model::FailureClass::WrongOwner,
                super::model::Table::Generated,
                index,
                Some(generated.target.owner.0),
            ));
        }
        if target_is_mapped(record, generated.target) {
            return Err(reject(
                super::model::FailureClass::GeneratedAlias,
                super::model::Table::Generated,
                index,
                Some(generated.target.local),
            ));
        }
        if let Some(raw_prior) = record.generated_nodes[..raw_index]
            .iter()
            .position(|prior| prior.generation == generated.generation)
        {
            return Err(reject(
                super::model::FailureClass::DuplicateGeneration,
                super::model::Table::Generated,
                index,
                Some(position(raw_prior, super::model::Table::Generated)?),
            ));
        }
        if let Some(raw_prior) =
            record.generated_nodes[..raw_index].iter().position(|prior| prior.target == generated.target)
        {
            return Err(reject(
                super::model::FailureClass::DuplicateHirId,
                super::model::Table::Generated,
                index,
                Some(position(raw_prior, super::model::Table::Generated)?),
            ));
        }
    }
    Ok(())
}
