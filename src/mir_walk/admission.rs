//! Trace, coverage, reachability, order, and mutation admission.

fn reject(
    class: super::model::FailureClass,
    entry_index: Option<u32>,
    location: Option<super::model::Location>,
) -> super::model::Rejection {
    super::model::Rejection {
        class,
        entry_index,
        location,
    }
}

fn item_index(index: usize) -> Result<u32, super::model::Rejection> {
    u32::try_from(index).map_err(|_| reject(super::model::FailureClass::TraceTooWide, None, None))
}

fn order_tag(order: super::model::Order) -> u8 {
    match order {
        super::model::Order::Declared => super::proofs::DECLARED_ORDER_TAG,
        super::model::Order::Preorder | super::model::Order::ReversePostorder => super::proofs::PRESENT_TAG,
    }
}

fn reachability_tag(policy: super::model::ReachabilityPolicy) -> u8 {
    match policy {
        super::model::ReachabilityPolicy::ReachableOnly => super::proofs::REACHABLE_ONLY_TAG,
        super::model::ReachabilityPolicy::AllBlocks => super::proofs::MISSING_TAG,
    }
}

fn owning_block(location: super::model::Location) -> Option<u32> {
    match location {
        super::model::Location::Body | super::model::Location::Local { .. } => None,
        super::model::Location::Block { block }
        | super::model::Location::Statement { block, .. }
        | super::model::Location::Terminator { block }
        | super::model::Location::Rvalue { block, .. }
        | super::model::Location::Successor { block, .. } => Some(block),
        super::model::Location::Place { owner, .. } | super::model::Location::Operand { owner, .. } => match owner {
            super::model::Owner::Statement { block, .. } | super::model::Owner::Terminator { block } => Some(block),
        },
    }
}

fn location_is_reachable(location: super::model::Location, reachable: &[u32]) -> bool {
    owning_block(location).is_none_or(|block| reachable.contains(&block))
}

fn check_location(
    body: &crate::mir_core::Body,
    location: super::model::Location,
    position: u32,
    reachable: Option<&[u32]>,
) -> Result<(), super::model::Rejection> {
    super::bounds::admit_location(body, &location).map_err(|class| reject(class, Some(position), Some(location)))?;
    if let Some(blocks) = reachable {
        if !location_is_reachable(location, blocks) {
            return Err(reject(super::model::FailureClass::ReachabilityMismatch, Some(position), Some(location)));
        }
    }
    Ok(())
}

fn mutation_shape(
    body: &crate::mir_core::Body,
    after: Option<&crate::mir_core::Body>,
    class: super::model::MutationClass,
) -> Result<super::model::Shape, super::model::Rejection> {
    match (class, after) {
        (super::model::MutationClass::Immutable, None) => super::rewrite::body_shape(body),
        (super::model::MutationClass::ValuesOnly, Some(changed)) => {
            super::rewrite::admit_mutation(body, changed, class)
        }
        (super::model::MutationClass::Immutable, Some(_))
        | (super::model::MutationClass::ValuesOnly, None)
        | (super::model::MutationClass::UnsupportedStructural, _) => {
            Err(reject(super::model::FailureClass::MutationMismatch, None, None))
        }
    }
}

fn check_requested(
    body: &crate::mir_core::Body,
    requested: &[super::model::Location],
    reachable: Option<&[u32]>,
) -> Result<(), super::model::Rejection> {
    for (raw_index, location) in requested.iter().copied().enumerate() {
        let position = item_index(raw_index)?;
        check_location(body, location, position, reachable)?;
        if requested[..raw_index].contains(&location) {
            return Err(reject(super::model::FailureClass::DuplicateRequest, Some(position), Some(location)));
        }
    }
    Ok(())
}

fn check_entries(
    body: &crate::mir_core::Body,
    trace: &super::model::Trace,
    requested: &[super::model::Location],
    reachable: Option<&[u32]>,
) -> Result<(), super::model::Rejection> {
    for (raw_index, entry) in trace.entries.iter().enumerate() {
        let position = item_index(raw_index)?;
        check_location(body, entry.location, position, reachable)?;
        if trace.coverage == super::model::CoverageMode::ExactlyOnce {
            if trace.entries[..raw_index].iter().any(|prior| prior.location == entry.location) {
                return Err(reject(super::model::FailureClass::DuplicateVisit, Some(position), Some(entry.location)));
            }
            if !requested.contains(&entry.location) {
                return Err(reject(super::model::FailureClass::OutsideRequest, Some(position), Some(entry.location)));
            }
        }
    }

    if trace.coverage == super::model::CoverageMode::ExactlyOnce {
        for location in requested {
            if !trace.entries.iter().any(|entry| entry.location == *location) {
                return Err(reject(super::model::FailureClass::MissingVisit, None, Some(*location)));
            }
        }
    }
    Ok(())
}

fn block_order(trace: &super::model::Trace) -> Vec<u32> {
    trace
        .entries
        .iter()
        .filter_map(|entry| match entry.location {
            super::model::Location::Block { block } => Some(block),
            super::model::Location::Body
            | super::model::Location::Local { .. }
            | super::model::Location::Statement { .. }
            | super::model::Location::Terminator { .. }
            | super::model::Location::Place { .. }
            | super::model::Location::Operand { .. }
            | super::model::Location::Rvalue { .. }
            | super::model::Location::Successor { .. } => None,
        })
        .collect()
}

fn check_order(
    body: &crate::mir_core::Body,
    trace: &super::model::Trace,
    actual: &[u32],
) -> Result<(), super::model::Rejection> {
    let expected = match trace.order {
        super::model::Order::Declared => return Ok(()),
        super::model::Order::Preorder => super::traversal::preorder_blocks(body, trace.entry)?,
        super::model::Order::ReversePostorder => super::traversal::reverse_postorder_blocks(body, trace.entry)?,
    };
    if actual == expected {
        Ok(())
    } else {
        Err(reject(super::model::FailureClass::OrderMismatch, None, None))
    }
}

/// Admits a finite visitor trace and exports facts only after all checks pass.
// r[impl mir-visitor.trace]
// r[impl mir-visitor.coverage]
// r[impl mir-visitor.traversal-order]
// r[impl mir-visitor.admission]
pub fn admit_trace(
    body: &crate::mir_core::Body,
    after: Option<&crate::mir_core::Body>,
    requested: &[super::model::Location],
    trace: &super::model::Trace,
) -> Result<super::model::TraceFacts, super::model::Rejection> {
    super::bounds::admit_location(body, &super::model::Location::Block { block: trace.entry })
        .map_err(|class| reject(class, None, Some(super::model::Location::Block { block: trace.entry })))?;
    if !super::proofs::mode_admitted(order_tag(trace.order), reachability_tag(trace.reachability)) {
        return Err(reject(super::model::FailureClass::InvalidMode, None, None));
    }
    let shape = mutation_shape(body, after, trace.mutation)?;
    let reachable_blocks = if trace.reachability == super::model::ReachabilityPolicy::ReachableOnly {
        Some(super::traversal::preorder_blocks(body, trace.entry)?)
    } else {
        None
    };
    let reachable = reachable_blocks.as_deref();
    check_requested(body, requested, reachable)?;
    check_entries(body, trace, requested, reachable)?;
    let actual_order = block_order(trace);
    check_order(body, trace, &actual_order)?;
    if !super::proofs::coverage_admitted(0, 0, 0, 0) {
        return Err(reject(super::model::FailureClass::MissingVisit, None, None));
    }
    Ok(super::model::TraceFacts {
        visited_count: item_index(trace.entries.len())?,
        requested_count: item_index(requested.len())?,
        block_order: actual_order,
        shape,
    })
}
