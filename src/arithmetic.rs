//! Verified arithmetic operations.

// r[impl arithmetic.abs-diff]
/// Returns the absolute difference between two `u64` values.
///
/// Guaranteed to never overflow — the result is always representable
/// as `u64` since both inputs are unsigned.
pub fn abs_diff(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { b - a }
}

// r[impl arithmetic.saturating-add]
/// Saturating addition: returns `a + b` clamped to `u64::MAX`.
pub fn saturating_add(a: u64, b: u64) -> u64 {
    a.saturating_add(b)
}

// r[impl arithmetic.saturating-sub]
/// Saturating subtraction: returns `a - b` clamped to `0`.
pub fn saturating_sub(a: u64, b: u64) -> u64 {
    a.saturating_sub(b)
}

// r[impl arithmetic.checked-mul]
/// Checked multiplication: returns `None` on overflow.
pub fn checked_mul(a: u64, b: u64) -> Option<u64> {
    a.checked_mul(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    // r[verify arithmetic.abs-diff]
    #[test]
    fn test_abs_diff() {
        assert_eq!(abs_diff(10, 3), 7);
        assert_eq!(abs_diff(3, 10), 7);
        assert_eq!(abs_diff(5, 5), 0);
        assert_eq!(abs_diff(0, u64::MAX), u64::MAX);
        assert_eq!(abs_diff(u64::MAX, 0), u64::MAX);
    }

    // r[verify arithmetic.saturating-add]
    #[test]
    fn test_saturating_add() {
        assert_eq!(saturating_add(1, 2), 3);
        assert_eq!(saturating_add(u64::MAX, 1), u64::MAX);
        assert_eq!(saturating_add(u64::MAX, u64::MAX), u64::MAX);
        assert_eq!(saturating_add(0, 0), 0);
    }

    // r[verify arithmetic.saturating-sub]
    #[test]
    fn test_saturating_sub() {
        assert_eq!(saturating_sub(10, 3), 7);
        assert_eq!(saturating_sub(3, 10), 0);
        assert_eq!(saturating_sub(0, 0), 0);
        assert_eq!(saturating_sub(0, u64::MAX), 0);
    }

    // r[verify arithmetic.checked-mul]
    #[test]
    fn test_checked_mul() {
        assert_eq!(checked_mul(3, 4), Some(12));
        assert_eq!(checked_mul(u64::MAX, 2), None);
        assert_eq!(checked_mul(0, u64::MAX), Some(0));
        assert_eq!(checked_mul(1, u64::MAX), Some(u64::MAX));
    }
}
