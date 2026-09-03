//! Pure fixed-result cursor and exactly-once location inspection.

fn reject(class: super::model::FailureClass, index: Option<u32>) -> super::model::Rejection {
    super::model::Rejection {
        class,
        table: super::model::Table::Result,
        index,
        subject: None,
    }
}

fn position(index: usize) -> Result<u32, super::model::Rejection> {
    u32::try_from(index).map_err(|_| reject(super::model::FailureClass::InvalidResultPoint, None))
}

fn block_state(
    states: &[super::model::StateKey],
    block: u32,
) -> Result<super::model::StateKey, super::model::Rejection> {
    let index =
        usize::try_from(block).map_err(|_| reject(super::model::FailureClass::InvalidResultPoint, Some(block)))?;
    states
        .get(index)
        .copied()
        .ok_or_else(|| reject(super::model::FailureClass::InvalidResultPoint, Some(block)))
}

/// Reads one declared point from fixed results.
// r[impl mir-dataflow.results]
pub fn inspect_state(
    results: &super::model::Results,
    point: super::model::Point,
) -> Result<super::model::StateKey, super::model::Rejection> {
    match point {
        super::model::Point::BlockEntry { block } => block_state(&results.block_entries, block),
        super::model::Point::BlockExit { block } => block_state(&results.block_exits, block),
        super::model::Point::Before { location } => results
            .locations
            .iter()
            .find(|state| state.location == location)
            .map(|state| state.after_before)
            .ok_or_else(|| reject(super::model::FailureClass::InvalidResultPoint, None)),
        super::model::Point::Primary { location } => results
            .locations
            .iter()
            .find(|state| state.location == location)
            .map(|state| state.after_primary)
            .ok_or_else(|| reject(super::model::FailureClass::InvalidResultPoint, None)),
        super::model::Point::Edge { source, target } => results
            .edges
            .iter()
            .find(|edge| edge.source == source && edge.target == target)
            .map(|edge| edge.state)
            .ok_or_else(|| reject(super::model::FailureClass::InvalidResultPoint, None)),
    }
}

/// Returns each requested fixed location once in request order.
// r[impl mir-dataflow.results]
pub fn visit_locations(
    results: &super::model::Results,
    requested: &[crate::mir_visitor_traversal::Location],
) -> Result<Vec<super::model::LocationState>, super::model::Rejection> {
    let mut observed = Vec::with_capacity(requested.len());
    for (raw_index, location) in requested.iter().copied().enumerate() {
        let index = position(raw_index)?;
        if requested[..raw_index].contains(&location) {
            return Err(reject(super::model::FailureClass::DuplicateRequest, Some(index)));
        }
        let state = results
            .locations
            .iter()
            .find(|state| state.location == location)
            .cloned()
            .ok_or_else(|| reject(super::model::FailureClass::InvalidResultPoint, Some(index)))?;
        observed.push(state);
    }
    Ok(observed)
}
