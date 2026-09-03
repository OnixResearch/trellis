//! Canonical delimiter, separator, symbol, and quoted-text rendering.

const RENDER_EVENTS_PER_NODE: u32 = 2;

struct Output {
    text: String,
}

impl Output {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            text: String::with_capacity(capacity),
        }
    }

    fn push(&mut self, character: char) {
        self.text.push(character);
    }

    fn push_str(&mut self, text: &str) {
        self.text.push_str(text);
    }
}

#[derive(Clone, Copy)]
enum Event<'a> {
    Value(&'a super::model::Value),
    Separator,
    Close,
}

fn reject(class: super::model::FailureClass, offset: u32) -> super::model::Rejection {
    super::model::Rejection { class, offset }
}

fn render_atom(atom: &super::model::Atom, output: &mut Output) {
    match atom {
        super::model::Atom::Symbol(symbol) => output.push_str(symbol),
        super::model::Atom::Text(text) => {
            output.push('"');
            for character in text.chars() {
                match character {
                    '"' => output.push_str("\\\""),
                    '\\' => output.push_str("\\\\"),
                    '\n' => output.push_str("\\n"),
                    '\r' => output.push_str("\\r"),
                    '\t' => output.push_str("\\t"),
                    other => output.push(other),
                }
            }
            output.push('"');
        }
    }
}

/// Renders the unique canonical text for one admitted bounded tree.
// r[impl s-expression.canonical-serialization]
pub fn serialize_canonical(
    value: &super::model::Value,
    budget: super::model::Budget,
) -> Result<String, super::model::Rejection> {
    let measures = super::admission::admit_value(value, budget)?;
    let output_capacity =
        usize::try_from(measures.bytes).map_err(|_| reject(super::model::FailureClass::ByteBudget, measures.bytes))?;
    let event_count = budget
        .max_nodes
        .checked_mul(RENDER_EVENTS_PER_NODE)
        .and_then(|count| count.checked_add(1))
        .ok_or_else(|| reject(super::model::FailureClass::SizeBudget, budget.max_nodes))?;
    let event_capacity =
        usize::try_from(event_count).map_err(|_| reject(super::model::FailureClass::SizeBudget, event_count))?;
    let mut output = Output::with_capacity(output_capacity);
    let mut events = Vec::with_capacity(event_capacity);
    events.push(Event::Value(value));

    while let Some(event) = events.pop() {
        match event {
            Event::Value(super::model::Value::Atom(atom)) => render_atom(atom, &mut output),
            Event::Value(super::model::Value::List(children)) => {
                output.push('(');
                events.push(Event::Close);
                for (reverse_index, child) in children.iter().rev().enumerate() {
                    events.push(Event::Value(child));
                    if reverse_index.saturating_add(1) < children.len() {
                        events.push(Event::Separator);
                    }
                }
            }
            Event::Separator => output.push(' '),
            Event::Close => output.push(')'),
        }
    }

    let output_bytes =
        u32::try_from(output.text.len()).map_err(|_| reject(super::model::FailureClass::ByteBudget, measures.bytes))?;
    if output_bytes != measures.bytes {
        return Err(reject(super::model::FailureClass::ByteBudget, output_bytes));
    }
    Ok(output.text)
}
