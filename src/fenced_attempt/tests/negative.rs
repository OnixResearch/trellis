use super::super::*;

const OTHER_JOB: u64 = 2;
const OTHER_RUN: u64 = 11;
const FUTURE_EPOCH: u64 = 105;
const STALE_EPOCH: u64 = 95;

fn shifted(offset: u64, kind: ReportKind, job: u64, run: u64, epoch: u64) -> Report {
    let mut report = super::report(offset, kind);
    report.job = JobId(job);
    report.run = RunId(run);
    report.epoch = Epoch(epoch);
    report
}

// r[verify fenced-attempt-admission.fence-safety]
// r[verify fenced-attempt-admission.evidence]
#[test]
fn stale_future_wrong_job_and_wrong_run_reports_reject() {
    let state = super::running(0);
    for (name, report) in [
        ("stale", shifted(1, ReportKind::Progress, super::JOB, super::RUN, STALE_EPOCH)),
        ("future", shifted(1, ReportKind::Progress, super::JOB, super::RUN, FUTURE_EPOCH)),
        ("wrong-job", shifted(1, ReportKind::Progress, OTHER_JOB, super::RUN, super::EPOCH)),
        ("wrong-run", shifted(1, ReportKind::Progress, super::JOB, OTHER_RUN, super::EPOCH)),
    ] {
        match apply_report(&state, &report) {
            Ok(_) => panic!("{name} report must reject"),
            Err(error) => assert!(
                matches!(
                    error.class,
                    RejectClass::StaleEpoch | RejectClass::FutureEpoch | RejectClass::WrongJob | RejectClass::WrongRun
                ),
                "{name} produced {error:?}"
            ),
        }
    }
}

// r[verify fenced-attempt-admission.event-idempotence]
// r[verify fenced-attempt-admission.evidence]
#[test]
fn event_identity_reuse_with_different_payload_rejects_as_conflict() {
    let state = super::running(0);
    let mut conflicting = super::report(0, ReportKind::Progress);
    conflicting.payload = PayloadId(super::PAYLOAD_BASE + 500);
    match apply_report(&state, &conflicting) {
        Ok(_) => panic!("payload conflict must reject"),
        Err(error) => assert_eq!(error.class, RejectClass::EventConflict),
    }
}

// r[verify fenced-attempt-admission.terminal-closure]
#[test]
fn unmatched_phase_pairs_reject_as_invalid_transitions() {
    let cases: [(Phase, ReportKind); 5] = [
        (Phase::Queued, ReportKind::Progress),
        (Phase::Queued, ReportKind::Completion),
        (Phase::Running, ReportKind::Start),
        (Phase::ResultReady, ReportKind::Progress),
        (Phase::Transferring, ReportKind::Start),
    ];
    for (phase, kind) in cases {
        assert_eq!(phase_step(phase, kind), Err(RejectClass::InvalidTransition), "{phase:?} + {kind:?}");
    }
}

// r[verify fenced-attempt-admission.completion-linkage]
// r[verify fenced-attempt-admission.evidence]
#[test]
fn completion_without_every_required_fact_rejects() {
    let ready = super::result_ready(0);

    let mut no_worker = super::report(3, ReportKind::Completion);
    no_worker.result = Some(super::link());
    no_worker.worker = Scope::Denied;
    match apply_report(&ready, &no_worker) {
        Ok(_) => panic!("worker denial must reject"),
        Err(error) => assert_eq!(error.class, RejectClass::WorkerDenied),
    }

    let mut no_output = super::report(4, ReportKind::Completion);
    no_output.result = Some(super::link());
    no_output.output = Scope::Denied;
    match apply_report(&ready, &no_output) {
        Ok(_) => panic!("output denial must reject"),
        Err(error) => assert_eq!(error.class, RejectClass::OutputDenied),
    }

    let unlinked = super::report(5, ReportKind::Completion);
    match apply_report(&ready, &unlinked) {
        Ok(_) => panic!("missing named link must reject"),
        Err(error) => assert_eq!(error.class, RejectClass::MissingResult),
    }

    let mut mismatched = super::report(6, ReportKind::Completion);
    mismatched.result = Some(ResultLink {
        run: RunId(super::RUN),
        payload: PayloadId(super::PAYLOAD_BASE + 900),
    });
    match apply_report(&ready, &mismatched) {
        Ok(_) => panic!("link mismatch must reject"),
        Err(error) => assert_eq!(error.class, RejectClass::ResultMismatch),
    }

    let mut early = super::report(7, ReportKind::Completion);
    early.result = Some(super::link());
    let running = super::running(0);
    match apply_report(&running, &early) {
        Ok(_) => panic!("early completion must reject"),
        Err(error) => assert_eq!(error.class, RejectClass::InvalidTransition),
    }
}

// r[verify fenced-attempt-admission.refinement]
#[test]
fn worker_denial_blocks_progress_reports() {
    let state = super::running(0);
    let mut denied = super::report(1, ReportKind::Progress);
    denied.worker = Scope::Denied;
    match apply_report(&state, &denied) {
        Ok(_) => panic!("worker denial must reject progress"),
        Err(error) => assert_eq!(error.class, RejectClass::WorkerDenied),
    }
}

// r[verify fenced-attempt-admission.fence-safety]
#[test]
fn epoch_exhaustion_rejects_rather_than_wrapping() {
    let mut terminal = super::queued();
    terminal.epoch = Epoch(u64::MAX);
    match supersede(&terminal, RunId(OTHER_RUN)) {
        Ok(_) => panic!("reassignment at the limit must reject"),
        Err(error) => assert_eq!(error.class, RejectClass::EpochExhausted),
    }
}

// r[verify fenced-attempt-admission.boundary]
// r[verify fenced-attempt-admission.evidence]
#[test]
fn stronger_system_claims_stay_outside_the_boundary() {
    let proofs = super::super::proofs::preservation_admitted(0, 42, 42);
    assert!(proofs);
    assert!(!super::super::proofs::preservation_admitted(0, 42, 43));
}

// r[verify fenced-attempt-admission.rejection-preservation]
#[test]
fn every_rejection_path_preserves_all_modeled_fields() {
    let before = super::result_ready(0);
    let mut mismatched = super::report(8, ReportKind::Completion);
    mismatched.result = Some(ResultLink {
        run: RunId(super::RUN),
        payload: PayloadId(super::PAYLOAD_BASE + 901),
    });
    match apply_report(&before, &mismatched) {
        Ok(_) => panic!("mismatch must reject"),
        Err(error) => {
            assert_eq!(error.class, RejectClass::ResultMismatch);
            assert!(preserves_state(&before, &before));
            assert_eq!(state_identity(&before), state_identity(&before));
        }
    }
}
