//! Atom, tree, and finite-budget admission with checked measures.

const ESCAPED_BYTES: u32 = 2;

#[derive(Clone, Copy)]
struct MeasureFrame<'a> {
    value: &'a super::model::Value,
    depth: u32,
}

fn reject(class: super::model::FailureClass, offset: u32) -> super::model::Rejection {
    super::model::Rejection { class, offset }
}

fn width(len: usize, class: super::model::FailureClass) -> Result<u32, super::model::Rejection> {
    u32::try_from(len).map_err(|_| reject(class, 0))
}

fn add(left: u32, right: u32, class: super::model::FailureClass) -> Result<u32, super::model::Rejection> {
    left.checked_add(right).ok_or_else(|| reject(class, 0))
}

pub(super) fn budget_is_admitted(budget: super::model::Budget) -> bool {
    budget.max_nodes > 0
        && budget.max_nodes <= super::model::MAX_NODE_COUNT
        && budget.max_depth > 0
        && budget.max_depth <= super::model::MAX_NESTING_DEPTH
        && budget.max_bytes > 0
        && budget.max_bytes <= super::model::MAX_TEXT_BYTES
}

fn symbol_char_is_admitted(character: char) -> bool {
    character.is_ascii_alphanumeric()
        || matches!(character, '-' | '_' | '+' | '*' | '/' | '?' | '!' | '=' | '<' | '>' | '.')
}

fn text_char_is_admitted(character: char) -> bool {
    !character.is_control() || matches!(character, '\n' | '\r' | '\t')
}

/// Admits one decoded dialect-neutral atom.
// r[impl s-expression.atom-admission]
pub fn admit_atom(atom: &super::model::Atom) -> Result<(), super::model::Rejection> {
    match atom {
        super::model::Atom::Symbol(symbol) => {
            if symbol.is_empty() {
                return Err(reject(super::model::FailureClass::EmptyAtom, 0));
            }
            for (raw_offset, character) in symbol.char_indices() {
                if !symbol_char_is_admitted(character) {
                    return Err(reject(
                        super::model::FailureClass::ReservedAtom,
                        width(raw_offset, super::model::FailureClass::ByteBudget)?,
                    ));
                }
            }
            Ok(())
        }
        super::model::Atom::Text(text) => {
            for (raw_offset, character) in text.char_indices() {
                if !text_char_is_admitted(character) {
                    return Err(reject(
                        super::model::FailureClass::InvalidTextControl,
                        width(raw_offset, super::model::FailureClass::ByteBudget)?,
                    ));
                }
            }
            Ok(())
        }
    }
}

fn atom_bytes(atom: &super::model::Atom) -> Result<u32, super::model::Rejection> {
    match atom {
        super::model::Atom::Symbol(symbol) => width(symbol.len(), super::model::FailureClass::ByteBudget),
        super::model::Atom::Text(text) => {
            let mut bytes = super::proofs::LIST_DELIMITER_COUNT;
            for character in text.chars() {
                let amount = if matches!(character, '"' | '\\' | '\n' | '\r' | '\t') {
                    ESCAPED_BYTES
                } else {
                    width(character.len_utf8(), super::model::FailureClass::ByteBudget)?
                };
                bytes = add(bytes, amount, super::model::FailureClass::ByteBudget)?;
            }
            Ok(bytes)
        }
    }
}

fn list_bytes(children: usize) -> Result<u32, super::model::Rejection> {
    let separators = children.saturating_sub(1);
    add(
        super::proofs::LIST_DELIMITER_COUNT,
        width(separators, super::model::FailureClass::ByteBudget)?,
        super::model::FailureClass::ByteBudget,
    )
}

/// Admits one finite tree and returns checked node, depth, and byte measures.
// r[impl s-expression.ast]
// r[impl s-expression.budgets]
pub fn admit_value(
    value: &super::model::Value,
    budget: super::model::Budget,
) -> Result<super::model::Measures, super::model::Rejection> {
    if !budget_is_admitted(budget) {
        return Err(reject(super::model::FailureClass::InvalidBudget, 0));
    }
    let capacity = usize::try_from(budget.max_nodes).map_err(|_| reject(super::model::FailureClass::SizeBudget, 0))?;
    let mut stack = Vec::with_capacity(capacity);
    stack.push(MeasureFrame { value, depth: 1 });
    let mut nodes = 0_u32;
    let mut depth = 0_u32;
    let mut bytes = 0_u32;

    while let Some(frame) = stack.pop() {
        if frame.depth > budget.max_depth {
            return Err(reject(super::model::FailureClass::DepthBudget, frame.depth));
        }
        nodes = add(nodes, 1, super::model::FailureClass::SizeBudget)?;
        if nodes > budget.max_nodes {
            return Err(reject(super::model::FailureClass::SizeBudget, nodes));
        }
        depth = depth.max(frame.depth);
        match frame.value {
            super::model::Value::Atom(atom) => {
                admit_atom(atom)?;
                bytes = add(bytes, atom_bytes(atom)?, super::model::FailureClass::ByteBudget)?;
            }
            super::model::Value::List(children) => {
                bytes = add(bytes, list_bytes(children.len())?, super::model::FailureClass::ByteBudget)?;
                let child_depth = add(frame.depth, 1, super::model::FailureClass::DepthBudget)?;
                for child in children.iter().rev() {
                    stack.push(MeasureFrame {
                        value: child,
                        depth: child_depth,
                    });
                }
            }
        }
        if bytes > budget.max_bytes {
            return Err(reject(super::model::FailureClass::ByteBudget, bytes));
        }
    }
    Ok(super::model::Measures { nodes, depth, bytes })
}
