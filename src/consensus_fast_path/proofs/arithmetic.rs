//! Quorum arithmetic and finite-cardinality lemmas.

use vstd::prelude::*;

verus! {

// r[impl consensus.fast-path.quorum]
pub open spec fn cluster_count_spec(faults: int) -> int {
    2 * faults + 1
}

// r[impl consensus.fast-path.quorum]
pub open spec fn original_majority_spec(faults: int) -> int {
    faults + 1
}

// r[impl consensus.fast-path.quorum]
pub open spec fn support_count_spec(faults: int) -> int {
    (faults + 1) / 2 + 1
}

// r[impl consensus.fast-path.quorum]
pub open spec fn accelerated_count_spec(faults: int) -> int {
    faults + (faults + 1) / 2 + 1
}

proof fn quotient_bound(value: int, divisor: int, dividend: int)
    requires divisor >= 1, value >= 0, dividend >= 0,
    ensures (value <= dividend / divisor) <==> (value * divisor <= dividend),
{
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod(dividend, divisor);
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    assert(dividend == divisor * quotient + remainder);
    if value <= quotient {
        assert(divisor * value <= divisor * quotient) by(nonlinear_arith)
            requires value >= 0, value <= quotient, divisor >= 1;
        assert(divisor * value <= dividend);
    }
    if value > quotient {
        assert(value >= quotient + 1);
        assert(divisor * value >= divisor * (quotient + 1)) by(nonlinear_arith)
            requires value >= quotient + 1, divisor >= 1;
        assert(divisor * (quotient + 1) == divisor * quotient + divisor) by(nonlinear_arith)
            requires quotient >= 0, divisor >= 1;
        assert(divisor * value > dividend);
    }
}

proof fn half_partition(faults: int)
    requires faults >= 0,
    ensures faults / 2 + (faults + 1) / 2 == faults,
{
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod(faults, 2);
    vstd::arithmetic::div_mod::lemma_mod_pos_bound(faults, 2);
    vstd::arithmetic::div_mod::lemma_div_pos_is_pos(faults, 2);
    let floor = faults / 2;
    let remainder = faults % 2;
    assert(faults == 2 * floor + remainder);
    assert(0 <= remainder < 2);
    assert(floor >= 0);
    if remainder == 0 {
        quotient_bound(floor, 2, faults + 1);
        quotient_bound(floor + 1, 2, faults + 1);
        assert(floor * 2 <= faults + 1);
        assert(floor <= (faults + 1) / 2);
        assert((floor + 1) * 2 > faults + 1);
        assert(!(floor + 1 <= (faults + 1) / 2));
        assert((faults + 1) / 2 == floor);
    } else {
        assert(remainder == 1);
        quotient_bound(floor + 1, 2, faults + 1);
        quotient_bound(floor + 2, 2, faults + 1);
        assert((floor + 1) * 2 <= faults + 1);
        assert(floor + 1 <= (faults + 1) / 2);
        assert((floor + 2) * 2 > faults + 1);
        assert(!(floor + 2 <= (faults + 1) / 2));
        assert((faults + 1) / 2 == floor + 1);
    }
}

// r[verify consensus.fast-path.quorum]
pub proof fn quorum_is_bounded(faults: int)
    requires faults >= 1,
    ensures
        accelerated_count_spec(faults) <= cluster_count_spec(faults),
        accelerated_count_spec(faults) > original_majority_spec(faults),
{
    half_partition(faults);
    vstd::arithmetic::div_mod::lemma_div_pos_is_pos(faults, 2);
    assert(faults / 2 >= 0);
    assert((faults + 1) / 2 <= faults);
}

// r[verify consensus.fast-path.quorum]
pub proof fn failure_tolerance_is_floor_half(faults: int)
    requires faults >= 1,
    ensures cluster_count_spec(faults) - accelerated_count_spec(faults) == faults / 2,
{
    half_partition(faults);
}

// r[verify consensus.fast-path.quorum]
pub proof fn quorum_recovery_identity(faults: int)
    requires faults >= 1,
    ensures accelerated_count_spec(faults) + original_majority_spec(faults)
        - cluster_count_spec(faults) == support_count_spec(faults),
{
}

// r[verify consensus.fast-path.quorum]
pub proof fn finite_membership_intersection(
    faults: int,
    union_count: int,
    overlap_count: int,
)
    requires
        faults >= 1,
        union_count >= 0,
        overlap_count >= 0,
        union_count <= cluster_count_spec(faults),
        accelerated_count_spec(faults) + original_majority_spec(faults)
            == union_count + overlap_count,
    ensures overlap_count >= support_count_spec(faults),
{
    quorum_recovery_identity(faults);
}

// r[verify consensus.fast-path.quorum]
pub proof fn threshold_is_sharp(faults: int)
    requires faults >= 1,
    ensures
        (accelerated_count_spec(faults) - 1) + original_majority_spec(faults)
            - cluster_count_spec(faults) == support_count_spec(faults) - 1,
        (accelerated_count_spec(faults) - 1) + original_majority_spec(faults)
            - cluster_count_spec(faults) < support_count_spec(faults),
{
    quorum_recovery_identity(faults);
}

// r[verify consensus.fast-path.quorum]
pub proof fn three_replica_corollary()
    ensures
        cluster_count_spec(1) == 3,
        original_majority_spec(1) == 2,
        accelerated_count_spec(1) == 3,
        cluster_count_spec(1) - accelerated_count_spec(1) == 0,
{
}

// r[verify consensus.fast-path.quorum]
pub proof fn five_replica_corollary()
    ensures
        cluster_count_spec(2) == 5,
        original_majority_spec(2) == 3,
        accelerated_count_spec(2) == 4,
        support_count_spec(2) == 2,
{
}

// r[verify consensus.fast-path.certificate]
pub proof fn conflicting_pair_cannot_coexist(
    faults: int,
    union_count: int,
    overlap_count: int,
    both_valid: bool,
    same_view: bool,
    conflicting: bool,
)
    requires
        faults >= 1,
        union_count >= 0,
        overlap_count >= 0,
        union_count <= cluster_count_spec(faults),
        2 * accelerated_count_spec(faults) == union_count + overlap_count,
        same_view,
        conflicting,
        both_valid ==> !(same_view && conflicting && overlap_count > 0),
    ensures !both_valid,
{
    half_partition(faults);
    vstd::arithmetic::div_mod::lemma_div_pos_is_pos(faults + 1, 2);
    assert((faults + 1) / 2 >= 0);
    assert(2 * accelerated_count_spec(faults) > cluster_count_spec(faults));
    assert(overlap_count > 0);
}

} // verus!
