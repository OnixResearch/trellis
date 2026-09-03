//! Finite owner-scoped HIR identifier and lowering-table values.

/// Maximum entries admitted in each finite record table.
pub const MAX_RECORDS: u32 = 4_096;

/// One HIR owner identity.
// r[impl hir-id.owner-scope]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OwnerKey(pub u32);

/// One source AST node identity.
// r[impl hir-id.lowering-map]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceKey(pub u32);

/// One owner-local HIR identity.
// r[impl hir-id.owner-scope]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScopedKey {
    pub owner: OwnerKey,
    pub local: u32,
}

/// One finite definition identity supplied by a caller-owned collector.
// r[impl hir-id.def-correspondence]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DefinitionKey(pub u32);

/// One source-node to HIR-ID assignment.
// r[impl hir-id.lowering-map]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mapping {
    pub source: SourceKey,
    pub target: ScopedKey,
}

/// Whether a created HIR ID must appear in a HIR structure.
// r[impl hir-id.admission]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsePolicy {
    Required,
    ExplicitDiscard,
}

/// One created HIR ID and its declared use policy.
// r[impl hir-id.admission]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CreatedKey {
    pub id: ScopedKey,
    pub policy: UsePolicy,
}

/// One synthetic HIR node introduced by lowering.
// r[impl hir-id.fresh-generated]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GeneratedNode {
    pub generation: u32,
    pub target: ScopedKey,
}

/// One source-node to definition-metadata correspondence.
// r[impl hir-id.def-correspondence]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DefinitionEntry {
    pub source: SourceKey,
    pub definition: DefinitionKey,
}

/// Complete finite input for HIR-ID admission.
// r[impl hir-id.owner-scope]
// r[impl hir-id.lowering-map]
// r[impl hir-id.admission]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    pub owner: OwnerKey,
    pub source_nodes: Vec<SourceKey>,
    pub mappings: Vec<Mapping>,
    pub created_ids: Vec<CreatedKey>,
    pub used_ids: Vec<ScopedKey>,
    pub generated_nodes: Vec<GeneratedNode>,
    pub definition_requirements: Vec<SourceKey>,
    pub definitions: Vec<DefinitionEntry>,
}

/// Stable table identities for deterministic diagnostics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Table {
    Source,
    Mapping,
    Created,
    Used,
    Generated,
    DefinitionRequirement,
    Definition,
}

/// Stable first-failure classes.
// r[impl hir-id.admission]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FailureClass {
    TableTooWide,
    WrongOwner,
    DuplicateSource,
    UnknownSource,
    UnloweredSource,
    DuplicateHirId,
    MissingCreatedId,
    DuplicateCreatedId,
    UnknownUsedId,
    DuplicateUsedId,
    UnusedCreatedId,
    DiscardedIdUsed,
    DuplicateGeneration,
    GeneratedAlias,
    MissingDefinition,
    DuplicateDefinition,
    UnknownDefinitionSource,
}

/// Deterministic table-relative failure metadata.
// r[impl hir-id.admission]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rejection {
    pub class: FailureClass,
    pub table: Table,
    pub index: Option<u32>,
    pub related: Option<u32>,
}

/// Counts exported only after complete admission.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Facts {
    pub source_count: u32,
    pub mapping_count: u32,
    pub generated_count: u32,
    pub used_count: u32,
    pub definition_count: u32,
}
