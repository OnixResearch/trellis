//! Verified checkpoint and resume decisions for durable execution.
//!
//! Checkpoints advance only within the contiguous durable prefix. Recovery
//! resumes at the first not-yet-integrated durable step, unless durable
//! history already fixes a terminal outcome.

use vstd::prelude::*;

use crate::durable_history::{durable_prefix_len, DurableEvent};

verus! {

pub open spec fn checkpoint_valid_spec(old_checkpoint: usize, events: Seq<DurableEvent>) -> bool {
    old_checkpoint <= crate::durable_history::durable_prefix_len_spec(events)
}

// r[depends durable-execution.checkpoint-advance]
// r[impl durable-execution.checkpoint-prefix]
/// Spec: checkpoint advances to the durable prefix when the old checkpoint is valid.
pub open spec fn advance_checkpoint_spec(old_checkpoint: usize, events: Seq<DurableEvent>) -> usize {
    let prefix = crate::durable_history::durable_prefix_len_spec(events);
    if old_checkpoint <= prefix {
        prefix as usize
    } else {
        old_checkpoint
    }
}

// r[impl durable-execution.checkpoint-advance]
// r[impl durable-execution.checkpoint-prefix]
/// Advance the checkpoint as far as the contiguous durable prefix allows.
pub fn advance_checkpoint(old_checkpoint: usize, events: &[DurableEvent]) -> (result: usize)
    ensures result == advance_checkpoint_spec(old_checkpoint, events@)
{
    let prefix = durable_prefix_len(events);
    if old_checkpoint <= prefix {
        prefix
    } else {
        old_checkpoint
    }
}

// r[depends durable-execution.resume-step]
// r[impl durable-execution.resume-bounds]
/// Spec: recovery resumes at the durable prefix, unless the run is terminal.
pub open spec fn resume_step_spec(old_checkpoint: usize, events: Seq<DurableEvent>) -> Option<usize> {
    match crate::durable_terminal::recovered_status_spec(events) {
        Some(_) => None,
        None => Some(advance_checkpoint_spec(old_checkpoint, events)),
    }
}

// r[impl durable-execution.resume-step]
// r[impl durable-execution.resume-bounds]
/// Return the next resumable step, or `None` if durable history is terminal.
pub fn resume_step(old_checkpoint: usize, events: &[DurableEvent]) -> (result: Option<usize>)
    ensures result == resume_step_spec(old_checkpoint, events@)
{
    if crate::durable_terminal::is_terminal_execution(events) {
        None
    } else {
        Some(advance_checkpoint(old_checkpoint, events))
    }
}


} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::durable_history::{DurableEvent, EVENT_CANCELLED, EVENT_WORK};

    fn work(seq_num: usize) -> DurableEvent {
        DurableEvent { seq_num, kind: EVENT_WORK, effect_id: 0 }
    }

    // r[verify durable-execution.checkpoint-advance]
    #[test]
    fn test_advance_checkpoint() {
        let events = vec![work(0), work(1), work(2)];
        assert_eq!(advance_checkpoint(0, &events), 3);
        assert_eq!(advance_checkpoint(2, &events), 3);
        assert_eq!(advance_checkpoint(3, &events), 3);
    }

    // r[verify durable-execution.resume-step]
    #[test]
    fn test_resume_step() {
        let events = vec![work(0), work(1), work(2)];
        assert_eq!(resume_step(0, &events), Some(3));
        assert_eq!(resume_step(2, &events), Some(3));
        assert_eq!(resume_step(0, &[]), Some(0));

        let terminal = vec![work(0), DurableEvent { seq_num: 1, kind: EVENT_CANCELLED, effect_id: 0 }];
        assert_eq!(resume_step(0, &terminal), None);
    }

    proptest! {
        // r[verify durable-execution.checkpoint-prefix]
        #[test]
        fn prop_valid_checkpoint_advances_to_prefix(len in 0usize..40, old in 0usize..40) {
            let events: Vec<DurableEvent> = (0..len).map(work).collect();
            let prefix = durable_prefix_len(&events);
            let checkpoint = old.min(prefix);
            prop_assert_eq!(advance_checkpoint(checkpoint, &events), prefix);
        }

        // r[verify durable-execution.resume-bounds]
        #[test]
        fn prop_nonterminal_resume_is_prefix(len in 0usize..40) {
            let events: Vec<DurableEvent> = (0..len).map(work).collect();
            prop_assert_eq!(resume_step(0, &events), Some(len));
        }
    }
}
