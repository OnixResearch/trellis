//! Proof artifacts for policy composition and phase agreement.

use vstd::prelude::*;

use super::admission::*;
use super::agreement::*;
use super::model::*;
use super::registry::*;
use super::rules::*;

verus! {

// r[verify hir-attr.policy]
// r[verify hir-attr.admission]
pub proof fn admitted_occurrence_satisfies_policies(
    descriptor: Descriptor,
    occurrence: Occurrence,
    input_index: u32,
)
    requires occurrence_admission_spec(Some(descriptor), occurrence, input_index).reason
        == Reason::Accepted
    ensures
        descriptor_admission_spec(descriptor, input_index).reason == Reason::Accepted,
        occurrence.name_id == descriptor.name_id,
        occurrence.count > 0,
        arguments_admitted_spec(descriptor, occurrence),
        target_admitted_spec(descriptor, occurrence),
        duplicates_admitted_spec(descriptor, occurrence),
        safety_admitted_spec(descriptor, occurrence),
        stability_admitted_spec(descriptor, occurrence),
{}

// r[verify hir-attr.parser-classes]
pub proof fn valid_no_args_descriptor_is_admitted(
    name_id: u64,
    template_id: u64,
    allowed_targets: u16,
    input_index: u32,
)
    requires
        name_id > 0,
        template_id > 0,
        target_mask_valid_spec(allowed_targets),
    ensures descriptor_admission_spec(
        Descriptor {
            name_id,
            class: ParserClass::NoArgs,
            template_id,
            expected_arguments: ArgumentShape::Empty,
            allowed_targets,
            duplicate_rule: DuplicateRule::Reject,
            safety_rule: SafetyRule::Normal,
            stability_rule: StabilityRule::Stable,
        },
        input_index,
    ).reason == Reason::Accepted
{}

// r[verify hir-attr.early-late-agreement]
pub proof fn matching_early_late_values_agree(value: ParsedValue, input_index: u32)
    ensures compare_phases_spec(
        PhaseResult {
            phase: Phase::Early,
            decision: Decision { reason: Reason::Accepted, input_index },
            value: Some(value),
        },
        PhaseResult {
            phase: Phase::Late,
            decision: Decision { reason: Reason::Accepted, input_index },
            value: Some(value),
        },
    ) == Agreement::Agreed
{}

// r[verify hir-attr.early-late-agreement]
pub proof fn different_early_late_values_diverge(
    early_value: ParsedValue,
    late_value: ParsedValue,
    input_index: u32,
)
    requires !values_equal_spec(early_value, late_value)
    ensures compare_phases_spec(
        PhaseResult {
            phase: Phase::Early,
            decision: Decision { reason: Reason::Accepted, input_index },
            value: Some(early_value),
        },
        PhaseResult {
            phase: Phase::Late,
            decision: Decision { reason: Reason::Accepted, input_index },
            value: Some(late_value),
        },
    ) == Agreement::Diverged
{}

} // verus!
