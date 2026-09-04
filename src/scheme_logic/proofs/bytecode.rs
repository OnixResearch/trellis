//! Constant-pool and stack-summary lemmas.

use vstd::prelude::*;

verus! {

// r[impl scheme-logic.bytecode-safety]
pub open spec fn add_or_get_spec(
    old_length: int,
    existing_index: int,
    returned_index: int,
    was_present: bool,
) -> bool {
    if was_present {
        0 <= existing_index < old_length && returned_index == existing_index
    } else {
        old_length >= 0 && returned_index == old_length
    }
}

// r[verify scheme-logic.bytecode-safety]
pub proof fn existing_constant_keeps_index(
    old_length: int,
    existing_index: int,
    returned_index: int,
)
    requires add_or_get_spec(
        old_length,
        existing_index,
        returned_index,
        true,
    ),
    ensures returned_index == existing_index,
        0 <= returned_index < old_length,
{
}

// r[impl scheme-logic.bytecode-safety]
pub open spec fn rollback_spec(constants: Seq<int>, retained: int) -> Seq<int>
    recommends 0 <= retained <= constants.len(),
{
    constants.subrange(0, retained)
}

// r[verify scheme-logic.bytecode-safety]
pub proof fn rollback_preserves_every_retained_constant(
    constants: Seq<int>,
    retained: int,
    index: int,
)
    requires 0 <= index < retained <= constants.len(),
    ensures rollback_spec(constants, retained)[index] == constants[index],
        rollback_spec(constants, retained).len() == retained,
{
}

// r[impl scheme-logic.bytecode-safety]
pub open spec fn push_height_spec(height: int) -> int {
    height + 1
}

// r[verify scheme-logic.bytecode-safety]
pub proof fn push_then_pop_restores_height(height: int)
    requires height >= 0,
    ensures push_height_spec(height) - 1 == height,
{
}

// r[impl scheme-logic.bytecode-safety]
pub open spec fn call_height_spec(height: int, argument_count: int) -> int {
    height - argument_count
}

// r[verify scheme-logic.bytecode-safety]
pub proof fn admitted_call_cannot_underflow(
    height: int,
    argument_count: int,
)
    requires argument_count >= 0,
        height >= argument_count + 1,
    ensures call_height_spec(height, argument_count) >= 1,
{
}

// r[impl scheme-logic.bytecode-safety]
pub open spec fn operand_in_bounds_spec(operand: int, bound: int) -> bool {
    0 <= operand < bound
}

// r[verify scheme-logic.bytecode-safety]
pub proof fn admitted_operand_names_declared_slot(operand: int, bound: int)
    requires operand_in_bounds_spec(operand, bound),
    ensures 0 <= operand,
        operand < bound,
{
}

} // verus!
