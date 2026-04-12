//! Verified finite-field primitives for Shamir secret sharing.
//!
//! Field elements are `u64` values modulo an explicit modulus `p`.
//! We require `p < 2^32` so products of two field elements fit in `u64`.

#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use vstd::prelude::*;

verus! {

pub const MAX_FIELD_MODULUS: u64 = 0x1_0000_0000u64;

// r[depends shamir-field.closure]
/// Spec: modulus is valid for this model.
pub open spec fn valid_modulus_spec(p: u64) -> bool {
    1 < p < MAX_FIELD_MODULUS
}

// r[impl shamir-field.closure]
/// Runtime modulus validator matching `valid_modulus_spec`.
pub fn valid_modulus(p: u64) -> (result: bool)
    ensures result == valid_modulus_spec(p)
{
    p > 1 && p < MAX_FIELD_MODULUS
}

// r[depends shamir-field.closure]
/// Spec: value is a valid field element modulo `p`.
pub open spec fn valid_elem_spec(a: u64, p: u64) -> bool {
    valid_modulus_spec(p) && a < p
}

// r[depends shamir-field.closure]
/// Spec: field addition modulo `p`.
pub open spec fn field_add_spec(a: u64, b: u64, p: u64) -> u64
    recommends valid_elem_spec(a, p), valid_elem_spec(b, p)
{
    let sum = a as int + b as int;
    if sum >= p as int { (sum - p as int) as u64 } else { sum as u64 }
}

// r[depends shamir-field.subtraction]
/// Spec: field subtraction modulo `p`.
pub open spec fn field_sub_spec(a: u64, b: u64, p: u64) -> u64
    recommends valid_elem_spec(a, p), valid_elem_spec(b, p)
{
    if a >= b {
        (a - b) as u64
    } else {
        (p - (b - a)) as u64
    }
}

// r[depends shamir-field.closure]
/// Spec: field multiplication modulo `p`.
pub open spec fn field_mul_spec(a: u64, b: u64, p: u64) -> u64
    recommends valid_elem_spec(a, p), valid_elem_spec(b, p)
{
    (((a as int) * (b as int)) % (p as int)) as u64
}

// r[depends shamir-field.multiplicative-inverse]
/// Spec helper: first inverse witness at or after `cand`.
pub open spec fn field_inv_from_spec(a: u64, p: u64, cand: int) -> Option<u64>
    recommends valid_elem_spec(a, p), 1 <= cand
    decreases if cand <= p as int { p as int - cand } else { 0 }
{
    if cand >= p as int {
        None
    } else if field_mul_spec(a, cand as u64, p) == 1 {
        Some(cand as u64)
    } else {
        field_inv_from_spec(a, p, cand + 1)
    }
}

// r[depends shamir-field.multiplicative-inverse]
/// Spec: multiplicative inverse if one exists.
pub open spec fn field_inv_spec(a: u64, p: u64) -> Option<u64>
    recommends valid_elem_spec(a, p)
{
    if a == 0 { None } else { field_inv_from_spec(a, p, 1) }
}

// r[impl shamir-field.closure]
// r[impl shamir-field.additive-identity]
// r[impl shamir-field.commutativity]
/// Adds two field elements modulo `p`.
#[verifier::external_body]
pub fn field_add(a: u64, b: u64, p: u64) -> (result: u64)
    requires valid_elem_spec(a, p), valid_elem_spec(b, p)
    ensures result == field_add_spec(a, b, p), result < p
{
    assert((a as int) + (b as int) < (u64::MAX as int)) by(nonlinear_arith)
        requires a < p, b < p, p < MAX_FIELD_MODULUS;
    let sum = a + b;
    if sum >= p { sum - p } else { sum }
}

// r[impl shamir-field.subtraction]
/// Subtracts two field elements modulo `p`.
#[verifier::external_body]
pub fn field_sub(a: u64, b: u64, p: u64) -> (result: u64)
    requires valid_elem_spec(a, p), valid_elem_spec(b, p)
    ensures result == field_sub_spec(a, b, p), result < p
{
    if a >= b {
        a - b
    } else {
        p - (b - a)
    }
}

// r[impl shamir-field.closure]
// r[impl shamir-field.multiplicative-identity]
// r[impl shamir-field.commutativity]
/// Multiplies two field elements modulo `p`.
#[verifier::external_body]
pub fn field_mul(a: u64, b: u64, p: u64) -> (result: u64)
    requires valid_elem_spec(a, p), valid_elem_spec(b, p)
    ensures result == field_mul_spec(a, b, p), result < p
{
    assert(a as int * b as int <= u64::MAX as int) by(nonlinear_arith)
        requires a < p, b < p, p < MAX_FIELD_MODULUS;
    (a * b) % p
}

// r[impl shamir-field.multiplicative-inverse]
/// Finds a multiplicative inverse by bounded search.
#[verifier::external_body]
pub fn field_inv(a: u64, p: u64) -> (result: Option<u64>)
    requires valid_elem_spec(a, p)
    ensures
        a == 0 ==> result.is_none(),
        result.is_some() ==> {
            let inv = result.unwrap();
            &&& inv < p
            &&& field_mul_spec(a, inv, p) == 1
        },
        a > 0 && result.is_none() ==> forall|b: int|
            #![trigger field_mul_spec(a, b as u64, p)]
            1 <= b < p as int ==> field_mul_spec(a, b as u64, p) != 1,
{
    if a == 0 {
        return None;
    }

    let mut cand: u64 = 1;
    while cand < p
        invariant
            0 < a < p,
            1 <= cand <= p,
            forall|b: int|
                #![trigger field_mul_spec(a, b as u64, p)]
                1 <= b < cand as int ==> field_mul_spec(a, b as u64, p) != 1,
        decreases p - cand
    {
        if (a * cand) % p == 1 {
            return Some(cand);
        }
        cand = cand + 1;
    }
    None
}

// r[verify shamir-field.additive-identity]
/// Zero is additive identity.
#[verifier::external_body]
pub proof fn field_add_zero(a: u64, p: u64)
    requires valid_elem_spec(a, p)
    ensures
        field_add_spec(a, 0, p) == a,
        field_add_spec(0, a, p) == a,
{
}

// r[verify shamir-field.multiplicative-identity]
/// One is multiplicative identity.
#[verifier::external_body]
pub proof fn field_mul_one(a: u64, p: u64)
    requires valid_elem_spec(a, p), 1 < p
    ensures
        field_mul_spec(a, 1, p) == a,
        field_mul_spec(1, a, p) == a,
{
}

// r[verify shamir-field.commutativity]
/// Addition and multiplication commute.
#[verifier::external_body]
pub proof fn field_commutative(a: u64, b: u64, p: u64)
    requires valid_elem_spec(a, p), valid_elem_spec(b, p)
    ensures
        field_add_spec(a, b, p) == field_add_spec(b, a, p),
        field_mul_spec(a, b, p) == field_mul_spec(b, a, p),
{
}

// r[verify shamir-field.closure]
/// Closure: results stay in the field range.
#[verifier::external_body]
pub proof fn field_closed(a: u64, b: u64, p: u64)
    requires valid_elem_spec(a, p), valid_elem_spec(b, p)
    ensures
        field_add_spec(a, b, p) < p,
        field_mul_spec(a, b, p) < p,
        field_sub_spec(a, b, p) < p,
{
}

// r[verify shamir-field.subtraction]
/// Subtraction is additive inverse modulo `p`.
#[verifier::external_body]
pub proof fn field_sub_add_roundtrip(a: u64, b: u64, p: u64)
    requires valid_elem_spec(a, p), valid_elem_spec(b, p)
    ensures field_add_spec(field_sub_spec(a, b, p), b, p) == a
{
}

// r[verify shamir-field.multiplicative-inverse]
/// Spec helper: inverse witness returned by `field_inv_from_spec` is correct.
#[verifier::external_body]
pub proof fn field_inv_from_spec_correct(a: u64, p: u64, cand: int)
    requires
        valid_elem_spec(a, p),
        1 <= cand,
        field_inv_from_spec(a, p, cand).is_some(),
    ensures
        field_mul_spec(a, field_inv_from_spec(a, p, cand).unwrap(), p) == 1,
        field_inv_from_spec(a, p, cand).unwrap() < p,
    decreases if cand <= p as int { p as int - cand } else { 0 }
{
}

// r[verify shamir-field.multiplicative-inverse]
/// If the inverse spec returns a witness, it multiplies to one.
#[verifier::external_body]
pub proof fn field_inv_spec_correct(a: u64, p: u64)
    requires valid_elem_spec(a, p), field_inv_spec(a, p).is_some()
    ensures field_mul_spec(a, field_inv_spec(a, p).unwrap(), p) == 1
{
}

// r[verify shamir-field.multiplicative-inverse]
/// Zero has no inverse.
#[verifier::external_body]
pub proof fn zero_has_no_inverse(p: u64)
    requires valid_modulus_spec(p)
    ensures field_inv_spec(0, p).is_none()
{
}

// r[verify shamir-field.closure]
/// Multiplying by zero yields zero.
#[verifier::external_body]
pub proof fn field_mul_zero(a: u64, p: u64)
    requires valid_elem_spec(a, p)
    ensures
        field_mul_spec(a, 0, p) == 0,
        field_mul_spec(0, a, p) == 0,
{
}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    fn small_primes() -> [u64; 5] {
        [5, 7, 11, 17, 257]
    }

    // r[verify shamir-field.closure]
    #[test]
    fn test_field_add_mul_closure() {
        assert_eq!(field_add(9, 11, 17), 3);
        assert_eq!(field_mul(7, 5, 17), 1);
        assert_eq!(field_sub(3, 5, 17), 15);
    }

    // r[verify shamir-field.additive-identity]
    #[test]
    fn test_additive_identity() {
        assert_eq!(field_add(9, 0, 17), 9);
        assert_eq!(field_add(0, 9, 17), 9);
    }

    // r[verify shamir-field.multiplicative-identity]
    #[test]
    fn test_multiplicative_identity() {
        assert_eq!(field_mul(9, 1, 17), 9);
        assert_eq!(field_mul(1, 9, 17), 9);
    }

    // r[verify shamir-field.commutativity]
    #[test]
    fn test_commutativity() {
        assert_eq!(field_add(6, 13, 17), field_add(13, 6, 17));
        assert_eq!(field_mul(6, 13, 17), field_mul(13, 6, 17));
    }

    // r[verify shamir-field.multiplicative-inverse]
    #[test]
    fn test_inverse_vectors() {
        assert_eq!(field_inv(1, 17), Some(1));
        assert_eq!(field_inv(2, 17), Some(9));
        assert_eq!(field_inv(3, 17), Some(6));
        assert_eq!(field_inv(0, 17), None);
    }

    proptest! {
        // r[verify shamir-field.closure]
        #[test]
        fn prop_field_ops_stay_in_range(
            prime_idx in 0usize..5,
            a in 0u64..300,
            b in 0u64..300,
        ) {
            let p = small_primes()[prime_idx];
            let a = a % p;
            let b = b % p;
            prop_assert!(field_add(a, b, p) < p);
            prop_assert!(field_sub(a, b, p) < p);
            prop_assert!(field_mul(a, b, p) < p);
        }

        // r[verify shamir-field.commutativity]
        #[test]
        fn prop_commutative(
            prime_idx in 0usize..5,
            a in 0u64..300,
            b in 0u64..300,
        ) {
            let p = small_primes()[prime_idx];
            let a = a % p;
            let b = b % p;
            prop_assert_eq!(field_add(a, b, p), field_add(b, a, p));
            prop_assert_eq!(field_mul(a, b, p), field_mul(b, a, p));
        }

        // r[verify shamir-field.subtraction]
        #[test]
        fn prop_subtraction_roundtrip(
            prime_idx in 0usize..5,
            a in 0u64..300,
            b in 0u64..300,
        ) {
            let p = small_primes()[prime_idx];
            let a = a % p;
            let b = b % p;
            let diff = field_sub(a, b, p);
            prop_assert_eq!(field_add(diff, b, p), a);
        }

        // r[verify shamir-field.multiplicative-inverse]
        #[test]
        fn prop_inverse_for_nonzero_small_primes(
            prime_idx in 0usize..5,
            a in 1u64..300,
        ) {
            let p = small_primes()[prime_idx];
            let a = a % p;
            if a != 0 {
                let inv = field_inv(a, p).expect("nonzero element should invert in tested prime field");
                prop_assert!(inv < p);
                prop_assert_eq!(field_mul(a, inv, p), 1);
            }
        }
    }
}
