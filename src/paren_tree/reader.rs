//! Bounded canonical grammar parser with deterministic byte offsets.

const OPEN: u8 = b'(';
const CLOSE: u8 = b')';
const QUOTE: u8 = b'"';
const ESCAPE: u8 = b'\\';
const SPACE: u8 = b' ';
const DELETE: u8 = 0x7f;

struct Parser<'a> {
    input: &'a str,
    bytes: &'a [u8],
    cursor: usize,
    nodes: u32,
    budget: super::model::Budget,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str, budget: super::model::Budget) -> Self {
        Self {
            input,
            bytes: input.as_bytes(),
            cursor: 0,
            nodes: 0,
            budget,
        }
    }

    fn offset(&self) -> u32 {
        u32::try_from(self.cursor).unwrap_or(super::model::MAX_TEXT_BYTES)
    }

    fn reject(&self, class: super::model::FailureClass) -> super::model::Rejection {
        super::model::Rejection {
            class,
            offset: self.offset(),
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.cursor).copied()
    }

    fn advance(&mut self, amount: usize) -> Result<(), super::model::Rejection> {
        self.cursor =
            self.cursor.checked_add(amount).ok_or_else(|| self.reject(super::model::FailureClass::ByteBudget))?;
        Ok(())
    }

    fn admit_node(&mut self, depth: u32) -> Result<(), super::model::Rejection> {
        if depth > self.budget.max_depth {
            return Err(self.reject(super::model::FailureClass::DepthBudget));
        }
        self.nodes = self.nodes.checked_add(1).ok_or_else(|| self.reject(super::model::FailureClass::SizeBudget))?;
        if self.nodes > self.budget.max_nodes {
            return Err(self.reject(super::model::FailureClass::SizeBudget));
        }
        Ok(())
    }

    fn parse_value(&mut self, depth: u32) -> Result<super::model::Value, super::model::Rejection> {
        self.admit_node(depth)?;
        match self.peek() {
            Some(OPEN) => self.parse_list(depth),
            Some(QUOTE) => self.parse_text(),
            Some(CLOSE) => Err(self.reject(super::model::FailureClass::UnbalancedDelimiter)),
            Some(SPACE) => Err(self.reject(super::model::FailureClass::NonCanonical)),
            Some(_) => self.parse_symbol(),
            None => Err(self.reject(super::model::FailureClass::EmptyAtom)),
        }
    }

    fn parse_symbol(&mut self) -> Result<super::model::Value, super::model::Rejection> {
        let start = self.cursor;
        while let Some(byte) = self.peek() {
            if matches!(byte, OPEN | CLOSE | SPACE) {
                break;
            }
            self.advance(1)?;
        }
        let symbol = self
            .input
            .get(start..self.cursor)
            .ok_or_else(|| self.reject(super::model::FailureClass::ReservedAtom))?
            .to_owned();
        let atom = super::model::Atom::Symbol(symbol);
        super::admission::admit_atom(&atom).map_err(|error| super::model::Rejection {
            class: error.class,
            offset: u32::try_from(start).unwrap_or(super::model::MAX_TEXT_BYTES).saturating_add(error.offset),
        })?;
        Ok(super::model::Value::Atom(atom))
    }

    fn parse_escape(&mut self) -> Result<char, super::model::Rejection> {
        let escaped = self.peek().ok_or_else(|| self.reject(super::model::FailureClass::InvalidEscape))?;
        self.advance(1)?;
        match escaped {
            QUOTE => Ok('"'),
            ESCAPE => Ok('\\'),
            b'n' => Ok('\n'),
            b'r' => Ok('\r'),
            b't' => Ok('\t'),
            _ => Err(self.reject(super::model::FailureClass::InvalidEscape)),
        }
    }

    fn parse_text(&mut self) -> Result<super::model::Value, super::model::Rejection> {
        self.advance(1)?;
        let remaining = self.bytes.len().saturating_sub(self.cursor);
        let mut output = String::with_capacity(remaining);
        loop {
            match self.peek() {
                Some(QUOTE) => {
                    self.advance(1)?;
                    return Ok(super::model::Value::Atom(super::model::Atom::Text(output)));
                }
                Some(ESCAPE) => {
                    self.advance(1)?;
                    output.push(self.parse_escape()?);
                }
                Some(byte) if byte < SPACE || byte == DELETE => {
                    return Err(self.reject(super::model::FailureClass::NonCanonical));
                }
                Some(_) => {
                    let character = self
                        .input
                        .get(self.cursor..)
                        .and_then(|remaining| remaining.chars().next())
                        .ok_or_else(|| self.reject(super::model::FailureClass::InvalidTextControl))?;
                    output.push(character);
                    self.advance(character.len_utf8())?;
                }
                None => return Err(self.reject(super::model::FailureClass::InvalidEscape)),
            }
        }
    }

    fn parse_list(&mut self, depth: u32) -> Result<super::model::Value, super::model::Rejection> {
        self.advance(1)?;
        let remaining_nodes = self.budget.max_nodes.saturating_sub(self.nodes);
        let capacity =
            usize::try_from(remaining_nodes).map_err(|_| self.reject(super::model::FailureClass::SizeBudget))?;
        let mut children = Vec::with_capacity(capacity.min(self.bytes.len().saturating_sub(self.cursor)));
        if self.peek() == Some(CLOSE) {
            self.advance(1)?;
            return Ok(super::model::Value::List(children));
        }
        if self.peek() == Some(SPACE) {
            return Err(self.reject(super::model::FailureClass::NonCanonical));
        }
        loop {
            let child_depth =
                depth.checked_add(1).ok_or_else(|| self.reject(super::model::FailureClass::DepthBudget))?;
            children.push(self.parse_value(child_depth)?);
            match self.peek() {
                Some(CLOSE) => {
                    self.advance(1)?;
                    return Ok(super::model::Value::List(children));
                }
                Some(SPACE) => {
                    self.advance(1)?;
                    if matches!(self.peek(), Some(SPACE | CLOSE) | None) {
                        return Err(self.reject(super::model::FailureClass::NonCanonical));
                    }
                }
                Some(_) => return Err(self.reject(super::model::FailureClass::NonCanonical)),
                None => return Err(self.reject(super::model::FailureClass::UnbalancedDelimiter)),
            }
        }
    }
}

/// Parses only the Trellis canonical grammar within explicit budgets.
// r[impl s-expression.parse-admission]
// r[impl s-expression.budgets]
pub fn parse_canonical(
    input: &str,
    budget: super::model::Budget,
) -> Result<super::model::Value, super::model::Rejection> {
    if !super::admission::budget_is_admitted(budget) {
        return Err(super::model::Rejection {
            class: super::model::FailureClass::InvalidBudget,
            offset: 0,
        });
    }
    let input_bytes = u32::try_from(input.len()).map_err(|_| super::model::Rejection {
        class: super::model::FailureClass::ByteBudget,
        offset: 0,
    })?;
    if input_bytes > budget.max_bytes {
        return Err(super::model::Rejection {
            class: super::model::FailureClass::ByteBudget,
            offset: budget.max_bytes,
        });
    }
    let mut parser = Parser::new(input, budget);
    let value = parser.parse_value(1)?;
    if parser.cursor != parser.bytes.len() {
        let class = if parser.peek() == Some(CLOSE) {
            super::model::FailureClass::UnbalancedDelimiter
        } else {
            super::model::FailureClass::TrailingInput
        };
        return Err(parser.reject(class));
    }
    let rendered = super::emit::serialize_canonical(&value, budget)?;
    if rendered != input {
        return Err(parser.reject(super::model::FailureClass::NonCanonical));
    }
    Ok(value)
}
