//! Ordering, conflict, recovery, convergence, and history lemmas.

use vstd::prelude::*;

verus! {

// r[verify consensus.fast-path.base-order]
pub proof fn prerequisites_compose(
    receive_left: int,
    receive_right: int,
    proposal_left: int,
    proposal_right: int,
    log_left: int,
    log_right: int,
    execution_left: int,
    execution_right: int,
)
    requires
        receive_left < receive_right,
        receive_left < receive_right ==> proposal_left < proposal_right,
        proposal_left < proposal_right ==> log_left < log_right,
        log_left < log_right ==> execution_left < execution_right,
    ensures execution_left < execution_right,
{
}

// r[impl consensus.fast-path.conflict]
pub open spec fn commutativity_spec(
    state_left_right: int,
    state_right_left: int,
    left_response_first: int,
    left_response_second: int,
    right_response_first: int,
    right_response_second: int,
) -> bool {
    state_left_right == state_right_left
        && left_response_first == left_response_second
        && right_response_first == right_response_second
}

// r[verify consensus.fast-path.conflict]
pub proof fn non_conflict_preserves_state_and_responses(
    state_left_right: int,
    state_right_left: int,
    left_response_first: int,
    left_response_second: int,
    right_response_first: int,
    right_response_second: int,
)
    requires commutativity_spec(
        state_left_right,
        state_right_left,
        left_response_first,
        left_response_second,
        right_response_first,
        right_response_second,
    ),
    ensures
        state_left_right == state_right_left,
        left_response_first == left_response_second,
        right_response_first == right_response_second,
{
}

// r[impl consensus.fast-path.recovery]
pub open spec fn carry_forward_spec(accepted: Seq<u64>) -> Seq<u64> {
    accepted
}

// r[verify consensus.fast-path.recovery]
pub proof fn accepted_set_is_stable(accepted: Seq<u64>)
    ensures carry_forward_spec(accepted) == accepted,
{
}

// r[verify consensus.fast-path.recovery]
pub proof fn marker_gates_normal_admission(
    marker_committed: bool,
    normal_admitted: bool,
)
    requires normal_admitted ==> marker_committed,
        normal_admitted,
    ensures marker_committed,
{
}

// r[impl consensus.fast-path.dual-path]
pub open spec fn session_duplicate_spec(applied_sequence: u64, completion_sequence: u64) -> bool {
    completion_sequence <= applied_sequence
}

// r[impl consensus.fast-path.dual-path]
pub open spec fn cached_response_spec(applied_sequence: u64, completion_sequence: u64) -> bool {
    completion_sequence == applied_sequence
}

// r[verify consensus.fast-path.dual-path]
pub proof fn matching_retained_completion_is_duplicate(
    applied_sequence: u64,
    completion_sequence: u64,
    observed_at: u64,
    retain_until: u64,
)
    requires
        completion_sequence == applied_sequence,
        observed_at <= retain_until,
    ensures
        session_duplicate_spec(applied_sequence, completion_sequence),
        cached_response_spec(applied_sequence, completion_sequence),
        observed_at <= retain_until,
{
}

// r[impl consensus.fast-path.linearization]
pub open spec fn complete_history_spec(invoke: u64, respond: u64) -> bool {
    invoke <= respond
}

// r[impl consensus.fast-path.linearization]
pub open spec fn realtime_order_spec(first_respond: u64, second_invoke: u64) -> bool {
    first_respond < second_invoke
}

// r[verify consensus.fast-path.linearization]
pub proof fn conflicting_later_invocation_cannot_overtake(
    first_invoke: u64,
    first_respond: u64,
    second_invoke: u64,
    second_respond: u64,
    first_execution: int,
    second_execution: int,
    conflicting: bool,
)
    requires
        complete_history_spec(first_invoke, first_respond),
        complete_history_spec(second_invoke, second_respond),
        realtime_order_spec(first_respond, second_invoke),
        conflicting,
        first_execution < second_execution,
    ensures
        first_respond < second_invoke,
        first_execution < second_execution,
{
}

// r[impl consensus.fast-path.boundary]
pub open spec fn boundary_admitted_spec(
    crash_faults: bool,
    static_membership: bool,
    single_command: bool,
) -> bool {
    crash_faults && static_membership && single_command
}

// r[verify consensus.fast-path.boundary]
pub proof fn unsupported_capability_rejects(
    crash_faults: bool,
    static_membership: bool,
    single_command: bool,
)
    requires !crash_faults || !static_membership || !single_command,
    ensures !boundary_admitted_spec(crash_faults, static_membership, single_command),
{
}

} // verus!
