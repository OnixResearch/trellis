use super::super::*;

const SIZE_LIMIT: u32 = 2;
const DEPTH_LIMIT: u32 = 2;
const BYTE_LIMIT: u32 = 4;

fn parse_class(input: &str, budget: Budget) -> Option<FailureClass> {
    match parse_canonical(input, budget) {
        Ok(_) => None,
        Err(error) => Some(error.class),
    }
}

// r[verify s-expression.atom-admission]
// r[verify s-expression.evidence.negative]
#[test]
fn empty_symbol_is_rejected() {
    assert_eq!(admit_atom(&Atom::Symbol(String::new())).map_err(|error| error.class), Err(FailureClass::EmptyAtom));
}

// r[verify s-expression.atom-admission]
// r[verify s-expression.evidence.negative]
#[test]
fn reserved_symbol_character_is_rejected() {
    assert_eq!(
        admit_atom(&Atom::Symbol("a b".to_owned())).map_err(|error| error.class),
        Err(FailureClass::ReservedAtom)
    );
}

// r[verify s-expression.atom-admission]
// r[verify s-expression.evidence.negative]
#[test]
fn unsupported_text_control_is_rejected() {
    assert_eq!(
        admit_atom(&Atom::Text("a\0b".to_owned())).map_err(|error| error.class),
        Err(FailureClass::InvalidTextControl)
    );
}

// r[verify s-expression.parse-admission]
// r[verify s-expression.evidence.negative]
#[test]
fn unknown_escape_is_rejected() {
    assert_eq!(parse_class("\"bad\\q\"", super::budget()), Some(FailureClass::InvalidEscape));
}

// r[verify s-expression.parse-admission]
// r[verify s-expression.evidence.negative]
#[test]
fn unterminated_text_escape_is_rejected() {
    assert_eq!(parse_class("\"bad\\", super::budget()), Some(FailureClass::InvalidEscape));
}

// r[verify s-expression.parse-admission]
// r[verify s-expression.evidence.negative]
#[test]
fn unbalanced_open_delimiter_is_rejected() {
    assert_eq!(parse_class("(alpha", super::budget()), Some(FailureClass::UnbalancedDelimiter));
}

// r[verify s-expression.parse-admission]
// r[verify s-expression.evidence.negative]
#[test]
fn unmatched_close_delimiter_is_rejected() {
    assert_eq!(parse_class(")", super::budget()), Some(FailureClass::UnbalancedDelimiter));
}

// r[verify s-expression.parse-admission]
// r[verify s-expression.evidence.negative]
#[test]
fn trailing_input_is_rejected() {
    assert_eq!(parse_class("alpha beta", super::budget()), Some(FailureClass::TrailingInput));
}

// r[verify s-expression.canonical-serialization]
// r[verify s-expression.evidence.negative]
#[test]
fn repeated_or_leading_spaces_are_rejected() {
    assert_eq!(parse_class("( alpha)", super::budget()), Some(FailureClass::NonCanonical));
    assert_eq!(parse_class("(alpha  beta)", super::budget()), Some(FailureClass::NonCanonical));
}

// r[verify s-expression.parse-admission]
// r[verify s-expression.evidence.negative]
#[test]
fn missing_child_separator_is_rejected() {
    assert_eq!(parse_class("(alpha(beta))", super::budget()), Some(FailureClass::NonCanonical));
}

// r[verify s-expression.budgets]
// r[verify s-expression.evidence.negative]
#[test]
fn size_budget_exhaustion_is_rejected() {
    let budget = Budget {
        max_nodes: SIZE_LIMIT,
        ..super::budget()
    };
    assert_eq!(parse_class("(a b)", budget), Some(FailureClass::SizeBudget));
}

// r[verify s-expression.budgets]
// r[verify s-expression.evidence.negative]
#[test]
fn depth_budget_exhaustion_is_rejected() {
    let budget = Budget {
        max_depth: DEPTH_LIMIT,
        ..super::budget()
    };
    assert_eq!(parse_class("((a))", budget), Some(FailureClass::DepthBudget));
}

// r[verify s-expression.budgets]
// r[verify s-expression.evidence.negative]
#[test]
fn byte_budget_exhaustion_is_rejected() {
    let budget = Budget {
        max_bytes: BYTE_LIMIT,
        ..super::budget()
    };
    assert_eq!(parse_class("alpha", budget), Some(FailureClass::ByteBudget));
}

// r[verify s-expression.budgets]
#[test]
fn zero_budget_is_rejected() {
    let budget = Budget {
        max_nodes: 0,
        max_depth: 0,
        max_bytes: 0,
    };
    assert_eq!(parse_class("a", budget), Some(FailureClass::InvalidBudget));
}

// r[verify s-expression.parse-admission]
#[test]
fn first_noncanonical_failure_is_deterministic() {
    let first = parse_canonical("(a  b)", super::budget());
    let second = parse_canonical("(a  b)", super::budget());
    assert_eq!(first, second);
    assert!(matches!(
        first,
        Err(Rejection {
            class: FailureClass::NonCanonical,
            ..
        })
    ));
}
