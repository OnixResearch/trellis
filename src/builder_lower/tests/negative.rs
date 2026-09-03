use super::super::*;

const FIRST: u32 = 0;
const SECOND: u32 = 1;
const FIRST_POSITION: usize = 0;
const SECOND_POSITION: usize = 1;
const THIRD: u32 = 2;
const CALL_ENTRY: u32 = 4;
const BRANCH_ENTRY: u32 = 5;
const APPENDED_ENTRY: u32 = 7;
const MISSING_ORDINAL: u32 = 9;

fn rejection(class: FailureClass, table: Table, index: u32, subject: Option<u32>) -> Rejection {
    Rejection {
        class,
        table,
        index: Some(index),
        subject,
    }
}

fn statement_entry(operation: Operation) -> Entry {
    Entry {
        source: crate::mir_visitor_traversal::Location::Statement {
            block: super::ENTRY_BLOCK,
            statement: 0,
        },
        emission_block: BlockKey(super::DIRECT_ENTRY),
        operation,
    }
}

// r[verify backend-trace.handles]
// r[verify backend-trace.evidence]
#[test]
fn undeclared_type_handle_is_rejected() {
    let mut record = super::valid_record();
    record.declarations.types = Vec::new();
    assert_eq!(
        admit_record(&super::body(), &record),
        Err(rejection(FailureClass::MissingDeclaration, Table::Entry, FIRST, Some(super::TYPE),))
    );
}

// r[verify backend-trace.block-mapping]
// r[verify backend-trace.evidence]
#[test]
fn out_of_range_source_block_mapping_is_rejected() {
    let mut record = super::valid_record();
    record.mappings = vec![BlockMapping {
        source_block: super::UNKNOWN_BLOCK,
        direct: BlockKey(super::DIRECT_ENTRY),
        expansions: Vec::new(),
    }];
    assert_eq!(
        admit_record(&super::body(), &record),
        Err(rejection(FailureClass::InvalidSourceBlock, Table::Mapping, FIRST, Some(super::UNKNOWN_BLOCK),))
    );
}

// r[verify backend-trace.block-mapping]
// r[verify backend-trace.evidence]
#[test]
fn missing_block_mapping_is_rejected() {
    let mut record = super::valid_record();
    record.mappings.retain(|mapping| mapping.source_block != super::EXIT_BLOCK);
    assert_eq!(
        admit_record(&super::body(), &record),
        Err(rejection(FailureClass::MissingMapping, Table::Mapping, super::EXIT_BLOCK, Some(super::EXIT_BLOCK),))
    );
}

// r[verify backend-trace.block-mapping]
// r[verify backend-trace.evidence]
#[test]
fn duplicate_block_assignment_is_rejected() {
    let mut record = super::valid_record();
    for mapping in &mut record.mappings {
        if mapping.source_block == super::NEXT_BLOCK {
            mapping.direct = BlockKey(super::DIRECT_ENTRY);
        }
    }
    assert_eq!(
        admit_record(&super::body(), &record),
        Err(rejection(FailureClass::DuplicateBlockAssignment, Table::Mapping, SECOND, Some(super::DIRECT_ENTRY),))
    );
}

// r[verify backend-trace.lowering-trace]
// r[verify backend-trace.evidence]
#[test]
fn undeclared_expansion_emission_is_rejected() {
    let mut record = super::valid_record();
    let mut entry = statement_entry(Operation::EmitStatement { inputs: Vec::new() });
    entry.emission_block = BlockKey(super::UNKNOWN_BLOCK);
    record.entries.insert(FIRST_POSITION, entry);
    assert_eq!(
        admit_record(&super::body(), &record),
        Err(rejection(FailureClass::EmissionBlockMismatch, Table::Entry, FIRST, Some(super::UNKNOWN_BLOCK),))
    );
}

// r[verify backend-trace.lowering-trace]
// r[verify backend-trace.evidence]
#[test]
fn invalid_source_location_is_rejected() {
    let mut record = super::valid_record();
    record.entries.insert(FIRST_POSITION, Entry {
        source: crate::mir_visitor_traversal::Location::Operand {
            owner: super::statement_owner(),
            ordinal: MISSING_ORDINAL,
        },
        emission_block: BlockKey(super::DIRECT_ENTRY),
        operation: Operation::EmitOperand {
            result: ValueKey(super::UNKNOWN_VALUE),
            type_key: TypeKey(super::TYPE),
        },
    });
    assert_eq!(
        admit_record(&super::body(), &record),
        Err(rejection(FailureClass::InvalidLocation, Table::Entry, FIRST, None))
    );
}

// r[verify backend-trace.lowering-trace]
// r[verify backend-trace.evidence]
#[test]
fn operation_kind_must_match_source_location() {
    let mut record = super::valid_record();
    record.entries.insert(FIRST_POSITION, Entry {
        source: crate::mir_visitor_traversal::Location::Operand {
            owner: super::statement_owner(),
            ordinal: FIRST,
        },
        emission_block: BlockKey(super::DIRECT_ENTRY),
        operation: Operation::EmitPlace {
            place: PlaceKey(super::PLACE),
        },
    });
    assert_eq!(
        admit_record(&super::body(), &record),
        Err(rejection(FailureClass::LocationKindMismatch, Table::Entry, FIRST, None,))
    );
}

// r[verify backend-trace.handle-validity]
// r[verify backend-trace.evidence]
#[test]
fn value_use_before_definition_is_rejected() {
    let mut record = super::valid_record();
    record.entries.insert(
        FIRST_POSITION,
        statement_entry(Operation::EmitStatement {
            inputs: vec![ValueKey(super::UNKNOWN_VALUE)],
        }),
    );
    assert_eq!(
        admit_record(&super::body(), &record),
        Err(rejection(FailureClass::UseBeforeDefinition, Table::Entry, FIRST, Some(super::UNKNOWN_VALUE),))
    );
}

// r[verify backend-trace.handle-validity]
// r[verify backend-trace.evidence]
#[test]
fn duplicate_value_definition_is_rejected() {
    let mut record = super::valid_record();
    record.entries.insert(SECOND_POSITION, Entry {
        source: crate::mir_visitor_traversal::Location::Operand {
            owner: super::statement_owner(),
            ordinal: SECOND,
        },
        emission_block: BlockKey(super::DIRECT_ENTRY),
        operation: Operation::EmitOperand {
            result: ValueKey(super::FIRST_VALUE),
            type_key: TypeKey(super::TYPE),
        },
    });
    assert_eq!(
        admit_record(&super::body(), &record),
        Err(rejection(FailureClass::DuplicateValueDefinition, Table::Entry, SECOND, Some(super::FIRST_VALUE),))
    );
}

// r[verify backend-trace.terminators]
// r[verify backend-trace.evidence]
#[test]
fn invalid_branch_target_is_rejected() {
    let mut record = super::valid_record();
    for entry in &mut record.entries {
        if matches!(entry.operation, Operation::Branch { .. }) {
            entry.operation = Operation::Branch {
                targets: vec![BlockKey(super::UNKNOWN_BLOCK)],
            };
        }
    }
    assert_eq!(
        admit_record(&super::body(), &record),
        Err(rejection(
            FailureClass::InvalidControlTarget,
            Table::Entry,
            BRANCH_ENTRY,
            Some(super::UNKNOWN_BLOCK),
        ))
    );
}

// r[verify backend-trace.terminators]
// r[verify backend-trace.evidence]
#[test]
fn missing_terminator_operation_is_rejected() {
    let mut record = super::valid_record();
    record.entries.retain(|entry| {
        entry.source
            != crate::mir_visitor_traversal::Location::Terminator {
                block: super::EXIT_BLOCK,
            }
    });
    assert_eq!(
        admit_record(&super::body(), &record),
        Err(rejection(FailureClass::MissingTerminator, Table::Entry, super::EXIT_BLOCK, Some(super::EXIT_BLOCK),))
    );
}

// r[verify backend-trace.terminators]
// r[verify backend-trace.admission]
#[test]
fn operation_after_terminator_is_rejected() {
    let mut record = super::valid_record();
    record.entries.push(statement_entry(Operation::EmitStatement { inputs: Vec::new() }));
    assert_eq!(
        admit_record(&super::body(), &record),
        Err(rejection(
            FailureClass::OperationAfterTerminator,
            Table::Entry,
            APPENDED_ENTRY,
            Some(super::ENTRY_BLOCK),
        ))
    );
}

// r[verify backend-trace.admission]
// r[verify backend-trace.evidence]
#[test]
fn unsupported_operation_is_rejected() {
    let mut record = super::valid_record();
    record.entries.insert(FIRST_POSITION, statement_entry(Operation::Unsupported));
    assert_eq!(
        admit_record(&super::body(), &record),
        Err(rejection(FailureClass::UnsupportedOperation, Table::Entry, FIRST, None,))
    );
}

// r[verify backend-trace.ssa-like-locals]
// r[verify backend-trace.evidence]
#[test]
fn conflicting_local_class_is_rejected() {
    let mut record = super::valid_record();
    record.locals.push(LocalClass {
        local: super::RETURN_LOCAL,
        storage: LocalStorage::MemoryBacked,
    });
    assert_eq!(
        admit_record(&super::body(), &record),
        Err(rejection(FailureClass::ConflictingLocalClass, Table::Local, THIRD, Some(super::RETURN_LOCAL),))
    );
}

// r[verify backend-trace.ssa-like-locals]
// r[verify backend-trace.evidence]
#[test]
fn missing_local_class_is_rejected() {
    let mut record = super::valid_record();
    record.locals.retain(|class| class.local != super::TEMP_LOCAL);
    assert_eq!(
        admit_record(&super::body(), &record),
        Err(rejection(FailureClass::MissingLocalClass, Table::Local, super::TEMP_LOCAL, Some(super::TEMP_LOCAL),))
    );
}

// r[verify backend-trace.handle-validity]
#[test]
fn repeated_dangling_function_has_same_first_failure() {
    let mut record = super::valid_record();
    record.declarations.functions = Vec::new();
    let expected = Err(rejection(FailureClass::MissingDeclaration, Table::Entry, CALL_ENTRY, Some(super::FUNCTION)));
    assert_eq!(admit_record(&super::body(), &record), expected);
    assert_eq!(admit_record(&super::body(), &record), expected);
}
