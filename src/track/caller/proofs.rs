//! Proof artifacts for caller-location propagation and shim admission.

use vstd::prelude::*;

use super::admission::*;
use super::chain::*;
use super::model::*;
use super::trace::*;

verus! {

// r[verify track-caller.direct-call]
pub proof fn untracked_direct_uses_call_site(location: Location)
    requires location.origin == LocationOrigin::CallSite
    ensures select_location_spec(CallContext {
        propagates_location: false,
        call_site: Some(location),
        forwarded: None,
    }) == Some(location)
{}

// r[verify track-caller.propagation]
pub proof fn valid_chain_preserves_nearest_boundary(
    root: Location,
    entries: Seq<Option<Location>>,
)
    requires chain_is_valid_spec(root, entries)
    ensures exists|location: Location| chain_output_spec(root, entries) == Some(location)
        && location.id == root.id
{
    if entries.len() == 1 {
        assert(chain_output_spec(root, entries) == Some(root));
    } else {
        let forwarded = forwarded_location_spec(root);
        assert(forwarded.id == root.id);
        assert(chain_output_spec(root, entries) == Some(forwarded));
    }
}

// r[verify track-caller.reify-shim]
pub proof fn admitted_tracked_pointer_has_matching_shim(pointer: PointerUse)
    requires
        pointer.target_tracked,
        pointer_admission_spec(pointer).code == AdmissionCode::Accepted,
    ensures exists|shim: ReifyShim| pointer.shim == Some(shim)
        && shim.target_id == pointer.target_id
        && shim.definition_site.origin == LocationOrigin::DefinitionSite
{}

// r[verify track-caller.trait-inheritance]
pub proof fn inherited_markers_remain_tracked(method_id: u64)
    ensures effective_trait_tracking_spec(TraitMethod {
        id: method_id,
        body: TraitBody::Override,
        prototype_marker: MarkerState::Present,
        default_marker: MarkerState::Absent,
        local_marker: MarkerState::Absent,
        observed_effective: MarkerState::Present,
    })
{}

// r[verify track-caller.const-codegen-agreement]
pub proof fn differing_single_entry_traces_disagree(left: TraceEntry, right: TraceEntry)
    requires !trace_entries_equal_spec(left, right)
    ensures !traces_agree_spec(seq![left], seq![right])
{
    assert(!trace_entries_spec(seq![left], seq![right])) by {
        if trace_entries_spec(seq![left], seq![right]) {
            assert(trace_entries_equal_spec(seq![left][0], seq![right][0]));
            assert(seq![left][0] == left);
            assert(seq![right][0] == right);
        }
    }
}

} // verus!
