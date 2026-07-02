use vstd::prelude::*;

verus! {

/// A tiny domain-shaped predicate: a request is allowed when it is absent or granted.
pub open spec fn allowed_spec(granted: bool, requested: bool) -> bool {
    !requested || granted
}

// r[impl runnable-proof-examples.basic]
// r[impl runnable-proof-examples.commands]
/// Runtime implementation refined exactly to `allowed_spec`.
pub fn allow_when_granted(granted: bool, requested: bool) -> (result: bool)
    ensures
        result == allowed_spec(granted, requested),
{
    !requested || granted
}

// r[verify runnable-proof-examples.basic]
/// Positive proof: absent requests are always allowed.
pub proof fn absent_request_allowed(granted: bool)
    ensures
        allowed_spec(granted, false),
{
}

// r[verify runnable-proof-examples.commands]
/// Negative-domain property: an ungranted present request is rejected.
pub proof fn ungranted_request_rejected()
    ensures
        !allowed_spec(false, true),
{
}

} // verus!
