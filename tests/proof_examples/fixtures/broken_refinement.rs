use vstd::prelude::*;

verus! {

pub open spec fn allowed_spec(granted: bool, requested: bool) -> bool {
    !requested || granted
}

/// Intentional negative fixture: this grants only when `granted` is true, so it
/// violates the spec for an absent request.
pub fn broken_allow_when_granted(granted: bool, requested: bool) -> (result: bool)
    ensures
        result == allowed_spec(granted, requested),
{
    granted
}

} // verus!
