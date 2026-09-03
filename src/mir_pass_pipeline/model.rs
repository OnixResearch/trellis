//! Finite MIR phase, pass, artifact-state, and failure metadata.

/// The stable abstract MIR dialect order modeled by Trellis.
// r[impl mir-pass.phase-order]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Built,
    ConstPrepared,
    Promoted,
    DropElaboratedConstChecked,
    Optimized,
}

/// The read and steal state of the current intermediate artifact.
// r[impl mir-pass.artifact-state]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactState {
    Available,
    Borrowed,
    Stolen,
}

/// A finite optimization gate for optional passes.
// r[impl mir-pass.optimization-gating]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OptimizationLevel {
    O0,
    O1,
    O2,
    O3,
}

/// Structural validation metadata attached to the current artifact.
// r[impl mir-pass.pass-compatibility]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValidationKind {
    Missing,
    Structural,
    Dialect,
}

/// Finite pass identities with repository-reviewed registrations.
// r[impl mir-pass.pass-compatibility]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    SimplifyCfg,
    RemoveStorageMarkers,
    PrepareConst,
    PromoteTemps,
    ElaborateDrops,
    InstCombine,
}

/// The structural mutation shape declared by a pass.
// r[impl mir-pass.pass-compatibility]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MutationKind {
    ReadOnly,
    InPlace,
}

/// The validation effect declared by an in-place pass.
// r[impl mir-pass.pass-compatibility]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StructureEffect {
    Preserved,
    RequiresValidation,
}

/// A pass registration checked against its canonical finite identity.
// r[impl mir-pass.pass-compatibility]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Descriptor {
    pub kind: Kind,
    pub input: Phase,
    pub output: Phase,
    pub mutation: MutationKind,
    pub structure: StructureEffect,
    pub required_validation: ValidationKind,
    pub minimum_optimization: OptimizationLevel,
}

/// Pipeline policy selected by the caller.
// r[impl mir-pass.optimization-gating]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub optimization: OptimizationLevel,
}

/// The deterministic state carried between trace steps.
// r[impl mir-pass.artifact-state]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct State {
    pub phase: Phase,
    pub artifact: ArtifactState,
    pub validation: ValidationKind,
    pub borrow_read_recorded: bool,
}

/// The dependency ordering required before a steal event.
// r[impl mir-pass.steal-order]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StealPolicy {
    Unconditional,
    PriorRead,
}

/// One finite trace event.
// r[impl mir-pass.admission]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Validate(ValidationKind),
    RunPass(Descriptor),
    Read,
    Borrow,
    Steal(StealPolicy),
    Advance(Phase),
}

/// Stable rejection classes for the first malformed trace step.
// r[impl mir-pass.failure-metadata]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FailureClass {
    PhaseOrder,
    PassCompatibility,
    MissingValidation,
    OptimizationGate,
    StealOrder,
    TraceLimit,
}

/// A deterministic failure class and zero-based step index.
// r[impl mir-pass.failure-metadata]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rejection {
    pub class: FailureClass,
    pub step_index: usize,
}

impl State {
    /// Returns the initial built-MIR state.
    #[must_use]
    pub const fn initial() -> Self {
        Self {
            phase: Phase::Built,
            artifact: ArtifactState::Available,
            validation: ValidationKind::Missing,
            borrow_read_recorded: false,
        }
    }
}
