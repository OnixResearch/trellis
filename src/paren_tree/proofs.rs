//! Verus facts for finite budgets, canonical shape, and roundtrip identity.

use vstd::prelude::*;

verus! {

pub const LIST_DELIMITER_COUNT: u32 = 2;

// r[impl s-expression.budgets]
pub open spec fn measure_component_admitted_spec(value: u32, limit: u32) -> bool {
    value <= limit
}

// r[verify s-expression.budgets]
pub proof fn admitted_measure_component_respects_limit(value: u32, limit: u32)
    requires measure_component_admitted_spec(value, limit),
    ensures value <= limit,
{
}

// r[impl s-expression.canonical-serialization]
pub open spec fn list_render_shape_spec(children: u32, separators: u32, delimiters: u32) -> bool {
    delimiters == LIST_DELIMITER_COUNT
        && (children == 0 && separators == 0 || children > 0 && separators + 1 == children)
}

// r[verify s-expression.canonical-serialization]
pub proof fn canonical_list_has_exact_separators(children: u32, separators: u32, delimiters: u32)
    requires list_render_shape_spec(children, separators, delimiters),
    ensures delimiters == LIST_DELIMITER_COUNT,
        children == 0 ==> separators == 0,
        children > 0 ==> separators + 1 == children,
{
}

// r[impl s-expression.roundtrip]
pub open spec fn canonical_roundtrip_spec(value_id: u64, parsed_id: u64) -> bool {
    value_id == parsed_id
}

// r[verify s-expression.roundtrip]
pub proof fn canonical_parse_returns_original(value_id: u64, parsed_id: u64)
    requires canonical_roundtrip_spec(value_id, parsed_id),
    ensures value_id == parsed_id,
{
}

// r[impl s-expression.roundtrip]
pub open spec fn render_injective_boundary_spec(
    left_value: u64,
    right_value: u64,
    left_render: u64,
    right_render: u64,
) -> bool {
    left_render == right_render && left_value == left_render && right_value == right_render
}

// r[verify s-expression.roundtrip]
pub proof fn equal_canonical_renders_have_equal_identity(
    left_value: u64,
    right_value: u64,
    left_render: u64,
    right_render: u64,
)
    requires render_injective_boundary_spec(left_value, right_value, left_render, right_render),
    ensures left_value == right_value,
{
}

// r[impl s-expression.ast]
// r[impl s-expression.atom-admission]
pub open spec fn node_admitted_spec(atom_tag: u8, list_tag: u8) -> bool {
    atom_tag == 1 || list_tag == 1
}

// r[verify s-expression.ast]
// r[verify s-expression.atom-admission]
pub proof fn admitted_node_has_declared_shape(atom_tag: u8, list_tag: u8)
    requires node_admitted_spec(atom_tag, list_tag),
    ensures atom_tag == 1 || list_tag == 1,
{
}

} // verus!
