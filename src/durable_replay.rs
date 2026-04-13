//! Verified replay decisions for durable execution.
//!
//! Replay reads only the contiguous durable prefix. Durable effect markers
//! suppress fresh execution for the same `(seq_num, effect_id)` pair.

use vstd::prelude::*;

use crate::durable_history::{durable_prefix_len, DurableEvent, EVENT_EFFECT_DONE};

verus! {

// r[depends durable-execution.replayable-event]
/// Spec: replay returns an event only inside the durable prefix.
pub open spec fn replayable_event_at_spec(events: Seq<DurableEvent>, index: int) -> Option<DurableEvent> {
    if 0 <= index < crate::durable_history::durable_prefix_len_spec(events) {
        Some(events[index])
    } else {
        None
    }
}

// r[impl durable-execution.replayable-event]
/// Return the replayable durable event at `index`, if any.
pub fn replayable_event_at(events: &[DurableEvent], index: usize) -> (result: Option<DurableEvent>)
    ensures result == replayable_event_at_spec(events@, index as int)
{
    let prefix = durable_prefix_len(events);
    proof {
        crate::durable_history::durable_prefix_bounded(events@);
        assert(prefix <= events@.len());
    }
    if index < prefix {
        Some(events[index])
    } else {
        None
    }
}

pub open spec fn effect_completed_spec(events: Seq<DurableEvent>, seq_num: usize, effect_id: u64) -> bool {
    exists|i: int| {
        &&& 0 <= i < crate::durable_history::durable_prefix_len_spec(events)
        &&& events[i].seq_num == seq_num
        &&& events[i].kind == EVENT_EFFECT_DONE
        &&& events[i].effect_id == effect_id
    }
}

// r[depends durable-execution.effect-dedup]
/// Spec: an effect should execute only if no matching durable completion exists.
pub open spec fn should_execute_effect_spec(events: Seq<DurableEvent>, seq_num: usize, effect_id: u64) -> bool {
    !effect_completed_spec(events, seq_num, effect_id)
}

// r[impl durable-execution.effect-dedup]
/// Decide whether a fresh effect execution is still allowed.
pub fn should_execute_effect(events: &[DurableEvent], seq_num: usize, effect_id: u64) -> (result: bool)
    ensures result == should_execute_effect_spec(events@, seq_num, effect_id)
{
    let prefix = durable_prefix_len(events);
    let mut i: usize = 0;
    proof {
        crate::durable_history::durable_prefix_bounded(events@);
        assert(prefix <= events@.len());
    }
    while i < prefix
        invariant
            0 <= i <= prefix,
            prefix == crate::durable_history::durable_prefix_len_spec(events@),
            prefix <= events@.len(),
            forall|k: int| 0 <= k < i as int ==> !(events@[k].seq_num == seq_num
                && events@[k].kind == EVENT_EFFECT_DONE
                && events@[k].effect_id == effect_id),
        decreases prefix - i,
    {
        if events[i].seq_num == seq_num
            && events[i].kind == EVENT_EFFECT_DONE
            && events[i].effect_id == effect_id
        {
            return false;
        }
        i = i + 1;
    }
    true
}


} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::durable_history::{DurableEvent, EVENT_CANCELLED, EVENT_EFFECT_DONE, EVENT_WORK};

    fn work(seq_num: usize) -> DurableEvent {
        DurableEvent { seq_num, kind: EVENT_WORK, effect_id: 0 }
    }

    fn effect(seq_num: usize, effect_id: u64) -> DurableEvent {
        DurableEvent { seq_num, kind: EVENT_EFFECT_DONE, effect_id }
    }

    // r[verify durable-execution.replayable-event]
    #[test]
    fn test_replayable_event_at() {
        let events = vec![work(0), effect(1, 9), DurableEvent { seq_num: 2, kind: EVENT_CANCELLED, effect_id: 0 }];
        assert_eq!(replayable_event_at(&events, 0), Some(work(0)));
        assert_eq!(replayable_event_at(&events, 1), Some(effect(1, 9)));
        assert_eq!(replayable_event_at(&events, 2), Some(DurableEvent { seq_num: 2, kind: EVENT_CANCELLED, effect_id: 0 }));
        assert_eq!(replayable_event_at(&events, 3), None);
    }

    // r[verify durable-execution.effect-dedup]
    #[test]
    fn test_should_execute_effect() {
        let events = vec![work(0), effect(1, 42)];
        assert!(should_execute_effect(&events, 0, 42));
        assert!(!should_execute_effect(&events, 1, 42));
        assert!(should_execute_effect(&events, 1, 99));
    }

    proptest! {
        // r[verify durable-execution.effect-dedup]
        #[test]
        fn prop_matching_marker_blocks(effect_id in 0u64..1000) {
            let events = vec![work(0), effect(1, effect_id)];
            prop_assert!(!should_execute_effect(&events, 1, effect_id));
        }

        // r[verify durable-execution.replayable-event]
        #[test]
        fn prop_replay_outside_prefix_rejected(len in 0usize..30) {
            let mut events: Vec<DurableEvent> = (0..len).map(work).collect();
            events.push(DurableEvent { seq_num: len + 1, kind: EVENT_WORK, effect_id: 0 });
            let prefix = durable_prefix_len(&events);
            prop_assert_eq!(replayable_event_at(&events, prefix), None);
        }
    }
}
