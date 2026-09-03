//! Finite canonical parenthesized-tree values and parser metadata.

/// Maximum node budget accepted by the bounded core.
pub const MAX_NODE_COUNT: u32 = 4_096;
/// Maximum recursive nesting depth accepted by the bounded core.
pub const MAX_NESTING_DEPTH: u32 = 128;
/// Maximum UTF-8 input or output bytes accepted by the bounded core.
pub const MAX_TEXT_BYTES: u32 = 1_048_576;

/// A dialect-neutral symbol or quoted text atom.
// r[impl s-expression.ast]
// r[impl s-expression.atom-admission]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Atom {
    Symbol(String),
    Text(String),
}

/// A finite atom-or-list tree.
// r[impl s-expression.ast]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Atom(Atom),
    List(Vec<Value>),
}

/// Caller-selected budgets, constrained by the core maxima.
// r[impl s-expression.budgets]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Budget {
    pub max_nodes: u32,
    pub max_depth: u32,
    pub max_bytes: u32,
}

/// Checked finite-tree measures.
// r[impl s-expression.budgets]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Measures {
    pub nodes: u32,
    pub depth: u32,
    pub bytes: u32,
}

/// Stable admission and parse failure classes.
// r[impl s-expression.parse-admission]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FailureClass {
    InvalidBudget,
    EmptyAtom,
    ReservedAtom,
    InvalidTextControl,
    InvalidEscape,
    UnbalancedDelimiter,
    TrailingInput,
    NonCanonical,
    SizeBudget,
    DepthBudget,
    ByteBudget,
    RoundtripMismatch,
}

/// Deterministic zero-based UTF-8 byte offset metadata.
// r[impl s-expression.parse-admission]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rejection {
    pub class: FailureClass,
    pub offset: u32,
}

impl Budget {
    /// Returns the largest budget admitted by this bounded core.
    // r[impl s-expression.budgets]
    #[must_use]
    pub const fn maximum() -> Self {
        Self {
            max_nodes: MAX_NODE_COUNT,
            max_depth: MAX_NESTING_DEPTH,
            max_bytes: MAX_TEXT_BYTES,
        }
    }
}
