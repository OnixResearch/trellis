//! Verified bitwise operations.
//!
//! Bit manipulation with machine-checked correctness. Proofs use Verus's
//! `by(bit_vector)` mode which switches Z3 to bit-vector theory for
//! precise reasoning about AND, OR, XOR, and shifts.

use vstd::prelude::*;

verus! {

// --- is_power_of_two ---

// r[depends bitwise.is-power-of-two]
/// Spec: x is a power of two (exactly one bit set).
pub open spec fn is_power_of_two_spec(x: u64) -> bool {
    x != 0 && (x & sub(x, 1)) == 0
}

// r[impl bitwise.is-power-of-two]
/// Returns true if `x` is a power of two.
///
/// Uses the classic bit trick: a power of two has exactly one bit set,
/// so `x & (x - 1)` clears that bit, giving zero.
pub fn is_power_of_two(x: u64) -> (result: bool)
    ensures result == is_power_of_two_spec(x)
{
    x != 0 && (x & (x - 1)) == 0
}

// r[verify bitwise.is-power-of-two]
proof fn power_of_two_examples()
    ensures
        is_power_of_two_spec(1),
        is_power_of_two_spec(2),
        is_power_of_two_spec(4),
        !is_power_of_two_spec(0),
        !is_power_of_two_spec(3),
{
    assert(1u64 & sub(1u64, 1) == 0u64) by(bit_vector);
    assert(2u64 & sub(2u64, 1) == 0u64) by(bit_vector);
    assert(4u64 & sub(4u64, 1) == 0u64) by(bit_vector);
    assert(3u64 & sub(3u64, 1) != 0u64) by(bit_vector);
}

// --- bit_at ---

// r[depends bitwise.bit-at]
/// Spec: the bit at position `pos` is set.
pub open spec fn bit_at_spec(x: u64, pos: u64) -> bool
    recommends pos < 64
{
    (x >> pos) & 1u64 == 1u64
}

// r[impl bitwise.bit-at]
/// Returns true if the bit at `pos` is set in `x`.
pub fn bit_at(x: u64, pos: u64) -> (result: bool)
    requires pos < 64
    ensures result == bit_at_spec(x, pos)
{
    (x >> pos) & 1 == 1
}

// --- set_bit ---

// r[depends bitwise.set-bit]
/// Spec: set the bit at position `pos`.
pub open spec fn set_bit_spec(x: u64, pos: u64) -> u64
    recommends pos < 64
{
    x | (1u64 << pos)
}

// r[impl bitwise.set-bit]
/// Returns `x` with the bit at `pos` set to 1.
pub fn set_bit(x: u64, pos: u64) -> (result: u64)
    requires pos < 64
    ensures result == set_bit_spec(x, pos)
{
    x | (1u64 << pos)
}

// r[verify bitwise.set-bit]
/// Proof: setting a bit makes it read as 1.
proof fn set_then_check(x: u64, pos: u64)
    requires pos < 64
    ensures bit_at_spec(set_bit_spec(x, pos), pos)
{
    assert(pos < 64 ==> ((x | (1u64 << pos)) >> pos) & 1u64 == 1u64) by(bit_vector);
}

// r[verify bitwise.set-bit]
/// Proof: setting a bit twice is the same as setting it once.
proof fn set_bit_idempotent(x: u64, pos: u64)
    requires pos < 64
    ensures set_bit_spec(set_bit_spec(x, pos), pos) == set_bit_spec(x, pos)
{
    assert(pos < 64 ==> (x | (1u64 << pos)) | (1u64 << pos) == x | (1u64 << pos)) by(bit_vector);
}

// --- clear_bit ---

// r[depends bitwise.clear-bit]
/// Spec: clear the bit at position `pos`.
pub open spec fn clear_bit_spec(x: u64, pos: u64) -> u64
    recommends pos < 64
{
    x & !(1u64 << pos)
}

// r[impl bitwise.clear-bit]
/// Returns `x` with the bit at `pos` set to 0.
pub fn clear_bit(x: u64, pos: u64) -> (result: u64)
    requires pos < 64
    ensures result == clear_bit_spec(x, pos)
{
    x & !(1u64 << pos)
}

// r[verify bitwise.clear-bit]
/// Proof: clearing a bit makes it read as 0.
proof fn clear_then_check(x: u64, pos: u64)
    requires pos < 64
    ensures !bit_at_spec(clear_bit_spec(x, pos), pos)
{
    assert(pos < 64 ==> ((x & !(1u64 << pos)) >> pos) & 1u64 == 0u64) by(bit_vector);
}

// r[verify bitwise.clear-bit]
/// Proof: clearing a bit twice is the same as clearing it once.
proof fn clear_bit_idempotent(x: u64, pos: u64)
    requires pos < 64
    ensures clear_bit_spec(clear_bit_spec(x, pos), pos) == clear_bit_spec(x, pos)
{
    assert(pos < 64 ==> (x & !(1u64 << pos)) & !(1u64 << pos) == x & !(1u64 << pos)) by(bit_vector);
}

// --- toggle_bit ---

// r[depends bitwise.toggle-bit]
/// Spec: toggle (flip) the bit at position `pos`.
pub open spec fn toggle_bit_spec(x: u64, pos: u64) -> u64
    recommends pos < 64
{
    x ^ (1u64 << pos)
}

// r[impl bitwise.toggle-bit]
/// Returns `x` with the bit at `pos` flipped.
pub fn toggle_bit(x: u64, pos: u64) -> (result: u64)
    requires pos < 64
    ensures result == toggle_bit_spec(x, pos)
{
    x ^ (1u64 << pos)
}

// r[verify bitwise.toggle-bit]
/// Proof: toggling twice returns the original value.
proof fn toggle_bit_involution(x: u64, pos: u64)
    requires pos < 64
    ensures toggle_bit_spec(toggle_bit_spec(x, pos), pos) == x
{
    assert(pos < 64 ==> (x ^ (1u64 << pos)) ^ (1u64 << pos) == x) by(bit_vector);
}

} // verus!

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // r[verify bitwise.is-power-of-two]
    #[test]
    fn test_is_power_of_two() {
        assert!(is_power_of_two(1));
        assert!(is_power_of_two(2));
        assert!(is_power_of_two(4));
        assert!(is_power_of_two(1 << 63));
        assert!(!is_power_of_two(0));
        assert!(!is_power_of_two(3));
        assert!(!is_power_of_two(6));
        assert!(!is_power_of_two(u64::MAX));
    }

    // r[verify bitwise.bit-at]
    #[test]
    fn test_bit_at() {
        assert!(bit_at(0b1010, 1));
        assert!(bit_at(0b1010, 3));
        assert!(!bit_at(0b1010, 0));
        assert!(!bit_at(0b1010, 2));
        assert!(bit_at(1 << 63, 63));
    }

    // r[verify bitwise.set-bit]
    #[test]
    fn test_set_bit() {
        assert_eq!(set_bit(0, 0), 1);
        assert_eq!(set_bit(0, 3), 8);
        assert_eq!(set_bit(0b1010, 0), 0b1011);
        assert_eq!(set_bit(0b1010, 1), 0b1010); // already set
    }

    // r[verify bitwise.clear-bit]
    #[test]
    fn test_clear_bit() {
        assert_eq!(clear_bit(0b1010, 1), 0b1000);
        assert_eq!(clear_bit(0b1010, 0), 0b1010); // already clear
        assert_eq!(clear_bit(u64::MAX, 63), u64::MAX >> 1_u32);
    }

    // r[verify bitwise.toggle-bit]
    #[test]
    fn test_toggle_bit() {
        assert_eq!(toggle_bit(0, 0), 1);
        assert_eq!(toggle_bit(1, 0), 0);
        assert_eq!(toggle_bit(0b1010, 0), 0b1011);
        assert_eq!(toggle_bit(0b1010, 1), 0b1000);
    }

    proptest! {
        // r[verify bitwise.is-power-of-two]
        #[test]
        fn prop_power_of_two_matches_stdlib(x: u64) {
            prop_assert_eq!(is_power_of_two(x), x.is_power_of_two());
        }

        // r[verify bitwise.set-bit]
        #[test]
        fn prop_set_then_read(x: u64, pos in 0u64..64) {
            prop_assert!(bit_at(set_bit(x, pos), pos));
        }

        // r[verify bitwise.clear-bit]
        #[test]
        fn prop_clear_then_read(x: u64, pos in 0u64..64) {
            prop_assert!(!bit_at(clear_bit(x, pos), pos));
        }

        // r[verify bitwise.set-bit]
        #[test]
        fn prop_set_idempotent(x: u64, pos in 0u64..64) {
            prop_assert_eq!(set_bit(set_bit(x, pos), pos), set_bit(x, pos));
        }

        // r[verify bitwise.clear-bit]
        #[test]
        fn prop_clear_idempotent(x: u64, pos in 0u64..64) {
            prop_assert_eq!(clear_bit(clear_bit(x, pos), pos), clear_bit(x, pos));
        }

        // r[verify bitwise.toggle-bit]
        #[test]
        fn prop_toggle_involution(x: u64, pos in 0u64..64) {
            prop_assert_eq!(toggle_bit(toggle_bit(x, pos), pos), x);
        }
    }
}
