//! Verified FIFO store buffer with store-to-load forwarding.
//!
//! Models pending stores as a FIFO sequence of entries. Loads may forward
//! from the youngest pending store that fully covers the requested byte range.
//! Drains preserve enqueue order, fences require emptiness, and capacity
//! checks fail closed.

use vstd::prelude::*;

verus! {

pub const MIN_ACCESS_BYTES: u64 = 1u64;
pub const MAX_ACCESS_BYTES: u64 = 8u64;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StoreEntry {
    pub addr: u64,
    pub size: u64,
    pub data: u64,
    pub seq: u64,
}

fn clone_entries(buffer: &[StoreEntry]) -> (result: Vec<StoreEntry>)
    ensures result@ == buffer@
{
    let mut result: Vec<StoreEntry> = Vec::new();
    let mut i: usize = 0;
    while i < buffer.len()
        invariant
            0 <= i <= buffer@.len(),
            result@ == buffer@.subrange(0, i as int),
        decreases buffer@.len() - i,
    {
        result.push(buffer[i]);
        i += 1;
    }
    result
}

fn clone_from(buffer: &[StoreEntry], from: usize) -> (result: Vec<StoreEntry>)
    requires from <= buffer@.len()
    ensures result@ == buffer@.subrange(from as int, buffer@.len() as int)
{
    let mut result: Vec<StoreEntry> = Vec::new();
    let mut i: usize = from;
    while i < buffer.len()
        invariant
            from <= i <= buffer@.len(),
            result@ == buffer@.subrange(from as int, i as int),
        decreases buffer@.len() - i,
    {
        result.push(buffer[i]);
        i += 1;
    }
    result
}

// r[depends store-buffer.input-domain]
pub open spec fn valid_access_size_spec(size: u64) -> bool {
    MIN_ACCESS_BYTES <= size <= MAX_ACCESS_BYTES
}

// r[impl store-buffer.input-domain]
fn valid_access_size(size: u64) -> (result: bool)
    ensures result == valid_access_size_spec(size)
{
    MIN_ACCESS_BYTES <= size && size <= MAX_ACCESS_BYTES
}

// r[depends store-buffer.input-domain]
pub open spec fn valid_entry_spec(entry: StoreEntry) -> bool {
    valid_access_size_spec(entry.size)
}

// r[impl store-buffer.input-domain]
pub open spec fn well_formed_buffer_spec(buffer: Seq<StoreEntry>) -> bool {
    &&& forall|i: int| 0 <= i < buffer.len() ==> valid_entry_spec(buffer[i])
    &&& forall|i: int, j: int| 0 <= i < j < buffer.len() ==> buffer[i].seq < buffer[j].seq
}

pub open spec fn valid_buffer_spec(buffer: Seq<StoreEntry>, capacity: nat) -> bool {
    &&& buffer.len() <= capacity
    &&& well_formed_buffer_spec(buffer)
}

pub open spec fn next_entry_seq_spec(buffer: Seq<StoreEntry>, entry: StoreEntry) -> bool {
    &&& valid_entry_spec(entry)
    &&& (buffer.len() == 0 || buffer[buffer.len() - 1].seq < entry.seq)
}

pub open spec fn can_forward_spec(entry: StoreEntry, load_addr: u64, load_size: u64) -> bool {
    let offset = load_addr as int - entry.addr as int;
    &&& valid_entry_spec(entry)
    &&& valid_access_size_spec(load_size)
    &&& entry.addr <= load_addr
    &&& 0 <= offset <= entry.size as int
    &&& load_size as int <= entry.size as int - offset
}

pub open spec fn enqueue_spec(buffer: Seq<StoreEntry>, entry: StoreEntry, capacity: nat) -> Option<Seq<StoreEntry>> {
    if buffer.len() < capacity && next_entry_seq_spec(buffer, entry) {
        Some(buffer.push(entry))
    } else {
        None
    }
}

pub open spec fn drain_oldest_spec(buffer: Seq<StoreEntry>) -> Option<(StoreEntry, Seq<StoreEntry>)> {
    if buffer.len() == 0 {
        None
    } else {
        Some((buffer[0], buffer.subrange(1, buffer.len() as int)))
    }
}

pub open spec fn forward_index_prefix_spec(
    buffer: Seq<StoreEntry>,
    upto: nat,
    load_addr: u64,
    load_size: u64,
) -> Option<int>
    recommends upto <= buffer.len()
    decreases upto
{
    if upto == 0 {
        None
    } else {
        let previous = forward_index_prefix_spec(buffer, (upto - 1) as nat, load_addr, load_size);
        let idx = (upto - 1) as int;
        if can_forward_spec(buffer[idx], load_addr, load_size) {
            Some(idx)
        } else {
            previous
        }
    }
}

pub open spec fn forward_lookup_prefix_spec(
    buffer: Seq<StoreEntry>,
    upto: nat,
    load_addr: u64,
    load_size: u64,
) -> Option<StoreEntry>
    recommends upto <= buffer.len()
{
    match forward_index_prefix_spec(buffer, upto, load_addr, load_size) {
        Some(idx) => Some(buffer[idx]),
        None => None,
    }
}

pub open spec fn forward_lookup_spec(buffer: Seq<StoreEntry>, load_addr: u64, load_size: u64) -> Option<StoreEntry> {
    forward_lookup_prefix_spec(buffer, buffer.len() as nat, load_addr, load_size)
}

pub open spec fn drain_order_spec(buffer: Seq<StoreEntry>) -> Seq<StoreEntry>
    decreases buffer.len()
{
    if buffer.len() == 0 {
        Seq::<StoreEntry>::empty()
    } else {
        seq![buffer[0]] + drain_order_spec(buffer.subrange(1, buffer.len() as int))
    }
}

pub open spec fn drain_all_remaining_spec(buffer: Seq<StoreEntry>) -> Seq<StoreEntry>
    decreases buffer.len()
{
    if buffer.len() == 0 {
        buffer
    } else {
        drain_all_remaining_spec(buffer.subrange(1, buffer.len() as int))
    }
}

// r[impl store-buffer.input-domain]
// r[impl store-buffer.capacity]
/// Append a new store entry when capacity permits.
pub fn enqueue(buffer: &[StoreEntry], entry: StoreEntry, capacity: usize) -> (result: Option<Vec<StoreEntry>>)
    requires
        valid_buffer_spec(buffer@, capacity as nat),
        next_entry_seq_spec(buffer@, entry),
    ensures
        result.is_none() <==> enqueue_spec(buffer@, entry, capacity as nat).is_none(),
        result.is_some() ==> result.unwrap()@ == enqueue_spec(buffer@, entry, capacity as nat).unwrap(),
        result.is_some() ==> valid_buffer_spec(result.unwrap()@, capacity as nat),
{
    if buffer.len() >= capacity {
        return None;
    }
    let mut next = clone_entries(buffer);
    next.push(entry);
    proof {
        push_preserves_well_formed_buffer(buffer@, entry);
    }
    Some(next)
}

// r[impl store-buffer.drain-ordering]
/// Remove the oldest buffered store.
pub fn drain_oldest(buffer: &[StoreEntry], capacity: usize) -> (result: Option<(StoreEntry, Vec<StoreEntry>)>)
    requires valid_buffer_spec(buffer@, capacity as nat)
    ensures
        result.is_none() <==> drain_oldest_spec(buffer@).is_none(),
        result.is_some() ==> result.unwrap().0 == drain_oldest_spec(buffer@).unwrap().0,
        result.is_some() ==> result.unwrap().1@ == drain_oldest_spec(buffer@).unwrap().1,
        result.is_some() ==> valid_buffer_spec(result.unwrap().1@, capacity as nat),
{
    if buffer.len() == 0 {
        return None;
    }
    let oldest = buffer[0];
    let remaining = clone_from(buffer, 1usize);
    proof {
        subrange_preserves_well_formed_buffer(buffer@, 1);
    }
    Some((oldest, remaining))
}

// r[impl store-buffer.full-coverage]
/// Full-coverage forwarding predicate for one buffered store.
pub fn can_forward(entry: StoreEntry, load_addr: u64, load_size: u64) -> (result: bool)
    ensures result == can_forward_spec(entry, load_addr, load_size)
{
    if !valid_access_size(entry.size) || !valid_access_size(load_size) {
        return false;
    }
    if entry.addr > load_addr {
        return false;
    }
    let offset = load_addr - entry.addr;
    if offset > entry.size {
        return false;
    }
    load_size <= entry.size - offset
}

// r[impl store-buffer.youngest-match]
// r[impl store-buffer.cache-fallback]
/// Return the youngest buffered store that fully covers the load.
pub fn forward_lookup(buffer: &[StoreEntry], load_addr: u64, load_size: u64) -> (result: Option<StoreEntry>)
    requires
        well_formed_buffer_spec(buffer@),
        valid_access_size_spec(load_size),
    ensures result == forward_lookup_spec(buffer@, load_addr, load_size)
{
    let mut candidate: Option<StoreEntry> = None;
    let mut i: usize = 0;
    while i < buffer.len()
        invariant
            0 <= i <= buffer@.len(),
            candidate == forward_lookup_prefix_spec(buffer@, i as nat, load_addr, load_size),
            valid_access_size_spec(load_size),
        decreases buffer@.len() - i,
    {
        let entry = buffer[i];
        let matches = can_forward(entry, load_addr, load_size);
        if matches {
            candidate = Some(entry);
            proof {
                assert(entry == buffer@[i as int]);
                assert(forward_lookup_prefix_spec(buffer@, i as nat + 1, load_addr, load_size) == Some(buffer@[i as int]));
            }
        } else {
            proof {
                assert(forward_lookup_prefix_spec(buffer@, i as nat + 1, load_addr, load_size)
                    == forward_lookup_prefix_spec(buffer@, i as nat, load_addr, load_size));
            }
        }
        i += 1;
    }
    candidate
}

// r[impl store-buffer.fence]
/// A store fence is satisfied exactly when the buffer is empty.
pub fn is_fence_satisfied(buffer: &[StoreEntry]) -> (result: bool)
    ensures result <==> buffer@.len() == 0
{
    buffer.len() == 0
}

proof fn seq_head_tail(buffer: Seq<StoreEntry>)
    requires buffer.len() > 0
    ensures buffer == seq![buffer[0]] + buffer.subrange(1, buffer.len() as int)
{
    assert forall|i: int| 0 <= i < buffer.len() implies #[trigger] buffer[i] == (seq![buffer[0]] + buffer.subrange(1, buffer.len() as int))[i] by {
        if i == 0 {
        } else {
            assert((seq![buffer[0]] + buffer.subrange(1, buffer.len() as int))[i]
                == buffer.subrange(1, buffer.len() as int)[i - 1]);
            assert(buffer.subrange(1, buffer.len() as int)[i - 1] == buffer[i]);
        }
    }
}

proof fn subrange_preserves_well_formed_buffer(buffer: Seq<StoreEntry>, from: int)
    requires
        well_formed_buffer_spec(buffer),
        0 <= from <= buffer.len(),
    ensures
        well_formed_buffer_spec(buffer.subrange(from, buffer.len() as int))
{
    assert forall|i: int| 0 <= i < buffer.subrange(from, buffer.len() as int).len() implies valid_entry_spec(buffer.subrange(from, buffer.len() as int)[i]) by {
        assert(buffer.subrange(from, buffer.len() as int)[i] == buffer[from + i]);
    }
    assert forall|i: int, j: int| 0 <= i < j < buffer.subrange(from, buffer.len() as int).len() implies buffer.subrange(from, buffer.len() as int)[i].seq < buffer.subrange(from, buffer.len() as int)[j].seq by {
        assert(buffer.subrange(from, buffer.len() as int)[i] == buffer[from + i]);
        assert(buffer.subrange(from, buffer.len() as int)[j] == buffer[from + j]);
    }
}

// r[verify store-buffer.input-domain]
proof fn push_preserves_well_formed_buffer(buffer: Seq<StoreEntry>, entry: StoreEntry)
    requires
        well_formed_buffer_spec(buffer),
        next_entry_seq_spec(buffer, entry),
    ensures
        well_formed_buffer_spec(buffer.push(entry))
{
    assert forall|i: int| 0 <= i < buffer.push(entry).len() implies valid_entry_spec(buffer.push(entry)[i]) by {
        if i < buffer.len() {
            assert(buffer.push(entry)[i] == buffer[i]);
        } else {
            assert(i == buffer.len());
        }
    }
    assert forall|i: int, j: int| 0 <= i < j < buffer.push(entry).len() implies buffer.push(entry)[i].seq < buffer.push(entry)[j].seq by {
        if j < buffer.len() {
            assert(buffer.push(entry)[i] == buffer[i]);
            assert(buffer.push(entry)[j] == buffer[j]);
        } else {
            assert(j == buffer.len());
            assert(buffer.push(entry)[i] == buffer[i]);
        }
    }
}

// r[verify store-buffer.full-coverage]
proof fn can_forward_implies_range_coverage(entry: StoreEntry, load_addr: u64, load_size: u64)
    requires can_forward_spec(entry, load_addr, load_size)
    ensures
        entry.addr as int <= load_addr as int,
        load_addr as int + load_size as int <= entry.addr as int + entry.size as int,
{
    let offset = load_addr as int - entry.addr as int;
    assert(load_addr as int + load_size as int == entry.addr as int + offset + load_size as int);
    assert(offset + load_size as int <= entry.size as int);
}

// r[verify store-buffer.full-coverage]
proof fn can_forward_nonwrapping_boundary(entry: StoreEntry, load_addr: u64, load_size: u64)
    requires can_forward_spec(entry, load_addr, load_size)
    ensures load_addr as int + load_size as int <= entry.addr as int + entry.size as int
{
    can_forward_implies_range_coverage(entry, load_addr, load_size);
}

// r[verify store-buffer.youngest-match]
// r[verify store-buffer.cache-fallback]
proof fn forward_index_prefix_maximal(
    buffer: Seq<StoreEntry>,
    upto: nat,
    load_addr: u64,
    load_size: u64,
)
    requires upto <= buffer.len()
    ensures
        match forward_index_prefix_spec(buffer, upto, load_addr, load_size) {
            Some(idx) => {
                &&& 0 <= idx < upto as int
                &&& can_forward_spec(buffer[idx], load_addr, load_size)
                &&& forall|j: int| 0 <= j < upto as int && can_forward_spec(buffer[j], load_addr, load_size) ==> j <= idx
            },
            None => forall|j: int| 0 <= j < upto as int ==> !can_forward_spec(buffer[j], load_addr, load_size),
        }
    decreases upto
{
    if upto == 0 {
    } else {
        forward_index_prefix_maximal(buffer, (upto - 1) as nat, load_addr, load_size);
        let idx = (upto - 1) as int;
        if can_forward_spec(buffer[idx], load_addr, load_size) {
            assert(forward_index_prefix_spec(buffer, upto, load_addr, load_size) == Some(idx));
            assert forall|j: int| 0 <= j < upto as int && can_forward_spec(buffer[j], load_addr, load_size) implies j <= idx by {
            }
        } else {
            match forward_index_prefix_spec(buffer, (upto - 1) as nat, load_addr, load_size) {
                Some(prev) => {
                    assert(forward_index_prefix_spec(buffer, upto, load_addr, load_size) == Some(prev));
                    assert forall|j: int| 0 <= j < upto as int && can_forward_spec(buffer[j], load_addr, load_size) implies j <= prev by {
                        if j == idx {
                            assert(false);
                        }
                    }
                },
                None => {
                    assert(forward_index_prefix_spec(buffer, upto, load_addr, load_size).is_none());
                    assert forall|j: int| 0 <= j < upto as int implies !can_forward_spec(buffer[j], load_addr, load_size) by {
                        if j == idx {
                            assert(!can_forward_spec(buffer[idx], load_addr, load_size));
                        }
                    }
                },
            }
        }
    }
}

// r[verify store-buffer.youngest-match]
// r[verify store-buffer.cache-fallback]
proof fn forward_lookup_youngest_match(buffer: Seq<StoreEntry>, load_addr: u64, load_size: u64)
    requires
        well_formed_buffer_spec(buffer),
        valid_access_size_spec(load_size),
    ensures
        match forward_lookup_spec(buffer, load_addr, load_size) {
            Some(entry) => {
                &&& can_forward_spec(entry, load_addr, load_size)
                &&& forall|j: int| 0 <= j < buffer.len() && can_forward_spec(buffer[j], load_addr, load_size) ==> buffer[j].seq <= entry.seq
            },
            None => forall|j: int| 0 <= j < buffer.len() ==> !can_forward_spec(buffer[j], load_addr, load_size),
        }
{
    let idx_opt = forward_index_prefix_spec(buffer, buffer.len() as nat, load_addr, load_size);
    forward_index_prefix_maximal(buffer, buffer.len() as nat, load_addr, load_size);
    match idx_opt {
        Some(idx) => {
            assert(forward_lookup_spec(buffer, load_addr, load_size) == Some(buffer[idx]));
            assert(can_forward_spec(buffer[idx], load_addr, load_size));
            assert forall|j: int| 0 <= j < buffer.len() && can_forward_spec(buffer[j], load_addr, load_size) implies buffer[j].seq <= buffer[idx].seq by {
                assert(j <= idx);
                if j < idx {
                    assert(buffer[j].seq < buffer[idx].seq);
                }
            }
        },
        None => {
            assert(forward_lookup_spec(buffer, load_addr, load_size).is_none());
            assert forall|j: int| 0 <= j < buffer.len() implies !can_forward_spec(buffer[j], load_addr, load_size) by {
            }
        }
    }
}

// r[verify store-buffer.drain-ordering]
proof fn drain_order_is_identity(buffer: Seq<StoreEntry>)
    ensures drain_order_spec(buffer) == buffer
    decreases buffer.len()
{
    if buffer.len() == 0 {
    } else {
        let tail = buffer.subrange(1, buffer.len() as int);
        drain_order_is_identity(tail);
        seq_head_tail(buffer);
        assert(drain_order_spec(buffer) == seq![buffer[0]] + drain_order_spec(tail));
        assert(drain_order_spec(tail) == tail);
    }
}

// r[verify store-buffer.drain-ordering]
proof fn drain_preserves_sequence_numbers(buffer: Seq<StoreEntry>, capacity: nat)
    requires
        valid_buffer_spec(buffer, capacity),
        buffer.len() > 0,
    ensures
        forall|i: int| 0 <= i < buffer.subrange(1, buffer.len() as int).len()
            ==> buffer.subrange(1, buffer.len() as int)[i].seq == buffer[i + 1].seq,
{
    assert forall|i: int| 0 <= i < buffer.subrange(1, buffer.len() as int).len()
        implies buffer.subrange(1, buffer.len() as int)[i].seq == buffer[i + 1].seq by {
        assert(buffer.subrange(1, buffer.len() as int)[i] == buffer[i + 1]);
    }
}

// r[verify store-buffer.fence]
proof fn drain_all_remaining_is_empty(buffer: Seq<StoreEntry>)
    ensures
        drain_all_remaining_spec(buffer).len() == 0,
        is_fence_satisfied_spec(drain_all_remaining_spec(buffer)),
    decreases buffer.len()
{
    if buffer.len() == 0 {
    } else {
        drain_all_remaining_is_empty(buffer.subrange(1, buffer.len() as int));
    }
}

pub open spec fn is_fence_satisfied_spec(buffer: Seq<StoreEntry>) -> bool {
    buffer.len() == 0
}

// r[verify store-buffer.drain-ordering]
proof fn drain_empty_preserves_buffer(buffer: Seq<StoreEntry>)
    requires buffer.len() == 0
    ensures drain_oldest_spec(buffer).is_none()
{
}

// r[verify store-buffer.fence]
proof fn empty_buffer_satisfies_fence(buffer: Seq<StoreEntry>)
    requires buffer.len() == 0
    ensures is_fence_satisfied_spec(buffer)
{
}

// r[verify store-buffer.fence]
proof fn nonempty_buffer_blocks_fence(buffer: Seq<StoreEntry>)
    requires buffer.len() > 0
    ensures !is_fence_satisfied_spec(buffer)
{
}

// r[verify store-buffer.fence]
proof fn post_fence_store_enters_empty_buffer(entry: StoreEntry, capacity: nat)
    requires
        valid_entry_spec(entry),
        capacity > 0,
    ensures
        enqueue_spec(Seq::<StoreEntry>::empty(), entry, capacity).is_some(),
        enqueue_spec(Seq::<StoreEntry>::empty(), entry, capacity).unwrap() == seq![entry],
{
}

// r[verify store-buffer.capacity]
proof fn enqueue_full_returns_none(buffer: Seq<StoreEntry>, entry: StoreEntry, capacity: nat)
    requires
        valid_buffer_spec(buffer, capacity),
        next_entry_seq_spec(buffer, entry),
        buffer.len() >= capacity,
    ensures
        enqueue_spec(buffer, entry, capacity).is_none(),
{
}

// r[verify store-buffer.capacity]
proof fn enqueue_full_preserves_buffer(buffer: Seq<StoreEntry>, entry: StoreEntry, capacity: nat)
    requires
        valid_buffer_spec(buffer, capacity),
        next_entry_seq_spec(buffer, entry),
        buffer.len() >= capacity,
    ensures
        enqueue_spec(buffer, entry, capacity).is_none(),
{
}

// r[verify store-buffer.capacity]
proof fn enqueue_nonfull_grows(buffer: Seq<StoreEntry>, entry: StoreEntry, capacity: nat)
    requires
        valid_buffer_spec(buffer, capacity),
        next_entry_seq_spec(buffer, entry),
        buffer.len() < capacity,
    ensures
        enqueue_spec(buffer, entry, capacity).unwrap().len() == buffer.len() + 1,
{
}

// r[verify store-buffer.capacity]
proof fn drain_nonempty_shrinks(buffer: Seq<StoreEntry>, capacity: nat)
    requires
        valid_buffer_spec(buffer, capacity),
        buffer.len() > 0,
    ensures
        drain_oldest_spec(buffer).unwrap().1.len() + 1 == buffer.len(),
{
}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    const TEST_CAPACITY_SMALL: usize = 2usize;
    const TEST_CAPACITY_MEDIUM: usize = 4usize;
    const TEST_CAPACITY_PROP: usize = 6usize;
    const TEST_OPERATION_COUNT: usize = 16usize;
    const TEST_BUFFER_MAX_LEN: usize = 6usize;
    const TEST_ADDRESS_BASE: u64 = 10u64;
    const TEST_ADDRESS_NEXT: u64 = 12u64;
    const TEST_ADDRESS_FAR: u64 = 20u64;
    const TEST_LOAD_OFFSET_ONE: u64 = 1u64;
    const TEST_LOAD_OFFSET_TWO: u64 = 2u64;
    const TEST_SEQ_BASE: u64 = 100u64;
    const TEST_SEQ_STEP: u64 = 1u64;
    const HALFWORD_WIDTH: u64 = 2u64;
    const WORD_WIDTH: u64 = 4u64;
    const DOUBLEWORD_WIDTH: u64 = MAX_ACCESS_BYTES;
    const DATA_ALPHA: u64 = 11u64;
    const DATA_BETA: u64 = 22u64;
    const DATA_GAMMA: u64 = 33u64;
    const DATA_DELTA: u64 = 44u64;
    const PROP_MAX_ADDRESS: u64 = 32u64;
    const HIGH_ADDRESS_BASE: u64 = u64::MAX - WORD_WIDTH + TEST_SEQ_STEP;

    fn make_entry(addr: u64, size: u64, data: u64, seq: u64) -> StoreEntry {
        StoreEntry { addr, size, data, seq }
    }

    fn build_buffer(raw: &[(u64, u64, u64)]) -> Vec<StoreEntry> {
        let mut buffer = Vec::new();
        for (seq, &(addr, size, data)) in (TEST_SEQ_BASE..).zip(raw.iter()) {
            buffer.push(make_entry(addr, size, data, seq));
        }
        buffer
    }

    fn brute_force_forward(buffer: &[StoreEntry], load_addr: u64, load_size: u64) -> Option<StoreEntry> {
        buffer.iter().rev().copied().find(|entry| {
            if !valid_access_size(entry.size) || !valid_access_size(load_size) {
                return false;
            }
            if entry.addr > load_addr {
                return false;
            }
            let offset = load_addr - entry.addr;
            if offset > entry.size {
                return false;
            }
            load_size <= entry.size - offset
        })
    }

    // r[verify store-buffer.input-domain]
    // r[verify store-buffer.capacity]
    #[test]
    fn test_enqueue_and_drain_roundtrip() {
        let empty: Vec<StoreEntry> = Vec::new();
        let first = make_entry(TEST_ADDRESS_BASE, WORD_WIDTH, DATA_ALPHA, TEST_SEQ_BASE);
        let second = make_entry(TEST_ADDRESS_NEXT, WORD_WIDTH, DATA_BETA, TEST_SEQ_BASE + TEST_SEQ_STEP);

        let with_first = enqueue(&empty, first, TEST_CAPACITY_MEDIUM).unwrap();
        let with_second = enqueue(&with_first, second, TEST_CAPACITY_MEDIUM).unwrap();

        let drained_first = drain_oldest(&with_second, TEST_CAPACITY_MEDIUM).unwrap();
        assert_eq!(drained_first.0, first);
        let drained_second = drain_oldest(&drained_first.1, TEST_CAPACITY_MEDIUM).unwrap();
        assert_eq!(drained_second.0, second);
        assert_eq!(drain_oldest(&drained_second.1, TEST_CAPACITY_MEDIUM), None);
    }

    // r[verify store-buffer.capacity]
    #[test]
    fn test_enqueue_full_returns_none() {
        let first = make_entry(TEST_ADDRESS_BASE, WORD_WIDTH, DATA_ALPHA, TEST_SEQ_BASE);
        let second = make_entry(TEST_ADDRESS_NEXT, WORD_WIDTH, DATA_BETA, TEST_SEQ_BASE + TEST_SEQ_STEP);
        let third = make_entry(TEST_ADDRESS_FAR, WORD_WIDTH, DATA_GAMMA, TEST_SEQ_BASE + TEST_SEQ_STEP + TEST_SEQ_STEP);

        let empty: Vec<StoreEntry> = Vec::new();
        let full_once = enqueue(&empty, first, TEST_CAPACITY_SMALL).unwrap();
        let full_twice = enqueue(&full_once, second, TEST_CAPACITY_SMALL).unwrap();
        assert_eq!(enqueue(&full_twice, third, TEST_CAPACITY_SMALL), None);
    }

    // r[verify store-buffer.drain-ordering]
    #[test]
    fn test_drain_empty_returns_none() {
        let empty: Vec<StoreEntry> = Vec::new();
        assert_eq!(drain_oldest(&empty, TEST_CAPACITY_SMALL), None);
    }

    // r[verify store-buffer.full-coverage]
    #[test]
    fn test_forward_exact_match() {
        let buffer = vec![
            make_entry(TEST_ADDRESS_BASE, WORD_WIDTH, DATA_ALPHA, TEST_SEQ_BASE),
        ];
        assert_eq!(
            forward_lookup(&buffer, TEST_ADDRESS_BASE, WORD_WIDTH),
            Some(buffer[0])
        );
    }

    // r[verify store-buffer.full-coverage]
    #[test]
    fn test_forward_subrange_match() {
        let buffer = vec![
            make_entry(TEST_ADDRESS_BASE, WORD_WIDTH, DATA_ALPHA, TEST_SEQ_BASE),
        ];
        assert_eq!(
            forward_lookup(&buffer, TEST_ADDRESS_BASE + TEST_LOAD_OFFSET_ONE, HALFWORD_WIDTH),
            Some(buffer[0])
        );
    }

    // r[verify store-buffer.full-coverage]
    #[test]
    fn test_forward_partial_rejection() {
        let buffer = vec![
            make_entry(TEST_ADDRESS_BASE, HALFWORD_WIDTH, DATA_ALPHA, TEST_SEQ_BASE),
        ];
        assert_eq!(
            forward_lookup(&buffer, TEST_ADDRESS_BASE, WORD_WIDTH),
            None
        );
    }

    // r[verify store-buffer.full-coverage]
    #[test]
    fn test_forward_reversed_address_rejection() {
        let buffer = vec![
            make_entry(TEST_ADDRESS_NEXT, WORD_WIDTH, DATA_ALPHA, TEST_SEQ_BASE),
        ];
        assert_eq!(
            forward_lookup(&buffer, TEST_ADDRESS_BASE, HALFWORD_WIDTH),
            None
        );
    }

    // r[verify store-buffer.full-coverage]
    #[test]
    fn test_forward_high_address_nonwrapping_boundary() {
        let buffer = vec![
            make_entry(HIGH_ADDRESS_BASE, WORD_WIDTH, DATA_ALPHA, TEST_SEQ_BASE),
        ];
        assert_eq!(
            forward_lookup(&buffer, HIGH_ADDRESS_BASE + TEST_LOAD_OFFSET_ONE, HALFWORD_WIDTH),
            Some(buffer[0])
        );
    }

    // r[verify store-buffer.youngest-match]
    #[test]
    fn test_forward_picks_youngest_match() {
        let older = make_entry(TEST_ADDRESS_BASE, DOUBLEWORD_WIDTH, DATA_ALPHA, TEST_SEQ_BASE);
        let younger = make_entry(TEST_ADDRESS_BASE, DOUBLEWORD_WIDTH, DATA_BETA, TEST_SEQ_BASE + TEST_SEQ_STEP);
        let buffer = vec![older, younger];
        assert_eq!(
            forward_lookup(&buffer, TEST_ADDRESS_BASE + TEST_LOAD_OFFSET_TWO, HALFWORD_WIDTH),
            Some(younger)
        );
    }

    // r[verify store-buffer.cache-fallback]
    #[test]
    fn test_forward_miss_returns_none() {
        let buffer = vec![
            make_entry(TEST_ADDRESS_BASE, WORD_WIDTH, DATA_ALPHA, TEST_SEQ_BASE),
        ];
        assert_eq!(forward_lookup(&buffer, TEST_ADDRESS_FAR, WORD_WIDTH), None);
    }

    // r[verify store-buffer.fence]
    #[test]
    fn test_fence_matches_emptiness() {
        let empty: Vec<StoreEntry> = Vec::new();
        let non_empty = vec![make_entry(TEST_ADDRESS_BASE, WORD_WIDTH, DATA_DELTA, TEST_SEQ_BASE)];
        assert!(is_fence_satisfied(&empty));
        assert!(!is_fence_satisfied(&non_empty));
    }

    proptest! {
        // r[verify store-buffer.youngest-match]
        // r[verify store-buffer.full-coverage]
        #[test]
        fn prop_forward_lookup_matches_oracle(
            raw in prop::collection::vec(
                (
                    0u64..PROP_MAX_ADDRESS,
                    MIN_ACCESS_BYTES..=MAX_ACCESS_BYTES,
                    any::<u64>(),
                ),
                0..TEST_BUFFER_MAX_LEN,
            ),
            load_addr in 0u64..PROP_MAX_ADDRESS,
            load_size in MIN_ACCESS_BYTES..=MAX_ACCESS_BYTES,
        ) {
            let buffer = build_buffer(&raw);
            let expected = brute_force_forward(&buffer, load_addr, load_size);
            let actual = forward_lookup(&buffer, load_addr, load_size);
            prop_assert_eq!(actual, expected);
        }

        // r[verify store-buffer.drain-ordering]
        #[test]
        fn prop_fifo_order_preserved(
            raw in prop::collection::vec(
                (
                    0u64..PROP_MAX_ADDRESS,
                    MIN_ACCESS_BYTES..=MAX_ACCESS_BYTES,
                    any::<u64>(),
                ),
                0..TEST_BUFFER_MAX_LEN,
            ),
        ) {
            let buffer = build_buffer(&raw);
            let mut queue = Vec::new();
            for entry in &buffer {
                queue = enqueue(&queue, *entry, TEST_CAPACITY_PROP).unwrap();
            }
            let mut drained = Vec::new();
            while let Some((entry, rest)) = drain_oldest(&queue, TEST_CAPACITY_PROP) {
                drained.push(entry);
                queue = rest;
            }
            prop_assert_eq!(drained, buffer);
        }

        // r[verify store-buffer.capacity]
        #[test]
        fn prop_buffer_size_never_exceeds_capacity(
            ops in prop::collection::vec(
                (
                    any::<bool>(),
                    0u64..PROP_MAX_ADDRESS,
                    MIN_ACCESS_BYTES..=MAX_ACCESS_BYTES,
                    any::<u64>(),
                ),
                0..TEST_OPERATION_COUNT,
            ),
        ) {
            let mut buffer: Vec<StoreEntry> = Vec::new();
            let mut seq = TEST_SEQ_BASE;
            for (do_enqueue, addr, size, data) in ops {
                if do_enqueue {
                    let entry = make_entry(addr, size, data, seq);
                    if let Some(next) = enqueue(&buffer, entry, TEST_CAPACITY_SMALL) {
                        buffer = next;
                        seq += TEST_SEQ_STEP;
                    }
                } else if let Some((_, rest)) = drain_oldest(&buffer, TEST_CAPACITY_SMALL) {
                    buffer = rest;
                }
                prop_assert!(buffer.len() <= TEST_CAPACITY_SMALL);
            }
        }
    }
}
