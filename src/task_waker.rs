//! Finite Rust task-waker slot primitives.
//!
//! This module models registration and notification as generation-tagged slot
//! state. It deliberately records only observable finite state: no callback
//! execution, raw-waker vtables, scheduler fairness, or OS wake delivery.

use vstd::prelude::*;

verus! {
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WakerSlot {
    Empty,
    Registered { generation: u64 },
    Notified { generation: u64 },
    Taken { generation: u64 },
}

// r[depends rust-primitives.task-waker]
pub open spec fn waker_slot_generation_spec(slot: WakerSlot) -> Option<u64> {
    match slot {
        WakerSlot::Empty => None,
        WakerSlot::Registered { generation } => Some(generation),
        WakerSlot::Notified { generation } => Some(generation),
        WakerSlot::Taken { generation } => Some(generation),
    }
}

// r[depends rust-primitives.task-waker]
pub open spec fn waker_slot_pending_spec(slot: WakerSlot, generation: u64) -> bool {
    slot == WakerSlot::Notified { generation }
}

// r[depends rust-primitives.task-waker]
pub open spec fn waker_register_spec(slot: WakerSlot, generation: u64) -> WakerSlot {
    WakerSlot::Registered { generation }
}

// r[depends rust-primitives.task-waker]
pub open spec fn waker_wake_spec(slot: WakerSlot, generation: u64) -> Option<WakerSlot> {
    match slot {
        WakerSlot::Registered { generation: current } => if current == generation {
            Some(WakerSlot::Notified { generation })
        } else {
            None
        },
        WakerSlot::Notified { generation: current } => if current == generation {
            Some(WakerSlot::Notified { generation })
        } else {
            None
        },
        WakerSlot::Empty => None,
        WakerSlot::Taken { .. } => None,
    }
}

// r[depends rust-primitives.task-waker]
pub open spec fn waker_take_spec(slot: WakerSlot) -> Option<WakerSlot> {
    match slot {
        WakerSlot::Notified { generation } => Some(WakerSlot::Taken { generation }),
        WakerSlot::Empty => None,
        WakerSlot::Registered { .. } => None,
        WakerSlot::Taken { .. } => None,
    }
}

// r[impl rust-primitives.task-waker]
pub fn waker_slot_generation(slot: WakerSlot) -> (result: Option<u64>)
    ensures result == waker_slot_generation_spec(slot)
{
    match slot {
        WakerSlot::Empty => None,
        WakerSlot::Registered { generation }
        | WakerSlot::Notified { generation }
        | WakerSlot::Taken { generation } => Some(generation),
    }
}

// r[impl rust-primitives.task-waker]
pub fn waker_register(slot: WakerSlot, generation: u64) -> (result: WakerSlot)
    ensures result == waker_register_spec(slot, generation)
{
    match slot {
        WakerSlot::Empty
        | WakerSlot::Registered { .. }
        | WakerSlot::Notified { .. }
        | WakerSlot::Taken { .. } => WakerSlot::Registered { generation },
    }
}

// r[impl rust-primitives.task-waker]
pub fn waker_wake(slot: WakerSlot, generation: u64) -> (result: Option<WakerSlot>)
    ensures result == waker_wake_spec(slot, generation)
{
    match slot {
        WakerSlot::Registered { generation: current }
        | WakerSlot::Notified { generation: current } => if current == generation {
            Some(WakerSlot::Notified { generation })
        } else {
            None
        },
        WakerSlot::Empty | WakerSlot::Taken { .. } => None,
    }
}

// r[impl rust-primitives.task-waker]
pub fn waker_take(slot: WakerSlot) -> (result: Option<WakerSlot>)
    ensures result == waker_take_spec(slot)
{
    match slot {
        WakerSlot::Notified { generation } => Some(WakerSlot::Taken { generation }),
        WakerSlot::Empty | WakerSlot::Registered { .. } | WakerSlot::Taken { .. } => None,
    }
}

// r[verify rust-primitives.task-waker]
pub proof fn wake_after_registration_is_observable(generation: u64)
    ensures waker_wake_spec(waker_register_spec(WakerSlot::Empty, generation), generation)
        == Some(WakerSlot::Notified { generation }),
        waker_slot_pending_spec(WakerSlot::Notified { generation }, generation)
{}

// r[verify rust-primitives.task-waker]
pub proof fn repeated_wake_is_idempotent(generation: u64)
    ensures waker_wake_spec(WakerSlot::Notified { generation }, generation)
        == Some(WakerSlot::Notified { generation })
{}

// r[verify rust-primitives.task-waker]
pub proof fn replaced_waker_ignores_stale_generation(old_generation: u64, new_generation: u64)
    requires old_generation != new_generation
    ensures waker_wake_spec(waker_register_spec(WakerSlot::Registered { generation: old_generation }, new_generation), old_generation)
        == None::<WakerSlot>
{}

// r[verify rust-primitives.task-waker]
pub proof fn taking_notification_consumes_at_most_one(generation: u64)
    ensures waker_take_spec(WakerSlot::Notified { generation }) == Some(WakerSlot::Taken { generation }),
        waker_take_spec(WakerSlot::Taken { generation }) == None::<WakerSlot>
{}
} // verus!

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wake_after_registration_records_pending_generation() {
        let registered = waker_register(WakerSlot::Empty, 7);
        assert_eq!(registered, WakerSlot::Registered { generation: 7 });
        assert_eq!(waker_wake(registered, 7), Some(WakerSlot::Notified { generation: 7 }));
    }

    #[test]
    fn repeated_wake_is_idempotent() {
        let notified = WakerSlot::Notified { generation: 9 };
        assert_eq!(waker_wake(notified, 9), Some(notified));
        assert_eq!(waker_take(notified), Some(WakerSlot::Taken { generation: 9 }));
        assert_eq!(waker_take(WakerSlot::Taken { generation: 9 }), None);
    }

    #[test]
    fn replacement_rejects_stale_generation() {
        let replaced = waker_register(WakerSlot::Registered { generation: 1 }, 2);
        assert_eq!(replaced, WakerSlot::Registered { generation: 2 });
        assert_eq!(waker_wake(replaced, 1), None);
        assert_eq!(waker_wake(replaced, 2), Some(WakerSlot::Notified { generation: 2 }));
    }

    #[test]
    fn empty_or_registered_take_has_no_notification() {
        assert_eq!(waker_take(WakerSlot::Empty), None);
        assert_eq!(waker_take(WakerSlot::Registered { generation: 3 }), None);
    }
}
