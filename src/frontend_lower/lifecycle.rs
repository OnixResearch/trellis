//! Created-ID use and optional definition-correspondence checks.

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

fn created_entry(record: &super::model::Record, id: super::model::ScopedKey) -> Option<super::model::CreatedKey> {
    record.created_ids.iter().find(|created| created.id == id).copied()
}

fn key_is_used(record: &super::model::Record, id: super::model::ScopedKey) -> bool {
    record.used_ids.contains(&id)
}

fn check_created_ids(record: &super::model::Record) -> Result<(), super::model::Rejection> {
    for (raw_index, created) in record.created_ids.iter().copied().enumerate() {
        let index = position(raw_index, super::model::Table::Created)?;
        if !super::proofs::owner_admitted(created.id.owner.0, record.owner.0) {
            return Err(reject(
                super::model::FailureClass::WrongOwner,
                super::model::Table::Created,
                index,
                Some(created.id.owner.0),
            ));
        }
        if let Some(raw_prior) = record.created_ids[..raw_index].iter().position(|prior| prior.id == created.id) {
            return Err(reject(
                super::model::FailureClass::DuplicateCreatedId,
                super::model::Table::Created,
                index,
                Some(position(raw_prior, super::model::Table::Created)?),
            ));
        }
    }
    Ok(())
}

fn require_created(
    record: &super::model::Record,
    id: super::model::ScopedKey,
    table: super::model::Table,
    index: u32,
) -> Result<(), super::model::Rejection> {
    let created = match created_entry(record, id) {
        Some(created) => created,
        None => {
            return Err(reject(super::model::FailureClass::MissingCreatedId, table, index, Some(id.local)));
        }
    };
    if created.policy != super::model::UsePolicy::Required {
        return Err(reject(super::model::FailureClass::DiscardedIdUsed, table, index, Some(id.local)));
    }
    Ok(())
}

fn check_referenced_ids(record: &super::model::Record) -> Result<(), super::model::Rejection> {
    for (raw_index, mapping) in record.mappings.iter().copied().enumerate() {
        require_created(
            record,
            mapping.target,
            super::model::Table::Mapping,
            position(raw_index, super::model::Table::Mapping)?,
        )?;
    }
    for (raw_index, generated) in record.generated_nodes.iter().copied().enumerate() {
        require_created(
            record,
            generated.target,
            super::model::Table::Generated,
            position(raw_index, super::model::Table::Generated)?,
        )?;
    }
    Ok(())
}

fn check_used_ids(record: &super::model::Record) -> Result<(), super::model::Rejection> {
    for (raw_index, used) in record.used_ids.iter().copied().enumerate() {
        let index = position(raw_index, super::model::Table::Used)?;
        if !super::proofs::owner_admitted(used.owner.0, record.owner.0) {
            return Err(reject(
                super::model::FailureClass::WrongOwner,
                super::model::Table::Used,
                index,
                Some(used.owner.0),
            ));
        }
        if created_entry(record, used).is_none() {
            return Err(reject(
                super::model::FailureClass::UnknownUsedId,
                super::model::Table::Used,
                index,
                Some(used.local),
            ));
        }
        if let Some(raw_prior) = record.used_ids[..raw_index].iter().position(|prior| *prior == used) {
            return Err(reject(
                super::model::FailureClass::DuplicateUsedId,
                super::model::Table::Used,
                index,
                Some(position(raw_prior, super::model::Table::Used)?),
            ));
        }
    }
    Ok(())
}

fn check_use_policies(record: &super::model::Record) -> Result<(), super::model::Rejection> {
    for (raw_index, created) in record.created_ids.iter().copied().enumerate() {
        let required_tag = match created.policy {
            super::model::UsePolicy::Required => super::proofs::PRESENT_TAG,
            super::model::UsePolicy::ExplicitDiscard => super::proofs::MISSING_TAG,
        };
        let used_tag = if key_is_used(record, created.id) {
            super::proofs::PRESENT_TAG
        } else {
            super::proofs::MISSING_TAG
        };
        if !super::proofs::created_use_admitted(required_tag, used_tag) {
            let class = match created.policy {
                super::model::UsePolicy::Required => super::model::FailureClass::UnusedCreatedId,
                super::model::UsePolicy::ExplicitDiscard => super::model::FailureClass::DiscardedIdUsed,
            };
            return Err(reject(
                class,
                super::model::Table::Created,
                position(raw_index, super::model::Table::Created)?,
                Some(created.id.local),
            ));
        }
    }
    Ok(())
}

// r[impl hir-id.admission]
pub(super) fn check_usage(record: &super::model::Record) -> Result<(), super::model::Rejection> {
    check_created_ids(record)?;
    check_referenced_ids(record)?;
    check_used_ids(record)?;
    check_use_policies(record)
}

// r[impl hir-id.def-correspondence]
pub(super) fn check_definitions(record: &super::model::Record) -> Result<(), super::model::Rejection> {
    for (raw_index, source) in record.definition_requirements.iter().copied().enumerate() {
        let index = position(raw_index, super::model::Table::DefinitionRequirement)?;
        if !super::catalog::source_is_declared(record, source)
            || !record.mappings.iter().any(|mapping| mapping.source == source)
        {
            return Err(reject(
                super::model::FailureClass::UnknownDefinitionSource,
                super::model::Table::DefinitionRequirement,
                index,
                Some(source.0),
            ));
        }
        if let Some(raw_prior) = record.definition_requirements[..raw_index].iter().position(|prior| *prior == source) {
            return Err(reject(
                super::model::FailureClass::DuplicateDefinition,
                super::model::Table::DefinitionRequirement,
                index,
                Some(position(raw_prior, super::model::Table::DefinitionRequirement)?),
            ));
        }
    }

    for (raw_index, definition) in record.definitions.iter().copied().enumerate() {
        let index = position(raw_index, super::model::Table::Definition)?;
        if !record.definition_requirements.contains(&definition.source) {
            return Err(reject(
                super::model::FailureClass::UnknownDefinitionSource,
                super::model::Table::Definition,
                index,
                Some(definition.source.0),
            ));
        }
        if let Some(raw_prior) = record.definitions[..raw_index]
            .iter()
            .position(|prior| prior.source == definition.source || prior.definition == definition.definition)
        {
            return Err(reject(
                super::model::FailureClass::DuplicateDefinition,
                super::model::Table::Definition,
                index,
                Some(position(raw_prior, super::model::Table::Definition)?),
            ));
        }
    }

    for (raw_index, source) in record.definition_requirements.iter().copied().enumerate() {
        if !record.definitions.iter().any(|definition| definition.source == source) {
            return Err(reject(
                super::model::FailureClass::MissingDefinition,
                super::model::Table::DefinitionRequirement,
                position(raw_index, super::model::Table::DefinitionRequirement)?,
                Some(source.0),
            ));
        }
    }
    Ok(())
}
