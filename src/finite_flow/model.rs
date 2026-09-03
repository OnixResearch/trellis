//! Finite lattice, effect, fixed-point, and result-inspection values.

/// Maximum states in one finite lattice.
pub const MAX_STATES: u32 = 16;
/// Maximum effect rows in one analysis.
pub const MAX_ROWS: u32 = 4_096;
/// Maximum admitted fixed-point rounds.
pub const MAX_ROUNDS: u32 = 1_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateKey(pub u32);

/// Finite join table and declared lattice bounds.
// r[impl mir-dataflow.domain]
// r[impl mir-dataflow.lattice-laws]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Domain {
    pub state_count: u32,
    pub row_width: u32,
    pub bottom: Option<StateKey>,
    pub top: Option<StateKey>,
    pub height: Option<u32>,
    pub join: Vec<StateKey>,
}

/// Analysis traversal direction.
// r[impl mir-dataflow.direction]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Backward,
}

/// Explicit total state-transfer table.
// r[impl mir-dataflow.effects]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transfer {
    pub outputs: Vec<StateKey>,
}

/// Before and primary effects for one admitted MIR location.
// r[impl mir-dataflow.effects]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocationEffect {
    pub location: crate::mir_visitor_traversal::Location,
    pub before: Transfer,
    pub primary: Transfer,
}

/// One successor-local transfer.
// r[impl mir-dataflow.per-edge]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EdgeEffect {
    pub source: u32,
    pub target: u32,
    pub transfer: Transfer,
}

/// Complete finite analysis descriptor.
// r[impl mir-dataflow.admission]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Analysis {
    pub domain: Domain,
    pub direction: Direction,
    pub boundary_block: u32,
    pub boundary_state: StateKey,
    pub effects: Vec<LocationEffect>,
    pub edge_effects: Vec<EdgeEffect>,
    pub convergence_budget: u32,
}

/// State snapshots around one location's ordered effects.
// r[impl mir-dataflow.results]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocationState {
    pub location: crate::mir_visitor_traversal::Location,
    pub input: StateKey,
    pub after_before: StateKey,
    pub after_primary: StateKey,
}

/// State supplied along one admitted CFG edge.
// r[impl mir-dataflow.results]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EdgeState {
    pub source: u32,
    pub target: u32,
    pub state: StateKey,
}

/// Stable fixed-point result metadata.
// r[impl mir-dataflow.convergence]
// r[impl mir-dataflow.results]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Results {
    pub block_entries: Vec<StateKey>,
    pub block_exits: Vec<StateKey>,
    pub locations: Vec<LocationState>,
    pub edges: Vec<EdgeState>,
    pub rounds: u32,
}

/// Pure cursor points over fixed results.
// r[impl mir-dataflow.results]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Point {
    BlockEntry {
        block: u32,
    },
    BlockExit {
        block: u32,
    },
    Before {
        location: crate::mir_visitor_traversal::Location,
    },
    Primary {
        location: crate::mir_visitor_traversal::Location,
    },
    Edge {
        source: u32,
        target: u32,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Table {
    Domain,
    Effect,
    Edge,
    Result,
}

/// Stable admission and convergence failures.
// r[impl mir-dataflow.admission]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FailureClass {
    InvalidBody,
    InvalidDomain,
    MissingBottom,
    MissingTop,
    MissingHeight,
    InvalidJoinTable,
    LatticeLaw,
    InvalidBoundary,
    InvalidLocation,
    DuplicateEffect,
    InvalidTransfer,
    NonMonotoneTransfer,
    InvalidEdgeEffect,
    DuplicateEdgeEffect,
    BudgetTooSmall,
    BudgetTooLarge,
    BudgetExhausted,
    InvalidResultPoint,
    DuplicateRequest,
}

/// Deterministic table-relative failure metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rejection {
    pub class: FailureClass,
    pub table: Table,
    pub index: Option<u32>,
    pub subject: Option<u32>,
}

/// Admission facts available before fixed-point execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmissionFacts {
    pub block_count: u32,
    pub required_budget: u32,
    pub effect_count: u32,
    pub edge_effect_count: u32,
}
