//! Finite phase, report, identity, scope, linkage, and decision values.

/// Maximum events retained in one modeled history.
pub const MAX_EVENTS: u32 = 4_096;
/// Largest modeled epoch before reassignment must reject.
pub const EPOCH_LIMIT: u64 = u64::MAX;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JobId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RunId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Epoch(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PayloadId(pub u64);

/// Closed modeled lifecycle phases.
// r[impl fenced-attempt-admission.boundary]
// r[impl fenced-attempt-admission.terminal-closure]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Phase {
    Queued,
    Running,
    Transferring,
    ResultReady,
    Completed,
    Failed,
    Superseded,
}

impl Phase {
    /// Returns whether this phase admits further state-changing reports.
    // r[impl fenced-attempt-admission.terminal-closure]
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Superseded)
    }

    /// Returns the closed finite phase tag used by verified correspondences.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::Queued => 0,
            Self::Running => 1,
            Self::Transferring => 2,
            Self::ResultReady => 3,
            Self::Completed => 4,
            Self::Failed => 5,
            Self::Superseded => 6,
        }
    }
}

impl core::fmt::Display for Phase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Transferring => "transferring",
            Self::ResultReady => "result-ready",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Superseded => "superseded",
        })
    }
}

/// Closed modeled report kinds.
// r[impl fenced-attempt-admission.boundary]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReportKind {
    Start,
    Progress,
    Checkpoint,
    ResultReady,
    Failure,
    Completion,
}

impl ReportKind {
    /// Returns the closed finite kind tag used by verified correspondences.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::Start => 0,
            Self::Progress => 1,
            Self::Checkpoint => 2,
            Self::ResultReady => 3,
            Self::Failure => 4,
            Self::Completion => 5,
        }
    }
}

/// Caller-admitted worker or output authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Scope {
    Granted,
    Denied,
}

impl Scope {
    /// Returns whether this scope admits the gated decision.
    #[must_use]
    pub const fn is_granted(self) -> bool {
        matches!(self, Self::Granted)
    }

    /// Returns the finite scope tag used by verified correspondences.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::Denied => 0,
            Self::Granted => 1,
        }
    }
}

/// Retained result identity exposed by completion.
// r[impl fenced-attempt-admission.completion-linkage]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ResultLink {
    pub run: RunId,
    pub payload: PayloadId,
}

/// One caller-admitted report over normalized identity facts.
// r[impl fenced-attempt-admission.refinement]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Report {
    pub job: JobId,
    pub run: RunId,
    pub epoch: Epoch,
    pub event: EventId,
    pub payload: PayloadId,
    pub kind: ReportKind,
    pub worker: Scope,
    pub output: Scope,
    pub result: Option<ResultLink>,
}

/// Complete modeled state for one durable job's current run.
// r[impl fenced-attempt-admission.refinement]
// r[impl fenced-attempt-admission.rejection-preservation]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct State {
    pub job: JobId,
    pub run: RunId,
    pub epoch: Epoch,
    pub phase: Phase,
    pub progress: u32,
    pub events: Vec<(EventId, PayloadId)>,
    pub result: Option<ResultLink>,
}

impl State {
    /// Returns a queued initial state for one job identity.
    // r[impl fenced-attempt-admission.refinement]
    #[must_use]
    pub fn initial(job: JobId, run: RunId, epoch: Epoch) -> Self {
        Self {
            job,
            run,
            epoch,
            phase: Phase::Queued,
            progress: 0,
            events: Vec::new(),
            result: None,
        }
    }
}

/// Successful report dispositions.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Disposition {
    Applied(State),
    Replayed,
}

/// Stable first-failure reason classes.
// r[impl fenced-attempt-admission.fence-safety]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RejectClass {
    StaleEpoch,
    FutureEpoch,
    WrongJob,
    WrongRun,
    EventConflict,
    TerminalPhase,
    InvalidTransition,
    WorkerDenied,
    OutputDenied,
    MissingResult,
    ResultMismatch,
    EpochExhausted,
    HistoryFull,
}

impl RejectClass {
    /// Returns the finite rejection tag used by verified correspondences.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::StaleEpoch => 0,
            Self::FutureEpoch => 1,
            Self::WrongJob => 2,
            Self::WrongRun => 3,
            Self::EventConflict => 4,
            Self::TerminalPhase => 5,
            Self::InvalidTransition => 6,
            Self::WorkerDenied => 7,
            Self::OutputDenied => 8,
            Self::MissingResult => 9,
            Self::ResultMismatch => 10,
            Self::EpochExhausted => 11,
            Self::HistoryFull => 12,
        }
    }
}

/// Deterministic rejection metadata naming the failing check.
// r[impl fenced-attempt-admission.rejection-preservation]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rejection {
    pub class: RejectClass,
    pub event: EventId,
}
