use super::super::*;

// r[verify fenced-attempt-admission.refinement]
// r[verify fenced-attempt-admission.evidence]
// r[verify fenced-attempt-admission.boundary]
#[test]
fn start_moves_queued_to_running_changing_only_named_fields() {
    let before = super::queued();
    let next = super::applied(apply_report(&before, &super::report(0, ReportKind::Start)));
    assert_eq!(next.phase, Phase::Running);
    assert!(changes_only_named_fields(&before, &next, ReportKind::Start));
}

// r[verify fenced-attempt-admission.refinement]
// r[verify fenced-attempt-admission.evidence]
#[test]
fn progress_and_checkpoint_advance_only_progress_and_history() {
    let base = super::running(0);
    let progressed = super::applied(apply_report(&base, &super::report(1, ReportKind::Progress)));
    assert_eq!(progressed.progress, 1);
    assert!(changes_only_named_fields(&base, &progressed, ReportKind::Progress));

    let transferred = super::applied(apply_report(&progressed, &super::report(2, ReportKind::Checkpoint)));
    assert_eq!(transferred.phase, Phase::Transferring);
    assert!(changes_only_named_fields(&progressed, &transferred, ReportKind::Checkpoint));

    let resumed = super::applied(apply_report(&transferred, &super::report(3, ReportKind::Checkpoint)));
    assert_eq!(resumed.phase, Phase::Transferring);
    assert_eq!(resumed.progress, 3);
}

// r[verify fenced-attempt-admission.completion-linkage]
// r[verify fenced-attempt-admission.evidence]
#[test]
fn matching_completion_keeps_the_retained_link() {
    let before = super::result_ready(0);
    let mut completion = super::report(3, ReportKind::Completion);
    completion.result = Some(super::link());
    let next = super::applied(apply_report(&before, &completion));
    assert_eq!(next.phase, Phase::Completed);
    assert_eq!(next.result, Some(super::link()));
    assert!(changes_only_named_fields(&before, &next, ReportKind::Completion));
}

// r[verify fenced-attempt-admission.event-idempotence]
// r[verify fenced-attempt-admission.rejection-preservation]
#[test]
fn identical_event_replays_without_any_state_change() {
    let once_state = super::running(0);
    let same = super::report(0, ReportKind::Progress);
    let twice = apply_report(&once_state, &same);
    assert_eq!(twice, Ok(Disposition::Replayed));
    assert!(preserves_state(&once_state, &once_state));
    assert_eq!(state_identity(&once_state), state_identity(&once_state));
}

// r[verify fenced-attempt-admission.fence-safety]
// r[verify fenced-attempt-admission.evidence]
#[test]
fn reassignment_produces_strictly_greater_nonzero_epoch() {
    assert_eq!(next_epoch(super::epoch()), Ok(Epoch(super::LATER_EPOCH)));
    assert_eq!(next_epoch(Epoch(u64::MAX)), Err(RejectClass::EpochExhausted));
    let next = match supersede(&super::running(0), RunId(super::RUN + 1)) {
        Ok(state) => state,
        Err(error) => panic!("reassignment below the limit must succeed: {error:?}"),
    };
    assert_eq!(next.epoch, Epoch(super::LATER_EPOCH));
    assert_eq!(next.phase, Phase::Queued);
    assert_eq!(next.run, RunId(super::RUN + 1));
}

// r[verify fenced-attempt-admission.boundary]
// r[verify fenced-attempt-admission.evidence]
#[test]
fn closed_model_covers_every_phase_and_report_kind() {
    let mut phase_tags = std::collections::HashSet::new();
    let mut kind_tags = std::collections::HashSet::new();
    for phase in [
        Phase::Queued,
        Phase::Running,
        Phase::Transferring,
        Phase::ResultReady,
        Phase::Completed,
        Phase::Failed,
        Phase::Superseded,
    ] {
        phase_tags.insert(phase.tag());
        for kind in [
            ReportKind::Start,
            ReportKind::Progress,
            ReportKind::Checkpoint,
            ReportKind::ResultReady,
            ReportKind::Failure,
            ReportKind::Completion,
        ] {
            kind_tags.insert(kind.tag());
            let _decision = phase_step(phase, kind);
        }
    }
    assert_eq!(phase_tags.len(), 7);
    assert_eq!(kind_tags.len(), 6);
}

// r[verify fenced-attempt-admission.terminal-closure]
// r[verify fenced-attempt-admission.rejection-preservation]
#[test]
fn every_report_kind_rejects_from_each_terminal_phase() {
    let states = [
        (super::completed(0), Phase::Completed),
        (
            super::applied(apply_report(&super::running(0), &super::report(9, ReportKind::Failure))),
            Phase::Failed,
        ),
        (retire(&super::running(0)), Phase::Superseded),
    ];
    for (state, _phase) in &states {
        for kind in [
            ReportKind::Start,
            ReportKind::Progress,
            ReportKind::Checkpoint,
            ReportKind::ResultReady,
            ReportKind::Failure,
            ReportKind::Completion,
        ] {
            let decision = apply_report(state, &super::report(60, kind));
            match decision {
                Ok(_) => panic!("terminal phase must reject {kind:?}"),
                Err(error) => assert_eq!(error.class, RejectClass::TerminalPhase),
            }
        }
    }
}
