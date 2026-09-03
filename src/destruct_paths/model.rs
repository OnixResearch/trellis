//! Finite structural-path, initializedness, and destruction-decision values.

pub const MAX_PATHS: u32 = 4_096;
pub const MAX_EXPANSION_STEPS: u32 = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathKey(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlagKey(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValueKey(pub u32);

/// One MIR place and its parent-relative path metadata.
// r[impl mir-drop.obligations]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Path {
    pub key: PathKey,
    pub place: crate::mir_core::Place,
    pub parent: Option<PathKey>,
    pub depth: u32,
}

/// One enum variant's droppable child paths.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariantFields {
    pub variant: u32,
    pub children: Vec<PathKey>,
}

/// Caller-supplied finite type destruction shape.
// r[impl mir-drop.obligations]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Shape {
    NoGlue,
    Direct,
    Fields(Vec<PathKey>),
    Enum(Vec<VariantFields>),
    Unsupported,
}

/// Destruction metadata for one structural path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Metadata {
    pub path: PathKey,
    pub shape: Shape,
}

/// Complete path and type-metadata input over an admitted MIR body.
// r[impl mir-drop.admission]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    pub body: crate::mir_core::Body,
    pub paths: Vec<Path>,
    pub metadata: Vec<Metadata>,
}

/// Caller-supplied maybe facts and active obligations.
// r[impl mir-drop.initializedness]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub maybe_initialized: Vec<PathKey>,
    pub maybe_uninitialized: Vec<PathKey>,
    pub obligations: Vec<PathKey>,
}

/// Finite initializedness updates.
// r[impl mir-drop.initializedness]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Initialize(PathKey),
    Move(PathKey),
    Overwrite(PathKey),
}

/// Whole-target or descendant-sensitive conditional policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConditionalPolicy {
    WholeTarget,
    OpenDescendants,
}

/// Static, dead, conditional, or open destruction class.
// r[impl mir-drop.classification]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Classification {
    Static,
    Dead,
    Conditional,
    Open,
}

/// Active enum variant supplied for one open enum path.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveVariant {
    pub path: PathKey,
    pub variant: u32,
}

/// One bounded expansion request with explicit enum facts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpansionRequest {
    pub target: PathKey,
    pub policy: ConditionalPolicy,
    pub active_variants: Vec<ActiveVariant>,
    pub budget: u32,
}

/// One deterministic old-target then replacement request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplacementRequest {
    pub expansion: ExpansionRequest,
    pub replacement: ValueKey,
}

/// One emitted metadata step. No destructor is executed.
// r[impl mir-drop.drop-flags]
// r[impl mir-drop.drop-and-replace]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Call { path: PathKey },
    Guarded { path: PathKey, flag: FlagKey },
    Write { path: PathKey, value: ValueKey },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Table {
    Path,
    Metadata,
    State,
    Expansion,
}

/// Stable first-failure classes.
// r[impl mir-drop.admission]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FailureClass {
    InvalidBody,
    TableTooWide,
    DuplicatePath,
    InvalidPath,
    InvalidParent,
    InvalidDepth,
    MissingMetadata,
    DuplicateMetadata,
    UnsupportedMetadata,
    InvalidChild,
    DuplicateFact,
    InvalidObligation,
    InconsistentFacts,
    MissingActiveVariant,
    InvalidActiveVariant,
    BudgetExhausted,
}

/// Deterministic table-relative rejection metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rejection {
    pub class: FailureClass,
    pub table: Table,
    pub index: Option<u32>,
    pub subject: Option<u32>,
}

/// Validated path and metadata counts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Facts {
    pub path_count: u32,
    pub metadata_count: u32,
}
