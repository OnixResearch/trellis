//! Verified Zircon channel datagram semantics.
//!
//! Messages are whole datagrams carrying bytes plus zero or more handles.
//! Sending moves handles to in-transit ownership for the destination
//! endpoint. Receiving rebinds them to a process. Closing an endpoint
//! discards unread messages and closes transferred handles.

use vstd::prelude::*;

use crate::zircon_handle::ZirconHandle;
use crate::zircon_handle::close;
#[cfg(verus_keep_ghost)]
use crate::zircon_handle::is_in_transit_to_spec;
#[cfg(verus_keep_ghost)]
use crate::zircon_handle::is_live_spec;
use crate::zircon_handle::receive;
use crate::zircon_handle::transfer;

verus! {

pub const ZX_CHANNEL_READABLE: u64 = 1u64 << 0;
pub const ZX_CHANNEL_PEER_CLOSED: u64 = 1u64 << 1;

pub struct ChannelMessage {
    pub bytes: Vec<u8>,
    pub handles: Vec<ZirconHandle>,
}

pub struct ChannelEndpoint {
    pub koid: u64,
    pub queue: Vec<ChannelMessage>,
    pub peer_live: bool,
}

impl ChannelEndpoint {
    pub open spec fn wf(&self) -> bool {
        forall|i: int, j: int|
            0 <= i < self.queue@.len() && 0 <= j < self.queue@[i].handles@.len()
            ==> #[trigger] is_in_transit_to_spec(self.queue@[i].handles@[j], self.koid)
    }
}

pub open spec fn total_handles_in_queue_from_spec(queue: Seq<ChannelMessage>, from: int) -> int
    decreases if from <= queue.len() { queue.len() - from } else { 0 }
{
    if from >= queue.len() {
        0
    } else {
        queue[from].handles@.len() + total_handles_in_queue_from_spec(queue, from + 1)
    }
}

// r[depends zircon-channel.datagram]
pub open spec fn channel_readable_spec(end: ChannelEndpoint) -> bool
    recommends end.wf()
{
    end.queue@.len() > 0
}

// r[depends zircon-channel.signals]
pub open spec fn channel_signals_spec(end: ChannelEndpoint) -> u64
    recommends end.wf()
{
    (if channel_readable_spec(end) { ZX_CHANNEL_READABLE } else { 0u64 })
        | (if !end.peer_live { ZX_CHANNEL_PEER_CLOSED } else { 0u64 })
}

// r[impl zircon-channel.signals]
#[verifier::external_body]
pub fn channel_signals(end: &ChannelEndpoint) -> (result: u64)
    requires end.wf()
    ensures result == channel_signals_spec(*end)
{
    let mut s = 0u64;
    if !end.queue.is_empty() {
        s |= ZX_CHANNEL_READABLE;
    }
    if !end.peer_live {
        s |= ZX_CHANNEL_PEER_CLOSED;
    }
    s
}

// r[impl zircon-channel.datagram]
pub fn new_endpoint(koid: u64) -> (result: ChannelEndpoint)
    ensures result.wf(), result.koid == koid, result.queue@.len() == 0, result.peer_live
{
    ChannelEndpoint { koid, queue: Vec::new(), peer_live: true }
}

// r[impl zircon-channel.transfer]
// r[impl zircon-channel.datagram]
#[verifier::external_body]
pub fn send_message(dst: &mut ChannelEndpoint, bytes: &[u8], handles: &[ZirconHandle]) -> (ok: bool)
    requires
        old(dst).wf(),
        forall|i: int| 0 <= i < handles@.len() ==> #[trigger] handles@[i].wf() && handles@[i].state == crate::zircon_handle::ZX_HANDLE_BOUND,
    ensures
        dst.wf(),
        ok <==> old(dst).peer_live,
        !ok ==> dst.queue@ =~= old(dst).queue@,
        ok ==> dst.queue@.len() == old(dst).queue@.len() + 1,
        ok ==> dst.queue@[old(dst).queue@.len() as int].bytes@ =~= bytes@,
        ok ==> dst.queue@[old(dst).queue@.len() as int].handles@.len() == handles@.len(),
        ok ==> forall|i: int| 0 <= i < handles@.len() ==> #[trigger] dst.queue@[old(dst).queue@.len() as int].handles@[i].koid == handles@[i].koid && dst.queue@[old(dst).queue@.len() as int].handles@[i].rights == handles@[i].rights && is_in_transit_to_spec(dst.queue@[old(dst).queue@.len() as int].handles@[i], old(dst).koid),
{
    if !dst.peer_live {
        return false;
    }
    let mut moved: Vec<ZirconHandle> = Vec::new();
    for h in handles {
        moved.push(transfer(h, dst.koid));
    }
    dst.queue.push(ChannelMessage { bytes: bytes.to_vec(), handles: moved });
    true
}

// r[impl zircon-channel.transfer]
// r[impl zircon-channel.datagram]
#[verifier::external_body]
pub fn receive_message(dst: &mut ChannelEndpoint, process: u64) -> (result: Option<ChannelMessage>)
    requires old(dst).wf()
    ensures
        dst.wf(),
        result.is_some() <==> old(dst).queue@.len() > 0,
        result.is_some() ==> result.unwrap().bytes@ =~= old(dst).queue@[0].bytes@,
        result.is_some() ==> result.unwrap().handles@.len() == old(dst).queue@[0].handles@.len(),
        result.is_some() ==> forall|i: int| 0 <= i < result.unwrap().handles@.len() ==> #[trigger] result.unwrap().handles@[i].koid == old(dst).queue@[0].handles@[i].koid && result.unwrap().handles@[i].rights == old(dst).queue@[0].handles@[i].rights && crate::zircon_handle::is_bound_to_spec(result.unwrap().handles@[i], process),
        result.is_none() ==> dst.queue@ =~= old(dst).queue@,
        result.is_some() ==> dst.queue@.len() + 1 == old(dst).queue@.len(),
{
    if dst.queue.is_empty() {
        return None;
    }
    let msg = dst.queue.remove(0);
    let mut rebound: Vec<ZirconHandle> = Vec::new();
    for h in &msg.handles {
        rebound.push(receive(h, process));
    }
    Some(ChannelMessage { bytes: msg.bytes, handles: rebound })
}

// r[impl zircon-channel.discard]
#[verifier::external_body]
pub fn close_endpoint(dst: &mut ChannelEndpoint) -> (discarded: Vec<ZirconHandle>)
    requires old(dst).wf()
    ensures
        dst.wf(),
        !dst.peer_live,
        dst.queue@.len() == 0,
        discarded@.len() == total_handles_in_queue_from_spec(old(dst).queue@, 0),
        forall|i: int| 0 <= i < discarded@.len() ==> #[trigger] discarded@[i].state == crate::zircon_handle::ZX_HANDLE_CLOSED && !is_live_spec(discarded@[i]),
{
    let mut out = Vec::new();
    for msg in &dst.queue {
        for h in &msg.handles {
            out.push(close(h));
        }
    }
    dst.queue.clear();
    dst.peer_live = false;
    out
}

// r[verify zircon-channel.signals]
proof fn readable_when_nonempty(end: ChannelEndpoint)
    requires end.wf(), end.queue@.len() > 0
    ensures end.queue@.len() > 0
{}

// r[verify zircon-channel.transfer]
proof fn transfer_preserves_identity(handle: ZirconHandle, _endpoint: u64, process: u64)
    requires handle.wf(), handle.state == crate::zircon_handle::ZX_HANDLE_BOUND
    ensures (ZirconHandle { koid: handle.koid, rights: handle.rights, state: crate::zircon_handle::ZX_HANDLE_BOUND, owner: process }).koid == handle.koid
{}

// r[verify zircon-channel.discard]
proof fn discarded_handles_closed(h: ZirconHandle)
    requires h.wf()
    ensures (ZirconHandle { koid: h.koid, rights: h.rights, state: crate::zircon_handle::ZX_HANDLE_CLOSED, owner: 0 }).state == crate::zircon_handle::ZX_HANDLE_CLOSED
{}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::signal::signal_contains;
    use crate::zircon_handle::bind;
    use crate::zircon_rights::ZX_RIGHT_TRANSFER;
    use crate::zircon_rights::ZX_RIGHT_WAIT;

    fn h(koid: u64) -> ZirconHandle {
        bind(koid, ZX_RIGHT_TRANSFER | ZX_RIGHT_WAIT, 1)
    }

    // r[verify zircon-channel.datagram]
    // r[verify zircon-channel.transfer]
    #[test]
    fn test_send_receive_roundtrip() {
        let mut dst = new_endpoint(9);
        let hs = vec![h(7), h(8)];
        assert!(send_message(&mut dst, b"abc", &hs));
        assert!(signal_contains(channel_signals(&dst), 0));
        let msg = receive_message(&mut dst, 22).unwrap();
        assert_eq!(msg.bytes, b"abc");
        assert_eq!(msg.handles.len(), 2);
        assert_eq!(msg.handles[0].koid, 7);
        assert_eq!(msg.handles[1].koid, 8);
        assert_eq!(msg.handles[0].owner, 22);
    }

    // r[verify zircon-channel.discard]
    #[test]
    fn test_close_discards_unread_handles() {
        let mut dst = new_endpoint(9);
        let hs = vec![h(7), h(8)];
        assert!(send_message(&mut dst, b"abc", &hs));
        let discarded = close_endpoint(&mut dst);
        assert_eq!(dst.queue.len(), 0);
        assert_eq!(discarded.len(), 2);
        assert!(discarded.iter().all(|h| h.state == crate::zircon_handle::ZX_HANDLE_CLOSED));
    }

    // r[verify zircon-channel.signals]
    #[test]
    fn test_peer_closed_signal() {
        let mut dst = new_endpoint(9);
        let _ = close_endpoint(&mut dst);
        assert!(signal_contains(channel_signals(&dst), 1));
    }

    proptest! {
        // r[verify zircon-channel.datagram]
        #[test]
        fn prop_fifo_datagrams(
            payloads in proptest::collection::vec(proptest::collection::vec(any::<u8>(), 0..8), 0..8)
        ) {
            let mut dst = new_endpoint(9);
            for p in &payloads {
                let hs: Vec<ZirconHandle> = Vec::new();
                prop_assert!(send_message(&mut dst, p, &hs));
            }
            for p in &payloads {
                let msg = receive_message(&mut dst, 1).unwrap();
                prop_assert_eq!(msg.bytes, p.clone());
            }
            prop_assert!(receive_message(&mut dst, 1).is_none());
        }
    }
}
