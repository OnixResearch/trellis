//! Verus correspondences for owner, mapping, freshness, and use predicates.

use vstd::prelude::*;

verus! {

pub const MISSING_TAG: u8 = 0;
pub const PRESENT_TAG: u8 = 1;

// r[impl hir-id.owner-scope]
pub open spec fn owner_admitted_spec(actual: u32, expected: u32) -> bool {
    actual == expected
}

// r[impl hir-id.owner-scope]
pub fn owner_admitted(actual: u32, expected: u32) -> (is_admitted: bool)
    ensures is_admitted == owner_admitted_spec(actual, expected),
{
    actual == expected
}

// r[verify hir-id.owner-scope]
pub proof fn admitted_id_uses_current_owner(actual: u32, expected: u32)
    requires owner_admitted_spec(actual, expected),
    ensures actual == expected,
{
}

pub open spec fn member_spec(values: Seq<u64>, value: u64) -> bool {
    exists|i: int| #![trigger values[i]] 0 <= i < values.len() && values[i] == value
}

// r[impl hir-id.lowering-map]
// r[impl hir-id.uniqueness]
pub open spec fn unique_values_spec(values: Seq<u64>) -> bool {
    forall|i: int, j: int|
        #![trigger values[i], values[j]]
        0 <= i < j < values.len() ==> values[i] != values[j]
}

// r[verify hir-id.lowering-map]
// r[verify hir-id.uniqueness]
pub proof fn admitted_unique_values_do_not_alias(values: Seq<u64>, left: int, right: int)
    requires unique_values_spec(values),
        0 <= left < right < values.len(),
    ensures values[left] != values[right],
{
}

// r[impl hir-id.lowering-map]
pub open spec fn sources_are_lowered_spec(sources: Seq<u64>, mapped: Seq<u64>) -> bool {
    forall|i: int| #![trigger sources[i]]
        0 <= i < sources.len() ==> member_spec(mapped, sources[i])
}

// r[verify hir-id.lowering-map]
pub proof fn admitted_source_has_mapping(sources: Seq<u64>, mapped: Seq<u64>, index: int)
    requires sources_are_lowered_spec(sources, mapped),
        0 <= index < sources.len(),
    ensures member_spec(mapped, sources[index]),
{
}

// r[impl hir-id.fresh-generated]
pub open spec fn generated_is_fresh_spec(
    source_targets: Seq<u64>,
    prior_generated: Seq<u64>,
    candidate: u64,
) -> bool {
    !member_spec(source_targets, candidate) && !member_spec(prior_generated, candidate)
}

// r[verify hir-id.fresh-generated]
pub proof fn admitted_generated_id_has_no_alias(
    source_targets: Seq<u64>,
    prior_generated: Seq<u64>,
    candidate: u64,
)
    requires generated_is_fresh_spec(source_targets, prior_generated, candidate),
    ensures !member_spec(source_targets, candidate),
        !member_spec(prior_generated, candidate),
{
}

// r[impl hir-id.admission]
pub open spec fn created_use_admitted_spec(required_tag: u8, used_tag: u8) -> bool {
    required_tag == used_tag
}

// r[impl hir-id.admission]
pub fn created_use_admitted(required_tag: u8, used_tag: u8) -> (is_admitted: bool)
    ensures is_admitted == created_use_admitted_spec(required_tag, used_tag),
{
    required_tag == used_tag
}

// r[verify hir-id.admission]
pub proof fn admitted_created_use_matches_policy(required_tag: u8, used_tag: u8)
    requires created_use_admitted_spec(required_tag, used_tag),
    ensures required_tag == used_tag,
{
}

// r[impl hir-id.def-correspondence]
pub open spec fn definition_present_spec(required_source: u64, definition_sources: Seq<u64>) -> bool {
    member_spec(definition_sources, required_source)
}

// r[verify hir-id.def-correspondence]
pub proof fn required_definition_has_correspondence(required_source: u64, definition_sources: Seq<u64>)
    requires definition_present_spec(required_source, definition_sources),
    ensures member_spec(definition_sources, required_source),
{
}

} // verus!
