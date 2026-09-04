//! Scheme-specific collection and bytevector shapes.

/// Raw bytevector envelope that can represent invalid elements.
// r[impl scheme-logic.collections-bytevectors]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BytevectorEnvelope {
    pub declared_length: u32,
    pub elements: Vec<u16>,
}

/// Finite hash-map and hash-set shape metadata.
// r[impl scheme-logic.collections-bytevectors]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HashShape {
    pub entries: Vec<(super::NodeId, super::NodeId)>,
    pub set_members: Vec<super::NodeId>,
}

/// Stable collection-shape failures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollectionFault {
    TooManyItems,
    DeclaredLengthMismatch,
    ByteOutsideDomain,
    IndexOutsideVector,
    ImproperList,
    CyclicList,
    BudgetTooLarge,
    BudgetExhausted,
    InvalidGraph(super::GraphFaultSite),
    DuplicateHashKey,
    DuplicateSetMember,
    DanglingHashNode,
}

/// Admits exact bytevector length and byte-domain elements.
// r[impl scheme-logic.collections-bytevectors]
pub fn admit_bytevector(envelope: &BytevectorEnvelope) -> Result<(), CollectionFault> {
    let count = u32::try_from(envelope.elements.len()).map_err(|_| CollectionFault::TooManyItems)?;
    if count > super::NODE_LIMIT {
        return Err(CollectionFault::TooManyItems);
    }
    if count != envelope.declared_length {
        return Err(CollectionFault::DeclaredLengthMismatch);
    }
    if envelope.elements.iter().any(|element| *element > u16::from(u8::MAX)) {
        return Err(CollectionFault::ByteOutsideDomain);
    }
    Ok(())
}

/// Performs one immutable vector update with exact length preservation.
// r[impl scheme-logic.collections-bytevectors]
pub fn update_vector(
    elements: &[super::NodeId],
    index: u32,
    value: super::NodeId,
) -> Result<Vec<super::NodeId>, CollectionFault> {
    let count = u32::try_from(elements.len()).map_err(|_| CollectionFault::TooManyItems)?;
    if count > super::NODE_LIMIT {
        return Err(CollectionFault::TooManyItems);
    }
    if index >= count {
        return Err(CollectionFault::IndexOutsideVector);
    }
    let platform_index = usize::try_from(index).map_err(|_| CollectionFault::IndexOutsideVector)?;
    let mut result = elements.to_vec();
    result[platform_index] = value;
    Ok(result)
}

/// Admits unique hash keys, unique set members, and declared graph nodes.
// r[impl scheme-logic.collections-bytevectors]
pub fn admit_hash_shape(graph: &super::ValueGraph, shape: &HashShape) -> Result<(), CollectionFault> {
    super::admit_values(graph).map_err(CollectionFault::InvalidGraph)?;
    let entry_count = u32::try_from(shape.entries.len()).map_err(|_| CollectionFault::TooManyItems)?;
    let member_count = u32::try_from(shape.set_members.len()).map_err(|_| CollectionFault::TooManyItems)?;
    if entry_count.checked_add(member_count).is_none_or(|total| total > super::NODE_LIMIT) {
        return Err(CollectionFault::TooManyItems);
    }
    for (index, (key, value)) in shape.entries.iter().enumerate() {
        if shape.entries[..index].iter().any(|(prior_key, _)| prior_key == key) {
            return Err(CollectionFault::DuplicateHashKey);
        }
        if super::node(graph, *key).is_none() || super::node(graph, *value).is_none() {
            return Err(CollectionFault::DanglingHashNode);
        }
    }
    for (index, member) in shape.set_members.iter().enumerate() {
        if shape.set_members[..index].contains(member) {
            return Err(CollectionFault::DuplicateSetMember);
        }
        if super::node(graph, *member).is_none() {
            return Err(CollectionFault::DanglingHashNode);
        }
    }
    Ok(())
}

/// Returns the proper-list length under explicit cycle and fuel checks.
// r[impl scheme-logic.collections-bytevectors]
pub fn proper_list_length(graph: &super::ValueGraph, root: super::NodeId, budget: u32) -> Result<u32, CollectionFault> {
    super::admit_values(graph).map_err(CollectionFault::InvalidGraph)?;
    if budget > super::NODE_LIMIT {
        return Err(CollectionFault::BudgetTooLarge);
    }
    if super::node(graph, root).is_none() {
        return Err(CollectionFault::ImproperList);
    }
    let mut visited = Vec::with_capacity(graph.nodes.len());
    let mut current = root;
    let mut length = 0u32;
    loop {
        let value = super::node(graph, current).ok_or(CollectionFault::ImproperList)?;
        match value {
            super::ValueNode::EmptyList => return Ok(length),
            super::ValueNode::Pair { cdr, .. } => {
                if visited.contains(&current) {
                    return Err(CollectionFault::CyclicList);
                }
                if length >= budget {
                    return Err(CollectionFault::BudgetExhausted);
                }
                visited.push(current);
                current = *cdr;
                length += 1;
            }
            super::ValueNode::Scalar(_)
            | super::ValueNode::Symbol(_)
            | super::ValueNode::Vector { .. }
            | super::ValueNode::ProcedurePlaceholder { .. } => {
                return Err(CollectionFault::ImproperList);
            }
        }
    }
}
