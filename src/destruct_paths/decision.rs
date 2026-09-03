//! Destruction classification, guarded expansion, and replacement ordering.

fn reject(class: super::model::FailureClass, index: Option<u32>, subject: Option<u32>) -> super::model::Rejection {
    super::model::Rejection {
        class,
        table: super::model::Table::Expansion,
        index,
        subject,
    }
}

struct PathStack {
    items: Vec<super::model::PathKey>,
}

impl PathStack {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, path: super::model::PathKey) {
        self.items.push(path);
    }

    fn pop(&mut self) -> Option<super::model::PathKey> {
        self.items.pop()
    }

    fn push_children(&mut self, children: &[super::model::PathKey]) {
        for child in children.iter().rev() {
            self.items.push(*child);
        }
    }
}

fn descendants_are_mixed(
    record: &super::model::Record,
    state: &super::model::State,
    target: super::model::PathKey,
) -> bool {
    let mut has_initialized = false;
    let mut has_uninitialized = false;
    for path in &record.paths {
        if path.key == target || !super::paths::descendant_or_same(record, path.key, target) {
            continue;
        }
        has_initialized |= state.maybe_initialized.contains(&path.key);
        has_uninitialized |= state.maybe_uninitialized.contains(&path.key);
    }
    has_initialized && has_uninitialized
}

/// Classifies one admitted path from finite maybe-initialized facts.
// r[impl mir-drop.classification]
pub fn classify(
    record: &super::model::Record,
    state: &super::model::State,
    target: super::model::PathKey,
    policy: super::model::ConditionalPolicy,
) -> Result<super::model::Classification, super::model::Rejection> {
    super::facts::admit_state(record, state)?;
    let metadata = super::paths::metadata_for(record, target)
        .ok_or_else(|| reject(super::model::FailureClass::MissingMetadata, None, Some(target.0)))?;
    if !super::paths::shape_has_glue(&metadata.shape) {
        return Ok(super::model::Classification::Dead);
    }
    let is_maybe_initialized = state.maybe_initialized.contains(&target);
    let is_maybe_uninitialized = state.maybe_uninitialized.contains(&target);
    match (is_maybe_initialized, is_maybe_uninitialized) {
        (true, false) => Ok(super::model::Classification::Static),
        (false, true) => Ok(super::model::Classification::Dead),
        (true, true) => {
            if policy == super::model::ConditionalPolicy::OpenDescendants
                && matches!(metadata.shape, super::model::Shape::Fields(_) | super::model::Shape::Enum(_))
                && descendants_are_mixed(record, state, target)
            {
                Ok(super::model::Classification::Open)
            } else {
                Ok(super::model::Classification::Conditional)
            }
        }
        (false, false) => Err(reject(super::model::FailureClass::InconsistentFacts, None, Some(target.0))),
    }
}

fn active_variant(
    active: &[super::model::ActiveVariant],
    path: super::model::PathKey,
) -> Result<u32, super::model::Rejection> {
    let mut selected = None;
    for (raw_index, candidate) in active.iter().copied().enumerate() {
        if candidate.path == path {
            if selected.is_some() {
                let index = u32::try_from(raw_index)
                    .map_err(|_| reject(super::model::FailureClass::TableTooWide, None, Some(path.0)))?;
                return Err(reject(super::model::FailureClass::InvalidActiveVariant, Some(index), Some(path.0)));
            }
            selected = Some(candidate.variant);
        }
    }
    selected.ok_or_else(|| reject(super::model::FailureClass::MissingActiveVariant, None, Some(path.0)))
}

/// Expands one finite target into static or guarded call metadata.
// r[impl mir-drop.drop-flags]
// r[impl mir-drop.open-drop]
pub fn expand(
    record: &super::model::Record,
    state: &super::model::State,
    request: &super::model::ExpansionRequest,
) -> Result<Vec<super::model::Step>, super::model::Rejection> {
    super::facts::admit_state(record, state)?;
    if super::paths::path_for(record, request.target).is_none() {
        return Err(reject(super::model::FailureClass::InvalidPath, None, Some(request.target.0)));
    }
    if request.budget == 0 || request.budget > super::model::MAX_EXPANSION_STEPS {
        return Err(reject(super::model::FailureClass::BudgetExhausted, None, Some(request.budget)));
    }
    let mut stack = PathStack::with_capacity(record.paths.len());
    let mut steps = Vec::with_capacity(record.paths.len());
    stack.push(request.target);
    let mut consumed = 0_u32;
    while let Some(path) = stack.pop() {
        consumed = consumed
            .checked_add(1)
            .ok_or_else(|| reject(super::model::FailureClass::BudgetExhausted, None, Some(request.budget)))?;
        if consumed > request.budget {
            return Err(reject(super::model::FailureClass::BudgetExhausted, None, Some(consumed)));
        }
        let metadata = super::paths::metadata_for(record, path)
            .ok_or_else(|| reject(super::model::FailureClass::MissingMetadata, None, Some(path.0)))?;
        match classify(record, state, path, request.policy)? {
            super::model::Classification::Static => {
                steps.push(super::model::Step::Call { path });
            }
            super::model::Classification::Dead => {}
            super::model::Classification::Conditional => {
                steps.push(super::model::Step::Guarded {
                    path,
                    flag: super::model::FlagKey(path.0),
                });
            }
            super::model::Classification::Open => match &metadata.shape {
                super::model::Shape::Fields(children) => stack.push_children(children),
                super::model::Shape::Enum(variants) => {
                    let selected = active_variant(&request.active_variants, path)?;
                    let variant = variants.iter().find(|variant| variant.variant == selected).ok_or_else(|| {
                        reject(super::model::FailureClass::InvalidActiveVariant, None, Some(selected))
                    })?;
                    stack.push_children(&variant.children);
                }
                super::model::Shape::NoGlue | super::model::Shape::Direct | super::model::Shape::Unsupported => {
                    return Err(reject(super::model::FailureClass::UnsupportedMetadata, None, Some(path.0)));
                }
            },
        }
    }
    Ok(steps)
}

/// Emits old-target destruction metadata before one replacement write.
// r[impl mir-drop.drop-and-replace]
pub fn split_replace(
    record: &super::model::Record,
    state: &super::model::State,
    request: &super::model::ReplacementRequest,
) -> Result<Vec<super::model::Step>, super::model::Rejection> {
    let drops = expand(record, state, &request.expansion)?;
    let capacity = drops
        .len()
        .checked_add(1)
        .ok_or_else(|| reject(super::model::FailureClass::TableTooWide, None, Some(request.expansion.target.0)))?;
    let mut steps = Vec::with_capacity(capacity);
    steps.extend(drops);
    steps.push(super::model::Step::Write {
        path: request.expansion.target,
        value: request.replacement,
    });
    Ok(steps)
}
