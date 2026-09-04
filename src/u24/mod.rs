//! Verified unsigned 24-bit modular addition.
//!
//! Runtime values use `u64`, while admission and executable contracts keep
//! every public result inside the explicit `u24` domain.

#[cfg(verus_keep_ghost)]
use vstd::arithmetic::div_mod::lemma_add_mod_noop;
#[cfg(verus_keep_ghost)]
use vstd::arithmetic::div_mod::lemma_mod_pos_bound;
#[cfg(verus_keep_ghost)]
use vstd::arithmetic::div_mod::lemma_small_mod;
use vstd::prelude::*;

verus! {

pub const BITS: u32 = 24;
pub const MODULUS: u64 = 16_777_216;
pub const MAX: u64 = 16_777_215;
pub const MASK: u64 = MAX;
pub const ZERO: u64 = 0;
pub const WIDENED_SUM_MAX: u64 = 33_554_430;

// r[impl bounded-modular-tree-fold.u24-domain]
// r[depends bounded-modular-tree-fold.u24-domain]
/// Returns true when `value` is in the closed `u24` domain.
pub open spec fn valid_spec(value: u64) -> bool {
    value < MODULUS
}

// r[impl bounded-modular-tree-fold.u24-domain]
// r[depends bounded-modular-tree-fold.u24-domain]
/// Mathematical normalization into the `u24` domain.
pub open spec fn normalize_spec(value: u64) -> u64 {
    ((value as int) % (MODULUS as int)) as u64
}

// r[impl bounded-modular-tree-fold.u24-add]
// r[impl bounded-modular-tree-fold.algebra]
// r[depends bounded-modular-tree-fold.u24-add]
/// Mathematical wrapping addition over admitted `u24` values.
pub open spec fn add_spec(left: u64, right: u64) -> u64 {
    (((left as int) + (right as int)) % (MODULUS as int)) as u64
}

// r[impl bounded-modular-tree-fold.u24-domain]
/// Checks the declared `u24` domain at runtime.
pub fn is_valid(value: u64) -> (result: bool)
    ensures result == valid_spec(value)
{
    value < MODULUS
}

// r[impl bounded-modular-tree-fold.u24-domain]
/// Normalizes any `u64` into the declared `u24` domain.
pub fn normalize(value: u64) -> (result: u64)
    ensures
        result == normalize_spec(value),
        valid_spec(result),
{
    proof {
        lemma_mod_pos_bound(value as int, MODULUS as int);
    }
    value % MODULUS
}

// r[impl bounded-modular-tree-fold.u24-add]
/// Adds two admitted `u24` values without runtime overflow.
pub fn add(left: u64, right: u64) -> (result: u64)
    requires
        valid_spec(left),
        valid_spec(right),
    ensures
        result == add_spec(left, right),
        valid_spec(result),
{
    assert((left as int) + (right as int) <= WIDENED_SUM_MAX as int) by(nonlinear_arith)
        requires left < MODULUS, right < MODULUS;
    assert(WIDENED_SUM_MAX < u64::MAX);
    let sum = left + right;
    proof {
        lemma_mod_pos_bound(sum as int, MODULUS as int);
    }
    sum % MODULUS
}

// r[verify bounded-modular-tree-fold.u24-domain]
/// Normalization always closes over the `u24` domain.
pub proof fn u24_normalize_closed(value: u64)
    ensures valid_spec(normalize_spec(value))
{
    lemma_mod_pos_bound(value as int, MODULUS as int);
}

// r[verify bounded-modular-tree-fold.u24-domain]
/// Normalizing an admitted value leaves it unchanged.
pub proof fn u24_normalize_valid_identity(value: u64)
    requires valid_spec(value)
    ensures normalize_spec(value) == value
{
    lemma_small_mod(value as nat, MODULUS as nat);
}

// r[verify bounded-modular-tree-fold.u24-domain]
/// Normalization is idempotent.
pub proof fn u24_normalize_idempotent(value: u64)
    ensures normalize_spec(normalize_spec(value)) == normalize_spec(value)
{
    u24_normalize_closed(value);
    u24_normalize_valid_identity(normalize_spec(value));
}

// r[verify bounded-modular-tree-fold.u24-add]
/// Wrapping addition remains in the admitted domain.
pub proof fn u24_add_closed(left: u64, right: u64)
    ensures valid_spec(add_spec(left, right))
{
    lemma_mod_pos_bound((left as int) + (right as int), MODULUS as int);
}

// r[verify bounded-modular-tree-fold.algebra]
/// Zero is the two-sided identity for admitted values.
pub proof fn u24_add_zero_identity(value: u64)
    requires valid_spec(value)
    ensures
        add_spec(value, ZERO) == value,
        add_spec(ZERO, value) == value,
{
    lemma_small_mod(value as nat, MODULUS as nat);
}

// r[verify bounded-modular-tree-fold.algebra]
/// Wrapping addition is commutative.
pub proof fn u24_add_commutative(left: u64, right: u64)
    ensures add_spec(left, right) == add_spec(right, left)
{
    assert((left as int) + (right as int) == (right as int) + (left as int)) by(nonlinear_arith);
}

// r[verify bounded-modular-tree-fold.algebra]
/// Wrapping addition is associative for admitted values.
pub proof fn u24_add_associative(left: u64, middle: u64, right: u64)
    requires
        valid_spec(left),
        valid_spec(middle),
        valid_spec(right),
    ensures
        add_spec(add_spec(left, middle), right)
            == add_spec(left, add_spec(middle, right)),
{
    let modulus = MODULUS as int;
    let left_middle = ((left as int) + (middle as int)) % modulus;
    let middle_right = ((middle as int) + (right as int)) % modulus;
    lemma_mod_pos_bound((left as int) + (middle as int), modulus);
    lemma_mod_pos_bound((middle as int) + (right as int), modulus);
    assert(add_spec(left, middle) as int == left_middle);
    assert(add_spec(middle, right) as int == middle_right);
    lemma_add_mod_noop((left as int) + (middle as int), right as int, modulus);
    lemma_add_mod_noop(left as int, (middle as int) + (right as int), modulus);
    assert((left_middle + (right as int)) % modulus
        == ((left as int) + (middle as int) + (right as int)) % modulus);
    assert(((left as int) + middle_right) % modulus
        == ((left as int) + (middle as int) + (right as int)) % modulus);
}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    const ORDINARY_LEFT_OPERAND: u64 = 11;
    const ORDINARY_RIGHT_OPERAND: u64 = 31;
    const ORDINARY_SUM: u64 = 42;

    // r[verify bounded-modular-tree-fold.u24-domain]
    #[test]
    fn normalization_covers_zero_max_and_wide_input() {
        assert_eq!(normalize(ZERO), ZERO);
        assert_eq!(normalize(MAX), MAX);
        assert_eq!(normalize(MODULUS), ZERO);
        assert_eq!(normalize(u64::MAX), MAX);
    }

    // r[verify bounded-modular-tree-fold.u24-add]
    #[test]
    fn addition_covers_ordinary_and_wrapping_results() {
        assert_eq!(add(ORDINARY_LEFT_OPERAND, ORDINARY_RIGHT_OPERAND), ORDINARY_SUM,);
        assert_eq!(add(MAX, 1), ZERO);
        assert_eq!(add(MAX, MAX), MAX - 1);
    }

    // r[impl bounded-modular-tree-fold.fixtures]
    // r[verify bounded-modular-tree-fold.fixtures]
    #[test]
    fn domain_rejects_first_out_of_range_value() {
        assert!(is_valid(MAX));
        assert!(!is_valid(MODULUS));
    }

    proptest! {
        // r[verify bounded-modular-tree-fold.algebra]
        #[test]
        fn wrapping_addition_obeys_concrete_algebra(
            left in 0u64..MODULUS,
            middle in 0u64..MODULUS,
            right in 0u64..MODULUS,
        ) {
            prop_assert_eq!(add(left, ZERO), left);
            prop_assert_eq!(add(left, right), add(right, left));
            prop_assert_eq!(
                add(add(left, middle), right),
                add(left, add(middle, right)),
            );
        }

        // r[verify bounded-modular-tree-fold.u24-domain]
        #[test]
        fn normalization_is_closed_and_idempotent(value in any::<u64>()) {
            let once = normalize(value);
            prop_assert!(once <= MAX);
            prop_assert_eq!(normalize(once), once);
        }
    }
}
