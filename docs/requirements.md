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

r[collections.is-partitioned]
is_partitioned MUST return true if and only if all elements below the
threshold precede all elements at or above it. Sorted sequences MUST be
partitioned by any threshold. Empty slices MUST be partitioned.

r[collections.find-first]
find_first MUST return the index of the first occurrence of target, or
None if absent. When found, no earlier index may contain the target.
find_first on an empty slice MUST return None.

r[collections.count-matches]
count_matches MUST return the number of elements equal to target. The
count MUST be zero for empty slices. The count MUST never exceed the
slice length.

## Integer

r[integer.is-even]
is_even MUST return true if and only if x is divisible by 2. is_even and
is_odd MUST be complements: exactly one is true for any input.

r[integer.is-odd]
is_odd MUST return true if and only if x is not divisible by 2.

r[integer.align-down]
align_down MUST return the largest multiple of `align` that is <= x. The
result MUST be <= x. The result MUST be divisible by `align`. Alignment 1
MUST be the identity function.

r[integer.align-up]
align_up MUST return the smallest multiple of `align` that is >= x, or
None if the result would overflow u64. When defined, the result MUST be
>= x and divisible by `align`. Alignment 1 MUST be the identity.

r[integer.log2-floor]
log2_floor MUST return floor(log2(x)) for x > 0. log2_floor(1) MUST be 0.
log2_floor(2) MUST be 1.

r[integer.next-power-of-two]
next_power_of_two MUST return the smallest power of 2 >= x, or None on
overflow. next_power_of_two(0) and next_power_of_two(1) MUST return
Some(1).

## Ordering

r[ordering.min]
min MUST return the smaller of two values. min MUST be commutative,
associative, and idempotent.

r[ordering.max]
max MUST return the larger of two values. max MUST be commutative,
associative, and idempotent. min(a, b) <= max(a, b) MUST hold.

r[ordering.median-of-three]
median_of_three MUST return the middle value of three inputs. The result
MUST be bounded by the overall min and max of the three values. The median
of three equal values MUST be that value.

r[ordering.sign]
sign MUST return -1 if a < b, 0 if a == b, and 1 if a > b.
sign MUST be antisymmetric: sign(a, b) == -sign(b, a).

r[ordering.compare-and-swap]
compare_and_swap MUST return (min(a,b), max(a,b)). The first element MUST
be less than or equal to the second. Both input values MUST be preserved.

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

r[properties.min-max-abs-diff]
max(a, b) - min(a, b) MUST equal abs_diff(a, b). The gap between min and
max is the absolute difference.

r[properties.clamp-via-min-max]
clamp(v, lo, hi) MUST equal min(max(v, lo), hi) when lo <= hi. Clamp is
expressible as nested min/max.

r[properties.align-down-remainder]
x - align_down(x, a) MUST equal x % a. The distance from x to the
aligned-down value is the remainder.

r[properties.align-up-via-div-ceil]
When align_up(x, a) is defined, it MUST equal div_ceil(x, a) * a. Rounding
up to an alignment is ceiling division times the alignment.

r[properties.median-is-clamp]
median_of_three(a, b, c) MUST equal clamp(a, min(b, c), max(b, c)).
The median is a clamped value.

r[properties.all-equal-implies-count]
If all_equal(s, v) holds, count_matches(s, v) MUST equal s.len(). Every
element matching means the count is the full length.

r[properties.chunk-count-div-ceil]
div_ceil(n, k) MUST equal chunk_count(n, k) + 1 when n has a remainder
mod k, and chunk_count(n, k) when it divides evenly. Floor and ceiling
division differ by exactly the presence of a remainder.

r[properties.narrow-iff-in-range]
narrow_u32(x).is_some() MUST hold if and only if in_range(x, 0, u32::MAX)
holds. Checked narrowing and range predicates MUST agree.

r[properties.all-equal-implies-partitioned]
If all_equal(s, v) holds, is_partitioned(s, threshold) MUST hold for any
threshold. A uniform sequence is trivially partitioned.

r[properties.majority-via-div-ceil]
majority(n) MUST equal div_ceil(n + 1, 2) for all n > 0. The majority
threshold is ceiling division of (n+1) by 2.

r[properties.epoch-ge-term-index]
epoch_ge(a, b) MUST imply a.term > b.term, or a.term == b.term and
a.index >= b.index. The lexicographic comparison decomposes into term
and index conditions.

r[properties.log-prefix-term-eq]
If logs_match_prefix(a, b, k) holds, the term sequences of a and b
MUST be equal over [0, k). Log term-agreement is equivalent to prefix
equality on the projected term sequence.

## Option Combinators

r[option.unwrap-or-default]
unwrap_or_default MUST return the contained value if Some, or the provided
default if None.

r[option.map-preserves-none]
Mapping over None MUST produce None regardless of the mapping function.
Mapping MUST satisfy the functor composition law: map(map(opt, f), g)
equals map(opt, g∘f).

r[option.and-then]
and_then MUST apply `f` to the contained value if Some, returning its
result. and_then on None MUST return None. and_then MUST satisfy the
monad identity laws: and_then(Some(v), f) == f(v) and
and_then(opt, Some) == opt.

r[option.or-else]
or_else MUST return the option unchanged if Some. or_else on None MUST
return the fallback value. or_else with a Some fallback MUST always
yield Some.

r[option.flatten]
flatten on Some(Some(v)) MUST return Some(v). flatten on Some(None) and
flatten on None MUST both return None.

r[option.zip]
zip MUST return Some((a, b)) if and only if both inputs are Some.
If either input is None, zip MUST return None.

r[option.filter]
filter MUST return Some(v) when the predicate holds, None when it does
not. filter on None MUST return None. filter MUST be idempotent:
filter(filter(opt, p), p) == filter(opt, p).

## Result Combinators

r[result.unwrap-or]
unwrap_or MUST return the Ok value if present, or the provided default
if Err.

r[result.map-ok]
map_ok MUST apply `f` to the Ok value, leaving Err untouched. map_ok
MUST satisfy the functor composition law. map_ok and map_err MUST
commute: the order of application does not matter.

r[result.map-err]
map_err MUST apply `f` to the Err value, leaving Ok untouched.

r[result.and-then]
and_then on Ok(v) MUST apply `f`, returning its result. and_then on
Err(e) MUST propagate the error unchanged. and_then MUST satisfy the
monad right identity: and_then(res, Ok) == res.

r[result.or-else]
or_else on Ok(v) MUST return Ok(v) unchanged. or_else on Err MUST return
the fallback. or_else with an Ok fallback MUST always yield Ok.

## Slice

r[slice.chunk-count]
chunk_count MUST return the number of full chunks of `chunk_size` that fit
in a length of `len`. The result times chunk_size MUST not exceed len. The
remainder MUST be less than chunk_size. chunk_size MUST be nonzero.

r[slice.prefix-eq]
prefix_eq MUST return true if and only if the first `n` elements of both
slices are equal. When n is zero, prefix_eq MUST return true. When n
exceeds either slice length, prefix_eq MUST return false.

r[slice.suffix-eq]
suffix_eq MUST return true if and only if the last `n` elements of both
slices are equal. When n is zero, suffix_eq MUST return true. When n
exceeds either slice length, suffix_eq MUST return false.

r[slice.all-equal]
all_equal MUST return true if and only if every element in the slice
equals the given value. all_equal on an empty slice MUST return true.

r[slice.mismatch-index]
mismatch_index MUST return the index of the first position where two
slices differ, comparing up to the shorter length. MUST return None if
all compared elements match. mismatch_index of two empty slices MUST
return None.

## Conversion

r[conversion.narrow-u32]
narrow_u32 MUST return Some when the input fits in u32, None otherwise.
Widening a u32 then narrowing MUST return the original value. Values
above u32::MAX MUST be rejected.

r[conversion.narrow-u16]
narrow_u16 MUST return Some when the input fits in u16, None otherwise.
Widening a u16 then narrowing MUST return the original value. Values
above u16::MAX MUST be rejected.

r[conversion.narrow-u8]
narrow_u8 MUST return Some when the input fits in u8, None otherwise.
Widening a u8 then narrowing MUST return the original value. Values
above u8::MAX MUST be rejected.

r[conversion.signed-to-unsigned]
signed_to_unsigned MUST return Some(x as u64) for non-negative i64
values, and None for negative values. The i64 → u64 → i64 roundtrip
MUST preserve the value for non-negative inputs.

r[conversion.unsigned-to-signed]
unsigned_to_signed MUST return Some(x as i64) when x <= i64::MAX, and
None otherwise. The u64 → i64 → u64 roundtrip MUST preserve the value
for inputs that fit in i64.

## Expiration

r[expiration.expires-at]
expires_at(now_ms, ttl_seconds) MUST return now_ms + ttl_seconds * 1000,
saturating at u64::MAX on overflow. The result MUST always be >= now_ms.

r[expiration.is-expired]
is_expired(expires_at_ms, now_ms) MUST return true if and only if
now_ms > expires_at_ms. At the exact expiration time, the entry is NOT
expired.

r[expiration.is-expired-opt]
is_expired_opt(expires_at_ms, now_ms) MUST return false when expires_at_ms
is None (no TTL means never expires). When Some, MUST delegate to
is_expired.

r[expiration.time-remaining]
time_remaining(expires_at_ms, now_ms) MUST return None when expires_at_ms
is None. MUST return Some(0) when already expired. MUST return
Some(expires_at - now) when not yet expired.

r[expiration.is-ttl-valid]
is_ttl_valid(ttl_seconds, max_ttl) MUST return true if and only if
ttl_seconds <= max_ttl.

## Chain Hash

r[chain.hash-deterministic]
The hash function MUST be deterministic: same inputs MUST always produce
the same output. Modeled as an uninterpreted function with this axiom.

r[chain.compute-entry-hash]
compute_entry_hash MUST combine prev_hash, log_index, term, and data
into a single hash input and return the hash. The result MUST depend on
all four inputs.

r[chain.chain-valid]
A chain is valid if every entry's hash equals the hash computed from its
predecessor's hash, its index, its term, and its data. The first entry
uses the genesis hash as its predecessor.

r[chain.append-preserves]
Appending a correctly-hashed entry to a valid chain MUST produce a valid
chain. All prior entries MUST remain valid.

r[chain.collision-free]
The hash function MUST be collision-free: if two calls to hash_spec
produce the same output, all four inputs MUST be identical. This is an
idealized cryptographic assumption (trusted axiom) modeling the
collision resistance of Blake3.

r[chain.agreement]
If two valid chains have matching hashes at position k, all entries at
positions 0 through k MUST be identical. This is the fundamental
security guarantee: the hash at any position commits to the entire
chain prefix.

r[chain.divergence-detected]
If two valid chains differ at any position j, their hashes at every
position k >= j MUST differ. Hash chains are tamper-evident: any
modification to any entry is detectable by comparing tip hashes.

r[chain.single-entry-valid]
A single entry correctly hashed from the genesis hash MUST form a
valid chain of length one.

## Batch Limits

r[batch.check-limits]
check_limits MUST report whether adding an operation would exceed the
entry count or byte size limit. An empty batch (zero entries) MUST
never report exceeded — the first operation always fits.

r[batch.flush-decision]
flush_decision MUST return Immediate when the batch is full (entries or
bytes), when batching is disabled (immediate mode), or when the batch
is empty. MUST return Delayed when the batch has items but is not full
and no flush is already scheduled. MUST return None when a flush is
already scheduled.

r[batch.op-size]
op_size MUST return key_len + value_len, saturating at u64::MAX.
delete_op_size MUST return key_len.

## Interval

r[interval.contains]
contains(lo, hi, x) MUST return true if and only if lo <= x < hi.
An empty interval (lo >= hi) MUST contain nothing.

r[interval.is-empty]
is_empty(lo, hi) MUST return true if and only if lo >= hi.

r[interval.overlaps]
overlaps(a_lo, a_hi, b_lo, b_hi) MUST return true if and only if the
intervals share at least one point. Two empty intervals MUST NOT overlap.
Overlap MUST be symmetric.

r[interval.split]
split(lo, hi, mid) MUST produce two intervals [lo, mid) and [mid, hi)
that cover the original without overlap. mid MUST satisfy lo < mid < hi.

r[interval.adjacent]
adjacent(a_lo, a_hi, b_lo, b_hi) MUST return true if and only if one
interval ends exactly where the other begins. Adjacent MUST be symmetric.

r[interval.merge]
merge(a_lo, a_hi, b_lo, b_hi) MUST combine two adjacent intervals into
one covering both. Merge of a split MUST recover the original.

## Compare-and-Swap

r[cas.check-condition]
check_condition MUST evaluate a typed condition against a current value:
VALUE_EQUALS succeeds when current matches expected, KEY_EXISTS succeeds
when current is Some, KEY_NOT_EXISTS succeeds when current is None.
Unknown condition types MUST return false.

r[cas.check-cas]
check_cas MUST return true if and only if expected and current match:
both None, or both Some with equal values. Any other combination MUST
return false.

r[cas.evaluate-conditions]
evaluate_conditions MUST check conditions in order, returning the index
of the first failure. If all conditions pass, MUST return all-met.
An empty condition list MUST return all-met.

## Bitmap

r[bitmap.new]
new MUST create a bitmap with num_bits bits, all initially clear. The
number of backing u64 words MUST equal ceil(num_bits / 64).

r[bitmap.get]
get MUST return true if and only if the bit at the given position is set.
Position MUST be less than num_bits.

r[bitmap.set]
set MUST set the bit at the given position to 1 without affecting other
words. After set, get at the same position MUST return true.

r[bitmap.clear]
clear MUST set the bit at the given position to 0 without affecting other
words. After clear, get at the same position MUST return false.

r[bitmap.toggle]
toggle MUST flip the bit at the given position. Toggling twice MUST
restore the original word value.

r[bitmap.len]
len MUST return the total number of bits in the bitmap.

r[bitmap.word-count]
word_count MUST return the number of u64 words backing the bitmap.

## Ring Buffer

r[ring-buffer.new]
new MUST create an empty ring buffer with the given power-of-two capacity.
The result MUST satisfy the well-formedness invariant with count zero.

r[ring-buffer.push]
push MUST write the value at the tail position and advance the tail
with wraparound. push MUST return true on success and false when full.
Count MUST increase by one on success. The buffer MUST remain well-formed.

r[ring-buffer.pop]
pop MUST return Some with the head element and advance the head with
wraparound. pop MUST return None when empty. Count MUST decrease by one
on success. The buffer MUST remain well-formed.

r[ring-buffer.peek]
peek MUST return Some with the head element without modifying the buffer.
peek MUST return None when empty.

r[ring-buffer.len]
len MUST return the current count of elements.

r[ring-buffer.is-empty]
is_empty MUST return true if and only if count is zero. An empty buffer
with nonzero capacity MUST NOT be full.

r[ring-buffer.is-full]
is_full MUST return true if and only if count equals capacity.

r[ring-buffer.capacity]
capacity MUST return the buffer's fixed capacity.

r[ring-buffer.remaining]
remaining MUST return capacity minus current count. remaining plus len
MUST equal capacity.

r[ring-buffer.full-batches]
full_batches MUST return the number of complete batches of the given
size that can be popped from the current contents.

## Quorum

r[quorum.majority]
majority(n) MUST return n / 2 + 1 for any n > 0. majority(1) MUST be 1.
majority(2) MUST be 2. majority(5) MUST be 3. The result MUST satisfy
1 <= majority(n) <= n.

r[quorum.is-quorum]
is_quorum(count, total) MUST return true if and only if count >=
majority(total). total MUST be greater than zero.

r[quorum.intersection]
If is_quorum(a, n) and is_quorum(b, n) both hold, then a + b MUST be
greater than n. Two quorums of the same set MUST always overlap.

r[quorum.fault-tolerance]
fault_tolerance(n) MUST return (n - 1) / 2 for n > 0, and 0 for n == 0.
This is the maximum number of node failures a cluster of size n can
tolerate while maintaining quorum.

r[quorum.can-remove-safely]
can_remove_safely(n) MUST return true if and only if removing one voter
from a cluster of n still leaves a quorum. MUST return false for n <= 2
and true for n >= 3.

r[quorum.min-cluster-size]
min_cluster_size(f) MUST return 2*f + 1 — the smallest cluster that
tolerates f failures. min_cluster_size(fault_tolerance(n)) MUST be <= n
for all n > 0.

## Monotonic

r[monotonic.is-advance]
is_advance(old, new) MUST return true if and only if new >= old. Advance
MUST be reflexive, transitive, and antisymmetric.

r[monotonic.is-strict-advance]
is_strict_advance(old, new) MUST return true if and only if new > old.
Strict advance MUST be irreflexive and transitive. A strict advance
followed by a non-strict advance MUST still be a strict advance.

## Epoch

r[epoch.cmp]
epoch_cmp MUST compare (term, index) pairs lexicographically: first by
term, then by index. MUST return -1 if a < b, 0 if a == b, 1 if a > b.
The ordering MUST be total, transitive, and antisymmetric.

r[epoch.ge]
epoch_ge(a, b) MUST return true if and only if epoch a is greater than
or equal to b in the lexicographic ordering. This is the vote-granting
predicate: a candidate's log must be at least as up-to-date as the voter's.

## ACID Transaction Properties

r[acid.apply-write]
apply_write MUST update the state at the given index to the new value,
leaving all other positions unchanged. The state length MUST be preserved.
All write indices MUST be within state bounds.

r[acid.commit]
commit (apply_writes) MUST apply all write operations in sequence.
Committing an empty write set MUST return the state unchanged. The
state length MUST be preserved after commit.

r[acid.abort]
abort MUST return the state unchanged. No positions are modified.

r[acid.atomicity]
A committed transaction MUST have all writes applied — the final state
reflects every write in order. An aborted transaction MUST leave the
state identical to the original. The state length MUST be preserved.
No partial application is possible.

r[acid.consistency]
Positions not targeted by any write operation MUST retain their original
values after commit. Only explicitly written positions may change.

r[acid.isolation]
Two writes to distinct positions MUST commute: applying write A then B
MUST produce the same state as applying B then A when their indices
differ. Reading a position not targeted by a write MUST return the
original value.

r[acid.durability]
WAL recovery MUST replay all committed entries and skip uncommitted
entries. Recovering a fully committed WAL MUST produce the same state
as a direct commit. Recovering a fully uncommitted WAL MUST leave
the state unchanged. Recovery MUST preserve state length.

r[acid.writes-disjoint]
writes_disjoint MUST return true if and only if no write index in the
first set matches any write index in the second set.

## Copy-on-Write

r[cow.should-copy]
should_copy(ref_count) MUST return true if and only if ref_count > 1.
An exclusive owner (ref_count == 1) MUST NOT require a copy.

r[cow.ref-count-after-clone]
ref_count_after_clone MUST return ref_count + 1, saturating at u64::MAX.
The result MUST always be greater than or equal to the input.

r[cow.ref-count-after-drop]
ref_count_after_drop MUST return ref_count - 1 when ref_count > 0, and
0 when ref_count is already 0. The result MUST always be less than or
equal to the input.

r[cow.is-exclusive]
is_exclusive MUST return true if and only if ref_count == 1. Exclusive
ownership and should_copy MUST be complements for any valid (> 0) ref
count.

r[cow.can-reclaim]
can_reclaim MUST return true if and only if ref_count == 0. Storage
backing a COW value MUST only be freed when no references remain.

r[cow.snapshot-isolation]
After a COW write on shared data, readers MUST see the old value and
the writer MUST see the new value. When exclusive, the writer sees the
new value and no other readers exist.

## Approval

r[approval.count-valid]
count_valid_approvals MUST return the number of entries where both
approved and matches_head_commit are true. The count MUST be less than
or equal to the total length. An empty input MUST yield zero.

r[approval.is-merge-allowed]
is_merge_allowed MUST return true if and only if ci_passed is true AND
valid_count >= required. If CI has not passed, merge MUST NOT be allowed
regardless of approval count.

r[approval.needs-more]
needs_more_approvals MUST return required - valid_count when valid_count
< required, and zero otherwise. The result MUST be zero if and only if
valid_count >= required.

r[approval.has-stale]
has_stale_approvals MUST return true if any entry has approved true but
matches_head_commit false. An empty input MUST return false.

## Bounded Range

r[bounded-range.contains]
contains MUST return true if x >= lower and either the range is unbounded
or x < upper. An unbounded range MUST contain all x >= lower. An empty
bounded range MUST contain nothing.

r[bounded-range.is-empty]
is_empty MUST return true if and only if the range is bounded and
lower >= upper. Unbounded ranges MUST never be empty.

r[bounded-range.overlaps]
overlaps MUST return true if and only if two non-empty ranges share at
least one point. Overlap MUST be symmetric. Empty ranges MUST NOT
overlap. Two non-empty unbounded ranges MUST always overlap.

r[bounded-range.is-bounded]
is_bounded MUST return true if and only if has_upper is true and
lower < upper. Bounded implies non-empty.

r[bounded-range.restrict]
restrict MUST produce a bounded range with the same lower bound and an
upper bound of new_upper (or min of current and new_upper for already-
bounded ranges). The result MUST be a subset of the input range.

## Chunk

r[chunk.count-ceil]
chunk_count_ceil MUST return the number of chunks needed including a
partial last chunk. MUST use ceiling division without overflow:
if total_size == 0 then 0, else (total_size - 1) / chunk_size + 1.
chunk_size MUST be nonzero.

r[chunk.size-at]
chunk_size_at MUST return the size of the chunk at the given index. All
chunks except possibly the last MUST have size chunk_size. The last chunk
MUST have size total_size - index * chunk_size. The result MUST be > 0
and <= chunk_size. index MUST be < chunk_count_ceil.

r[chunk.offset]
chunk_offset MUST return index * chunk_size. The multiplication MUST
NOT overflow. chunk_size MUST be nonzero.

r[chunk.index-for-offset]
chunk_index_for_offset MUST return offset / chunk_size. chunk_size MUST
be nonzero.

r[chunk.offset-within]
offset_within_chunk MUST return offset % chunk_size. The result MUST be
less than chunk_size. chunk_size MUST be nonzero.

## Drain

r[drain.can-start-op]
can_start_op MUST return true if and only if is_draining is false. New
operations MUST NOT be admitted during drain.

r[drain.after-start-op]
after_start_op MUST return in_flight + 1. in_flight MUST be less than
u64::MAX to prevent overflow.

r[drain.after-finish-op]
after_finish_op MUST return in_flight - 1. in_flight MUST be greater
than zero to prevent underflow.

r[drain.is-drain-complete]
is_drain_complete MUST return true if and only if is_draining is true
AND in_flight is zero. Both conditions MUST hold.

r[drain.should-notify]
should_notify MUST return true if and only if is_draining is true,
old_in_flight > 0, and new_in_flight == 0. This detects the transition
when the last operation finishes during drain.

r[drain.start-drain]
start_drain_decision MUST return 0 if already draining, 1 if drain
completes immediately (no in-flight ops), 2 if drain starts but must
wait. The result MUST always be 0, 1, or 2.

## Eviction

r[eviction.should-evict]
should_evict MUST return true if and only if count >= capacity. The
cache is at or over capacity and eviction is needed.

r[eviction.find-lru]
find_lru_candidate MUST return the index of the entry with the smallest
access_time among non-protected entries. MUST return None if all entries
are protected or the slice is empty. When found, the entry at the
returned index MUST NOT be protected and MUST have access_time <= all
other non-protected entries.

r[eviction.cache-valid]
is_cache_valid MUST return true if and only if count <= capacity + 1
AND protected_count <= count. The cache may temporarily exceed capacity
by one before eviction runs.

r[eviction.evictable-count]
evictable_count MUST return count - protected_count using saturating
subtraction. The result MUST be <= count.

## Hash Bucket

r[hash-bucket.assign]
assign_bucket MUST return hash % num_buckets. The result MUST be less
than num_buckets. num_buckets MUST be nonzero.

r[hash-bucket.assign-pow2]
assign_bucket_pow2 MUST return hash & (num_buckets - 1). The result
MUST be less than num_buckets. num_buckets MUST be a power of two.

r[hash-bucket.valid-count]
bucket_count_valid MUST return true if and only if num_buckets > 0.

r[hash-bucket.is-pow2]
is_power_of_two_buckets MUST return true if and only if num_buckets > 0
and num_buckets & (num_buckets - 1) == 0.

## Log

r[log.agree-at]
agree_at(a, b, i) MUST return true if and only if both logs have an entry
at index i and the entries have the same term. Agreement checks term
equality, not value equality.

r[log.prefix-matching]
If two logs agree at every index in [0, k), they MUST have matching terms
at all those indices. logs_match_prefix(a, b, k) MUST hold when
agree_at(a, b, i) holds for all 0 <= i < k.

r[log.append]
Appending an entry MUST increase the log length by 1. The appended entry
MUST appear at the new last index. All prior entries MUST be unchanged.

r[log.truncate]
Truncating a log to length k MUST discard all entries at index k and
beyond. The first k entries MUST be unchanged. The resulting length MUST
be k. k MUST not exceed the current length.

## Partition

r[partition.is-valid]
is_valid_partition MUST check that a sequence of intervals forms a valid
non-overlapping gap-free covering: every interval is non-empty (lo < hi),
intervals are sorted by start, and each interval's end equals the next
interval's start. An empty sequence MUST be a valid partition.

r[partition.is-non-empty-interval]
is_non_empty_interval MUST return true if and only if lo < hi.

r[partition.is-adjacent-pair]
is_adjacent_pair MUST return true if and only if a_hi == b_lo. Adjacent
intervals form a continuous covering with no gap or overlap.

r[partition.covers]
partition_covers MUST verify that a non-empty partition exactly covers
a specified range [lo, hi): the first interval starts at lo and the
last interval ends at hi.

r[partition.span]
partition_span_lo MUST return the first interval's start.
partition_span_hi MUST return the last interval's end. Both MUST
require a non-empty partition.

## Pressure

r[pressure.classify]
classify_level MUST return 0 (Normal) when value < warn_threshold,
1 (Warning) when warn_threshold <= value < crit_threshold, and
2 (Critical) when value >= crit_threshold. The result MUST always be
0, 1, or 2. Classification MUST be monotonic: higher values produce
same or higher levels. warn_threshold MUST be <= crit_threshold.

r[pressure.is-critical]
is_critical MUST return true if and only if level == 2.

r[pressure.should-pause]
should_pause MUST return true if and only if level >= 2. Critical
pressure MUST trigger a pause.

r[pressure.most-severe]
most_severe MUST return the maximum of two levels, capped at 2. MUST
be commutative, associative, and idempotent. The result MUST be >= both
inputs.

r[pressure.is-all-normal]
is_all_normal MUST return true if and only if every element in the
slice equals 0. An empty slice MUST return true.

## Rate Limit

r[rate-limit.refill]
tokens_after_refill MUST return tokens + elapsed_ms * rate_per_sec / 1000,
capped at burst. On multiplication overflow, MUST return burst. Zero
elapsed time MUST return tokens unchanged. The result MUST never exceed
burst. Refill MUST be monotonic in elapsed_ms.

r[rate-limit.try-consume]
try_consume MUST return (true, tokens - cost) when tokens >= cost, and
(false, tokens) otherwise. On denial, tokens MUST be unchanged.

r[rate-limit.within-burst]
is_within_burst MUST return true if and only if tokens <= burst. After
any refill, tokens MUST be within burst.

r[rate-limit.wait-time]
tokens_to_wait_ms MUST return 0 when tokens >= cost, and
ceil((cost - tokens) * 1000 / rate_per_sec) otherwise. rate_per_sec
MUST be nonzero.

## Sequential

r[sequential.is-sequential]
is_sequential MUST return true if and only if offset == last_offset +
last_size, handling u64 overflow by returning false when the addition
would wrap. offset 0 with last_offset 0 and last_size 0 MUST be
sequential.

r[sequential.update-consecutive]
update_consecutive MUST return consecutive + 1 (saturating at u64::MAX)
when is_seq is true, and 0 when is_seq is false. Non-sequential access
MUST reset the counter.

r[sequential.should-prefetch]
should_prefetch MUST return true if and only if consecutive >= threshold.
threshold MUST be nonzero. MUST return false when consecutive < threshold.

r[sequential.prefetch-size]
prefetch_size MUST return min(consecutive * chunk_size, max_prefetch),
saturating on multiplication overflow. chunk_size MUST be nonzero. The
result MUST never exceed max_prefetch.

## Cross-Module Properties (New Modules)

r[properties.chunk-ceil-is-div-ceil]
chunk_count_ceil(n, k) MUST equal div_ceil(n, k) for all n, k > 0.
The two independent ceiling division implementations MUST agree.

r[properties.bounded-range-specializes-interval]
When has_upper is true, bounded_range.contains(lo, hi, true, x) MUST
equal interval.contains(lo, hi, x). The generic range MUST reduce to
the numeric interval when bounded.

r[properties.partition-no-overlap]
For adjacent intervals in a valid partition, interval.overlaps MUST
return false. Partition adjacency (a_hi == b_lo) MUST guarantee
interval non-overlap.

r[properties.pressure-classify-monotonic]
If value1 <= value2, classify_level(value1, w, c) MUST be <=
classify_level(value2, w, c). Monotonic inputs produce monotonic
severity levels.

r[properties.hash-bucket-in-interval]
interval.contains(0, num_buckets, assign_bucket(hash, num_buckets))
MUST hold for all hash and num_buckets > 0. Hash assignment MUST
land in the valid bucket interval [0, num_buckets).

r[properties.refill-within-bounds]
in_range(tokens_after_refill(t, e, r, b), 0, b) MUST hold. The
refill result MUST always be within [0, burst].

r[properties.drain-start-finish-roundtrip]
after_finish_op(after_start_op(n)) MUST equal n for valid n. Starting
then finishing an operation MUST be a no-op on the in-flight count.

r[properties.sequential-chunk-continuation]
When reading chunk i then chunk i+1 of the same file, is_sequential
MUST return true. Sequential chunk reads MUST be detected as sequential.

## Permission

r[permission.bit-position]
bit_position MUST map (who, perm) to a bit index in [0, 8]. who encodes
the role: 0=owner, 1=group, 2=other. perm encodes the access type:
0=execute, 1=write, 2=read. The formula MUST be (2 - who) * 3 + perm.

r[permission.has]
has_permission MUST return true if and only if the permission bit at
bit_position(who, perm) is set in mode. who MUST be <= 2. perm MUST
be <= 2.

r[permission.set]
set_permission MUST return mode with the permission bit at
bit_position(who, perm) set to 1, leaving all other bits unchanged.
Setting an already-set permission MUST be idempotent.

r[permission.clear]
clear_permission MUST return mode with the permission bit at
bit_position(who, perm) set to 0, leaving all other bits unchanged.
Clearing an already-clear permission MUST be idempotent.

r[permission.apply-umask]
apply_umask MUST return mode & !umask. The result MUST be a bitwise
subset of mode. The result MUST have no bits set that are in umask.

## Signal

r[signal.add]
signal_add MUST return the signal set with the given signal added.
Adding a signal already in the set MUST be idempotent. sig MUST be
in [0, 64).

r[signal.remove]
signal_remove MUST return the signal set with the given signal removed.
Removing a signal not in the set MUST be idempotent. sig MUST be
in [0, 64).

r[signal.contains]
signal_contains MUST return true if and only if the signal is present
in the set. sig MUST be in [0, 64).

r[signal.union]
signal_union MUST return the bitwise OR of two signal sets. Union MUST
be commutative and associative. Union with empty set MUST be identity.

r[signal.intersect]
signal_intersect MUST return the bitwise AND of two signal sets.
Intersect MUST be commutative and associative. Intersect with full set
MUST be identity.

r[signal.complement]
signal_complement MUST return the bitwise NOT of a signal set.
Complementing twice MUST return the original set.

r[signal.deliverable]
signal_deliverable MUST return pending & !blocked — the signals that
are pending and not masked. The result MUST be a subset of pending.
The result MUST have no bits in common with blocked.

## Process State

r[process-state.is-valid]
is_valid_state MUST return true if and only if state is one of:
0 (Created), 1 (Ready), 2 (Running), 3 (Blocked), 4 (Terminated).

r[process-state.can-transition]
can_transition MUST return true for exactly these state pairs:
Created->Ready, Ready->Running, Running->Ready (preempt),
Running->Blocked (wait), Running->Terminated (exit),
Blocked->Ready (wakeup). All other transitions MUST return false.

r[process-state.is-terminal]
is_terminal MUST return true if and only if state == 4 (Terminated).
A terminal state MUST have no valid outgoing transitions.

r[process-state.is-schedulable]
is_schedulable MUST return true if and only if state == 1 (Ready).
Only ready processes can be selected by the scheduler.

## Page

r[page.frame-number]
frame_number MUST return addr / page_size. page_size MUST be nonzero.
The result times page_size MUST NOT exceed addr.

r[page.offset]
page_offset MUST return addr % page_size. page_size MUST be nonzero.
The result MUST be strictly less than page_size.

r[page.is-aligned]
is_page_aligned MUST return true if and only if addr % page_size == 0.
page_size MUST be nonzero. Zero MUST always be page-aligned.

r[page.start-address]
page_start MUST return frame * page_size. The multiplication MUST NOT
overflow. The result MUST be page-aligned.

r[page.pages-needed]
pages_needed MUST return the minimum number of pages required to cover
size bytes. pages_needed(0, ps) MUST be 0. For size > 0, the result
times page_size MUST be >= size. page_size MUST be nonzero.

## Fencing

r[fencing.issue-token]
issue_token MUST return current + 1. The current token MUST be less
than u64::MAX (precondition).

r[fencing.is-valid-write]
is_valid_write MUST return true if and only if request_token >= fence.
A write is valid when its token is at least as recent as the fence.

r[fencing.is-stale]
is_stale MUST return true if and only if token < fence.
is_stale and is_valid_write MUST be exact complements for the same
token and fence values.

r[fencing.issuance-strictly-increasing]
issue_token(t) MUST be strictly greater than t. Successive issuances
produce a strictly monotonic sequence.

r[fencing.no-false-rejection]
A freshly issued token MUST be valid against the fence it was issued
from. issue_token(fence) produces a token where
is_valid_write(token, fence) is true.

r[fencing.stale-implies-not-valid]
is_stale(token, fence) MUST imply !is_valid_write(token, fence) and
vice versa. The two predicates partition all (token, fence) pairs.

r[properties.fencing-is-strict-advance]
issue_token(t) MUST satisfy monotonic::is_strict_advance(t, issue_token(t)).
Fencing token issuance is a strict monotonic advance.

r[properties.fencing-stale-is-not-advance]
is_stale(token, fence) MUST imply !is_valid_write(token, fence).
A stale token cannot pass the validity check.

## Ballot

r[ballot.can-grant-vote]
can_grant_vote MUST return true only if the candidate_term is newer than voted_term (or same term and same candidate), AND the candidate's log epoch is >= our log epoch.

r[ballot.grant-vote]
grant_vote MUST return a VoteState with voted_term set to candidate_term and voted_for set to candidate.

r[ballot.has-won-election]
has_won_election MUST return true if and only if votes_received constitutes a quorum of cluster_size (delegates to quorum::is_quorum).

## Vector Clock

r[vector-clock.happens-before]
happens_before MUST return true if and only if all components of a are <= corresponding components of b, AND at least one component of a is strictly less than the corresponding component of b. Clocks MUST have equal length.

r[vector-clock.concurrent]
concurrent MUST return true if and only if neither clock happens-before the other AND the clocks differ in at least one component.

r[vector-clock.merge]
merge MUST return a clock where each component is the maximum of the corresponding components of the two inputs. The result length MUST equal the input lengths.

r[vector-clock.increment]
increment MUST return a clock identical to the input except at position idx, which MUST be incremented by 1. The component at idx MUST be less than u64::MAX (precondition).

## Lease

r[lease.effective-expiry]
effective_expiry MUST return min(expiry, revoked_at) if the lease is revoked, or expiry otherwise.

r[lease.is-valid]
is_valid MUST return true if and only if now >= start AND now < effective_expiry.

r[lease.grant]
grant MUST return a Lease with the given resource_id, start, expiry, and is_revoked set to false. start MUST be less than expiry.

r[lease.renew]
renew MUST return a Lease with the same fields except expiry set to new_expiry. new_expiry MUST be greater than the current expiry. The lease MUST NOT be revoked.

r[lease.revoke]
revoke MUST return a Lease with is_revoked set to true and revoked_at set to the given timestamp.

## Two-Phase Commit

r[two-phase.can-commit]
can_commit MUST return true if and only if votes is non-empty AND every vote is Some(true).

r[two-phase.must-abort]
must_abort MUST return true if and only if any vote is Some(false).

r[two-phase.decide]
decide MUST return 1 (commit) if can_commit is true, 2 (abort) if must_abort is true, or 0 (pending) otherwise. can_commit and must_abort MUST be mutually exclusive.

## Circuit Breaker

r[circuit-breaker.is-valid-state]
is_valid_state MUST return true if and only if state is CLOSED (0), OPEN (1), or HALF_OPEN (2).

r[circuit-breaker.next-state]
next_state MUST return OPEN if current is CLOSED and failure_count >= threshold, HALF_OPEN if current is OPEN and elapsed >= timeout, CLOSED if current is HALF_OPEN and probe_succeeded, OPEN if current is HALF_OPEN and !probe_succeeded, or the current state otherwise. The result MUST always be a valid state.

r[circuit-breaker.allow-request]
allow_request MUST return true if state is CLOSED, false if state is OPEN, and !probe_active if state is HALF_OPEN.

## Backoff

r[backoff.compute-backoff]
compute_backoff MUST return min(base * 2^attempt, cap). If base is 0, the result MUST be 0. The result MUST never exceed cap.

r[backoff.jitter-range]
jitter_range MUST return (0, backoff_value). The lower bound MUST be 0 and the upper bound MUST equal the backoff value.

## Nix Version

r[nix-version.compare-versions]
compare_versions MUST compare two version strings component-by-component, with numeric components beating alphabetic components, returning -1, 0, or 1.

r[nix-version.parse-drv-name]
parse_drv_name MUST split a derivation name at the first hyphen-digit boundary.

## Nix String

r[nix-string.string-length]
string_length MUST return the byte length of a string.

r[nix-string.substring]
substring MUST extract a substring with clamping semantics for out-of-bounds indices.

r[nix-string.replace-strings]
replace_strings MUST perform left-to-right non-overlapping replacement of patterns.

## Nix Attribute Set

r[nix-attrset.attr-names]
attr_names MUST extract keys from a sorted attribute set.

r[nix-attrset.attr-values]
attr_values MUST extract values from an attribute set.

r[nix-attrset.has-attr]
has_attr MUST check if a key exists in the attribute set.

r[nix-attrset.get-attr]
get_attr MUST look up a value by key, returning None if the key is absent.

r[nix-attrset.merge]
merge MUST merge two attribute sets, with the right set winning on key conflicts.

r[nix-attrset.remove-attrs]
remove_attrs MUST filter out specified keys from the attribute set.

r[nix-attrset.intersect-attrs]
intersect_attrs MUST keep only shared keys, using values from the second set.

r[nix-attrset.list-to-attrs]
list_to_attrs MUST sort and deduplicate key-value pairs to construct an attribute set.

## Nix List

r[nix-list.length]
length MUST return the length of a list.

r[nix-list.elem-at]
elem_at MUST get the element at a given index.

r[nix-list.head]
head MUST return the first element of a list.

r[nix-list.tail]
tail MUST return all elements except the first.

r[nix-list.elem]
elem MUST check if an element is present in the list.

r[nix-list.gen-list]
gen_list MUST generate a list [0..n-1] for a given n.

r[nix-list.concat-lists]
concat_lists MUST flatten a list of lists into a single list.

## Nix Closure

r[nix-closure.generic-closure]
generic_closure MUST compute the transitive closure with unique keys.

## CRDT

r[crdt.g-counter-merge]
G-Counter merge MUST return a counter where each component is the maximum of the corresponding components from both inputs. Merge MUST be commutative, associative, and idempotent.

r[crdt.g-counter-value]
G-Counter value MUST return the sum of all components. The value MUST be monotonic: merge or increment operations never decrease the value.

r[crdt.g-counter-increment]
G-Counter increment MUST increase the component at the given node_id by 1. The component MUST be less than u64::MAX (precondition). Only the specified component changes.

r[crdt.pn-counter-merge]
PN-Counter merge MUST merge the positive and negative G-Counters independently. The result MUST preserve the merge properties: commutativity, associativity, idempotency.

r[crdt.lww-merge]
LWW-Register merge MUST return the register with the higher timestamp. When timestamps are equal, the merge MUST be deterministic (e.g., by comparing values). Merge MUST be commutative, associative, and idempotent.

## Bloom Filter

r[bloom.insert]
Bloom filter insert MUST set all hash-derived bit positions for the key to 1. The operation MUST be idempotent - inserting the same key multiple times has no additional effect.

r[bloom.query]
Bloom filter query MUST return true if all hash-derived bit positions for the key are set. MUST guarantee no false negatives: if insert(key) was called, query(key) MUST return true.

## Consistent Hashing

r[consistent-hash.lookup]
Consistent hash lookup MUST return the first node in the ring whose hash position is >= the key hash (wrapping to first node if no such node exists). Empty rings MUST NOT be allowed.

## Gossip

r[gossip.infection-lower-bound]
Gossip infection MUST spread to at least (fanout^rounds) nodes, capped at total population. The infection MUST be monotonic - more rounds never reduce the infected count.

r[gossip.convergence-rounds]
Gossip convergence MUST complete within ceil(log_fanout(n)) rounds for full infection of n nodes. The bound MUST account for the fanout parameter and network size.

## Causal Order

r[causal-order.is-causally-ready]
A message is causally ready if its vector timestamp is <= the recipient's vector clock plus 1 at the sender position. This ensures causal delivery order.

r[causal-order.is-consistent-cut]
A cut is consistent if for any event in the cut, all causally preceding events are also in the cut. Uses vector clock happens-before relation.

r[causal-order.cut-join]
Cut join MUST return the component-wise maximum (vector clock merge) of two cuts. The result preserves consistency if both inputs are consistent cuts.

## Cost Tracker

r[cost-tracker.cost-non-negative]
compute_cost MUST return a non-negative value for any inputs. Zero tokens or zero price MUST yield zero cost.

r[cost-tracker.cost-monotonic]
Accumulating a turn cost MUST produce a total >= the previous total (monotonically non-decreasing).

r[cost-tracker.cost-additive]
Accumulating a turn cost MUST produce exactly total_before + turn_cost. Two accumulations MUST compose associatively.

r[cost-tracker.threshold-fires-once]
A threshold crossing MUST fire when prev < limit AND current >= limit. It MUST NOT fire when already crossed (prev >= limit) or not yet reached (current < limit). Over a monotonic sequence, at most one adjacent pair crosses any given threshold.

r[cost-tracker.milestone-count]
Milestone count MUST equal floor(current/interval) - floor(prev/interval). Count MUST be zero when both values fall in the same interval. Milestones MUST compose additively: count(a,c) == count(a,b) + count(b,c).

r[cost-tracker.milestone-non-negative]
Milestone count MUST be non-negative when current >= prev and interval > 0.

r[cost-tracker.budget-status-exhaustive]
Budget status MUST map every (soft, hard, total) triple to exactly one of NoBudget, Ok, Warning, or Exceeded. No limits means NoBudget. Under all defined limits means Ok. Between soft and hard means Warning. At or above hard means Exceeded.

r[cost-tracker.budget-status-monotone]
Budget status MUST be monotone: increasing total never moves status backward. Warning is stable (stays Warning or advances to Exceeded). Exceeded is absorbing.

r[cost-tracker.hard-budget-overrides-hint]
Model selection MUST return smol (cheapest) when cost >= hard_limit, regardless of user hint or complexity score.

r[cost-tracker.hint-overrides-score]
When under hard limit, a user hint MUST override the complexity score in model selection.

r[cost-tracker.score-thresholds-partition]
Score thresholds MUST partition the score range into smol (< low), default ([low, high]), and slow (> high) with no gaps.

r[cost-tracker.soft-budget-demotes]
Soft budget pressure (score/2) MUST never promote — adjusted score <= original score. The resulting role MUST be <= the role without soft pressure.

r[cost-tracker.score-non-negative]
Complexity score MUST be non-negative when all weights are non-negative (u64 inputs guarantee this).

## Cross-module: Cost Tracker

r[properties.accumulate-is-advance]
Cost accumulation is a monotonic advance: total_after >= total_before.

r[properties.threshold-crossing-irreversible]
Once a threshold is crossed in a monotonic sequence, it stays crossed — no later adjacent pair can cross the same threshold.

r[properties.budget-warning-implies-not-ok]
Warning status implies we are past Ok — the two states are mutually exclusive.

r[properties.budget-exceeded-stays-exceeded]
Exceeded is absorbing under increasing totals — once exceeded, always exceeded.

## Environment Scrubbing

r[env-scrub.suffix-case-insensitive]
Secret suffix detection MUST be case-insensitive. ASCII uppercasing MUST be idempotent and length-preserving.

r[env-scrub.blocked-keys-absent]
Every key in the explicit blocklist MUST be identified as secret. Byte sequence equality MUST be used for exact matching.

r[env-scrub.suffix-matched-absent]
Every key whose uppercased form ends with a secret suffix (_SECRET, _TOKEN, _PASSWORD, _CREDENTIALS, _API_KEY, _APIKEY, _PRIVATE_KEY) MUST be identified as secret.

r[env-scrub.sandbox-marker-present]
The sandbox marker key (CLANKERS_SANDBOX) MUST NOT be blocked and MUST NOT match any secret suffix. It is not a secret.

r[env-scrub.output-size-bounded]
The filter count MUST be bounded by the input size. Filtering can only remove entries, never add them.

r[env-scrub.filter-idempotent]
Re-filtering already-filtered keys MUST keep all of them. If every key in the list is non-secret, the count equals the list length.

r[env-scrub.blocklist-disjoint-safe]
Common safe variable names (PATH, HOME) MUST NOT be in the blocklist and MUST NOT match any secret suffix.

r[env-scrub.empty-yields-marker]
Filtering an empty key list MUST yield zero safe entries.

## Cross-module: Environment Scrubbing

r[properties.to-upper-preserves-suffix]
If a key ends with an uppercase suffix, uppercasing the key preserves the suffix — uppercase suffixes are fixed points of to_upper.

r[properties.secret-implies-not-safe]
A secret key is never safe — is_secret and !is_secret are mutually exclusive.

## Scope Coverage

r[scope.coverage-reflexive]
A scope MUST cover itself. Coverage is defined as starts-with on byte sequences: covers(scope, target) iff target starts with scope.

r[scope.coverage-transitive]
If scope A covers B and B covers C, then A MUST cover C. Prefix containment is transitive.

r[scope.coverage-rejects-non-prefix]
A scope MUST NOT cover a target that does not start with the scope prefix.

r[scope.coverage-narrowing-monotone]
If parent scope covers child scope, then everything the child covers is also covered by the parent. Narrowing is monotone.

r[scope.compound-requires-all-fields]
Compound scoping (mount + prefix) MUST require all fields to match. Mount mismatch MUST deny regardless of prefix match.

r[scope.optional-none-unconstrained]
An optional scope constraint of None MUST allow any target. None is unconstrained.

r[scope.optional-some-none-denies]
An optional scope constraint of Some MUST deny when the target is None.

r[scope.optional-some-some-delegates]
When both cap and target are Some, optional coverage MUST delegate to covers.

## Access Mode Ordering

r[access-mode.valid-mode]
Valid access modes are READ (0), WRITE (1), DELETE (2), and FULL (3). The set is exhaustive.

r[access-mode.order-reflexive]
Every mode MUST be ≤ itself. The ordering is reflexive.

r[access-mode.order-transitive]
The mode ordering MUST be transitive. READ, WRITE, DELETE are each ≤ FULL but pairwise incomparable.

r[access-mode.order-antisymmetric]
The mode ordering MUST be antisymmetric. If a ≤ b and b ≤ a then a == b.

r[access-mode.permits-ordering]
mode_permits(cap, op) MUST return true iff the operation mode is ≤ the capability mode. Read-only MUST deny write and delete. Full MUST allow all.

r[access-mode.no-escalation]
Delegation MUST require child mode ≤ parent mode. Escalation (child > parent) MUST be denied. Failed delegation MUST also fail mode_permits.

## Pattern Matching

r[pattern.wildcard-matches-all]
A wildcard pattern MUST match any input value. The wildcard sentinel is -1.

r[pattern.exact-match-reflexive]
A singleton pattern MUST match its own element.

r[pattern.non-member-rejected]
If a value is not in the pattern items and the pattern is not wildcard, matching MUST fail.

r[pattern.containment-subset]
pattern_contains(p1, p2) MUST hold iff p1 is wildcard or every item in p2 appears in p1. Containment implies match propagation.

r[pattern.wildcard-contains-any]
A wildcard pattern MUST contain any other pattern.

r[pattern.non-wildcard-cannot-contain-wildcard]
A non-wildcard pattern that does not contain the sentinel MUST NOT contain the wildcard pattern.

r[pattern.containment-reflexive]
Every pattern MUST contain itself.

r[pattern.containment-transitive]
Pattern containment MUST be transitive for well-formed patterns (non-wildcard patterns must not contain the sentinel value).

## Glob Matching

r[glob.wildcard-matches-all]
A glob pattern of exactly `"*"` MUST match any input.

r[glob.exact-match]
A glob pattern with no `*` MUST match only exact byte equality and reject different inputs.

r[glob.prefix-match]
A glob pattern with trailing `*` MUST match inputs starting with the prefix before the `*`.

r[glob.multi-segment-match]
A glob pattern with multiple `*` wildcards MUST match inputs where segments appear in order: first anchored at start, last at end, middle anywhere.

r[glob.non-match-rejected]
A glob pattern MUST reject inputs that don't satisfy the pattern structure.

## Variant Subsumption

r[subsumption.identity]
Every variant MUST subsume itself. Subsumption is reflexive and forms a partial order (reflexive, antisymmetric, transitive).

r[subsumption.acyclic]
Subsumption MUST be acyclic. If a subsumes b and b subsumes a, then a == b. Subsumption MUST be transitive.

r[subsumption.admin-full]
Full MUST subsume Read, Write, Delete, and Watch. These are the admin-to-leaf edges in the subsumption DAG.

r[subsumption.soundness]
If variant a does not subsume variant b, then no capability of type a can authorize operations of type b. Leaf variants MUST NOT subsume Full. Independent variants (Prompt, ToolUse, Delegate) MUST NOT subsume each other.

## Authorization Composition

r[authorize.decomposition]
authorizes(cap, op) MUST equal subsumes(cap.variant, op.variant) && covers(cap.scope, op.target) && mode_permits(cap.mode, op.mode). Each factor is independently verified.

r[authorize.rejects-scope-failure]
Scope mismatch MUST deny even with matching variant and mode.

r[authorize.rejects-mode-failure]
Mode mismatch MUST deny even with matching variant and scope.

r[authorize.rejects-variant-failure]
Variant mismatch MUST deny even with matching scope and mode.

r[authorize.all-match]
When all three factors match, authorization MUST succeed.

r[authorize.compound-scope]
Authorization with compound scope MUST require mount equality and prefix coverage. Mount mismatch MUST deny.

r[authorize.optional-constraint]
Authorization with optional constraint: None cap constraint MUST NOT block. Some cap with None target MUST block.

## Delegation Containment

r[containment.decomposition]
contains(parent, child) MUST equal subsumes(parent.variant, child.variant) && covers(parent.scope, child.scope) && mode_le(child.mode, parent.mode).

r[containment.no-scope-widening]
If the parent scope does not cover the child scope, containment MUST fail.

r[containment.no-mode-escalation]
If the child mode is not ≤ the parent mode, containment MUST fail.

r[containment.no-variant-escalation]
If the parent variant does not subsume the child variant, containment MUST fail.

r[containment.transitive]
Containment MUST be transitive, following from transitivity of each factor.

r[containment.self-contained]
Every capability MUST contain itself.

r[containment.attenuation-allowed]
A child with narrower scope and equal-or-lower mode MUST be contained by the parent.

r[containment.compound-scope]
Compound containment MUST require mount equality. Mount mismatch MUST deny.

r[containment.optional-constraint]
Optional containment: parent None MUST contain any child. Parent Some MUST block child None.

## Delegation Chain

r[delegation.depth-bounded]
Chain depth MUST NOT exceed MAX_DELEGATION_DEPTH (8). Depths 0-8 are valid, 9+ are invalid.

r[delegation.root-depth-zero]
Root tokens MUST have depth 0.

r[delegation.depth-increments]
Each delegation level MUST increment depth by exactly 1.

r[delegation.delegate-gate]
Delegation MUST require the parent to hold the Delegate capability. Without Delegate, delegation is denied.

r[delegation.chain-attenuation]
Each parent-to-child transition MUST attenuate: parent variant subsumes child, child mode <= parent mode. Escalation on either axis MUST be rejected.

r[delegation.chain-valid]
A full chain is valid iff: non-empty, root at depth 0, last depth within bounds, and every consecutive pair passes level validation.

## Token Timestamps

r[token-timestamp.expiration-tolerance]
A token is expired when now > expires_at + tolerance. Saturating addition MUST prevent overflow. Within the tolerance window, the token MUST NOT be expired. Zero tolerance means expired iff now > expires_at.

r[token-timestamp.fresh-token-valid]
A token checked at its issued_at time with issued_at <= expires_at MUST NOT be expired.

r[token-timestamp.expiration-monotonic]
Once a token is expired, it MUST stay expired as time advances.

r[token-timestamp.future-rejection]
A token with issued_at > now + tolerance MUST be rejected as from the future. Clock skew within tolerance MUST be accepted.

r[token-timestamp.refresh-threshold]
A token needs refresh when remaining lifetime <= 20% of total lifetime. Expired tokens always need refresh. Fresh tokens (at issued_at) MUST NOT need refresh.

r[token-timestamp.token-valid]
A token is temporally valid iff not expired and not from the future. A well-formed token checked at its issued_at time MUST be valid.

## Cross-module: UCAN Authorization

r[properties.delegation-safety]
If a parent contains a capability and that capability authorizes an operation, then the parent also authorizes the operation. Delegation cannot create new access.

r[properties.auth-implies-subsumes]
Authorization implies all three factors hold: variant subsumption, scope coverage, and mode permission.

r[properties.containment-implies-factors]
Containment implies all three factors hold: variant subsumption, scope coverage, and mode ordering.

r[properties.token-valid-at-issue]
A well-formed token is temporally valid at its issued_at time.

r[properties.chain-valid-implies-depth-bounded]
A valid delegation chain has depth within MAX_DELEGATION_DEPTH (8). Chains exceeding the bound are rejected.

## Region Arithmetic

r[region.contains-addr]
A region (base, size) MUST contain address x iff base <= x < base + size.
Empty regions (size == 0) MUST contain no address. Overflow: base + size
MUST NOT exceed u64::MAX + 1 (enforced via requires).

r[region.end-addr]
The exclusive end address of a region MUST equal base + size.

r[region.is-empty]
A region is empty iff its size is zero.

r[region.disjoint]
Two regions MUST be disjoint iff they share no address. Disjointness
MUST be symmetric.

r[region.contains-region]
Region A contains region B iff every address in B is also in A.
Containment MUST be reflexive and transitive.

r[region.adjacent]
Two regions MUST be adjacent iff one ends exactly where the other
begins, with no gap and no overlap. Adjacency MUST be symmetric.

r[region.merge]
Merging two adjacent regions MUST produce a region covering exactly
the combined span. Split-then-merge MUST be a roundtrip.

r[region.split]
Splitting a region at an offset within (0, size) MUST produce two
non-empty, non-overlapping, adjacent sub-regions whose sizes sum
to the original size.

## Cross-module: Region ↔ Interval

r[properties.region-contains-agrees-interval]
region_contains_addr(base, size, x) MUST agree with interval::contains(base, base+size, x)
for valid (non-overflowing) regions.

r[properties.region-disjoint-agrees-interval]
regions_disjoint(a_base, a_size, b_base, b_size) MUST agree with
!interval::overlaps(a_base, a_end, b_base, b_end) for valid regions.

r[properties.region-adjacent-agrees-interval]
region_adjacent(a_base, a_size, b_base, b_size) MUST agree with
interval::adjacent(a_base, a_end, b_base, b_end) for valid regions.

## Slab Allocation

r[slab.slot-index]
slot_index MUST return (addr - base) / slot_size for addresses within the slab region.
The slot index MUST be bounded: slot_index < slab_capacity.

r[slab.slot-offset]
slot_offset MUST return (addr - base) % slot_size, the byte position within a slot.

r[slab.slot-address]
slot_address MUST return base + index * slot_size. On aligned addresses,
slot_address(slot_index(addr)) MUST equal addr (roundtrip).

r[slab.is-slot-aligned]
An address is slot-aligned iff its slot_offset is zero.

r[slab.slab-capacity]
slab_capacity MUST return region_size / slot_size — the number of whole
slots that fit. Zero slot_size or region_size MUST yield zero.

r[slab.generation-valid]
A generation is valid if it is less than a maximum generation limit.
generation_increment MUST produce a valid successor. Validity MUST be
monotonic: if gen is invalid, gen+1 is also invalid.

r[slab.handle-pack]
handle_pack MUST combine an index and generation into a single u64.
handle_unpack_index and handle_unpack_generation MUST recover the
original values (roundtrip).

## Cross-module: Slab ↔ Page

r[properties.slab-slot-is-page-frame]
slot_index(addr, base, slot_size) with base=0 MUST agree with
page::frame_number(addr, slot_size).

r[properties.slab-capacity-is-pages-dual]
For exact multiples: slab_capacity(n * s, s) == n, matching the
inverse relationship with pages_needed.

## Buddy System

r[buddy.block-size]
block_size(order) MUST return 2^order. block_size(0) == 1.

r[buddy.buddy-of]
buddy_of(addr, order) MUST return the buddy address by flipping bit `order`
in addr. buddy_of MUST be an involution: buddy_of(buddy_of(addr, order), order) == addr.

r[buddy.order-for-size]
order_for_size(size) MUST return the minimum order k such that 2^k >= size.
For exact powers of two, order_for_size(2^k) == k.

r[buddy.can-coalesce]
Two blocks can coalesce iff they are buddies at the same order and both
are aligned to that order. Coalescing produces a block at order+1.

r[buddy.parent-block]
parent_block(addr, order) MUST return the start of the parent block at
order+1. The parent is aligned to order+1.

r[buddy.is-aligned-to-order]
An address is aligned to order iff addr % 2^order == 0.

## Cross-module: Buddy ↔ Bitwise/Integer

r[properties.buddy-uses-toggle]
buddy_of(addr, order) MUST equal bitwise::toggle_bit(addr, order).

r[properties.block-size-is-pow2]
block_size(order) MUST equal integer::pow2_spec(order).

## Watermark Allocation

r[watermark.remaining]
watermark_remaining(mark, base, capacity) MUST return capacity - (mark - base).
When mark == base, remaining MUST equal capacity.

r[watermark.fits]
watermark_fits(mark, size, base, capacity) MUST return true iff an allocation
of `size` bytes fits in the remaining space.

r[watermark.alloc]
watermark_alloc(mark, size) MUST return the new mark after allocating `size` bytes
(mark + size). The allocation occupies [mark, mark + size).

r[watermark.reset]
watermark_reset returns the base address. After reset, remaining == capacity.

r[watermark.is-empty]
The watermark is empty (no allocations) when mark == base.

r[watermark.monotonic]
Allocation only moves the mark forward: new_mark >= old_mark.

r[watermark.non-overlap]
Two successive allocations from the same watermark return disjoint regions.

r[watermark.capacity-conservation]
remaining + used == capacity, where used = mark - base.

## Cross-module: Watermark

r[properties.watermark-advance-is-monotonic]
watermark_alloc advances the mark, which satisfies monotonic::is_advance.

r[properties.watermark-alloc-disjoint-regions]
Two allocations from sequential marks produce regions that are
region::regions_disjoint.

## Eviction Score

r[eviction-score.weighted-score]
weighted_score MUST compute the weighted sum of four factor scores
using integer weights that sum to 100. Each factor score MUST be in
the range [0, 100]. The result MUST equal
w1*s1 + w2*s2 + w3*s3 + w4*s4.

r[eviction-score.full-score]
full_score MUST multiply the weighted base score by a priority
multiplier and a tier multiplier. Lower scores are evicted first.

r[eviction-score.weights-valid]
Eviction weights are valid when all four are in [0, 100] and their
sum equals 100.

r[eviction-score.priority-multiplier]
priority_multiplier MUST return 1 for Low (0), 2 for Normal (1),
4 for High (2), 20 for Critical (3). Higher priority entries get
higher scores (harder to evict).

r[eviction-score.tier-multiplier]
tier_multiplier MUST return 1 for Cold (0), 2 for Warm (1),
4 for Hot (2). Hotter entries get higher scores (harder to evict).

## Tier

r[tier.classify]
classify_tier MUST return Hot (2) if access_count >= hot_threshold,
Warm (1) if access_count >= warm_threshold, Cold (0) otherwise.
Thresholds MUST be ordered: warm_threshold <= hot_threshold.

r[tier.promote]
promote MUST return the next higher tier: Cold (0) becomes Warm (1),
Warm (1) becomes Hot (2), Hot (2) stays Hot (2).

r[tier.demote]
demote MUST return the next lower tier: Hot (2) becomes Warm (1),
Warm (1) becomes Cold (0), Cold (0) stays Cold (0).

r[tier.should-demote]
should_demote MUST return true when idle_time >= cold_idle_threshold
and current tier is above Cold.

## Markov

r[markov.confidence]
confidence_meets MUST return true when count * 100 >=
total * threshold_pct. This is the integer equivalent of
(count / total) >= (threshold_pct / 100).

r[markov.record-bounded]
After recording a transition, the count for that transition
MUST NOT exceed the total. The total MUST NOT exceed the
configured maximum.

r[markov.prediction-valid]
A prediction is valid when count > 0 and count <= total
and total > 0. The confidence ratio count/total MUST be
in (0, 1].

r[markov.best-prediction]
best_prediction MUST return the count of the most frequent
transition. If multiple transitions tie, any of them MAY be
returned.

## Type Lattice

r[type-lattice.can-convert]
can_convert MUST return true if and only if a value of the source
type can be converted to the target type. The conversion graph
MUST be reflexive: can_convert(t, t) for all valid types.

r[type-lattice.any-top]
ANY MUST act as the top element: can_convert(ANY, t) and
can_convert(t, ANY) MUST both return true for all valid types.

r[type-lattice.string-source]
STRING MUST be convertible to every other valid type.
can_convert(STRING, t) MUST return true for all valid t.

r[type-lattice.numeric-widening]
INTEGER MUST be convertible to FLOAT. FLOAT MUST be convertible
to INTEGER. Both MUST be convertible to STRING.

r[type-lattice.temporal-widening]
DATE MUST be convertible to DATETIME and vice versa. Both MUST
be convertible to STRING.

## Cross-Module Properties (Wave 1)

r[properties.signal-add-is-set-bit]
signal_add(set, sig) MUST equal bitwise::set_bit(set, sig) for all
set and sig < 64. The signal set-add operation is the same as setting
a bit.

r[properties.signal-remove-is-clear-bit]
signal_remove(set, sig) MUST equal bitwise::clear_bit(set, sig) for
all set and sig < 64. Signal removal is bit clearing.

r[properties.signal-contains-is-bit-at]
signal_contains(set, sig) MUST equal bitwise::bit_at(set, sig) for
all set and sig < 64. Signal membership test is bit inspection.

r[properties.permission-set-check-roundtrip]
has_permission(set_permission(mode, who, perm), who, perm) MUST be
true for all valid mode, who <= 2, perm <= 2. Setting a permission
then checking it always succeeds.

r[properties.permission-set-independence]
Setting permission (w1, p1) MUST NOT affect has_permission(mode, w2, p2)
when (w1, p1) != (w2, p2) and both are valid. Different permission
bits are independent.

r[properties.pages-needed-is-div-ceil]
pages_needed(size, ps) MUST equal div_ceil(size, ps) for all size
and ps > 0. Page count and ceiling division are the same computation.

r[properties.page-aligned-is-align-down-fixpoint]
is_page_aligned(addr, ps) MUST equal (align_down(addr, ps) == addr)
for ps > 0. An aligned address is a fixpoint of align_down.

r[properties.lease-valid-implies-not-expired]
For a non-revoked lease, is_valid(lease, now) MUST imply
!is_expired(lease.expiry, now). Lease validity is strictly stronger
than non-expiration.

r[properties.tier-score-monotonic]
If count_a <= count_b and warm_t <= hot_t, then
tier_multiplier(classify_tier(count_a, w, h)) MUST be <=
tier_multiplier(classify_tier(count_b, w, h)). Higher access count
never produces a lower tier multiplier.

r[properties.backoff-in-range]
compute_backoff(base, attempt, cap) MUST be <= cap for all inputs.
The backoff value is always bounded by the cap.

r[properties.approval-majority-implies-quorum]
If required >= majority(total) and is_merge_allowed(valid, required, true),
then is_quorum(valid, total). Meeting an approval threshold at least
as strong as majority guarantees a quorum.

r[properties.lease-renewal-is-advance]
For a non-revoked lease, effective_expiry(renew(lease, new_expiry))
>= effective_expiry(lease) when new_expiry > lease.expiry. Lease
renewal satisfies monotonic::is_advance on the effective expiry.

r[properties.two-phase-unanimity-implies-quorum]
If can_commit(votes) is true, then is_quorum(n, n) holds where
n = votes.len(). Unanimity (2PC requirement) implies any quorum
threshold.

r[properties.expires-at-monotonic-is-advance]
If ttl_a <= ttl_b, then is_advance(expires_at(now, ttl_a),
expires_at(now, ttl_b)). Longer TTL produces later (or equal)
expiration, satisfying monotonic advance.

r[properties.cow-clone-is-drain-start]
ref_count_after_clone(n) MUST equal after_start_op(n) for all n.
CoW cloning and drain start are the same increment-with-saturation.

r[properties.cow-drop-is-drain-finish]
ref_count_after_drop(n) MUST equal after_finish_op(n) for all n.
CoW drop and drain finish are the same decrement-with-floor.

## Cross-Module Properties (Wave 2)

r[properties.type-lattice-reflexive]
can_convert(t, t) MUST be true for all valid types. Type conversion
is reflexive — every type converts to itself.

r[properties.type-lattice-any-top]
can_convert(t, ANY) and can_convert(ANY, t) MUST both be true for
all valid types. ANY is the universal converter — top element in
both directions.

r[properties.circuit-breaker-preserves-validity]
next_state applied to a valid state MUST produce a valid state.
State transitions never leave the valid state space.

r[properties.process-terminal-blocks-circuit-open-blocks]
process_state::is_terminal(4) implies no outgoing transitions, and
circuit_breaker::allow_request(OPEN) is false. Both state machines
have a "blocked" state that prevents forward progress.

r[properties.bitmap-signal-agree]
For a single-word bitmap (num_bits <= 64), bitmap.get_spec(pos) MUST
equal signal_contains_spec(bitmap.words[0], pos). The bitmap's bit
decomposition agrees with signal's direct bit check.

r[properties.slab-slot-in-region]
slot_address(i, base, ss) MUST be contained in region (base, capacity)
when i < slab_capacity(capacity, ss) and slot_size > 0. Every valid
slab slot falls within the backing memory region.

r[properties.pressure-eviction-pipeline]
classify_level(count, capacity, capacity) >= 2 implies
should_evict(count, capacity). Critical memory pressure triggers
the eviction decision.

r[properties.eviction-score-cold-minimum]
tier_multiplier(COLD) MUST be the minimum multiplier value. Cold
entries get the lowest protection score, making them first to evict.

## Cross-Module Properties (Wave 3)

r[properties.evictable-count-is-saturating-sub]
evictable_count(count, protected) MUST equal saturating_sub(count, protected).
Both compute max(a - b, 0) — the eviction module's counter agrees with
the arithmetic module's generic subtraction.

r[properties.gossip-infection-bounded]
infection_lower_bound(n, fanout, rounds) MUST be <= n for all inputs.
Gossip infection never exceeds the cluster size.

r[properties.gossip-infection-monotonic-rounds]
If rounds_a <= rounds_b, then infection_lower_bound(n, f, rounds_a)
<= infection_lower_bound(n, f, rounds_b). More rounds never reduce
the number of infected nodes.

r[properties.acid-empty-commit-identity]
commit(state, empty) MUST equal state for all states. Committing no
writes is a no-op.

r[properties.priority-multiplier-monotonic]
If a <= b and both are valid priorities (0-3), then
priority_multiplier(a) MUST be <= priority_multiplier(b).
Higher priority always produces a higher or equal multiplier.

r[properties.full-score-bounded]
With valid weights (sum 100, each <= 100), factor scores in [0, 100],
priority in [0, 3], and tier in [0, 2], full_score MUST be <= 800000.
The eviction score has a known maximum.

r[properties.consistent-hash-lookup-in-ring]
For a non-empty ring, lookup(ring, key_hash) MUST equal some ring[i].1.
The lookup result is always a node that appears in the ring.

r[properties.bloom-insert-length-preserved]
insert_spec(bits, positions) MUST have the same length as bits.
Bloom filter insertion never changes the bit array size.

## Layout Constraint

r[layout-constraint.satisfies]
satisfies_constraint MUST return true when the resolved size matches the
constraint: Fixed(n) requires resolved == n, Proportion always satisfies,
Min(n) requires resolved >= n, MinMax(lo, hi) requires lo <= resolved <= hi.

r[layout-constraint.clamp]
clamp_to_constraint MUST produce a value satisfying the constraint.
Clamping MUST be idempotent: clamp(clamp(x)) == clamp(x). Clamping a
value that already satisfies the constraint MUST be a no-op.

r[layout-constraint.conservation]
The sum of resolved sizes plus inter-item gaps MUST NOT exceed the
available total space. sum(sizes) + (n-1) * gap <= total.

r[layout-constraint.length-preservation]
The resolver output sequence MUST have the same length as the input
constraint sequence.

r[layout-constraint.zero-total]
When total available space is zero, proportional constraints MUST
resolve to zero.

r[layout-constraint.fixed-conservation]
When all constraints are Fixed, the sum of resolved sizes MUST equal
the sum of the fixed values exactly.

## Tiling

r[tiling.rect-end]
right(x, width) MUST return x + width. bottom(y, height) MUST return
y + height. Both use widened arithmetic to avoid overflow.

r[tiling.has-area]
has_area MUST return true if and only if both width > 0 and height > 0.

r[tiling.disjoint]
rects_disjoint MUST return true when two rectangles do not share any
point. Disjointness MUST be symmetric. A zero-area rect MUST be
disjoint from everything.

r[tiling.contained]
contained_in MUST return true when the inner rect fits entirely within
the outer rect. Containment MUST be reflexive and transitive. If inner
is contained in outer and other is disjoint from outer, then other MUST
be disjoint from inner.

r[tiling.strip-adjacent]
segments_adjacent MUST return true when one 1D segment ends exactly
where the next begins. Adjacent non-empty segments MUST NOT overlap.

r[tiling.gapped-position]
In a uniform gapped strip, items at different indices MUST be disjoint.
The first item MUST start at position zero.

r[tiling.clip]
Viewport clipping: a fully visible segment MUST clip to itself. A fully
outside segment MUST clip to zero size. The clipped segment MUST fit
within the viewport.

r[tiling.scroll-bounds]
A scroll offset is valid when the viewport does not extend past the
content, or when the content fits within the viewport.

## Balance

r[balance.credit]
credit(balance, amount) MUST return balance + amount when the result fits
in u64, None otherwise. Credit MUST be monotonic: the result is always
>= balance. Crediting zero MUST be the identity function.

r[balance.debit]
debit(balance, amount) MUST return balance - amount when amount <= balance,
None otherwise. Debit MUST be monotonic: the result is always <= balance
when defined. Debiting zero MUST be the identity function.

r[balance.has-funds]
has_funds MUST return true if and only if amount <= balance. has_funds(b, 0)
MUST always return true. has_funds and debit failure MUST be complementary.

r[balance.valid-amount]
is_valid_amount MUST return true if and only if amount > 0.

r[balance.credit-debit-inverse]
Credit and debit MUST be inverses: debit(credit(b, a), a) == b when credit
does not overflow, and credit(debit(b, a), a) == b when debit does not
underflow.

r[balance.conservation]
For any balance b and amount a: credit_spec(b, a) - a == b and
debit_spec(b, a) + a == b. Value is neither created nor destroyed.

r[balance.net-change]
net_change MUST return the signed difference credits - debits as a
(is_positive, abs_value) pair using abs_diff.

## Transfer

r[transfer.valid]
transfer_valid MUST return true if and only if amount > 0, sender has
sufficient funds, and receiver balance will not overflow. All three
conditions MUST hold.

r[transfer.conservation]
After a valid transfer, sender_balance + receiver_balance MUST equal the
same total as before. Value MUST be neither created nor destroyed. Two
successive valid transfers MUST preserve the three-account total.

r[transfer.execute]
execute_transfer MUST return (from - amount, to + amount) when the
transfer is valid. The sender decreases by exactly amount; the receiver
increases by exactly amount. Transferring zero changes nothing.

r[transfer.idempotency]
is_duplicate MUST return true when request_id <= last_processed_id.
Duplicate transfers MUST be rejected before any balance check. A
non-duplicate request MUST have a strictly higher id.

r[transfer.validate-exhaustive]
validate_transfer MUST return exactly one of: OK (0), INSUFFICIENT_FUNDS
(1), INVALID_AMOUNT (2), RECEIVER_OVERFLOW (3), or DUPLICATE_TRANSFER (4).
The status codes MUST be mutually exclusive and exhaustive. OK if and only
if transfer_valid and not duplicate.

## Double Entry

r[double-entry.sum]
sum_amounts MUST return the sum of all amounts in the sequence. The sum
MUST be non-negative. Sum of an empty sequence MUST be zero. Sum MUST
distribute over concatenation: sum(a ++ b) == sum(a) + sum(b). Appending
an element increases the sum by that element.

r[double-entry.balanced]
is_balanced MUST return true if and only if sum(debits) == sum(credits).
Balanced MUST be symmetric: is_balanced(d, c) == is_balanced(c, d).
Empty sequences MUST be balanced. Adding equal amounts to both sides
MUST preserve balance. Concatenating two balanced pairs MUST stay balanced.

r[double-entry.trial-balance]
trial_balance MUST return sum(debits) - sum(credits). Trial balance
MUST be zero if and only if the entry is balanced. Trial balance MUST
be additive across concatenated entries.

r[double-entry.all-positive]
all_positive MUST return true if and only if every amount in the
sequence is greater than zero. Empty sequences MUST satisfy all_positive
vacuously.

r[double-entry.simple-entry]
is_simple_entry MUST return true when debit and credit amounts are equal
and positive. A simple entry is always balanced.

## Journal

r[journal.apply-entry]
apply_entry MUST return prior_balance - amount for debits, prior_balance
+ amount for credits. Returns None on underflow (debit exceeds balance)
or overflow (credit exceeds u64::MAX). Credits increase balance; debits
decrease balance.

r[journal.entry-balance-correct]
A journal entry's running_balance MUST equal the result of applying its
operation to the prior balance. The running balance MUST be non-negative
and fit in u64.

r[journal.seq-nums-monotonic]
Sequence numbers in a valid journal MUST be strictly increasing.
Appending an entry with a higher sequence number MUST preserve
monotonicity.

r[journal.valid]
A journal is valid when all running balances are correct (computed from
initial_balance through each entry in order). Empty journals MUST be
valid. Validity of a prefix is preserved.

r[journal.append-preserves]
Appending a valid entry to a valid journal MUST produce a valid journal.
The prefix of the extended journal MUST equal the original journal.

r[journal.final-balance]
final_balance MUST return the running_balance of the last entry, or
initial_balance if the journal is empty. After append, final balance is
the new entry's running balance.

r[journal.credit-debit-roundtrip]
A credit of amount a followed by a debit of amount a (or vice versa)
MUST return to the original balance.

## Cross-Module: Ledger

r[properties.balance-credit-is-saturating-add]
When credit does not overflow, credit_spec(b, a) as u64 MUST equal
saturating_add_spec(b, a). The two agree in the non-saturating range.

r[properties.balance-has-funds-is-in-range]
has_funds_spec(b, a) MUST equal in_range_spec(a, 0, b). The sufficient-
funds predicate is a range check.

r[properties.transfer-uses-debit-credit]
A valid transfer's sender_after equals balance::debit_spec and
receiver_after equals balance::credit_spec. Transfer composes balance
operations.

r[properties.transfer-idempotency-is-fencing]
A non-duplicate request (request_id > last_processed_id) satisfies
fencing::is_valid_write_spec. Idempotency keys are fencing tokens.

r[properties.journal-seq-num-is-strict-advance]
Monotonic sequence numbers in a journal satisfy
monotonic::is_strict_advance_spec for every pair i < j.

r[properties.balanced-simple-is-transfer]
A simple balanced entry (equal debit and credit of amount a) implies
transfer_valid_spec(a, 0, a). Simple bookkeeping entries correspond
to valid transfers.

## Voter Set

r[voter-set.promote]
Promoting a learner MUST add the learner's ID to the voter set. If
the learner is already a voter, the set MUST be unchanged (idempotent).
The result MUST contain no duplicate IDs.

r[voter-set.replace]
When promoting with a replacement target, the replaced node MUST be
absent from the result. All voters other than the replaced node MUST
be preserved. The learner MUST be present in the result.

r[voter-set.no-duplicates]
The result of any promotion operation MUST contain no duplicate node
IDs, regardless of whether the learner was already in the voter set.

r[voter-set.quorum-preserved]
If the previous voter set had size > 0, promotion without replacement
MUST preserve quorum. Promotion with replacement MUST preserve quorum
when the previous size satisfies can_remove_safely.

r[voter-set.learner-present]
The learner ID MUST appear in the result of every promotion operation.

r[voter-set.replaced-absent]
When replace_id is Some, the replaced ID MUST NOT appear in the result.

r[voter-set.others-preserved]
Every voter in the previous set that is not the replaced node MUST
appear in the result.

## Bounded Peers

r[bounded-peers.collect]
Building a bootstrap peer list from candidates MUST add candidates in
order until the capacity is reached. The result MUST be a subsequence
of the input candidates (minus self).

r[bounded-peers.bounded]
The result size MUST never exceed max_peers.

r[bounded-peers.self-excluded]
The node's own endpoint ID MUST NOT appear in the bootstrap peer list.

r[bounded-peers.non-empty]
When at least one non-self candidate exists and max_peers > 0, the
result MUST contain at least one peer.

r[bounded-peers.no-duplicates]
The result MUST NOT contain duplicate node IDs when the input candidates
have no duplicates.

## Node Classification

r[node-classify.voter-equivalence]
A node MUST be classified as a voter if and only if its ID is in the
member set. A node MUST be classified as a learner if and only if it is
not a voter. is_voter and is_learner MUST be mutually exclusive and
exhaustive.

r[node-classify.leader-is-voter]
If a node is classified as leader, it MUST also be classified as voter.
A learner MUST NOT be the leader.

r[node-classify.at-most-one-leader]
At most one node in the classification output MUST have is_leader set
to true. If leader_id is None, zero nodes MUST be marked as leader.

r[node-classify.bounded-output]
The classification output MUST contain at most MAX_NODES entries.

r[node-classify.no-duplicates]
The classification output MUST NOT contain duplicate node IDs when the
input has no duplicates.

## KV Version

r[kv-version.compute-versions]
compute_kv_versions MUST return a KvVersions struct given an optional
previous state (create_revision, version) and a log_index. For new keys
(None), version MUST be 1, create_revision MUST equal log_index. For
updates (Some), version MUST be previous version + 1 (saturating at
u64::MAX), and create_revision MUST be preserved from the previous state.

r[kv-version.version-always-positive]
The output version MUST always be >= 1 for any input.

r[kv-version.create-revision-preserved]
In the update case, create_revision in the output MUST equal the
create_revision from the input.

r[kv-version.mod-revision-equals-log-index]
mod_revision MUST always equal the log_index parameter.

r[kv-version.version-increments]
In the update case, output version MUST equal input version + 1 when
input version < u64::MAX.

r[kv-version.version-saturates]
When input version == u64::MAX, the output version MUST remain u64::MAX.

r[kv-version.deterministic]
The same inputs MUST always produce the same output.

## Snapshot Integrity

r[snapshot-integrity.compute]
compute_integrity MUST produce a SnapshotIntegrity struct from data_hash,
meta_hash, and chain_hash_at_snapshot by combining the two component
hashes into a combined_hash.

r[snapshot-integrity.verify]
verify_integrity MUST return true if and only if recomputing the
combined_hash from the stored data_hash and meta_hash matches the
stored combined_hash.

r[snapshot-integrity.verify-with-chain-hash]
verify_with_chain_hash MUST return true if and only if verify_integrity
succeeds AND the provided chain_hash matches chain_hash_at_snapshot.

r[snapshot-integrity.round-trip]
Computing integrity then verifying MUST always return true.

r[snapshot-integrity.data-tamper-detected]
If data_hash differs between compute and verify inputs, verify MUST
return false (assuming collision resistance).

r[snapshot-integrity.meta-tamper-detected]
If meta_hash differs between compute and verify inputs, verify MUST
return false (assuming collision resistance).

r[snapshot-integrity.chain-hash-mismatch-detected]
If the provided chain_hash does not match chain_hash_at_snapshot,
verify_with_chain_hash MUST return false.

r[snapshot-integrity.combined-commits-to-both]
The combined_hash MUST depend on both data_hash and meta_hash — changing
either one MUST change the combined_hash (assuming collision resistance).

## Constant-Time Equality

r[constant-time-eq.xor-fold]
xor_fold MUST compute the bitwise OR accumulation of XOR differences
across all 32 byte positions.

r[constant-time-eq.equality]
constant_time_eq MUST return true if and only if the two 32-byte inputs
are identical.

r[constant-time-eq.equal-returns-true]
When both inputs are identical, constant_time_eq MUST return true.

r[constant-time-eq.any-difference-returns-false]
When any byte position differs, constant_time_eq MUST return false.

r[constant-time-eq.equivalence-to-equality]
constant_time_eq(a, b) MUST be equivalent to (a == b) pointwise.

r[constant-time-eq.fold-visits-all-bytes]
The fold MUST visit all 32 byte positions regardless of intermediate
accumulator values.

## Cross-Module: KV Storage

r[properties.kv-version-mod-revision-advance]
When log_index increases, the mod_revision output satisfies
monotonic::is_advance.

r[properties.kv-version-version-advance]
For successive updates with increasing log_index, the version output
satisfies monotonic::is_advance.

r[properties.snapshot-chain-consistency]
The snapshot integrity hash axioms are consistent with chain.rs hash
axioms — both use the same uninterpreted function pattern.

r[properties.constant-time-eq-agreement]
constant_time_eq agrees with direct byte-by-byte equality.

## Election Jitter

r[election-jitter.compute]
compute_jitter MUST return a deterministic jitter value derived from
node_id via Knuth multiplicative hash modulo jitter_range.

r[election-jitter.bounded]
The jitter value MUST be strictly less than jitter_range. When
jitter_range is zero, jitter MUST be zero.

r[election-jitter.deterministic]
The same node_id and jitter_range MUST always produce the same jitter.

## Raft Timing

r[raft-timing.validate-heartbeat]
validate_heartbeat MUST return true if and only if heartbeat_ms > 0.

r[raft-timing.validate-election-range]
validate_election_range MUST return true if and only if election_min > 0
AND election_max > election_min.

r[raft-timing.validate-order]
validate_timing_order MUST return true if and only if
heartbeat_ms < election_min. The heartbeat interval MUST be strictly
less than the minimum election timeout.

r[raft-timing.validate-all]
validate_all MUST return true if and only if all three validators pass.
It MUST use fail-fast evaluation.

## Trust Level

r[trust-level.resolve]
resolve_trust MUST return Blocked when in_blocked is true (regardless
of in_trusted). It MUST return Trusted when in_trusted and not in_blocked.
It MUST return Public otherwise.

r[trust-level.block-precedence]
Blocked status MUST take precedence over Trusted status. A node that
is both blocked and trusted MUST resolve to Blocked.

r[trust-level.can-access]
can_access MUST return true for Trusted nodes. For Public nodes, it
MUST return true only when is_public is true. For Blocked nodes, it
MUST always return false.

r[trust-level.mutual-exclusivity]
Adding to trusted MUST remove from blocked. Blocking MUST remove from
trusted. The add/block operations MUST maintain mutual exclusivity.

## Node Lifecycle

r[node-lifecycle.valid-transition]
is_valid_transition MUST return true for: Starting→Online,
Online→Degraded, Degraded→Online, Online→Offline, Degraded→Offline,
Starting→Offline. All other transitions MUST return false.

r[node-lifecycle.is-operational]
is_operational MUST return true for Online and Degraded only.

r[node-lifecycle.terminal]
Offline MUST have no valid outgoing transitions.

r[node-lifecycle.reachability]
Offline MUST be reachable from every non-Offline state.

## Drain Protocol

r[drain-protocol.try-start-recheck]
try_start_with_recheck MUST increment in_flight only if both the
initial check and the recheck find draining == false.

r[drain-protocol.finish-notify]
finish_with_notify MUST decrement in_flight and return should_notify
true when in_flight reaches zero while draining.

r[drain-protocol.fully-drained]
is_fully_drained MUST return true if and only if is_draining is true
AND in_flight is zero.

## Tiered Rate Limit

r[tiered-rate-limit.check-tiered]
check_tiered MUST check the global bucket first, then per-peer. If global
denies, the result MUST be GLOBAL_DENIED without evaluating per-peer.

r[tiered-rate-limit.global-first]
When global tokens < cost, the result MUST be GLOBAL_DENIED and both
token counts MUST remain unchanged.

r[tiered-rate-limit.peer-map-bounded]
The peer map count MUST be tracked and bounded by max_peers.

r[tiered-rate-limit.capacity-invariant]
After eviction, the peer map count MUST remain within bounds.

r[tiered-rate-limit.evict-reduces]
Evicting one entry MUST reduce the count by 1 (or stay at 0).

## Peer Spec

r[peer-spec.find-separator]
find_separator MUST return the index of the first `@` byte, or -1 if
no `@` is present.

r[peer-spec.parse]
parse_peer_spec MUST split the input at the first `@` into node_id
and endpoint parts. It MUST reject inputs with no `@`, empty node_id,
or empty endpoint.

r[peer-spec.error-completeness]
Every input MUST map to exactly one of: OK, ERR_NO_SEPARATOR,
ERR_EMPTY_NODE_ID, or ERR_EMPTY_ENDPOINT.

## Cross-Module: Cluster Coordination

r[properties.tiered-rate-limit-uses-try-consume]
Tiered rate limit ALLOWED result means both underlying try_consume
calls succeeded.

r[properties.jitter-bounded-like-backoff]
Election jitter bounded by range mirrors backoff bounded by cap.

r[properties.drain-protocol-extends-drain]
drain_protocol try_start refines drain::can_start_op.

r[properties.lifecycle-operational-pressure]
node_lifecycle is_operational combined with pressure classification
determines scheduling eligibility.

r[properties.trust-access-exhaustive]
trust_level can_access is exhaustive over all TrustLevel × is_public
combinations.

## Cross-Module: Cluster Membership

r[properties.promote-preserves-quorum]
Promotion without replacement MUST preserve quorum: the full result set
always forms a quorum of itself when the input was non-empty.

r[properties.replace-quorum-guard]
Promotion with replacement produces a result whose size is at least
voters.len() - 1. Combined with can_remove_safely, quorum holds.

## Bitfield

r[bitfield.field-mask]
field_mask MUST return a bitmask with exactly `width` consecutive 1-bits
starting at position `offset`. When width is 0 the result MUST be 0.
Requires offset + width <= 64.

r[bitfield.extract-field]
extract_field MUST return bits [offset, offset+width) of the value,
right-shifted to the least-significant position. The result MUST be
less than 2^width.

r[bitfield.insert-field]
insert_field MUST replace bits [offset, offset+width) of the value with
the given field, leaving all other bits unchanged. The field MUST fit
within width bits.

r[bitfield.clear-field]
clear_field MUST zero bits [offset, offset+width) of the value, leaving
all other bits unchanged.

r[bitfield.round-trip]
Extracting a field after inserting it MUST return the original field value:
extract_field(insert_field(v, f, o, w), o, w) == f when f < 2^width.

r[bitfield.non-interference]
Inserting a field MUST NOT change bits outside [offset, offset+width).

r[bitfield.clear-then-extract]
Extracting from a cleared field MUST return 0.

r[bitfield.extract-bounded]
The result of extract_field MUST be less than 2^width.

## Election

r[election.state-predicates]
is_leader, is_follower, is_transitioning MUST return true for exactly
one of the three states. States are Follower (0), Transitioning (1),
Leader (2).

r[election.valid-transition]
is_valid_state_transition MUST permit: Follower→Transitioning,
Transitioning→Leader, Transitioning→Follower, Leader→Follower,
and self-transitions. Leader→Leader self-transition MUST require
the same fencing token. All other transitions MUST be rejected.

r[election.preconditions]
can_start_election MUST require Follower state and running=true.
can_win_election MUST require Transitioning and new_token > max_token.
can_lose_election MUST require Transitioning.
can_stepdown MUST require Leader.
can_lose_leadership MUST require Leader.

r[election.state-after]
get_state_after_start MUST return Transitioning.
get_state_after_win(token) MUST return Leader.
get_state_after_lose MUST return Follower.
get_state_after_stepdown MUST return Follower.

r[election.token-monotonicity]
compute_next_election_token MUST return current + 1 (saturating).
compute_max_token_after_win MUST return max(token, max_token).
Tokens MUST never decrease.

r[election.well-formedness]
A leader is well-formed iff fencing_token > 0 AND fencing_token <=
max_fencing_token. Non-leaders are always well-formed.

r[election.timing]
calculate_election_timeout MUST return a value in
[base, base + jitter_range] using saturating arithmetic.
is_renewal_time MUST return true when elapsed >= interval.
should_start_election MUST return true when elapsed >= timeout.
should_maintain_leadership MUST return true when elapsed < timeout.

## Distributed Lock

r[distributed-lock.token-always-positive]
compute_next_fencing_token MUST return >= 1. No previous entry
produces 1. Previous entry produces prev + 1 (saturating).

r[distributed-lock.token-monotonicity]
Fencing tokens MUST never decrease across acquisitions.

r[distributed-lock.expiry]
is_lock_expired returns true when now > deadline.
remaining_ttl returns deadline - now when not expired, else 0.
compute_lock_deadline returns acquired_at + ttl (saturating).

r[distributed-lock.backoff]
compute_backoff_with_jitter returns delay in [0, min(current*2, max)].
Next base = min(current*2, max).

## Sequence

r[sequence.monotonic-values]
compute_new_sequence_value MUST return > current. Overflow returns
an error indicator, never wraps.

r[sequence.batch-state]
should_refill_batch returns true when next >= batch_end.
batch_remaining returns batch_end - next when next < batch_end, else 0.

r[sequence.batch-computation]
compute_batch_end returns start + size, None on overflow.
compute_next_after_refill returns start + 1, None on overflow.

r[sequence.range-start]
compute_range_start returns current + 1, None on overflow.
is_initial_reservation returns true iff current == start_value.

r[sequence.cas-expected]
compute_cas_expected returns None for initial, Some(current) otherwise.

## RWLock

r[rwlock.mutual-exclusion]
Write mode and Read mode MUST NOT coexist. Write mode requires
active_readers == 0.

r[rwlock.mode-transitions]
can_acquire_read is true when Free, or Read with no pending writers,
or writer expired. can_acquire_write is true only when Free or all
expired.

r[rwlock.mode-after]
mode_after_read_acquire returns Read. mode_after_read_release returns
Free when count drops to 0, Read otherwise. mode_after_write_release
returns Free.

r[rwlock.reader-count]
increment_reader_count saturates at u32::MAX.
decrement_reader_count saturates at 0.

r[rwlock.writer-token]
compute_next_write_token returns current + 1 (saturating). Tokens
are monotonically increasing.

r[rwlock.expiry]
is_reader_expired and is_writer_expired return true when now > deadline.

## Cross-Module: Coordination

r[properties.election-token-is-fencing-issue]
election::compute_next_election_token and fencing::issue_token compute
the same value (current + 1, saturating at MAX).

r[properties.lock-expired-is-expiration-expired]
distributed_lock::is_lock_expired and expiration::is_expired use the
same predicate (now > deadline).

r[properties.lock-deadline-is-expiration-expires-at]
distributed_lock::compute_lock_deadline uses saturating addition like
expiration::expires_at (though with ms vs seconds granularity).

r[properties.rwlock-writer-token-is-fencing-issue]
rwlock::compute_next_write_token and fencing::issue_token compute the
same value.

r[properties.rwlock-expiry-is-expiration-expired]
rwlock::is_reader_expired and rwlock::is_writer_expired match
expiration::is_expired semantics.

r[properties.sequence-overflow-returns-none]
sequence::compute_batch_end and compute_range_start return None exactly
when addition would overflow u64.

## AppendEntries Consistency Check

r[append-entries.check]
check_append_entries(log, prev_log_index, prev_log_term) MUST return true
if and only if prev_log_index == 0 (empty prefix) OR the log has an entry
at prev_log_index - 1 whose term equals prev_log_term. MUST return false
when the log is too short or the term does not match. If two logs both
pass the check for the same (prev_log_index, prev_log_term), they MUST
agree at that index (same term).

## Log Conflict Resolution

r[log-conflict.find-conflict]
find_conflict(follower, leader_entries, start_index) MUST return the first
relative offset where the follower's log and leader's entries disagree on
term. MUST return None when all overlapping entries agree or when the
leader extends beyond the follower. The conflict offset MUST be within
leader_entries range. At the conflict point, terms MUST differ. Before
the conflict point, all terms MUST agree.

r[log-conflict.resolve]
resolve_conflict MUST truncate the follower log at the conflict point and
append leader entries from that point onward. When no conflict exists,
MUST append leader entries that extend beyond the follower. Entries before
the conflict point MUST be unchanged. The result MUST match leader entries
at all covered indices.

## Match Index Tracking

r[match-index.init]
init_match_indices(cluster_size) MUST return a sequence of cluster_size zeros.
init_next_indices(cluster_size, last_log_index) MUST return a sequence where
every element is last_log_index + 1.

r[match-index.update]
update_match_index(matches, server, new_match) MUST set matches[server] to
new_match. new_match MUST be >= matches[server] (monotonic). The update MUST
NOT modify any other server's match index. Length MUST be preserved.

r[match-index.backtrack]
backtrack_next_index(nexts, server) MUST decrement nexts[server] by 1, with
a floor of 1. The update MUST NOT modify any other server's next index.

r[match-index.count-replicated]
count_replicated(matches, index) MUST return the count of servers with
matchIndex >= index. The count MUST be non-negative and bounded by the
array length.

## Commit Index Advancement

r[commit-advance.term-guard]
can_commit_at(log, index, current_term) MUST return true only when
log[index-1].term == current_term. Prior-term entries MUST NOT be directly
committable. Index 0 MUST never be committable. This is the §5.4.2 rule.

r[commit-advance.quorum-check]
has_quorum_at(match_indices, index, cluster_size) MUST return true if and
only if the count of matchIndex[i] >= index constitutes a quorum. Delegates
to quorum::is_quorum.

r[commit-advance.advance]
advance_commit_index MUST return the largest index N where N > current_commit
AND can_commit_at(log, N, current_term) AND has_quorum_at(match_indices, N,
cluster_size). If no such N exists, MUST return current_commit. The result
MUST always be >= current_commit (monotonic). Empty log means no advancement.

## Term Authority

r[term-authority.step-down]
apply_higher_term(current_term, current_state, voted_for, message_term)
MUST adopt message_term, become follower, and clear votedFor when
message_term > current_term. MUST be a no-op when message_term <=
current_term. The resulting term MUST always be >= current_term.

## Leader Append-Only

r[leader-append-only.invariant]
leader_log_invariant(old_log, new_log) MUST return true if and only if
new_log.len() >= old_log.len() AND all entries at [0, old_log.len()) are
unchanged. The invariant MUST be reflexive and transitive. Truncation of
a non-empty log MUST violate the invariant.

r[leader-append-only.append]
leader_append_entry MUST append an entry with the given term and value.
The appended entry's term MUST equal current_term. Appending MUST
satisfy the leader log invariant. Multiple appends MUST compose.

## No-Op Commitment

r[noop-commit.make-noop]
make_noop_entry(current_term) MUST return a log entry with term ==
current_term and value == NOOP_VALUE (0). The entry MUST be recognized
by is_noop. Appending a no-op MUST preserve all prior log entries.

r[noop-commit.is-noop]
is_noop MUST return true if and only if entry.value == NOOP_VALUE.

r[noop-commit.commit-index]
noop_commit_index(log_len_before_noop) MUST return log_len_before_noop + 1.
Once committed, all entries at indices [0, log_len_before_noop) MUST be
below the commit index (committed as prefix).

## Leader Completeness

r[leader-completeness.quorum-overlap]
The commitment quorum and election quorum MUST overlap (pigeonhole via
quorum::prove_quorum_intersection). Two joint quorums sharing a voter
ensures the new leader has the committed entry.

r[leader-completeness.log-freshness]
A leader whose log is at least as up-to-date as the committed entry's
epoch (by epoch_ge) MUST have that entry. Vote granting requires epoch_ge,
so transitivity through the shared voter proves the leader has the entry.

## State Machine Safety

r[state-machine-safety.index-agreement]
If two logs have the same term at index i, they agree at that index (by
definition of agree_at). Same-term entries at the same index are identical.

r[state-machine-safety.prefix-agreement]
If two logs match on prefix [0, k), every entry in that prefix agrees.
Committed entries within a matching prefix are safe.

r[state-machine-safety.commit-safety]
If logs match through the commit point and an entry is committed (index <
commit_index), that entry is identical across both logs.

## Pre-Vote Protocol

r[pre-vote.grant]
can_grant_pre_vote MUST return true when candidate_term >= my_term AND the
candidate's log is at least as up-to-date (epoch_ge). MUST reject stale
candidates and outdated logs. The voter's term MUST NOT change.

r[pre-vote.start-election]
should_start_real_election MUST return true when pre-votes form a quorum.
Delegates to quorum::is_quorum.

## Read Index

r[read-index.confirm-authority]
confirm_read_authority MUST return true when ack_count forms a quorum.
Delegates to quorum::is_quorum. A deposed leader with only self-ack
MUST fail the check.

r[read-index.safe-at]
read_safe_at(read_index, commit_index) MUST return true when commit_index
>= read_index. MUST be monotonic in commit_index. The exact commit point
MUST be safe.

## Joint Consensus

r[joint-consensus.dual-quorum]
is_joint_quorum MUST return true when BOTH old and new configurations
independently form quorums. Two joint quorums MUST overlap in both configs.
Full clusters MUST always form a joint quorum.

## Install Snapshot

r[install-snapshot.trigger]
should_install_snapshot MUST return true when next_index <= first_log_index.

r[install-snapshot.apply]
apply_snapshot_commit MUST return max(current_commit, snapshot_index). The
commit index MUST never decrease.

r[install-snapshot.matching-entry]
has_matching_entry MUST return true when the log has an entry at
snapshot_index with the given term.
