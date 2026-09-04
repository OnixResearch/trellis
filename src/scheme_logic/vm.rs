//! Constant-pool and bytecode-shape admission without execution.

const CODE_ITEM_LIMIT: u32 = 4_096;
const STACK_LIMIT: u32 = 65_536;

/// Finite constant kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ConstantValue {
    Integer(i64),
    Boolean(bool),
    Symbol(super::registry::InternKey),
    EmptyList,
}

/// Deduplicated finite constant pool.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstantPool {
    pub values: Vec<ConstantValue>,
}

/// Add-or-get result with explicit new pool state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PoolResult {
    pub index: u32,
    pub pool: ConstantPool,
    pub was_inserted: bool,
}

/// Supported instruction summaries.
// r[impl scheme-logic.bytecode-safety]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    PushConstant(u32),
    LoadLocal(u32),
    LoadCapture(u32),
    Pop,
    Call { argument_count: u32 },
    Jump(u32),
    Noop,
}

/// Complete finite bytecode-admission envelope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeEnvelope {
    pub pool: ConstantPool,
    pub instructions: Vec<Instruction>,
    pub local_count: u32,
    pub capture_count: u32,
    pub initial_stack_height: u32,
    pub expected_final_stack_height: u32,
}

/// Successful stack-effect summary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodeSummary {
    pub instruction_count: u32,
    pub final_stack_height: u32,
}

/// Stable pool and bytecode rejection classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeFault {
    TooManyConstants,
    TooManyInstructions,
    DuplicateConstant,
    ConstantPoolFull,
    RollbackOutsidePool,
    LocalCountTooLarge,
    CaptureCountTooLarge,
    StackHeightTooLarge,
    ConstantIndexOutsidePool,
    LocalIndexOutsideFrame,
    CaptureIndexOutsideClosure,
    JumpTargetOutsideCode,
    CallArityTooLarge,
    StackUnderflow,
    FinalStackMismatch,
}

/// Deterministic pool or bytecode rejection metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodeFaultSite {
    pub class: CodeFault,
    pub index: Option<u32>,
}

fn position(index: usize) -> Option<u32> {
    u32::try_from(index).ok()
}

fn reject<T>(class: CodeFault, index: Option<u32>) -> Result<T, CodeFaultSite> {
    Err(CodeFaultSite { class, index })
}

/// Admits only bounded, deduplicated constants.
// r[impl scheme-logic.bytecode-safety]
pub fn admit_pool(pool: &ConstantPool) -> Result<(), CodeFaultSite> {
    let count = u32::try_from(pool.values.len()).map_err(|_| CodeFaultSite {
        class: CodeFault::TooManyConstants,
        index: None,
    })?;
    if count > CODE_ITEM_LIMIT {
        return reject(CodeFault::TooManyConstants, None);
    }
    for (index, value) in pool.values.iter().enumerate() {
        if pool.values[..index].contains(value) {
            return reject(CodeFault::DuplicateConstant, position(index));
        }
    }
    Ok(())
}

/// Returns an existing index or appends one unique constant.
// r[impl scheme-logic.bytecode-safety]
pub fn add_or_get_constant(pool: &ConstantPool, value: ConstantValue) -> Result<PoolResult, CodeFaultSite> {
    admit_pool(pool)?;
    if let Some(index) = pool.values.iter().position(|candidate| *candidate == value) {
        let index = u32::try_from(index).map_err(|_| CodeFaultSite {
            class: CodeFault::ConstantPoolFull,
            index: None,
        })?;
        return Ok(PoolResult {
            index,
            pool: pool.clone(),
            was_inserted: false,
        });
    }
    let index = u32::try_from(pool.values.len()).map_err(|_| CodeFaultSite {
        class: CodeFault::ConstantPoolFull,
        index: None,
    })?;
    if index >= CODE_ITEM_LIMIT {
        return reject(CodeFault::ConstantPoolFull, None);
    }
    let mut values = Vec::with_capacity(pool.values.len() + 1);
    values.extend(pool.values.iter().copied());
    values.push(value);
    Ok(PoolResult {
        index,
        pool: ConstantPool { values },
        was_inserted: true,
    })
}

/// Returns the exact retained prefix at one admitted rollback index.
// r[impl scheme-logic.bytecode-safety]
pub fn rollback_pool(pool: &ConstantPool, retained_count: u32) -> Result<ConstantPool, CodeFaultSite> {
    admit_pool(pool)?;
    let retained = usize::try_from(retained_count).map_err(|_| CodeFaultSite {
        class: CodeFault::RollbackOutsidePool,
        index: None,
    })?;
    if retained > pool.values.len() {
        return reject(CodeFault::RollbackOutsidePool, None);
    }
    Ok(ConstantPool {
        values: pool.values[..retained].to_vec(),
    })
}

fn increment_stack(height: u32, at: Option<u32>) -> Result<u32, CodeFaultSite> {
    let next = height.checked_add(1).ok_or(CodeFaultSite {
        class: CodeFault::StackHeightTooLarge,
        index: at,
    })?;
    if next > STACK_LIMIT {
        return reject(CodeFault::StackHeightTooLarge, at);
    }
    Ok(next)
}

#[derive(Clone, Copy)]
struct CodeBounds {
    instructions: u32,
    constants: u32,
    locals: u32,
    captures: u32,
}

fn code_bounds(envelope: &CodeEnvelope) -> Result<CodeBounds, CodeFaultSite> {
    admit_pool(&envelope.pool)?;
    let instruction_count = u32::try_from(envelope.instructions.len()).map_err(|_| CodeFaultSite {
        class: CodeFault::TooManyInstructions,
        index: None,
    })?;
    if instruction_count > CODE_ITEM_LIMIT {
        return reject(CodeFault::TooManyInstructions, None);
    }
    if envelope.local_count > CODE_ITEM_LIMIT {
        return reject(CodeFault::LocalCountTooLarge, None);
    }
    if envelope.capture_count > CODE_ITEM_LIMIT {
        return reject(CodeFault::CaptureCountTooLarge, None);
    }
    if envelope.initial_stack_height > STACK_LIMIT || envelope.expected_final_stack_height > STACK_LIMIT {
        return reject(CodeFault::StackHeightTooLarge, None);
    }
    let constant_count = u32::try_from(envelope.pool.values.len()).map_err(|_| CodeFaultSite {
        class: CodeFault::TooManyConstants,
        index: None,
    })?;
    Ok(CodeBounds {
        instructions: instruction_count,
        constants: constant_count,
        locals: envelope.local_count,
        captures: envelope.capture_count,
    })
}

fn apply_instruction(
    bounds: CodeBounds,
    instruction: Instruction,
    stack_height: u32,
    at: Option<u32>,
) -> Result<u32, CodeFaultSite> {
    match instruction {
        Instruction::PushConstant(constant) => {
            if constant >= bounds.constants {
                return reject(CodeFault::ConstantIndexOutsidePool, at);
            }
            increment_stack(stack_height, at)
        }
        Instruction::LoadLocal(local) => {
            if local >= bounds.locals {
                return reject(CodeFault::LocalIndexOutsideFrame, at);
            }
            increment_stack(stack_height, at)
        }
        Instruction::LoadCapture(capture) => {
            if capture >= bounds.captures {
                return reject(CodeFault::CaptureIndexOutsideClosure, at);
            }
            increment_stack(stack_height, at)
        }
        Instruction::Pop => stack_height.checked_sub(1).ok_or(CodeFaultSite {
            class: CodeFault::StackUnderflow,
            index: at,
        }),
        Instruction::Call { argument_count } => {
            if argument_count > CODE_ITEM_LIMIT {
                return reject(CodeFault::CallArityTooLarge, at);
            }
            let required = argument_count.checked_add(1).ok_or(CodeFaultSite {
                class: CodeFault::CallArityTooLarge,
                index: at,
            })?;
            if stack_height < required {
                return reject(CodeFault::StackUnderflow, at);
            }
            Ok(stack_height - argument_count)
        }
        Instruction::Jump(target) => {
            if target >= bounds.instructions {
                return reject(CodeFault::JumpTargetOutsideCode, at);
            }
            Ok(stack_height)
        }
        Instruction::Noop => Ok(stack_height),
    }
}

/// Admits operand bounds and one exact stack-effect summary.
// r[impl scheme-logic.bytecode-safety]
pub fn admit_code(envelope: &CodeEnvelope) -> Result<CodeSummary, CodeFaultSite> {
    let bounds = code_bounds(envelope)?;
    let mut stack_height = envelope.initial_stack_height;
    for (index, instruction) in envelope.instructions.iter().enumerate() {
        stack_height = apply_instruction(bounds, *instruction, stack_height, position(index))?;
    }
    if stack_height != envelope.expected_final_stack_height {
        return reject(CodeFault::FinalStackMismatch, None);
    }
    Ok(CodeSummary {
        instruction_count: bounds.instructions,
        final_stack_height: stack_height,
    })
}
