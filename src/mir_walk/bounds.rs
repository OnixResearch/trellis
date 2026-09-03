//! Bounds checks for every explicit MIR visit-location family.

const ONE_COMPONENT: u32 = 1;
const BINARY_OPERAND_COUNT: u32 = 2;

fn width(len: usize) -> Result<u32, super::model::FailureClass> {
    u32::try_from(len).map_err(|_| super::model::FailureClass::TraceTooWide)
}

fn index(value: u32) -> Result<usize, super::model::FailureClass> {
    usize::try_from(value).map_err(|_| super::model::FailureClass::InvalidLocation)
}

fn block(
    body: &crate::mir_core::Body,
    block_index: u32,
) -> Result<&crate::mir_core::BasicBlock, super::model::FailureClass> {
    body.blocks.get(index(block_index)?).ok_or(super::model::FailureClass::InvalidLocation)
}

fn statement(
    body: &crate::mir_core::Body,
    block_index: u32,
    statement_index: u32,
) -> Result<&crate::mir_core::Statement, super::model::FailureClass> {
    block(body, block_index)?
        .statements
        .get(index(statement_index)?)
        .ok_or(super::model::FailureClass::InvalidLocation)
}

fn terminator(
    body: &crate::mir_core::Body,
    block_index: u32,
) -> Result<&crate::mir_core::Terminator, super::model::FailureClass> {
    block(body, block_index)?.terminator.as_ref().ok_or(super::model::FailureClass::InvalidLocation)
}

fn operand_place_count(operand: &crate::mir_core::Operand) -> u32 {
    match operand {
        crate::mir_core::Operand::Constant(_) => 0,
        crate::mir_core::Operand::Copy(_) | crate::mir_core::Operand::Move(_) => ONE_COMPONENT,
    }
}

fn operands_place_count(operands: &[crate::mir_core::Operand]) -> Result<u32, super::model::FailureClass> {
    let mut count = 0_u32;
    for operand in operands {
        count = count.checked_add(operand_place_count(operand)).ok_or(super::model::FailureClass::TraceTooWide)?;
    }
    Ok(count)
}

fn rvalue_operand_count(rvalue: &crate::mir_core::Rvalue) -> Result<u32, super::model::FailureClass> {
    match rvalue {
        crate::mir_core::Rvalue::Use(_) => Ok(ONE_COMPONENT),
        crate::mir_core::Rvalue::Binary { .. } => Ok(BINARY_OPERAND_COUNT),
        crate::mir_core::Rvalue::Reference(_) | crate::mir_core::Rvalue::Nested => Ok(0),
        crate::mir_core::Rvalue::Aggregate(operands) => width(operands.len()),
    }
}

fn rvalue_place_count(rvalue: &crate::mir_core::Rvalue) -> Result<u32, super::model::FailureClass> {
    match rvalue {
        crate::mir_core::Rvalue::Use(operand) => Ok(operand_place_count(operand)),
        crate::mir_core::Rvalue::Binary { left, right, .. } => operand_place_count(left)
            .checked_add(operand_place_count(right))
            .ok_or(super::model::FailureClass::TraceTooWide),
        crate::mir_core::Rvalue::Reference(_) => Ok(ONE_COMPONENT),
        crate::mir_core::Rvalue::Aggregate(operands) => operands_place_count(operands),
        crate::mir_core::Rvalue::Nested => Ok(0),
    }
}

fn statement_operand_count(statement: &crate::mir_core::Statement) -> Result<u32, super::model::FailureClass> {
    match statement {
        crate::mir_core::Statement::Assign { value, .. } => rvalue_operand_count(value),
        crate::mir_core::Statement::StorageLive { .. }
        | crate::mir_core::Statement::StorageDead { .. }
        | crate::mir_core::Statement::Nop => Ok(0),
    }
}

fn statement_place_count(statement: &crate::mir_core::Statement) -> Result<u32, super::model::FailureClass> {
    match statement {
        crate::mir_core::Statement::Assign { value, .. } => ONE_COMPONENT
            .checked_add(rvalue_place_count(value)?)
            .ok_or(super::model::FailureClass::TraceTooWide),
        crate::mir_core::Statement::StorageLive { .. }
        | crate::mir_core::Statement::StorageDead { .. }
        | crate::mir_core::Statement::Nop => Ok(0),
    }
}

fn terminator_operand_count(terminator: &crate::mir_core::Terminator) -> Result<u32, super::model::FailureClass> {
    match terminator {
        crate::mir_core::Terminator::Switch { .. } => Ok(ONE_COMPONENT),
        crate::mir_core::Terminator::Call { arguments, .. } => {
            ONE_COMPONENT.checked_add(width(arguments.len())?).ok_or(super::model::FailureClass::TraceTooWide)
        }
        crate::mir_core::Terminator::Return
        | crate::mir_core::Terminator::Goto { .. }
        | crate::mir_core::Terminator::Drop { .. }
        | crate::mir_core::Terminator::Unreachable => Ok(0),
    }
}

fn terminator_place_count(terminator: &crate::mir_core::Terminator) -> Result<u32, super::model::FailureClass> {
    match terminator {
        crate::mir_core::Terminator::Switch { discriminant, .. } => Ok(operand_place_count(discriminant)),
        crate::mir_core::Terminator::Call {
            function,
            arguments,
            destination,
        } => {
            let operand_places = operand_place_count(function)
                .checked_add(operands_place_count(arguments)?)
                .ok_or(super::model::FailureClass::TraceTooWide)?;
            operand_places
                .checked_add(u32::from(destination.is_some()))
                .ok_or(super::model::FailureClass::TraceTooWide)
        }
        crate::mir_core::Terminator::Drop { .. } => Ok(ONE_COMPONENT),
        crate::mir_core::Terminator::Return
        | crate::mir_core::Terminator::Goto { .. }
        | crate::mir_core::Terminator::Unreachable => Ok(0),
    }
}

pub(super) fn successor_count(terminator: &crate::mir_core::Terminator) -> Result<u32, super::model::FailureClass> {
    match terminator {
        crate::mir_core::Terminator::Goto { .. } | crate::mir_core::Terminator::Drop { .. } => Ok(ONE_COMPONENT),
        crate::mir_core::Terminator::Switch { targets, .. } => width(targets.len()),
        crate::mir_core::Terminator::Call { destination, .. } => Ok(u32::from(destination.is_some())),
        crate::mir_core::Terminator::Return | crate::mir_core::Terminator::Unreachable => Ok(0),
    }
}

enum NestedKind {
    Place,
    Operand,
}

fn owner_count(
    body: &crate::mir_core::Body,
    owner: super::model::Owner,
    kind: NestedKind,
) -> Result<u32, super::model::FailureClass> {
    match (owner, kind) {
        (super::model::Owner::Statement { block, statement: item }, NestedKind::Place) => {
            statement_place_count(statement(body, block, item)?)
        }
        (super::model::Owner::Statement { block, statement: item }, NestedKind::Operand) => {
            statement_operand_count(statement(body, block, item)?)
        }
        (super::model::Owner::Terminator { block }, NestedKind::Place) => {
            terminator_place_count(terminator(body, block)?)
        }
        (super::model::Owner::Terminator { block }, NestedKind::Operand) => {
            terminator_operand_count(terminator(body, block)?)
        }
    }
}

fn ordinal_is_admitted(ordinal: u32, count: u32) -> bool {
    super::proofs::index_admitted(ordinal, count)
}

/// Admits one explicit visit location against an admitted MIR-core body.
// r[impl mir-visitor.locations]
// r[impl mir-visitor.admission]
pub fn admit_location(
    body: &crate::mir_core::Body,
    location: &super::model::Location,
) -> Result<(), super::model::FailureClass> {
    crate::mir_core::admit_body(body).map_err(|_| super::model::FailureClass::InvalidBody)?;
    match *location {
        super::model::Location::Body => Ok(()),
        super::model::Location::Local { local } => {
            if ordinal_is_admitted(local, width(body.locals.len())?) {
                Ok(())
            } else {
                Err(super::model::FailureClass::InvalidLocation)
            }
        }
        super::model::Location::Block { block: item } | super::model::Location::Terminator { block: item } => {
            block(body, item).map(|_| ())
        }
        super::model::Location::Statement {
            block: owner,
            statement: item,
        }
        | super::model::Location::Rvalue {
            block: owner,
            statement: item,
        } => {
            let statement = statement(body, owner, item)?;
            if matches!(location, super::model::Location::Rvalue { .. })
                && !matches!(statement, crate::mir_core::Statement::Assign { .. })
            {
                return Err(super::model::FailureClass::InvalidLocation);
            }
            Ok(())
        }
        super::model::Location::Place { owner, ordinal } => {
            if ordinal_is_admitted(ordinal, owner_count(body, owner, NestedKind::Place)?) {
                Ok(())
            } else {
                Err(super::model::FailureClass::InvalidLocation)
            }
        }
        super::model::Location::Operand { owner, ordinal } => {
            if ordinal_is_admitted(ordinal, owner_count(body, owner, NestedKind::Operand)?) {
                Ok(())
            } else {
                Err(super::model::FailureClass::InvalidLocation)
            }
        }
        super::model::Location::Successor { block: owner, ordinal } => {
            if ordinal_is_admitted(ordinal, successor_count(terminator(body, owner)?)?) {
                Ok(())
            } else {
                Err(super::model::FailureClass::InvalidLocation)
            }
        }
    }
}
