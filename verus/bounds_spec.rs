use vstd::prelude::*;

verus! {

// --- in_range ---

// r[depends bounds.in-range]
/// Spec: value is within inclusive range [lo, hi].
pub open spec fn in_range_spec(value: u64, lo: u64, hi: u64) -> bool {
    lo <= value && value <= hi
}

// r[verify bounds.in-range]
/// Proof: boundary values are in range.
proof fn in_range_boundaries(lo: u64, hi: u64)
    requires lo <= hi
    ensures
        in_range_spec(lo, lo, hi),
        in_range_spec(hi, lo, hi),
{
}

// r[verify bounds.in-range]
/// Proof: empty range contains nothing.
proof fn in_range_empty(value: u64, lo: u64, hi: u64)
    requires lo > hi
    ensures !in_range_spec(value, lo, hi)
{
}

// --- clamp ---

// r[depends bounds.clamp]
/// Spec: clamp value to [lo, hi].
pub open spec fn clamp_spec(value: u64, lo: u64, hi: u64) -> u64
    recommends lo <= hi
{
    if value < lo {
        lo
    } else if value > hi {
        hi
    } else {
        value
    }
}

// r[verify bounds.clamp]
/// Proof: clamp result is always in range [lo, hi].
proof fn clamp_in_range(value: u64, lo: u64, hi: u64)
    requires lo <= hi
    ensures in_range_spec(clamp_spec(value, lo, hi), lo, hi)
{
}

// r[verify bounds.clamp]
/// Proof: clamp is idempotent — clamping an already-in-range value
/// returns it unchanged.
proof fn clamp_idempotent(value: u64, lo: u64, hi: u64)
    requires lo <= hi, in_range_spec(value, lo, hi)
    ensures clamp_spec(value, lo, hi) == value
{
}

// r[verify bounds.clamp]
/// Proof: clamp(clamp(v, lo, hi), lo, hi) == clamp(v, lo, hi).
proof fn clamp_double_application(value: u64, lo: u64, hi: u64)
    requires lo <= hi
    ensures clamp_spec(clamp_spec(value, lo, hi), lo, hi) == clamp_spec(value, lo, hi)
{
    // First show the inner clamp is in range, then idempotence follows.
    clamp_in_range(value, lo, hi);
    clamp_idempotent(clamp_spec(value, lo, hi), lo, hi);
}

} // verus!
