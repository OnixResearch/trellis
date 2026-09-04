use proptest::prelude::*;

use super::*;
use crate::u24::MAX;
use crate::u24::MODULUS;

const TWO_LEAVES: u64 = 2;
const THREE_LEAVES: u64 = 3;
const FOUR_LEAVES: u64 = 4;
const EIGHT_LEAF_COUNT: usize = 8;
const EIGHT_LEAVES: u64 = EIGHT_LEAF_COUNT as u64;
const EIGHT_COMBINES: u64 = 7;
const EIGHT_ROUNDS: u32 = 3;
const MAX_PROPERTY_EXPONENT: u32 = 7;
const MULTI_ROUND_LEAVES: [u64; EIGHT_LEAF_COUNT] = [MAX, 1, 2, 3, 4, 5, 6, 7];

fn independent_math_oracle(leaves: &[u64]) -> u64 {
    let modulus = u128::from(MODULUS);
    let sum = leaves.iter().fold(0u128, |accumulator, value| accumulator + u128::from(*value));
    u64::try_from(sum % modulus).expect("u24 modular sum must fit in u64")
}

// r[verify bounded-modular-tree-fold.admission]
// r[verify bounded-modular-tree-fold.fixtures]
#[test]
fn admission_accepts_exact_power_of_two_profile() {
    let leaves = [0, 1, MAX, 1];
    let admission = admit(&leaves, FOUR_LEAVES).expect("valid profile must be admitted");
    assert_eq!(admission.leaf_count, FOUR_LEAVES);
    assert_eq!(admission.leaf_limit, FOUR_LEAVES);
    assert_eq!(admission.profile_version, BALANCED_U24_PROFILE_VERSION);
}

// r[verify bounded-modular-tree-fold.admission]
// r[verify bounded-modular-tree-fold.fixtures]
#[test]
fn admission_rejects_every_declared_boundary() {
    assert_eq!(admit(&[], 1), Err(Rejection::EmptyInput));
    assert_eq!(admit(&[0], 0), Err(Rejection::ZeroLeafLimit));
    assert_eq!(
        classify_admission(AdmissionFacts {
            leaf_count: MAX_ACCOUNTED_LEAVES + 1,
            leaf_limit: u64::MAX,
            power_of_two: true,
            leaves_valid: true,
        }),
        Err(Rejection::AccountingOverflow),
    );
    assert_eq!(admit(&[0, 1], 1), Err(Rejection::OverLeafLimit),);
    assert_eq!(
        classify_admission(AdmissionFacts {
            leaf_count: u128::from(THREE_LEAVES),
            leaf_limit: THREE_LEAVES,
            power_of_two: false,
            leaves_valid: true,
        }),
        Err(Rejection::NonPowerOfTwo),
    );
    assert_eq!(admit(&[0, MODULUS], TWO_LEAVES), Err(Rejection::InvalidLeaf),);
}

// r[verify bounded-modular-tree-fold.refinement]
// r[verify bounded-modular-tree-fold.work]
#[test]
fn singleton_has_zero_work_and_preserves_leaf() {
    let outcome = balanced_u24_sum(&[MAX], 1).expect("singleton must be admitted");
    assert_eq!(outcome.value, MAX);
    assert_eq!(outcome.combine_count, 0);
    assert_eq!(outcome.round_count, 0);
}

// r[verify bounded-modular-tree-fold.refinement]
// r[verify bounded-modular-tree-fold.equivalence]
// r[verify bounded-modular-tree-fold.work]
#[test]
fn multi_round_reduction_matches_sequential_oracle_and_counts() {
    let balanced = balanced_u24_sum(&MULTI_ROUND_LEAVES, EIGHT_LEAVES).expect("balanced input must pass");
    let sequential = sequential_u24_sum(&MULTI_ROUND_LEAVES, EIGHT_LEAVES).expect("sequential input must pass");
    assert_eq!(balanced.value, sequential.value);
    assert_eq!(balanced.value, independent_math_oracle(&MULTI_ROUND_LEAVES));
    assert_eq!(balanced.combine_count, EIGHT_COMBINES);
    assert_eq!(balanced.round_count, EIGHT_ROUNDS);
}

// r[verify bounded-modular-tree-fold.boundary]
// r[verify bounded-modular-tree-fold.evidence]
// r[verify bounded-modular-tree-fold.fixtures]
// r[verify bounded-modular-tree-fold.non-claims]
// r[verify bounded-modular-tree-fold.docs]
#[test]
fn profile_and_oracle_identifiers_are_exact_and_versioned() {
    assert_eq!(BALANCED_U24_PROFILE, "trellis.balanced-u24-wrapping-sum.v1");
    assert_eq!(SEQUENTIAL_U24_ORACLE_SYMBOL, "verified_logic::bounded_modular_tree_fold::sequential_u24_sum",);
    assert_ne!(BALANCED_U24_PROFILE, "trellis.balanced-u24-wrapping-sum.v2");
}

fn admitted_leaf_vectors() -> impl Strategy<Value = Vec<u64>> {
    (0u32..MAX_PROPERTY_EXPONENT).prop_flat_map(|exponent| {
        let leaf_count = 1usize << exponent;
        proptest::collection::vec(0u64..MODULUS, leaf_count)
    })
}

proptest! {
    // r[verify bounded-modular-tree-fold.fixtures]
    // r[verify bounded-modular-tree-fold.equivalence]
    // r[verify bounded-modular-tree-fold.validation]
    #[test]
    fn generated_admitted_vectors_match_three_oracles(leaves in admitted_leaf_vectors()) {
        let leaf_limit = u64::try_from(leaves.len()).expect("generated leaf count must fit in u64");
        let sequential = sequential_u24_sum(&leaves, leaf_limit).expect("generated profile must pass");
        let balanced = balanced_u24_sum(&leaves, leaf_limit).expect("generated profile must pass");
        let independent = independent_math_oracle(&leaves);
        prop_assert_eq!(sequential.value, independent);
        prop_assert_eq!(balanced.value, independent);
        prop_assert_eq!(balanced.combine_count, leaf_limit - 1);
    }
}
