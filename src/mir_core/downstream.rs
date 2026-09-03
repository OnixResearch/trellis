//! Downstream CFG and table-width facts for admitted MIR bodies.

fn push_edge(
    mut edges: Vec<super::model::Edge>,
    source: u32,
    target: u32,
    block_count: u32,
) -> Result<Vec<super::model::Edge>, super::model::Rejection> {
    if !super::proofs::adapter_edge_admitted(source, target, block_count) {
        return Err(super::model::Rejection {
            class: super::model::FailureClass::OutOfRangeSuccessor,
            block_index: Some(source),
            statement_index: None,
            subject_index: Some(target),
            projection_index: None,
        });
    }
    edges.push(super::model::Edge { source, target });
    Ok(edges)
}

fn append_terminator_edges(
    terminator: &super::model::Terminator,
    source: u32,
    block_count: u32,
    mut edges: Vec<super::model::Edge>,
) -> Result<Vec<super::model::Edge>, super::model::Rejection> {
    match terminator {
        super::model::Terminator::Goto { target } | super::model::Terminator::Drop { target, .. } => {
            push_edge(edges, source, *target, block_count)
        }
        super::model::Terminator::Switch { targets, .. } => {
            for target in targets {
                edges = push_edge(edges, source, *target, block_count)?;
            }
            Ok(edges)
        }
        super::model::Terminator::Call {
            destination: Some((_, target)),
            ..
        } => push_edge(edges, source, *target, block_count),
        super::model::Terminator::Call { destination: None, .. }
        | super::model::Terminator::Return
        | super::model::Terminator::Unreachable => Ok(edges),
    }
}

/// Validates a body before exposing bounded downstream facts.
// r[impl mir-core.downstream-adapter]
// r[impl mir-core.failure-metadata]
pub fn extract_adapter_facts(body: &super::model::Body) -> Result<super::model::AdapterFacts, super::model::Rejection> {
    super::admission::admit_body(body)?;

    let block_count = u32::try_from(body.blocks.len()).map_err(|_| super::model::Rejection {
        class: super::model::FailureClass::TableTooWide,
        block_index: None,
        statement_index: None,
        subject_index: None,
        projection_index: None,
    })?;
    let local_count = u32::try_from(body.locals.len()).map_err(|_| super::model::Rejection {
        class: super::model::FailureClass::TableTooWide,
        block_index: None,
        statement_index: None,
        subject_index: None,
        projection_index: None,
    })?;
    let mut edges = Vec::new();
    for (raw_source, block) in body.blocks.iter().enumerate() {
        let source = u32::try_from(raw_source).map_err(|_| super::model::Rejection {
            class: super::model::FailureClass::TableTooWide,
            block_index: None,
            statement_index: None,
            subject_index: None,
            projection_index: None,
        })?;
        let terminator = match &block.terminator {
            Some(terminator) => terminator,
            None => {
                return Err(super::model::Rejection {
                    class: super::model::FailureClass::MissingTerminator,
                    block_index: Some(source),
                    statement_index: None,
                    subject_index: None,
                    projection_index: None,
                });
            }
        };
        edges = append_terminator_edges(terminator, source, block_count, edges)?;
    }

    Ok(super::model::AdapterFacts {
        block_count,
        local_count,
        edges,
    })
}
