//! Contract arity, composition, and blame lemmas.

use vstd::prelude::*;

verus! {

// r[impl scheme-logic.contract-blame]
pub open spec fn arity_matches_spec(
    expected_parameters: int,
    actual_parameters: int,
    expected_results: int,
    actual_results: int,
) -> bool {
    expected_parameters == actual_parameters && expected_results == actual_results
}

// r[verify scheme-logic.contract-blame]
pub proof fn admitted_function_shape_matches_both_arities(
    expected_parameters: int,
    actual_parameters: int,
    expected_results: int,
    actual_results: int,
)
    requires arity_matches_spec(
        expected_parameters,
        actual_parameters,
        expected_results,
        actual_results,
    ),
    ensures expected_parameters == actual_parameters,
        expected_results == actual_results,
{
}

// r[impl scheme-logic.contract-blame]
pub open spec fn blamed_party_spec(
    is_domain_position: bool,
    caller: int,
    provider: int,
) -> int {
    if is_domain_position { caller } else { provider }
}

// r[verify scheme-logic.contract-blame]
pub proof fn domain_failure_blames_caller(caller: int, provider: int)
    ensures blamed_party_spec(true, caller, provider) == caller,
{
}

// r[verify scheme-logic.contract-blame]
pub proof fn codomain_failure_blames_provider(caller: int, provider: int)
    ensures blamed_party_spec(false, caller, provider) == provider,
{
}

// r[impl scheme-logic.contract-blame]
pub open spec fn wrappers_compose_spec(
    inner_parameters: int,
    inner_results: int,
    outer_parameters: int,
    outer_results: int,
    inner_provider: int,
    outer_caller: int,
) -> bool {
    inner_parameters == outer_parameters
        && inner_results == outer_results
        && inner_provider == outer_caller
}

// r[verify scheme-logic.contract-blame]
pub proof fn composition_preserves_shape_and_party_boundary(
    inner_parameters: int,
    inner_results: int,
    outer_parameters: int,
    outer_results: int,
    inner_provider: int,
    outer_caller: int,
)
    requires wrappers_compose_spec(
        inner_parameters,
        inner_results,
        outer_parameters,
        outer_results,
        inner_provider,
        outer_caller,
    ),
    ensures inner_parameters == outer_parameters,
        inner_results == outer_results,
        inner_provider == outer_caller,
{
}

} // verus!
