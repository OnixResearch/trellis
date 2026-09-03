//! Deterministic occurrence admission and parsed metadata projection.

use vstd::prelude::*;

use super::model::*;
use super::registry::*;
use super::rules::*;

verus! {

// r[impl hir-attr.admission]
pub open spec fn occurrence_admission_spec(
    descriptor: Option<Descriptor>,
    occurrence: Occurrence,
    input_index: u32,
) -> Decision {
    let reason = match descriptor {
        None => Reason::UnknownParser,
        Some(parser) => {
            if !decision_is_accepted_spec(descriptor_admission_spec(parser, input_index)) {
                Reason::DescriptorRejected
            } else if occurrence.name_id != parser.name_id {
                Reason::NameMismatch
            } else if occurrence.count == 0 {
                Reason::MissingOccurrence
            } else if occurrence.arguments == ArgumentShape::Malformed {
                Reason::MalformedArguments
            } else if !arguments_admitted_spec(parser, occurrence) {
                Reason::WrongArguments
            } else if !target_admitted_spec(parser, occurrence) {
                Reason::BadTarget
            } else if !duplicates_admitted_spec(parser, occurrence) {
                Reason::Duplicate
            } else if !safety_admitted_spec(parser, occurrence) {
                Reason::MissingUnsafeWrapper
            } else {
                match parser.stability_rule {
                    StabilityRule::Stable => Reason::Accepted,
                    StabilityRule::FeatureGate { gate_id } => match occurrence.feature_gate {
                        None => Reason::MissingFeatureGate,
                        Some(observed) => {
                            if observed == gate_id { Reason::Accepted } else { Reason::WrongFeatureGate }
                        },
                    },
                }
            }
        },
    };
    Decision { reason, input_index }
}

// r[impl hir-attr.admission]
pub fn admit_occurrence(
    descriptor: Option<Descriptor>,
    occurrence: Occurrence,
    input_index: u32,
) -> (result: Decision)
    ensures result == occurrence_admission_spec(descriptor, occurrence, input_index)
{
    let parser = match descriptor {
        None => return Decision { reason: Reason::UnknownParser, input_index },
        Some(parser) => parser,
    };
    if !decision_is_accepted(admit_descriptor(parser, input_index)) {
        return Decision { reason: Reason::DescriptorRejected, input_index };
    }
    if occurrence.name_id != parser.name_id {
        return Decision { reason: Reason::NameMismatch, input_index };
    }
    if occurrence.count == 0 {
        return Decision { reason: Reason::MissingOccurrence, input_index };
    }
    if matches!(occurrence.arguments, ArgumentShape::Malformed) {
        return Decision { reason: Reason::MalformedArguments, input_index };
    }
    if !arguments_admitted(parser, occurrence) {
        return Decision { reason: Reason::WrongArguments, input_index };
    }
    if !target_admitted(parser, occurrence) {
        return Decision { reason: Reason::BadTarget, input_index };
    }
    if !duplicates_admitted(parser, occurrence) {
        return Decision { reason: Reason::Duplicate, input_index };
    }
    if !safety_admitted(parser, occurrence) {
        return Decision { reason: Reason::MissingUnsafeWrapper, input_index };
    }
    let reason = match parser.stability_rule {
        StabilityRule::Stable => Reason::Accepted,
        StabilityRule::FeatureGate { gate_id } => match occurrence.feature_gate {
            None => Reason::MissingFeatureGate,
            Some(observed) => {
                if observed == gate_id { Reason::Accepted } else { Reason::WrongFeatureGate }
            },
        },
    };
    Decision { reason, input_index }
}

// r[impl hir-attr.parser-classes]
pub open spec fn selected_occurrence_spec(descriptor: Descriptor, occurrence: Occurrence) -> u32 {
    if occurrence.count == 0 {
        0
    } else {
        match descriptor.duplicate_rule {
            DuplicateRule::KeepLast => (occurrence.count - 1) as u32,
            DuplicateRule::Reject | DuplicateRule::KeepFirst | DuplicateRule::Combine => 0,
        }
    }
}

// r[impl hir-attr.parser-classes]
pub fn selected_occurrence(descriptor: Descriptor, occurrence: Occurrence) -> (result: u32)
    ensures result == selected_occurrence_spec(descriptor, occurrence)
{
    if occurrence.count == 0 {
        0
    } else {
        match descriptor.duplicate_rule {
            DuplicateRule::KeepLast => occurrence.count - 1,
            DuplicateRule::Reject | DuplicateRule::KeepFirst | DuplicateRule::Combine => 0,
        }
    }
}

// r[impl hir-attr.parser-classes]
pub open spec fn projected_value_spec(
    descriptor: Descriptor,
    occurrence: Occurrence,
) -> ParsedValue {
    ParsedValue {
        name_id: occurrence.name_id,
        arguments: occurrence.arguments,
        argument_value: occurrence.argument_value,
        combined_occurrences: occurrence.count,
        selected_occurrence: selected_occurrence_spec(descriptor, occurrence),
    }
}

// r[impl hir-attr.parser-classes]
pub fn projected_value(descriptor: Descriptor, occurrence: Occurrence) -> (result: ParsedValue)
    ensures result == projected_value_spec(descriptor, occurrence)
{
    ParsedValue {
        name_id: occurrence.name_id,
        arguments: occurrence.arguments,
        argument_value: occurrence.argument_value,
        combined_occurrences: occurrence.count,
        selected_occurrence: selected_occurrence(descriptor, occurrence),
    }
}

// r[impl hir-attr.admission]
// r[impl hir-attr.early-late-agreement]
pub open spec fn evaluate_spec(request: Request) -> PhaseResult {
    let decision = occurrence_admission_spec(
        request.descriptor,
        request.occurrence,
        request.input_index,
    );
    let value = match request.descriptor {
        Some(descriptor) => {
            if decision_is_accepted_spec(decision) {
                Some(projected_value_spec(descriptor, request.occurrence))
            } else {
                None
            }
        },
        None => None,
    };
    PhaseResult { phase: request.phase, decision, value }
}

// r[impl hir-attr.admission]
// r[impl hir-attr.early-late-agreement]
pub fn evaluate(request: Request) -> (result: PhaseResult)
    ensures result == evaluate_spec(request)
{
    let decision = admit_occurrence(
        request.descriptor,
        request.occurrence,
        request.input_index,
    );
    let value = match request.descriptor {
        Some(descriptor) => {
            if decision_is_accepted(decision) {
                Some(projected_value(descriptor, request.occurrence))
            } else {
                None
            }
        },
        None => None,
    };
    PhaseResult { phase: request.phase, decision, value }
}

} // verus!
