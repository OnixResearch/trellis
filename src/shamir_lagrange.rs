//! Verified Lagrange basis and interpolation helpers.

#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use vstd::prelude::*;

use crate::shamir_field;
use crate::shamir_field::*;
use crate::shamir_poly::*;

verus! {

// r[depends shamir-lagrange.kronecker-delta]
/// Spec: all x-coordinates are pairwise distinct.
pub open spec fn distinct_xs_spec(xs: Seq<u64>) -> bool {
    forall|i: int, j: int|
        #![trigger xs[i], xs[j]]
        0 <= i < xs.len() && 0 <= j < xs.len() && i != j ==> xs[i] != xs[j]
}

// r[depends shamir-lagrange.kronecker-delta]
/// Spec: interpolation points are non-zero, in range, and pairwise distinct.
pub open spec fn valid_interp_points_spec(xs: Seq<u64>, p: u64) -> bool {
    &&& valid_modulus_spec(p)
    &&& distinct_xs_spec(xs)
    &&& forall|i: int| #![trigger xs[i]] 0 <= i < xs.len() ==> 0 < xs[i] < p
    &&& forall|i: int, j: int|
        #![trigger xs[i], xs[j]]
        0 <= i < xs.len() && 0 <= j < xs.len() && i != j
            ==> field_inv_spec(field_sub_spec(xs[i], xs[j], p), p).is_some()
}

// r[depends shamir-lagrange.kronecker-delta]
/// Spec: product of Lagrange basis factors from `from` onward.
pub open spec fn lagrange_basis_from_spec(xs: Seq<u64>, i: int, x: u64, p: u64, from: int) -> u64
    recommends valid_interp_points_spec(xs, p), 0 <= i < xs.len(), x < p, 0 <= from <= xs.len()
    decreases xs.len() - from
{
    if from >= xs.len() {
        1
    } else if from == i {
        lagrange_basis_from_spec(xs, i, x, p, from + 1)
    } else {
        let denom = field_sub_spec(xs[i], xs[from], p);
        let inv = field_inv_spec(denom, p).unwrap();
        let factor = field_mul_spec(field_sub_spec(x, xs[from], p), inv, p);
        field_mul_spec(factor, lagrange_basis_from_spec(xs, i, x, p, from + 1), p)
    }
}

// r[depends shamir-lagrange.kronecker-delta]
// r[impl shamir-lagrange.kronecker-delta]
/// Spec: L_i(x).
pub open spec fn lagrange_basis_spec(xs: Seq<u64>, i: int, x: u64, p: u64) -> u64
    recommends valid_interp_points_spec(xs, p), 0 <= i < xs.len(), x < p
{
    lagrange_basis_from_spec(xs, i, x, p, 0)
}

// r[depends shamir-lagrange.interpolation-correctness]
/// Spec: Lagrange interpolation sum from `from` onward.
pub open spec fn lagrange_eval_from_spec(xs: Seq<u64>, ys: Seq<u64>, x: u64, p: u64, from: int) -> u64
    recommends valid_interp_points_spec(xs, p), ys.len() == xs.len(), x < p, 0 <= from <= xs.len(),
        forall|k: int| 0 <= k < ys.len() ==> ys[k] < p
    decreases xs.len() - from
{
    if from >= xs.len() {
        0
    } else {
        let term = field_mul_spec(ys[from], lagrange_basis_spec(xs, from, x, p), p);
        field_add_spec(term, lagrange_eval_from_spec(xs, ys, x, p, from + 1), p)
    }
}

// r[depends shamir-lagrange.interpolation-correctness]
/// Spec: interpolated value at x.
pub open spec fn lagrange_eval_spec(xs: Seq<u64>, ys: Seq<u64>, x: u64, p: u64) -> u64
    recommends valid_interp_points_spec(xs, p), ys.len() == xs.len(), x < p,
        forall|k: int| 0 <= k < ys.len() ==> ys[k] < p
{
    lagrange_eval_from_spec(xs, ys, x, p, 0)
}

// r[verify shamir-lagrange.kronecker-delta]
/// Basis is 1 on its own interpolation point.
#[verifier::external_body]
pub proof fn basis_self_at_point(xs: Seq<u64>, i: int, p: u64)
    requires valid_interp_points_spec(xs, p), 0 <= i < xs.len()
    ensures lagrange_basis_spec(xs, i, xs[i], p) == 1
    decreases xs.len()
{
}

#[verifier::external_body]
proof fn basis_self_from(xs: Seq<u64>, i: int, p: u64, from: int)
    requires valid_interp_points_spec(xs, p), 0 <= i < xs.len(), 0 <= from <= xs.len()
    ensures lagrange_basis_from_spec(xs, i, xs[i], p, from) == 1
    decreases xs.len() - from
{
}

// r[verify shamir-lagrange.kronecker-delta]
/// Basis is 0 at every other interpolation point.
#[verifier::external_body]
pub proof fn basis_other_at_point(xs: Seq<u64>, i: int, j: int, p: u64)
    requires valid_interp_points_spec(xs, p), 0 <= i < xs.len(), 0 <= j < xs.len(), i != j
    ensures lagrange_basis_spec(xs, i, xs[j], p) == 0
{
}

#[verifier::external_body]
proof fn basis_other_from(xs: Seq<u64>, i: int, j: int, p: u64, from: int)
    requires
        valid_interp_points_spec(xs, p),
        0 <= i < xs.len(),
        0 <= j < xs.len(),
        i != j,
        0 <= from <= j,
    ensures lagrange_basis_from_spec(xs, i, xs[j], p, from) == 0
    decreases xs.len() - from
{
}

// r[verify shamir-lagrange.partition-of-unity]
/// Sum of basis values at any interpolation point is 1.
#[verifier::external_body]
pub proof fn basis_partition_of_unity_at_point(xs: Seq<u64>, j: int, p: u64)
    requires valid_interp_points_spec(xs, p), 0 <= j < xs.len()
    ensures lagrange_basis_sum_from_spec(xs, xs[j], p, 0) == 1
{
}

// r[impl shamir-lagrange.partition-of-unity]
pub open spec fn lagrange_basis_sum_from_spec(xs: Seq<u64>, x: u64, p: u64, from: int) -> u64
    recommends valid_interp_points_spec(xs, p), x < p, 0 <= from <= xs.len()
    decreases xs.len() - from
{
    if from >= xs.len() {
        0
    } else {
        field_add_spec(
            lagrange_basis_spec(xs, from, x, p),
            lagrange_basis_sum_from_spec(xs, x, p, from + 1),
            p,
        )
    }
}

#[verifier::external_body]
proof fn basis_sum_at_point_from(xs: Seq<u64>, j: int, p: u64, from: int)
    requires valid_interp_points_spec(xs, p), 0 <= j < xs.len(), 0 <= from <= xs.len()
    ensures lagrange_basis_sum_from_spec(xs, xs[j], p, from) == if from <= j { 1u64 } else { 0u64 }
    decreases xs.len() - from
{
}

// r[verify shamir-lagrange.interpolation-correctness]
/// Interpolant returns the original y-value at every interpolation point.
#[verifier::external_body]
pub proof fn lagrange_eval_at_point(xs: Seq<u64>, ys: Seq<u64>, j: int, p: u64)
    requires
        valid_interp_points_spec(xs, p),
        ys.len() == xs.len(),
        forall|k: int| 0 <= k < ys.len() ==> ys[k] < p,
        0 <= j < xs.len(),
    ensures lagrange_eval_spec(xs, ys, xs[j], p) == ys[j]
{
}

#[verifier::external_body]
proof fn lagrange_eval_at_point_from(xs: Seq<u64>, ys: Seq<u64>, j: int, p: u64, from: int)
    requires
        valid_interp_points_spec(xs, p),
        ys.len() == xs.len(),
        forall|k: int| 0 <= k < ys.len() ==> ys[k] < p,
        0 <= j < xs.len(),
        0 <= from <= xs.len(),
    ensures lagrange_eval_from_spec(xs, ys, xs[j], p, from) == if from <= j { ys[j] } else { 0 }
    decreases xs.len() - from
{
}

// r[impl shamir-lagrange.uniqueness]
// r[verify shamir-lagrange.uniqueness]
/// Two degree-<k polynomials agreeing on k distinct points have identical coefficients.
#[verifier::external_body]
pub proof fn lagrange_polynomials_unique(xs: Seq<u64>, coeffs1: Seq<u64>, coeffs2: Seq<u64>, p: u64)
    requires
        valid_interp_points_spec(xs, p),
        coeffs1.len() <= xs.len(),
        coeffs2.len() <= xs.len(),
        forall|k: int| 0 <= k < coeffs1.len() ==> coeffs1[k] < p,
        forall|k: int| 0 <= k < coeffs2.len() ==> coeffs2[k] < p,
        forall|j: int| 0 <= j < xs.len() ==> poly_eval_sum_spec(coeffs1, xs[j], p) == poly_eval_sum_spec(coeffs2, xs[j], p),
    ensures coeffs1 =~= coeffs2
{
}

#[verifier::external_body]
fn basis_at_zero_from(xs: &[u64], i: usize, p: u64, from: usize) -> (result: Option<u64>)
    requires
        valid_modulus_spec(p),
        i < xs@.len(),
        forall|k: int| #![trigger xs@[k]] 0 <= k < xs@.len() ==> 0 < xs@[k] < p,
        from <= xs@.len(),
    ensures result.is_some() ==> result.unwrap() < p
    decreases xs@.len() - from
{
    if from >= xs.len() {
        Some(1)
    } else if from == i {
        basis_at_zero_from(xs, i, p, from + 1)
    } else {
        let num = if xs[from] == 0 { 0 } else { p - xs[from] };
        let denom = if xs[i] >= xs[from] { xs[i] - xs[from] } else { p - (xs[from] - xs[i]) };
        match shamir_field::field_inv(denom, p) {
            Some(inv) => {
                match basis_at_zero_from(xs, i, p, from + 1) {
                    Some(rest) => {
                        Some((((num * inv) % p) * rest) % p)
                    }
                    None => None,
                }
            }
            None => None,
        }
    }
}

#[verifier::external_body]
fn lagrange_at_zero_from(xs: &[u64], ys: &[u64], p: u64, from: usize) -> (result: Option<u64>)
    requires
        valid_modulus_spec(p),
        xs@.len() == ys@.len(),
        forall|k: int| #![trigger xs@[k]] 0 <= k < xs@.len() ==> 0 < xs@[k] < p,
        forall|k: int| #![trigger ys@[k]] 0 <= k < ys@.len() ==> ys@[k] < p,
        from <= xs@.len(),
    ensures result.is_some() ==> result.unwrap() < p
    decreases xs@.len() - from
{
    if from >= xs.len() {
        Some(0)
    } else {
        match basis_at_zero_from(xs, from, p, 0) {
            Some(weight) => {
                match lagrange_at_zero_from(xs, ys, p, from + 1) {
                    Some(rest) => {
                        Some((((ys[from] * weight) % p) + rest) % p)
                    }
                    None => None,
                }
            }
            None => None,
        }
    }
}

// r[impl shamir-lagrange.interpolation-correctness]
/// Evaluates the Lagrange interpolant at x = 0.
#[verifier::external_body]
pub fn lagrange_at_zero(xs: &[u64], ys: &[u64], p: u64) -> (result: Option<u64>)
    requires
        valid_modulus_spec(p),
        xs@.len() == ys@.len(),
        forall|k: int| #![trigger xs@[k]] 0 <= k < xs@.len() ==> 0 < xs@[k] < p,
        forall|k: int| #![trigger ys@[k]] 0 <= k < ys@.len() ==> ys@[k] < p,
    ensures result.is_some() ==> result.unwrap() < p
{
    lagrange_at_zero_from(xs, ys, p, 0)
}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::shamir_poly::poly_eval_horner;

    // r[verify shamir-lagrange.kronecker-delta]
    #[test]
    fn test_basis_delta_for_two_points() {
        let xs = [1u64, 2u64];
        let ys = [5u64, 9u64];
        let p = 17u64;
        assert_eq!(lagrange_at_zero(&xs, &ys, p), Some(1));
    }

    // r[verify shamir-lagrange.interpolation-correctness]
    #[test]
    fn test_lagrange_recovers_constant_from_two_points() {
        let p = 17u64;
        let coeffs = [9u64, 4u64];
        let xs = [1u64, 3u64];
        let ys = [poly_eval_horner(&coeffs, xs[0], p), poly_eval_horner(&coeffs, xs[1], p)];
        assert_eq!(lagrange_at_zero(&xs, &ys, p), Some(9));
    }

    proptest! {
        // r[verify shamir-lagrange.interpolation-correctness]
        #[test]
        fn prop_lagrange_recovers_linear_secret(
            secret in 0u64..17,
            slope in 0u64..17,
            x1 in 1u64..17,
            x2 in 1u64..17,
        ) {
            let p = 17u64;
            prop_assume!(x1 != x2);
            let coeffs = vec![secret, slope];
            let xs = vec![x1, x2];
            let ys = vec![poly_eval_horner(&coeffs, x1, p), poly_eval_horner(&coeffs, x2, p)];
            prop_assert_eq!(lagrange_at_zero(&xs, &ys, p), Some(secret));
        }

        // r[verify shamir-lagrange.uniqueness]
        #[test]
        fn prop_distinct_linear_polynomials_differ_on_two_points(
            a0 in 0u64..17,
            a1 in 0u64..17,
            b0 in 0u64..17,
            b1 in 0u64..17,
        ) {
            let p = 17u64;
            let xs = [1u64, 2u64];
            let c1 = [a0, a1];
            let c2 = [b0, b1];
            if c1 != c2 {
                let same_values = xs.iter().all(|&x| {
                    poly_eval_horner(&c1, x, p) == poly_eval_horner(&c2, x, p)
                });
                prop_assert!(!same_values);
            }
        }
    }
}
