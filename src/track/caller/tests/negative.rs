use super::*;

// r[verify track-caller.attribute-validity]
// r[verify track-caller.admission]
#[test]
fn failure_priority_is_deterministic() {
    let malformed = FunctionMetadata {
        convention: CallingConvention::C,
        kind: ItemKind::Closure,
        naked: true,
        ..tracked_function()
    };
    let first = admit_function(malformed);
    let second = admit_function(malformed);
    assert_eq!(first, second);
    assert_eq!(first.code, AdmissionCode::InvalidConvention);
    assert_eq!(first.subject_id, FUNCTION_ID);
}

// r[verify track-caller.attribute-validity]
#[test]
fn invalid_attribute_targets_are_rejected() {
    let closure = FunctionMetadata {
        kind: ItemKind::Closure,
        ..tracked_function()
    };
    assert_eq!(admit_function(closure).code, AdmissionCode::ClosureTarget);

    let naked = FunctionMetadata {
        naked: true,
        ..tracked_function()
    };
    assert_eq!(admit_function(naked).code, AdmissionCode::NakedTarget);

    let foreign = FunctionMetadata {
        kind: ItemKind::Foreign,
        ..tracked_function()
    };
    assert_eq!(admit_function(foreign).code, AdmissionCode::UnsupportedItem);

    let untracked = FunctionMetadata {
        convention: CallingConvention::System,
        kind: ItemKind::Foreign,
        naked: true,
        annotation: AnnotationSource::Absent,
        ..tracked_function()
    };
    assert_eq!(admit_function(untracked).code, AdmissionCode::Accepted);
}

// r[verify track-caller.direct-call]
// r[verify track-caller.admission]
#[test]
fn direct_call_rejections_are_typed() {
    let missing = DirectEdge {
        id: EDGE_ID,
        callee_tracked: true,
        context: tracked_context(),
        supplied: None,
    };
    assert_eq!(admit_direct(missing).code, AdmissionCode::MissingLocation);

    let wrong = DirectEdge {
        supplied: Some(Location {
            id: OTHER_SITE_ID,
            origin: LocationOrigin::Forwarded,
        }),
        ..missing
    };
    assert_eq!(admit_direct(wrong).code, AdmissionCode::WrongLocation);

    let unexpected = DirectEdge {
        callee_tracked: false,
        supplied: Some(call_site()),
        ..missing
    };
    assert_eq!(admit_direct(unexpected).code, AdmissionCode::UnexpectedLocation);
}

// r[verify track-caller.propagation]
// r[verify track-caller.admission]
#[test]
fn malformed_or_excessive_chains_are_rejected() {
    let missing = vec![Some(call_site()), None];
    assert!(!is_valid(call_site(), &missing));

    let wrong = vec![
        Some(call_site()),
        Some(Location {
            id: OTHER_SITE_ID,
            origin: LocationOrigin::Forwarded,
        }),
    ];
    assert!(!is_valid(call_site(), &wrong));
    assert!(!is_valid(call_site(), &Vec::new()));

    let excessive = vec![Some(call_site()); MAX_METADATA_ENTRIES + 1];
    assert!(!is_valid(call_site(), &excessive));
}

// r[verify track-caller.reify-shim]
#[test]
fn missing_or_wrong_shims_are_rejected() {
    let shim = ReifyShim {
        target_id: TARGET_ID,
        definition_site: definition_site(),
    };
    assert_eq!(admit_pointer(tracked_pointer(None)).code, AdmissionCode::MissingShim);

    let wrong_target = ReifyShim {
        target_id: OTHER_TARGET_ID,
        ..shim
    };
    assert_eq!(admit_pointer(tracked_pointer(Some(wrong_target))).code, AdmissionCode::WrongShim);

    let wrong_origin = ReifyShim {
        definition_site: call_site(),
        ..shim
    };
    assert_eq!(admit_pointer(tracked_pointer(Some(wrong_origin))).code, AdmissionCode::WrongShim);

    let untracked = PointerUse {
        target_tracked: false,
        shim: Some(shim),
        ..tracked_pointer(None)
    };
    assert_eq!(admit_pointer(untracked).code, AdmissionCode::WrongShim);
}

// r[verify track-caller.trait-inheritance]
// r[verify track-caller.admission]
#[test]
fn missing_inherited_tracking_is_rejected() {
    let method = TraitMethod {
        prototype_marker: MarkerState::Present,
        observed_effective: MarkerState::Absent,
        ..trait_method(TraitBody::Override)
    };
    assert_eq!(admit_trait(method).code, AdmissionCode::InheritanceMismatch);
}

// r[verify track-caller.const-codegen-agreement]
#[test]
fn divergent_const_and_codegen_traces_are_rejected() {
    let entry = TraceEntry {
        edge_id: EDGE_ID,
        location: forwarded(),
    };
    let matching = vec![entry];

    let wrong_location = vec![TraceEntry {
        location: Location {
            id: OTHER_SITE_ID,
            origin: LocationOrigin::Forwarded,
        },
        ..entry
    }];
    assert!(!traces_agree(&matching, &wrong_location));

    let wrong_edge = vec![TraceEntry {
        edge_id: POINTER_ID,
        ..entry
    }];
    assert!(!traces_agree(&matching, &wrong_edge));
    assert!(!traces_agree(&matching, &Vec::new()));

    let excessive = vec![entry; MAX_METADATA_ENTRIES + 1];
    assert!(!traces_agree(&excessive, &excessive));
}
