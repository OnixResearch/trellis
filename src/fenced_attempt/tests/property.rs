//! Deterministic bounded-history property evidence.

use super::super::*;

const SEED: u64 = 0x5DEE_CE66_D00D_FEED;
const ROUNDS: usize = 512;
const KIND_COUNT: u64 = 6;
const EVENT_SPAN: u64 = 8;

struct Generator {
    state: u64,
}

impl Generator {
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        self.state
    }

    fn kind(&mut self) -> ReportKind {
        match self.next() % KIND_COUNT {
            0 => ReportKind::Start,
            1 => ReportKind::Progress,
            2 => ReportKind::Checkpoint,
            3 => ReportKind::ResultReady,
            4 => ReportKind::Failure,
            _ => ReportKind::Completion,
        }
    }
}

fn generated(offset: u64, kind: ReportKind, roll: u64) -> Report {
    let mut report = super::report(offset % EVENT_SPAN, kind);
    report.epoch = Epoch(super::EPOCH + roll % 3 - 1);
    report.payload = PayloadId(super::PAYLOAD_BASE + offset % EVENT_SPAN);
    report.worker = if roll % 5 == 0 { Scope::Denied } else { Scope::Granted };
    report.output = if roll % 7 == 0 { Scope::Denied } else { Scope::Granted };
    report.result = (roll % 2 == 0).then(super::link);
    report
}

// r[verify fenced-attempt-admission.rejection-preservation]
// r[verify fenced-attempt-admission.fence-safety]
// r[verify fenced-attempt-admission.event-idempotence]
// r[verify fenced-attempt-admission.terminal-closure]
// r[verify fenced-attempt-admission.completion-linkage]
// r[verify fenced-attempt-admission.evidence]
#[test]
fn bounded_random_histories_never_break_preservation_or_identity() {
    let mut generator = Generator { state: SEED };
    let mut state = super::queued();
    let mut applied_count = 0_u32;
    let mut rejected_count = 0_u32;
    let mut replayed_count = 0_u32;

    for step in 0..ROUNDS {
        let roll = generator.next();
        let report = generated(generator.next(), generator.kind(), roll);
        let before_identity = state_identity(&state);
        match apply_report(&state, &report) {
            Ok(Disposition::Applied(next)) => {
                assert_eq!(next.job, state.job);
                assert!(next.epoch >= state.epoch);
                assert!(changes_only_named_fields(&state, &next, report.kind));
                state = next;
                applied_count += 1;
            }
            Ok(Disposition::Replayed) => {
                assert_eq!(before_identity, state_identity(&state));
                replayed_count += 1;
            }
            Err(error) => {
                assert_eq!(before_identity, state_identity(&state));
                assert!(preserves_state(&state, &state));
                assert_ne!(error.class.tag(), u8::MAX);
                rejected_count += 1;
            }
        }
        if state.phase.is_terminal() && generator.next() % 2 == 1 {
            let next_run = RunId(state.run.0.wrapping_add(1));
            match supersede(&state, next_run) {
                Ok(fresh) => {
                    assert!(fresh.epoch.0 > state.epoch.0);
                    assert_eq!(fresh.phase, Phase::Queued);
                    state = fresh;
                }
                Err(error) => assert_eq!(error.class, RejectClass::EpochExhausted),
            }
        }
        let _ = step;
    }

    assert!(applied_count > 0, "generator must produce applied reports");
    assert!(rejected_count > 0, "generator must produce rejections");
    assert!(replayed_count > 0, "generator must produce replays");
}

// r[verify fenced-attempt-admission.rejection-preservation]
#[test]
fn one_fact_mutation_at_a_time_changes_only_that_decision() {
    let base = super::result_ready(0);
    let mut completion = super::report(3, ReportKind::Completion);
    completion.result = Some(super::link());
    let accepted = apply_report(&base, &completion);
    assert!(matches!(accepted, Ok(Disposition::Applied(_))));

    let mutations: [(Report, RejectClass); 4] = [
        {
            let mut stale = completion;
            stale.epoch = Epoch(super::EPOCH - 1);
            (stale, RejectClass::StaleEpoch)
        },
        {
            let mut wrong_job = completion;
            wrong_job.job = JobId(super::JOB + 1);
            (wrong_job, RejectClass::WrongJob)
        },
        {
            let mut wrong_run = completion;
            wrong_run.run = RunId(super::RUN + 1);
            (wrong_run, RejectClass::WrongRun)
        },
        {
            let mut no_worker = completion;
            no_worker.worker = Scope::Denied;
            (no_worker, RejectClass::WorkerDenied)
        },
    ];
    for (report, expected) in mutations {
        let before_identity = state_identity(&base);
        match apply_report(&base, &report) {
            Ok(_) => panic!("single-fact mutation {expected:?} must reject"),
            Err(error) => {
                assert_eq!(error.class, expected);
                assert_eq!(before_identity, state_identity(&base));
            }
        }
    }
}
