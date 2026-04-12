//! Object-level consistency predicates.
//!
//! Reuses the existing trace vocabulary and sequential-trace checker shape from
//! the Raft linearizability module, but packages the object-model hierarchy in
//! a small standalone API.

use vstd::prelude::*;

use crate::trace_relation::{TraceEvent, EVENT_CLIENT_REQUEST, EVENT_CLIENT_RESPONSE};

verus! {

/// Helper: sequential witness check from pair index `from` onward.
pub open spec fn sequential_witness_from_spec(trace: Seq<TraceEvent>, from: int) -> bool
    decreases trace.len() / 2 - from
{
    if trace.len() % 2 != 0 {
        false
    } else if from >= trace.len() / 2 {
        true
    } else {
        trace[2 * from].event_type == EVENT_CLIENT_REQUEST
        && trace[2 * from + 1].event_type == EVENT_CLIENT_RESPONSE
        && trace[2 * from].uid == trace[2 * from + 1].uid
        && sequential_witness_from_spec(trace, from + 1)
    }
}

proof fn sequential_prefix_to_start(trace: Seq<TraceEvent>, upto: int)
    requires
        trace.len() % 2 == 0,
        0 <= upto <= trace.len() / 2,
        forall|j: int| #![trigger trace[2 * j]]
            0 <= j < upto ==>
                trace[2 * j].event_type == EVENT_CLIENT_REQUEST
                && trace[2 * j + 1].event_type == EVENT_CLIENT_RESPONSE
                && trace[2 * j].uid == trace[2 * j + 1].uid,
        sequential_witness_from_spec(trace, upto),
    ensures sequential_witness_spec(trace)
    decreases upto,
{
    if upto > 0 {
        sequential_prefix_to_start(trace, upto - 1);
    }
}

proof fn sequential_tail_implies_pair(trace: Seq<TraceEvent>, from: int, pair_idx: int)
    requires
        trace.len() % 2 == 0,
        0 <= from <= pair_idx < trace.len() / 2,
        sequential_witness_from_spec(trace, from),
    ensures
        trace[2 * pair_idx].event_type == EVENT_CLIENT_REQUEST,
        trace[2 * pair_idx + 1].event_type == EVENT_CLIENT_RESPONSE,
        trace[2 * pair_idx].uid == trace[2 * pair_idx + 1].uid,
    decreases pair_idx - from,
{
    if from < pair_idx {
        sequential_tail_implies_pair(trace, from + 1, pair_idx);
    }
}

// r[depends consistency-object.sequential-witness]
/// Spec: a trace is a sequential witness when all pairs form matching
/// request/response steps.
pub open spec fn sequential_witness_spec(trace: Seq<TraceEvent>) -> bool {
    sequential_witness_from_spec(trace, 0)
}

// r[impl consistency-object.sequential-witness]
/// Runtime checker for the alternating request/response witness shape.
pub fn sequential_witness(trace: &[TraceEvent]) -> (result: bool)
    ensures result == sequential_witness_spec(trace@)
{
    if trace.len() % 2 != 0 {
        return false;
    }
    let mut k: usize = 0;
    while k < trace.len() / 2
        invariant
            0 <= k <= trace@.len() / 2,
            trace@.len() % 2 == 0,
            forall|j: int| #![trigger trace@[2 * j]]
                0 <= j < k as int ==>
                    trace@[2 * j].event_type == EVENT_CLIENT_REQUEST
                    && trace@[2 * j + 1].event_type == EVENT_CLIENT_RESPONSE
                    && trace@[2 * j].uid == trace@[2 * j + 1].uid,
        decreases trace@.len() / 2 - k,
    {
        let req = &trace[2 * k];
        let resp = &trace[2 * k + 1];
        if req.event_type != EVENT_CLIENT_REQUEST
            || resp.event_type != EVENT_CLIENT_RESPONSE
            || req.uid != resp.uid
        {
            proof {
                if sequential_witness_spec(trace@) {
                    sequential_tail_implies_pair(trace@, 0, k as int);
                    assert(trace@[2 * (k as int)].event_type == EVENT_CLIENT_REQUEST);
                }
            }
            return false;
        }
        k += 1;
    }
    proof {
        assert(sequential_witness_from_spec(trace@, k as int));
        sequential_prefix_to_start(trace@, k as int);
    }
    true
}

// r[depends consistency-object.sequential-consistency]
/// Spec: sequential consistency requires a sequential witness and preserved
/// program order.
pub open spec fn sequential_consistency_spec(
    sequential_witness: bool,
    program_order_respected: bool,
) -> bool {
    sequential_witness && program_order_respected
}

// r[impl consistency-object.sequential-consistency]
pub fn sequential_consistency(
    sequential_witness: bool,
    program_order_respected: bool,
) -> (result: bool)
    ensures result == sequential_consistency_spec(sequential_witness, program_order_respected)
{
    sequential_witness && program_order_respected
}

// r[depends consistency-object.linearizability]
/// Spec: linearizability is sequential consistency plus real-time respect.
pub open spec fn linearizability_spec(
    sequential_witness: bool,
    program_order_respected: bool,
    realtime_respected: bool,
) -> bool {
    sequential_consistency_spec(sequential_witness, program_order_respected)
    && realtime_respected
}

// r[impl consistency-object.linearizability]
pub fn linearizability(
    sequential_witness: bool,
    program_order_respected: bool,
    realtime_respected: bool,
) -> (result: bool)
    ensures result == linearizability_spec(
        sequential_witness,
        program_order_respected,
        realtime_respected,
    )
{
    sequential_consistency(sequential_witness, program_order_respected)
        && realtime_respected
}

// r[verify consistency-object.linearizability]
/// Linearizability implies sequential consistency.
proof fn linearizability_implies_sequential_consistency(
    sequential_witness: bool,
    program_order_respected: bool,
    realtime_respected: bool,
)
    ensures linearizability_spec(
        sequential_witness,
        program_order_respected,
        realtime_respected,
    ) ==> sequential_consistency_spec(sequential_witness, program_order_respected)
{}

} // verus!

#[cfg(test)]
mod tests {
    use super::*;

    fn req(uid: u64, cmd: u64) -> TraceEvent {
        TraceEvent {
            event_type: EVENT_CLIENT_REQUEST,
            uid,
            command: cmd,
            result: 0,
        }
    }

    fn resp(uid: u64, result: u64) -> TraceEvent {
        TraceEvent {
            event_type: EVENT_CLIENT_RESPONSE,
            uid,
            command: 0,
            result,
        }
    }

    // r[verify consistency-object.sequential-witness]
    #[test]
    fn test_sequential_witness_accepts_alternating_trace() {
        let trace = [req(1, 10), resp(1, 11), req(2, 20), resp(2, 21)];
        assert!(sequential_witness(&trace));
    }

    // r[verify consistency-object.sequential-witness]
    #[test]
    fn test_sequential_witness_rejects_broken_trace() {
        let trace = [req(1, 10), req(2, 20), resp(2, 21)];
        assert!(!sequential_witness(&trace));
    }

    // r[verify consistency-object.sequential-consistency]
    #[test]
    fn test_sequential_consistency_requires_program_order() {
        assert!(sequential_consistency(true, true));
        assert!(!sequential_consistency(true, false));
    }

    // r[verify consistency-object.linearizability]
    #[test]
    fn test_linearizability_adds_realtime() {
        assert!(linearizability(true, true, true));
        assert!(!linearizability(true, true, false));
    }
}
