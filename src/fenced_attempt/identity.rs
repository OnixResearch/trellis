//! Verified finite fenced-attempt report admission.
//!
//! The module owns a closed phase, report, epoch, event, scope, and linkage
//! model with deterministic rejections and exact preservation facts. It owns no
//! transport, storage, crypto, clock, worker, sandbox, or runtime authority.
// r[impl fenced-attempt-admission.boundary]
// r[impl fenced-attempt-admission.evidence]
// r[impl fenced-attempt-admission.rejection-preservation]

mod decide;
mod model;
mod proofs;
mod relation;
mod transition;

pub use decide::apply_report;
pub use decide::changes_only_named_fields;
pub use decide::preserves_state;
pub use decide::state_identity;
pub use model::Disposition;
pub use model::Epoch;
pub use model::EventId;
pub use model::JobId;
pub use model::PayloadId;
pub use model::Phase;
pub use model::RejectClass;
pub use model::Rejection;
pub use model::Report;
pub use model::ReportKind;
pub use model::ResultLink;
pub use model::RunId;
pub use model::Scope;
pub use model::State;
pub use model::EPOCH_LIMIT;
pub use model::MAX_EVENTS;
pub use relation::classify_epoch;
pub use relation::classify_event;
pub use relation::identity_rejection;
pub use transition::authority_rejection;
pub use transition::next_epoch;
pub use transition::phase_step;
pub use transition::retained_result;
pub use transition::retire;
pub use transition::supersede;

#[cfg(test)]
mod tests;
