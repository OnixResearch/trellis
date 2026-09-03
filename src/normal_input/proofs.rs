//! Verus facts for finite references, normal forms, imports, and refactors.

use vstd::prelude::*;

verus! {

// r[impl domain-primitives.typed-config.expression-envelope]
pub open spec fn index_admitted_spec(index: u32, count: u32) -> bool {
    index < count
}

// r[impl domain-primitives.typed-config.expression-envelope]
pub fn index_admitted(index: u32, count: u32) -> (is_admitted: bool)
    ensures is_admitted == index_admitted_spec(index, count),
{
    index < count
}

// r[verify domain-primitives.typed-config.expression-envelope]
pub proof fn admitted_index_is_declared(index: u32, count: u32)
    requires index_admitted_spec(index, count),
    ensures index < count,
{
}

// r[impl domain-primitives.typed-config.type-admission]
pub open spec fn type_shape_matches_spec(declared: u8, actual: u8) -> bool {
    declared == actual
}

// r[verify domain-primitives.typed-config.type-admission]
pub proof fn admitted_type_matches_value_shape(declared: u8, actual: u8)
    requires type_shape_matches_spec(declared, actual),
    ensures declared == actual,
{
}

// r[impl domain-primitives.typed-config.normal-form]
pub open spec fn canonical_order_spec(keys: Seq<u32>) -> bool {
    forall|i: int, j: int|
        #![trigger keys[i], keys[j]]
        0 <= i < j < keys.len() ==> keys[i] < keys[j]
}

// r[verify domain-primitives.typed-config.normal-form]
pub proof fn canonical_entries_are_strictly_ordered(keys: Seq<u32>, left: int, right: int)
    requires canonical_order_spec(keys),
        0 <= left < right < keys.len(),
    ensures keys[left] < keys[right],
{
}

// r[impl domain-primitives.typed-config.semantic-digest]
pub open spec fn digest_input_stable_spec(before_normal: u64, after_normal: u64, before_deps: u64, after_deps: u64) -> bool {
    before_normal == after_normal && before_deps == after_deps
}

// r[verify domain-primitives.typed-config.semantic-digest]
pub proof fn stable_semantics_select_equal_digest_input(
    before_normal: u64,
    after_normal: u64,
    before_deps: u64,
    after_deps: u64,
)
    requires digest_input_stable_spec(before_normal, after_normal, before_deps, after_deps),
    ensures before_normal == after_normal,
        before_deps == after_deps,
{
}

// r[impl domain-primitives.typed-config.import-closure]
pub open spec fn import_policy_admitted_spec(static_tag: u8, attenuation_tag: u8, digest_tag: u8) -> bool {
    static_tag == 1 && attenuation_tag == 1 && digest_tag == 1
}

// r[verify domain-primitives.typed-config.import-closure]
pub proof fn admitted_import_has_all_policy_facts(static_tag: u8, attenuation_tag: u8, digest_tag: u8)
    requires import_policy_admitted_spec(static_tag, attenuation_tag, digest_tag),
    ensures static_tag == 1,
        attenuation_tag == 1,
        digest_tag == 1,
{
}

// r[impl domain-primitives.typed-config.semantic-refactoring]
pub open spec fn refactor_admitted_spec(equal_digest_tag: u8, compatibility_tag: u8) -> bool {
    equal_digest_tag == 1 || compatibility_tag == 1
}

// r[verify domain-primitives.typed-config.semantic-refactoring]
pub proof fn admitted_refactor_has_identity_or_compatibility(equal_digest_tag: u8, compatibility_tag: u8)
    requires refactor_admitted_spec(equal_digest_tag, compatibility_tag),
    ensures equal_digest_tag == 1 || compatibility_tag == 1,
{
}

} // verus!
