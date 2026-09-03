//! Deterministic bounded fixed-point execution over admitted finite tables.

fn reject(class: super::model::FailureClass, subject: Option<u32>) -> super::model::Rejection {
    super::model::Rejection {
        class,
        table: super::model::Table::Result,
        index: None,
        subject,
    }
}

fn index(value: u32) -> Result<usize, super::model::Rejection> {
    usize::try_from(value).map_err(|_| reject(super::model::FailureClass::InvalidResultPoint, Some(value)))
}

fn apply_location(
    effect: &super::model::LocationEffect,
    input: super::model::StateKey,
) -> Result<super::model::LocationState, super::model::Rejection> {
    let after_before = super::tables::apply_transfer(&effect.before, input)?;
    let after_primary = super::tables::apply_transfer(&effect.primary, after_before)?;
    Ok(super::model::LocationState {
        location: effect.location,
        input,
        after_before,
        after_primary,
    })
}

fn apply_block(
    analysis: &super::model::Analysis,
    block: u32,
    input: super::model::StateKey,
) -> Result<super::model::StateKey, super::model::Rejection> {
    let mut state = input;
    match analysis.direction {
        super::model::Direction::Forward => {
            for effect in &analysis.effects {
                if super::tables::location_block(effect.location) == Some(block) {
                    state = apply_location(effect, state)?.after_primary;
                }
            }
        }
        super::model::Direction::Backward => {
            for effect in analysis.effects.iter().rev() {
                if super::tables::location_block(effect.location) == Some(block) {
                    state = apply_location(effect, state)?.after_primary;
                }
            }
        }
    }
    Ok(state)
}

fn edge_transfer(analysis: &super::model::Analysis, source: u32, target: u32) -> Option<&super::model::Transfer> {
    analysis
        .edge_effects
        .iter()
        .find(|effect| effect.source == source && effect.target == target)
        .map(|effect| &effect.transfer)
}

fn apply_edge(
    analysis: &super::model::Analysis,
    source: u32,
    target: u32,
    input: super::model::StateKey,
) -> Result<super::model::StateKey, super::model::Rejection> {
    match edge_transfer(analysis, source, target) {
        Some(transfer) => super::tables::apply_transfer(transfer, input),
        None => Ok(input),
    }
}

fn compute_exits(
    analysis: &super::model::Analysis,
    entries: &[super::model::StateKey],
) -> Result<Vec<super::model::StateKey>, super::model::Rejection> {
    let mut exits = Vec::with_capacity(entries.len());
    for (raw_block, entry) in entries.iter().copied().enumerate() {
        let block =
            u32::try_from(raw_block).map_err(|_| reject(super::model::FailureClass::InvalidResultPoint, None))?;
        exits.push(apply_block(analysis, block, entry)?);
    }
    Ok(exits)
}

fn next_entries(
    facts: &crate::mir_core::AdapterFacts,
    analysis: &super::model::Analysis,
    exits: &[super::model::StateKey],
) -> Result<Vec<super::model::StateKey>, super::model::Rejection> {
    let bottom = analysis.domain.bottom.ok_or_else(|| reject(super::model::FailureClass::MissingBottom, None))?;
    let mut next = vec![bottom; exits.len()];
    let boundary = next
        .get_mut(index(analysis.boundary_block)?)
        .ok_or_else(|| reject(super::model::FailureClass::InvalidBoundary, Some(analysis.boundary_block)))?;
    *boundary = super::lattice::join_state(&analysis.domain, *boundary, analysis.boundary_state)?;

    for edge in &facts.edges {
        let (from, to) = match analysis.direction {
            super::model::Direction::Forward => (edge.source, edge.target),
            super::model::Direction::Backward => (edge.target, edge.source),
        };
        let output = *exits
            .get(index(from)?)
            .ok_or_else(|| reject(super::model::FailureClass::InvalidEdgeEffect, Some(from)))?;
        let transferred = apply_edge(analysis, edge.source, edge.target, output)?;
        let destination = next
            .get_mut(index(to)?)
            .ok_or_else(|| reject(super::model::FailureClass::InvalidEdgeEffect, Some(to)))?;
        *destination = super::lattice::join_state(&analysis.domain, *destination, transferred)?;
    }
    Ok(next)
}

fn capture_locations(
    analysis: &super::model::Analysis,
    entries: &[super::model::StateKey],
) -> Result<Vec<super::model::LocationState>, super::model::Rejection> {
    let mut locations = Vec::with_capacity(analysis.effects.len());
    for (raw_block, entry) in entries.iter().copied().enumerate() {
        let block =
            u32::try_from(raw_block).map_err(|_| reject(super::model::FailureClass::InvalidResultPoint, None))?;
        let mut state = entry;
        match analysis.direction {
            super::model::Direction::Forward => {
                for effect in &analysis.effects {
                    if super::tables::location_block(effect.location) == Some(block) {
                        let snapshot = apply_location(effect, state)?;
                        state = snapshot.after_primary;
                        locations.push(snapshot);
                    }
                }
            }
            super::model::Direction::Backward => {
                for effect in analysis.effects.iter().rev() {
                    if super::tables::location_block(effect.location) == Some(block) {
                        let snapshot = apply_location(effect, state)?;
                        state = snapshot.after_primary;
                        locations.push(snapshot);
                    }
                }
            }
        }
    }
    Ok(locations)
}

fn capture_edges(
    facts: &crate::mir_core::AdapterFacts,
    analysis: &super::model::Analysis,
    exits: &[super::model::StateKey],
) -> Result<Vec<super::model::EdgeState>, super::model::Rejection> {
    let mut edges = Vec::with_capacity(facts.edges.len());
    for edge in &facts.edges {
        let from = match analysis.direction {
            super::model::Direction::Forward => edge.source,
            super::model::Direction::Backward => edge.target,
        };
        let output = *exits
            .get(index(from)?)
            .ok_or_else(|| reject(super::model::FailureClass::InvalidEdgeEffect, Some(from)))?;
        edges.push(super::model::EdgeState {
            source: edge.source,
            target: edge.target,
            state: apply_edge(analysis, edge.source, edge.target, output)?,
        });
    }
    Ok(edges)
}

/// Runs the admitted finite analysis until a stable point or budget exhaustion.
// r[impl mir-dataflow.convergence]
// r[impl mir-dataflow.results]
pub fn run_to_fixed_point(
    body: &crate::mir_core::Body,
    analysis: &super::model::Analysis,
) -> Result<super::model::Results, super::model::Rejection> {
    super::tables::admit_analysis(body, analysis)?;
    let facts = crate::mir_core::extract_adapter_facts(body)
        .map_err(|_| reject(super::model::FailureClass::InvalidBody, None))?;
    let bottom = analysis.domain.bottom.ok_or_else(|| reject(super::model::FailureClass::MissingBottom, None))?;
    let block_count =
        usize::try_from(facts.block_count).map_err(|_| reject(super::model::FailureClass::InvalidBody, None))?;
    let mut entries = vec![bottom; block_count];
    let boundary = entries
        .get_mut(index(analysis.boundary_block)?)
        .ok_or_else(|| reject(super::model::FailureClass::InvalidBoundary, None))?;
    *boundary = super::lattice::join_state(&analysis.domain, *boundary, analysis.boundary_state)?;

    for round in 1..=analysis.convergence_budget {
        let exits = compute_exits(analysis, &entries)?;
        let next = next_entries(&facts, analysis, &exits)?;
        if next == entries {
            return Ok(super::model::Results {
                block_entries: entries.clone(),
                block_exits: exits.clone(),
                locations: capture_locations(analysis, &entries)?,
                edges: capture_edges(&facts, analysis, &exits)?,
                rounds: round,
            });
        }
        entries = next;
    }
    Err(reject(super::model::FailureClass::BudgetExhausted, Some(analysis.convergence_budget)))
}
