//! Positive and negative visitor-traversal evidence.

mod negative;
mod positive;

const ENTRY_BLOCK: u32 = 0;
const LEFT_BLOCK: u32 = 1;
const RIGHT_BLOCK: u32 = 2;
const EXIT_BLOCK: u32 = 3;
const UNREACHABLE_BLOCK: u32 = 4;
const RETURN_LOCAL: u32 = 0;
const TEMP_LOCAL: u32 = 1;
const BODY_BLOCK_COUNT: u32 = 5;
const BODY_LOCAL_COUNT: u32 = 2;
const BODY_STATEMENT_COUNT: u32 = 1;
const BODY_TERMINATOR_COUNT: u32 = 5;
const BODY_SUCCESSOR_COUNT: u32 = 4;
const FIRST_BITS: u64 = 7;
const SECOND_BITS: u64 = 9;

fn local(role: crate::mir_core::LocalRole, type_tag: crate::mir_core::TypeTag) -> crate::mir_core::LocalDecl {
    crate::mir_core::LocalDecl { role, type_tag }
}

fn place(local: u32) -> crate::mir_core::Place {
    crate::mir_core::Place {
        local,
        projections: Vec::new(),
    }
}

fn constant(bits: u64) -> crate::mir_core::Operand {
    crate::mir_core::Operand::Constant(crate::mir_core::Constant {
        type_tag: crate::mir_core::TypeTag::U64,
        bits,
    })
}

fn return_body() -> crate::mir_core::Body {
    crate::mir_core::Body {
        locals: vec![local(
            crate::mir_core::LocalRole::Return,
            crate::mir_core::TypeTag::Unit,
        )],
        blocks: vec![crate::mir_core::BasicBlock {
            statements: Vec::new(),
            terminator: Some(crate::mir_core::Terminator::Return),
        }],
    }
}

fn branch_body(bits: u64) -> crate::mir_core::Body {
    crate::mir_core::Body {
        locals: vec![
            local(crate::mir_core::LocalRole::Return, crate::mir_core::TypeTag::Unit),
            local(crate::mir_core::LocalRole::Temporary, crate::mir_core::TypeTag::U64),
        ],
        blocks: vec![
            crate::mir_core::BasicBlock {
                statements: vec![crate::mir_core::Statement::Assign {
                    destination: place(RETURN_LOCAL),
                    value: crate::mir_core::Rvalue::Binary {
                        operator: crate::mir_core::BinaryOperator::Add,
                        left: crate::mir_core::Operand::Copy(place(TEMP_LOCAL)),
                        right: constant(bits),
                    },
                }],
                terminator: Some(crate::mir_core::Terminator::Switch {
                    discriminant: constant(bits),
                    targets: vec![LEFT_BLOCK, RIGHT_BLOCK],
                }),
            },
            crate::mir_core::BasicBlock {
                statements: Vec::new(),
                terminator: Some(crate::mir_core::Terminator::Call {
                    function: constant(bits),
                    arguments: vec![crate::mir_core::Operand::Copy(place(TEMP_LOCAL))],
                    destination: Some((place(RETURN_LOCAL), EXIT_BLOCK)),
                }),
            },
            crate::mir_core::BasicBlock {
                statements: Vec::new(),
                terminator: Some(crate::mir_core::Terminator::Drop {
                    place: place(TEMP_LOCAL),
                    target: EXIT_BLOCK,
                }),
            },
            crate::mir_core::BasicBlock {
                statements: Vec::new(),
                terminator: Some(crate::mir_core::Terminator::Return),
            },
            crate::mir_core::BasicBlock {
                statements: Vec::new(),
                terminator: Some(crate::mir_core::Terminator::Unreachable),
            },
        ],
    }
}

fn entry(location: super::model::Location) -> super::model::Entry {
    super::model::Entry { location }
}

fn immutable_trace(
    entries: Vec<super::model::Entry>,
    order: super::model::Order,
    reachability: super::model::ReachabilityPolicy,
) -> super::model::Trace {
    super::model::Trace {
        entries,
        entry: ENTRY_BLOCK,
        coverage: super::model::CoverageMode::ExactlyOnce,
        order,
        reachability,
        mutation: super::model::MutationClass::Immutable,
    }
}

fn branch_shape() -> super::model::Shape {
    super::model::Shape {
        locals: BODY_LOCAL_COUNT,
        blocks: BODY_BLOCK_COUNT,
        statements: BODY_STATEMENT_COUNT,
        terminators: BODY_TERMINATOR_COUNT,
        successors: BODY_SUCCESSOR_COUNT,
    }
}
