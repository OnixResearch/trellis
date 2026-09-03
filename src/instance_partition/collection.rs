//! Concrete-instance validation and bounded reachable-item collection.

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

pub(super) fn item_for(record: &super::model::Record, key: super::model::ItemKey) -> Option<&super::model::Item> {
    record.items.iter().find(|item| item.key == key)
}

fn expected_stability(item: &super::model::Item) -> super::model::Stability {
    match (item.origin, item.shape) {
        (super::model::Origin::Local, super::model::ConcreteShape::Plain | super::model::ConcreteShape::Static) => {
            super::model::Stability::Stable
        }
        (super::model::Origin::Local | super::model::Origin::External, _) => super::model::Stability::Volatile,
    }
}

fn item_shape_is_admitted(item: &super::model::Item) -> bool {
    let has_arguments = !item.type_arguments.is_empty() || !item.const_arguments.is_empty();
    match (item.kind, item.shape, has_arguments, item.shim_source) {
        (super::model::ItemKind::Function, super::model::ConcreteShape::Plain, false, None)
        | (
            super::model::ItemKind::Function,
            super::model::ConcreteShape::Generic | super::model::ConcreteShape::Specialized,
            true,
            None,
        )
        | (super::model::ItemKind::Static, super::model::ConcreteShape::Static, false, None)
        | (super::model::ItemKind::Shim, super::model::ConcreteShape::Shim, false, Some(_)) => true,
        (super::model::ItemKind::Function | super::model::ItemKind::Static | super::model::ItemKind::Shim, _, _, _) => {
            false
        }
    }
}

fn validate_items(record: &super::model::Record) -> Result<(), super::model::Rejection> {
    width(record.items.len(), super::model::Table::Item)?;
    for (raw_index, item) in record.items.iter().enumerate() {
        let index = position(raw_index, super::model::Table::Item)?;
        if record.items[..raw_index].iter().any(|prior| prior.key == item.key) {
            return Err(reject(
                super::model::FailureClass::DuplicateItem,
                super::model::Table::Item,
                index,
                Some(item.key.0),
            ));
        }
        if !item_shape_is_admitted(item) {
            return Err(reject(
                super::model::FailureClass::MalformedInstance,
                super::model::Table::Item,
                index,
                Some(item.key.0),
            ));
        }
        if item.stability != expected_stability(item) {
            return Err(reject(
                super::model::FailureClass::ClassificationMismatch,
                super::model::Table::Item,
                index,
                Some(item.key.0),
            ));
        }
    }
    for (raw_index, item) in record.items.iter().enumerate() {
        if let Some(source) = item.shim_source {
            if item_for(record, source).is_none() {
                return Err(reject(
                    super::model::FailureClass::DanglingReference,
                    super::model::Table::Item,
                    position(raw_index, super::model::Table::Item)?,
                    Some(source.0),
                ));
            }
        }
    }
    Ok(())
}

fn validate_graph(record: &super::model::Record) -> Result<(), super::model::Rejection> {
    validate_items(record)?;
    width(record.roots.len(), super::model::Table::Root)?;
    width(record.references.len(), super::model::Table::Reference)?;
    for (raw_index, root) in record.roots.iter().copied().enumerate() {
        if item_for(record, root).is_none() {
            return Err(reject(
                super::model::FailureClass::MissingRoot,
                super::model::Table::Root,
                position(raw_index, super::model::Table::Root)?,
                Some(root.0),
            ));
        }
    }
    for (raw_index, edge) in record.references.iter().copied().enumerate() {
        if item_for(record, edge.source).is_none() || item_for(record, edge.target).is_none() {
            let subject = if item_for(record, edge.source).is_none() {
                edge.source.0
            } else {
                edge.target.0
            };
            return Err(reject(
                super::model::FailureClass::DanglingReference,
                super::model::Table::Reference,
                position(raw_index, super::model::Table::Reference)?,
                Some(subject),
            ));
        }
    }
    Ok(())
}

/// Returns deterministic canonical reachability from roots in edge order.
// r[impl mono.collection]
// r[impl mono.dedup]
pub fn collect_reachable(record: &super::model::Record) -> Result<Vec<super::model::ItemKey>, super::model::Rejection> {
    validate_graph(record)?;
    let mut reachable = Vec::with_capacity(record.items.len());
    for root in &record.roots {
        if !reachable.contains(root) {
            reachable.push(*root);
        }
    }
    let mut cursor = 0_usize;
    while cursor < reachable.len() {
        let source = match reachable.get(cursor).copied() {
            Some(source) => source,
            None => {
                return Err(super::model::Rejection {
                    class: super::model::FailureClass::TableTooWide,
                    table: super::model::Table::Reference,
                    index: None,
                    subject: None,
                });
            }
        };
        cursor = cursor.checked_add(1).ok_or(super::model::Rejection {
            class: super::model::FailureClass::TableTooWide,
            table: super::model::Table::Reference,
            index: None,
            subject: None,
        })?;
        for edge in record.references.iter().filter(|edge| edge.source == source) {
            if !reachable.contains(&edge.target) {
                reachable.push(edge.target);
            }
        }
    }
    Ok(reachable)
}
