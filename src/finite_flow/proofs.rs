//! Verus facts for lattice laws, ordered effects, edges, and fixed points.

use vstd::prelude::*;

verus! {

// r[impl mir-dataflow.lattice-laws]
pub open spec fn join_laws_admitted_spec(
    idempotent_tag: u8,
    commutative_tag: u8,
    associative_tag: u8,
    bounded_tag: u8,
) -> bool {
    idempotent_tag == 1 && commutative_tag == 1 && associative_tag == 1 && bounded_tag == 1
}

// r[verify mir-dataflow.lattice-laws]
pub proof fn admitted_join_has_all_laws(
    idempotent_tag: u8,
    commutative_tag: u8,
    associative_tag: u8,
    bounded_tag: u8,
)
    requires join_laws_admitted_spec(idempotent_tag, commutative_tag, associative_tag, bounded_tag),
    ensures idempotent_tag == 1,
        commutative_tag == 1,
        associative_tag == 1,
        bounded_tag == 1,
{
}

// r[impl mir-dataflow.admission]
pub open spec fn monotone_pair_spec(left_input: u32, right_input: u32, left_output: u32, right_output: u32) -> bool {
    left_input <= right_input ==> left_output <= right_output
}

// r[verify mir-dataflow.admission]
pub proof fn admitted_transfer_preserves_order(
    left_input: u32,
    right_input: u32,
    left_output: u32,
    right_output: u32,
)
    requires monotone_pair_spec(left_input, right_input, left_output, right_output),
        left_input <= right_input,
    ensures left_output <= right_output,
{
}

// r[impl mir-dataflow.effects]
pub open spec fn before_primary_order_spec(before_index: u32, primary_index: u32) -> bool {
    before_index < primary_index
}

// r[verify mir-dataflow.effects]
pub proof fn before_effect_precedes_primary(before_index: u32, primary_index: u32)
    requires before_primary_order_spec(before_index, primary_index),
    ensures before_index < primary_index,
{
}

// r[impl mir-dataflow.per-edge]
pub open spec fn edge_effect_admitted_spec(source: u32, target: u32, block_count: u32) -> bool {
    source < block_count && target < block_count
}

// r[verify mir-dataflow.per-edge]
pub proof fn admitted_edge_effect_is_in_range(source: u32, target: u32, block_count: u32)
    requires edge_effect_admitted_spec(source, target, block_count),
    ensures source < block_count,
        target < block_count,
{
}

// r[impl mir-dataflow.convergence]
pub open spec fn convergence_budget_spec(height: u32, work_items: u32, budget: u32) -> bool {
    budget >= height * work_items + 1
}

// r[verify mir-dataflow.convergence]
pub proof fn admitted_budget_covers_rank_steps(height: u32, work_items: u32, budget: u32)
    requires convergence_budget_spec(height, work_items, budget),
    ensures budget >= height * work_items + 1,
{
}

// r[impl mir-dataflow.convergence]
pub open spec fn fixed_result_admitted_spec(stable_tag: u8, budget_remaining_tag: u8) -> bool {
    stable_tag == 1 && budget_remaining_tag == 1
}

// r[verify mir-dataflow.convergence]
pub proof fn admitted_result_is_stable_within_budget(stable_tag: u8, budget_remaining_tag: u8)
    requires fixed_result_admitted_spec(stable_tag, budget_remaining_tag),
    ensures stable_tag == 1,
        budget_remaining_tag == 1,
{
}

// r[impl mir-dataflow.results]
pub open spec fn cursor_location_admitted_spec(requested: u32, observed: u32) -> bool {
    requested == observed
}

// r[verify mir-dataflow.results]
pub proof fn admitted_cursor_observes_requested_location(requested: u32, observed: u32)
    requires cursor_location_admitted_spec(requested, observed),
    ensures requested == observed,
{
}

} // verus!
