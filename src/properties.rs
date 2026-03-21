//! Cross-module verified properties.
//!
//! Proofs that relate functions from different modules — triangle inequality,
//! round-trip inverses, monotonicity, nearest-point optimality. These are
//! the properties that make the library useful as a verified foundation:
//! downstream code can rely on these relationships holding for all inputs.
//!
//! The spec/proof fns are ghost code (erased in normal builds). Only the
//! tests below are compiled by cargo.

// Spec and proof fns reference spec fns from other modules. These only
// exist during Verus verification (verus_keep_ghost), not in normal cargo
// builds. The verus! macro handles erasure of the proof bodies, but the
// use-statements for ghost items need cfg-gating.

#[cfg(verus_keep_ghost)]
mod verus_proofs {
    use vstd::prelude::*;

    use crate::arithmetic::abs_diff_spec;
    use crate::arithmetic::saturating_add_spec;
    use crate::arithmetic::saturating_sub_spec;
    use crate::bounds::clamp_spec;
    use crate::bounds::in_range_spec;

    verus! {

    // --- abs_diff is a metric ---

    // r[impl properties.abs-diff-zero-iff-equal]
    pub open spec fn abs_diff_zero_iff_equal_prop(a: u64, b: u64) -> bool {
        abs_diff_spec(a, b) == 0 <==> a == b
    }

    // r[verify properties.abs-diff-zero-iff-equal]
    proof fn prove_abs_diff_zero_iff_equal(a: u64, b: u64)
        ensures abs_diff_zero_iff_equal_prop(a, b)
    {}

    // r[impl properties.triangle-inequality]
    pub open spec fn triangle_inequality_prop(a: u64, b: u64, c: u64) -> bool {
        abs_diff_spec(a, c) as int <= abs_diff_spec(a, b) as int + abs_diff_spec(b, c) as int
    }

    // r[verify properties.triangle-inequality]
    proof fn prove_triangle_inequality(a: u64, b: u64, c: u64)
        ensures triangle_inequality_prop(a, b, c)
    {}

    // --- arithmetic inverses ---

    // r[impl properties.add-sub-roundtrip]
    pub open spec fn add_sub_roundtrip_prop(a: u64, b: u64) -> bool
        recommends a as int + b as int <= u64::MAX as int
    {
        saturating_sub_spec(saturating_add_spec(a, b), b) == a
    }

    // r[verify properties.add-sub-roundtrip]
    proof fn prove_add_sub_roundtrip(a: u64, b: u64)
        requires a as int + b as int <= u64::MAX as int
        ensures add_sub_roundtrip_prop(a, b)
    {}

    // r[impl properties.sub-add-inverse]
    pub open spec fn sub_add_inverse_prop(a: u64, b: u64) -> bool
        recommends b <= a
    {
        saturating_add_spec(saturating_sub_spec(a, b), b) == a
    }

    // r[verify properties.sub-add-inverse]
    proof fn prove_sub_add_inverse(a: u64, b: u64)
        requires b <= a
        ensures sub_add_inverse_prop(a, b)
    {}

    // --- clamp properties ---

    // r[impl properties.clamp-nearest-point]
    pub open spec fn clamp_nearest_point_prop(value: u64, x: u64, lo: u64, hi: u64) -> bool
        recommends lo <= hi, in_range_spec(x, lo, hi)
    {
        abs_diff_spec(value, clamp_spec(value, lo, hi)) <= abs_diff_spec(value, x)
    }

    // r[verify properties.clamp-nearest-point]
    proof fn prove_clamp_nearest_point(value: u64, x: u64, lo: u64, hi: u64)
        requires lo <= hi, in_range_spec(x, lo, hi)
        ensures clamp_nearest_point_prop(value, x, lo, hi)
    {}

    // r[impl properties.clamp-monotonic]
    pub open spec fn clamp_monotonic_prop(a: u64, b: u64, lo: u64, hi: u64) -> bool
        recommends lo <= hi, a <= b
    {
        clamp_spec(a, lo, hi) <= clamp_spec(b, lo, hi)
    }

    // r[verify properties.clamp-monotonic]
    proof fn prove_clamp_monotonic(a: u64, b: u64, lo: u64, hi: u64)
        requires lo <= hi, a <= b
        ensures clamp_monotonic_prop(a, b, lo, hi)
    {}

    // r[impl properties.distance-decomposition]
    pub open spec fn distance_decomposition_prop(value: u64, lo: u64, hi: u64) -> bool
        recommends lo <= hi, in_range_spec(value, lo, hi)
    {
        abs_diff_spec(value, lo) as int + abs_diff_spec(value, hi) as int
            == abs_diff_spec(lo, hi) as int
    }

    // r[verify properties.distance-decomposition]
    proof fn prove_distance_decomposition(value: u64, lo: u64, hi: u64)
        requires lo <= hi, in_range_spec(value, lo, hi)
        ensures distance_decomposition_prop(value, lo, hi)
    {}

    // --- bitwise cross-module properties ---

    use crate::ordering::{min_spec, max_spec};

    // --- ordering × arithmetic ---

    // r[impl properties.min-max-abs-diff]
    /// max(a,b) - min(a,b) == abs_diff(a,b).
    pub open spec fn min_max_abs_diff_prop(a: u64, b: u64) -> bool {
        (max_spec(a, b) - min_spec(a, b)) as u64 == abs_diff_spec(a, b)
    }

    // r[verify properties.min-max-abs-diff]
    proof fn prove_min_max_abs_diff(a: u64, b: u64)
        ensures min_max_abs_diff_prop(a, b)
    {}

    // --- ordering × bounds ---

    // r[impl properties.clamp-via-min-max]
    /// clamp(v, lo, hi) == min(max(v, lo), hi).
    pub open spec fn clamp_via_min_max_prop(v: u64, lo: u64, hi: u64) -> bool
        recommends lo <= hi
    {
        clamp_spec(v, lo, hi) == min_spec(max_spec(v, lo), hi)
    }

    // r[verify properties.clamp-via-min-max]
    proof fn prove_clamp_via_min_max(v: u64, lo: u64, hi: u64)
        requires lo <= hi
        ensures clamp_via_min_max_prop(v, lo, hi)
    {}

    // --- bitwise cross-module properties ---

    use crate::bitwise::{
        bit_at_spec, set_bit_spec, clear_bit_spec, toggle_bit_spec,
    };

    // r[impl properties.set-clear-roundtrip]
    /// Setting then clearing the same bit leaves it cleared.
    pub open spec fn set_clear_roundtrip_prop(x: u64, pos: u64) -> bool
        recommends pos < 64
    {
        clear_bit_spec(set_bit_spec(x, pos), pos) == clear_bit_spec(x, pos)
    }

    // r[verify properties.set-clear-roundtrip]
    proof fn prove_set_clear_roundtrip(x: u64, pos: u64)
        requires pos < 64
        ensures set_clear_roundtrip_prop(x, pos)
    {
        assert(pos < 64 ==> (x | (1u64 << pos)) & !(1u64 << pos) == x & !(1u64 << pos)) by(bit_vector);
    }

    // r[impl properties.toggle-involution]
    /// Toggling a bit twice returns the original value.
    pub open spec fn toggle_involution_prop(x: u64, pos: u64) -> bool
        recommends pos < 64
    {
        toggle_bit_spec(toggle_bit_spec(x, pos), pos) == x
    }

    // r[verify properties.toggle-involution]
    proof fn prove_toggle_involution(x: u64, pos: u64)
        requires pos < 64
        ensures toggle_involution_prop(x, pos)
    {
        assert(pos < 64 ==> (x ^ (1u64 << pos)) ^ (1u64 << pos) == x) by(bit_vector);
    }

    } // verus!
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::arithmetic::*;
    use crate::bitwise::*;
    use crate::bounds::*;
    use crate::ordering::*;

    proptest! {
        // r[verify properties.abs-diff-zero-iff-equal]
        #[test]
        fn prop_abs_diff_zero_iff_equal(a: u64, b: u64) {
            if a == b {
                prop_assert_eq!(abs_diff(a, b), 0);
            } else {
                prop_assert_ne!(abs_diff(a, b), 0);
            }
        }

        // r[verify properties.triangle-inequality]
        #[test]
        fn prop_triangle_inequality(a: u64, b: u64, c: u64) {
            let ac = u128::from(abs_diff(a, c));
            let ab = u128::from(abs_diff(a, b));
            let bc = u128::from(abs_diff(b, c));
            prop_assert!(ac <= ab + bc);
        }

        // r[verify properties.add-sub-roundtrip]
        #[test]
        fn prop_add_sub_roundtrip(a: u64, b in 0..=u64::MAX / 2) {
            let a = a % (u64::MAX - b + 1);
            prop_assert_eq!(saturating_sub(saturating_add(a, b), b), a);
        }

        // r[verify properties.sub-add-inverse]
        #[test]
        fn prop_sub_add_inverse(a: u64, b: u64) {
            if b <= a {
                prop_assert_eq!(saturating_add(saturating_sub(a, b), b), a);
            }
        }

        // r[verify properties.clamp-nearest-point]
        #[test]
        fn prop_clamp_nearest_point(
            value: u64,
            lo in 0..=u64::MAX / 2,
            spread in 0..=u64::MAX / 2,
        ) {
            let hi = lo + spread;
            let clamped = clamp(value, lo, hi);
            let dist_clamped = abs_diff(value, clamped);
            prop_assert!(dist_clamped <= abs_diff(value, lo));
            prop_assert!(dist_clamped <= abs_diff(value, hi));
        }

        // r[verify properties.clamp-monotonic]
        #[test]
        fn prop_clamp_monotonic(
            a: u64,
            b: u64,
            lo in 0..=u64::MAX / 2,
            spread in 0..=u64::MAX / 2,
        ) {
            let hi = lo + spread;
            if a <= b {
                prop_assert!(clamp(a, lo, hi) <= clamp(b, lo, hi));
            }
        }

        // r[verify properties.distance-decomposition]
        #[test]
        fn prop_distance_decomposition(
            lo in 0..=u64::MAX / 2,
            spread in 0..=u64::MAX / 2,
        ) {
            let hi = lo + spread;
            let value = lo + spread / 2;
            let d_lo = u128::from(abs_diff(value, lo));
            let d_hi = u128::from(abs_diff(value, hi));
            let range = u128::from(abs_diff(lo, hi));
            prop_assert_eq!(d_lo + d_hi, range);
        }

        // r[verify properties.min-max-abs-diff]
        #[test]
        fn prop_min_max_abs_diff(a: u64, b: u64) {
            prop_assert_eq!(max(a, b) - min(a, b), abs_diff(a, b));
        }

        // r[verify properties.clamp-via-min-max]
        #[test]
        fn prop_clamp_via_min_max(
            v: u64,
            lo in 0..=u64::MAX / 2,
            spread in 0..=u64::MAX / 2,
        ) {
            let hi = lo + spread;
            prop_assert_eq!(clamp(v, lo, hi), min(max(v, lo), hi));
        }

        // r[verify properties.set-clear-roundtrip]
        #[test]
        fn prop_set_clear_roundtrip(x: u64, pos in 0u64..64) {
            prop_assert_eq!(clear_bit(set_bit(x, pos), pos), clear_bit(x, pos));
        }

        // r[verify properties.toggle-involution]
        #[test]
        fn prop_toggle_involution(x: u64, pos in 0u64..64) {
            prop_assert_eq!(toggle_bit(toggle_bit(x, pos), pos), x);
        }
    }
}
