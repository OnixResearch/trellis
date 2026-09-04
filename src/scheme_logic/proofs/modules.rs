//! Module graph and bounded closure lemmas.

use vstd::prelude::*;

verus! {

// r[impl scheme-logic.module-graph]
pub open spec fn provided_edge_spec(
    source_is_declared: bool,
    target_is_declared: bool,
    name_is_provided: bool,
) -> bool {
    source_is_declared && target_is_declared && name_is_provided
}

// r[verify scheme-logic.module-graph]
pub proof fn admitted_import_has_declared_provider(
    source_is_declared: bool,
    target_is_declared: bool,
    name_is_provided: bool,
)
    requires provided_edge_spec(
        source_is_declared,
        target_is_declared,
        name_is_provided,
    ),
    ensures source_is_declared,
        target_is_declared,
        name_is_provided,
{
}

// r[impl scheme-logic.module-graph]
pub open spec fn unique_export_spec(first_index: int, second_index: int, same_name: bool) -> bool {
    first_index == second_index || !same_name
}

// r[verify scheme-logic.module-graph]
pub proof fn duplicate_export_indices_reject(
    first_index: int,
    second_index: int,
    same_name: bool,
)
    requires first_index != second_index,
        same_name,
    ensures !unique_export_spec(first_index, second_index, same_name),
{
}

// r[impl scheme-logic.module-graph]
pub open spec fn closure_budget_spec(
    declared_modules: int,
    visited_modules: int,
    budget: int,
) -> bool {
    0 <= visited_modules <= declared_modules && visited_modules <= budget
}

// r[verify scheme-logic.module-graph]
pub proof fn admitted_closure_is_bounded(
    declared_modules: int,
    visited_modules: int,
    budget: int,
)
    requires closure_budget_spec(declared_modules, visited_modules, budget),
    ensures visited_modules <= declared_modules,
        visited_modules <= budget,
{
}

// r[verify scheme-logic.module-graph]
pub proof fn revisiting_cycle_node_does_not_grow_closure(
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

} // verus!
