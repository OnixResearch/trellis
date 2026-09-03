//! Independent finite checks used by occurrence admission.

use vstd::prelude::*;

use super::model::*;
use super::registry::*;

verus! {

// r[impl hir-attr.parser-classes]
pub open spec fn shapes_equal_spec(left: ArgumentShape, right: ArgumentShape) -> bool {
    match (left, right) {
        (ArgumentShape::Empty, ArgumentShape::Empty)
        | (ArgumentShape::Word, ArgumentShape::Word)
        | (ArgumentShape::List, ArgumentShape::List)
        | (ArgumentShape::NameValue, ArgumentShape::NameValue)
        | (ArgumentShape::Malformed, ArgumentShape::Malformed) => true,
        _ => false,
    }
}

// r[impl hir-attr.parser-classes]
pub fn shapes_equal(left: ArgumentShape, right: ArgumentShape) -> (result: bool)
    ensures result == shapes_equal_spec(left, right)
{
    matches!(
        (left, right),
        (ArgumentShape::Empty, ArgumentShape::Empty)
            | (ArgumentShape::Word, ArgumentShape::Word)
            | (ArgumentShape::List, ArgumentShape::List)
            | (ArgumentShape::NameValue, ArgumentShape::NameValue)
            | (ArgumentShape::Malformed, ArgumentShape::Malformed)
    )
}

// r[impl hir-attr.admission]
pub open spec fn decision_is_accepted_spec(decision: Decision) -> bool {
    match decision.reason {
        Reason::Accepted => true,
        _ => false,
    }
}

// r[impl hir-attr.admission]
pub fn decision_is_accepted(decision: Decision) -> (result: bool)
    ensures result == decision_is_accepted_spec(decision)
{
    matches!(decision.reason, Reason::Accepted)
}

// r[impl hir-attr.policy]
pub open spec fn target_admitted_spec(descriptor: Descriptor, occurrence: Occurrence) -> bool {
    descriptor.allowed_targets & target_bit_spec(occurrence.target) != 0
}

// r[impl hir-attr.policy]
pub fn target_admitted(descriptor: Descriptor, occurrence: Occurrence) -> (result: bool)
    ensures result == target_admitted_spec(descriptor, occurrence)
{
    descriptor.allowed_targets & target_bit(occurrence.target) != 0
}

// r[impl hir-attr.parser-classes]
pub open spec fn arguments_admitted_spec(
    descriptor: Descriptor,
    occurrence: Occurrence,
) -> bool {
    match descriptor.class {
        ParserClass::NoArgs => occurrence.arguments == ArgumentShape::Empty,
        ParserClass::Single | ParserClass::Combine | ParserClass::Generic => {
            shapes_equal_spec(occurrence.arguments, descriptor.expected_arguments)
        },
    }
}

// r[impl hir-attr.parser-classes]
pub fn arguments_admitted(descriptor: Descriptor, occurrence: Occurrence) -> (result: bool)
    ensures result == arguments_admitted_spec(descriptor, occurrence)
{
    match descriptor.class {
        ParserClass::NoArgs => matches!(occurrence.arguments, ArgumentShape::Empty),
        ParserClass::Single | ParserClass::Combine | ParserClass::Generic => {
            shapes_equal(occurrence.arguments, descriptor.expected_arguments)
        },
    }
}

// r[impl hir-attr.policy]
pub open spec fn duplicates_admitted_spec(
    descriptor: Descriptor,
    occurrence: Occurrence,
) -> bool {
    occurrence.count > 0 && (occurrence.count == 1
        || match descriptor.duplicate_rule {
            DuplicateRule::Reject => false,
            DuplicateRule::KeepFirst | DuplicateRule::KeepLast | DuplicateRule::Combine => true,
        })
}

// r[impl hir-attr.policy]
pub fn duplicates_admitted(descriptor: Descriptor, occurrence: Occurrence) -> (result: bool)
    ensures result == duplicates_admitted_spec(descriptor, occurrence)
{
    occurrence.count > 0 && (occurrence.count == 1
        || match descriptor.duplicate_rule {
            DuplicateRule::Reject => false,
            DuplicateRule::KeepFirst | DuplicateRule::KeepLast | DuplicateRule::Combine => true,
        })
}

// r[impl hir-attr.policy]
pub open spec fn safety_admitted_spec(descriptor: Descriptor, occurrence: Occurrence) -> bool {
    match descriptor.safety_rule {
        SafetyRule::Normal => true,
        SafetyRule::UnsafeWrapper => occurrence.wrapper == WrapperState::Unsafe,
    }
}

// r[impl hir-attr.policy]
pub fn safety_admitted(descriptor: Descriptor, occurrence: Occurrence) -> (result: bool)
    ensures result == safety_admitted_spec(descriptor, occurrence)
{
    match descriptor.safety_rule {
        SafetyRule::Normal => true,
        SafetyRule::UnsafeWrapper => matches!(occurrence.wrapper, WrapperState::Unsafe),
    }
}

// r[impl hir-attr.policy]
pub open spec fn stability_admitted_spec(
    descriptor: Descriptor,
    occurrence: Occurrence,
) -> bool {
    match descriptor.stability_rule {
        StabilityRule::Stable => true,
        StabilityRule::FeatureGate { gate_id } => occurrence.feature_gate == Some(gate_id),
    }
}

// r[impl hir-attr.policy]
pub fn stability_admitted(descriptor: Descriptor, occurrence: Occurrence) -> (result: bool)
    ensures result == stability_admitted_spec(descriptor, occurrence)
{
    match descriptor.stability_rule {
        StabilityRule::Stable => true,
        StabilityRule::FeatureGate { gate_id } => match occurrence.feature_gate {
            Some(observed) => observed == gate_id,
            None => false,
        },
    }
}

} // verus!
