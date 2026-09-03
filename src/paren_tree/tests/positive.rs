use super::super::*;

const NESTED_NODE_COUNT: u32 = 6;
const NESTED_DEPTH: u32 = 3;

fn nested_value() -> Value {
    super::list(vec![
        super::symbol("root"),
        super::list(vec![super::text("line\n"), super::symbol("child")]),
        super::list(Vec::new()),
    ])
}

// r[verify s-expression.ast]
// r[verify s-expression.atom-admission]
// r[verify s-expression.evidence.positive]
#[test]
fn symbol_and_text_atoms_are_admitted() {
    assert_eq!(admit_atom(&Atom::Symbol("alpha-1".to_owned())), Ok(()));
    assert_eq!(admit_atom(&Atom::Text(String::new())), Ok(()));
}

// r[verify s-expression.canonical-serialization]
// r[verify s-expression.evidence.positive]
#[test]
fn text_escapes_have_one_canonical_rendering() {
    let value = super::text("quote\"slash\\line\n\t");
    assert_eq!(serialize_canonical(&value, super::budget()), Ok("\"quote\\\"slash\\\\line\\n\\t\"".to_owned()));
}

// r[verify s-expression.ast]
// r[verify s-expression.canonical-serialization]
#[test]
fn empty_list_has_exact_delimiters() {
    assert_eq!(serialize_canonical(&super::list(Vec::new()), super::budget()), Ok("()".to_owned()));
}

// r[verify s-expression.ast]
// r[verify s-expression.budgets]
// r[verify s-expression.evidence.positive]
#[test]
fn nested_tree_has_checked_measures() {
    let measures = admit_value(&nested_value(), super::budget());
    assert!(matches!(
        measures,
        Ok(Measures {
            nodes: NESTED_NODE_COUNT,
            depth: NESTED_DEPTH,
            ..
        })
    ));
}

// r[verify s-expression.parse-admission]
// r[verify s-expression.canonical-serialization]
// r[verify s-expression.evidence.positive]
#[test]
fn canonical_nested_text_parses_to_original_tree() {
    let value = nested_value();
    let rendered = "(root (\"line\\n\" child) ())";
    assert_eq!(serialize_canonical(&value, super::budget()), Ok(rendered.to_owned()));
    assert_eq!(parse_canonical(rendered, super::budget()), Ok(value));
}

// r[verify s-expression.roundtrip]
// r[verify s-expression.evidence.positive]
// r[verify s-expression.docs]
// r[verify s-expression.final-validation]
#[test]
fn admitted_values_roundtrip_through_canonical_text() {
    let values = [
        super::symbol("alpha"),
        super::text("βeta"),
        super::list(Vec::new()),
        nested_value(),
    ];
    for value in values {
        assert_eq!(check_roundtrip(&value, super::budget()), Ok(()));
    }
}

// r[verify s-expression.parse-admission]
#[test]
fn quoted_unicode_text_is_canonical() {
    let rendered = "\"βeta\"";
    assert_eq!(parse_canonical(rendered, super::budget()), Ok(super::text("βeta")));
}

// r[verify s-expression.non-claims]
#[test]
fn public_result_is_a_tree_not_an_evaluation() {
    assert_eq!(
        parse_canonical("(add 1 2)", super::budget()),
        Ok(super::list(vec![super::symbol("add"), super::symbol("1"), super::symbol("2"),]))
    );
}
