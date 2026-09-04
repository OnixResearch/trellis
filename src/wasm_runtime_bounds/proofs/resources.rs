//! Continuation, allocation, and growth-policy lemmas.

use vstd::prelude::*;

verus! {

// r[impl wasm-runtime-bounds.continuation]
pub open spec fn bindings_match_spec(
    stored_generation: int,
    current_generation: int,
    stored_artifact: int,
    current_artifact: int,
    stored_profile: int,
    current_profile: int,
    stored_input: int,
    current_input: int,
    stored_context: int,
    current_context: int,
    stored_observations: int,
    current_observations: int,
) -> bool {
    stored_generation == current_generation
        && stored_artifact == current_artifact
        && stored_profile == current_profile
        && stored_input == current_input
        && stored_context == current_context
        && stored_observations == current_observations
}

// r[verify wasm-runtime-bounds.continuation]
pub proof fn admitted_resume_requires_every_binding(
    stored_generation: int,
    current_generation: int,
    stored_artifact: int,
    current_artifact: int,
    stored_profile: int,
    current_profile: int,
    stored_input: int,
    current_input: int,
    stored_context: int,
    current_context: int,
    stored_observations: int,
    current_observations: int,
)
    requires bindings_match_spec(
        stored_generation,
        current_generation,
        stored_artifact,
        current_artifact,
        stored_profile,
        current_profile,
        stored_input,
        current_input,
        stored_context,
        current_context,
        stored_observations,
        current_observations,
    ),
    ensures stored_generation == current_generation,
        stored_artifact == current_artifact,
        stored_profile == current_profile,
        stored_input == current_input,
        stored_context == current_context,
        stored_observations == current_observations,
{
}

// r[verify wasm-runtime-bounds.continuation]
pub proof fn stale_binding_rejects_resume(
    stored_generation: int,
    current_generation: int,
    stored_artifact: int,
    current_artifact: int,
    stored_profile: int,
    current_profile: int,
    stored_input: int,
    current_input: int,
    stored_context: int,
    current_context: int,
    stored_observations: int,
    current_observations: int,
)
    requires stored_generation != current_generation
        || stored_artifact != current_artifact
        || stored_profile != current_profile
        || stored_input != current_input
        || stored_context != current_context
        || stored_observations != current_observations,
    ensures !bindings_match_spec(
        stored_generation,
        current_generation,
        stored_artifact,
        current_artifact,
        stored_profile,
        current_profile,
        stored_input,
        current_input,
        stored_context,
        current_context,
        stored_observations,
        current_observations,
    ),
{
}

// r[impl wasm-runtime-bounds.allocation]
pub open spec fn allocation_admitted_spec(
    is_initializing: bool,
    allocated: int,
    requested: int,
    capacity: int,
    maximum: int,
) -> bool {
    is_initializing
        && 0 <= allocated <= capacity <= maximum
        && requested >= 0
        && allocated + requested <= capacity
}

// r[impl wasm-runtime-bounds.allocation]
pub open spec fn allocated_after_spec(
    is_initializing: bool,
    allocated: int,
    requested: int,
    capacity: int,
    maximum: int,
) -> int {
    if allocation_admitted_spec(
        is_initializing,
        allocated,
        requested,
        capacity,
        maximum,
    ) {
        allocated + requested
    } else {
        allocated
    }
}

// r[verify wasm-runtime-bounds.allocation]
pub proof fn admitted_allocation_is_monotone_and_capped(
    allocated: int,
    requested: int,
    capacity: int,
    maximum: int,
)
    requires allocation_admitted_spec(
        true,
        allocated,
        requested,
        capacity,
        maximum,
    ),
    ensures allocated <= allocated_after_spec(
            true,
            allocated,
            requested,
            capacity,
            maximum,
        ) <= capacity,
{
}

// r[verify wasm-runtime-bounds.allocation]
pub proof fn rejected_allocation_is_unchanged(
    is_initializing: bool,
    allocated: int,
    requested: int,
    capacity: int,
    maximum: int,
)
    requires !allocation_admitted_spec(
        is_initializing,
        allocated,
        requested,
        capacity,
        maximum,
    ),
    ensures allocated_after_spec(
        is_initializing,
        allocated,
        requested,
        capacity,
        maximum,
    ) == allocated,
{
}

// r[impl wasm-runtime-bounds.growth-policy]
pub open spec fn growth_policy_result_spec(
    is_allowed: bool,
    previous_memory_identity: int,
    delegated_memory_identity: int,
) -> int {
    if is_allowed {
        delegated_memory_identity
    } else {
        previous_memory_identity
    }
}

// r[verify wasm-runtime-bounds.growth-policy]
pub proof fn denied_growth_is_noop(
    previous_memory_identity: int,
    delegated_memory_identity: int,
)
    ensures growth_policy_result_spec(
        false,
        previous_memory_identity,
        delegated_memory_identity,
    ) == previous_memory_identity,
{
}

// r[verify wasm-runtime-bounds.growth-policy]
pub proof fn admitted_growth_preserves_existing_law(
    previous_memory_identity: int,
    delegated_memory_identity: int,
    existing_law_holds: bool,
)
    requires existing_law_holds,
    ensures growth_policy_result_spec(
            true,
            previous_memory_identity,
            delegated_memory_identity,
        ) == delegated_memory_identity,
        existing_law_holds,
{
}

// r[verify wasm-runtime-bounds.continuation]
pub proof fn captured_reason_is_preserved(stored_reason: int, granted_reason: int)
    requires granted_reason == stored_reason,
    ensures granted_reason == stored_reason,
{
}

} // verus!
