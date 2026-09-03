//! Positive and negative finite analysis evidence.

mod negative;
mod positive;

const BOTTOM: u32 = 0;
const MIDDLE: u32 = 1;
const TOP: u32 = 2;
const STATE_COUNT: u32 = 3;
const JOIN_TABLE_LEN: usize = 9;
const ENTRY_BLOCK: u32 = 0;
const MIDDLE_BLOCK: u32 = 1;
const EXIT_BLOCK: u32 = 2;
const BLOCK_COUNT: u32 = 3;
const EFFECT_COUNT: u32 = 2;
const EDGE_EFFECT_COUNT: u32 = 1;
const REQUIRED_BUDGET: u32 = 19;
const TOO_LARGE_BUDGET: u32 = 1_000_001;
const INVALID_INDEX: u32 = 9;

fn state(raw: u32) -> super::model::StateKey {
    super::model::StateKey(raw)
}

fn transfer(outputs: &[u32]) -> super::model::Transfer {
    super::model::Transfer {
        outputs: outputs.iter().copied().map(state).collect(),
    }
}

fn identity_transfer() -> super::model::Transfer {
    transfer(&[BOTTOM, MIDDLE, TOP])
}

fn raise_one() -> super::model::Transfer {
    transfer(&[MIDDLE, MIDDLE, TOP])
}

fn raise_middle() -> super::model::Transfer {
    transfer(&[BOTTOM, TOP, TOP])
}

fn domain() -> super::model::Domain {
    super::model::Domain {
        state_count: STATE_COUNT,
        row_width: STATE_COUNT,
        bottom: Some(state(BOTTOM)),
        top: Some(state(TOP)),
        height: Some(STATE_COUNT),
        join: vec![
            state(BOTTOM),
            state(MIDDLE),
            state(TOP),
            state(MIDDLE),
            state(MIDDLE),
            state(TOP),
            state(TOP),
            state(TOP),
            state(TOP),
        ],
    }
}

fn place(local: u32) -> crate::mir_core::Place {
    crate::mir_core::Place {
        local,
        projections: Vec::new(),
    }
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
                    destination: place(0),
                    value: crate::mir_core::Rvalue::Use(crate::mir_core::Operand::Copy(place(1))),
                }],
                terminator: Some(crate::mir_core::Terminator::Goto { target: MIDDLE_BLOCK }),
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

fn statement_location() -> crate::mir_visitor_traversal::Location {
    crate::mir_visitor_traversal::Location::Statement {
        block: ENTRY_BLOCK,
        statement: 0,
    }
}

fn terminator_location() -> crate::mir_visitor_traversal::Location {
    crate::mir_visitor_traversal::Location::Terminator { block: ENTRY_BLOCK }
}

fn analysis(direction: super::model::Direction) -> super::model::Analysis {
    super::model::Analysis {
        domain: domain(),
        direction,
        boundary_block: ENTRY_BLOCK,
        boundary_state: state(BOTTOM),
        effects: vec![
            super::model::LocationEffect {
                location: statement_location(),
                before: identity_transfer(),
                primary: raise_one(),
            },
            super::model::LocationEffect {
                location: terminator_location(),
                before: identity_transfer(),
                primary: raise_middle(),
            },
        ],
        edge_effects: vec![super::model::EdgeEffect {
            source: MIDDLE_BLOCK,
            target: EXIT_BLOCK,
            transfer: identity_transfer(),
        }],
        convergence_budget: REQUIRED_BUDGET,
    }
}
