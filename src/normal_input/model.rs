//! Finite typed values, canonical normal forms, and import policy metadata.

/// Maximum rows accepted by each finite table.
pub const MAX_ROWS: u32 = 4_096;
/// Width of one opaque consumer-supplied semantic digest.
pub const DIGEST_BYTES: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeKey(pub u32);

/// Finite declared value tags.
// r[impl domain-primitives.typed-config.expression-envelope]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeTag {
    Bool,
    Natural,
    Text,
    List,
    Record,
    Variant,
    Optional,
    Reference,
    Import,
}

/// Supported scalar values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Scalar {
    Bool(bool),
    Natural(u64),
    Text(String),
}

/// One named record child.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub value: NodeKey,
}

/// Supported finite value shapes. `Unsupported` is a raw rejection marker.
// r[impl domain-primitives.typed-config.expression-envelope]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Scalar(Scalar),
    List(Vec<NodeKey>),
    Record(Vec<Field>),
    Variant { tag: String, value: Option<NodeKey> },
    Optional(Option<NodeKey>),
    Reference(NodeKey),
    Import { edge: u32 },
    Unsupported,
}

/// One finite expression node. Source labels and comments are non-semantic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub key: NodeKey,
    pub declared_type: TypeTag,
    pub kind: Kind,
    pub source_label: String,
    pub comment: String,
}

/// One required record field and its expected child type.
// r[impl domain-primitives.typed-config.type-admission]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequiredField {
    pub record: NodeKey,
    pub name: String,
    pub expected: TypeTag,
}

/// Opaque digest metadata. Hash computation remains outside this core.
// r[impl domain-primitives.typed-config.semantic-digest]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Digest(pub [u8; DIGEST_BYTES]);

/// Import source authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Origin {
    Local,
    Environment,
    Remote { authority: u32 },
}

/// Import target class.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Relative { authority: u32 },
    LocalFile,
    Home,
    Environment,
    Remote { authority: u32 },
    Computed,
}

/// Whether import selection depends on a value branch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selection {
    Static,
    Conditional,
}

/// Header forwarding authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeaderPolicy {
    None,
    Forward,
}

/// One frozen finite import edge.
// r[impl domain-primitives.typed-config.import-closure]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImportEdge {
    pub source: Origin,
    pub target: Target,
    pub selection: Selection,
    pub headers: HeaderPolicy,
    pub declared: Digest,
    pub observed: Digest,
}

/// Complete finite input for type, normalization, and import admission.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    pub root: NodeKey,
    pub nodes: Vec<Node>,
    pub required_fields: Vec<RequiredField>,
    pub imports: Vec<ImportEdge>,
    pub dependency_digests: Vec<Digest>,
}

/// One canonical child reference.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NormalChild {
    pub label: String,
    pub target: NodeKey,
}

/// One canonical node with source-only metadata removed.
// r[impl domain-primitives.typed-config.normal-form]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NormalEntry {
    pub key: NodeKey,
    pub type_tag: TypeTag,
    pub payload: String,
    pub children: Vec<NormalChild>,
}

/// Canonical key-ordered normal-form metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NormalForm {
    pub root: NodeKey,
    pub entries: Vec<NormalEntry>,
}

/// Exact identity input selected for a future stack-owned digest.
// r[impl domain-primitives.typed-config.semantic-digest]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DigestInput {
    pub normal: NormalForm,
    pub dependencies: Vec<Digest>,
}

/// Explicit authority for a digest-changing refactor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compatibility {
    None,
    Compatible,
    UnresolvedDependents,
}

/// One before/after semantic refactor request.
// r[impl domain-primitives.typed-config.semantic-refactoring]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refactor {
    pub before: DigestInput,
    pub after: DigestInput,
    pub compatibility: Compatibility,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Table {
    Node,
    RequiredField,
    Import,
    Refactor,
}

/// Stable first-failure classes.
// r[impl domain-primitives.typed-config.explicit-failure]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FailureClass {
    TableTooWide,
    MissingRoot,
    DuplicateNode,
    DanglingNode,
    Cycle,
    TypeMismatch,
    MissingField,
    DuplicateField,
    UnsupportedForm,
    InvalidImportIndex,
    ComputedImport,
    ConditionalImport,
    RemoteAttenuation,
    CrossOriginHeaders,
    DigestMismatch,
    DigestChangingRefactor,
}

/// Deterministic table-relative failure metadata.
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
    pub node_count: u32,
    pub field_count: u32,
    pub import_count: u32,
}
