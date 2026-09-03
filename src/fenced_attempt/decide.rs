//! Deterministic report application, state identity, and preservation.

fn reject(class: super::model::RejectClass, event: super::model::EventId) -> super::model::Rejection {
    super::model::Rejection { class, event }
}

fn preserved(
    class: super::model::RejectClass,
    event: super::model::EventId,
    state: &super::model::State,
) -> super::model::Rejection {
    let identity = state_identity(state);
    debug_assert!(super::proofs::preservation_admitted(0, identity, identity));
    reject(class, event)
}

fn push_event(
    state: &mut super::model::State,
    event: super::model::EventId,
    payload: super::model::PayloadId,
) -> Result<(), super::model::Rejection> {
    let count = u32::try_from(state.events.len()).map_err(|_| reject(super::model::RejectClass::HistoryFull, event))?;
    if count >= super::model::MAX_EVENTS {
        return Err(reject(super::model::RejectClass::HistoryFull, event));
    }
    state.events.push((event, payload));
    Ok(())
}

/// Applies one report and returns the next disposition or rejection.
///
/// Every rejection and replay preserves every modeled state field.
// r[impl fenced-attempt-admission.refinement]
// r[impl fenced-attempt-admission.fence-safety]
// r[impl fenced-attempt-admission.event-idempotence]
// r[impl fenced-attempt-admission.terminal-closure]
// r[impl fenced-attempt-admission.completion-linkage]
// r[impl fenced-attempt-admission.rejection-preservation]
pub fn apply_report(
    state: &super::model::State,
    report: &super::model::Report,
) -> Result<super::model::Disposition, super::model::Rejection> {
    if let Some(class) = super::relation::identity_rejection(state, report) {
        return Err(preserved(class, report.event, state));
    }
    let outcome = super::relation::classify_event(&state.events, report.event, report.payload);
    let outcome_tag = outcome.tag();
    if outcome_tag == super::proofs::CONFLICT_TAG || !super::proofs::event_outcome_admitted(outcome_tag) {
        return Err(preserved(super::model::RejectClass::EventConflict, report.event, state));
    }
    if outcome_tag == 1 {
        return Ok(super::model::Disposition::Replayed);
    }
    let next_phase = super::transition::phase_step(state.phase, report.kind)
        .map_err(|class| preserved(class, report.event, state))?;
    if let Some(class) = super::transition::authority_rejection(state, report) {
        return Err(preserved(class, report.event, state));
    }

    let mut next = super::model::State {
        job: state.job,
        run: state.run,
        epoch: state.epoch,
        phase: next_phase,
        progress: state.progress,
        events: Vec::with_capacity(state.events.len().saturating_add(1)),
        result: state.result,
    };
    next.events.extend_from_slice(&state.events);
    push_event(&mut next, report.event, report.payload)?;

    if report.kind == super::model::ReportKind::Progress || report.kind == super::model::ReportKind::Checkpoint {
        next.progress =
            state.progress.checked_add(1).ok_or(reject(super::model::RejectClass::HistoryFull, report.event))?;
    }
    if report.kind == super::model::ReportKind::ResultReady {
        next.result = report.result;
    }
    if report.kind == super::model::ReportKind::Completion {
        next.result = super::transition::retained_result(state, report);
    }
    Ok(super::model::Disposition::Applied(next))
}

/// Folds every modeled state field into one comparable identity value.
// r[impl fenced-attempt-admission.rejection-preservation]
#[must_use]
pub fn state_identity(state: &super::model::State) -> u64 {
    let mut acc = state.job.0
        ^ state.run.0.rotate_left(17)
        ^ state.epoch.0.rotate_left(31)
        ^ u64::from(state.phase.tag())
        ^ u64::from(state.progress).rotate_left(7);
    acc ^= state.result.map_or(0, |link| link.run.0).rotate_left(3);
    for (index, (event, payload)) in state.events.iter().enumerate() {
        let salt = (index as u64).wrapping_add(1).rotate_left(11);
        acc ^= event.0.wrapping_mul(salt) ^ payload.0.rotate_left(13);
    }
    acc
}

/// Confirms one disposition preserved the modeled state, when required.
///
/// Applied dispositions may change only the fields their report names, so the
/// caller supplies the observed next state for rejected or replayed outcomes.
// r[impl fenced-attempt-admission.rejection-preservation]
#[must_use]
pub fn preserves_state(before: &super::model::State, after: &super::model::State) -> bool {
    before == after
}

/// Confirms one applied report changed only the fields its kind names.
// r[impl fenced-attempt-admission.refinement]
#[must_use]
pub fn changes_only_named_fields(
    before: &super::model::State,
    after: &super::model::State,
    kind: super::model::ReportKind,
) -> bool {
    let has_history_grown =
        after.events.len() == before.events.len() + 1 && after.events[..before.events.len()] == before.events[..];
    let has_fixed_identity = before.job == after.job && before.run == after.run && before.epoch == after.epoch;
    match kind {
        super::model::ReportKind::Start => {
            has_fixed_identity
                && has_history_grown
                && after.progress == before.progress
                && after.result.is_none()
                && after.phase == super::model::Phase::Running
        }
        super::model::ReportKind::Progress | super::model::ReportKind::Checkpoint => {
            has_fixed_identity
                && has_history_grown
                && after.result == before.result
                && matches!(after.phase, super::model::Phase::Running | super::model::Phase::Transferring)
        }
        super::model::ReportKind::ResultReady => {
            has_fixed_identity
                && has_history_grown
                && after.progress == before.progress
                && after.phase == super::model::Phase::ResultReady
        }
        super::model::ReportKind::Failure => {
            has_fixed_identity
                && has_history_grown
                && after.result == before.result
                && after.phase == super::model::Phase::Failed
        }
        super::model::ReportKind::Completion => {
            has_fixed_identity
                && has_history_grown
                && after.progress == before.progress
                && after.result == before.result
                && after.phase == super::model::Phase::Completed
        }
    }
}
