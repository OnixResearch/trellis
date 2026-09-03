//! Initializedness and obligation admission plus deterministic updates.

struct PathSet {
    items: Vec<super::model::PathKey>,
}

impl PathSet {
    fn from(items: &[super::model::PathKey]) -> Self {
        let mut copy = Vec::with_capacity(items.len());
        copy.extend_from_slice(items);
        Self { items: copy }
    }

    fn insert(&mut self, key: super::model::PathKey) {
        if !self.items.contains(&key) {
            self.items.push(key);
        }
    }

    fn remove(&mut self, key: super::model::PathKey) {
        self.items.retain(|item| *item != key);
    }
}

fn reject(class: super::model::FailureClass, index: Option<u32>, subject: Option<u32>) -> super::model::Rejection {
    super::model::Rejection {
        class,
        table: super::model::Table::State,
        index,
        subject,
    }
}

fn position(index: usize) -> Result<u32, super::model::Rejection> {
    u32::try_from(index).map_err(|_| reject(super::model::FailureClass::TableTooWide, None, None))
}

fn check_set(record: &super::model::Record, items: &[super::model::PathKey]) -> Result<(), super::model::Rejection> {
    let count = position(items.len())?;
    if count > super::model::MAX_PATHS {
        return Err(reject(super::model::FailureClass::TableTooWide, None, Some(count)));
    }
    for (raw_index, key) in items.iter().copied().enumerate() {
        let index = position(raw_index)?;
        if super::paths::path_for(record, key).is_none() {
            return Err(reject(super::model::FailureClass::InvalidPath, Some(index), Some(key.0)));
        }
        if items[..raw_index].contains(&key) {
            return Err(reject(super::model::FailureClass::DuplicateFact, Some(index), Some(key.0)));
        }
    }
    Ok(())
}

/// Admits finite maybe-fact and obligation sets.
// r[impl mir-drop.initializedness]
// r[impl mir-drop.admission]
pub fn admit_state(record: &super::model::Record, state: &super::model::State) -> Result<(), super::model::Rejection> {
    super::paths::admit_record(record)?;
    check_set(record, &state.maybe_initialized)?;
    check_set(record, &state.maybe_uninitialized)?;
    check_set(record, &state.obligations)?;
    for (raw_index, obligation) in state.obligations.iter().copied().enumerate() {
        let index = position(raw_index)?;
        let metadata = super::paths::metadata_for(record, obligation)
            .ok_or_else(|| reject(super::model::FailureClass::MissingMetadata, Some(index), Some(obligation.0)))?;
        if !super::paths::shape_has_glue(&metadata.shape) {
            return Err(reject(super::model::FailureClass::InvalidObligation, Some(index), Some(obligation.0)));
        }
    }
    Ok(())
}

fn action_target(action: super::model::Action) -> super::model::PathKey {
    match action {
        super::model::Action::Initialize(path)
        | super::model::Action::Move(path)
        | super::model::Action::Overwrite(path) => path,
    }
}

/// Applies one initialization, move, or overwrite to a finite path subtree.
// r[impl mir-drop.obligations]
// r[impl mir-drop.initializedness]
pub fn apply_action(
    record: &super::model::Record,
    state: &super::model::State,
    action: super::model::Action,
) -> Result<super::model::State, super::model::Rejection> {
    admit_state(record, state)?;
    let target = action_target(action);
    if super::paths::path_for(record, target).is_none() {
        return Err(reject(super::model::FailureClass::InvalidPath, None, Some(target.0)));
    }
    let mut initialized = PathSet::from(&state.maybe_initialized);
    let mut uninitialized = PathSet::from(&state.maybe_uninitialized);
    let mut obligations = PathSet::from(&state.obligations);
    for path in &record.paths {
        if !super::paths::descendant_or_same(record, path.key, target) {
            continue;
        }
        match action {
            super::model::Action::Move(_) => {
                initialized.remove(path.key);
                uninitialized.insert(path.key);
                obligations.remove(path.key);
            }
            super::model::Action::Initialize(_) | super::model::Action::Overwrite(_) => {
                initialized.insert(path.key);
                uninitialized.remove(path.key);
                let metadata = super::paths::metadata_for(record, path.key)
                    .ok_or_else(|| reject(super::model::FailureClass::MissingMetadata, None, Some(path.key.0)))?;
                if super::paths::shape_has_glue(&metadata.shape) {
                    obligations.insert(path.key);
                }
            }
        }
    }
    Ok(super::model::State {
        maybe_initialized: initialized.items,
        maybe_uninitialized: uninitialized.items,
        obligations: obligations.items,
    })
}
