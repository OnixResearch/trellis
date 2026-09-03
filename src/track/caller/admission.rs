//! Pure attribute, call-edge, shim, and trait admission.

use vstd::prelude::*;

use super::model::*;

verus! {

// r[impl track-caller.attribute-validity]
// r[impl track-caller.admission]
pub open spec fn function_admission_spec(metadata: FunctionMetadata) -> Admission {
    let accepted = Admission { code: AdmissionCode::Accepted, subject_id: metadata.id };
    match metadata.annotation {
        AnnotationSource::Absent => accepted,
        _ => match metadata.convention {
            CallingConvention::C | CallingConvention::System => Admission {
                code: AdmissionCode::InvalidConvention,
                subject_id: metadata.id,
            },
            CallingConvention::Rust => match metadata.kind {
                ItemKind::Closure => Admission {
                    code: AdmissionCode::ClosureTarget,
                    subject_id: metadata.id,
                },
                _ if metadata.naked => Admission {
                    code: AdmissionCode::NakedTarget,
                    subject_id: metadata.id,
                },
                ItemKind::Foreign => Admission {
                    code: AdmissionCode::UnsupportedItem,
                    subject_id: metadata.id,
                },
                _ => accepted,
            },
        },
    }
}

// r[impl track-caller.attribute-validity]
// r[impl track-caller.admission]
pub fn admit_function(metadata: FunctionMetadata) -> (result: Admission)
    ensures result == function_admission_spec(metadata)
{
    let accepted = Admission { code: AdmissionCode::Accepted, subject_id: metadata.id };
    match metadata.annotation {
        AnnotationSource::Absent => accepted,
        _ => match metadata.convention {
            CallingConvention::C | CallingConvention::System => Admission {
                code: AdmissionCode::InvalidConvention,
                subject_id: metadata.id,
            },
            CallingConvention::Rust => match metadata.kind {
                ItemKind::Closure => Admission {
                    code: AdmissionCode::ClosureTarget,
                    subject_id: metadata.id,
                },
                _ if metadata.naked => Admission {
                    code: AdmissionCode::NakedTarget,
                    subject_id: metadata.id,
                },
                ItemKind::Foreign => Admission {
                    code: AdmissionCode::UnsupportedItem,
                    subject_id: metadata.id,
                },
                _ => accepted,
            },
        },
    }
}

// r[impl track-caller.location-metadata]
// r[impl track-caller.direct-call]
pub open spec fn select_location_spec(context: CallContext) -> Option<Location> {
    if context.propagates_location {
        match context.forwarded {
            Some(location) => match location.origin {
                LocationOrigin::Forwarded => Some(location),
                LocationOrigin::DefinitionSite | LocationOrigin::CallSite => None,
            },
            None => None,
        }
    } else {
        match context.call_site {
            Some(location) => match location.origin {
                LocationOrigin::CallSite => Some(location),
                LocationOrigin::DefinitionSite | LocationOrigin::Forwarded => None,
            },
            None => None,
        }
    }
}

// r[impl track-caller.location-metadata]
// r[impl track-caller.direct-call]
pub fn select_location(context: CallContext) -> (result: Option<Location>)
    ensures result == select_location_spec(context)
{
    if context.propagates_location {
        match context.forwarded {
            Some(location) => match location.origin {
                LocationOrigin::Forwarded => Some(location),
                LocationOrigin::DefinitionSite | LocationOrigin::CallSite => None,
            },
            None => None,
        }
    } else {
        match context.call_site {
            Some(location) => match location.origin {
                LocationOrigin::CallSite => Some(location),
                LocationOrigin::DefinitionSite | LocationOrigin::Forwarded => None,
            },
            None => None,
        }
    }
}

// r[impl track-caller.direct-call]
// r[impl track-caller.admission]
pub open spec fn direct_admission_spec(edge: DirectEdge) -> Admission {
    if !edge.callee_tracked {
        match edge.supplied {
            None => Admission { code: AdmissionCode::Accepted, subject_id: edge.id },
            Some(_) => Admission { code: AdmissionCode::UnexpectedLocation, subject_id: edge.id },
        }
    } else {
        match (select_location_spec(edge.context), edge.supplied) {
            (None, _) | (_, None) => Admission { code: AdmissionCode::MissingLocation, subject_id: edge.id },
            (Some(expected), Some(actual)) if locations_equal_spec(actual, expected) => {
                Admission { code: AdmissionCode::Accepted, subject_id: edge.id }
            },
            _ => Admission { code: AdmissionCode::WrongLocation, subject_id: edge.id },
        }
    }
}

// r[impl track-caller.direct-call]
// r[impl track-caller.admission]
pub fn admit_direct(edge: DirectEdge) -> (result: Admission)
    ensures result == direct_admission_spec(edge)
{
    if !edge.callee_tracked {
        match edge.supplied {
            None => Admission { code: AdmissionCode::Accepted, subject_id: edge.id },
            Some(_) => Admission { code: AdmissionCode::UnexpectedLocation, subject_id: edge.id },
        }
    } else {
        match (select_location(edge.context), edge.supplied) {
            (None, _) | (_, None) => Admission { code: AdmissionCode::MissingLocation, subject_id: edge.id },
            (Some(expected), Some(actual)) if locations_equal(actual, expected) => {
                Admission { code: AdmissionCode::Accepted, subject_id: edge.id }
            },
            _ => Admission { code: AdmissionCode::WrongLocation, subject_id: edge.id },
        }
    }
}

// r[impl track-caller.reify-shim]
// r[impl track-caller.admission]
pub open spec fn pointer_admission_spec(pointer: PointerUse) -> Admission {
    match (pointer.target_tracked, pointer.shim) {
        (false, None) => Admission { code: AdmissionCode::Accepted, subject_id: pointer.id },
        (false, Some(_)) => Admission { code: AdmissionCode::WrongShim, subject_id: pointer.id },
        (true, None) => Admission { code: AdmissionCode::MissingShim, subject_id: pointer.id },
        (true, Some(shim)) => if shim.target_id == pointer.target_id
            && shim.definition_site.origin == LocationOrigin::DefinitionSite {
            Admission { code: AdmissionCode::Accepted, subject_id: pointer.id }
        } else {
            Admission { code: AdmissionCode::WrongShim, subject_id: pointer.id }
        },
    }
}

// r[impl track-caller.reify-shim]
// r[impl track-caller.admission]
pub fn admit_pointer(pointer: PointerUse) -> (result: Admission)
    ensures result == pointer_admission_spec(pointer)
{
    match (pointer.target_tracked, pointer.shim) {
        (false, None) => Admission { code: AdmissionCode::Accepted, subject_id: pointer.id },
        (false, Some(_)) => Admission { code: AdmissionCode::WrongShim, subject_id: pointer.id },
        (true, None) => Admission { code: AdmissionCode::MissingShim, subject_id: pointer.id },
        (true, Some(shim)) => if shim.target_id == pointer.target_id
            && matches!(shim.definition_site.origin, LocationOrigin::DefinitionSite) {
            Admission { code: AdmissionCode::Accepted, subject_id: pointer.id }
        } else {
            Admission { code: AdmissionCode::WrongShim, subject_id: pointer.id }
        },
    }
}

// r[impl track-caller.trait-inheritance]
pub open spec fn effective_trait_tracking_spec(method: TraitMethod) -> bool {
    match method.body {
        TraitBody::Prototype => marker_present_spec(method.prototype_marker),
        TraitBody::Default => marker_present_spec(method.prototype_marker)
            || marker_present_spec(method.default_marker),
        TraitBody::Override => marker_present_spec(method.prototype_marker)
            || marker_present_spec(method.default_marker)
            || marker_present_spec(method.local_marker),
    }
}

// r[impl track-caller.trait-inheritance]
pub fn effective_trait_tracking(method: TraitMethod) -> (result: bool)
    ensures result == effective_trait_tracking_spec(method)
{
    match method.body {
        TraitBody::Prototype => marker_present(method.prototype_marker),
        TraitBody::Default => {
            marker_present(method.prototype_marker) || marker_present(method.default_marker)
        },
        TraitBody::Override => {
            marker_present(method.prototype_marker)
                || marker_present(method.default_marker)
                || marker_present(method.local_marker)
        },
    }
}

// r[impl track-caller.trait-inheritance]
// r[impl track-caller.admission]
pub fn admit_trait(method: TraitMethod) -> (result: Admission)
    ensures
        result.code == if effective_trait_tracking_spec(method)
            == marker_present_spec(method.observed_effective) {
            AdmissionCode::Accepted
        } else {
            AdmissionCode::InheritanceMismatch
        },
        result.subject_id == method.id,
{
    let code = if effective_trait_tracking(method) == marker_present(method.observed_effective) {
        AdmissionCode::Accepted
    } else {
        AdmissionCode::InheritanceMismatch
    };
    Admission { code, subject_id: method.id }
}

} // verus!
