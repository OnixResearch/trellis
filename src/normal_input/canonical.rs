//! Deterministic source-independent normal-form and digest-input selection.

fn reject(class: super::model::FailureClass, subject: Option<u32>) -> super::model::Rejection {
    super::model::Rejection {
        class,
        table: super::model::Table::Node,
        index: None,
        subject,
    }
}

fn list_children(items: &[super::model::NodeKey]) -> Result<Vec<super::model::NormalChild>, super::model::Rejection> {
    let mut children = Vec::with_capacity(items.len());
    for (raw_index, target) in items.iter().copied().enumerate() {
        let index =
            u32::try_from(raw_index).map_err(|_| reject(super::model::FailureClass::TableTooWide, Some(target.0)))?;
        children.push(super::model::NormalChild {
            label: index.to_string(),
            target,
        });
    }
    Ok(children)
}

fn record_children(fields: &[super::model::Field]) -> Vec<super::model::NormalChild> {
    let mut children: Vec<super::model::NormalChild> = fields
        .iter()
        .map(|field| super::model::NormalChild {
            label: field.name.clone(),
            target: field.value,
        })
        .collect();
    children.sort();
    children
}

fn normal_entry(node: &super::model::Node) -> Result<super::model::NormalEntry, super::model::Rejection> {
    let (payload, children) = match &node.kind {
        super::model::Kind::Scalar(super::model::Scalar::Bool(value)) => {
            (if *value { "true" } else { "false" }.to_owned(), Vec::new())
        }
        super::model::Kind::Scalar(super::model::Scalar::Natural(value)) => (value.to_string(), Vec::new()),
        super::model::Kind::Scalar(super::model::Scalar::Text(value)) => (value.clone(), Vec::new()),
        super::model::Kind::List(items) => (String::new(), list_children(items)?),
        super::model::Kind::Record(fields) => (String::new(), record_children(fields)),
        super::model::Kind::Variant { tag, value } => {
            let children = value.map_or_else(Vec::new, |target| {
                vec![super::model::NormalChild {
                    label: "value".to_owned(),
                    target,
                }]
            });
            (tag.clone(), children)
        }
        super::model::Kind::Optional(value) => {
            let children = value.map_or_else(Vec::new, |target| {
                vec![super::model::NormalChild {
                    label: "value".to_owned(),
                    target,
                }]
            });
            (if value.is_some() { "some" } else { "none" }.to_owned(), children)
        }
        super::model::Kind::Reference(target) => (String::new(), vec![super::model::NormalChild {
            label: "reference".to_owned(),
            target: *target,
        }]),
        super::model::Kind::Import { edge } => (edge.to_string(), Vec::new()),
        super::model::Kind::Unsupported => {
            return Err(reject(super::model::FailureClass::UnsupportedForm, Some(node.key.0)));
        }
    };
    Ok(super::model::NormalEntry {
        key: node.key,
        type_tag: node.declared_type,
        payload,
        children,
    })
}

/// Produces canonical key and record-field order after complete admission.
// r[impl domain-primitives.typed-config.normal-form]
pub fn normalize(record: &super::model::Record) -> Result<super::model::NormalForm, super::model::Rejection> {
    super::graph::check_structure(record)?;
    super::schema::check_required_fields(record)?;
    super::policy::admit_imports(record)?;
    let mut entries = Vec::with_capacity(record.nodes.len());
    for node in &record.nodes {
        entries.push(normal_entry(node)?);
    }
    entries.sort();
    Ok(super::model::NormalForm {
        root: record.root,
        entries,
    })
}

/// Selects exact canonical metadata and declared dependency digests.
// r[impl domain-primitives.typed-config.semantic-digest]
pub fn select_digest_input(
    record: &super::model::Record,
) -> Result<super::model::DigestInput, super::model::Rejection> {
    Ok(super::model::DigestInput {
        normal: normalize(record)?,
        dependencies: record.dependency_digests.clone(),
    })
}

/// Admits stable digest inputs or an explicit compatibility boundary.
// r[impl domain-primitives.typed-config.semantic-refactoring]
pub fn admit_refactor(request: &super::model::Refactor) -> Result<(), super::model::Rejection> {
    if request.before == request.after
        || matches!(
            request.compatibility,
            super::model::Compatibility::Compatible | super::model::Compatibility::UnresolvedDependents
        )
    {
        Ok(())
    } else {
        Err(super::model::Rejection {
            class: super::model::FailureClass::DigestChangingRefactor,
            table: super::model::Table::Refactor,
            index: None,
            subject: None,
        })
    }
}
