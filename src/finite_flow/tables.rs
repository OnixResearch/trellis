//! Transfer-table, location, edge, monotonicity, and budget admission.

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
    u32::try_from(index).map_err(|_| reject(super::model::FailureClass::InvalidTransfer, table, None, None))
}

fn width(len: usize, table: super::model::Table) -> Result<u32, super::model::Rejection> {
    let count = position(len, table)?;
    if count > super::model::MAX_ROWS {
        Err(reject(super::model::FailureClass::InvalidTransfer, table, None, Some(count)))
    } else {
        Ok(count)
    }
}

/// Applies one admitted total transfer table.
// r[impl mir-dataflow.effects]
pub fn apply_transfer(
    transfer: &super::model::Transfer,
    input: super::model::StateKey,
) -> Result<super::model::StateKey, super::model::Rejection> {
    let index = usize::try_from(input.0).map_err(|_| {
        reject(super::model::FailureClass::InvalidTransfer, super::model::Table::Effect, None, Some(input.0))
    })?;
    transfer.outputs.get(index).copied().ok_or_else(|| {
        reject(super::model::FailureClass::InvalidTransfer, super::model::Table::Effect, None, Some(input.0))
    })
}

fn check_transfer(
    domain: &super::model::Domain,
    transfer: &super::model::Transfer,
    table: super::model::Table,
    index: u32,
) -> Result<(), super::model::Rejection> {
    let count = position(transfer.outputs.len(), table)?;
    if count != domain.state_count || transfer.outputs.iter().any(|output| output.0 >= domain.state_count) {
        return Err(reject(super::model::FailureClass::InvalidTransfer, table, Some(index), Some(count)));
    }
    for left_raw in 0..domain.state_count {
        let left = super::model::StateKey(left_raw);
        for right_raw in 0..domain.state_count {
            let right = super::model::StateKey(right_raw);
            if super::lattice::state_is_ordered(domain, left, right)? {
                let left_output = apply_transfer(transfer, left)?;
                let right_output = apply_transfer(transfer, right)?;
                if !super::lattice::state_is_ordered(domain, left_output, right_output)? {
                    return Err(reject(
                        super::model::FailureClass::NonMonotoneTransfer,
                        table,
                        Some(index),
                        Some(left_raw),
                    ));
                }
            }
        }
    }
    Ok(())
}

pub(super) fn location_block(location: crate::mir_visitor_traversal::Location) -> Option<u32> {
    match location {
        crate::mir_visitor_traversal::Location::Statement { block, .. }
        | crate::mir_visitor_traversal::Location::Terminator { block } => Some(block),
        crate::mir_visitor_traversal::Location::Body
        | crate::mir_visitor_traversal::Location::Local { .. }
        | crate::mir_visitor_traversal::Location::Block { .. }
        | crate::mir_visitor_traversal::Location::Place { .. }
        | crate::mir_visitor_traversal::Location::Operand { .. }
        | crate::mir_visitor_traversal::Location::Rvalue { .. }
        | crate::mir_visitor_traversal::Location::Successor { .. } => None,
    }
}

fn check_location_effects(
    body: &crate::mir_core::Body,
    analysis: &super::model::Analysis,
) -> Result<u32, super::model::Rejection> {
    let count = width(analysis.effects.len(), super::model::Table::Effect)?;
    for (raw_index, effect) in analysis.effects.iter().enumerate() {
        let index = position(raw_index, super::model::Table::Effect)?;
        crate::mir_visitor_traversal::admit_location(body, &effect.location).map_err(|_| {
            reject(super::model::FailureClass::InvalidLocation, super::model::Table::Effect, Some(index), None)
        })?;
        if location_block(effect.location).is_none() {
            return Err(reject(
                super::model::FailureClass::InvalidLocation,
                super::model::Table::Effect,
                Some(index),
                None,
            ));
        }
        if analysis.effects[..raw_index].iter().any(|prior| prior.location == effect.location) {
            return Err(reject(
                super::model::FailureClass::DuplicateEffect,
                super::model::Table::Effect,
                Some(index),
                None,
            ));
        }
        check_transfer(&analysis.domain, &effect.before, super::model::Table::Effect, index)?;
        check_transfer(&analysis.domain, &effect.primary, super::model::Table::Effect, index)?;
    }
    Ok(count)
}

fn check_edge_effects(
    facts: &crate::mir_core::AdapterFacts,
    analysis: &super::model::Analysis,
) -> Result<u32, super::model::Rejection> {
    let count = width(analysis.edge_effects.len(), super::model::Table::Edge)?;
    for (raw_index, effect) in analysis.edge_effects.iter().enumerate() {
        let index = position(raw_index, super::model::Table::Edge)?;
        if !facts.edges.iter().any(|edge| edge.source == effect.source && edge.target == effect.target) {
            return Err(reject(
                super::model::FailureClass::InvalidEdgeEffect,
                super::model::Table::Edge,
                Some(index),
                Some(effect.target),
            ));
        }
        if analysis.edge_effects[..raw_index]
            .iter()
            .any(|prior| prior.source == effect.source && prior.target == effect.target)
        {
            return Err(reject(
                super::model::FailureClass::DuplicateEdgeEffect,
                super::model::Table::Edge,
                Some(index),
                Some(effect.target),
            ));
        }
        check_transfer(&analysis.domain, &effect.transfer, super::model::Table::Edge, index)?;
    }
    Ok(count)
}

fn required_budget(
    analysis: &super::model::Analysis,
    block_count: u32,
    effect_count: u32,
    edge_effect_count: u32,
) -> Result<u32, super::model::Rejection> {
    let height = analysis
        .domain
        .height
        .ok_or_else(|| reject(super::model::FailureClass::MissingHeight, super::model::Table::Domain, None, None))?;
    let work_items = block_count
        .checked_add(effect_count)
        .and_then(|count| count.checked_add(edge_effect_count))
        .ok_or_else(|| reject(super::model::FailureClass::BudgetTooLarge, super::model::Table::Domain, None, None))?;
    height
        .checked_mul(work_items)
        .and_then(|budget| budget.checked_add(1))
        .ok_or_else(|| reject(super::model::FailureClass::BudgetTooLarge, super::model::Table::Domain, None, None))
}

/// Admits a finite domain, body, effects, monotonicity, edges, and budget.
// r[impl mir-dataflow.admission]
// r[impl mir-dataflow.convergence]
pub fn admit_analysis(
    body: &crate::mir_core::Body,
    analysis: &super::model::Analysis,
) -> Result<super::model::AdmissionFacts, super::model::Rejection> {
    super::lattice::admit_domain(&analysis.domain)?;
    let facts = crate::mir_core::extract_adapter_facts(body)
        .map_err(|_| reject(super::model::FailureClass::InvalidBody, super::model::Table::Domain, None, None))?;
    if analysis.boundary_block >= facts.block_count || analysis.boundary_state.0 >= analysis.domain.state_count {
        return Err(reject(
            super::model::FailureClass::InvalidBoundary,
            super::model::Table::Domain,
            None,
            Some(analysis.boundary_block),
        ));
    }
    let effect_count = check_location_effects(body, analysis)?;
    let edge_effect_count = check_edge_effects(&facts, analysis)?;
    let required_budget = required_budget(analysis, facts.block_count, effect_count, edge_effect_count)?;
    if analysis.convergence_budget < required_budget {
        return Err(reject(
            super::model::FailureClass::BudgetTooSmall,
            super::model::Table::Domain,
            None,
            Some(required_budget),
        ));
    }
    if analysis.convergence_budget > super::model::MAX_ROUNDS {
        return Err(reject(
            super::model::FailureClass::BudgetTooLarge,
            super::model::Table::Domain,
            None,
            Some(analysis.convergence_budget),
        ));
    }
    Ok(super::model::AdmissionFacts {
        block_count: facts.block_count,
        required_budget,
        effect_count,
        edge_effect_count,
    })
}
