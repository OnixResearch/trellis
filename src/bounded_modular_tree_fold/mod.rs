//! Verified bounded balanced reduction over the Trellis `u24` domain.
//!
//! The sequential oracle and pairwise reducer share one ordered modular-sum
//! specification. External Bend and HVM2 behavior remains outside this module.

use vstd::prelude::*;

// r[impl bounded-modular-tree-fold.boundary]
// r[impl bounded-modular-tree-fold.evidence]
// r[impl bounded-modular-tree-fold.non-claims]
// r[impl bounded-modular-tree-fold.docs]
// r[impl bounded-modular-tree-fold.validation]
pub const BALANCED_U24_PROFILE: &str = "trellis.balanced-u24-wrapping-sum.v1";
pub const SEQUENTIAL_U24_ORACLE_SYMBOL: &str = "verified_logic::bounded_modular_tree_fold::sequential_u24_sum";
pub const BALANCED_U24_PROFILE_VERSION: u32 = 1;
const PAIR_WIDTH_U64: u64 = PAIR_WIDTH as u64;
pub const MAX_ACCOUNTED_LEAVES: u128 = u64::MAX as u128;
pub const MAX_U64_HALVING_ROUNDS: u32 = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rejection {
    EmptyInput,
    ZeroLeafLimit,
    AccountingOverflow,
    OverLeafLimit,
    NonPowerOfTwo,
    InvalidLeaf,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Admission {
    pub leaf_count: u64,
    pub leaf_limit: u64,
    pub profile_version: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Outcome {
    pub value: u64,
    pub leaf_count: u64,
    pub combine_count: u64,
    pub round_count: u32,
    pub profile_version: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdmissionFacts {
    pub leaf_count: u128,
    pub leaf_limit: u64,
    pub power_of_two: bool,
    pub leaves_valid: bool,
}

// r[impl bounded-modular-tree-fold.admission]
/// Classifies supplied profile facts without reading caller input or host state.
pub fn classify_admission(facts: AdmissionFacts) -> Result<Admission, Rejection> {
    if facts.leaf_count == 0 {
        return Err(Rejection::EmptyInput);
    }
    if facts.leaf_limit == 0 {
        return Err(Rejection::ZeroLeafLimit);
    }
    if facts.leaf_count > MAX_ACCOUNTED_LEAVES {
        return Err(Rejection::AccountingOverflow);
    }
    let leaf_count = match u64::try_from(facts.leaf_count) {
        Ok(value) => value,
        Err(_) => return Err(Rejection::AccountingOverflow),
    };
    if facts.leaf_count > u128::from(facts.leaf_limit) {
        return Err(Rejection::OverLeafLimit);
    }
    if !facts.power_of_two {
        return Err(Rejection::NonPowerOfTwo);
    }
    if !facts.leaves_valid {
        return Err(Rejection::InvalidLeaf);
    }
    Ok(Admission {
        leaf_count,
        leaf_limit: facts.leaf_limit,
        profile_version: BALANCED_U24_PROFILE_VERSION,
    })
}

fn runtime_power_of_two(mut value: u64) -> bool {
    if value == 0 {
        return false;
    }
    while value > 1 {
        if value % PAIR_WIDTH_U64 != 0 {
            return false;
        }
        value /= PAIR_WIDTH_U64;
    }
    true
}

// r[impl bounded-modular-tree-fold.admission]
/// Admits one immutable leaf slice under an explicit caller-provided limit.
pub fn admit(leaves: &[u64], leaf_limit: u64) -> Result<Admission, Rejection> {
    let leaf_count = leaves.len() as u128;
    let count_for_shape = match u64::try_from(leaf_count) {
        Ok(value) => value,
        Err(_) => return Err(Rejection::AccountingOverflow),
    };
    let facts = AdmissionFacts {
        leaf_count,
        leaf_limit,
        power_of_two: runtime_power_of_two(count_for_shape),
        leaves_valid: leaves.iter().all(|value| *value <= crate::u24::MAX),
    };
    classify_admission(facts)
}

fn runtime_halving_rounds(mut leaf_count: u64) -> u32 {
    let mut rounds = 0u32;
    while leaf_count > 1 {
        assert!(rounds < MAX_U64_HALVING_ROUNDS);
        leaf_count /= PAIR_WIDTH_U64;
        rounds += 1;
    }
    rounds
}

verus! {

// r[impl bounded-modular-tree-fold.refinement]
/// Number of leaves combined by each balanced-reduction step.
pub const PAIR_WIDTH: usize = 2;

// r[impl bounded-modular-tree-fold.admission]
// r[depends bounded-modular-tree-fold.admission]
pub open spec fn power_of_two_spec(value: nat) -> bool
    decreases value,
{
    if value == 1 {
        true
    } else if value == 0 || value % (PAIR_WIDTH as nat) != 0 {
        false
    } else {
        power_of_two_spec(value / (PAIR_WIDTH as nat))
    }
}

// r[impl bounded-modular-tree-fold.admission]
// r[depends bounded-modular-tree-fold.admission]
pub open spec fn all_u24_spec(leaves: Seq<u64>) -> bool {
    forall|index: int| 0 <= index < leaves.len() ==> crate::u24::valid_spec(leaves[index])
}

// r[impl bounded-modular-tree-fold.refinement]
// r[depends bounded-modular-tree-fold.refinement]
pub open spec fn ordered_u24_sum_from_spec(leaves: Seq<u64>, from: int) -> u64
    decreases leaves.len() - from,
{
    if from < 0 || from >= leaves.len() {
        crate::u24::ZERO
    } else {
        crate::u24::add_spec(leaves[from], ordered_u24_sum_from_spec(leaves, from + 1))
    }
}

// r[impl bounded-modular-tree-fold.refinement]
// r[depends bounded-modular-tree-fold.refinement]
pub open spec fn ordered_u24_sum_spec(leaves: Seq<u64>) -> u64 {
    ordered_u24_sum_from_spec(leaves, 0)
}

// r[impl bounded-modular-tree-fold.refinement]
// r[impl bounded-modular-tree-fold.equivalence]
// r[depends bounded-modular-tree-fold.refinement]
pub open spec fn pair_round_spec(leaves: Seq<u64>) -> Seq<u64> {
    Seq::new(leaves.len() / (PAIR_WIDTH as nat), |pair: int| {
        crate::u24::add_spec(
            leaves[pair * (PAIR_WIDTH as int)],
            leaves[pair * (PAIR_WIDTH as int) + 1],
        )
    })
}

// r[impl bounded-modular-tree-fold.refinement]
// r[depends bounded-modular-tree-fold.refinement]
pub open spec fn balanced_u24_sum_spec(leaves: Seq<u64>) -> u64 {
    ordered_u24_sum_spec(leaves)
}

// r[impl bounded-modular-tree-fold.work]
// r[depends bounded-modular-tree-fold.work]
pub open spec fn balanced_combine_count_spec(leaf_count: nat) -> nat
    decreases leaf_count,
{
    if leaf_count <= 1 {
        0
    } else {
        leaf_count / (PAIR_WIDTH as nat)
            + balanced_combine_count_spec(leaf_count / (PAIR_WIDTH as nat))
    }
}

// r[impl bounded-modular-tree-fold.work]
// r[depends bounded-modular-tree-fold.work]
pub open spec fn balanced_round_count_spec(leaf_count: nat) -> nat
    decreases leaf_count,
{
    if leaf_count <= 1 {
        0
    } else {
        1 + balanced_round_count_spec(leaf_count / (PAIR_WIDTH as nat))
    }
}

} // verus!

mod core;

pub use core::balanced_reducer;
pub use core::pair_round;
pub use core::sequential_oracle;

// r[impl bounded-modular-tree-fold.refinement]
// r[impl bounded-modular-tree-fold.admission]
/// Runs the optimized sequential oracle after exact profile admission.
pub fn sequential_u24_sum(leaves: &[u64], leaf_limit: u64) -> Result<Outcome, Rejection> {
    let admission = admit(leaves, leaf_limit)?;
    let owned = leaves.to_vec();
    let value = sequential_oracle(&owned);
    Ok(Outcome {
        value,
        leaf_count: admission.leaf_count,
        combine_count: admission.leaf_count - 1,
        round_count: runtime_halving_rounds(admission.leaf_count),
        profile_version: admission.profile_version,
    })
}

// r[impl bounded-modular-tree-fold.refinement]
// r[impl bounded-modular-tree-fold.admission]
/// Runs the iterative adjacent-pair reducer after exact profile admission.
pub fn balanced_u24_sum(leaves: &[u64], leaf_limit: u64) -> Result<Outcome, Rejection> {
    let admission = admit(leaves, leaf_limit)?;
    let owned = leaves.to_vec();
    let value = balanced_reducer(&owned);
    Ok(Outcome {
        value,
        leaf_count: admission.leaf_count,
        combine_count: admission.leaf_count - 1,
        round_count: runtime_halving_rounds(admission.leaf_count),
        profile_version: admission.profile_version,
    })
}

#[cfg(test)]
mod tests;
