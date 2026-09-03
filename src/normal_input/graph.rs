//! Finite graph, type-shape, acyclicity, and required-field admission.

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
    if count > super::model::MAX_ROWS {
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

pub(super) fn node_for(record: &super::model::Record, key: super::model::NodeKey) -> Option<&super::model::Node> {
    record.nodes.iter().find(|node| node.key == key)
}

fn node_position(record: &super::model::Record, key: super::model::NodeKey) -> Option<usize> {
    record.nodes.iter().position(|node| node.key == key)
}

fn expected_type(kind: &super::model::Kind) -> Option<super::model::TypeTag> {
    match kind {
        super::model::Kind::Scalar(super::model::Scalar::Bool(_)) => Some(super::model::TypeTag::Bool),
        super::model::Kind::Scalar(super::model::Scalar::Natural(_)) => Some(super::model::TypeTag::Natural),
        super::model::Kind::Scalar(super::model::Scalar::Text(_)) => Some(super::model::TypeTag::Text),
        super::model::Kind::List(_) => Some(super::model::TypeTag::List),
        super::model::Kind::Record(_) => Some(super::model::TypeTag::Record),
        super::model::Kind::Variant { .. } => Some(super::model::TypeTag::Variant),
        super::model::Kind::Optional(_) => Some(super::model::TypeTag::Optional),
        super::model::Kind::Reference(_) => Some(super::model::TypeTag::Reference),
        super::model::Kind::Import { .. } => Some(super::model::TypeTag::Import),
        super::model::Kind::Unsupported => None,
    }
}

fn children(node: &super::model::Node) -> Vec<super::model::NodeKey> {
    match &node.kind {
        super::model::Kind::List(items) => items.clone(),
        super::model::Kind::Record(fields) => fields.iter().map(|field| field.value).collect(),
        super::model::Kind::Variant { value: Some(value), .. }
        | super::model::Kind::Optional(Some(value))
        | super::model::Kind::Reference(value) => vec![*value],
        super::model::Kind::Scalar(_)
        | super::model::Kind::Variant { value: None, .. }
        | super::model::Kind::Optional(None)
        | super::model::Kind::Import { .. }
        | super::model::Kind::Unsupported => Vec::new(),
    }
}

fn check_record_fields(fields: &[super::model::Field], node_index: u32) -> Result<(), super::model::Rejection> {
    width(fields.len(), super::model::Table::Node)?;
    for (raw_index, field) in fields.iter().enumerate() {
        if field.name.is_empty() || fields[..raw_index].iter().any(|prior| prior.name == field.name) {
            return Err(reject(
                super::model::FailureClass::DuplicateField,
                super::model::Table::Node,
                node_index,
                Some(position(raw_index, super::model::Table::Node)?),
            ));
        }
    }
    Ok(())
}

fn check_node_shapes(record: &super::model::Record) -> Result<u32, super::model::Rejection> {
    let count = width(record.nodes.len(), super::model::Table::Node)?;
    if node_for(record, record.root).is_none() {
        return Err(super::model::Rejection {
            class: super::model::FailureClass::MissingRoot,
            table: super::model::Table::Node,
            index: None,
            subject: Some(record.root.0),
        });
    }
    for (raw_index, node) in record.nodes.iter().enumerate() {
        let index = position(raw_index, super::model::Table::Node)?;
        if record.nodes[..raw_index].iter().any(|prior| prior.key == node.key) {
            return Err(reject(
                super::model::FailureClass::DuplicateNode,
                super::model::Table::Node,
                index,
                Some(node.key.0),
            ));
        }
        let type_tag = match expected_type(&node.kind) {
            Some(type_tag) => type_tag,
            None => {
                return Err(reject(
                    super::model::FailureClass::UnsupportedForm,
                    super::model::Table::Node,
                    index,
                    Some(node.key.0),
                ));
            }
        };
        if type_tag != node.declared_type {
            return Err(reject(
                super::model::FailureClass::TypeMismatch,
                super::model::Table::Node,
                index,
                Some(node.key.0),
            ));
        }
        if let super::model::Kind::Record(fields) = &node.kind {
            check_record_fields(fields, index)?;
        }
        if let super::model::Kind::Import { edge } = node.kind {
            let import_count = width(record.imports.len(), super::model::Table::Import)?;
            if !super::proofs::index_admitted(edge, import_count) {
                return Err(reject(
                    super::model::FailureClass::InvalidImportIndex,
                    super::model::Table::Node,
                    index,
                    Some(edge),
                ));
            }
        }
        for child in children(node) {
            if node_for(record, child).is_none() {
                return Err(reject(
                    super::model::FailureClass::DanglingNode,
                    super::model::Table::Node,
                    index,
                    Some(child.0),
                ));
            }
        }
    }
    Ok(count)
}

fn build_indegree(record: &super::model::Record) -> Result<Vec<u32>, super::model::Rejection> {
    let mut indegree = vec![0_u32; record.nodes.len()];
    for node in &record.nodes {
        for child in children(node) {
            let target = node_position(record, child).ok_or(super::model::Rejection {
                class: super::model::FailureClass::DanglingNode,
                table: super::model::Table::Node,
                index: None,
                subject: Some(child.0),
            })?;
            let degree = indegree.get_mut(target).ok_or(super::model::Rejection {
                class: super::model::FailureClass::DanglingNode,
                table: super::model::Table::Node,
                index: None,
                subject: Some(child.0),
            })?;
            *degree = degree.checked_add(1).ok_or(super::model::Rejection {
                class: super::model::FailureClass::TableTooWide,
                table: super::model::Table::Node,
                index: None,
                subject: Some(child.0),
            })?;
        }
    }
    Ok(indegree)
}

fn check_acyclic(record: &super::model::Record) -> Result<(), super::model::Rejection> {
    let mut indegree = build_indegree(record)?;
    let mut queue = Vec::with_capacity(record.nodes.len());
    for (raw_index, degree) in indegree.iter().copied().enumerate() {
        if degree == 0 {
            queue.push(raw_index);
        }
    }
    let mut cursor = 0_usize;
    while cursor < queue.len() {
        let source = *queue.get(cursor).ok_or(super::model::Rejection {
            class: super::model::FailureClass::Cycle,
            table: super::model::Table::Node,
            index: None,
            subject: None,
        })?;
        cursor = cursor.checked_add(1).ok_or(super::model::Rejection {
            class: super::model::FailureClass::TableTooWide,
            table: super::model::Table::Node,
            index: None,
            subject: None,
        })?;
        let node = record.nodes.get(source).ok_or(super::model::Rejection {
            class: super::model::FailureClass::DanglingNode,
            table: super::model::Table::Node,
            index: None,
            subject: None,
        })?;
        for child in children(node) {
            let target = node_position(record, child).ok_or(super::model::Rejection {
                class: super::model::FailureClass::DanglingNode,
                table: super::model::Table::Node,
                index: None,
                subject: Some(child.0),
            })?;
            let degree = indegree.get_mut(target).ok_or(super::model::Rejection {
                class: super::model::FailureClass::DanglingNode,
                table: super::model::Table::Node,
                index: None,
                subject: Some(child.0),
            })?;
            *degree = degree.saturating_sub(1);
            if *degree == 0 {
                queue.push(target);
            }
        }
    }
    if queue.len() == record.nodes.len() {
        Ok(())
    } else {
        Err(super::model::Rejection {
            class: super::model::FailureClass::Cycle,
            table: super::model::Table::Node,
            index: None,
            subject: None,
        })
    }
}

pub(super) fn check_structure(record: &super::model::Record) -> Result<u32, super::model::Rejection> {
    let count = check_node_shapes(record)?;
    check_acyclic(record)?;
    Ok(count)
}
