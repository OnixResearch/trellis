//! Declaration, block-mapping, and local-class admission.

fn position(index: usize, table: super::model::Table) -> Result<u32, super::model::Rejection> {
    u32::try_from(index).map_err(|_| super::model::Rejection {
        class: super::model::FailureClass::TableTooWide,
        table,
        index: None,
        subject: None,
    })
}

fn reject(
    class: super::model::FailureClass,
    table: super::model::Table,
    index: u32,
    subject: Option<u32>,
) -> super::model::Rejection {
    super::model::Rejection {
        class,
        table,
        index: Some(index),
        subject,
    }
}

fn width(len: usize, table: super::model::Table) -> Result<u32, super::model::Rejection> {
    let count = position(len, table)?;
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

fn duplicate_position<T: PartialEq>(items: &[T], raw_index: usize) -> Option<usize> {
    items[..raw_index].iter().position(|prior| *prior == items[raw_index])
}

// r[impl backend-trace.handles]
pub(super) fn check_declarations(record: &super::model::Record) -> Result<(), super::model::Rejection> {
    width(record.declarations.blocks.len(), super::model::Table::Declaration)?;
    width(record.declarations.functions.len(), super::model::Table::Declaration)?;
    width(record.declarations.types.len(), super::model::Table::Declaration)?;
    width(record.declarations.places.len(), super::model::Table::Declaration)?;

    for (raw_index, _) in record.declarations.blocks.iter().enumerate() {
        if let Some(raw_prior) = duplicate_position(&record.declarations.blocks, raw_index) {
            return Err(reject(
                super::model::FailureClass::DuplicateDeclaration,
                super::model::Table::Declaration,
                position(raw_index, super::model::Table::Declaration)?,
                Some(position(raw_prior, super::model::Table::Declaration)?),
            ));
        }
    }
    for (raw_index, _) in record.declarations.functions.iter().enumerate() {
        if let Some(raw_prior) = duplicate_position(&record.declarations.functions, raw_index) {
            return Err(reject(
                super::model::FailureClass::DuplicateDeclaration,
                super::model::Table::Declaration,
                position(raw_index, super::model::Table::Declaration)?,
                Some(position(raw_prior, super::model::Table::Declaration)?),
            ));
        }
    }
    for (raw_index, _) in record.declarations.types.iter().enumerate() {
        if let Some(raw_prior) = duplicate_position(&record.declarations.types, raw_index) {
            return Err(reject(
                super::model::FailureClass::DuplicateDeclaration,
                super::model::Table::Declaration,
                position(raw_index, super::model::Table::Declaration)?,
                Some(position(raw_prior, super::model::Table::Declaration)?),
            ));
        }
    }
    for (raw_index, _) in record.declarations.places.iter().enumerate() {
        if let Some(raw_prior) = duplicate_position(&record.declarations.places, raw_index) {
            return Err(reject(
                super::model::FailureClass::DuplicateDeclaration,
                super::model::Table::Declaration,
                position(raw_index, super::model::Table::Declaration)?,
                Some(position(raw_prior, super::model::Table::Declaration)?),
            ));
        }
    }
    Ok(())
}

pub(super) fn mapping_for(record: &super::model::Record, source_block: u32) -> Option<&super::model::BlockMapping> {
    record.mappings.iter().find(|mapping| mapping.source_block == source_block)
}

pub(super) fn emission_is_allowed(
    record: &super::model::Record,
    source_block: u32,
    block: super::model::BlockKey,
) -> bool {
    mapping_for(record, source_block)
        .is_some_and(|mapping| mapping.direct == block || mapping.expansions.contains(&block))
}

fn block_was_assigned(record: &super::model::Record, raw_mapping: usize, block: super::model::BlockKey) -> bool {
    record.mappings[..raw_mapping]
        .iter()
        .any(|mapping| mapping.direct == block || mapping.expansions.contains(&block))
}

fn check_assigned_blocks(
    record: &super::model::Record,
    raw_mapping: usize,
    mapping: &super::model::BlockMapping,
    index: u32,
) -> Result<(), super::model::Rejection> {
    if !record.declarations.blocks.contains(&mapping.direct) {
        return Err(reject(
            super::model::FailureClass::MissingDeclaration,
            super::model::Table::Mapping,
            index,
            Some(mapping.direct.0),
        ));
    }
    if block_was_assigned(record, raw_mapping, mapping.direct) {
        return Err(reject(
            super::model::FailureClass::DuplicateBlockAssignment,
            super::model::Table::Mapping,
            index,
            Some(mapping.direct.0),
        ));
    }
    width(mapping.expansions.len(), super::model::Table::Mapping)?;
    for (raw_expansion, expansion) in mapping.expansions.iter().copied().enumerate() {
        if !record.declarations.blocks.contains(&expansion) {
            return Err(reject(
                super::model::FailureClass::MissingDeclaration,
                super::model::Table::Mapping,
                index,
                Some(expansion.0),
            ));
        }
        if expansion == mapping.direct
            || mapping.expansions[..raw_expansion].contains(&expansion)
            || block_was_assigned(record, raw_mapping, expansion)
        {
            return Err(reject(
                super::model::FailureClass::DuplicateBlockAssignment,
                super::model::Table::Mapping,
                index,
                Some(expansion.0),
            ));
        }
    }
    Ok(())
}

// r[impl backend-trace.block-mapping]
pub(super) fn check_mappings(
    body: &crate::mir_core::Body,
    record: &super::model::Record,
) -> Result<u32, super::model::Rejection> {
    crate::mir_core::admit_body(body).map_err(|_| super::model::Rejection {
        class: super::model::FailureClass::InvalidBody,
        table: super::model::Table::Mapping,
        index: None,
        subject: None,
    })?;
    let block_count = position(body.blocks.len(), super::model::Table::Mapping)?;
    width(record.mappings.len(), super::model::Table::Mapping)?;

    for (raw_index, mapping) in record.mappings.iter().enumerate() {
        let index = position(raw_index, super::model::Table::Mapping)?;
        if !super::proofs::index_admitted(mapping.source_block, block_count) {
            return Err(reject(
                super::model::FailureClass::InvalidSourceBlock,
                super::model::Table::Mapping,
                index,
                Some(mapping.source_block),
            ));
        }
        if record.mappings[..raw_index].iter().any(|prior| prior.source_block == mapping.source_block) {
            return Err(reject(
                super::model::FailureClass::DuplicateMapping,
                super::model::Table::Mapping,
                index,
                Some(mapping.source_block),
            ));
        }
        check_assigned_blocks(record, raw_index, mapping, index)?;
    }

    for source_block in 0..block_count {
        if mapping_for(record, source_block).is_none() {
            return Err(reject(
                super::model::FailureClass::MissingMapping,
                super::model::Table::Mapping,
                source_block,
                Some(source_block),
            ));
        }
    }
    Ok(block_count)
}

// r[impl backend-trace.ssa-like-locals]
pub(super) fn check_locals(
    body: &crate::mir_core::Body,
    record: &super::model::Record,
) -> Result<u32, super::model::Rejection> {
    let local_count = position(body.locals.len(), super::model::Table::Local)?;
    width(record.locals.len(), super::model::Table::Local)?;
    for (raw_index, class) in record.locals.iter().copied().enumerate() {
        let index = position(raw_index, super::model::Table::Local)?;
        if !super::proofs::index_admitted(class.local, local_count) {
            return Err(reject(
                super::model::FailureClass::InvalidLocal,
                super::model::Table::Local,
                index,
                Some(class.local),
            ));
        }
        if record.locals[..raw_index].iter().any(|prior| prior.local == class.local) {
            return Err(reject(
                super::model::FailureClass::ConflictingLocalClass,
                super::model::Table::Local,
                index,
                Some(class.local),
            ));
        }
    }
    for local in 0..local_count {
        if !record.locals.iter().any(|class| class.local == local) {
            return Err(reject(
                super::model::FailureClass::MissingLocalClass,
                super::model::Table::Local,
                local,
                Some(local),
            ));
        }
    }
    Ok(local_count)
}
