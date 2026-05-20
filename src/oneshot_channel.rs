//! Finite Rust oneshot channel primitives.
//!
//! This module models a single-slot oneshot channel admission boundary. It
//! tracks whether a value can be sent once, received once, closed before send,
//! or cancelled before delivery. The model is finite and does not implement
//! allocation, parking, callbacks, or executor behavior.

use vstd::prelude::*;

verus! {
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OneshotState {
    Open,
    Sent { value: u64 },
    Received { value: u64 },
    Closed,
    Cancelled,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OneshotReceive {
    pub value: u64,
    pub state: OneshotState,
}

// r[depends rust-primitives.oneshot-channel]
pub open spec fn oneshot_terminal_spec(state: OneshotState) -> bool {
    match state {
        OneshotState::Received { .. } | OneshotState::Closed | OneshotState::Cancelled => true,
        OneshotState::Open | OneshotState::Sent { .. } => false,
    }
}

// r[depends rust-primitives.oneshot-channel]
pub open spec fn oneshot_send_spec(state: OneshotState, value: u64) -> Option<OneshotState> {
    if state == OneshotState::Open {
        Some(OneshotState::Sent { value })
    } else {
        None
    }
}

// r[depends rust-primitives.oneshot-channel]
pub open spec fn oneshot_receive_spec(state: OneshotState) -> Option<OneshotReceive> {
    match state {
        OneshotState::Sent { value } => Some(OneshotReceive {
            value,
            state: OneshotState::Received { value },
        }),
        OneshotState::Open
        | OneshotState::Received { .. }
        | OneshotState::Closed
        | OneshotState::Cancelled => None,
    }
}

// r[depends rust-primitives.oneshot-channel]
pub open spec fn oneshot_close_spec(state: OneshotState) -> Option<OneshotState> {
    if state == OneshotState::Open {
        Some(OneshotState::Closed)
    } else {
        None
    }
}

// r[depends rust-primitives.oneshot-channel]
pub open spec fn oneshot_cancel_spec(state: OneshotState) -> Option<OneshotState> {
    match state {
        OneshotState::Open | OneshotState::Sent { .. } => Some(OneshotState::Cancelled),
        OneshotState::Received { .. } | OneshotState::Closed | OneshotState::Cancelled => None,
    }
}

// r[impl rust-primitives.oneshot-channel]
pub fn oneshot_is_terminal(state: OneshotState) -> (result: bool)
    ensures result == oneshot_terminal_spec(state)
{
    match state {
        OneshotState::Open | OneshotState::Sent { .. } => false,
        OneshotState::Received { .. } | OneshotState::Closed | OneshotState::Cancelled => true,
    }
}

// r[impl rust-primitives.oneshot-channel]
pub fn oneshot_send(state: OneshotState, value: u64) -> (result: Option<OneshotState>)
    ensures result == oneshot_send_spec(state, value)
{
    match state {
        OneshotState::Open => Some(OneshotState::Sent { value }),
        OneshotState::Sent { .. }
        | OneshotState::Received { .. }
        | OneshotState::Closed
        | OneshotState::Cancelled => None,
    }
}

// r[impl rust-primitives.oneshot-channel]
pub fn oneshot_receive(state: OneshotState) -> (result: Option<OneshotReceive>)
    ensures result == oneshot_receive_spec(state)
{
    match state {
        OneshotState::Sent { value } => Some(OneshotReceive {
            value,
            state: OneshotState::Received { value },
        }),
        OneshotState::Open
        | OneshotState::Received { .. }
        | OneshotState::Closed
        | OneshotState::Cancelled => None,
    }
}

// r[impl rust-primitives.oneshot-channel]
pub fn oneshot_close(state: OneshotState) -> (result: Option<OneshotState>)
    ensures result == oneshot_close_spec(state)
{
    match state {
        OneshotState::Open => Some(OneshotState::Closed),
        OneshotState::Sent { .. }
        | OneshotState::Received { .. }
        | OneshotState::Closed
        | OneshotState::Cancelled => None,
    }
}

// r[impl rust-primitives.oneshot-channel]
pub fn oneshot_cancel(state: OneshotState) -> (result: Option<OneshotState>)
    ensures result == oneshot_cancel_spec(state)
{
    match state {
        OneshotState::Open | OneshotState::Sent { .. } => Some(OneshotState::Cancelled),
        OneshotState::Received { .. } | OneshotState::Closed | OneshotState::Cancelled => None,
    }
}

// r[verify rust-primitives.oneshot-channel]
pub proof fn send_then_receive_consumes_one_value(value: u64)
    ensures oneshot_receive_spec(OneshotState::Sent { value }) == Some(OneshotReceive {
            value,
            state: OneshotState::Received { value },
        }),
        oneshot_receive_spec(OneshotState::Received { value }) == None::<OneshotReceive>,
        oneshot_terminal_spec(OneshotState::Received { value })
{}

// r[verify rust-primitives.oneshot-channel]
pub proof fn open_send_succeeds_once(value: u64)
    ensures oneshot_send_spec(OneshotState::Open, value) == Some(OneshotState::Sent { value }),
        oneshot_send_spec(OneshotState::Sent { value }, value) == None::<OneshotState>
{}

// r[verify rust-primitives.oneshot-channel]
pub proof fn closed_channel_rejects_send(value: u64)
    ensures oneshot_send_spec(OneshotState::Closed, value) == None::<OneshotState>,
        oneshot_terminal_spec(OneshotState::Closed)
{}

// r[verify rust-primitives.oneshot-channel]
pub proof fn cancelled_receiver_prevents_duplicate_delivery(value: u64)
    ensures oneshot_cancel_spec(OneshotState::Sent { value }) == Some(OneshotState::Cancelled),
        oneshot_receive_spec(OneshotState::Cancelled) == None::<OneshotReceive>,
        oneshot_terminal_spec(OneshotState::Cancelled)
{}
} // verus!

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_then_receive_consumes_one_value_at_runtime() {
        let sent = oneshot_send(OneshotState::Open, 42);
        assert_eq!(sent, Some(OneshotState::Sent { value: 42 }));
        assert_eq!(
            oneshot_receive(sent.unwrap()),
            Some(OneshotReceive {
                value: 42,
                state: OneshotState::Received { value: 42 },
            })
        );
        assert_eq!(oneshot_receive(OneshotState::Received { value: 42 }), None);
    }

    #[test]
    fn send_succeeds_at_most_once() {
        assert_eq!(oneshot_send(OneshotState::Open, 7), Some(OneshotState::Sent { value: 7 }));
        assert_eq!(oneshot_send(OneshotState::Sent { value: 7 }, 8), None);
    }

    #[test]
    fn closed_channel_rejects_send() {
        assert_eq!(oneshot_close(OneshotState::Open), Some(OneshotState::Closed));
        assert_eq!(oneshot_send(OneshotState::Closed, 9), None);
        assert!(oneshot_is_terminal(OneshotState::Closed));
    }

    #[test]
    fn cancelled_receiver_prevents_duplicate_delivery() {
        assert_eq!(oneshot_cancel(OneshotState::Sent { value: 10 }), Some(OneshotState::Cancelled));
        assert_eq!(oneshot_receive(OneshotState::Cancelled), None);
        assert!(oneshot_is_terminal(OneshotState::Cancelled));
    }
}
