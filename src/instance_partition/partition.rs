//! Canonical collection output and disjoint complete unit admission.

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
    if count > super::model::MAX_ITEMS {
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

// r[impl mono.collection]
// r[impl mono.dedup]
pub(super) fn check_collected(
    record: &super::model::Record,
    reachable: &[super::model::ItemKey],
) -> Result<u32, super::model::Rejection> {
    let count = width(record.collected.len(), super::model::Table::Collected)?;
    for (raw_index, key) in record.collected.iter().copied().enumerate() {
        let index = position(raw_index, super::model::Table::Collected)?;
        if record.collected[..raw_index].contains(&key) {
            return Err(reject(
                super::model::FailureClass::DuplicateCollected,
                super::model::Table::Collected,
                index,
                Some(key.0),
            ));
        }
        if !reachable.contains(&key) {
            return Err(reject(
                super::model::FailureClass::ExtraCollected,
                super::model::Table::Collected,
                index,
                Some(key.0),
            ));
        }
    }
    for (raw_index, key) in reachable.iter().copied().enumerate() {
        if !record.collected.contains(&key) {
            return Err(reject(
                super::model::FailureClass::MissingReachable,
                super::model::Table::Collected,
                position(raw_index, super::model::Table::Collected)?,
                Some(key.0),
            ));
        }
    }
    Ok(count)
}

fn appears_before(record: &super::model::Record, raw_unit: usize, key: super::model::ItemKey) -> bool {
    record.units[..raw_unit].iter().any(|unit| unit.items.contains(&key))
}

// r[impl mono.cgu-classification]
// r[impl mono.cgu-partition]
pub(super) fn check_units(record: &super::model::Record) -> Result<u32, super::model::Rejection> {
    let count = width(record.units.len(), super::model::Table::Unit)?;
    for (raw_unit, unit) in record.units.iter().enumerate() {
        let unit_index = position(raw_unit, super::model::Table::Unit)?;
        if record.units[..raw_unit].iter().any(|prior| prior.key == unit.key) {
            return Err(reject(
                super::model::FailureClass::DuplicateUnit,
                super::model::Table::Unit,
                unit_index,
                Some(unit.key.0),
            ));
        }
        width(unit.items.len(), super::model::Table::Unit)?;
        for (raw_item, key) in unit.items.iter().copied().enumerate() {
            if !record.collected.contains(&key) {
                return Err(reject(
                    super::model::FailureClass::UnknownUnitItem,
                    super::model::Table::Unit,
                    unit_index,
                    Some(key.0),
                ));
            }
            if unit.items[..raw_item].contains(&key) || appears_before(record, raw_unit, key) {
                return Err(reject(
                    super::model::FailureClass::PartitionOverlap,
                    super::model::Table::Unit,
                    unit_index,
                    Some(key.0),
                ));
            }
            let item = super::collection::item_for(record, key).ok_or_else(|| {
                reject(super::model::FailureClass::UnknownUnitItem, super::model::Table::Unit, unit_index, Some(key.0))
            })?;
            if item.stability != unit.stability {
                return Err(reject(
                    super::model::FailureClass::UnitClassMismatch,
                    super::model::Table::Unit,
                    unit_index,
                    Some(key.0),
                ));
            }
        }
    }
    for (raw_index, key) in record.collected.iter().copied().enumerate() {
        if !record.units.iter().any(|unit| unit.items.contains(&key)) {
            return Err(reject(
                super::model::FailureClass::PartitionOmission,
                super::model::Table::Collected,
                position(raw_index, super::model::Table::Collected)?,
                Some(key.0),
            ));
        }
    }
    Ok(count)
}
