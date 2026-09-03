//! Verus correspondences for finite MIR-core admission predicates.

use vstd::prelude::*;

verus! {

pub const MISSING_TAG: u8 = 0;
pub const PRESENT_TAG: u8 = 1;
pub const NON_RETURN_ROLE_TAG: u8 = 2;
pub const INDEX_PROJECTION_TAG: u8 = 1;
pub const UNSUPPORTED_PROJECTION_TAG: u8 = 2;
pub const ADMITTED_RVALUE_TAG: u8 = 0;
pub const NESTED_RVALUE_TAG: u8 = 1;

// r[impl mir-core.return-place]
pub open spec fn return_place_admitted_spec(local_count: u32, role_tag: u8) -> bool {
    local_count > 0 && role_tag == PRESENT_TAG
}

// r[impl mir-core.return-place]
pub fn return_place_admitted(local_count: u32, role_tag: u8) -> (admitted: bool)
    ensures admitted == return_place_admitted_spec(local_count, role_tag),
{
    local_count > 0 && role_tag == PRESENT_TAG
}

// r[verify mir-core.return-place]
pub proof fn admitted_return_place_exists(local_count: u32, role_tag: u8)
    requires return_place_admitted_spec(local_count, role_tag),
    ensures local_count > 0,
        role_tag == PRESENT_TAG,
{
}

// r[impl mir-core.block-terminator]
pub open spec fn final_terminator_admitted_spec(terminator_tag: u8) -> bool {
    terminator_tag == PRESENT_TAG
}

// r[impl mir-core.block-terminator]
pub fn final_terminator_admitted(terminator_tag: u8) -> (admitted: bool)
    ensures admitted == final_terminator_admitted_spec(terminator_tag),
{
    terminator_tag == PRESENT_TAG
}

// r[verify mir-core.block-terminator]
pub proof fn admitted_block_has_final_terminator(terminator_tag: u8)
    requires final_terminator_admitted_spec(terminator_tag),
    ensures terminator_tag == PRESENT_TAG,
{
}

// r[impl mir-core.local-reference]
pub open spec fn local_index_admitted_spec(index: u32, local_count: u32) -> bool {
    index < local_count
}

// r[impl mir-core.local-reference]
pub fn local_index_admitted(index: u32, local_count: u32) -> (admitted: bool)
    ensures admitted == local_index_admitted_spec(index, local_count),
{
    index < local_count
}

// r[verify mir-core.local-reference]
pub proof fn admitted_local_is_in_range(index: u32, local_count: u32)
    requires local_index_admitted_spec(index, local_count),
    ensures index < local_count,
{
}

// r[impl mir-core.place-projection]
pub open spec fn projection_admitted_spec(
    projection_tag: u8,
    index_local: u32,
    local_count: u32,
) -> bool {
    projection_tag != UNSUPPORTED_PROJECTION_TAG
        && (projection_tag != INDEX_PROJECTION_TAG || index_local < local_count)
}

// r[impl mir-core.place-projection]
pub fn projection_admitted(
    projection_tag: u8,
    index_local: u32,
    local_count: u32,
) -> (admitted: bool)
    ensures admitted == projection_admitted_spec(projection_tag, index_local, local_count),
{
    projection_tag != UNSUPPORTED_PROJECTION_TAG
        && (projection_tag != INDEX_PROJECTION_TAG || index_local < local_count)
}

// r[verify mir-core.place-projection]
pub proof fn admitted_projection_has_valid_reference(
    projection_tag: u8,
    index_local: u32,
    local_count: u32,
)
    requires projection_admitted_spec(projection_tag, index_local, local_count),
    ensures projection_tag != UNSUPPORTED_PROJECTION_TAG,
        projection_tag == INDEX_PROJECTION_TAG ==> index_local < local_count,
{
}

// r[impl mir-core.rvalue-nonnesting]
pub open spec fn rvalue_shape_admitted_spec(rvalue_tag: u8) -> bool {
    rvalue_tag != NESTED_RVALUE_TAG
}

// r[impl mir-core.rvalue-nonnesting]
pub fn rvalue_shape_admitted(rvalue_tag: u8) -> (admitted: bool)
    ensures admitted == rvalue_shape_admitted_spec(rvalue_tag),
{
    rvalue_tag != NESTED_RVALUE_TAG
}

// r[verify mir-core.rvalue-nonnesting]
pub proof fn admitted_rvalue_is_not_nested(rvalue_tag: u8)
    requires rvalue_shape_admitted_spec(rvalue_tag),
    ensures rvalue_tag != NESTED_RVALUE_TAG,
{
}

// r[impl mir-core.successors]
pub open spec fn successor_admitted_spec(target: u32, block_count: u32) -> bool {
    target < block_count
}

// r[impl mir-core.successors]
pub fn successor_admitted(target: u32, block_count: u32) -> (admitted: bool)
    ensures admitted == successor_admitted_spec(target, block_count),
{
    target < block_count
}

// r[verify mir-core.successors]
pub proof fn admitted_successor_is_in_range(target: u32, block_count: u32)
    requires successor_admitted_spec(target, block_count),
    ensures target < block_count,
{
}

// r[impl mir-core.downstream-adapter]
pub open spec fn adapter_edge_admitted_spec(source: u32, target: u32, block_count: u32) -> bool {
    source < block_count && target < block_count
}

// r[impl mir-core.downstream-adapter]
pub fn adapter_edge_admitted(source: u32, target: u32, block_count: u32) -> (admitted: bool)
    ensures admitted == adapter_edge_admitted_spec(source, target, block_count),
{
    source < block_count && target < block_count
}

// r[verify mir-core.downstream-adapter]
pub proof fn admitted_adapter_edge_is_in_range(source: u32, target: u32, block_count: u32)
    requires adapter_edge_admitted_spec(source, target, block_count),
    ensures source < block_count,
        target < block_count,
{
}

} // verus!
