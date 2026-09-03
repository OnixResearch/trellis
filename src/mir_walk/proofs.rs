//! Verus correspondences for finite visitor admission predicates.

use vstd::prelude::*;

verus! {

pub const MISSING_TAG: u8 = 0;
pub const PRESENT_TAG: u8 = 1;
pub const DECLARED_ORDER_TAG: u8 = 0;
pub const REACHABLE_ONLY_TAG: u8 = 1;

// r[impl mir-visitor.locations]
pub open spec fn index_admitted_spec(index: u32, width: u32) -> bool {
    index < width
}

// r[impl mir-visitor.locations]
pub fn index_admitted(index: u32, width: u32) -> (admitted: bool)
    ensures admitted == index_admitted_spec(index, width),
{
    index < width
}

// r[verify mir-visitor.locations]
pub proof fn admitted_index_is_in_range(index: u32, width: u32)
    requires index_admitted_spec(index, width),
    ensures index < width,
{
}

// r[impl mir-visitor.coverage]
pub open spec fn coverage_admitted_spec(
    missing: u32,
    duplicates: u32,
    outside: u32,
    invalid: u32,
) -> bool {
    missing == 0 && duplicates == 0 && outside == 0 && invalid == 0
}

// r[impl mir-visitor.coverage]
pub fn coverage_admitted(
    missing: u32,
    duplicates: u32,
    outside: u32,
    invalid: u32,
) -> (admitted: bool)
    ensures admitted == coverage_admitted_spec(missing, duplicates, outside, invalid),
{
    missing == 0 && duplicates == 0 && outside == 0 && invalid == 0
}

// r[verify mir-visitor.coverage]
pub proof fn admitted_coverage_has_no_gap(
    missing: u32,
    duplicates: u32,
    outside: u32,
    invalid: u32,
)
    requires coverage_admitted_spec(missing, duplicates, outside, invalid),
    ensures missing == 0,
        duplicates == 0,
        outside == 0,
        invalid == 0,
{
}

// r[impl mir-visitor.coverage]
pub open spec fn location_member_spec(locations: Seq<u64>, location: u64) -> bool {
    exists|i: int| #![trigger locations[i]] 0 <= i < locations.len() && locations[i] == location
}

// r[impl mir-visitor.coverage]
pub open spec fn no_duplicate_locations_spec(locations: Seq<u64>) -> bool {
    forall|i: int, j: int|
        #![trigger locations[i], locations[j]]
        0 <= i < j < locations.len() ==> locations[i] != locations[j]
}

// r[impl mir-visitor.coverage]
pub open spec fn requested_locations_covered_spec(requested: Seq<u64>, visited: Seq<u64>) -> bool {
    forall|i: int| #![trigger requested[i]]
        0 <= i < requested.len() ==> location_member_spec(visited, requested[i])
}

// r[impl mir-visitor.coverage]
pub open spec fn no_outside_locations_spec(requested: Seq<u64>, visited: Seq<u64>) -> bool {
    forall|i: int| #![trigger visited[i]]
        0 <= i < visited.len() ==> location_member_spec(requested, visited[i])
}

// r[impl mir-visitor.coverage]
pub open spec fn exactly_once_locations_spec(requested: Seq<u64>, visited: Seq<u64>) -> bool {
    no_duplicate_locations_spec(requested)
        && no_duplicate_locations_spec(visited)
        && requested_locations_covered_spec(requested, visited)
        && no_outside_locations_spec(requested, visited)
}

// r[verify mir-visitor.coverage]
pub proof fn exact_coverage_contains_requested(
    requested: Seq<u64>,
    visited: Seq<u64>,
    index: int,
)
    requires exactly_once_locations_spec(requested, visited),
        0 <= index < requested.len(),
    ensures location_member_spec(visited, requested[index]),
{
}

// r[verify mir-visitor.coverage]
pub proof fn exact_coverage_contains_no_outside(
    requested: Seq<u64>,
    visited: Seq<u64>,
    index: int,
)
    requires exactly_once_locations_spec(requested, visited),
        0 <= index < visited.len(),
    ensures location_member_spec(requested, visited[index]),
{
}

// r[impl mir-visitor.locations]
pub open spec fn all_location_indices_in_range_spec(locations: Seq<u32>, width: u32) -> bool {
    forall|i: int| #![trigger locations[i]]
        0 <= i < locations.len() ==> locations[i] < width
}

// r[verify mir-visitor.locations]
pub proof fn admitted_location_index_is_valid(locations: Seq<u32>, width: u32, index: int)
    requires all_location_indices_in_range_spec(locations, width),
        0 <= index < locations.len(),
    ensures locations[index] < width,
{
}

// r[impl mir-visitor.traversal-order]
pub open spec fn order_starts_at_entry_spec(first: u32, entry: u32, count: u32) -> bool {
    count > 0 && first == entry
}

// r[impl mir-visitor.traversal-order]
pub fn order_starts_at_entry(first: u32, entry: u32, count: u32) -> (admitted: bool)
    ensures admitted == order_starts_at_entry_spec(first, entry, count),
{
    count > 0 && first == entry
}

// r[verify mir-visitor.traversal-order]
pub proof fn admitted_order_starts_at_entry(first: u32, entry: u32, count: u32)
    requires order_starts_at_entry_spec(first, entry, count),
    ensures count > 0,
        first == entry,
{
}

// r[impl mir-visitor.mut-shape]
pub open spec fn mutation_admitted_spec(
    mismatch_count: u32,
    before_admitted: u8,
    after_admitted: u8,
) -> bool {
    mismatch_count == 0
        && before_admitted == PRESENT_TAG
        && after_admitted == PRESENT_TAG
}

// r[impl mir-visitor.mut-shape]
pub fn mutation_admitted(
    mismatch_count: u32,
    before_admitted: u8,
    after_admitted: u8,
) -> (admitted: bool)
    ensures admitted == mutation_admitted_spec(mismatch_count, before_admitted, after_admitted),
{
    mismatch_count == 0
        && before_admitted == PRESENT_TAG
        && after_admitted == PRESENT_TAG
}

// r[verify mir-visitor.mut-shape]
pub proof fn admitted_mutation_preserves_shape(
    mismatch_count: u32,
    before_admitted: u8,
    after_admitted: u8,
)
    requires mutation_admitted_spec(mismatch_count, before_admitted, after_admitted),
    ensures mismatch_count == 0,
        before_admitted == PRESENT_TAG,
        after_admitted == PRESENT_TAG,
{
}

// r[impl mir-visitor.admission]
pub open spec fn mode_admitted_spec(order_tag: u8, reachability_tag: u8) -> bool {
    order_tag == DECLARED_ORDER_TAG || reachability_tag == REACHABLE_ONLY_TAG
}

// r[impl mir-visitor.admission]
pub fn mode_admitted(order_tag: u8, reachability_tag: u8) -> (admitted: bool)
    ensures admitted == mode_admitted_spec(order_tag, reachability_tag),
{
    order_tag == DECLARED_ORDER_TAG || reachability_tag == REACHABLE_ONLY_TAG
}

// r[verify mir-visitor.admission]
pub proof fn ordered_mode_requires_reachable_policy(order_tag: u8, reachability_tag: u8)
    requires mode_admitted_spec(order_tag, reachability_tag),
        order_tag != DECLARED_ORDER_TAG,
    ensures reachability_tag == REACHABLE_ONLY_TAG,
{
}

} // verus!
