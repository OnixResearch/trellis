//! Verus refinements and local proof functions for balanced `u24` reduction.

// Verus sequence views require the exact owned vector type on these internal proof-facing helpers.
#![allow(tigerstyle::borrowed_argument_types)]

#[cfg(verus_keep_ghost)]
use vstd::arithmetic::div_mod::lemma_fundamental_div_mod;
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use super::all_u24_spec;
#[cfg(verus_keep_ghost)]
use super::balanced_combine_count_spec;
#[cfg(verus_keep_ghost)]
use super::balanced_u24_sum_spec;
#[cfg(verus_keep_ghost)]
use super::ordered_u24_sum_from_spec;
#[cfg(verus_keep_ghost)]
use super::ordered_u24_sum_spec;
#[cfg(verus_keep_ghost)]
use super::pair_round_spec;
#[cfg(verus_keep_ghost)]
use super::power_of_two_spec;

verus! {

proof fn ordered_sum_closed_from(leaves: Seq<u64>, from: int)
    requires
        0 <= from <= leaves.len(),
        all_u24_spec(leaves),
    ensures crate::u24::valid_spec(ordered_u24_sum_from_spec(leaves, from))
    decreases leaves.len() - from,
{
    if from < leaves.len() {
        ordered_sum_closed_from(leaves, from + 1);
        crate::u24::u24_add_closed(leaves[from], ordered_u24_sum_from_spec(leaves, from + 1));
    }
}

// r[impl bounded-modular-tree-fold.refinement]
/// Iterative oracle over an already admitted immutable vector.
pub fn sequential_oracle(leaves: &Vec<u64>) -> (result: u64)
    requires
        leaves@.len() > 0,
        all_u24_spec(leaves@),
    ensures
        result == ordered_u24_sum_spec(leaves@),
        crate::u24::valid_spec(result),
{
    let mut index = 0usize;
    let mut accumulator = crate::u24::ZERO;
    proof {
        ordered_sum_closed_from(leaves@, 0);
        crate::u24::u24_add_zero_identity(ordered_u24_sum_spec(leaves@));
    }
    while index < leaves.len()
        invariant
            0 <= index <= leaves@.len(),
            crate::u24::valid_spec(accumulator),
            all_u24_spec(leaves@),
            crate::u24::add_spec(accumulator, ordered_u24_sum_from_spec(leaves@, index as int))
                == ordered_u24_sum_spec(leaves@),
        decreases leaves@.len() - index,
    {
        proof {
            ordered_sum_closed_from(leaves@, index as int + 1);
            crate::u24::u24_add_associative(
                accumulator,
                leaves@[index as int],
                ordered_u24_sum_from_spec(leaves@, index as int + 1),
            );
        }
        accumulator = crate::u24::add(accumulator, leaves[index]);
        index += 1;
    }
    proof {
        crate::u24::u24_add_zero_identity(accumulator);
    }
    accumulator
}

// r[impl bounded-modular-tree-fold.refinement]
/// Builds one adjacent-pair round without recursive runtime calls.
pub fn pair_round(leaves: &Vec<u64>) -> (result: Vec<u64>)
    requires
        leaves@.len() > 0,
        leaves@.len() % (super::PAIR_WIDTH as nat) == 0,
        all_u24_spec(leaves@),
    ensures
        result@ =~= pair_round_spec(leaves@),
        result@.len() == leaves@.len() / (super::PAIR_WIDTH as nat),
        all_u24_spec(result@),
{
    let pair_count = leaves.len() / super::PAIR_WIDTH;
    let mut result = Vec::with_capacity(pair_count);
    let mut pair = 0usize;
    while pair < pair_count
        invariant
            pair_count as int == leaves@.len() / (super::PAIR_WIDTH as nat),
            0 <= pair <= pair_count,
            result@.len() == pair,
            result@ =~= Seq::new(pair as nat, |item: int| {
                crate::u24::add_spec(
                    leaves@[item * (super::PAIR_WIDTH as int)],
                    leaves@[item * (super::PAIR_WIDTH as int) + 1],
                )
            }),
            all_u24_spec(leaves@),
            all_u24_spec(result@),
        decreases pair_count - pair,
    {
        assert(leaves@.len() == leaves.len() as int);
        assert(leaves@.len() <= usize::MAX as int);
        assert((pair as int) * (super::PAIR_WIDTH as int) + 1 < leaves@.len());
        assert((pair as int) * (super::PAIR_WIDTH as int) <= usize::MAX as int) by(nonlinear_arith)
            requires
                (pair as int) * (super::PAIR_WIDTH as int) + 1 < leaves@.len(),
                leaves@.len() <= usize::MAX as int;
        let left_index = pair * super::PAIR_WIDTH;
        let right_index = left_index + 1;
        assert(right_index < leaves.len());
        let combined = crate::u24::add(leaves[left_index], leaves[right_index]);
        result.push(combined);
        assert(result@ =~= Seq::new((pair + 1) as nat, |item: int| {
            crate::u24::add_spec(
                leaves@[item * (super::PAIR_WIDTH as int)],
                leaves@[item * (super::PAIR_WIDTH as int) + 1],
            )
        }));
        pair += 1;
    }
    result
}

proof fn pair_round_sum_from(leaves: Seq<u64>, pair: int)
    requires
        leaves.len() > 0,
        leaves.len() % (super::PAIR_WIDTH as nat) == 0,
        0 <= pair <= leaves.len() / (super::PAIR_WIDTH as nat),
        all_u24_spec(leaves),
    ensures
        ordered_u24_sum_from_spec(pair_round_spec(leaves), pair)
            == ordered_u24_sum_from_spec(leaves, pair * (super::PAIR_WIDTH as int))
    decreases leaves.len() / (super::PAIR_WIDTH as nat) - pair,
{
    let round = pair_round_spec(leaves);
    assert(round.len() == leaves.len() / (super::PAIR_WIDTH as nat));
    lemma_fundamental_div_mod(leaves.len() as int, super::PAIR_WIDTH as int);
    assert(leaves.len()
        == (leaves.len() / (super::PAIR_WIDTH as nat)) * (super::PAIR_WIDTH as nat)) by(nonlinear_arith)
        requires leaves.len() % (super::PAIR_WIDTH as nat) == 0;
    if pair < leaves.len() / (super::PAIR_WIDTH as nat) {
        assert(0 <= pair * (super::PAIR_WIDTH as int));
        assert(pair * (super::PAIR_WIDTH as int) + 1 < leaves.len()) by(nonlinear_arith)
            requires
                pair < leaves.len() / (super::PAIR_WIDTH as nat),
                leaves.len()
                    == (leaves.len() / (super::PAIR_WIDTH as nat)) * (super::PAIR_WIDTH as nat);
        assert(pair * (super::PAIR_WIDTH as int) + (super::PAIR_WIDTH as int) <= leaves.len()) by(nonlinear_arith)
            requires
                pair < leaves.len() / (super::PAIR_WIDTH as nat),
                leaves.len()
                    == (leaves.len() / (super::PAIR_WIDTH as nat)) * (super::PAIR_WIDTH as nat);
        pair_round_sum_from(leaves, pair + 1);
        ordered_sum_closed_from(
            leaves,
            pair * (super::PAIR_WIDTH as int) + (super::PAIR_WIDTH as int),
        );
        let left = leaves[pair * (super::PAIR_WIDTH as int)];
        let right = leaves[pair * (super::PAIR_WIDTH as int) + 1];
        let tail = ordered_u24_sum_from_spec(
            leaves,
            pair * (super::PAIR_WIDTH as int) + (super::PAIR_WIDTH as int),
        );
        assert(round[pair] == crate::u24::add_spec(left, right));
        assert(ordered_u24_sum_from_spec(round, pair)
            == crate::u24::add_spec(crate::u24::add_spec(left, right), ordered_u24_sum_from_spec(round, pair + 1)));
        assert(ordered_u24_sum_from_spec(leaves, pair * (super::PAIR_WIDTH as int))
            == crate::u24::add_spec(
                left,
                ordered_u24_sum_from_spec(leaves, pair * (super::PAIR_WIDTH as int) + 1),
            ));
        assert(ordered_u24_sum_from_spec(leaves, pair * (super::PAIR_WIDTH as int) + 1)
            == crate::u24::add_spec(right, tail));
        assert(ordered_u24_sum_from_spec(round, pair + 1) == tail);
        crate::u24::u24_add_associative(left, right, tail);
    } else {
        assert(pair == leaves.len() / (super::PAIR_WIDTH as nat));
        assert(pair * (super::PAIR_WIDTH as int) == leaves.len());
        assert(ordered_u24_sum_from_spec(round, pair) == crate::u24::ZERO);
        assert(ordered_u24_sum_from_spec(
            leaves,
            pair * (super::PAIR_WIDTH as int),
        ) == crate::u24::ZERO);
    }
}

// r[verify bounded-modular-tree-fold.equivalence]
/// One adjacent-pair round preserves the ordered modular sum.
pub proof fn u24_pair_round_preserves_sum(leaves: Seq<u64>)
    requires
        leaves.len() > 0,
        leaves.len() % (super::PAIR_WIDTH as nat) == 0,
        all_u24_spec(leaves),
    ensures ordered_u24_sum_spec(pair_round_spec(leaves)) == ordered_u24_sum_spec(leaves)
{
    pair_round_sum_from(leaves, 0);
}

// r[verify bounded-modular-tree-fold.work]
/// A nonterminal power-of-two round remains admitted and strictly shrinks.
pub proof fn power_of_two_round_progress(leaf_count: nat)
    requires
        leaf_count > 1,
        power_of_two_spec(leaf_count),
    ensures
        leaf_count % (super::PAIR_WIDTH as nat) == 0,
        power_of_two_spec(leaf_count / (super::PAIR_WIDTH as nat)),
        0 < leaf_count / (super::PAIR_WIDTH as nat) < leaf_count,
{
}

// r[verify bounded-modular-tree-fold.work]
/// Every admitted balanced schedule performs exactly `n - 1` combines.
pub proof fn power_of_two_combine_count_exact(leaf_count: nat)
    requires
        leaf_count > 0,
        power_of_two_spec(leaf_count),
    ensures balanced_combine_count_spec(leaf_count) == leaf_count - 1
    decreases leaf_count,
{
    if leaf_count > 1 {
        power_of_two_round_progress(leaf_count);
        power_of_two_combine_count_exact(leaf_count / (super::PAIR_WIDTH as nat));
    }
}

// r[impl bounded-modular-tree-fold.refinement]
/// Iterative balanced reducer over an already admitted immutable vector.
pub fn balanced_reducer(leaves: &Vec<u64>) -> (result: u64)
    requires
        leaves@.len() > 0,
        power_of_two_spec(leaves@.len()),
        all_u24_spec(leaves@),
    ensures
        result == balanced_u24_sum_spec(leaves@),
        result == ordered_u24_sum_spec(leaves@),
        crate::u24::valid_spec(result),
{
    let ghost original = leaves@;
    let mut current = leaves.clone();
    while current.len() > 1
        invariant
            current@.len() > 0,
            power_of_two_spec(current@.len()),
            all_u24_spec(current@),
            ordered_u24_sum_spec(current@) == ordered_u24_sum_spec(original),
        decreases current@.len(),
    {
        proof {
            power_of_two_round_progress(current@.len());
            u24_pair_round_preserves_sum(current@);
        }
        current = pair_round(&current);
    }
    assert(current@.len() == 1);
    proof {
        assert(ordered_u24_sum_from_spec(current@, 1) == crate::u24::ZERO);
        assert(ordered_u24_sum_spec(current@)
            == crate::u24::add_spec(current@[0], crate::u24::ZERO));
        crate::u24::u24_add_zero_identity(current@[0]);
        assert(current@[0] == ordered_u24_sum_spec(current@));
        assert(current@[0] == ordered_u24_sum_spec(original));
        assert(balanced_u24_sum_spec(original) == ordered_u24_sum_spec(original));
    }
    current[0]
}

} // verus!
