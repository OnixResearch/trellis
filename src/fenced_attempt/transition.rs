//! Phase-step admission and completion-linkage checks.

/// Returns the target phase for one admitted state-changing report.
///
/// Terminal phases and unmatched pairs reject before any state changes.
// r[impl fenced-attempt-admission.terminal-closure]
// r[impl fenced-attempt-admission.refinement]
pub fn phase_step(
    phase: super::model::Phase,
    kind: super::model::ReportKind,
) -> Result<super::model::Phase, super::model::RejectClass> {
    let terminal_tag = if phase.is_terminal() {
        super::proofs::TERMINAL_TAG
    } else {
        0
    };
    if !super::proofs::phase_open(terminal_tag) {
        return Err(super::model::RejectClass::TerminalPhase);
    }
    match (phase, kind) {
        (super::model::Phase::Queued, super::model::ReportKind::Start)
        | (super::model::Phase::Running, super::model::ReportKind::Progress) => Ok(super::model::Phase::Running),
        (super::model::Phase::Running | super::model::Phase::Transferring, super::model::ReportKind::Checkpoint) => {
            Ok(super::model::Phase::Transferring)
        }
        (super::model::Phase::Running | super::model::Phase::Transferring, super::model::ReportKind::ResultReady) => {
            Ok(super::model::Phase::ResultReady)
        }
        (
            super::model::Phase::Running | super::model::Phase::Transferring | super::model::Phase::ResultReady,
            super::model::ReportKind::Failure,
        ) => Ok(super::model::Phase::Failed),
        (super::model::Phase::ResultReady, super::model::ReportKind::Completion) => Ok(super::model::Phase::Completed),
        _ => Err(super::model::RejectClass::InvalidTransition),
    }
}

fn completion_rejection(
    state: &super::model::State,
    report: &super::model::Report,
) -> Option<super::model::RejectClass> {
    if state.phase != super::model::Phase::ResultReady {
        return Some(super::model::RejectClass::InvalidTransition);
    }
    if !scope_admits(report.worker) {
        return Some(super::model::RejectClass::WorkerDenied);
    }
    if !scope_admits(report.output) {
        return Some(super::model::RejectClass::OutputDenied);
    }
    let named = match report.result {
        Some(link) => link,
        None => return Some(super::model::RejectClass::MissingResult),
    };
    match state.result {
        None => Some(super::model::RejectClass::MissingResult),
        Some(retained) => {
            let link_match_tag = u8::from(retained == named);
            if super::proofs::completion_admitted(1, 1, 1, link_match_tag) {
                None
            } else {
                Some(super::model::RejectClass::ResultMismatch)
            }
        }
    }
}

/// Returns the authority or linkage rejection class for one report, if any.
///
/// Completion is stricter than progress: only it may consult output scope and
/// the retained result identity.
// r[impl fenced-attempt-admission.completion-linkage]
// r[impl fenced-attempt-admission.refinement]
#[must_use]
pub fn authority_rejection(
    state: &super::model::State,
    report: &super::model::Report,
) -> Option<super::model::RejectClass> {
    if report.kind == super::model::ReportKind::Completion {
        return completion_rejection(state, report);
    }
    if matches!(
        report.kind,
        super::model::ReportKind::Start
            | super::model::ReportKind::Progress
            | super::model::ReportKind::Checkpoint
            | super::model::ReportKind::ResultReady
    ) && !scope_admits(report.worker)
    {
        return Some(super::model::RejectClass::WorkerDenied);
    }
    None
}

/// Returns the completed-phase result retained after a matching completion.
// r[impl fenced-attempt-admission.completion-linkage]
#[must_use]
pub fn retained_result(state: &super::model::State, report: &super::model::Report) -> Option<super::model::ResultLink> {
    if report.kind == super::model::ReportKind::Completion && authority_rejection(state, report).is_none() {
        state.result
    } else {
        None
    }
}

/// Returns the next strictly greater nonzero epoch for reassignment.
// r[impl fenced-attempt-admission.fence-safety]
pub fn next_epoch(current: super::model::Epoch) -> Result<super::model::Epoch, super::model::RejectClass> {
    match current.0.checked_add(1) {
        Some(value) => {
            debug_assert_eq!(super::proofs::next_epoch_checked(current.0, super::model::EPOCH_LIMIT), value);
            Ok(super::model::Epoch(value))
        }
        None => Err(super::model::RejectClass::EpochExhausted),
    }
}

/// Marks one state superseded and seeds a fresh queued run for reassignment.
// r[impl fenced-attempt-admission.fence-safety]
// r[impl fenced-attempt-admission.refinement]
pub fn supersede(
    state: &super::model::State,
    next_run: super::model::RunId,
) -> Result<super::model::State, super::model::Rejection> {
    let epoch = next_epoch(state.epoch).map_err(|class| super::model::Rejection {
        class,
        event: state.events.last().map_or(super::model::EventId(0), |(event, _)| *event),
    })?;
    Ok(super::model::State {
        job: state.job,
        run: next_run,
        epoch,
        phase: super::model::Phase::Queued,
        progress: 0,
        events: Vec::new(),
        result: None,
    })
}

/// Marks one state superseded, preserving every other modeled field.
///
/// Reassignment then seeds a fresh queued run with `supersede`.
// r[impl fenced-attempt-admission.terminal-closure]
// r[impl fenced-attempt-admission.rejection-preservation]
#[must_use]
pub fn retire(state: &super::model::State) -> super::model::State {
    super::model::State {
        job: state.job,
        run: state.run,
        epoch: state.epoch,
        phase: super::model::Phase::Superseded,
        progress: state.progress,
        events: state.events.clone(),
        result: state.result,
    }
}

/// Confirms whether one worker scope admits a gated decision.
#[must_use]
pub const fn scope_admits(scope: super::model::Scope) -> bool {
    scope.is_granted()
}
