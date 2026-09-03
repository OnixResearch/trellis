//! Shared fixtures for fenced-attempt evidence.

mod negative;
mod positive;
mod property;

const JOB: u64 = 1;
const RUN: u64 = 10;
const EPOCH: u64 = 100;
const LATER_EPOCH: u64 = 101;
const LINK_PAYLOAD: u64 = 777;
const EVENT_BASE: u64 = 200;
const PAYLOAD_BASE: u64 = 300;

fn job() -> super::JobId {
    super::JobId(JOB)
}

fn run() -> super::RunId {
    super::RunId(RUN)
}

fn epoch() -> super::Epoch {
    super::Epoch(EPOCH)
}

fn link() -> super::ResultLink {
    super::ResultLink {
        run: run(),
        payload: super::PayloadId(LINK_PAYLOAD),
    }
}

fn event(offset: u64) -> super::EventId {
    super::EventId(EVENT_BASE + offset)
}

fn payload(offset: u64) -> super::PayloadId {
    super::PayloadId(PAYLOAD_BASE + offset)
}

fn report(offset: u64, kind: super::ReportKind) -> super::Report {
    super::Report {
        job: job(),
        run: run(),
        epoch: epoch(),
        event: event(offset),
        payload: payload(offset),
        kind,
        worker: super::Scope::Granted,
        output: super::Scope::Granted,
        result: None,
    }
}

fn queued() -> super::State {
    super::State::initial(job(), run(), epoch())
}
fn applied(decision: Result<super::Disposition, super::Rejection>) -> super::State {
    match decision {
        Ok(super::Disposition::Applied(state)) => state,
        Ok(super::Disposition::Replayed) => panic!("expected an applied disposition"),
        Err(error) => panic!("expected an applied disposition, got {error:?}"),
    }
}

fn running(offset: u64) -> super::State {
    applied(super::apply_report(&queued(), &report(offset, super::ReportKind::Start)))
}

fn result_ready(offset: u64) -> super::State {
    let mut ready = report(offset + 2, super::ReportKind::ResultReady);
    ready.result = Some(link());
    applied(super::apply_report(&running(offset), &ready))
}

fn completed(offset: u64) -> super::State {
    let mut completion = report(offset + 3, super::ReportKind::Completion);
    completion.result = Some(link());
    applied(super::apply_report(&result_ready(offset), &completion))
}
