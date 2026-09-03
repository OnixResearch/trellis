//! Structural path and caller-supplied type-metadata admission.

fn reject(
    class: super::model::FailureClass,
    table: super::model::Table,
    index: Option<u32>,
    subject: Option<u32>,
) -> super::model::Rejection {
    super::model::Rejection {
        class,
        table,
        index,
        subject,
    }
}

fn position(index: usize, table: super::model::Table) -> Result<u32, super::model::Rejection> {
    u32::try_from(index).map_err(|_| reject(super::model::FailureClass::TableTooWide, table, None, None))
}

fn width(len: usize, table: super::model::Table) -> Result<u32, super::model::Rejection> {
    let count = position(len, table)?;
    if count > super::model::MAX_PATHS {
        Err(reject(super::model::FailureClass::TableTooWide, table, None, Some(count)))
    } else {
        Ok(count)
    }
}

pub(super) fn path_for(record: &super::model::Record, key: super::model::PathKey) -> Option<&super::model::Path> {
    record.paths.iter().find(|path| path.key == key)
}

pub(super) fn metadata_for(
    record: &super::model::Record,
    key: super::model::PathKey,
) -> Option<&super::model::Metadata> {
    record.metadata.iter().find(|metadata| metadata.path == key)
}

pub(super) fn shape_has_glue(shape: &super::model::Shape) -> bool {
    !matches!(shape, super::model::Shape::NoGlue)
}

pub(super) fn descendant_or_same(
    record: &super::model::Record,
    candidate: super::model::PathKey,
    ancestor: super::model::PathKey,
) -> bool {
    let mut current = Some(candidate);
    let mut remaining = record.paths.len();
    while let Some(key) = current {
        if key == ancestor {
            return true;
        }
        if remaining == 0 {
            return false;
        }
        remaining = remaining.saturating_sub(1);
        current = path_for(record, key).and_then(|path| path.parent);
    }
    false
}

fn check_path_rows(record: &super::model::Record) -> Result<u32, super::model::Rejection> {
    let count = width(record.paths.len(), super::model::Table::Path)?;
    let local_count = u32::try_from(record.body.locals.len())
        .map_err(|_| reject(super::model::FailureClass::TableTooWide, super::model::Table::Path, None, None))?;
    for (raw_index, path) in record.paths.iter().enumerate() {
        let index = position(raw_index, super::model::Table::Path)?;
        if record.paths[..raw_index].iter().any(|prior| prior.key == path.key) {
            return Err(reject(
                super::model::FailureClass::DuplicatePath,
                super::model::Table::Path,
                Some(index),
                Some(path.key.0),
            ));
        }
        crate::mir_core::admit_place(&path.place, local_count).map_err(|_| {
            reject(super::model::FailureClass::InvalidPath, super::model::Table::Path, Some(index), Some(path.key.0))
        })?;
        match path.parent {
            None if path.depth == 0 => {}
            None => {
                return Err(reject(
                    super::model::FailureClass::InvalidDepth,
                    super::model::Table::Path,
                    Some(index),
                    Some(path.key.0),
                ));
            }
            Some(parent) => {
                let parent_path = path_for(record, parent).ok_or_else(|| {
                    reject(
                        super::model::FailureClass::InvalidParent,
                        super::model::Table::Path,
                        Some(index),
                        Some(parent.0),
                    )
                })?;
                if parent_path.depth.checked_add(1) != Some(path.depth) {
                    return Err(reject(
                        super::model::FailureClass::InvalidDepth,
                        super::model::Table::Path,
                        Some(index),
                        Some(path.key.0),
                    ));
                }
            }
        }
    }
    Ok(count)
}

fn check_children(
    record: &super::model::Record,
    parent: super::model::PathKey,
    children: &[super::model::PathKey],
    index: u32,
) -> Result<(), super::model::Rejection> {
    width(children.len(), super::model::Table::Metadata)?;
    for (raw_child, child) in children.iter().copied().enumerate() {
        let path = path_for(record, child).ok_or_else(|| {
            reject(super::model::FailureClass::InvalidChild, super::model::Table::Metadata, Some(index), Some(child.0))
        })?;
        if path.parent != Some(parent) || children[..raw_child].contains(&child) {
            return Err(reject(
                super::model::FailureClass::InvalidChild,
                super::model::Table::Metadata,
                Some(index),
                Some(child.0),
            ));
        }
    }
    Ok(())
}

fn check_metadata_rows(record: &super::model::Record) -> Result<u32, super::model::Rejection> {
    let count = width(record.metadata.len(), super::model::Table::Metadata)?;
    for (raw_index, metadata) in record.metadata.iter().enumerate() {
        let index = position(raw_index, super::model::Table::Metadata)?;
        if path_for(record, metadata.path).is_none() {
            return Err(reject(
                super::model::FailureClass::InvalidPath,
                super::model::Table::Metadata,
                Some(index),
                Some(metadata.path.0),
            ));
        }
        if record.metadata[..raw_index].iter().any(|prior| prior.path == metadata.path) {
            return Err(reject(
                super::model::FailureClass::DuplicateMetadata,
                super::model::Table::Metadata,
                Some(index),
                Some(metadata.path.0),
            ));
        }
        match &metadata.shape {
            super::model::Shape::NoGlue | super::model::Shape::Direct => {}
            super::model::Shape::Fields(children) => {
                check_children(record, metadata.path, children, index)?;
            }
            super::model::Shape::Enum(variants) => {
                for (raw_variant, variant) in variants.iter().enumerate() {
                    if variants[..raw_variant].iter().any(|prior| prior.variant == variant.variant) {
                        return Err(reject(
                            super::model::FailureClass::InvalidChild,
                            super::model::Table::Metadata,
                            Some(index),
                            Some(variant.variant),
                        ));
                    }
                    check_children(record, metadata.path, &variant.children, index)?;
                }
            }
            super::model::Shape::Unsupported => {
                return Err(reject(
                    super::model::FailureClass::UnsupportedMetadata,
                    super::model::Table::Metadata,
                    Some(index),
                    Some(metadata.path.0),
                ));
            }
        }
    }
    for (raw_index, path) in record.paths.iter().enumerate() {
        if metadata_for(record, path.key).is_none() {
            return Err(reject(
                super::model::FailureClass::MissingMetadata,
                super::model::Table::Path,
                Some(position(raw_index, super::model::Table::Path)?),
                Some(path.key.0),
            ));
        }
    }
    Ok(count)
}

/// Admits a finite path tree and one type-shape row per path.
// r[impl mir-drop.obligations]
// r[impl mir-drop.admission]
pub fn admit_record(record: &super::model::Record) -> Result<super::model::Facts, super::model::Rejection> {
    crate::mir_core::admit_body(&record.body)
        .map_err(|_| reject(super::model::FailureClass::InvalidBody, super::model::Table::Path, None, None))?;
    let path_count = check_path_rows(record)?;
    let metadata_count = check_metadata_rows(record)?;
    Ok(super::model::Facts {
        path_count,
        metadata_count,
    })
}
