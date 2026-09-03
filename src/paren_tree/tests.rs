//! Positive and negative canonical-tree evidence.

mod negative;
mod positive;

const TEST_MAX_NODES: u32 = 16;
const TEST_MAX_DEPTH: u32 = 8;
const TEST_MAX_BYTES: u32 = 256;

fn budget() -> super::model::Budget {
    super::model::Budget {
        max_nodes: TEST_MAX_NODES,
        max_depth: TEST_MAX_DEPTH,
        max_bytes: TEST_MAX_BYTES,
    }
}

fn symbol(text: &str) -> super::model::Value {
    super::model::Value::Atom(super::model::Atom::Symbol(text.to_owned()))
}

fn text(value: &str) -> super::model::Value {
    super::model::Value::Atom(super::model::Atom::Text(value.to_owned()))
}

fn list(children: Vec<super::model::Value>) -> super::model::Value {
    super::model::Value::List(children)
}
