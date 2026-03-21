//! Verified bounds checking and clamping.

// r[impl bounds.in-range]
/// Returns `true` if `value` is within the inclusive range `[lo, hi]`.
///
/// If `lo > hi` the range is empty and the result is always `false`.
pub fn in_range(value: u64, lo: u64, hi: u64) -> bool {
    lo <= value && value <= hi
}

// r[impl bounds.clamp]
/// Clamps `value` to the inclusive range `[lo, hi]`.
///
/// Panics if `lo > hi`.
pub fn clamp(value: u64, lo: u64, hi: u64) -> u64 {
    assert!(lo <= hi, "clamp: lo must be <= hi");
    if value < lo {
        lo
    } else if value > hi {
        hi
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // r[verify bounds.in-range]
    #[test]
    fn test_in_range() {
        assert!(in_range(5, 0, 10));
        assert!(in_range(0, 0, 10));
        assert!(in_range(10, 0, 10));
        assert!(!in_range(11, 0, 10));
        assert!(in_range(5, 5, 5));
        assert!(!in_range(0, 5, 3)); // empty range
    }

    // r[verify bounds.clamp]
    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(0, 3, 10), 3);
        assert_eq!(clamp(15, 3, 10), 10);
        assert_eq!(clamp(3, 3, 3), 3);
        // clamp result is always in range
        for &v in &[0, 5, 10, 15, u64::MAX] {
            let c = clamp(v, 3, 10);
            assert!(in_range(c, 3, 10));
        }
    }

    #[test]
    #[should_panic(expected = "lo must be <= hi")]
    fn test_clamp_invalid_range() {
        clamp(5, 10, 3);
    }
}
