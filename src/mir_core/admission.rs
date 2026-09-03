//! Pure, deterministic admission for finite MIR-core values.

fn failure(
    class: super::model::FailureClass,
    subject_index: Option<u32>,
    projection_index: Option<u32>,
) -> super::model::Failure {
    super::model::Failure {
        class,
        subject_index,
        projection_index,
    }
}

fn table_width(len: usize) -> Result<u32, super::model::Failure> {
    u32::try_from(len).map_err(|_| failure(super::model::FailureClass::TableTooWide, None, None))
}

fn item_index(index: usize) -> Result<u32, super::model::Failure> {
    u32::try_from(index).map_err(|_| failure(super::model::FailureClass::TableTooWide, None, None))
}

fn role_tag(role: super::model::LocalRole) -> u8 {
    match role {
        super::model::LocalRole::Return => super::proofs::PRESENT_TAG,
        super::model::LocalRole::Argument | super::model::LocalRole::Variable | super::model::LocalRole::Temporary => {
            super::proofs::NON_RETURN_ROLE_TAG
        }
    }
}

fn projection_parts(projection: super::model::Projection) -> (u8, u32) {
    match projection {
        super::model::Projection::Index { local } => (super::proofs::INDEX_PROJECTION_TAG, local),
        super::model::Projection::Unsupported => (super::proofs::UNSUPPORTED_PROJECTION_TAG, 0),
        super::model::Projection::Dereference | super::model::Projection::Field { .. } => {
            (super::proofs::MISSING_TAG, 0)
        }
    }
}

/// Admits one place against an explicit local-table width.
// r[impl mir-core.local-reference]
// r[impl mir-core.place-projection]
pub fn admit_place(place: &super::model::Place, local_count: u32) -> Result<(), super::model::Failure> {
    if !super::proofs::local_index_admitted(place.local, local_count) {
        return Err(failure(super::model::FailureClass::InvalidLocal, Some(place.local), None));
    }

    for (raw_index, projection) in place.projections.iter().copied().enumerate() {
        let projection_index = item_index(raw_index)?;
        let (projection_tag, index_local) = projection_parts(projection);
        if !super::proofs::projection_admitted(projection_tag, index_local, local_count) {
            let class = if projection_tag == super::proofs::UNSUPPORTED_PROJECTION_TAG {
                super::model::FailureClass::UnsupportedProjection
            } else {
                super::model::FailureClass::InvalidLocal
            };
            let subject_index = if projection_tag == super::proofs::INDEX_PROJECTION_TAG {
                Some(index_local)
            } else {
                None
            };
            return Err(failure(class, subject_index, Some(projection_index)));
        }
    }
    Ok(())
}

/// Admits one operand and each place that it references.
// r[impl mir-core.local-reference]
// r[impl mir-core.rvalue-nonnesting]
pub fn admit_operand(operand: &super::model::Operand, local_count: u32) -> Result<(), super::model::Failure> {
    match operand {
        super::model::Operand::Constant(_) => Ok(()),
        super::model::Operand::Copy(place) | super::model::Operand::Move(place) => admit_place(place, local_count),
    }
}

/// Admits one shallow rvalue and its operands or place.
// r[impl mir-core.rvalue-nonnesting]
pub fn admit_rvalue(rvalue: &super::model::Rvalue, local_count: u32) -> Result<(), super::model::Failure> {
    let rvalue_tag = if matches!(rvalue, super::model::Rvalue::Nested) {
        super::proofs::NESTED_RVALUE_TAG
    } else {
        super::proofs::ADMITTED_RVALUE_TAG
    };
    if !super::proofs::rvalue_shape_admitted(rvalue_tag) {
        return Err(failure(super::model::FailureClass::NestedRvalue, None, None));
    }

    match rvalue {
        super::model::Rvalue::Use(operand) => admit_operand(operand, local_count),
        super::model::Rvalue::Binary { left, right, .. } => {
            admit_operand(left, local_count)?;
            admit_operand(right, local_count)
        }
        super::model::Rvalue::Reference(place) => admit_place(place, local_count),
        super::model::Rvalue::Aggregate(operands) => {
            for operand in operands {
                admit_operand(operand, local_count)?;
            }
            Ok(())
        }
        super::model::Rvalue::Nested => Err(failure(super::model::FailureClass::NestedRvalue, None, None)),
    }
}

/// Admits one statement and every local reference that it contains.
// r[impl mir-core.local-reference]
// r[impl mir-core.rvalue-nonnesting]
pub fn admit_statement(statement: &super::model::Statement, local_count: u32) -> Result<(), super::model::Failure> {
    match statement {
        super::model::Statement::Assign { destination, value } => {
            admit_place(destination, local_count)?;
            admit_rvalue(value, local_count)
        }
        super::model::Statement::StorageLive { local } | super::model::Statement::StorageDead { local } => {
            if super::proofs::local_index_admitted(*local, local_count) {
                Ok(())
            } else {
                Err(failure(super::model::FailureClass::InvalidLocal, Some(*local), None))
            }
        }
        super::model::Statement::Nop => Ok(()),
    }
}

fn admit_successor(target: u32, block_count: u32) -> Result<(), super::model::Failure> {
    if super::proofs::successor_admitted(target, block_count) {
        Ok(())
    } else {
        Err(failure(super::model::FailureClass::OutOfRangeSuccessor, Some(target), None))
    }
}

/// Admits one terminator, its local references, and its successor targets.
// r[impl mir-core.local-reference]
// r[impl mir-core.successors]
pub fn admit_terminator(
    terminator: &super::model::Terminator,
    local_count: u32,
    block_count: u32,
) -> Result<(), super::model::Failure> {
    match terminator {
        super::model::Terminator::Return | super::model::Terminator::Unreachable => Ok(()),
        super::model::Terminator::Goto { target } => admit_successor(*target, block_count),
        super::model::Terminator::Switch { discriminant, targets } => {
            admit_operand(discriminant, local_count)?;
            for target in targets {
                admit_successor(*target, block_count)?;
            }
            Ok(())
        }
        super::model::Terminator::Call {
            function,
            arguments,
            destination,
        } => {
            admit_operand(function, local_count)?;
            for argument in arguments {
                admit_operand(argument, local_count)?;
            }
            if let Some((place, target)) = destination {
                admit_place(place, local_count)?;
                admit_successor(*target, block_count)?;
            }
            Ok(())
        }
        super::model::Terminator::Drop { place, target } => {
            admit_place(place, local_count)?;
            admit_successor(*target, block_count)
        }
    }
}

fn rejection(
    failure: super::model::Failure,
    block_index: Option<u32>,
    statement_index: Option<u32>,
) -> super::model::Rejection {
    super::model::Rejection {
        class: failure.class,
        block_index,
        statement_index,
        subject_index: failure.subject_index,
        projection_index: failure.projection_index,
    }
}

/// Admits one body and returns its deterministic first structural failure.
// r[impl mir-core.body-shape]
// r[impl mir-core.return-place]
// r[impl mir-core.block-terminator]
// r[impl mir-core.failure-metadata]
pub fn admit_body(body: &super::model::Body) -> Result<(), super::model::Rejection> {
    if body.blocks.is_empty() {
        return Err(rejection(failure(super::model::FailureClass::EmptyBlockTable, None, None), None, None));
    }
    let local_count = table_width(body.locals.len()).map_err(|item| rejection(item, None, None))?;
    let block_count = table_width(body.blocks.len()).map_err(|item| rejection(item, None, None))?;
    let first_role_tag = body.locals.first().map_or(super::proofs::MISSING_TAG, |local| role_tag(local.role));
    if !super::proofs::return_place_admitted(local_count, first_role_tag) {
        return Err(rejection(
            failure(super::model::FailureClass::MissingReturnPlace, Some(super::model::RETURN_PLACE), None),
            None,
            None,
        ));
    }

    for (raw_block_index, block) in body.blocks.iter().enumerate() {
        let block_index = item_index(raw_block_index).map_err(|item| rejection(item, None, None))?;
        let terminator_tag = if block.terminator.is_some() {
            super::proofs::PRESENT_TAG
        } else {
            super::proofs::MISSING_TAG
        };
        if !super::proofs::final_terminator_admitted(terminator_tag) {
            return Err(rejection(
                failure(super::model::FailureClass::MissingTerminator, None, None),
                Some(block_index),
                None,
            ));
        }
        let terminator = match &block.terminator {
            Some(terminator) => terminator,
            None => {
                return Err(rejection(
                    failure(super::model::FailureClass::MissingTerminator, None, None),
                    Some(block_index),
                    None,
                ));
            }
        };
        for (raw_statement_index, statement) in block.statements.iter().enumerate() {
            let statement_index =
                item_index(raw_statement_index).map_err(|item| rejection(item, Some(block_index), None))?;
            admit_statement(statement, local_count)
                .map_err(|item| rejection(item, Some(block_index), Some(statement_index)))?;
        }
        admit_terminator(terminator, local_count, block_count)
            .map_err(|item| rejection(item, Some(block_index), None))?;
    }
    Ok(())
}
