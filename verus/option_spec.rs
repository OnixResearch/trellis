use vstd::prelude::*;

verus! {

// --- unwrap_or_default ---

// r[depends option.unwrap-or-default]
/// Spec: returns the contained value or the default.
pub open spec fn unwrap_or_default_spec(opt: Option<u64>, default: u64) -> u64 {
    match opt {
        Some(v) => v,
        None => default,
    }
}

// r[verify option.unwrap-or-default]
/// Proof: unwrap_or_default on Some returns the inner value.
proof fn unwrap_or_default_some(v: u64, default: u64)
    ensures unwrap_or_default_spec(Some(v), default) == v
{
}

// r[verify option.unwrap-or-default]
/// Proof: unwrap_or_default on None returns the default.
proof fn unwrap_or_default_none(default: u64)
    ensures unwrap_or_default_spec(None, default) == default
{
}

// --- map ---

// r[depends option.map-preserves-none]
/// Spec: mapping over an option applies the function to the inner
/// value if present, preserves None otherwise.
pub open spec fn map_spec(opt: Option<u64>, f: spec_fn(u64) -> u64) -> Option<u64> {
    match opt {
        Some(v) => Some(f(v)),
        None => None,
    }
}

// r[verify option.map-preserves-none]
/// Proof: mapping over None always produces None, regardless of f.
proof fn map_none_is_none(f: spec_fn(u64) -> u64)
    ensures map_spec(None, f) == None::<u64>
{
}

// r[verify option.map-preserves-none]
/// Proof: mapping over Some applies the function.
proof fn map_some_applies_f(v: u64, f: spec_fn(u64) -> u64)
    ensures map_spec(Some(v), f) == Some(f(v))
{
}

} // verus!
