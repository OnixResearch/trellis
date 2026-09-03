//! Finite metadata values for implicit caller-location admission.

use vstd::prelude::*;

verus! {

/// Maximum entries accepted by chain and trace helpers.
pub const MAX_METADATA_ENTRIES: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CallingConvention {
    Rust,
    C,
    System,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemKind {
    Function,
    TraitPrototype,
    TraitDefault,
    TraitImplementation,
    Closure,
    Foreign,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnnotationSource {
    Absent,
    Local,
    Prototype,
    Default,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocationOrigin {
    DefinitionSite,
    CallSite,
    Forwarded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FunctionMetadata {
    pub id: u64,
    pub convention: CallingConvention,
    pub kind: ItemKind,
    pub naked: bool,
    pub annotation: AnnotationSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Location {
    pub id: u64,
    pub origin: LocationOrigin,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallContext {
    pub propagates_location: bool,
    pub call_site: Option<Location>,
    pub forwarded: Option<Location>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectEdge {
    pub id: u64,
    pub callee_tracked: bool,
    pub context: CallContext,
    pub supplied: Option<Location>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReifyShim {
    pub target_id: u64,
    pub definition_site: Location,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PointerUse {
    pub id: u64,
    pub target_id: u64,
    pub target_tracked: bool,
    pub shim: Option<ReifyShim>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraitBody {
    Prototype,
    Default,
    Override,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarkerState {
    Absent,
    Present,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TraitMethod {
    pub id: u64,
    pub body: TraitBody,
    pub prototype_marker: MarkerState,
    pub default_marker: MarkerState,
    pub local_marker: MarkerState,
    pub observed_effective: MarkerState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TraceEntry {
    pub edge_id: u64,
    pub location: Location,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmissionCode {
    Accepted,
    InvalidConvention,
    ClosureTarget,
    NakedTarget,
    UnsupportedItem,
    MissingLocation,
    WrongLocation,
    UnexpectedLocation,
    MissingShim,
    WrongShim,
    InheritanceMismatch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Admission {
    pub code: AdmissionCode,
    pub subject_id: u64,
}

// r[impl track-caller.trait-inheritance]
pub open spec fn marker_present_spec(marker: MarkerState) -> bool {
    match marker {
        MarkerState::Absent => false,
        MarkerState::Present => true,
    }
}

// r[impl track-caller.trait-inheritance]
pub fn marker_present(marker: MarkerState) -> (result: bool)
    ensures result == marker_present_spec(marker)
{
    matches!(marker, MarkerState::Present)
}

// r[impl track-caller.location-metadata]
pub open spec fn origins_equal_spec(left: LocationOrigin, right: LocationOrigin) -> bool {
    match (left, right) {
        (LocationOrigin::DefinitionSite, LocationOrigin::DefinitionSite)
        | (LocationOrigin::CallSite, LocationOrigin::CallSite)
        | (LocationOrigin::Forwarded, LocationOrigin::Forwarded) => true,
        _ => false,
    }
}

// r[impl track-caller.location-metadata]
pub fn origins_equal(left: LocationOrigin, right: LocationOrigin) -> (result: bool)
    ensures result == origins_equal_spec(left, right)
{
    matches!(
        (left, right),
        (LocationOrigin::DefinitionSite, LocationOrigin::DefinitionSite)
            | (LocationOrigin::CallSite, LocationOrigin::CallSite)
            | (LocationOrigin::Forwarded, LocationOrigin::Forwarded)
    )
}

// r[impl track-caller.location-metadata]
pub open spec fn locations_equal_spec(left: Location, right: Location) -> bool {
    left.id == right.id && origins_equal_spec(left.origin, right.origin)
}

// r[impl track-caller.location-metadata]
pub fn locations_equal(left: Location, right: Location) -> (result: bool)
    ensures result == locations_equal_spec(left, right)
{
    left.id == right.id && origins_equal(left.origin, right.origin)
}

// r[impl track-caller.const-codegen-agreement]
pub open spec fn trace_entries_equal_spec(left: TraceEntry, right: TraceEntry) -> bool {
    left.edge_id == right.edge_id && locations_equal_spec(left.location, right.location)
}

// r[impl track-caller.const-codegen-agreement]
pub fn trace_entries_equal(left: TraceEntry, right: TraceEntry) -> (result: bool)
    ensures result == trace_entries_equal_spec(left, right)
{
    left.edge_id == right.edge_id && locations_equal(left.location, right.location)
}

// r[impl track-caller.location-metadata]
pub open spec fn forwarded_location_spec(root: Location) -> Location {
    Location { id: root.id, origin: LocationOrigin::Forwarded }
}

} // verus!
