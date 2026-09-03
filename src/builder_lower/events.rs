//! Source-location, handle-order, and control-operation admission.

#[derive(Clone, Copy, PartialEq, Eq)]
enum TerminalEffect {
    Continue,
    Finish,
}

struct DefinedValues {
    items: Vec<super::model::ValueKey>,
}

impl DefinedValues {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    fn require(&self, key: super::model::ValueKey, index: u32) -> Result<(), super::model::Rejection> {
        if self.items.contains(&key) {
            Ok(())
        } else {
            Err(reject(super::model::FailureClass::UseBeforeDefinition, index, Some(key.0)))
        }
    }

    fn define(&mut self, key: super::model::ValueKey, index: u32) -> Result<(), super::model::Rejection> {
        if self.items.contains(&key) {
            return Err(reject(super::model::FailureClass::DuplicateValueDefinition, index, Some(key.0)));
        }
        self.items.push(key);
        Ok(())
    }
}

struct Context<'a> {
    body: &'a crate::mir_core::Body,
    record: &'a super::model::Record,
    edges: &'a [crate::mir_core::Edge],
}

fn reject(class: super::model::FailureClass, index: u32, subject: Option<u32>) -> super::model::Rejection {
    super::model::Rejection {
        class,
        table: super::model::Table::Entry,
        index: Some(index),
        subject,
    }
}

fn position(index: usize) -> Result<u32, super::model::Rejection> {
    u32::try_from(index).map_err(|_| super::model::Rejection {
        class: super::model::FailureClass::TableTooWide,
        table: super::model::Table::Entry,
        index: None,
        subject: None,
    })
}

fn source_block(location: crate::mir_visitor_traversal::Location) -> Option<u32> {
    match location {
        crate::mir_visitor_traversal::Location::Statement { block, .. }
        | crate::mir_visitor_traversal::Location::Terminator { block }
        | crate::mir_visitor_traversal::Location::Rvalue { block, .. } => Some(block),
        crate::mir_visitor_traversal::Location::Place { owner, .. }
        | crate::mir_visitor_traversal::Location::Operand { owner, .. } => match owner {
            crate::mir_visitor_traversal::Owner::Statement { block, .. }
            | crate::mir_visitor_traversal::Owner::Terminator { block } => Some(block),
        },
        crate::mir_visitor_traversal::Location::Body
        | crate::mir_visitor_traversal::Location::Local { .. }
        | crate::mir_visitor_traversal::Location::Block { .. }
        | crate::mir_visitor_traversal::Location::Successor { .. } => None,
    }
}

fn operation_matches_location(
    operation: &super::model::Operation,
    location: crate::mir_visitor_traversal::Location,
) -> bool {
    if matches!(operation, super::model::Operation::Unsupported) {
        return true;
    }
    matches!(
        (operation, location),
        (
            super::model::Operation::EmitStatement { .. },
            crate::mir_visitor_traversal::Location::Statement { .. }
        ) | (super::model::Operation::EmitOperand { .. }, crate::mir_visitor_traversal::Location::Operand { .. })
            | (super::model::Operation::EmitPlace { .. }, crate::mir_visitor_traversal::Location::Place { .. })
            | (super::model::Operation::EmitRvalue { .. }, crate::mir_visitor_traversal::Location::Rvalue { .. })
            | (
                super::model::Operation::Branch { .. }
                    | super::model::Operation::Call { .. }
                    | super::model::Operation::Return { .. }
                    | super::model::Operation::Unreachable,
                crate::mir_visitor_traversal::Location::Terminator { .. }
            )
    )
}

fn control_target_is_allowed(context: &Context<'_>, source: u32, target: super::model::BlockKey) -> bool {
    if super::layout::mapping_for(context.record, source).is_some_and(|mapping| mapping.expansions.contains(&target)) {
        return true;
    }
    context.edges.iter().any(|edge| {
        edge.source == source
            && super::layout::mapping_for(context.record, edge.target).is_some_and(|mapping| mapping.direct == target)
    })
}

fn require_values(
    values: &DefinedValues,
    inputs: &[super::model::ValueKey],
    index: u32,
) -> Result<(), super::model::Rejection> {
    for input in inputs {
        values.require(*input, index)?;
    }
    Ok(())
}

fn require_declared<T: PartialEq>(
    declarations: &[T],
    value: &T,
    subject: u32,
    index: u32,
) -> Result<(), super::model::Rejection> {
    let declared_tag = if declarations.contains(value) {
        super::proofs::PRESENT_TAG
    } else {
        super::proofs::MISSING_TAG
    };
    if super::proofs::handle_declared(declared_tag) {
        Ok(())
    } else {
        Err(reject(super::model::FailureClass::MissingDeclaration, index, Some(subject)))
    }
}

fn require_target(
    context: &Context<'_>,
    source: u32,
    target: super::model::BlockKey,
    index: u32,
) -> Result<(), super::model::Rejection> {
    let target_tag = if control_target_is_allowed(context, source, target) {
        super::proofs::PRESENT_TAG
    } else {
        super::proofs::MISSING_TAG
    };
    if super::proofs::control_target_admitted(target_tag) {
        Ok(())
    } else {
        Err(reject(super::model::FailureClass::InvalidControlTarget, index, Some(target.0)))
    }
}

fn apply_operation(
    context: &Context<'_>,
    entry: &super::model::Entry,
    source: u32,
    index: u32,
    values: &mut DefinedValues,
) -> Result<TerminalEffect, super::model::Rejection> {
    match &entry.operation {
        super::model::Operation::EmitStatement { inputs } => {
            require_values(values, inputs, index)?;
            Ok(TerminalEffect::Continue)
        }
        super::model::Operation::EmitOperand { result, type_key } => {
            require_declared(&context.record.declarations.types, type_key, type_key.0, index)?;
            values.define(*result, index)?;
            Ok(TerminalEffect::Continue)
        }
        super::model::Operation::EmitPlace { place } => {
            require_declared(&context.record.declarations.places, place, place.0, index)?;
            Ok(TerminalEffect::Continue)
        }
        super::model::Operation::EmitRvalue {
            inputs,
            result,
            type_key,
        } => {
            require_values(values, inputs, index)?;
            require_declared(&context.record.declarations.types, type_key, type_key.0, index)?;
            values.define(*result, index)?;
            Ok(TerminalEffect::Continue)
        }
        super::model::Operation::Branch { targets } => {
            if targets.is_empty() {
                return Err(reject(super::model::FailureClass::InvalidControlTarget, index, None));
            }
            for target in targets {
                require_target(context, source, *target, index)?;
            }
            Ok(TerminalEffect::Finish)
        }
        super::model::Operation::Call {
            function,
            arguments,
            result,
            return_block,
            unwind_block,
        } => {
            require_declared(&context.record.declarations.functions, function, function.0, index)?;
            require_values(values, arguments, index)?;
            require_target(context, source, *return_block, index)?;
            if let Some(unwind) = unwind_block {
                require_target(context, source, *unwind, index)?;
            }
            if let Some((result, type_key)) = result {
                require_declared(&context.record.declarations.types, type_key, type_key.0, index)?;
                values.define(*result, index)?;
            }
            Ok(TerminalEffect::Finish)
        }
        super::model::Operation::Return { value } => {
            if let Some(value) = value {
                values.require(*value, index)?;
            }
            Ok(TerminalEffect::Finish)
        }
        super::model::Operation::Unreachable => Ok(TerminalEffect::Finish),
        super::model::Operation::Unsupported => {
            Err(reject(super::model::FailureClass::UnsupportedOperation, index, None))
        }
    }
}

// r[impl backend-trace.lowering-trace]
// r[impl backend-trace.handle-validity]
// r[impl backend-trace.terminators]
pub(super) fn check_entries(
    body: &crate::mir_core::Body,
    record: &super::model::Record,
) -> Result<u32, super::model::Rejection> {
    let facts = crate::mir_core::extract_adapter_facts(body).map_err(|_| super::model::Rejection {
        class: super::model::FailureClass::InvalidBody,
        table: super::model::Table::Entry,
        index: None,
        subject: None,
    })?;
    let context = Context {
        body,
        record,
        edges: &facts.edges,
    };
    let mut values = DefinedValues::with_capacity(record.entries.len());
    let mut terminals = vec![false; body.blocks.len()];
    for (raw_index, entry) in record.entries.iter().enumerate() {
        let index = position(raw_index)?;
        crate::mir_visitor_traversal::admit_location(context.body, &entry.source)
            .map_err(|_| reject(super::model::FailureClass::InvalidLocation, index, None))?;
        let source = source_block(entry.source)
            .ok_or_else(|| reject(super::model::FailureClass::InvalidLocation, index, None))?;
        if !operation_matches_location(&entry.operation, entry.source) {
            return Err(reject(super::model::FailureClass::LocationKindMismatch, index, None));
        }
        if !super::layout::emission_is_allowed(record, source, entry.emission_block) {
            return Err(reject(super::model::FailureClass::EmissionBlockMismatch, index, Some(entry.emission_block.0)));
        }
        let source_index = usize::try_from(source)
            .map_err(|_| reject(super::model::FailureClass::InvalidSourceBlock, index, Some(source)))?;
        let terminal = terminals
            .get_mut(source_index)
            .ok_or_else(|| reject(super::model::FailureClass::InvalidSourceBlock, index, Some(source)))?;
        if *terminal {
            return Err(reject(super::model::FailureClass::OperationAfterTerminator, index, Some(source)));
        }
        if apply_operation(&context, entry, source, index, &mut values)? == TerminalEffect::Finish {
            *terminal = true;
        }
    }
    let mut terminal_count = 0_u32;
    for (raw_block, terminal) in terminals.iter().copied().enumerate() {
        let block = position(raw_block)?;
        if !terminal {
            return Err(reject(super::model::FailureClass::MissingTerminator, block, Some(block)));
        }
        terminal_count = terminal_count
            .checked_add(1)
            .ok_or_else(|| reject(super::model::FailureClass::TableTooWide, block, None))?;
    }
    let block_count = position(body.blocks.len())?;
    if !super::proofs::terminal_coverage_admitted(block_count, terminal_count) {
        return Err(reject(super::model::FailureClass::MissingTerminator, block_count, None));
    }
    position(values.items.len())
}
