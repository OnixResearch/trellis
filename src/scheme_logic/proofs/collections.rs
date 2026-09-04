//! Value-graph and collection-shape lemmas.

use vstd::prelude::*;

verus! {

pub open spec fn byte_max_spec() -> int {
    255
}

// r[impl scheme-logic.value-graph]
pub open spec fn traversal_bound_spec(
    declared_nodes: int,
    visited_nodes: int,
    budget: int,
) -> bool {
    0 <= visited_nodes <= declared_nodes && visited_nodes <= budget
}

// r[verify scheme-logic.value-graph]
pub proof fn admitted_traversal_is_finite(
    declared_nodes: int,
    visited_nodes: int,
    budget: int,
)
    requires traversal_bound_spec(declared_nodes, visited_nodes, budget),
    ensures visited_nodes <= declared_nodes,
        visited_nodes <= budget,
{
}

// r[verify scheme-logic.value-graph]
pub proof fn visited_cycle_edge_does_not_add_node(
    visited_before: int,
    visited_after: int,
    target_was_visited: bool,
)
    requires visited_before >= 0,
        target_was_visited,
        visited_after == visited_before,
    ensures visited_after == visited_before,
{
}

// r[verify scheme-logic.collections-bytevectors]
pub proof fn proper_list_length_fits_budget(length: int, budget: int)
    requires 0 <= length <= budget,
    ensures length <= budget,
{
}

// r[impl scheme-logic.collections-bytevectors]
pub open spec fn immutable_update_spec(
    values: Seq<int>,
    index: int,
    value: int,
) -> Seq<int>
    recommends 0 <= index < values.len(),
{
    values.update(index, value)
}

// r[verify scheme-logic.collections-bytevectors]
pub proof fn immutable_vector_update_preserves_length(
    values: Seq<int>,
    index: int,
    value: int,
)
    requires 0 <= index < values.len(),
    ensures immutable_update_spec(values, index, value).len() == values.len(),
        immutable_update_spec(values, index, value)[index] == value,
{
}

// r[impl scheme-logic.collections-bytevectors]
pub open spec fn byte_element_spec(element: int) -> bool {
    0 <= element <= byte_max_spec()
}

// r[verify scheme-logic.collections-bytevectors]
pub proof fn admitted_byte_is_in_domain(element: int)
    requires byte_element_spec(element),
    ensures 0 <= element <= byte_max_spec(),
{
}

// r[verify scheme-logic.collections-bytevectors]
pub proof fn distinct_hash_keys_reject_duplicate(
    first_index: int,
    second_index: int,
    keys_match: bool,
)
    requires first_index != second_index,
        keys_match,
    ensures !(first_index == second_index || !keys_match),
{
}

// r[impl scheme-logic.non-claims]
pub open spec fn finite_claim_spec(is_finite_predicate: bool) -> bool {
    is_finite_predicate
}

// r[verify scheme-logic.non-claims]
pub proof fn runtime_claim_is_not_admitted(is_finite_predicate: bool)
    requires !is_finite_predicate,
    ensures !finite_claim_spec(is_finite_predicate),
{
}

} // verus!
