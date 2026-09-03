//! Epoch and event-history classification over normalized facts.

/// Epoch ordering relative to the modeled current epoch.
// r[impl fenced-attempt-admission.fence-safety]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EpochStance {
    Current,
    Stale,
    Future,
}

impl EpochStance {
    /// Returns the finite stance tag used by verified correspondences.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::Current => super::proofs::CURRENT_TAG,
            Self::Stale => super::proofs::STALE_TAG,
            Self::Future => super::proofs::FUTURE_TAG,
        }
    }
}

/// Event-history classification for one incoming identity pair.
// r[impl fenced-attempt-admission.event-idempotence]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventOutcome {
    New,
    Replayed,
    Conflict,
}

impl EventOutcome {
    /// Returns the finite outcome tag used by verified correspondences.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::New => 0,
            Self::Replayed => 1,
            Self::Conflict => super::proofs::CONFLICT_TAG,
        }
    }
}

/// Classifies one report epoch against the modeled epoch.
// r[impl fenced-attempt-admission.fence-safety]
// r[impl fenced-attempt-admission.refinement]
#[must_use]
pub fn classify_epoch(current: super::model::Epoch, observed: super::model::Epoch) -> EpochStance {
    if observed.0 < current.0 {
        EpochStance::Stale
    } else if observed.0 == current.0 {
        EpochStance::Current
    } else {
        EpochStance::Future
    }
}

/// Classifies one event identity against the modeled history.
// r[impl fenced-attempt-admission.event-idempotence]
// r[impl fenced-attempt-admission.refinement]
#[must_use]
pub fn classify_event(
    history: &[(super::model::EventId, super::model::PayloadId)],
    event: super::model::EventId,
    payload: super::model::PayloadId,
) -> EventOutcome {
    match history.iter().find(|(known, _)| *known == event) {
        None => EventOutcome::New,
        Some((_, known_payload)) if *known_payload == payload => EventOutcome::Replayed,
        Some(_) => EventOutcome::Conflict,
    }
}

/// Returns the first identity rejection class for one report, if any.
// r[impl fenced-attempt-admission.fence-safety]
#[must_use]
pub fn identity_rejection(
    state: &super::model::State,
    report: &super::model::Report,
) -> Option<super::model::RejectClass> {
    let stance_tag = classify_epoch(state.epoch, report.epoch).tag();
    if stance_tag == super::proofs::FUTURE_TAG {
        return Some(super::model::RejectClass::FutureEpoch);
    }
    if !super::proofs::epoch_relation_admitted(stance_tag) {
        return Some(super::model::RejectClass::StaleEpoch);
    }
    if report.job != state.job {
        return Some(super::model::RejectClass::WrongJob);
    }
    if report.run != state.run {
        return Some(super::model::RejectClass::WrongRun);
    }
    None
}
