//! Identity-bound continuation capture and resume admission.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GenerationId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArtifactId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProfileId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ContextId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObservationSetId(pub u64);

/// Exact identities that define one resumable execution context.
// r[impl wasm-runtime-bounds.continuation]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResumeBindings {
    pub generation: GenerationId,
    pub artifact: ArtifactId,
    pub profile: ProfileId,
    pub input: InputId,
    pub context: ContextId,
    pub observations: ObservationSetId,
}

/// The boundary that created a continuation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContinuationReason {
    HostPause,
    OutOfFuel,
}

/// One immutable continuation descriptor.
// r[impl wasm-runtime-bounds.continuation]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Continuation {
    pub cursor: super::execution::Cursor,
    pub reason: ContinuationReason,
    pub bindings: ResumeBindings,
}

/// Successful resume data with the original reason retained.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResumeGrant {
    pub cursor: super::execution::Cursor,
    pub reason: ContinuationReason,
}

/// Continuation-capture failures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaptureFault {
    UnsupportedBoundary,
    InconsistentBoundary,
}

/// Stable first-mismatch resume rejection classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResumeFault {
    InconsistentBoundary,
    GenerationMismatch,
    ArtifactMismatch,
    ProfileMismatch,
    InputMismatch,
    ContextMismatch,
    ObservationMismatch,
}

/// Captures only explicit host-pause and out-of-fuel boundaries.
// r[impl wasm-runtime-bounds.continuation]
pub fn capture_continuation(
    result: super::execution::RunResult,
    bindings: ResumeBindings,
) -> Result<Continuation, CaptureFault> {
    let reason = match result.class {
        super::execution::RunClass::HostPaused => {
            if result.cursor.status != super::execution::MachineStatus::HostPaused {
                return Err(CaptureFault::InconsistentBoundary);
            }
            ContinuationReason::HostPause
        }
        super::execution::RunClass::OutOfFuel => {
            if result.cursor.status != super::execution::MachineStatus::Running {
                return Err(CaptureFault::InconsistentBoundary);
            }
            ContinuationReason::OutOfFuel
        }
        super::execution::RunClass::Running
        | super::execution::RunClass::Finished
        | super::execution::RunClass::Trapped(_) => {
            return Err(CaptureFault::UnsupportedBoundary);
        }
    };
    Ok(Continuation {
        cursor: result.cursor,
        reason,
        bindings,
    })
}

fn prepared_cursor(continuation: &Continuation) -> super::execution::Cursor {
    let mut cursor = continuation.cursor;
    if continuation.reason == ContinuationReason::HostPause {
        cursor.status = super::execution::MachineStatus::Running;
    }
    cursor
}

fn boundary_consistent(continuation: &Continuation) -> bool {
    match continuation.reason {
        ContinuationReason::HostPause => continuation.cursor.status == super::execution::MachineStatus::HostPaused,
        ContinuationReason::OutOfFuel => continuation.cursor.status == super::execution::MachineStatus::Running,
    }
}

/// Admits resume only when every bound identity matches exactly.
// r[impl wasm-runtime-bounds.continuation]
pub fn admit_resume(continuation: &Continuation, current: ResumeBindings) -> Result<ResumeGrant, ResumeFault> {
    if !boundary_consistent(continuation) {
        return Err(ResumeFault::InconsistentBoundary);
    }
    if continuation.bindings.generation != current.generation {
        return Err(ResumeFault::GenerationMismatch);
    }
    if continuation.bindings.artifact != current.artifact {
        return Err(ResumeFault::ArtifactMismatch);
    }
    if continuation.bindings.profile != current.profile {
        return Err(ResumeFault::ProfileMismatch);
    }
    if continuation.bindings.input != current.input {
        return Err(ResumeFault::InputMismatch);
    }
    if continuation.bindings.context != current.context {
        return Err(ResumeFault::ContextMismatch);
    }
    if continuation.bindings.observations != current.observations {
        return Err(ResumeFault::ObservationMismatch);
    }
    Ok(ResumeGrant {
        cursor: prepared_cursor(continuation),
        reason: continuation.reason,
    })
}
