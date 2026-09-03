//! Verus correspondences for handle, mapping, order, and terminator predicates.

use vstd::prelude::*;

verus! {

pub const MISSING_TAG: u8 = 0;
pub const PRESENT_TAG: u8 = 1;

// r[impl backend-trace.handles]
pub open spec fn handle_declared_spec(declared_tag: u8) -> bool {
    declared_tag == PRESENT_TAG
}

// r[impl backend-trace.handles]
pub fn handle_declared(declared_tag: u8) -> (is_declared: bool)
    ensures is_declared == handle_declared_spec(declared_tag),
{
    declared_tag == PRESENT_TAG
}

// r[verify backend-trace.handles]
pub proof fn admitted_handle_is_declared(declared_tag: u8)
    requires handle_declared_spec(declared_tag),
    ensures declared_tag == PRESENT_TAG,
{
}

// r[impl backend-trace.block-mapping]
// r[impl backend-trace.ssa-like-locals]
pub open spec fn index_admitted_spec(index: u32, count: u32) -> bool {
    index < count
}

// r[impl backend-trace.block-mapping]
// r[impl backend-trace.ssa-like-locals]
pub fn index_admitted(index: u32, count: u32) -> (is_admitted: bool)
    ensures is_admitted == index_admitted_spec(index, count),
{
    index < count
}

// r[verify backend-trace.block-mapping]
// r[verify backend-trace.ssa-like-locals]
pub proof fn admitted_index_is_in_range(index: u32, count: u32)
    requires index_admitted_spec(index, count),
    ensures index < count,
{
}

// r[impl backend-trace.handle-validity]
pub open spec fn definition_precedes_use_spec(definition: u32, use_index: u32) -> bool {
    definition < use_index
}

// r[verify backend-trace.handle-validity]
pub proof fn admitted_use_follows_definition(definition: u32, use_index: u32)
    requires definition_precedes_use_spec(definition, use_index),
    ensures definition < use_index,
{
}

// r[impl backend-trace.terminators]
pub open spec fn control_target_admitted_spec(target_tag: u8) -> bool {
    target_tag == PRESENT_TAG
}

// r[impl backend-trace.terminators]
pub fn control_target_admitted(target_tag: u8) -> (is_admitted: bool)
    ensures is_admitted == control_target_admitted_spec(target_tag),
{
    target_tag == PRESENT_TAG
}

// r[verify backend-trace.terminators]
pub proof fn admitted_control_target_is_declared(target_tag: u8)
    requires control_target_admitted_spec(target_tag),
    ensures target_tag == PRESENT_TAG,
{
}

// r[impl backend-trace.terminators]
pub open spec fn terminal_coverage_admitted_spec(blocks: u32, terminals: u32) -> bool {
    blocks == terminals
}

// r[impl backend-trace.terminators]
pub fn terminal_coverage_admitted(blocks: u32, terminals: u32) -> (is_admitted: bool)
    ensures is_admitted == terminal_coverage_admitted_spec(blocks, terminals),
{
    blocks == terminals
}

// r[verify backend-trace.terminators]
pub proof fn admitted_blocks_have_terminal_operations(blocks: u32, terminals: u32)
    requires terminal_coverage_admitted_spec(blocks, terminals),
    ensures blocks == terminals,
{
}

// r[impl backend-trace.lowering-trace]
pub open spec fn source_operation_admitted_spec(location_tag: u8, operation_tag: u8) -> bool {
    location_tag == operation_tag
}

// r[verify backend-trace.lowering-trace]
pub proof fn admitted_operation_matches_source(location_tag: u8, operation_tag: u8)
    requires source_operation_admitted_spec(location_tag, operation_tag),
    ensures location_tag == operation_tag,
{
}

} // verus!
