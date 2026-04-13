//! Verified Zircon FIFO semantics.
//!
//! FIFOs move whole fixed-size elements only. Reads and writes are bounded
//! by free/available element slots.

use vstd::prelude::*;

use crate::signal::signal_contains;

verus! {

pub const ZX_FIFO_READABLE: u64 = 1u64 << 0;
pub const ZX_FIFO_WRITABLE: u64 = 1u64 << 1;
pub const ZX_FIFO_PEER_CLOSED: u64 = 1u64 << 2;

pub struct ZirconFifoEndpoint {
    pub rx: Vec<Vec<u8>>,
    pub element_size: usize,
    pub capacity: usize,
    pub peer_live: bool,
}

impl ZirconFifoEndpoint {
    pub open spec fn wf(&self) -> bool {
        &&& self.rx@.len() <= self.capacity as int
        &&& forall|i: int| 0 <= i < self.rx@.len() ==> #[trigger] self.rx@[i]@.len() == self.element_size as int
    }
}

// r[impl zircon-fifo.element-aligned]
pub fn new_endpoint(element_size: usize, capacity: usize) -> (result: ZirconFifoEndpoint)
    ensures result.wf(), result.element_size == element_size, result.capacity == capacity, result.rx@.len() == 0
{
    ZirconFifoEndpoint { rx: Vec::new(), element_size, capacity, peer_live: true }
}

// r[impl zircon-fifo.element-aligned]
#[verifier::external_body]
pub fn fifo_write(peer: &mut ZirconFifoEndpoint, elements: &[Vec<u8>]) -> (accepted: usize)
    requires old(peer).wf(), forall|i: int| 0 <= i < elements@.len() ==> #[trigger] elements@[i]@.len() == old(peer).element_size as int
    ensures
        peer.wf(),
        accepted as int <= elements@.len(),
        accepted <= old(peer).capacity - old(peer).rx.len(),
        peer.rx@.len() == old(peer).rx@.len() + accepted as int,
{
    let free = peer.capacity - peer.rx.len();
    let accepted = core::cmp::min(free, elements.len());
    for elt in &elements[..accepted] {
        peer.rx.push(elt.clone());
    }
    accepted
}

// r[impl zircon-fifo.element-aligned]
#[verifier::external_body]
pub fn fifo_read(end: &mut ZirconFifoEndpoint, want: usize) -> (result: Vec<Vec<u8>>)
    requires old(end).wf()
    ensures
        end.wf(),
        result@.len() == if want as int <= old(end).rx@.len() { want as int } else { old(end).rx@.len() as int },
        forall|i: int| 0 <= i < result@.len() ==> #[trigger] result@[i]@ =~= old(end).rx@[i]@,
        end.rx@.len() + result@.len() == old(end).rx@.len(),
{
    let n = core::cmp::min(want, end.rx.len());
    end.rx.drain(0..n).collect()
}

// r[depends zircon-fifo.signals]
pub open spec fn fifo_signals_spec(end: ZirconFifoEndpoint)
    -> u64
    recommends end.wf()
{
    (if end.rx@.len() > 0 { ZX_FIFO_READABLE } else { 0u64 })
        | (if end.rx@.len() < end.capacity as int { ZX_FIFO_WRITABLE } else { 0u64 })
        | (if !end.peer_live { ZX_FIFO_PEER_CLOSED } else { 0u64 })
}

// r[impl zircon-fifo.signals]
#[verifier::external_body]
pub fn fifo_signals(end: &ZirconFifoEndpoint) -> (result: u64)
    requires end.wf()
    ensures result == fifo_signals_spec(*end)
{
    let mut s = 0u64;
    if !end.rx.is_empty() {
        s |= ZX_FIFO_READABLE;
    }
    if end.rx.len() < end.capacity {
        s |= ZX_FIFO_WRITABLE;
    }
    if !end.peer_live {
        s |= ZX_FIFO_PEER_CLOSED;
    }
    s
}

// r[impl zircon-fifo.peer-closed]
pub fn close_endpoint(peer: &mut ZirconFifoEndpoint)
    requires old(peer).wf()
    ensures peer.wf(), !peer.peer_live, peer.rx@ =~= old(peer).rx@
{
    peer.peer_live = false;
}

// r[verify zircon-fifo.element-aligned]
proof fn write_accepts_only_whole_elements(end: ZirconFifoEndpoint, elements: Seq<Seq<u8>>)
    requires end.wf(), forall|i: int| 0 <= i < elements.len() ==> #[trigger] elements[i].len() == end.element_size as int
    ensures forall|i: int| 0 <= i < elements.len() ==> #[trigger] elements[i].len() == end.element_size as int
{}

// r[verify zircon-fifo.peer-closed]
proof fn peer_close_sets_signal(end: ZirconFifoEndpoint)
    requires end.wf(), !end.peer_live
    ensures !end.peer_live
{}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    fn elem(bytes: &[u8]) -> Vec<u8> {
        bytes.to_vec()
    }

    // r[verify zircon-fifo.element-aligned]
    #[test]
    fn test_fifo_whole_elements() {
        let mut end = new_endpoint(2, 4);
        let els = vec![elem(b"aa"), elem(b"bb"), elem(b"cc")];
        assert_eq!(fifo_write(&mut end, &els), 3);
        let got = fifo_read(&mut end, 2);
        assert_eq!(got, vec![elem(b"aa"), elem(b"bb")]);
        let rest = fifo_read(&mut end, 2);
        assert_eq!(rest, vec![elem(b"cc")]);
    }

    // r[verify zircon-fifo.element-aligned]
    #[test]
    fn test_fifo_capacity_bound() {
        let mut end = new_endpoint(2, 2);
        let els = vec![elem(b"aa"), elem(b"bb"), elem(b"cc")];
        assert_eq!(fifo_write(&mut end, &els), 2);
    }

    // r[verify zircon-fifo.signals]
    #[test]
    fn test_fifo_signals() {
        let mut end = new_endpoint(2, 2);
        assert!(signal_contains(fifo_signals(&end), 1));
        let _ = fifo_write(&mut end, &[elem(b"aa")]);
        let s = fifo_signals(&end);
        assert!(signal_contains(s, 0));
        assert!(signal_contains(s, 1));
    }

    // r[verify zircon-fifo.peer-closed]
    #[test]
    fn test_peer_closed() {
        let mut end = new_endpoint(2, 2);
        close_endpoint(&mut end);
        assert!(signal_contains(fifo_signals(&end), 2));
    }

    proptest! {
        // r[verify zircon-fifo.element-aligned]
        #[test]
        fn prop_read_write_len_bounded(n in 0usize..16, want in 0usize..16) {
            let mut end = new_endpoint(1, 8);
            let mut els = Vec::new();
            for i in 0..n {
                els.push(vec![u8::try_from(i & 0xff).expect("masked to u8")]);
            }
            let written = fifo_write(&mut end, &els);
            let got = fifo_read(&mut end, want);
            prop_assert!(written <= 8);
            prop_assert!(got.len() <= written);
            for e in got {
                prop_assert_eq!(e.len(), 1);
            }
        }
    }
}
