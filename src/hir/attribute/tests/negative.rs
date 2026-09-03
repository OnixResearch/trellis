use super::*;

// r[verify hir-attr.parser-envelope]
// r[verify hir-attr.admission]
#[test]
fn incomplete_descriptors_have_stable_first_failures() {
    let valid = descriptor(ParserClass::Single);
    let missing_name = Descriptor {
        name_id: 0,
        template_id: 0,
        allowed_targets: 0,
        ..valid
    };
    let first = admit_descriptor(missing_name, INPUT_INDEX);
    let second = admit_descriptor(missing_name, INPUT_INDEX);
    assert_eq!(first, second);
    assert_eq!(first.reason, Reason::MissingName);
    assert_eq!(first.input_index, INPUT_INDEX);

    let missing_template = Descriptor {
        template_id: 0,
        ..valid
    };
    assert_eq!(admit_descriptor(missing_template, INPUT_INDEX).reason, Reason::MissingTemplate);

    let invalid_mask = Descriptor {
        allowed_targets: 0,
        ..valid
    };
    assert_eq!(admit_descriptor(invalid_mask, INPUT_INDEX).reason, Reason::InvalidTargetMask);

    let malformed = Descriptor {
        expected_arguments: ArgumentShape::Malformed,
        ..valid
    };
    assert_eq!(admit_descriptor(malformed, INPUT_INDEX).reason, Reason::MalformedTemplate);

    let invalid_class = Descriptor {
        class: ParserClass::NoArgs,
        expected_arguments: ArgumentShape::Word,
        duplicate_rule: DuplicateRule::KeepFirst,
        ..valid
    };
    assert_eq!(admit_descriptor(invalid_class, INPUT_INDEX).reason, Reason::InvalidClassPolicy);
}

// r[verify hir-attr.admission]
#[test]
fn unknown_parser_and_name_mismatch_are_rejected() {
    let item = occurrence(ArgumentShape::Word);
    assert_eq!(admit_occurrence(None, item, INPUT_INDEX).reason, Reason::UnknownParser);

    let wrong_name = Occurrence {
        name_id: OTHER_NAME_ID,
        count: 0,
        arguments: ArgumentShape::Malformed,
        ..item
    };
    let result = admit_occurrence(Some(descriptor(ParserClass::Single)), wrong_name, INPUT_INDEX);
    assert_eq!(result.reason, Reason::NameMismatch);
}

// r[verify hir-attr.parser-classes]
// r[verify hir-attr.admission]
#[test]
fn malformed_wrong_and_duplicate_arguments_are_rejected() {
    let parser = descriptor(ParserClass::Single);
    let missing = Occurrence {
        count: 0,
        ..occurrence(ArgumentShape::Word)
    };
    assert_eq!(admit_occurrence(Some(parser), missing, INPUT_INDEX).reason, Reason::MissingOccurrence);

    let malformed = occurrence(ArgumentShape::Malformed);
    assert_eq!(admit_occurrence(Some(parser), malformed, INPUT_INDEX).reason, Reason::MalformedArguments);

    let wrong = occurrence(ArgumentShape::List);
    assert_eq!(admit_occurrence(Some(parser), wrong, INPUT_INDEX).reason, Reason::WrongArguments);

    let duplicate = Occurrence {
        count: DUPLICATE_COUNT,
        ..occurrence(ArgumentShape::Word)
    };
    assert_eq!(admit_occurrence(Some(parser), duplicate, INPUT_INDEX).reason, Reason::Duplicate);
}

// r[verify hir-attr.policy]
#[test]
fn bad_target_and_missing_safety_wrapper_are_rejected() {
    let parser = descriptor(ParserClass::Single);
    let bad_target = Occurrence {
        target: Target::Crate,
        ..occurrence(ArgumentShape::Word)
    };
    assert_eq!(admit_occurrence(Some(parser), bad_target, INPUT_INDEX).reason, Reason::BadTarget);

    let unsafe_parser = Descriptor {
        safety_rule: SafetyRule::UnsafeWrapper,
        ..parser
    };
    assert_eq!(
        admit_occurrence(Some(unsafe_parser), occurrence(ArgumentShape::Word), INPUT_INDEX).reason,
        Reason::MissingUnsafeWrapper
    );
}

// r[verify hir-attr.policy]
#[test]
fn unstable_parser_requires_the_exact_gate() {
    let parser = Descriptor {
        stability_rule: StabilityRule::FeatureGate { gate_id: FEATURE_ID },
        ..descriptor(ParserClass::Single)
    };
    let missing = occurrence(ArgumentShape::Word);
    assert_eq!(admit_occurrence(Some(parser), missing, INPUT_INDEX).reason, Reason::MissingFeatureGate);

    let wrong = Occurrence {
        feature_gate: Some(OTHER_FEATURE_ID),
        ..missing
    };
    assert_eq!(admit_occurrence(Some(parser), wrong, INPUT_INDEX).reason, Reason::WrongFeatureGate);
}

fn successful_result(phase: Phase, value: ParsedValue) -> PhaseResult {
    PhaseResult {
        phase,
        decision: Decision {
            reason: Reason::Accepted,
            input_index: INPUT_INDEX,
        },
        value: Some(value),
    }
}

// r[verify hir-attr.early-late-agreement]
#[test]
fn phase_disagreement_and_incomplete_results_fail_closed() {
    let parser = descriptor(ParserClass::Single);
    let value = projected_value(parser, occurrence(ArgumentShape::Word));
    let changed = ParsedValue {
        argument_value: OTHER_ARGUMENT_VALUE,
        ..value
    };
    assert_eq!(
        compare_phases(successful_result(Phase::Early, value), successful_result(Phase::Late, changed)),
        Agreement::Diverged
    );
    assert_eq!(
        compare_phases(successful_result(Phase::Late, value), successful_result(Phase::Early, value)),
        Agreement::Diverged
    );

    let failed = PhaseResult {
        phase: Phase::Early,
        decision: Decision {
            reason: Reason::MalformedArguments,
            input_index: INPUT_INDEX,
        },
        value: None,
    };
    assert_eq!(compare_phases(failed, successful_result(Phase::Late, value)), Agreement::NotComparable);
}
