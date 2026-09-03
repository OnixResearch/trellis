use super::*;

// r[verify hir-attr.parser-envelope]
// r[verify hir-attr.parser-classes]
#[test]
fn all_parser_classes_have_valid_descriptors() {
    for class in [
        ParserClass::NoArgs,
        ParserClass::Single,
        ParserClass::Combine,
        ParserClass::Generic,
    ] {
        assert_eq!(admit_descriptor(descriptor(class), INPUT_INDEX).reason, Reason::Accepted);
    }
}

// r[verify hir-attr.parser-classes]
// r[verify hir-attr.admission]
#[test]
fn no_args_and_single_occurrences_are_admitted() {
    let no_args = descriptor(ParserClass::NoArgs);
    let no_args_occurrence = occurrence(ArgumentShape::Empty);
    assert_eq!(admit_occurrence(Some(no_args), no_args_occurrence, INPUT_INDEX).reason, Reason::Accepted);

    let single = descriptor(ParserClass::Single);
    let single_occurrence = occurrence(ArgumentShape::Word);
    assert_eq!(admit_occurrence(Some(single), single_occurrence, INPUT_INDEX).reason, Reason::Accepted);
}

// r[verify hir-attr.parser-classes]
#[test]
fn replacement_and_combine_policies_project_deterministically() {
    let keep_first = Descriptor {
        duplicate_rule: DuplicateRule::KeepFirst,
        ..descriptor(ParserClass::Single)
    };
    let duplicated = Occurrence {
        count: DUPLICATE_COUNT,
        ..occurrence(ArgumentShape::Word)
    };
    assert_eq!(admit_occurrence(Some(keep_first), duplicated, INPUT_INDEX).reason, Reason::Accepted);
    assert_eq!(projected_value(keep_first, duplicated).selected_occurrence, 0);

    let keep_last = Descriptor {
        duplicate_rule: DuplicateRule::KeepLast,
        ..keep_first
    };
    assert_eq!(projected_value(keep_last, duplicated).selected_occurrence, DUPLICATE_COUNT - 1);

    let combine = descriptor(ParserClass::Combine);
    let combined = projected_value(combine, duplicated);
    assert_eq!(admit_occurrence(Some(combine), duplicated, INPUT_INDEX).reason, Reason::Accepted);
    assert_eq!(combined.combined_occurrences, DUPLICATE_COUNT);
}

// r[verify hir-attr.policy]
#[test]
fn target_safety_and_stability_policies_accept_declared_facts() {
    let unsafe_descriptor = Descriptor {
        safety_rule: SafetyRule::UnsafeWrapper,
        stability_rule: StabilityRule::FeatureGate { gate_id: FEATURE_ID },
        ..descriptor(ParserClass::Single)
    };
    let admitted = Occurrence {
        target: Target::Method,
        wrapper: WrapperState::Unsafe,
        feature_gate: Some(FEATURE_ID),
        ..occurrence(ArgumentShape::Word)
    };
    assert!(target_admitted(unsafe_descriptor, admitted));
    assert!(safety_admitted(unsafe_descriptor, admitted));
    assert!(stability_admitted(unsafe_descriptor, admitted));
    assert_eq!(admit_occurrence(Some(unsafe_descriptor), admitted, INPUT_INDEX).reason, Reason::Accepted);
}

// r[verify hir-attr.early-late-agreement]
#[test]
fn matching_early_and_late_results_agree() {
    let parser = descriptor(ParserClass::Generic);
    let item = occurrence(ArgumentShape::Word);
    let early = evaluate(request(parser, item, Phase::Early));
    let late = evaluate(request(parser, item, Phase::Late));
    assert_eq!(compare_phases(early, late), Agreement::Agreed);
}

// r[verify hir-attr.evidence]
// r[verify hir-attr.non-claims]
#[test]
fn parsed_metadata_contains_only_finite_declared_facts() {
    let parser = descriptor(ParserClass::Single);
    let result = evaluate(request(parser, occurrence(ArgumentShape::Word), Phase::Late));
    match result.value {
        Some(value) => {
            assert_eq!(value.name_id, NAME_ID);
            assert_eq!(value.argument_value, ARGUMENT_VALUE);
            assert_eq!(value.combined_occurrences, 1);
        }
        None => panic!("admitted finite metadata must be present"),
    }
}
