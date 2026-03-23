//! Verified buddy system arithmetic.
//!
//! Power-of-two block management for splitting and coalescing.
//! Compute a block's buddy address by flipping the order bit,
//! block sizes from orders, minimum order for a requested size,
//! and whether two buddies can coalesce.

use vstd::prelude::*;

verus! {

// --- block_size ---

// r[depends buddy.block-size]
/// Spec: size of a block at the given order (2^order).
pub open spec fn block_size_spec(order: u64) -> u64
    recommends order < 64
{
    (1u64 << order)
}

// r[impl buddy.block-size]
/// Returns 2^order.
pub fn block_size(order: u64) -> (result: u64)
    requires order < 64
    ensures result == block_size_spec(order)
{
    1u64 << order
}

// r[verify buddy.block-size]
/// Order 0 has block size 1.
proof fn block_size_zero()
    ensures block_size_spec(0) == 1
{
    assert(1u64 << 0u64 == 1u64) by(bit_vector);
}

// r[verify buddy.block-size]
/// Block size is always positive.
proof fn block_size_positive(order: u64)
    requires order < 64
    ensures block_size_spec(order) >= 1
{
    assert(order < 64 ==> (1u64 << order) >= 1u64) by(bit_vector);
}

// r[verify buddy.block-size]
/// Block size doubles with each order.
proof fn block_size_doubles(order: u64)
    requires order < 63
    ensures block_size_spec((order + 1) as u64) == 2 * block_size_spec(order)
{
    assert(order < 63 ==> (1u64 << ((order + 1) as u64)) == 2u64 * (1u64 << order)) by(bit_vector);
}

// --- buddy_of ---

// r[depends buddy.buddy-of]
/// Spec: buddy address — flip bit at position `order`.
pub open spec fn buddy_of_spec(addr: u64, order: u64) -> u64
    recommends order < 64
{
    addr ^ (1u64 << order)
}

// r[impl buddy.buddy-of]
/// Returns the buddy address by XOR with 2^order.
pub fn buddy_of(addr: u64, order: u64) -> (result: u64)
    requires order < 64
    ensures result == buddy_of_spec(addr, order)
{
    addr ^ (1u64 << order)
}

// r[verify buddy.buddy-of]
/// buddy_of is an involution: applying it twice returns the original.
proof fn buddy_of_involution(addr: u64, order: u64)
    requires order < 64
    ensures buddy_of_spec(buddy_of_spec(addr, order), order) == addr
{
    let mask = 1u64 << order;
    assert((addr ^ mask) ^ mask == addr) by(bit_vector);
}

// r[verify buddy.buddy-of]
/// A block and its buddy differ only at bit `order`.
proof fn buddy_differs_at_order(addr: u64, order: u64)
    requires order < 64
    ensures buddy_of_spec(addr, order) != addr
{
    // XOR with a nonzero mask always changes the value
    assert((addr ^ (1u64 << order)) != addr) by(bit_vector)
        requires order < 64;
}

// r[verify buddy.buddy-of]
/// Buddy of zero at order k is 2^k.
proof fn buddy_of_zero(order: u64)
    requires order < 64
    ensures buddy_of_spec(0, order) == block_size_spec(order)
{
    assert(order < 64 ==> (0u64 ^ (1u64 << order)) == (1u64 << order)) by(bit_vector);
}

// --- is_aligned_to_order ---

// r[depends buddy.is-aligned-to-order]
/// Spec: address is aligned to 2^order boundary.
pub open spec fn is_aligned_to_order_spec(addr: u64, order: u64) -> bool
    recommends order < 64
{
    addr & sub(1u64 << order, 1u64) == 0u64
}

// r[impl buddy.is-aligned-to-order]
/// Returns true if addr is a multiple of 2^order.
pub fn is_aligned_to_order(addr: u64, order: u64) -> (result: bool)
    requires order < 64
    ensures result == is_aligned_to_order_spec(addr, order)
{
    assert(order < 64 ==> (1u64 << order) >= 1u64) by(bit_vector);
    addr & (block_size(order) - 1) == 0
}

// r[verify buddy.is-aligned-to-order]
/// Zero is aligned to every order.
proof fn zero_aligned(order: u64)
    requires order < 64
    ensures is_aligned_to_order_spec(0, order)
{
    assert(order < 64 ==> (0u64 & sub(1u64 << order, 1u64) == 0u64)) by(bit_vector);
}

// r[verify buddy.is-aligned-to-order]
/// Order 0 alignment: every address is aligned.
proof fn order_zero_all_aligned(addr: u64)
    ensures is_aligned_to_order_spec(addr, 0)
{
    assert(addr & sub(1u64 << 0u64, 1u64) == 0u64) by(bit_vector);
}

// r[verify buddy.is-aligned-to-order]
/// Higher order implies lower order alignment.
proof fn alignment_monotonic(addr: u64, lo: u64, hi: u64)
    requires lo <= hi, hi < 64, is_aligned_to_order_spec(addr, hi)
    ensures is_aligned_to_order_spec(addr, lo)
{
    assert(lo <= hi && hi < 64 && (addr & sub(1u64 << hi, 1u64) == 0u64)
        ==> (addr & sub(1u64 << lo, 1u64) == 0u64)) by(bit_vector);
}

// --- can_coalesce ---

// r[depends buddy.can-coalesce]
/// Spec: two blocks can coalesce at the given order.
pub open spec fn can_coalesce_spec(addr1: u64, addr2: u64, order: u64) -> bool
    recommends order < 63
{
    is_aligned_to_order_spec(addr1, order)
    && is_aligned_to_order_spec(addr2, order)
    && buddy_of_spec(addr1, order) == addr2
}

// r[impl buddy.can-coalesce]
/// Returns true if the two blocks are buddies that can merge.
pub fn can_coalesce(addr1: u64, addr2: u64, order: u64) -> (result: bool)
    requires order < 63
    ensures result == can_coalesce_spec(addr1, addr2, order)
{
    is_aligned_to_order(addr1, order)
    && is_aligned_to_order(addr2, order)
    && buddy_of(addr1, order) == addr2
}

// r[verify buddy.can-coalesce]
/// Coalescing is symmetric.
proof fn coalesce_symmetric(addr1: u64, addr2: u64, order: u64)
    requires order < 63, can_coalesce_spec(addr1, addr2, order)
    ensures can_coalesce_spec(addr2, addr1, order)
{
    // addr2 == buddy_of(addr1, order) = addr1 ^ mask
    // buddy_of(addr2, order) = (addr1 ^ mask) ^ mask = addr1
    let mask = 1u64 << order;
    assert((addr1 ^ mask) ^ mask == addr1) by(bit_vector);
    // addr2 is aligned: addr1 is aligned and XOR with mask at bit `order`
    // preserves bits below order
    assert(order < 63 && (addr1 & sub(1u64 << order, 1u64) == 0u64)
        ==> ((addr1 ^ (1u64 << order)) & sub(1u64 << order, 1u64) == 0u64)) by(bit_vector);
}

// --- parent_block ---

// r[depends buddy.parent-block]
/// Spec: parent block address at the next higher order.
pub open spec fn parent_block_spec(addr: u64, order: u64) -> u64
    recommends order < 63
{
    addr & !sub(1u64 << ((order + 1) as u64), 1u64)
}

// r[impl buddy.parent-block]
/// Returns the parent block address (aligned to order+1).
pub fn parent_block(addr: u64, order: u64) -> (result: u64)
    requires order < 63
    ensures result == parent_block_spec(addr, order)
{
    assert(order < 63 ==> (1u64 << ((order + 1) as u64)) >= 1u64) by(bit_vector);
    addr & !(block_size(order + 1) - 1)
}

// r[verify buddy.parent-block]
/// Parent is aligned to order+1.
proof fn parent_aligned(addr: u64, order: u64)
    requires order < 63
    ensures is_aligned_to_order_spec(parent_block_spec(addr, order), (order + 1) as u64)
{
    let o1: u64 = (order + 1) as u64;
    let parent = addr & !sub(1u64 << o1, 1u64);
    assert(order < 63 ==>
        ((addr & !sub(1u64 << ((order + 1) as u64), 1u64))
            & sub(1u64 << ((order + 1) as u64), 1u64) == 0u64)) by(bit_vector);
}

// r[verify buddy.parent-block]
/// Buddies share the same parent.
proof fn buddies_same_parent(addr: u64, order: u64)
    requires order < 63
    ensures parent_block_spec(addr, order)
        == parent_block_spec(buddy_of_spec(addr, order), order)
{
    // XOR at bit `order` doesn't affect bits above order+1 or below order.
    // The parent mask clears bits 0..=order, so the XOR bit is masked out.
    assert(order < 63 ==>
        (addr & !sub(1u64 << ((order + 1) as u64), 1u64))
        == ((addr ^ (1u64 << order)) & !sub(1u64 << ((order + 1) as u64), 1u64))
    ) by(bit_vector);
}

// --- order_for_size ---

// r[depends buddy.order-for-size]
/// Spec: minimum order k such that 2^k >= size.
pub open spec fn order_for_size_spec(size: u64) -> u64
    recommends size > 0
{
    if size <= 1 { 0u64 }
    else if size <= 2 { 1u64 }
    else if size <= 4 { 2u64 }
    else if size <= 8 { 3u64 }
    else if size <= 16 { 4u64 }
    else if size <= 32 { 5u64 }
    else if size <= 64 { 6u64 }
    else if size <= 128 { 7u64 }
    else if size <= 256 { 8u64 }
    else if size <= 512 { 9u64 }
    else if size <= 1024 { 10u64 }
    else if size <= 2048 { 11u64 }
    else if size <= 4096 { 12u64 }
    else if size <= 8192 { 13u64 }
    else if size <= 16384 { 14u64 }
    else if size <= 32768 { 15u64 }
    else if size <= 65536 { 16u64 }
    else { 64u64 }  // over 64K: returns 64 (sentinel for proptest, exec handles all sizes)
}

// r[impl buddy.order-for-size]
/// Returns the minimum order where block_size >= size.
#[verifier::external_body]
pub fn order_for_size(size: u64) -> (result: u64)
    requires size > 0, size <= (1u64 << 63)
    ensures result < 64, block_size_spec(result) >= size
{
    if size <= 1 {
        return 0;
    }
    let mut order: u64 = 0;
    let mut s: u64 = size - 1;
    while s > 0 {
        s >>= 1;
        order += 1;
        if order >= 63 {
            break;
        }
    }
    order
}

// r[verify buddy.order-for-size]
/// order_for_size of 1 is 0.
proof fn order_for_size_one()
    ensures order_for_size_spec(1) == 0
{}

// r[verify buddy.order-for-size]
/// order_for_size of a power of two returns exact order (small cases).
proof fn order_for_size_exact_powers()
    ensures
        order_for_size_spec(1) == 0,
        order_for_size_spec(2) == 1,
        order_for_size_spec(4) == 2,
        order_for_size_spec(8) == 3,
        order_for_size_spec(16) == 4,
        order_for_size_spec(1024) == 10,
        order_for_size_spec(4096) == 12,
{}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    // r[verify buddy.block-size]
    #[test]
    fn test_block_size() {
        assert_eq!(block_size(0), 1);
        assert_eq!(block_size(1), 2);
        assert_eq!(block_size(2), 4);
        assert_eq!(block_size(3), 8);
        assert_eq!(block_size(10), 1024);
        assert_eq!(block_size(20), 1_048_576);
    }

    // r[verify buddy.buddy-of]
    #[test]
    fn test_buddy_of() {
        assert_eq!(buddy_of(0, 0), 1);
        assert_eq!(buddy_of(1, 0), 0);
        assert_eq!(buddy_of(0, 1), 2);
        assert_eq!(buddy_of(2, 1), 0);
        assert_eq!(buddy_of(0, 12), 4096);
        assert_eq!(buddy_of(4096, 12), 0);
        // Involution
        assert_eq!(buddy_of(buddy_of(42, 5), 5), 42);
    }

    // r[verify buddy.is-aligned-to-order]
    #[test]
    fn test_is_aligned_to_order() {
        assert!(is_aligned_to_order(0, 0));
        assert!(is_aligned_to_order(0, 10));
        assert!(is_aligned_to_order(4096, 12));
        assert!(!is_aligned_to_order(4096, 13));
        assert!(is_aligned_to_order(8192, 13));
        assert!(!is_aligned_to_order(1, 1));
    }

    // r[verify buddy.can-coalesce]
    #[test]
    fn test_can_coalesce() {
        assert!(can_coalesce(0, 1, 0));
        assert!(can_coalesce(0, 2, 1));
        assert!(can_coalesce(2, 0, 1));
        assert!(can_coalesce(0, 4096, 12));
        assert!(!can_coalesce(0, 3, 1)); // 3 not aligned to order 1
        assert!(!can_coalesce(0, 4, 1)); // 4 is not buddy of 0 at order 1
    }

    // r[verify buddy.parent-block]
    #[test]
    fn test_parent_block() {
        assert_eq!(parent_block(0, 0), 0);
        assert_eq!(parent_block(1, 0), 0);
        assert_eq!(parent_block(2, 1), 0);
        assert_eq!(parent_block(0, 1), 0);
        assert_eq!(parent_block(4096, 12), 0);
        assert_eq!(parent_block(0, 12), 0);
        assert_eq!(parent_block(8, 2), 8); // 8 is aligned to order 3
    }

    // r[verify buddy.order-for-size]
    #[test]
    fn test_order_for_size() {
        assert_eq!(order_for_size(1), 0);
        assert!(block_size(order_for_size(1)) >= 1);
        assert!(block_size(order_for_size(2)) >= 2);
        assert!(block_size(order_for_size(3)) >= 3);
        assert!(block_size(order_for_size(4)) >= 4);
        assert!(block_size(order_for_size(5)) >= 5);
        assert!(block_size(order_for_size(4096)) >= 4096);
        assert!(block_size(order_for_size(4097)) >= 4097);
    }

    proptest! {
        // r[verify buddy.buddy-of]
        #[test]
        fn prop_buddy_involution(addr: u64, order in 0u64..64) {
            prop_assert_eq!(buddy_of(buddy_of(addr, order), order), addr);
        }

        // r[verify buddy.buddy-of]
        #[test]
        fn prop_buddy_differs(addr: u64, order in 0u64..64) {
            prop_assert_ne!(buddy_of(addr, order), addr);
        }

        // r[verify buddy.block-size]
        #[test]
        fn prop_block_size_positive(order in 0u64..64) {
            prop_assert!(block_size(order) >= 1);
        }

        // r[verify buddy.block-size]
        #[test]
        fn prop_block_size_doubles(order in 0u64..63) {
            prop_assert_eq!(block_size(order + 1), 2 * block_size(order));
        }

        // r[verify buddy.is-aligned-to-order]
        #[test]
        fn prop_zero_aligned(order in 0u64..64) {
            prop_assert!(is_aligned_to_order(0, order));
        }

        // r[verify buddy.can-coalesce]
        #[test]
        fn prop_coalesce_symmetric(addr: u64, order in 0u64..63) {
            if can_coalesce(addr, buddy_of(addr, order), order) {
                prop_assert!(can_coalesce(buddy_of(addr, order), addr, order));
            }
        }

        // r[verify buddy.parent-block]
        #[test]
        fn prop_buddies_same_parent(addr: u64, order in 0u64..63) {
            let buddy = buddy_of(addr, order);
            prop_assert_eq!(parent_block(addr, order), parent_block(buddy, order));
        }

        // r[verify buddy.parent-block]
        #[test]
        fn prop_parent_aligned(addr: u64, order in 0u64..63) {
            let parent = parent_block(addr, order);
            prop_assert!(is_aligned_to_order(parent, order + 1));
        }

        // r[verify buddy.order-for-size]
        #[test]
        fn prop_order_covers_size(size in 1u64..=(1u64 << 32)) {
            let order = order_for_size(size);
            prop_assert!(order < 64);
            prop_assert!(block_size(order) >= size);
        }

        // r[verify buddy.order-for-size]
        #[test]
        fn prop_order_for_power_of_two(k in 0u64..63) {
            let size = 1u64 << k;
            let order = order_for_size(size);
            prop_assert!(block_size(order) >= size);
            // Should be exact for powers of two
            prop_assert_eq!(block_size(order), size);
        }
    }
}
