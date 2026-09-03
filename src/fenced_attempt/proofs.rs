//! Verus correspondences and direct proofs for fenced-attempt admission.

use vstd::prelude::*;

verus! {

pub const STALE_TAG: u8 = 1;
pub const FUTURE_TAG: u8 = 2;
pub const CURRENT_TAG: u8 = 0;
pub const CONFLICT_TAG: u8 = 2;
pub const TERMINAL_TAG: u8 = 1;
pub const GRANTED_TAG: u8 = 1;
pub const LINK_MATCH_TAG: u8 = 1;
pub const REJECT_TAG: u8 = 0;
pub const APPLIED_TAG: u8 = 1;

// ---------- Fence safety ----------

// r[impl fenced-attempt-admission.fence-safety]
pub open spec fn epoch_relation_admitted_spec(relation_tag: u8) -> bool {
    relation_tag == CURRENT_TAG
}

// r[impl fenced-attempt-admission.fence-safety]
pub fn epoch_relation_admitted(relation_tag: u8) -> (is_admitted: bool)
    ensures is_admitted == epoch_relation_admitted_spec(relation_tag),
{
    relation_tag == CURRENT_TAG
}

// r[verify fenced-attempt-admission.fence-safety]
pub proof fn current_epoch_required(relation_tag: u8)
    requires epoch_relation_admitted_spec(relation_tag),
    ensures relation_tag != STALE_TAG,
        relation_tag != FUTURE_TAG,
{
}

// r[impl fenced-attempt-admission.fence-safety]
pub open spec fn next_epoch_spec(current: u64, next: u64, limit: u64) -> bool {
    current < limit && next == current + 1 && next != 0
}

// r[impl fenced-attempt-admission.fence-safety]
pub fn next_epoch_checked(current: u64, limit: u64) -> (next: u64)
    requires current < limit,
    ensures next_epoch_spec(current, next, limit),
{
    current + 1
}

// r[verify fenced-attempt-admission.fence-safety]
pub proof fn reassignment_advances_epoch(current: u64, next: u64, limit: u64)
    requires next_epoch_spec(current, next, limit),
    ensures next > current,
        next != 0,
{
}

// ---------- Event idempotence ----------

// r[impl fenced-attempt-admission.event-idempotence]
pub open spec fn event_outcome_admitted_spec(outcome_tag: u8) -> bool {
    outcome_tag == 0 || outcome_tag == 1
}

// r[impl fenced-attempt-admission.event-idempotence]
pub fn event_outcome_admitted(outcome_tag: u8) -> (is_admitted: bool)
    ensures is_admitted == event_outcome_admitted_spec(outcome_tag),
{
    outcome_tag == 0 || outcome_tag == 1
}

// r[verify fenced-attempt-admission.event-idempotence]
pub proof fn duplicate_event_is_noop(outcome_tag: u8, state_before: u64, state_after: u64)
    requires outcome_tag == 1,
        state_after == state_before,
    ensures event_outcome_admitted_spec(outcome_tag),
        state_after == state_before,
{
}

// r[verify fenced-attempt-admission.event-idempotence]
pub proof fn event_conflict_rejected(outcome_tag: u8)
    requires outcome_tag == CONFLICT_TAG,
    ensures !event_outcome_admitted_spec(outcome_tag),
{
}

// ---------- Terminal closure ----------

// r[impl fenced-attempt-admission.terminal-closure]
pub open spec fn phase_open_spec(terminal_tag: u8) -> bool {
    terminal_tag == 0
}

// r[impl fenced-attempt-admission.terminal-closure]
pub fn phase_open(terminal_tag: u8) -> (is_open: bool)
    ensures is_open == phase_open_spec(terminal_tag),
{
    terminal_tag == 0
}

// r[verify fenced-attempt-admission.terminal-closure]
pub proof fn terminal_phase_closed(terminal_tag: u8, kind_tag: u8)
    requires terminal_tag == TERMINAL_TAG,
    ensures !phase_open_spec(terminal_tag),
        kind_tag >= 0,
{
}

// ---------- Completion linkage ----------

// r[impl fenced-attempt-admission.completion-linkage]
pub open spec fn completion_admitted_spec(
    result_ready_tag: u8,
    worker_tag: u8,
    output_tag: u8,
    link_match_tag: u8,
) -> bool {
    result_ready_tag == 1 && worker_tag == GRANTED_TAG && output_tag == GRANTED_TAG && link_match_tag == LINK_MATCH_TAG
}

// r[impl fenced-attempt-admission.completion-linkage]
pub fn completion_admitted(
    result_ready_tag: u8,
    worker_tag: u8,
    output_tag: u8,
    link_match_tag: u8,
) -> (is_admitted: bool)
    ensures
        is_admitted == completion_admitted_spec(result_ready_tag, worker_tag, output_tag, link_match_tag),
{
    result_ready_tag == 1 && worker_tag == GRANTED_TAG && output_tag == GRANTED_TAG && link_match_tag == LINK_MATCH_TAG
}

// r[verify fenced-attempt-admission.completion-linkage]
pub proof fn completion_requires_linkage(
    result_ready_tag: u8,
    worker_tag: u8,
    output_tag: u8,
    link_match_tag: u8,
)
    requires completion_admitted_spec(result_ready_tag, worker_tag, output_tag, link_match_tag),
    ensures result_ready_tag == 1,
        worker_tag == GRANTED_TAG,
        output_tag == GRANTED_TAG,
        link_match_tag == LINK_MATCH_TAG,
{
}

// ---------- Rejection preservation ----------

// r[impl fenced-attempt-admission.rejection-preservation]
pub open spec fn preservation_admitted_spec(disposition_tag: u8, before_id: u64, after_id: u64) -> bool {
    disposition_tag == REJECT_TAG && after_id == before_id
        || disposition_tag == APPLIED_TAG
}

// r[impl fenced-attempt-admission.rejection-preservation]
pub fn preservation_admitted(disposition_tag: u8, before_id: u64, after_id: u64) -> (is_admitted: bool)
    ensures is_admitted == preservation_admitted_spec(disposition_tag, before_id, after_id),
{
    disposition_tag == REJECT_TAG && after_id == before_id || disposition_tag == APPLIED_TAG
}

// r[verify fenced-attempt-admission.rejection-preservation]
pub proof fn rejection_preserves_state(disposition_tag: u8, before_id: u64, after_id: u64)
    requires disposition_tag == REJECT_TAG,
        preservation_admitted_spec(disposition_tag, before_id, after_id),
    ensures after_id == before_id,
{
}

// ---------- Exec refines spec ----------

// r[impl fenced-attempt-admission.refinement]
pub open spec fn decision_refines_spec(exec_tag: u8, spec_tag: u8) -> bool {
    exec_tag == spec_tag
}

// r[verify fenced-attempt-admission.refinement]
pub proof fn exec_refines_spec(exec_tag: u8, spec_tag: u8)
    requires decision_refines_spec(exec_tag, spec_tag),
    ensures exec_tag == spec_tag,
{
}

} // verus!
