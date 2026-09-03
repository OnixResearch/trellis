use super::*;

// r[verify track-caller.attribute-validity]
#[test]
fn valid_function_is_admitted() {
    assert_eq!(admit_function(tracked_function()).code, AdmissionCode::Accepted);
}

// r[verify track-caller.location-metadata]
// r[verify track-caller.direct-call]
#[test]
fn direct_calls_select_call_site_or_forwarded_metadata() {
    assert_eq!(select_location(untracked_context()), Some(call_site()));
    assert_eq!(select_location(tracked_context()), Some(forwarded()));

    let root_edge = DirectEdge {
        id: EDGE_ID,
        callee_tracked: true,
        context: untracked_context(),
        supplied: Some(call_site()),
    };
    assert_eq!(admit_direct(root_edge).code, AdmissionCode::Accepted);

    let nested_edge = DirectEdge {
        context: tracked_context(),
        supplied: Some(forwarded()),
        ..root_edge
    };
    assert_eq!(admit_direct(nested_edge).code, AdmissionCode::Accepted);
}

// r[verify track-caller.propagation]
#[test]
fn tracked_chain_forwards_the_root_identity() {
    let chain = vec![Some(call_site()), Some(forwarded()), Some(forwarded())];
    assert!(is_valid(call_site(), &chain));
    assert_eq!(output_location(call_site(), &chain), Some(forwarded()));

    let one_edge = vec![Some(call_site())];
    assert!(is_valid(call_site(), &one_edge));
    assert_eq!(output_location(call_site(), &one_edge), Some(call_site()));
}

// r[verify track-caller.reify-shim]
#[test]
fn matching_definition_site_shim_is_admitted() {
    let shim = ReifyShim {
        target_id: TARGET_ID,
        definition_site: definition_site(),
    };
    assert_eq!(admit_pointer(tracked_pointer(Some(shim))).code, AdmissionCode::Accepted);
}

// r[verify track-caller.trait-inheritance]
#[test]
fn trait_prototype_default_and_local_markers_are_inherited() {
    let prototype_declaration = TraitMethod {
        prototype_marker: MarkerState::Present,
        observed_effective: MarkerState::Present,
        ..trait_method(TraitBody::Prototype)
    };
    assert!(effective_trait_tracking(prototype_declaration));
    assert_eq!(admit_trait(prototype_declaration).code, AdmissionCode::Accepted);

    let inherited_prototype = TraitMethod {
        prototype_marker: MarkerState::Present,
        observed_effective: MarkerState::Present,
        ..trait_method(TraitBody::Override)
    };
    assert!(effective_trait_tracking(inherited_prototype));
    assert_eq!(admit_trait(inherited_prototype).code, AdmissionCode::Accepted);

    let default_body = TraitMethod {
        default_marker: MarkerState::Present,
        observed_effective: MarkerState::Present,
        ..trait_method(TraitBody::Default)
    };
    assert!(effective_trait_tracking(default_body));
    assert_eq!(admit_trait(default_body).code, AdmissionCode::Accepted);

    let inherited_default = TraitMethod {
        default_marker: MarkerState::Present,
        observed_effective: MarkerState::Present,
        ..trait_method(TraitBody::Override)
    };
    assert!(effective_trait_tracking(inherited_default));
    assert_eq!(admit_trait(inherited_default).code, AdmissionCode::Accepted);

    let local = TraitMethod {
        local_marker: MarkerState::Present,
        observed_effective: MarkerState::Present,
        ..trait_method(TraitBody::Override)
    };
    assert!(effective_trait_tracking(local));
    assert_eq!(admit_trait(local).code, AdmissionCode::Accepted);
}

// r[verify track-caller.const-codegen-agreement]
#[test]
fn matching_const_and_codegen_traces_are_admitted() {
    let entry = TraceEntry {
        edge_id: EDGE_ID,
        location: forwarded(),
    };
    let matching = vec![entry];
    assert!(traces_agree(&matching, &matching));
}

// r[verify track-caller.evidence]
// r[verify track-caller.non-claims]
#[test]
fn metadata_does_not_construct_a_runtime_location() {
    let selected = select_location(untracked_context());
    assert_eq!(selected, Some(call_site()));
    assert_ne!(selected, Some(definition_site()));
}
