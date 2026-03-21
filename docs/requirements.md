# Verified Logic Requirements

This document defines the requirements for the `verified-logic` library.
Every requirement has a Tracey ID and maps to verified Rust code.

## Bitwise

r[bitwise.is-power-of-two]
is_power_of_two MUST return true if and only if the value is a power of two
(exactly one bit set). Zero MUST return false.

r[bitwise.bit-at]
bit_at MUST return true if and only if the bit at the given position is set.
Position MUST be in the range [0, 64).

r[bitwise.set-bit]
set_bit MUST return a value with the bit at the given position set to 1,
leaving all other bits unchanged.

r[bitwise.clear-bit]
clear_bit MUST return a value with the bit at the given position set to 0,
leaving all other bits unchanged.

r[bitwise.toggle-bit]
toggle_bit MUST return a value with the bit at the given position flipped.
Toggling twice MUST return the original value.

## Arithmetic

r[arithmetic.abs-diff]
The absolute difference function MUST return |a - b| for any two unsigned
integers without overflow.

r[arithmetic.saturating-add]
Saturating addition MUST return a + b clamped to the maximum value of the
type, never wrapping.

r[arithmetic.saturating-sub]
Saturating subtraction MUST return a - b clamped to zero when b > a,
never wrapping.

r[arithmetic.checked-mul]
Checked multiplication MUST return None when the result would overflow
the type's range, and Some(a * b) otherwise.

r[arithmetic.gcd]
The greatest common divisor function MUST compute the GCD via the Euclidean
algorithm. gcd(a, 0) MUST return a. gcd(0, a) MUST return a.

r[arithmetic.div-ceil]
Ceiling division MUST return the smallest integer >= a/b. div_ceil(0, b)
MUST return 0. The divisor MUST be nonzero.

## Bounds

r[bounds.in-range]
The in-range predicate MUST return true if and only if the value is
within the inclusive range [lo, hi].

r[bounds.clamp]
Clamp MUST return lo when value < lo, hi when value > hi, and value
otherwise. The result MUST always satisfy in-range(result, lo, hi).

## Collections

r[collections.sorted-insert]
Inserting into a sorted sequence MUST produce a sorted sequence containing
all original elements plus the new element.

r[collections.binary-search]
Binary search on a sorted sequence MUST return the index of the target if
present, or None if absent. It MUST NOT access indices outside the sequence
bounds.

r[collections.merge-sorted]
Merging two sorted sequences MUST produce a sorted sequence whose length
equals the sum of the input lengths.

## Cross-Module Properties

r[properties.abs-diff-zero-iff-equal]
abs_diff(a, b) MUST equal zero if and only if a equals b.

r[properties.add-sub-roundtrip]
When a + b does not overflow, saturating_sub(saturating_add(a, b), b) MUST
return a. Addition and subtraction MUST be inverse operations within bounds.

r[properties.triangle-inequality]
abs_diff MUST satisfy the triangle inequality: abs_diff(a, c) is less than
or equal to abs_diff(a, b) + abs_diff(b, c) for all a, b, c.

r[properties.clamp-nearest-point]
For any x in [lo, hi], clamp(value, lo, hi) MUST be at least as close to
value as x is. Clamp returns the nearest point in the range.

r[properties.clamp-monotonic]
Clamp MUST be monotonic: if a <= b then clamp(a, lo, hi) <= clamp(b, lo, hi).

r[properties.distance-decomposition]
For any value in [lo, hi], the distances to the endpoints MUST sum to the
range width: abs_diff(value, lo) + abs_diff(value, hi) == abs_diff(lo, hi).

r[properties.sub-add-inverse]
When b <= a, saturating_add(saturating_sub(a, b), b) MUST return a.
Subtraction and addition MUST be inverse operations within bounds.

r[properties.set-clear-roundtrip]
Setting a bit then clearing it MUST leave the bit cleared. Clearing a bit
then setting it MUST leave the bit set. set_bit and clear_bit MUST be
complementary operations on individual bit positions.

r[properties.toggle-involution]
Toggling a bit twice MUST return the original value for any input and
any valid bit position.

## Option/Result Combinators

r[option.unwrap-or-default]
unwrap_or_default MUST return the contained value if Some, or the provided
default if None.

r[option.map-preserves-none]
Mapping over None MUST produce None regardless of the mapping function.
