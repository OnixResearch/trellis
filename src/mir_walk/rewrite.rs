//! Shape extraction and mutation-authority admission.

fn reject(class: super::model::FailureClass) -> super::model::Rejection {
    super::model::Rejection {
        class,
        entry_index: None,
        location: None,
    }
}

fn width(len: usize) -> Result<u32, super::model::Rejection> {
    u32::try_from(len).map_err(|_| reject(super::model::FailureClass::TraceTooWide))
}

fn add(total: u32, amount: u32) -> Result<u32, super::model::Rejection> {
    total.checked_add(amount).ok_or_else(|| reject(super::model::FailureClass::TraceTooWide))
}

/// Extracts bounded structural counts from one admitted MIR-core body.
// r[impl mir-visitor.mut-shape]
pub fn body_shape(body: &crate::mir_core::Body) -> Result<super::model::Shape, super::model::Rejection> {
    crate::mir_core::admit_body(body).map_err(|_| reject(super::model::FailureClass::InvalidBody))?;
    let mut statements = 0_u32;
    let mut terminators = 0_u32;
    let mut successors = 0_u32;
    for block in &body.blocks {
        statements = add(statements, width(block.statements.len())?)?;
        let terminator = match &block.terminator {
            Some(terminator) => terminator,
            None => return Err(reject(super::model::FailureClass::InvalidBody)),
        };
        terminators = add(terminators, 1)?;
        let successor_count = super::bounds::successor_count(terminator).map_err(reject)?;
        successors = add(successors, successor_count)?;
    }
    Ok(super::model::Shape {
        locals: width(body.locals.len())?,
        blocks: width(body.blocks.len())?,
        statements,
        terminators,
        successors,
    })
}

fn shape_mismatch_count(before: super::model::Shape, after: super::model::Shape) -> u32 {
    let fields = [
        before.locals == after.locals,
        before.blocks == after.blocks,
        before.statements == after.statements,
        before.terminators == after.terminators,
        before.successors == after.successors,
    ];
    fields.iter().fold(0_u32, |count, matches| if *matches { count } else { count.saturating_add(1) })
}

/// Admits only immutable identity or value-only, shape-preserving mutation.
// r[impl mir-visitor.mut-shape]
// r[impl mir-visitor.admission]
pub fn admit_mutation(
    before: &crate::mir_core::Body,
    after: &crate::mir_core::Body,
    class: super::model::MutationClass,
) -> Result<super::model::Shape, super::model::Rejection> {
    let before_shape = body_shape(before)?;
    let after_shape = body_shape(after)?;
    let mismatch_count = shape_mismatch_count(before_shape, after_shape);
    let identity_mismatch = u32::from(before != after);
    let is_admitted = match class {
        super::model::MutationClass::Immutable => {
            super::proofs::mutation_admitted(identity_mismatch, super::proofs::PRESENT_TAG, super::proofs::PRESENT_TAG)
        }
        super::model::MutationClass::ValuesOnly => {
            super::proofs::mutation_admitted(mismatch_count, super::proofs::PRESENT_TAG, super::proofs::PRESENT_TAG)
        }
        super::model::MutationClass::UnsupportedStructural => false,
    };
    if is_admitted {
        Ok(after_shape)
    } else {
        Err(reject(super::model::FailureClass::MutationMismatch))
    }
}
