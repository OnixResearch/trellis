use super::*;

const FUNCTION_ID: u64 = 11;
const EDGE_ID: u64 = 12;
const POINTER_ID: u64 = 13;
const TRAIT_ID: u64 = 14;
const TARGET_ID: u64 = 15;
const SITE_ID: u64 = 21;
const OTHER_SITE_ID: u64 = 22;
const OTHER_TARGET_ID: u64 = 23;

fn call_site() -> Location {
    Location {
        id: SITE_ID,
        origin: LocationOrigin::CallSite,
    }
}

fn forwarded() -> Location {
    Location {
        id: SITE_ID,
        origin: LocationOrigin::Forwarded,
    }
}

fn definition_site() -> Location {
    Location {
        id: SITE_ID,
        origin: LocationOrigin::DefinitionSite,
    }
}

fn tracked_function() -> FunctionMetadata {
    FunctionMetadata {
        id: FUNCTION_ID,
        convention: CallingConvention::Rust,
        kind: ItemKind::Function,
        naked: false,
        annotation: AnnotationSource::Local,
    }
}

fn untracked_context() -> CallContext {
    CallContext {
        propagates_location: false,
        call_site: Some(call_site()),
        forwarded: None,
    }
}

fn tracked_context() -> CallContext {
    CallContext {
        propagates_location: true,
        call_site: None,
        forwarded: Some(forwarded()),
    }
}

fn tracked_pointer(shim: Option<ReifyShim>) -> PointerUse {
    PointerUse {
        id: POINTER_ID,
        target_id: TARGET_ID,
        target_tracked: true,
        shim,
    }
}

fn trait_method(body: TraitBody) -> TraitMethod {
    TraitMethod {
        id: TRAIT_ID,
        body,
        prototype_marker: MarkerState::Absent,
        default_marker: MarkerState::Absent,
        local_marker: MarkerState::Absent,
        observed_effective: MarkerState::Absent,
    }
}

mod negative;
mod positive;
