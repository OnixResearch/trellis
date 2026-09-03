//! Finite concrete-item, reference-graph, and partition values.

/// Maximum rows accepted in each finite collection table.
pub const MAX_ITEMS: u32 = 4_096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ItemKey(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnitKey(pub u32);

/// Finite concrete item families.
// r[impl mono.item-envelope]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemKind {
    Function,
    Static,
    Shim,
}

/// Declared concrete-instance shape.
// r[impl mono.item-envelope]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConcreteShape {
    Plain,
    Generic,
    Specialized,
    Static,
    Shim,
}

/// Whether the source definition belongs to the current product unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Origin {
    Local,
    External,
}

/// Trellis-owned stable or volatile partition class.
// r[impl mono.cgu-classification]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stability {
    Stable,
    Volatile,
}

/// One declared concrete function, static, or shim.
// r[impl mono.item-envelope]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub key: ItemKey,
    pub kind: ItemKind,
    pub shape: ConcreteShape,
    pub type_arguments: Vec<u64>,
    pub const_arguments: Vec<u64>,
    pub shim_source: Option<ItemKey>,
    pub origin: Origin,
    pub stability: Stability,
}

/// One finite item-reference edge.
// r[impl mono.collection]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reference {
    pub source: ItemKey,
    pub target: ItemKey,
}

/// One disjoint output unit.
// r[impl mono.cgu-partition]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unit {
    pub key: UnitKey,
    pub stability: Stability,
    pub items: Vec<ItemKey>,
}

/// Complete collection and partition admission input.
// r[impl mono.admission]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    pub items: Vec<Item>,
    pub roots: Vec<ItemKey>,
    pub references: Vec<Reference>,
    pub collected: Vec<ItemKey>,
    pub units: Vec<Unit>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Table {
    Item,
    Root,
    Reference,
    Collected,
    Unit,
}

/// Stable first-failure classes.
// r[impl mono.admission]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FailureClass {
    TableTooWide,
    DuplicateItem,
    MalformedInstance,
    ClassificationMismatch,
    MissingRoot,
    DanglingReference,
    DuplicateCollected,
    MissingReachable,
    ExtraCollected,
    DuplicateUnit,
    UnknownUnitItem,
    PartitionOverlap,
    PartitionOmission,
    UnitClassMismatch,
}

/// Deterministic table-relative rejection metadata.
// r[impl mono.admission]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rejection {
    pub class: FailureClass,
    pub table: Table,
    pub index: Option<u32>,
    pub subject: Option<u32>,
}

/// Counts exported only after complete collection and partition admission.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Facts {
    pub item_count: u32,
    pub root_count: u32,
    pub collected_count: u32,
    pub unit_count: u32,
}
