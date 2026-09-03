//! Descriptor and target-policy admission for a finite parser registry.

use vstd::prelude::*;

use super::model::*;

verus! {

// r[impl hir-attr.policy]
pub open spec fn target_bit_spec(target: Target) -> u16 {
    match target {
        Target::Crate => TARGET_CRATE_BIT,
        Target::Function => TARGET_FUNCTION_BIT,
        Target::Static => TARGET_STATIC_BIT,
        Target::Method => TARGET_METHOD_BIT,
        Target::Module => TARGET_MODULE_BIT,
        Target::Field => TARGET_FIELD_BIT,
        Target::Statement => TARGET_STATEMENT_BIT,
        Target::Expression => TARGET_EXPRESSION_BIT,
    }
}

// r[impl hir-attr.policy]
pub fn target_bit(target: Target) -> (result: u16)
    ensures result == target_bit_spec(target)
{
    match target {
        Target::Crate => TARGET_CRATE_BIT,
        Target::Function => TARGET_FUNCTION_BIT,
        Target::Static => TARGET_STATIC_BIT,
        Target::Method => TARGET_METHOD_BIT,
        Target::Module => TARGET_MODULE_BIT,
        Target::Field => TARGET_FIELD_BIT,
        Target::Statement => TARGET_STATEMENT_BIT,
        Target::Expression => TARGET_EXPRESSION_BIT,
    }
}

// r[impl hir-attr.policy]
pub open spec fn target_mask_valid_spec(mask: u16) -> bool {
    mask != 0 && mask & !TARGET_ALL_MASK == 0
}

// r[impl hir-attr.policy]
pub fn target_mask_valid(mask: u16) -> (result: bool)
    ensures result == target_mask_valid_spec(mask)
{
    mask != 0 && mask & !TARGET_ALL_MASK == 0
}

// r[impl hir-attr.parser-classes]
pub open spec fn class_policy_valid_spec(descriptor: Descriptor) -> bool {
    match descriptor.class {
        ParserClass::NoArgs => {
            descriptor.expected_arguments == ArgumentShape::Empty
                && descriptor.duplicate_rule == DuplicateRule::Reject
        },
        ParserClass::Single => {
            descriptor.duplicate_rule == DuplicateRule::Reject
                || descriptor.duplicate_rule == DuplicateRule::KeepFirst
                || descriptor.duplicate_rule == DuplicateRule::KeepLast
        },
        ParserClass::Combine => descriptor.duplicate_rule == DuplicateRule::Combine,
        ParserClass::Generic => true,
    }
}

// r[impl hir-attr.parser-classes]
pub fn class_policy_valid(descriptor: Descriptor) -> (result: bool)
    ensures result == class_policy_valid_spec(descriptor)
{
    match descriptor.class {
        ParserClass::NoArgs => {
            matches!(descriptor.expected_arguments, ArgumentShape::Empty)
                && matches!(descriptor.duplicate_rule, DuplicateRule::Reject)
        },
        ParserClass::Single => matches!(
            descriptor.duplicate_rule,
            DuplicateRule::Reject | DuplicateRule::KeepFirst | DuplicateRule::KeepLast
        ),
        ParserClass::Combine => matches!(descriptor.duplicate_rule, DuplicateRule::Combine),
        ParserClass::Generic => true,
    }
}

// r[impl hir-attr.parser-envelope]
// r[impl hir-attr.admission]
pub open spec fn descriptor_admission_spec(descriptor: Descriptor, input_index: u32) -> Decision {
    let reason = if descriptor.name_id == 0 {
        Reason::MissingName
    } else if descriptor.template_id == 0 {
        Reason::MissingTemplate
    } else if !target_mask_valid_spec(descriptor.allowed_targets) {
        Reason::InvalidTargetMask
    } else if descriptor.expected_arguments == ArgumentShape::Malformed {
        Reason::MalformedTemplate
    } else if !class_policy_valid_spec(descriptor) {
        Reason::InvalidClassPolicy
    } else {
        Reason::Accepted
    };
    Decision { reason, input_index }
}

// r[impl hir-attr.parser-envelope]
// r[impl hir-attr.admission]
pub fn admit_descriptor(descriptor: Descriptor, input_index: u32) -> (result: Decision)
    ensures result == descriptor_admission_spec(descriptor, input_index)
{
    if descriptor.name_id == 0 {
        return Decision { reason: Reason::MissingName, input_index };
    }
    if descriptor.template_id == 0 {
        return Decision { reason: Reason::MissingTemplate, input_index };
    }
    if !target_mask_valid(descriptor.allowed_targets) {
        return Decision { reason: Reason::InvalidTargetMask, input_index };
    }
    if matches!(descriptor.expected_arguments, ArgumentShape::Malformed) {
        return Decision { reason: Reason::MalformedTemplate, input_index };
    }
    if !class_policy_valid(descriptor) {
        return Decision { reason: Reason::InvalidClassPolicy, input_index };
    }
    Decision { reason: Reason::Accepted, input_index }
}

} // verus!
