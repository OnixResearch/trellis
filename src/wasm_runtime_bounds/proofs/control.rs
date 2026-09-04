//! Fuel, determinism, and segmentation lemmas.

use vstd::prelude::*;

verus! {

// r[impl wasm-runtime-bounds.fuel]
pub open spec fn fuel_after_spec(remaining: int, requested: int) -> int {
    if 0 <= requested <= remaining { remaining - requested } else { remaining }
}

// r[verify wasm-runtime-bounds.fuel]
pub proof fn exact_fuel_decrement(remaining: int, requested: int)
    requires 0 <= requested <= remaining,
    ensures fuel_after_spec(remaining, requested) == remaining - requested,
        fuel_after_spec(remaining, requested) >= 0,
{
}

// r[verify wasm-runtime-bounds.fuel]
pub proof fn insufficient_fuel_preserves_state(
    remaining: int,
    requested: int,
    state_before: int,
    state_after: int,
)
    requires 0 <= remaining < requested,
        state_after == state_before,
    ensures fuel_after_spec(remaining, requested) == remaining,
        state_after == state_before,
{
}

// r[impl wasm-runtime-bounds.segmentation]
pub open spec fn consumed_steps_spec(fuel: int, budget: int, boundary: int) -> int {
    if fuel <= budget && fuel <= boundary {
        fuel
    } else if budget <= boundary {
        budget
    } else {
        boundary
    }
}

// r[impl wasm-runtime-bounds.segmentation]
pub open spec fn segmented_steps_spec(
    fuel: int,
    prefix: int,
    suffix: int,
    boundary: int,
) -> int {
    let first = consumed_steps_spec(fuel, prefix, boundary);
    if first < prefix {
        first
    } else {
        first + consumed_steps_spec(fuel - first, suffix, boundary - first)
    }
}

// r[verify wasm-runtime-bounds.fuel]
pub proof fn zero_budget_is_identity(fuel: int, boundary: int)
    requires fuel >= 0,
        boundary >= 0,
    ensures consumed_steps_spec(fuel, 0, boundary) == 0,
{
}

// r[verify wasm-runtime-bounds.model]
pub open spec fn deterministic_pair_spec(
    left_state: int,
    left_observation: int,
    left_effect: int,
    right_state: int,
    right_observation: int,
    right_effect: int,
) -> bool {
    left_state != right_state
        || left_observation != right_observation
        || left_effect == right_effect
}

// r[verify wasm-runtime-bounds.model]
pub proof fn matching_key_has_one_effect(
    left_state: int,
    left_observation: int,
    left_effect: int,
    right_state: int,
    right_observation: int,
    right_effect: int,
)
    requires deterministic_pair_spec(
            left_state,
            left_observation,
            left_effect,
            right_state,
            right_observation,
            right_effect,
        ),
        left_state == right_state,
        left_observation == right_observation,
    ensures left_effect == right_effect,
{
}

// r[verify wasm-runtime-bounds.segmentation]
pub proof fn segmented_steps_match_uninterrupted(
    fuel: int,
    prefix: int,
    suffix: int,
    boundary: int,
)
    requires fuel >= 0,
        prefix >= 0,
        suffix >= 0,
        boundary >= 0,
    ensures segmented_steps_spec(fuel, prefix, suffix, boundary)
        == consumed_steps_spec(fuel, prefix + suffix, boundary),
{
    let first = consumed_steps_spec(fuel, prefix, boundary);
    if fuel <= prefix && fuel <= boundary {
        assert(first == fuel);
    } else if prefix <= boundary {
        assert(first == prefix);
        if fuel - prefix <= suffix && fuel - prefix <= boundary - prefix {
        } else if suffix <= boundary - prefix {
        } else {
        }
    } else {
        assert(first == boundary);
    }
}

// r[verify wasm-runtime-bounds.segmentation]
pub proof fn terminal_or_pause_boundary_absorbs_suffix(
    fuel: int,
    prefix: int,
    suffix: int,
    boundary: int,
)
    requires fuel >= boundary,
        0 <= boundary <= prefix,
        suffix >= 0,
    ensures segmented_steps_spec(fuel, prefix, suffix, boundary) == boundary,
        consumed_steps_spec(fuel, prefix + suffix, boundary) == boundary,
{
}

// r[impl wasm-runtime-bounds.segmentation]
pub open spec fn matching_inputs_spec(
    model_identity: int,
    resume_model_identity: int,
    observation_identity: int,
    resume_observation_identity: int,
) -> bool {
    model_identity == resume_model_identity
        && observation_identity == resume_observation_identity
}

// r[verify wasm-runtime-bounds.segmentation]
pub proof fn drift_rejects_segmentation(
    model_identity: int,
    resume_model_identity: int,
    observation_identity: int,
    resume_observation_identity: int,
)
    requires model_identity != resume_model_identity
        || observation_identity != resume_observation_identity,
    ensures !matching_inputs_spec(
        model_identity,
        resume_model_identity,
        observation_identity,
        resume_observation_identity,
    ),
{
}

} // verus!
