//! Deterministic collection and partition admission orchestration.

fn width(len: usize, table: super::model::Table) -> Result<u32, super::model::Rejection> {
    let count = u32::try_from(len).map_err(|_| super::model::Rejection {
        class: super::model::FailureClass::TableTooWide,
        table,
        index: None,
        subject: None,
    })?;
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

/// Admits one finite reachability collection and disjoint complete partition.
// r[impl mono.item-envelope]
// r[impl mono.collection]
// r[impl mono.dedup]
// r[impl mono.cgu-classification]
// r[impl mono.cgu-partition]
// r[impl mono.admission]
pub fn admit_record(record: &super::model::Record) -> Result<super::model::Facts, super::model::Rejection> {
    let reachable = super::collection::collect_reachable(record)?;
    let collected_count = super::partition::check_collected(record, &reachable)?;
    let unit_count = super::partition::check_units(record)?;
    Ok(super::model::Facts {
        item_count: width(record.items.len(), super::model::Table::Item)?,
        root_count: width(record.roots.len(), super::model::Table::Root)?,
        collected_count,
        unit_count,
    })
}
