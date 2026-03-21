//! Verified arithmetic operations.
//!
//! Each function has a `spec fn` (mathematical definition) and an `exec fn`
//! whose `ensures` clause ties it to the spec. Verus mechanically checks
//! that the implementation matches — drift is impossible.

use vstd::prelude::*;

verus! {

// --- abs_diff ---

// r[depends arithmetic.abs-diff]
/// Spec: absolute difference of two u64 values.
pub open spec fn abs_diff_spec(a: u64, b: u64) -> u64 {
    if a >= b { (a - b) as u64 } else { (b - a) as u64 }
}

// r[impl arithmetic.abs-diff]
/// Returns the absolute difference between two `u64` values.
pub fn abs_diff(a: u64, b: u64) -> (result: u64)
    ensures result == abs_diff_spec(a, b)
{
    if a >= b { a - b } else { b - a }
}

// r[verify arithmetic.abs-diff]
proof fn abs_diff_symmetric(a: u64, b: u64)
    ensures abs_diff_spec(a, b) == abs_diff_spec(b, a)
{}

// r[verify arithmetic.abs-diff]
proof fn abs_diff_self_is_zero(x: u64)
    ensures abs_diff_spec(x, x) == 0u64
{}

// --- saturating_add ---

// r[depends arithmetic.saturating-add]
/// Spec: saturating addition.
pub open spec fn saturating_add_spec(a: u64, b: u64) -> u64 {
    if a as int + b as int > u64::MAX as int {
        u64::MAX
    } else {
        (a + b) as u64
    }
}

// r[impl arithmetic.saturating-add]
/// Saturating addition: returns `a + b` clamped to `u64::MAX`.
pub fn saturating_add(a: u64, b: u64) -> (result: u64)
    ensures result == saturating_add_spec(a, b)
{
    match a.checked_add(b) {
        Some(sum) => sum,
        None => u64::MAX,
    }
}

// r[verify arithmetic.saturating-add]
proof fn saturating_add_bounded(a: u64, b: u64)
    ensures saturating_add_spec(a, b) <= u64::MAX
{}

// r[verify arithmetic.saturating-add]
proof fn saturating_add_commutative(a: u64, b: u64)
    ensures saturating_add_spec(a, b) == saturating_add_spec(b, a)
{}

// --- saturating_sub ---

// r[depends arithmetic.saturating-sub]
/// Spec: saturating subtraction.
pub open spec fn saturating_sub_spec(a: u64, b: u64) -> u64 {
    if b > a { 0 } else { (a - b) as u64 }
}

// r[impl arithmetic.saturating-sub]
/// Saturating subtraction: returns `a - b` clamped to `0`.
pub fn saturating_sub(a: u64, b: u64) -> (result: u64)
    ensures result == saturating_sub_spec(a, b)
{
    if b > a { 0 } else { a - b }
}

// r[verify arithmetic.saturating-sub]
proof fn saturating_sub_exact(a: u64, b: u64)
    requires a >= b
    ensures saturating_sub_spec(a, b) == (a - b) as u64
{}

// --- checked_mul ---

// r[depends arithmetic.checked-mul]
/// Spec: checked multiplication.
pub open spec fn checked_mul_spec(a: u64, b: u64) -> Option<u64> {
    if a as int * b as int > u64::MAX as int {
        None
    } else {
        Some((a * b) as u64)
    }
}

// r[impl arithmetic.checked-mul]
/// Checked multiplication: returns `None` on overflow.
pub fn checked_mul(a: u64, b: u64) -> (result: Option<u64>)
    ensures result == checked_mul_spec(a, b)
{
    a.checked_mul(b)
}

// r[verify arithmetic.checked-mul]
proof fn checked_mul_zero(a: u64)
    ensures
        checked_mul_spec(a, 0) == Some(0u64),
        checked_mul_spec(0, a) == Some(0u64),
{}

} // verus!

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // r[verify arithmetic.abs-diff]
    #[test]
    fn test_abs_diff() {
        assert_eq!(abs_diff(10, 3), 7);
        assert_eq!(abs_diff(3, 10), 7);
        assert_eq!(abs_diff(5, 5), 0);
        assert_eq!(abs_diff(0, u64::MAX), u64::MAX);
    }

    // r[verify arithmetic.saturating-add]
    #[test]
    fn test_saturating_add() {
        assert_eq!(saturating_add(1, 2), 3);
        assert_eq!(saturating_add(u64::MAX, 1), u64::MAX);
        assert_eq!(saturating_add(0, 0), 0);
    }

    // r[verify arithmetic.saturating-sub]
    #[test]
    fn test_saturating_sub() {
        assert_eq!(saturating_sub(10, 3), 7);
        assert_eq!(saturating_sub(3, 10), 0);
        assert_eq!(saturating_sub(0, u64::MAX), 0);
    }

    // r[verify arithmetic.checked-mul]
    #[test]
    fn test_checked_mul() {
        assert_eq!(checked_mul(3, 4), Some(12));
        assert_eq!(checked_mul(u64::MAX, 2), None);
        assert_eq!(checked_mul(0, u64::MAX), Some(0));
    }

    proptest! {
        // r[verify arithmetic.abs-diff]
        #[test]
        fn prop_abs_diff_symmetric(a: u64, b: u64) {
            prop_assert_eq!(abs_diff(a, b), abs_diff(b, a));
        }

        // r[verify arithmetic.saturating-add]
        #[test]
        fn prop_saturating_add_commutative(a: u64, b: u64) {
            prop_assert_eq!(saturating_add(a, b), saturating_add(b, a));
        }

        // r[verify arithmetic.saturating-sub]
        #[test]
        fn prop_saturating_sub_self_is_zero(a: u64) {
            prop_assert_eq!(saturating_sub(a, a), 0);
        }

        // r[verify arithmetic.checked-mul]
        #[test]
        fn prop_checked_mul_zero(a: u64) {
            prop_assert_eq!(checked_mul(a, 0), Some(0));
            prop_assert_eq!(checked_mul(0, a), Some(0));
        }
    }
}
