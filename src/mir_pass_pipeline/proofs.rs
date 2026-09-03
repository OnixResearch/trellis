//! Verus correspondences for finite MIR pipeline admission predicates.

use vstd::prelude::*;

verus! {

pub const FINAL_PHASE_RANK: u8 = 4;
pub const STOLEN_STATE_TAG: u8 = 2;

// r[impl mir-pass.phase-order]
pub open spec fn phase_step_allowed_spec(current: u8, target: u8) -> bool {
    current < FINAL_PHASE_RANK && target == current + 1
}

// r[impl mir-pass.phase-order]
pub fn phase_step_allowed(current: u8, target: u8) -> (allowed: bool)
    ensures allowed == phase_step_allowed_spec(current, target),
{
    current < FINAL_PHASE_RANK && target == current + 1
}

// r[verify mir-pass.phase-order]
pub proof fn admitted_phase_step_is_forward(current: u8, target: u8)
    requires phase_step_allowed_spec(current, target),
    ensures current < FINAL_PHASE_RANK,
        target == current + 1,
{
}

// r[impl mir-pass.pass-compatibility]
pub open spec fn pass_phase_compatible_spec(
    current: u8,
    input: u8,
    output: u8,
) -> bool {
    current == input
        && (output == input || phase_step_allowed_spec(input, output))
}

// r[impl mir-pass.pass-compatibility]
pub fn pass_phase_compatible(current: u8, input: u8, output: u8) -> (allowed: bool)
    ensures allowed == pass_phase_compatible_spec(current, input, output),
{
    current == input && (output == input || phase_step_allowed(input, output))
}

// r[verify mir-pass.pass-compatibility]
pub proof fn admitted_pass_uses_declared_phase(current: u8, input: u8, output: u8)
    requires pass_phase_compatible_spec(current, input, output),
    ensures current == input,
        output == input || phase_step_allowed_spec(input, output),
{
}

// r[impl mir-pass.artifact-state]
// r[impl mir-pass.steal-order]
pub open spec fn artifact_readable_spec(state_tag: u8) -> bool {
    state_tag != STOLEN_STATE_TAG
}

// r[impl mir-pass.artifact-state]
// r[impl mir-pass.steal-order]
pub fn artifact_readable(state_tag: u8) -> (readable: bool)
    ensures readable == artifact_readable_spec(state_tag),
{
    state_tag != STOLEN_STATE_TAG
}

// r[verify mir-pass.artifact-state]
// r[verify mir-pass.steal-order]
pub proof fn stolen_artifact_is_not_readable()
    ensures !artifact_readable_spec(STOLEN_STATE_TAG),
{
}

// r[impl mir-pass.optimization-gating]
pub open spec fn optimization_admits_spec(enabled: u8, required: u8) -> bool {
    enabled >= required
}

// r[impl mir-pass.optimization-gating]
pub fn optimization_admits(enabled: u8, required: u8) -> (allowed: bool)
    ensures allowed == optimization_admits_spec(enabled, required),
{
    enabled >= required
}

// r[verify mir-pass.optimization-gating]
pub proof fn admitted_optimization_meets_gate(enabled: u8, required: u8)
    requires optimization_admits_spec(enabled, required),
    ensures enabled >= required,
{
}

// r[impl mir-pass.pass-compatibility]
pub open spec fn validation_admits_spec(actual: u8, required: u8) -> bool {
    actual >= required
}

// r[impl mir-pass.pass-compatibility]
pub fn validation_admits(actual: u8, required: u8) -> (allowed: bool)
    ensures allowed == validation_admits_spec(actual, required),
{
    actual >= required
}

} // verus!
