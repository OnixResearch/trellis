//! Verified terminal-state recovery for durable execution.
//!
//! A durable terminal event fixes the recovered outcome. Recovery reads the
//! last event in the valid durable prefix and never reopens a terminal run.

use vstd::prelude::*;

use crate::durable_history::{
    durable_prefix_len, DurableEvent, EVENT_CANCELLED, EVENT_COMPLETED, EVENT_FAILED,
};

verus! {

pub open spec fn terminal_status_spec(events: Seq<DurableEvent>) -> Option<u8> {
    let prefix = crate::durable_history::durable_prefix_len_spec(events);
    if prefix == 0 {
        None
    } else {
        let last = events[prefix - 1];
        if crate::durable_history::is_terminal_kind_spec(last.kind) {
            Some(last.kind)
        } else {
            None
        }
    }
}

// r[depends durable-execution.terminal]
/// Spec: execution is terminal when the durable prefix ends in a terminal event.
pub open spec fn is_terminal_execution_spec(events: Seq<DurableEvent>) -> bool {
    match terminal_status_spec(events) {
        Some(_) => true,
        None => false,
    }
}

// r[impl durable-execution.terminal]
/// Check whether durable history already fixes a terminal outcome.
pub fn is_terminal_execution(events: &[DurableEvent]) -> (result: bool)
    ensures result == is_terminal_execution_spec(events@)
{
    recovered_status(events).is_some()
}

// r[depends durable-execution.terminal-recovery]
/// Spec: recovered terminal status, if one exists.
pub open spec fn recovered_status_spec(events: Seq<DurableEvent>) -> Option<u8> {
    terminal_status_spec(events)
}

// r[impl durable-execution.terminal-recovery]
/// Recover the durable terminal status, if one exists.
pub fn recovered_status(events: &[DurableEvent]) -> (result: Option<u8>)
    ensures result == recovered_status_spec(events@)
{
    let prefix = durable_prefix_len(events);
    proof {
        crate::durable_history::durable_prefix_bounded(events@);
        assert(prefix <= events@.len());
    }
    if prefix == 0 {
        None
    } else {
        let last = events[prefix - 1];
        if last.kind == EVENT_COMPLETED || last.kind == EVENT_FAILED || last.kind == EVENT_CANCELLED {
            Some(last.kind)
        } else {
            None
        }
    }
}


} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::durable_history::{DurableEvent, EVENT_CANCELLED, EVENT_COMPLETED, EVENT_WORK};

    fn work(seq_num: usize) -> DurableEvent {
        DurableEvent { seq_num, kind: EVENT_WORK, effect_id: 0 }
    }

    fn completed(seq_num: usize) -> DurableEvent {
        DurableEvent { seq_num, kind: EVENT_COMPLETED, effect_id: 0 }
    }

    fn cancelled(seq_num: usize) -> DurableEvent {
        DurableEvent { seq_num, kind: EVENT_CANCELLED, effect_id: 0 }
    }

    // r[verify durable-execution.terminal]
    #[test]
    fn test_is_terminal_execution() {
        assert!(!is_terminal_execution(&[]));
        assert!(!is_terminal_execution(&[work(0), work(1)]));
        assert!(is_terminal_execution(&[work(0), completed(1)]));
        assert!(is_terminal_execution(&[cancelled(0)]));
    }

    // r[verify durable-execution.terminal-recovery]
    #[test]
    fn test_recovered_status() {
        assert_eq!(recovered_status(&[]), None);
        assert_eq!(recovered_status(&[work(0)]), None);
        assert_eq!(recovered_status(&[work(0), completed(1)]), Some(EVENT_COMPLETED));
        assert_eq!(recovered_status(&[cancelled(0)]), Some(EVENT_CANCELLED));
    }

    proptest! {
        // r[verify durable-execution.terminal]
        #[test]
        fn prop_completed_prefix_is_terminal(prefix in 0usize..20) {
            let mut events: Vec<DurableEvent> = (0..prefix).map(work).collect();
            events.push(completed(prefix));
            prop_assert!(is_terminal_execution(&events));
        }

        // r[verify durable-execution.terminal-recovery]
        #[test]
        fn prop_terminal_status_matches_last_terminal(prefix in 0usize..20) {
            let mut events: Vec<DurableEvent> = (0..prefix).map(work).collect();
            events.push(completed(prefix));
            prop_assert_eq!(recovered_status(&events), Some(EVENT_COMPLETED));
        }
    }
}
