//! Positive and negative builder-record evidence.

mod negative;
mod positive;

const ENTRY_BLOCK: u32 = 0;
const NEXT_BLOCK: u32 = 1;
const EXIT_BLOCK: u32 = 2;
const BLOCK_COUNT: u32 = 3;
const RETURN_LOCAL: u32 = 0;
const TEMP_LOCAL: u32 = 1;
const LOCAL_COUNT: u32 = 2;
const DIRECT_ENTRY: u32 = 10;
const EXPANDED_ENTRY: u32 = 11;
const DIRECT_NEXT: u32 = 20;
const DIRECT_EXIT: u32 = 30;
const UNKNOWN_BLOCK: u32 = 99;
const FIRST_VALUE: u32 = 100;
const SECOND_VALUE: u32 = 101;
const CALL_VALUE: u32 = 102;
const UNKNOWN_VALUE: u32 = 199;
const FUNCTION: u32 = 200;
const TYPE: u32 = 300;
const PLACE: u32 = 400;
const OPERATION_COUNT: u32 = 7;
const VALUE_COUNT: u32 = 3;

fn place(local: u32) -> crate::mir_core::Place {
    crate::mir_core::Place {
        local,
        projections: Vec::new(),
    }
}

fn constant() -> crate::mir_core::Operand {
    crate::mir_core::Operand::Constant(crate::mir_core::Constant {
        type_tag: crate::mir_core::TypeTag::U64,
        bits: 1,
    })
}

fn body() -> crate::mir_core::Body {
    crate::mir_core::Body {
        locals: vec![
            crate::mir_core::LocalDecl {
                role: crate::mir_core::LocalRole::Return,
                type_tag: crate::mir_core::TypeTag::Unit,
            },
            crate::mir_core::LocalDecl {
                role: crate::mir_core::LocalRole::Temporary,
                type_tag: crate::mir_core::TypeTag::U64,
            },
        ],
        blocks: vec![
            crate::mir_core::BasicBlock {
                statements: vec![crate::mir_core::Statement::Assign {
                    destination: place(RETURN_LOCAL),
                    value: crate::mir_core::Rvalue::Binary {
                        operator: crate::mir_core::BinaryOperator::Add,
                        left: crate::mir_core::Operand::Copy(place(TEMP_LOCAL)),
                        right: constant(),
                    },
                }],
                terminator: Some(crate::mir_core::Terminator::Call {
                    function: constant(),
                    arguments: vec![crate::mir_core::Operand::Copy(place(TEMP_LOCAL))],
                    destination: Some((place(RETURN_LOCAL), NEXT_BLOCK)),
                }),
            },
            crate::mir_core::BasicBlock {
                statements: Vec::new(),
                terminator: Some(crate::mir_core::Terminator::Goto { target: EXIT_BLOCK }),
            },
            crate::mir_core::BasicBlock {
                statements: Vec::new(),
                terminator: Some(crate::mir_core::Terminator::Return),
            },
        ],
    }
}

fn statement_owner() -> crate::mir_visitor_traversal::Owner {
    crate::mir_visitor_traversal::Owner::Statement {
        block: ENTRY_BLOCK,
        statement: 0,
    }
}

fn declarations() -> super::model::Declarations {
    super::model::Declarations {
        blocks: vec![
            super::model::BlockKey(DIRECT_ENTRY),
            super::model::BlockKey(EXPANDED_ENTRY),
            super::model::BlockKey(DIRECT_NEXT),
            super::model::BlockKey(DIRECT_EXIT),
        ],
        functions: vec![super::model::FunctionKey(FUNCTION)],
        types: vec![super::model::TypeKey(TYPE)],
        places: vec![super::model::PlaceKey(PLACE)],
    }
}

fn mappings() -> Vec<super::model::BlockMapping> {
    vec![
        super::model::BlockMapping {
            source_block: ENTRY_BLOCK,
            direct: super::model::BlockKey(DIRECT_ENTRY),
            expansions: vec![super::model::BlockKey(EXPANDED_ENTRY)],
        },
        super::model::BlockMapping {
            source_block: NEXT_BLOCK,
            direct: super::model::BlockKey(DIRECT_NEXT),
            expansions: Vec::new(),
        },
        super::model::BlockMapping {
            source_block: EXIT_BLOCK,
            direct: super::model::BlockKey(DIRECT_EXIT),
            expansions: Vec::new(),
        },
    ]
}

fn local_classes() -> Vec<super::model::LocalClass> {
    vec![
        super::model::LocalClass {
            local: RETURN_LOCAL,
            storage: super::model::LocalStorage::DirectValue,
        },
        super::model::LocalClass {
            local: TEMP_LOCAL,
            storage: super::model::LocalStorage::MemoryBacked,
        },
    ]
}

fn entries() -> Vec<super::model::Entry> {
    vec![
        super::model::Entry {
            source: crate::mir_visitor_traversal::Location::Operand {
                owner: statement_owner(),
                ordinal: 0,
            },
            emission_block: super::model::BlockKey(DIRECT_ENTRY),
            operation: super::model::Operation::EmitOperand {
                result: super::model::ValueKey(FIRST_VALUE),
                type_key: super::model::TypeKey(TYPE),
            },
        },
        super::model::Entry {
            source: crate::mir_visitor_traversal::Location::Place {
                owner: statement_owner(),
                ordinal: 0,
            },
            emission_block: super::model::BlockKey(DIRECT_ENTRY),
            operation: super::model::Operation::EmitPlace {
                place: super::model::PlaceKey(PLACE),
            },
        },
        super::model::Entry {
            source: crate::mir_visitor_traversal::Location::Rvalue {
                block: ENTRY_BLOCK,
                statement: 0,
            },
            emission_block: super::model::BlockKey(DIRECT_ENTRY),
            operation: super::model::Operation::EmitRvalue {
                inputs: vec![super::model::ValueKey(FIRST_VALUE)],
                result: super::model::ValueKey(SECOND_VALUE),
                type_key: super::model::TypeKey(TYPE),
            },
        },
        super::model::Entry {
            source: crate::mir_visitor_traversal::Location::Statement {
                block: ENTRY_BLOCK,
                statement: 0,
            },
            emission_block: super::model::BlockKey(DIRECT_ENTRY),
            operation: super::model::Operation::EmitStatement {
                inputs: vec![super::model::ValueKey(SECOND_VALUE)],
            },
        },
        super::model::Entry {
            source: crate::mir_visitor_traversal::Location::Terminator { block: ENTRY_BLOCK },
            emission_block: super::model::BlockKey(EXPANDED_ENTRY),
            operation: super::model::Operation::Call {
                function: super::model::FunctionKey(FUNCTION),
                arguments: vec![super::model::ValueKey(SECOND_VALUE)],
                result: Some((super::model::ValueKey(CALL_VALUE), super::model::TypeKey(TYPE))),
                return_block: super::model::BlockKey(DIRECT_NEXT),
                unwind_block: Some(super::model::BlockKey(EXPANDED_ENTRY)),
            },
        },
        super::model::Entry {
            source: crate::mir_visitor_traversal::Location::Terminator { block: NEXT_BLOCK },
            emission_block: super::model::BlockKey(DIRECT_NEXT),
            operation: super::model::Operation::Branch {
                targets: vec![super::model::BlockKey(DIRECT_EXIT)],
            },
        },
        super::model::Entry {
            source: crate::mir_visitor_traversal::Location::Terminator { block: EXIT_BLOCK },
            emission_block: super::model::BlockKey(DIRECT_EXIT),
            operation: super::model::Operation::Return {
                value: Some(super::model::ValueKey(CALL_VALUE)),
            },
        },
    ]
}

fn valid_record() -> super::model::Record {
    super::model::Record {
        declarations: declarations(),
        mappings: mappings(),
        locals: local_classes(),
        entries: entries(),
    }
}
