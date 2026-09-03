//! Finite Trellis-owned MIR values and deterministic failure metadata.

/// The distinguished return-place local index.
// r[impl mir-core.return-place]
pub const RETURN_PLACE: u32 = 0;

/// A declared local role in the finite MIR body.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalRole {
    Return,
    Argument,
    Variable,
    Temporary,
}

/// A shallow type tag. This is not a Rust type-system model.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeTag {
    Unit,
    Bool,
    U64,
    Aggregate,
    Reference,
}

/// One local declaration.
// r[impl mir-core.body-shape]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocalDecl {
    pub role: LocalRole,
    pub type_tag: TypeTag,
}

/// One place projection. `Unsupported` is a rejection marker for raw adapters.
// r[impl mir-core.place-projection]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Projection {
    Dereference,
    Field { field: u32 },
    Index { local: u32 },
    Unsupported,
}

/// A local plus its ordered projection chain.
// r[impl mir-core.local-reference]
// r[impl mir-core.place-projection]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Place {
    pub local: u32,
    pub projections: Vec<Projection>,
}

/// A finite constant value with a declared shallow type tag.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Constant {
    pub type_tag: TypeTag,
    pub bits: u64,
}

/// An operand accepted by the finite MIR core.
// r[impl mir-core.rvalue-nonnesting]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operand {
    Constant(Constant),
    Copy(Place),
    Move(Place),
}

/// A finite binary operator identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Equal,
}

/// A shallow rvalue. `Nested` represents a malformed raw candidate.
// r[impl mir-core.rvalue-nonnesting]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Rvalue {
    Use(Operand),
    Binary {
        operator: BinaryOperator,
        left: Operand,
        right: Operand,
    },
    Reference(Place),
    Aggregate(Vec<Operand>),
    Nested,
}

/// A statement in one basic block.
// r[impl mir-core.block-terminator]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Assign { destination: Place, value: Rvalue },
    StorageLive { local: u32 },
    StorageDead { local: u32 },
    Nop,
}

/// A final basic-block terminator.
// r[impl mir-core.successors]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Terminator {
    Return,
    Goto {
        target: u32,
    },
    Switch {
        discriminant: Operand,
        targets: Vec<u32>,
    },
    Call {
        function: Operand,
        arguments: Vec<Operand>,
        destination: Option<(Place, u32)>,
    },
    Drop {
        place: Place,
        target: u32,
    },
    Unreachable,
}

/// Zero or more statements and at most one final terminator candidate.
// r[impl mir-core.block-terminator]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BasicBlock {
    pub statements: Vec<Statement>,
    pub terminator: Option<Terminator>,
}

/// A finite MIR body whose tables provide all reference widths.
// r[impl mir-core.body-shape]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Body {
    pub locals: Vec<LocalDecl>,
    pub blocks: Vec<BasicBlock>,
}

/// Stable malformed-input classes.
// r[impl mir-core.failure-metadata]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FailureClass {
    EmptyBlockTable,
    TableTooWide,
    MissingReturnPlace,
    MissingTerminator,
    InvalidLocal,
    UnsupportedProjection,
    NestedRvalue,
    OutOfRangeSuccessor,
}

/// Context-free failure details from one admission family.
// r[impl mir-core.failure-metadata]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Failure {
    pub class: FailureClass,
    pub subject_index: Option<u32>,
    pub projection_index: Option<u32>,
}

/// Body-relative first-failure metadata.
// r[impl mir-core.failure-metadata]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rejection {
    pub class: FailureClass,
    pub block_index: Option<u32>,
    pub statement_index: Option<u32>,
    pub subject_index: Option<u32>,
    pub projection_index: Option<u32>,
}

/// One admitted CFG edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edge {
    pub source: u32,
    pub target: u32,
}

/// Product-neutral facts exposed to downstream analyses.
// r[impl mir-core.downstream-adapter]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdapterFacts {
    pub block_count: u32,
    pub local_count: u32,
    pub edges: Vec<Edge>,
}
