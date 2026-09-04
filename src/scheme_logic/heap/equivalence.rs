//! Cycle-safe finite graph-shape comparison.

fn queue_has_capacity(pending: &[(super::NodeId, super::NodeId)]) -> bool {
    u32::try_from(pending.len()).is_ok_and(|count| count < super::NODE_LIMIT)
}

fn enqueue_pair(
    mut pending: Vec<(super::NodeId, super::NodeId)>,
    visited: &[(super::NodeId, super::NodeId)],
    pair: (super::NodeId, super::NodeId),
) -> Result<Vec<(super::NodeId, super::NodeId)>, super::GraphFaultSite> {
    if !visited.contains(&pair) && !pending.contains(&pair) {
        if !queue_has_capacity(&pending) {
            return super::reject(super::GraphFault::BudgetExhausted, None);
        }
        pending.push(pair);
    }
    Ok(pending)
}

fn compare_node_pair(
    graph: &super::ValueGraph,
    pair: (super::NodeId, super::NodeId),
    mut pending: Vec<(super::NodeId, super::NodeId)>,
    visited: &[(super::NodeId, super::NodeId)],
) -> Result<(Vec<(super::NodeId, super::NodeId)>, bool), super::GraphFaultSite> {
    let left = super::node(graph, pair.0).ok_or(super::GraphFaultSite {
        class: super::GraphFault::DanglingEdge,
        index: None,
    })?;
    let right = super::node(graph, pair.1).ok_or(super::GraphFaultSite {
        class: super::GraphFault::DanglingEdge,
        index: None,
    })?;
    match (left, right) {
        (super::ValueNode::EmptyList, super::ValueNode::EmptyList) => {}
        (super::ValueNode::Scalar(left), super::ValueNode::Scalar(right)) if left == right => {}
        (super::ValueNode::Symbol(left), super::ValueNode::Symbol(right)) if left == right => {}
        (
            super::ValueNode::ProcedurePlaceholder { arity: left },
            super::ValueNode::ProcedurePlaceholder { arity: right },
        ) if left == right => {}
        (
            super::ValueNode::Pair {
                car: left_car,
                cdr: left_cdr,
            },
            super::ValueNode::Pair {
                car: right_car,
                cdr: right_cdr,
            },
        ) => {
            pending = enqueue_pair(pending, visited, (*left_car, *right_car))?;
            pending = enqueue_pair(pending, visited, (*left_cdr, *right_cdr))?;
        }
        (
            super::ValueNode::Vector {
                elements: left_elements,
            },
            super::ValueNode::Vector {
                elements: right_elements,
            },
        ) if left_elements.len() == right_elements.len() => {
            for child_pair in left_elements.iter().copied().zip(right_elements.iter().copied()) {
                pending = enqueue_pair(pending, visited, child_pair)?;
            }
        }
        _ => return Ok((pending, false)),
    }
    Ok((pending, true))
}

/// Compares finite graph shapes without recursive descent.
// r[impl scheme-logic.value-graph]
pub fn compare_values(
    graph: &super::ValueGraph,
    left: super::NodeId,
    right: super::NodeId,
    budget: u32,
) -> Result<bool, super::GraphFaultSite> {
    super::admit_values(graph)?;
    if super::node(graph, left).is_none() || super::node(graph, right).is_none() {
        return super::reject(super::GraphFault::RootOutsideGraph, None);
    }
    if budget > super::NODE_LIMIT {
        return super::reject(super::GraphFault::BudgetTooLarge, None);
    }
    let mut pending = Vec::with_capacity(graph.nodes.len());
    let mut visited = Vec::with_capacity(graph.nodes.len());
    pending.push((left, right));
    let mut cursor = 0usize;
    while cursor < pending.len() {
        let consumed = u32::try_from(cursor).map_err(|_| super::GraphFaultSite {
            class: super::GraphFault::BudgetExhausted,
            index: None,
        })?;
        if consumed >= budget {
            return super::reject(super::GraphFault::BudgetExhausted, Some(consumed));
        }
        let pair = pending[cursor];
        visited.push(pair);
        let (next_pending, is_equal) = compare_node_pair(graph, pair, pending, &visited)?;
        if !is_equal {
            return Ok(false);
        }
        pending = next_pending;
        cursor += 1;
    }
    Ok(true)
}
