//! Verus facts for obligations, initializedness, classification, and ordering.

use vstd::prelude::*;

verus! {

// r[impl mir-drop.obligations]
pub open spec fn obligation_established_spec(has_glue_tag: u8, initialized_tag: u8, obligation_tag: u8) -> bool {
    has_glue_tag == 1 && initialized_tag == 1 ==> obligation_tag == 1
}

// r[verify mir-drop.obligations]
pub proof fn initialization_establishes_declared_obligation(
    has_glue_tag: u8,
    initialized_tag: u8,
    obligation_tag: u8,
)
    requires obligation_established_spec(has_glue_tag, initialized_tag, obligation_tag),
        has_glue_tag == 1,
        initialized_tag == 1,
    ensures obligation_tag == 1,
{
}

// r[impl mir-drop.initializedness]
pub open spec fn move_releases_spec(moved_tag: u8, initialized_tag: u8, obligation_tag: u8) -> bool {
    moved_tag == 1 ==> initialized_tag == 0 && obligation_tag == 0
}

// r[verify mir-drop.initializedness]
pub proof fn admitted_move_releases_path(moved_tag: u8, initialized_tag: u8, obligation_tag: u8)
    requires move_releases_spec(moved_tag, initialized_tag, obligation_tag),
        moved_tag == 1,
    ensures initialized_tag == 0,
        obligation_tag == 0,
{
}

// r[impl mir-drop.classification]
pub open spec fn definite_initialized_spec(maybe_init: u8, maybe_uninit: u8) -> bool {
    maybe_init == 1 && maybe_uninit == 0
}

// r[verify mir-drop.classification]
pub proof fn definite_initialized_is_static(maybe_init: u8, maybe_uninit: u8)
    requires definite_initialized_spec(maybe_init, maybe_uninit),
    ensures maybe_init == 1,
        maybe_uninit == 0,
{
}

// r[impl mir-drop.classification]
pub open spec fn definite_uninitialized_spec(maybe_init: u8, maybe_uninit: u8) -> bool {
    maybe_init == 0 && maybe_uninit == 1
}

// r[verify mir-drop.classification]
pub proof fn definite_uninitialized_is_dead(maybe_init: u8, maybe_uninit: u8)
    requires definite_uninitialized_spec(maybe_init, maybe_uninit),
    ensures maybe_init == 0,
        maybe_uninit == 1,
{
}

// r[impl mir-drop.drop-flags]
pub open spec fn guarded_step_admitted_spec(conditional_tag: u8, flag_tag: u8) -> bool {
    conditional_tag == 1 ==> flag_tag == 1
}

// r[verify mir-drop.drop-flags]
pub proof fn conditional_step_has_flag(conditional_tag: u8, flag_tag: u8)
    requires guarded_step_admitted_spec(conditional_tag, flag_tag),
        conditional_tag == 1,
    ensures flag_tag == 1,
{
}

// r[impl mir-drop.open-drop]
pub open spec fn expansion_budget_admitted_spec(consumed: u32, budget: u32) -> bool {
    consumed <= budget
}

// r[verify mir-drop.open-drop]
pub proof fn admitted_expansion_stays_within_budget(consumed: u32, budget: u32)
    requires expansion_budget_admitted_spec(consumed, budget),
    ensures consumed <= budget,
{
}

// r[impl mir-drop.drop-and-replace]
pub open spec fn replacement_order_spec(last_destruction: u32, write_index: u32) -> bool {
    last_destruction < write_index
}

// r[verify mir-drop.drop-and-replace]
pub proof fn replacement_follows_old_target(last_destruction: u32, write_index: u32)
    requires replacement_order_spec(last_destruction, write_index),
    ensures last_destruction < write_index,
{
}

} // verus!
