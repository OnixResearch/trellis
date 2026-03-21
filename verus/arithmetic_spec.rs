use vstd::prelude::*;

verus! {

// --- abs_diff ---

// r[depends arithmetic.abs-diff]
/// Spec: absolute difference of two u64 values.
pub open spec fn abs_diff_spec(a: u64, b: u64) -> u64 {
    if a >= b { (a - b) as u64 } else { (b - a) as u64 }
}

// r[verify arithmetic.abs-diff]
/// Proof: abs_diff is symmetric.
proof fn abs_diff_symmetric(a: u64, b: u64)
    ensures abs_diff_spec(a, b) == abs_diff_spec(b, a)
{
}

// r[verify arithmetic.abs-diff]
/// Proof: abs_diff(x, x) == 0 for all x.
proof fn abs_diff_self_is_zero(x: u64)
    ensures abs_diff_spec(x, x) == 0
{
}

// --- saturating_add ---

// r[depends arithmetic.saturating-add]
/// Spec: saturating addition — returns a + b clamped to u64::MAX.
pub open spec fn saturating_add_spec(a: u64, b: u64) -> u64 {
    if a as int + b as int > u64::MAX as int {
        u64::MAX
    } else {
        (a + b) as u64
    }
}

// r[verify arithmetic.saturating-add]
/// Proof: saturating_add never exceeds u64::MAX.
proof fn saturating_add_bounded(a: u64, b: u64)
    ensures saturating_add_spec(a, b) <= u64::MAX
{
}

// r[verify arithmetic.saturating-add]
/// Proof: when no overflow, saturating_add returns exact sum.
proof fn saturating_add_exact_when_no_overflow(a: u64, b: u64)
    requires a as int + b as int <= u64::MAX as int
    ensures saturating_add_spec(a, b) == (a + b) as u64
{
}

// --- saturating_sub ---

// r[depends arithmetic.saturating-sub]
/// Spec: saturating subtraction — returns a - b clamped to 0.
pub open spec fn saturating_sub_spec(a: u64, b: u64) -> u64 {
    if b > a {
        0
    } else {
        (a - b) as u64
    }
}

// r[verify arithmetic.saturating-sub]
/// Proof: saturating_sub result is always >= 0 (trivial for u64, but
/// the spec encodes the clamping behavior).
proof fn saturating_sub_non_negative(a: u64, b: u64)
    ensures saturating_sub_spec(a, b) >= 0
{
}

// r[verify arithmetic.saturating-sub]
/// Proof: when a >= b, saturating_sub returns exact difference.
proof fn saturating_sub_exact_when_no_underflow(a: u64, b: u64)
    requires a >= b
    ensures saturating_sub_spec(a, b) == (a - b) as u64
{
}

// --- checked_mul ---

// r[depends arithmetic.checked-mul]
/// Spec: checked multiplication — None on overflow, Some(a*b) otherwise.
pub open spec fn checked_mul_spec(a: u64, b: u64) -> Option<u64> {
    if a as int * b as int > u64::MAX as int {
        None
    } else {
        Some((a * b) as u64)
    }
}

// r[verify arithmetic.checked-mul]
/// Proof: checked_mul returns Some iff the product fits in u64.
proof fn checked_mul_some_iff_no_overflow(a: u64, b: u64)
    ensures checked_mul_spec(a, b).is_some() <==> (a as int * b as int <= u64::MAX as int)
{
}

// r[verify arithmetic.checked-mul]
/// Proof: multiplying by zero always succeeds.
proof fn checked_mul_zero(a: u64)
    ensures
        checked_mul_spec(a, 0) == Some(0u64),
        checked_mul_spec(0, a) == Some(0u64),
{
}

} // verus!
