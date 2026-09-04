//! Interner stability and roundtrip lemmas.

use vstd::prelude::*;

verus! {

// r[impl scheme-logic.interning]
pub open spec fn stable_key_spec(
    existing_key: int,
    returned_key: int,
    was_inserted: bool,
) -> bool {
    !was_inserted && returned_key == existing_key
}

// r[verify scheme-logic.interning]
pub proof fn existing_association_keeps_key(
    existing_key: int,
    returned_key: int,
    was_inserted: bool,
)
    requires stable_key_spec(existing_key, returned_key, was_inserted),
    ensures returned_key == existing_key,
        !was_inserted,
{
}

// r[impl scheme-logic.interning]
pub open spec fn appended_association_spec(
    old_length: int,
    new_length: int,
    key: int,
    resolved_value: int,
    input_value: int,
) -> bool {
    0 <= old_length && new_length == old_length + 1
        && key == old_length
        && resolved_value == input_value
}

// r[verify scheme-logic.interning]
pub proof fn appended_key_resolves_input(
    old_length: int,
    new_length: int,
    key: int,
    resolved_value: int,
    input_value: int,
)
    requires appended_association_spec(
        old_length,
        new_length,
        key,
        resolved_value,
        input_value,
    ),
    ensures key == old_length,
        new_length == old_length + 1,
        resolved_value == input_value,
{
}

// r[impl scheme-logic.interning]
pub open spec fn bijection_pair_spec(
    left_key: int,
    left_value: int,
    right_key: int,
    right_value: int,
) -> bool {
    (left_key == right_key) == (left_value == right_value)
}

// r[verify scheme-logic.interning]
pub proof fn unique_keys_and_values_form_pairwise_bijection(
    left_key: int,
    left_value: int,
    right_key: int,
    right_value: int,
)
    requires bijection_pair_spec(left_key, left_value, right_key, right_value),
    ensures left_key == right_key ==> left_value == right_value,
        left_value == right_value ==> left_key == right_key,
{
}

} // verus!
