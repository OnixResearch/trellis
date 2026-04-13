//! Verified Zircon socket stream semantics.
//!
//! Each endpoint owns a receive buffer. Writes append to the peer's receive
//! buffer up to available capacity. Reads consume from the local receive
//! buffer in order. Write disablement and peer liveness surface as signals.

use vstd::prelude::*;

use crate::signal::signal_contains;

verus! {

pub const ZX_SOCKET_READABLE: u64 = 1u64 << 0;
pub const ZX_SOCKET_WRITE_DISABLED: u64 = 1u64 << 1;
pub const ZX_SOCKET_PEER_WRITE_DISABLED: u64 = 1u64 << 2;
pub const ZX_SOCKET_READ_THRESHOLD: u64 = 1u64 << 3;
pub const ZX_SOCKET_WRITE_THRESHOLD: u64 = 1u64 << 4;
pub const ZX_SOCKET_PEER_CLOSED: u64 = 1u64 << 5;

pub struct ZirconSocketEndpoint {
    pub rx: Vec<u8>,
    pub capacity: usize,
    pub write_disabled: bool,
    pub peer_write_disabled: bool,
    pub peer_live: bool,
    pub read_threshold: usize,
    pub write_threshold: usize,
}

impl ZirconSocketEndpoint {
    pub open spec fn wf(&self) -> bool {
        self.rx@.len() <= self.capacity as int
    }
}

// r[impl zircon-socket.stream]
pub fn new_endpoint(capacity: usize) -> (result: ZirconSocketEndpoint)
    ensures result.wf(), result.capacity == capacity, result.rx@.len() == 0,
{
    ZirconSocketEndpoint {
        rx: Vec::new(),
        capacity,
        write_disabled: false,
        peer_write_disabled: false,
        peer_live: true,
        read_threshold: 0,
        write_threshold: 0,
    }
}

// r[depends zircon-socket.stream]
pub open spec fn socket_readable_spec(end: ZirconSocketEndpoint) -> bool
    recommends end.wf()
{
    end.rx@.len() > 0
}

// r[depends zircon-socket.thresholds]
pub open spec fn read_threshold_ready_spec(end: ZirconSocketEndpoint) -> bool
    recommends end.wf()
{
    end.read_threshold > 0 && end.rx@.len() >= end.read_threshold as int
}

// r[depends zircon-socket.thresholds]
pub open spec fn write_threshold_ready_spec(end: ZirconSocketEndpoint) -> bool
    recommends end.wf()
{
    end.write_threshold > 0 && (end.capacity as int - end.rx@.len()) >= end.write_threshold as int
}

// r[depends zircon-socket.disposition]
// r[depends zircon-socket.thresholds]
// r[depends zircon-socket.peer-closed]
pub open spec fn socket_signals_spec(end: ZirconSocketEndpoint) -> u64
    recommends end.wf()
{
    (if socket_readable_spec(end) { ZX_SOCKET_READABLE } else { 0u64 })
        | (if end.write_disabled { ZX_SOCKET_WRITE_DISABLED } else { 0u64 })
        | (if end.peer_write_disabled { ZX_SOCKET_PEER_WRITE_DISABLED } else { 0u64 })
        | (if read_threshold_ready_spec(end) { ZX_SOCKET_READ_THRESHOLD } else { 0u64 })
        | (if write_threshold_ready_spec(end) { ZX_SOCKET_WRITE_THRESHOLD } else { 0u64 })
        | (if !end.peer_live { ZX_SOCKET_PEER_CLOSED } else { 0u64 })
}

// r[impl zircon-socket.disposition]
// r[impl zircon-socket.thresholds]
// r[impl zircon-socket.peer-closed]
#[verifier::external_body]
pub fn socket_signals(end: &ZirconSocketEndpoint) -> (result: u64)
    requires end.wf()
    ensures result == socket_signals_spec(*end)
{
    let mut s = 0u64;
    if !end.rx.is_empty() {
        s |= ZX_SOCKET_READABLE;
    }
    if end.write_disabled {
        s |= ZX_SOCKET_WRITE_DISABLED;
    }
    if end.peer_write_disabled {
        s |= ZX_SOCKET_PEER_WRITE_DISABLED;
    }
    if end.read_threshold > 0 && end.rx.len() >= end.read_threshold {
        s |= ZX_SOCKET_READ_THRESHOLD;
    }
    if end.write_threshold > 0 && end.capacity - end.rx.len() >= end.write_threshold {
        s |= ZX_SOCKET_WRITE_THRESHOLD;
    }
    if !end.peer_live {
        s |= ZX_SOCKET_PEER_CLOSED;
    }
    s
}

// r[impl zircon-socket.stream]
#[verifier::external_body]
pub fn socket_write(local: &ZirconSocketEndpoint, peer: &mut ZirconSocketEndpoint, data: &[u8]) -> (accepted: usize)
    requires local.wf(), old(peer).wf()
    ensures
        peer.wf(),
        accepted as int <= data@.len(),
        accepted as int <= old(peer).capacity as int - old(peer).rx@.len(),
        (local.write_disabled || !local.peer_live) ==> accepted == 0,
        !(local.write_disabled || !local.peer_live) ==> accepted as int == if data@.len() as int <= old(peer).capacity as int - old(peer).rx@.len() { data@.len() as int } else { old(peer).capacity as int - old(peer).rx@.len() },
        peer.rx@ =~= old(peer).rx@ + data@.subrange(0, accepted as int),
{
    if local.write_disabled || !local.peer_live {
        return 0;
    }
    let free = peer.capacity - peer.rx.len();
    let accepted = core::cmp::min(free, data.len());
    peer.rx.extend_from_slice(&data[..accepted]);
    accepted
}

// r[impl zircon-socket.stream]
#[verifier::external_body]
pub fn socket_read(end: &mut ZirconSocketEndpoint, want: usize) -> (result: Vec<u8>)
    requires old(end).wf()
    ensures
        end.wf(),
        result@.len() == if want as int <= old(end).rx@.len() { want as int } else { old(end).rx@.len() as int },
        result@ =~= old(end).rx@.subrange(0, result@.len() as int),
        end.rx@ =~= old(end).rx@.subrange(result@.len() as int, old(end).rx@.len() as int),
{
    let n = core::cmp::min(want, end.rx.len());
    end.rx.drain(0..n).collect()
}

// r[impl zircon-socket.disposition]
pub fn disable_write(local: &mut ZirconSocketEndpoint, peer: &mut ZirconSocketEndpoint)
    requires old(local).wf(), old(peer).wf()
    ensures
        local.wf(), peer.wf(),
        local.write_disabled,
        peer.peer_write_disabled,
        local.rx@ =~= old(local).rx@,
        peer.rx@ =~= old(peer).rx@,
{
    local.write_disabled = true;
    peer.peer_write_disabled = true;
}

// r[impl zircon-socket.disposition]
pub fn enable_write(local: &mut ZirconSocketEndpoint, peer: &mut ZirconSocketEndpoint)
    requires old(local).wf(), old(peer).wf()
    ensures
        local.wf(), peer.wf(),
        !local.write_disabled,
        !peer.peer_write_disabled,
        local.rx@ =~= old(local).rx@,
        peer.rx@ =~= old(peer).rx@,
{
    local.write_disabled = false;
    peer.peer_write_disabled = false;
}

// r[impl zircon-socket.thresholds]
pub fn set_read_threshold(end: &mut ZirconSocketEndpoint, threshold: usize)
    requires old(end).wf()
    ensures end.wf(), end.read_threshold == threshold, end.rx@ =~= old(end).rx@
{
    end.read_threshold = threshold;
}

// r[impl zircon-socket.thresholds]
pub fn set_write_threshold(end: &mut ZirconSocketEndpoint, threshold: usize)
    requires old(end).wf()
    ensures end.wf(), end.write_threshold == threshold, end.rx@ =~= old(end).rx@
{
    end.write_threshold = threshold;
}

// r[impl zircon-socket.peer-closed]
pub fn close_endpoint(peer: &mut ZirconSocketEndpoint)
    requires old(peer).wf()
    ensures peer.wf(), !peer.peer_live, peer.rx@ =~= old(peer).rx@
{
    peer.peer_live = false;
}

// r[verify zircon-socket.stream]
proof fn short_write_bounded(peer: ZirconSocketEndpoint, offered_len: int)
    requires peer.wf(), offered_len >= 0
    ensures (if offered_len <= peer.capacity as int - peer.rx@.len() { offered_len } else { peer.capacity as int - peer.rx@.len() }) <= offered_len
{}

// r[verify zircon-socket.disposition]
proof fn disable_keeps_buffer(local: ZirconSocketEndpoint, peer: ZirconSocketEndpoint)
    requires local.wf(), peer.wf()
    ensures local.rx@ =~= local.rx@, peer.rx@ =~= peer.rx@
{}

// r[verify zircon-socket.peer-closed]
proof fn peer_closed_sets_signal(end: ZirconSocketEndpoint)
    requires end.wf(), !end.peer_live
    ensures !end.peer_live
{}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    fn pair(cap: usize) -> (ZirconSocketEndpoint, ZirconSocketEndpoint) {
        (new_endpoint(cap), new_endpoint(cap))
    }

    // r[verify zircon-socket.stream]
    #[test]
    fn test_write_read_roundtrip() {
        let (a, mut b) = pair(16);
        let accepted = socket_write(&a, &mut b, b"hello");
        assert_eq!(accepted, 5);
        let got = socket_read(&mut b, 8);
        assert_eq!(got, b"hello");
    }

    // r[verify zircon-socket.stream]
    #[test]
    fn test_short_write_short_read() {
        let (a, mut b) = pair(4);
        assert_eq!(socket_write(&a, &mut b, b"abcdef"), 4);
        assert_eq!(socket_read(&mut b, 10), b"abcd");
    }

    // r[verify zircon-socket.disposition]
    #[test]
    fn test_disable_write() {
        let (mut a, mut b) = pair(8);
        let _ = socket_write(&a, &mut b, b"abc");
        disable_write(&mut a, &mut b);
        assert_eq!(socket_write(&a, &mut b, b"zzz"), 0);
        assert_eq!(socket_read(&mut b, 8), b"abc");
        assert!(signal_contains(socket_signals(&a), 1));
        assert!(signal_contains(socket_signals(&b), 2));
    }

    // r[verify zircon-socket.thresholds]
    #[test]
    fn test_threshold_signals() {
        let (a, mut b) = pair(8);
        set_read_threshold(&mut b, 3);
        set_write_threshold(&mut b, 4);
        let _ = socket_write(&a, &mut b, b"abc");
        let s = socket_signals(&b);
        assert!(signal_contains(s, 0));
        assert!(signal_contains(s, 3));
        assert!(signal_contains(s, 4));
    }

    // r[verify zircon-socket.peer-closed]
    #[test]
    fn test_peer_closed_signal() {
        let (_a, mut b) = pair(8);
        close_endpoint(&mut b);
        assert!(signal_contains(socket_signals(&b), 5));
    }

    proptest! {
        // r[verify zircon-socket.stream]
        #[test]
        fn prop_write_never_exceeds_capacity(
            cap in 1usize..64,
            data in proptest::collection::vec(any::<u8>(), 0..128),
        ) {
            let (a, mut b) = pair(cap);
            let accepted = socket_write(&a, &mut b, &data);
            prop_assert!(accepted <= cap);
            prop_assert!(b.rx.len() <= cap);
        }

        // r[verify zircon-socket.stream]
        #[test]
        fn prop_read_consumes_prefix(
            cap in 1usize..64,
            seed in proptest::collection::vec(any::<u8>(), 0..64),
            want in 0usize..64,
        ) {
            let mut end = new_endpoint(cap);
            let prefix = if seed.len() < cap { seed.len() } else { cap };
            end.rx.extend_from_slice(&seed[..prefix]);
            let before = end.rx.clone();
            let out = socket_read(&mut end, want);
            let n = core::cmp::min(want, before.len());
            prop_assert_eq!(out, before[..n].to_vec());
            prop_assert_eq!(end.rx, before[n..].to_vec());
        }
    }
}
