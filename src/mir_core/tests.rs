//! Positive and negative MIR-core admission evidence.

mod negative;
mod positive;

fn local(role: super::model::LocalRole, type_tag: super::model::TypeTag) -> super::model::LocalDecl {
    super::model::LocalDecl { role, type_tag }
}

fn return_local() -> super::model::LocalDecl {
    local(super::model::LocalRole::Return, super::model::TypeTag::Unit)
}

fn place(local: u32) -> super::model::Place {
    super::model::Place {
        local,
        projections: Vec::new(),
    }
}

fn constant(bits: u64) -> super::model::Operand {
    super::model::Operand::Constant(super::model::Constant {
        type_tag: super::model::TypeTag::U64,
        bits,
    })
}

fn return_block() -> super::model::BasicBlock {
    super::model::BasicBlock {
        statements: Vec::new(),
        terminator: Some(super::model::Terminator::Return),
    }
}

fn return_body() -> super::model::Body {
    super::model::Body {
        locals: vec![return_local()],
        blocks: vec![return_block()],
    }
}
