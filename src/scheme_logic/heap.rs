//! Finite value graphs and cycle-safe traversal admission.

const NODE_LIMIT: u32 = 4_096;
const PAIR_EDGE_COUNT: u32 = 2;

/// Finite value-node identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u32);

/// Supported finite value-node classes.
// r[impl scheme-logic.value-graph]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueNode {
    EmptyList,
    Scalar(i64),
    Symbol(super::registry::InternKey),
    Pair { car: NodeId, cdr: NodeId },
    Vector { elements: Vec<NodeId> },
    ProcedurePlaceholder { arity: u32 },
}

/// Complete finite value graph.
// r[impl scheme-logic.value-graph]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueGraph {
    pub nodes: Vec<ValueNode>,
}

/// Traversal use at the proof boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraversalPurpose {
    Display,
    Equality,
}

/// Successful bounded traversal metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Traversal {
    pub purpose: TraversalPurpose,
    pub visited: Vec<NodeId>,
    pub consumed_budget: u32,
}

/// Stable value-graph rejection classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GraphFault {
    TooManyNodes,
    TooManyEdges,
    DanglingEdge,
    RootOutsideGraph,
    BudgetTooLarge,
    BudgetExhausted,
}

/// Deterministic value-graph rejection metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GraphFaultSite {
    pub class: GraphFault,
    pub index: Option<u32>,
}

fn position(index: usize) -> Option<u32> {
    u32::try_from(index).ok()
}

fn reject<T>(class: GraphFault, index: Option<u32>) -> Result<T, GraphFaultSite> {
    Err(GraphFaultSite { class, index })
}

fn node(graph: &ValueGraph, id: NodeId) -> Option<&ValueNode> {
    usize::try_from(id.0).ok().and_then(|index| graph.nodes.get(index))
}

fn queue_has_capacity<T>(pending: &[T]) -> bool {
    u32::try_from(pending.len()).is_ok_and(|count| count < NODE_LIMIT)
}

fn append_edge_targets(
    value: &ValueNode,
    mut pending: Vec<NodeId>,
    visited: &[NodeId],
) -> Result<Vec<NodeId>, GraphFaultSite> {
    match value {
        ValueNode::Pair { car, cdr } => {
            for target in [*car, *cdr] {
                if !visited.contains(&target) && !pending.contains(&target) {
                    if !queue_has_capacity(&pending) {
                        return reject(GraphFault::BudgetExhausted, None);
                    }
                    pending.push(target);
                }
            }
        }
        ValueNode::Vector { elements } => {
            for target in elements {
                if !visited.contains(target) && !pending.contains(target) {
                    if !queue_has_capacity(&pending) {
                        return reject(GraphFault::BudgetExhausted, None);
                    }
                    pending.push(*target);
                }
            }
        }
        ValueNode::EmptyList | ValueNode::Scalar(_) | ValueNode::Symbol(_) | ValueNode::ProcedurePlaceholder { .. } => {
        }
    }
    Ok(pending)
}

/// Admits only bounded nodes and in-range edges.
// r[impl scheme-logic.value-graph]
pub fn admit_values(graph: &ValueGraph) -> Result<(), GraphFaultSite> {
    let node_count = u32::try_from(graph.nodes.len()).map_err(|_| GraphFaultSite {
        class: GraphFault::TooManyNodes,
        index: None,
    })?;
    if node_count > NODE_LIMIT {
        return reject(GraphFault::TooManyNodes, None);
    }
    let mut edge_count = 0u32;
    for (index, value) in graph.nodes.iter().enumerate() {
        let at = position(index);
        let local_edges = match value {
            ValueNode::Pair { .. } => PAIR_EDGE_COUNT,
            ValueNode::Vector { elements } => u32::try_from(elements.len()).map_err(|_| GraphFaultSite {
                class: GraphFault::TooManyEdges,
                index: at,
            })?,
            ValueNode::EmptyList
            | ValueNode::Scalar(_)
            | ValueNode::Symbol(_)
            | ValueNode::ProcedurePlaceholder { .. } => 0u32,
        };
        edge_count = edge_count.checked_add(local_edges).ok_or(GraphFaultSite {
            class: GraphFault::TooManyEdges,
            index: at,
        })?;
        if edge_count > NODE_LIMIT {
            return reject(GraphFault::TooManyEdges, at);
        }
        let has_dangling = match value {
            ValueNode::Pair { car, cdr } => car.0 >= node_count || cdr.0 >= node_count,
            ValueNode::Vector { elements } => elements.iter().any(|target| target.0 >= node_count),
            ValueNode::EmptyList
            | ValueNode::Scalar(_)
            | ValueNode::Symbol(_)
            | ValueNode::ProcedurePlaceholder { .. } => false,
        };
        if has_dangling {
            return reject(GraphFault::DanglingEdge, at);
        }
    }
    Ok(())
}

/// Traverses each reachable node at most once under explicit fuel.
// r[impl scheme-logic.value-graph]
pub fn traverse_values(
    graph: &ValueGraph,
    root: NodeId,
    purpose: TraversalPurpose,
    budget: u32,
) -> Result<Traversal, GraphFaultSite> {
    admit_values(graph)?;
    if node(graph, root).is_none() {
        return reject(GraphFault::RootOutsideGraph, None);
    }
    if budget > NODE_LIMIT {
        return reject(GraphFault::BudgetTooLarge, None);
    }
    let mut pending = Vec::with_capacity(graph.nodes.len());
    let mut visited = Vec::with_capacity(graph.nodes.len());
    pending.push(root);
    let mut cursor = 0usize;
    let mut consumed_budget = 0u32;
    while cursor < pending.len() {
        if consumed_budget >= budget {
            return reject(GraphFault::BudgetExhausted, position(cursor));
        }
        let current = pending[cursor];
        let current_value = node(graph, current).ok_or(GraphFaultSite {
            class: GraphFault::DanglingEdge,
            index: position(cursor),
        })?;
        visited.push(current);
        pending = append_edge_targets(current_value, pending, &visited)?;
        cursor += 1;
        consumed_budget += 1;
    }
    Ok(Traversal {
        purpose,
        visited,
        consumed_budget,
    })
}

mod equivalence;
mod shapes;

pub use equivalence::compare_values;
pub use shapes::BytevectorEnvelope;
pub use shapes::CollectionFault;
pub use shapes::HashShape;
pub use shapes::admit_bytevector;
pub use shapes::admit_hash_shape;
pub use shapes::proper_list_length;
pub use shapes::update_vector;
