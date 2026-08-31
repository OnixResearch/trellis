//! Finite parking-lot raw mutex state machine.

pub const RAW_MUTEX_LOCKED_BIT: u8 = 0b01;
pub const RAW_MUTEX_PARKED_BIT: u8 = 0b10;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    AcquiredFast,
    Queued,
    Unlocked,
    Handoff,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct State {
    pub bits: u8,
    pub owner: Option<crate::parking_lot_core_model::ThreadId>,
}

impl State {
    // r[impl domain-primitives.parking-lot-raw-mutex.state-machine]
    pub const fn unlocked() -> Self {
        Self { bits: 0, owner: None }
    }
}

// r[impl domain-primitives.parking-lot-raw-mutex.state-machine]
pub fn is_locked(state: State) -> bool {
    state.bits & RAW_MUTEX_LOCKED_BIT != 0
}

// r[impl domain-primitives.parking-lot-raw-mutex.state-machine]
pub fn has_parked_bit(state: State) -> bool {
    state.bits & RAW_MUTEX_PARKED_BIT != 0
}

// r[impl domain-primitives.parking-lot-raw-mutex.state-machine]
// r[impl newtypes.parking-lot-api] r[impl newtypes.parking-lot-identities]
pub fn state_valid(
    state: State,
    table: &crate::parking_lot_core_model::Table,
    key: crate::parking_lot_core_model::ParkKey,
) -> bool {
    let is_owner_match = is_locked(state) == state.owner.is_some();
    let is_waiter_present = table.queues.iter().any(|queue| queue.key == key && !queue.waiters.is_empty());
    is_owner_match && (!is_waiter_present || has_parked_bit(state))
}

// r[impl domain-primitives.parking-lot-raw-mutex.state-machine]
// r[impl newtypes.parking-lot-api] r[impl newtypes.parking-lot-semantics]
pub fn lock(state: State, thread_id: crate::parking_lot_core_model::ThreadId) -> (Step, State) {
    if !is_locked(state) {
        (Step::AcquiredFast, State {
            bits: RAW_MUTEX_LOCKED_BIT,
            owner: Some(thread_id),
        })
    } else {
        (Step::Queued, State {
            bits: state.bits | RAW_MUTEX_PARKED_BIT,
            owner: state.owner,
        })
    }
}

// r[impl domain-primitives.parking-lot-raw-mutex.state-machine]
// r[impl newtypes.parking-lot-api] r[impl newtypes.parking-lot-semantics]
pub fn unlock(
    state: State,
    table: &crate::parking_lot_core_model::Table,
    key: crate::parking_lot_core_model::ParkKey,
) -> (Step, State) {
    if !is_locked(state) {
        return (Step::Rejected, state);
    }
    let is_waiter_present = table.queues.iter().any(|queue| queue.key == key && !queue.waiters.is_empty());
    if is_waiter_present {
        (Step::Handoff, State {
            bits: RAW_MUTEX_LOCKED_BIT | RAW_MUTEX_PARKED_BIT,
            owner: None,
        })
    } else {
        (Step::Unlocked, State::unlocked())
    }
}

// r[impl domain-primitives.parking-lot-raw-mutex.state-machine]
// r[impl newtypes.parking-lot-api]
pub fn waiter_accounted(
    state: State,
    table: &crate::parking_lot_core_model::Table,
    key: crate::parking_lot_core_model::ParkKey,
    waiter: crate::parking_lot_core_model::ThreadId,
    explicitly_removed: bool,
) -> bool {
    !crate::parking_lot_core_model::has_waiter(table, waiter)
        || has_parked_bit(state)
        || explicitly_removed
        || table.queues.iter().all(|queue| queue.key != key)
}

// r[verify domain-primitives.parking-lot-raw-mutex.state-machine]
// r[verify newtypes.parking-lot-semantics]
// r[impl domain-primitives.parking-lot-raw-mutex.state-machine]
pub fn fast_path_locks(thread_id: crate::parking_lot_core_model::ThreadId) -> bool {
    let (step, state) = lock(State::unlocked(), thread_id);
    step == Step::AcquiredFast && is_locked(state) && state.owner == Some(thread_id)
}

// r[verify domain-primitives.parking-lot-raw-mutex.state-machine]
// r[verify newtypes.parking-lot-semantics]
// r[impl domain-primitives.parking-lot-raw-mutex.state-machine]
pub fn queued_sets_parked(
    holder: crate::parking_lot_core_model::ThreadId,
    waiter: crate::parking_lot_core_model::ThreadId,
) -> bool {
    let state = State {
        bits: RAW_MUTEX_LOCKED_BIT,
        owner: Some(holder),
    };
    let (step, next) = lock(state, waiter);
    step == Step::Queued && has_parked_bit(next)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parking_lot_core_model::ParkKey;
    use crate::parking_lot_core_model::Table;
    use crate::parking_lot_core_model::ThreadId;
    use crate::parking_lot_core_model::park_waiter;

    const MUTEX_KEY: ParkKey = ParkKey::new(5);
    const HOLDER: ThreadId = ThreadId::new(1);
    const WAITER: ThreadId = ThreadId::new(2);

    // r[verify newtypes.parking-lot-evidence] r[verify newtypes.parking-lot-api]
    #[test]
    fn fast_lock_and_empty_unlock() {
        let empty = Table { queues: Vec::new() };
        let (step, state) = lock(State::unlocked(), HOLDER);
        assert_eq!(step, Step::AcquiredFast);
        assert_eq!(state.owner, Some(HOLDER));
        assert!(state_valid(state, &empty, MUTEX_KEY));
        let (unlock_step, unlocked) = unlock(state, &empty, MUTEX_KEY);
        assert_eq!(unlock_step, Step::Unlocked);
        assert_eq!(unlocked, State::unlocked());
    }

    // r[verify newtypes.parking-lot-evidence]
    #[test]
    fn waiters_require_parked_bit() {
        let table = park_waiter(&Table { queues: Vec::new() }, MUTEX_KEY, WAITER);
        let bad = State {
            bits: RAW_MUTEX_LOCKED_BIT,
            owner: Some(HOLDER),
        };
        assert!(!state_valid(bad, &table, MUTEX_KEY));
        let (step, parked) = lock(bad, WAITER);
        assert_eq!(step, Step::Queued);
        assert!(state_valid(parked, &table, MUTEX_KEY));
        assert!(waiter_accounted(parked, &table, MUTEX_KEY, WAITER, false));
    }

    // r[verify newtypes.parking-lot-evidence]
    #[test]
    fn invalid_unlock_is_rejected_without_state_change() {
        let empty = Table { queues: Vec::new() };
        let state = State::unlocked();
        let (step, next) = unlock(state, &empty, MUTEX_KEY);
        assert_eq!(step, Step::Rejected);
        assert_eq!(next, state);
    }

    // r[verify newtypes.parking-lot-evidence]
    #[test]
    fn queued_unlock_reports_handoff() {
        let table = park_waiter(&Table { queues: Vec::new() }, MUTEX_KEY, WAITER);
        let state = State {
            bits: RAW_MUTEX_LOCKED_BIT | RAW_MUTEX_PARKED_BIT,
            owner: Some(HOLDER),
        };
        let (step, next) = unlock(state, &table, MUTEX_KEY);
        assert_eq!(step, Step::Handoff);
        assert_eq!(next.owner, None);
        assert!(is_locked(next));
    }
}
