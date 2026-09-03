//! Finite visitor locations, trace policy, mutation classes, and facts.

/// The enclosing MIR element for a nested place or operand visit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Owner {
    Statement { block: u32, statement: u32 },
    Terminator { block: u32 },
}

/// A finite visit location in an admitted MIR-core body.
// r[impl mir-visitor.locations]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Location {
    Body,
    Local { local: u32 },
    Block { block: u32 },
    Statement { block: u32, statement: u32 },
    Terminator { block: u32 },
    Place { owner: Owner, ordinal: u32 },
    Operand { owner: Owner, ordinal: u32 },
    Rvalue { block: u32, statement: u32 },
    Successor { block: u32, ordinal: u32 },
}

/// One ordered visitor-trace entry.
// r[impl mir-visitor.trace]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Entry {
    pub location: Location,
}

/// Coverage policy for a trace.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoverageMode {
    ExactlyOnce,
    Permissive,
}

/// Block-order policy for a trace.
// r[impl mir-visitor.traversal-order]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Order {
    Declared,
    Preorder,
    ReversePostorder,
}

/// Whether block-scoped locations may include unreachable blocks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReachabilityPolicy {
    ReachableOnly,
    AllBlocks,
}

/// The structural authority granted to one trace.
// r[impl mir-visitor.mut-shape]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MutationClass {
    Immutable,
    ValuesOnly,
    UnsupportedStructural,
}

/// One finite visitor trace and its declared policy.
// r[impl mir-visitor.trace]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trace {
    pub entries: Vec<Entry>,
    pub entry: u32,
    pub coverage: CoverageMode,
    pub order: Order,
    pub reachability: ReachabilityPolicy,
    pub mutation: MutationClass,
}

/// Structural body counts used by mutation admission.
// r[impl mir-visitor.mut-shape]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shape {
    pub locals: u32,
    pub blocks: u32,
    pub statements: u32,
    pub terminators: u32,
    pub successors: u32,
}

/// Facts exported only after trace admission succeeds.
// r[impl mir-visitor.coverage]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraceFacts {
    pub visited_count: u32,
    pub requested_count: u32,
    pub block_order: Vec<u32>,
    pub shape: Shape,
}

/// Stable visitor rejection classes.
// r[impl mir-visitor.admission]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FailureClass {
    InvalidBody,
    TraceTooWide,
    InvalidLocation,
    InvalidMode,
    DuplicateRequest,
    DuplicateVisit,
    MissingVisit,
    OutsideRequest,
    ReachabilityMismatch,
    OrderMismatch,
    MutationMismatch,
}

/// Deterministic first-failure metadata.
// r[impl mir-visitor.admission]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rejection {
    pub class: FailureClass,
    pub entry_index: Option<u32>,
    pub location: Option<Location>,
}
