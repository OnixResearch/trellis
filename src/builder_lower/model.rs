//! Finite builder handles, mappings, operations, and failure metadata.

/// Maximum finite operations and metadata rows admitted by one record.
pub const MAX_OPERATIONS: u32 = 4_096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockKey(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValueKey(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FunctionKey(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypeKey(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlaceKey(pub u32);

/// Globally declared non-value handles.
// r[impl backend-trace.handles]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Declarations {
    pub blocks: Vec<BlockKey>,
    pub functions: Vec<FunctionKey>,
    pub types: Vec<TypeKey>,
    pub places: Vec<PlaceKey>,
}

/// One MIR block's direct and declared expansion blocks.
// r[impl backend-trace.block-mapping]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockMapping {
    pub source_block: u32,
    pub direct: BlockKey,
    pub expansions: Vec<BlockKey>,
}

/// SSA-like local storage classification.
// r[impl backend-trace.ssa-like-locals]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalStorage {
    DirectValue,
    MemoryBacked,
}

/// One MIR local's lowering classification.
// r[impl backend-trace.ssa-like-locals]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocalClass {
    pub local: u32,
    pub storage: LocalStorage,
}

/// Finite builder-operation kinds.
// r[impl backend-trace.lowering-trace]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operation {
    EmitStatement {
        inputs: Vec<ValueKey>,
    },
    EmitOperand {
        result: ValueKey,
        type_key: TypeKey,
    },
    EmitPlace {
        place: PlaceKey,
    },
    EmitRvalue {
        inputs: Vec<ValueKey>,
        result: ValueKey,
        type_key: TypeKey,
    },
    Branch {
        targets: Vec<BlockKey>,
    },
    Call {
        function: FunctionKey,
        arguments: Vec<ValueKey>,
        result: Option<(ValueKey, TypeKey)>,
        return_block: BlockKey,
        unwind_block: Option<BlockKey>,
    },
    Return {
        value: Option<ValueKey>,
    },
    Unreachable,
    Unsupported,
}

/// One source-tied operation emitted into a declared block.
// r[impl backend-trace.lowering-trace]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub source: crate::mir_visitor_traversal::Location,
    pub emission_block: BlockKey,
    pub operation: Operation,
}

/// Complete finite input for builder-record admission.
// r[impl backend-trace.admission]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    pub declarations: Declarations,
    pub mappings: Vec<BlockMapping>,
    pub locals: Vec<LocalClass>,
    pub entries: Vec<Entry>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Table {
    Declaration,
    Mapping,
    Local,
    Entry,
}

/// Stable first-failure classes.
// r[impl backend-trace.admission]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FailureClass {
    TableTooWide,
    InvalidBody,
    DuplicateDeclaration,
    MissingDeclaration,
    InvalidSourceBlock,
    DuplicateMapping,
    MissingMapping,
    DuplicateBlockAssignment,
    InvalidLocation,
    LocationKindMismatch,
    EmissionBlockMismatch,
    UseBeforeDefinition,
    DuplicateValueDefinition,
    InvalidControlTarget,
    MissingTerminator,
    OperationAfterTerminator,
    UnsupportedOperation,
    InvalidLocal,
    ConflictingLocalClass,
    MissingLocalClass,
}

/// Deterministic table-relative rejection metadata.
// r[impl backend-trace.admission]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rejection {
    pub class: FailureClass,
    pub table: Table,
    pub index: Option<u32>,
    pub subject: Option<u32>,
}

/// Counts exported only after complete admission.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Facts {
    pub block_count: u32,
    pub operation_count: u32,
    pub value_count: u32,
    pub local_count: u32,
}
