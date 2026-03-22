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
