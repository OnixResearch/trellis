//! Deterministic iterative preorder and reverse-postorder traversal.

#[derive(Clone, Copy)]
struct Frame {
    block: u32,
    next_successor: usize,
}

fn reject(class: super::model::FailureClass, block: u32) -> super::model::Rejection {
    super::model::Rejection {
        class,
        entry_index: None,
        location: Some(super::model::Location::Block { block }),
    }
}

fn body_facts(
    body: &crate::mir_core::Body,
    entry: u32,
) -> Result<crate::mir_core::AdapterFacts, super::model::Rejection> {
    let facts = crate::mir_core::extract_adapter_facts(body)
        .map_err(|_| reject(super::model::FailureClass::InvalidBody, entry))?;
    if !super::proofs::index_admitted(entry, facts.block_count) {
        return Err(reject(super::model::FailureClass::InvalidLocation, entry));
    }
    Ok(facts)
}

fn index(block: u32) -> Result<usize, super::model::Rejection> {
    usize::try_from(block).map_err(|_| reject(super::model::FailureClass::InvalidLocation, block))
}

fn mark_seen(seen: &mut [bool], block: u32) -> Result<(), super::model::Rejection> {
    let slot = seen
        .get_mut(index(block)?)
        .ok_or_else(|| reject(super::model::FailureClass::InvalidLocation, block))?;
    *slot = true;
    Ok(())
}

fn block_is_seen(seen: &[bool], block: u32) -> Result<bool, super::model::Rejection> {
    seen.get(index(block)?)
        .copied()
        .ok_or_else(|| reject(super::model::FailureClass::InvalidLocation, block))
}

fn successors(edges: &[crate::mir_core::Edge], source: u32) -> Vec<u32> {
    edges.iter().filter(|edge| edge.source == source).map(|edge| edge.target).collect()
}

fn check_start(order: &[u32], entry: u32) -> Result<(), super::model::Rejection> {
    let first = order.first().copied().unwrap_or(entry);
    let count = u32::try_from(order.len()).map_err(|_| reject(super::model::FailureClass::TraceTooWide, entry))?;
    if super::proofs::order_starts_at_entry(first, entry, count) {
        Ok(())
    } else {
        Err(reject(super::model::FailureClass::OrderMismatch, entry))
    }
}

/// Returns deterministic reachable-block preorder from the supplied entry.
// r[impl mir-visitor.traversal-order]
pub fn preorder_blocks(body: &crate::mir_core::Body, entry: u32) -> Result<Vec<u32>, super::model::Rejection> {
    let facts = body_facts(body, entry)?;
    let mut seen = vec![false; body.blocks.len()];
    let mut stack = Vec::with_capacity(body.blocks.len());
    stack.push(entry);
    let mut order = Vec::with_capacity(body.blocks.len());

    while let Some(block) = stack.pop() {
        if block_is_seen(&seen, block)? {
            continue;
        }
        mark_seen(&mut seen, block)?;
        order.push(block);
        let targets = successors(&facts.edges, block);
        for target in targets.iter().rev() {
            if !block_is_seen(&seen, *target)? {
                stack.push(*target);
            }
        }
    }
    check_start(&order, entry)?;
    Ok(order)
}

/// Returns deterministic reverse postorder over blocks reachable from entry.
// r[impl mir-visitor.traversal-order]
pub fn reverse_postorder_blocks(body: &crate::mir_core::Body, entry: u32) -> Result<Vec<u32>, super::model::Rejection> {
    let facts = body_facts(body, entry)?;
    let mut seen = vec![false; body.blocks.len()];
    mark_seen(&mut seen, entry)?;
    let mut stack = Vec::with_capacity(body.blocks.len());
    stack.push(Frame {
        block: entry,
        next_successor: 0,
    });
    let mut postorder = Vec::with_capacity(body.blocks.len());

    while !stack.is_empty() {
        let frame = match stack.last().copied() {
            Some(frame) => frame,
            None => break,
        };
        let targets = successors(&facts.edges, frame.block);
        if frame.next_successor < targets.len() {
            let target = match targets.get(frame.next_successor).copied() {
                Some(target) => target,
                None => return Err(reject(super::model::FailureClass::OrderMismatch, frame.block)),
            };
            let last = match stack.last_mut() {
                Some(last) => last,
                None => return Err(reject(super::model::FailureClass::OrderMismatch, frame.block)),
            };
            last.next_successor = last
                .next_successor
                .checked_add(1)
                .ok_or_else(|| reject(super::model::FailureClass::TraceTooWide, frame.block))?;
            if !block_is_seen(&seen, target)? {
                mark_seen(&mut seen, target)?;
                stack.push(Frame {
                    block: target,
                    next_successor: 0,
                });
            }
        } else {
            stack.pop();
            postorder.push(frame.block);
        }
    }

    postorder.reverse();
    check_start(&postorder, entry)?;
    Ok(postorder)
}
