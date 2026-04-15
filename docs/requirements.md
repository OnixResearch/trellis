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

r[bitwise.is-sign-extended]
is_sign_extended(value, width) MUST return true if and only if bits [63:width]
of value are all copies of bit [width-1]. width MUST be in [1, 64].
is_sign_extended(value, 64) MUST always return true. If
is_sign_extended(value, w1) holds and w2 >= w1, then
is_sign_extended(value, w2) MUST also hold.

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

r[properties.chaoscontrol-subset-replay]
For normalized schedules, if `subset(source, indices)` is replayed from
`cursor = 0` by any nondecreasing time sequence whose final horizon covers all
subset faults, delivered faults MUST appear in the same relative order as in
`source`.

r[properties.chaoscontrol-fingerprint-trace]
For schedulers that start from the same initial state, or the same full
post-`apply_variant` reset state, executions with the same post-advance
turn/quantum trace MUST produce the same fingerprint even if the imperative
shell path differs.

r[properties.chaoscontrol-quorum-gate]
For valid fault states, a healthy-period gate MUST never report healthy unless
the corresponding `quorum_reachable` predicate holds for that same state.

## Consistency Model Vocabulary

r[consistency-vocab.valid-op-kind]
The shared consistency-history vocabulary MUST use four operation tags:
read, write, commit, and abort. Any other tag MUST be rejected.

r[consistency-vocab.op-complete]
A shared history operation MUST be complete when it has a valid tag and
its invocation time is less than or equal to its response time.

r[consistency-vocab.same-object]
Two history operations MUST be considered on the same object if and only
if their object identifiers are equal.

r[consistency-vocab.same-transaction]
Two history operations MUST be considered in the same transaction if and
only if their transaction identifiers are equal.

r[consistency-vocab.same-session]
Two history operations MUST be considered in the same session if and only
if their session identifiers are equal.

## Shamir Secret Sharing

r[shamir-field.prime-modulus]
Shamir field arithmetic MUST validate that the modulus is prime and less
than 2^32. Composite moduli MUST be rejected.

r[shamir-field.closure]
Field addition and multiplication modulo p MUST return values in [0, p).
The modulus MUST satisfy 1 < p < 2^32 and be prime.

r[shamir-field.additive-identity]
Zero MUST be the additive identity: a + 0 == a and 0 + a == a.

r[shamir-field.multiplicative-identity]
One MUST be the multiplicative identity: a * 1 == a and 1 * a == a.

r[shamir-field.commutativity]
Field addition and multiplication MUST be commutative.

r[shamir-field.multiplicative-inverse]
Any inverse returned for a non-zero element MUST multiply back to one.
Zero MUST have no inverse.

r[shamir-field.subtraction]
Field subtraction MUST satisfy (a - b) + b == a modulo p.

r[shamir-poly.horner-correctness]
Polynomial evaluation by Horner's method MUST match the recursive
polynomial sum definition. Evaluating at zero MUST return the constant term.

r[shamir-poly.degree-bound]
A polynomial with k coefficients MUST have degree at most k - 1.

r[shamir-lagrange.kronecker-delta]
The i-th Lagrange basis function MUST equal 1 at x_i and 0 at x_j for j != i.

r[shamir-lagrange.partition-of-unity]
The Lagrange basis values at any interpolation point MUST sum to 1.

r[shamir-lagrange.interpolation-correctness]
The Lagrange interpolant MUST reproduce the provided y-values at the
interpolation points. Evaluating it at zero MUST support reconstruction.

r[shamir-lagrange.uniqueness]
A value vector over fixed distinct interpolation points MUST be uniquely
determined by its interpolated values on those points.

r[shamir-split.share-count]
Split MUST return exactly one share per requested x-coordinate and preserve
those x-coordinates in the output.

r[shamir-split.parameter-validation]
Split MUST reject degenerate thresholds and invalid x-coordinates.
Threshold MUST satisfy 2 <= k <= n.

r[shamir-split.constant-term-preservation]
The secret MUST be the constant term of the split polynomial. Evaluating the
polynomial at x = 0 MUST return the secret.

r[shamir-reconstruct.correctness]
Reconstruction from k valid shares MUST return the secret encoded by the
threshold polynomial used to generate those shares.

r[shamir-reconstruct.insufficient-shares]
Reconstruction from fewer than k shares SHOULD NOT recover the original
secret except in degenerate low-degree cases.

r[shamir-threshold.information-theoretic-security]
Any k - 1 visible shares MUST be completable to any target secret by adding
a suitable k-th point at x = 0.

r[shamir-threshold.perfect-reconstruction-threshold]
Once the secret point is included, the completed interpolation data MUST
fix a unique secret value.

r[consistency-vocab.history-well-formed]
A history MUST be well formed when every operation in it is complete under
`consistency-vocab.op-complete`.

r[consistency-vocab.realtime-order]
Real-time order MUST hold from operation `a` to operation `b` exactly when
`a` responds before `b` is invoked.

r[consistency-vocab.session-order]
Session order MUST hold from operation `a` to operation `b` exactly when
they belong to the same session, have different operation IDs, and `a`
completes no later than `b` begins.

r[consistency-vocab.visibility]
Visibility MUST hold from a write to a read exactly when the read names
that write as its observed source on the same object.

r[consistency-vocab.arbitration]
Arbitration MUST give a deterministic order between two writes to the
same object by response time, with operation ID as the tie-breaker.

r[consistency-vocab.write-conflict]
Two writes MUST conflict exactly when they touch the same object but come
from different transactions.

r[consistency-vocab.happens-before]
The shared happens-before edge relation MUST be the union of real-time
order, session order, and visibility.

## Transactional Consistency Models

r[consistency-txn.read-uncommitted]
Read uncommitted MUST still forbid dirty writes.

r[consistency-txn.read-committed]
Read committed MUST extend read uncommitted by forbidding dirty reads.

r[consistency-txn.cursor-stability]
Cursor stability MUST extend read committed by protecting cursor-held
objects from concurrent modification.

r[consistency-txn.monotonic-atomic-view]
Monotonic atomic view MUST extend read committed by requiring all-or-none
visibility for effects of already-observed committed transactions.

r[consistency-txn.repeatable-read]
Repeatable read MUST extend read committed by keeping rereads of the same
object stable within a transaction.

r[consistency-txn.predicate-stability]
Predicate stability MUST hold exactly when reevaluating a predicate yields
the same membership result.

r[consistency-txn.first-committer-wins]
First-committer-wins MUST reject executions in which overlapping snapshots
with conflicting write sets both commit.

r[consistency-txn.snapshot-isolation]
Snapshot isolation MUST extend read committed with a consistent snapshot,
atomic visibility, and first-committer-wins.

r[consistency-txn.serializability]
Serializability MUST hold when there exists a serial witness order for the
committed transactions.

r[consistency-txn.dependency-acyclic]
An acyclic transaction-dependency graph MUST be a valid serializability
witness shape.

r[consistency-txn.strict-serializability]
Strict serializability MUST extend serializability with real-time and
per-session program-order respect across transactions.

## Object and Session Consistency Models

r[consistency-object.sequential-witness]
A sequential witness history MUST be a well-formed total order of complete
shared read/write operations expressed with the consistency-history
vocabulary.

r[consistency-object.sequential-consistency]
Sequential consistency MUST require a shared sequential witness and
preserved session/program order.

r[consistency-object.linearizability]
Linearizability MUST extend sequential consistency by also preserving
shared real-time order.

r[consistency-session.causal-dependency]
A reusable causal-dependency witness MUST agree with vector-clock
happens-before.

r[consistency-session.causal-ready]
Message-level causal readiness MUST reuse the existing causal-delivery
readiness rule.

r[consistency-session.read-your-writes]
Read-your-writes MUST hold exactly when a session's later reads include
its own earlier writes.

r[consistency-session.monotonic-reads]
Monotonic reads MUST hold exactly when later reads do not move to an
older observed state.

r[consistency-session.monotonic-writes]
Monotonic writes MUST hold exactly when same-session writes preserve issue
order everywhere.

r[consistency-session.writes-follow-reads]
Writes-follow-reads MUST hold exactly when a write after a read is not
ordered before the write that produced the read value.

r[consistency-session.pram]
PRAM MUST require monotonic reads, monotonic writes, and read-your-writes.

r[consistency-session.causal-consistency]
Causal consistency MUST extend PRAM with writes-follow-reads.

## Consistency Refinement and Availability

r[consistency-refinement.model-hierarchy]
The implementation MUST expose proof bridges for the implication edges in
the consistency-model hierarchy.

r[consistency-refinement.availability-envelope]
The implementation MUST classify each modeled consistency level by the
strongest availability envelope assigned in the OpenSpec taxonomy.

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

## Set-Associative Lookup

r[set-assoc.decomposition]
Decomposing an address into tag, index, and offset MUST preserve all
information needed to reconstruct the original address. For any valid
`offset_bits` and `index_bits` with `offset_bits + index_bits <= 64`,
`reconstruct_address(decompose_address(addr))` MUST equal `addr`.

r[set-assoc.component-bounds]
extract_offset MUST return a value that fits in `offset_bits` bits.
extract_index MUST return a value that fits in `index_bits` bits.
extract_tag MUST return a value that fits in the remaining
`64 - offset_bits - index_bits` tag bits.

r[set-assoc.hit]
find_way(set, tag) MUST return `Some(i)` only when `set[i] == Some(tag)`.
lookup_set(set, addr, offset_bits, index_bits) MUST return the way whose
stored tag matches `extract_tag(addr, offset_bits, index_bits)`.

r[set-assoc.miss]
find_way(set, tag) MUST return `None` only when no way in the set holds
`Some(tag)`. lookup_set MUST miss when every way disagrees with the
requested tag.

r[set-assoc.no-duplicates]
Within a set, at most one way SHOULD hold a given non-empty tag.
check_no_duplicate_tags MUST return true if and only if no two distinct
ways hold the same `Some(tag)` value.

r[set-assoc.insert-preserves-uniqueness]
Inserting a tag into an empty way of a set that already has no duplicate
tags, when that tag is absent from all other ways, MUST preserve the
no-duplicate-tags invariant.

r[set-assoc.no-aliasing]
For two addresses with the same index but different tags, a hit for one
MUST NOT be a hit for the other's tag. The matched way MUST identify the
requested line, not another tag that shares the set.

r[set-assoc.set-isolation]
Changing a cache set whose index differs from the requested address's
index MUST NOT change the lookup result for that address. Different
indices MUST select different sets.

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

r[page.radix-index]
radix_index(value, level, bits_per_level, base_offset) MUST return the
field at bit position (base_offset + level * bits_per_level) with width
bits_per_level. The result MUST agree with bitfield::extract_field at
the computed offset and width. The result MUST be strictly less than
2^bits_per_level. bits_per_level MUST be > 0. The computed offset +
width MUST be <= 64.

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

## Nickel Merge Split

r[nickel-merge.split]
split(m1, m2) MUST partition keys into three disjoint sets: left (keys
only in m1), center (keys in both), and right (keys only in m2). No key
MUST appear in more than one partition.

r[nickel-merge.split-completeness]
Every key in m1 MUST appear in either left or center. Every key in m2
MUST appear in either center or right. No keys are lost.

r[nickel-merge.split-value-preservation]
For keys in left, the value MUST come from m1. For keys in right, the
value MUST come from m2. For keys in center, both values MUST be
preserved as a pair.

## Nickel Priority

r[nickel-priority.total-order]
MergePriority MUST form a total order: Bottom < Neutral < Numeral(n) < Top.
Neutral MUST compare equal to Numeral(0). The order MUST be reflexive,
antisymmetric, and transitive.

r[nickel-priority.extrema]
Bottom MUST be less than or equal to every other priority. Top MUST be
greater than or equal to every other priority.

r[nickel-priority.numeral-ordering]
Numeral(a) < Numeral(b) if and only if a < b. Numeral ordering MUST
agree with the underlying integer ordering.

## Nickel Field Merge

r[nickel-field-merge.higher-wins]
When merging two fields with different priorities, the field with strictly
higher priority MUST be selected.

r[nickel-field-merge.equal-recurse]
When merging two fields with equal priority, both values MUST be equal
or the merge MUST fail (conflict).

r[nickel-field-merge.default-override]
A field with priority Bottom MUST be overridden by any field with higher
priority. A field with priority Top MUST override any field with lower
priority.

## Nickel Record Merge

r[nickel-record-merge.identity]
Merging any record with the empty record MUST return the original record
unchanged.

r[nickel-record-merge.associativity]
merge(a, merge(b, c)) MUST equal merge(merge(a, b), c) when all merges
succeed and priorities are uniform.

r[nickel-record-merge.key-coverage]
The keys of merge(a, b) MUST be a superset of keys(a) and keys(b).
No keys are lost or invented.

r[nickel-record-merge.disjoint-commutativity]
When records a and b have no keys in common, merge(a, b) MUST equal
merge(b, a).

## Nickel Subtype

r[nickel-subtype.reflexivity]
Every type T MUST be a subtype of itself: T <: T.

r[nickel-subtype.transitivity]
If T1 <: T2 and T2 <: T3, then T1 <: T3.

r[nickel-subtype.record-dict]
A record type MUST be a subtype of a dictionary type if and only if
every field type is a subtype of the dictionary value type.

r[nickel-subtype.array-covariance]
Array T <: Array U if and only if T <: U.

r[nickel-subtype.dict-covariance]
{_: T} <: {_: U} if and only if T <: U.

r[nickel-subtype.closed-records]
A closed record MUST be a subtype of another closed record if and only if
both records have exactly the same field names and the corresponding
field types are pairwise subtypes.

## Nickel Polarity

r[nickel-polarity.flip-domain]
When a contract checks a function argument, the polarity MUST flip.

r[nickel-polarity.preserve-codomain]
When a contract checks a function return value, the polarity MUST be
preserved.

r[nickel-polarity.double-flip]
Flipping polarity twice MUST return to the original polarity.

r[nickel-polarity.path-tracking]
The polarity at any point in a type path MUST equal the initial polarity
flipped once for each domain step.

## Nickel Pattern Coverage

r[nickel-pattern.coverage-completeness]
A set of patterns MUST be complete for a type when every possible value
matches at least one pattern.

r[nickel-pattern.overlap-detection]
When two patterns can match the same value, this overlap MUST be
detectable.

r[nickel-pattern.wildcard-covers-all]
A wildcard pattern MUST match any value. A pattern set containing a
wildcard MUST always be complete.

r[nickel-pattern.enum-coverage]
For an enum type, a pattern set MUST be complete if and only if it
contains a pattern for each variant or a wildcard.

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

## Type Unification

r[unify.unifier-exists]
If some finite fuel budget makes the equation-list solver succeed, the
public `unify_spec` chooser MUST return a substitution. This requirement
covers witness selection, not semantic completeness of unification.

r[unify.unifier-correct]
Applying any substitution returned by the solver to both input types MUST
yield identical resulting types.

r[unify.occurs-check]
Unification MUST reject binding a variable to a type that already
contains that variable. The occurs check prevents infinite self-referential
types.

r[unify.subst-compose]
Substitution composition MUST model sequential application.
Applying compose(s2, s1) to a type MUST equal applying s1 first and s2
second. Composition MUST be associative.

r[unify.failure]
Obvious structural clashes MUST fail. In particular, distinct concrete
constructors MUST not unify, and an application MUST not unify with a
concrete constructor.

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

r[trust-level.capacity-bounded]
is_bounded MUST return true when the set count does not exceed the
capacity limit.

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

## Left-Right

r[left-right.index-toggle]
The left-right state MUST track exactly two sides, LEFT and RIGHT. The
other-side operation MUST flip between them, and initial_state MUST start
with synchronized copies, zero readers, and a valid active side.

r[left-right.reader-tracking]
reader_enter MUST increment the reader count on the current active side and
return a ticket carrying that side and its version. reader_exit MUST
decrement the reader count on the ticket side and otherwise preserve the
state.

r[left-right.writer-publish]
writer_publish MUST require synchronized copies, write the next version to
the inactive side, and flip the active side atomically. The new active
version MUST equal the old active version plus one, and the old active side
MUST retain its previous version until catch-up.

r[left-right.quiescence]
A side MUST be quiescent exactly when its reader count is zero. can_catch_up
MUST be true exactly when the inactive side is quiescent. writer_catch_up
MUST only run after quiescence and MUST resynchronize both copies without
changing the active version.

r[left-right.snapshot-consistency]
A reader ticket obtained before a publish MUST continue to name the old
side's unchanged version after the publish. A live reader on the old side
MUST block catch-up until it exits, so readers never observe partial writer
updates.

## Surelock

r[surelock-lock-id.unique]
allocate_lock MUST allocate a fresh lock ID for each successful allocation.
Two successive successful allocations MUST never produce the same lock ID.

r[surelock-lock-id.monotonic]
Lock IDs MUST increase with creation order. Later successful allocations
MUST receive strictly greater IDs than earlier successful allocations.

r[surelock-lock-id.stable-identity]
A lock's ID MUST stay attached to the lock record itself. Reordering lock
records in a collection MUST NOT change any lock ID.

r[surelock-level-order.total-order]
Level comparison MUST be a strict total order: for any two levels exactly
one of less-than, equality, or greater-than holds.

r[surelock-level-order.lock-after]
lock_after MUST admit acquisitions exactly when current <= next in level
order. Equal-level reacquisition checks and upward acquisitions are allowed;
descending acquisitions are rejected.

r[surelock-level-order.derived-higher]
new_higher_from(parent) MUST return parent + 1, or None on overflow.
new_higher_pair(a, b) MUST return max(a, b) + 1, or None on overflow.

r[surelock-key-flow.progression]
Acquiring a lock MUST return a key at that lock's level when admitted.
Acquiring a lock group MUST return a key at the maximum level reached by
that group. Any lock or group below the current key level MUST be rejected.

r[surelock-key-flow.subscope]
Entering and leaving a subscope MUST preserve the current key level.

r[surelock-key-flow.no-backward]
Admitted surelock key transitions MUST be nondecreasing by level.
Backward transitions MUST be rejected.

r[surelock-lockset.canonical-ordering]
prepare_lockset MUST accept only same-level lock groups and produce a
canonical acquisition plan sorted by strictly increasing lock ID.

r[surelock-lockset.duplicate-rejection]
prepare_lockset MUST reject any candidate lock group that contains the
same lock ID more than once.

r[surelock-lockset.membership-preservation]
Canonical lock-set preparation MUST preserve lock membership: sorting MAY
change acquisition order but MUST NOT add, drop, or replace lock IDs.

r[surelock-lockset.caller-order-preservation]
caller_to_plan_positions MUST map each caller position back to the matching
lock in canonical plan, so logical caller order remains recoverable after
internal sorting.

r[surelock-deadlock-freedom.forward-wait-rank]
Every admitted wait edge MUST move strictly forward in lexicographic rank
(level, id). Same-level waits advance by ID; cross-level waits advance by
level.

r[surelock-deadlock-freedom.no-circular-wait]
Any wait graph whose every edge moves forward in surelock rank MUST be
acyclic.

r[surelock-deadlock-freedom.admitted-trace]
Any admitted surelock acquisition trace MUST be deadlock-free with respect
to circular wait.

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

## Last Applied Tracking

r[last-applied.advance]
advance_last_applied(last_applied, commit_index) MUST return
min(last_applied + 1, commit_index) when last_applied < commit_index,
and last_applied otherwise. The result MUST always be >= last_applied
(monotonic) and <= commit_index (never applied past committed).

r[last-applied.can-apply]
can_apply(last_applied, commit_index) MUST return true if and only if
last_applied < commit_index. There is at least one committed entry
waiting to be applied.

r[last-applied.apply-count]
pending_apply_count(last_applied, commit_index) MUST return
commit_index - last_applied when commit_index >= last_applied, else 0.
This is the number of entries waiting to be applied.

r[last-applied.invariant]
The last-applied invariant MUST hold: last_applied <= commit_index.
Advancing last_applied MUST preserve the invariant. Advancing
commit_index (via commit_advance) MUST preserve the invariant.

r[last-applied.apply-entry-index]
next_apply_index(last_applied) MUST return last_applied + 1 (1-based
log index of the next entry to apply). The caller MUST ensure
can_apply is true before calling.

r[last-applied.batch-apply]
batch_apply_end(last_applied, commit_index, max_batch) MUST return
min(last_applied + max_batch, commit_index). The result MUST be
>= last_applied and <= commit_index. The batch size MUST NOT exceed
max_batch.

## Leadership Transfer

r[leadership-transfer.can-start]
can_start_transfer(state, target_id, self_id) MUST return true if and
only if state == Leader AND target_id != self_id. A non-leader MUST
NOT initiate transfer. Self-transfer MUST be rejected.

r[leadership-transfer.target-caught-up]
is_target_caught_up(target_match_index, last_log_index) MUST return
true if and only if target_match_index >= last_log_index. The leader
MUST NOT send TimeoutNow until the target has replicated all entries.

r[leadership-transfer.should-send-timeout-now]
should_send_timeout_now(is_leader, target_caught_up, transfer_deadline,
now) MUST return true when all of: state is leader, target is caught
up, and now <= transfer_deadline (transfer has not expired). An expired
transfer MUST return false.

r[leadership-transfer.transfer-expired]
is_transfer_expired(transfer_deadline, now) MUST return true when
now > transfer_deadline. An expired transfer MUST be abandoned —
the leader resumes normal operation.

r[leadership-transfer.blocks-new-proposals]
should_reject_proposal(transfer_in_progress) MUST return true when a
transfer is in progress. The leader MUST NOT accept new client proposals
during transfer.

r[leadership-transfer.abort]
can_abort_transfer(is_leader, transfer_in_progress) MUST return true
when both hold. Aborting clears the transfer state and resumes normal
operation. Aborting a non-existent transfer MUST be a no-op.

## Log Compaction

r[log-compaction.should-compact]
should_compact(log_len, snapshot_threshold) MUST return true when
log_len >= snapshot_threshold. A threshold of 0 MUST always trigger.
A log shorter than the threshold MUST NOT trigger.

r[log-compaction.compact]
compact_log(log, snapshot_index) MUST discard all entries at indices
[0, snapshot_index) and return the suffix starting at snapshot_index.
The result length MUST equal log.len() - snapshot_index when
snapshot_index <= log.len(). If snapshot_index > log.len(), the
result MUST be empty.

r[log-compaction.prefix-removed]
After compaction, no entry from the original [0, snapshot_index) range
is present in the result. The first entry of the result (if non-empty)
corresponds to the original entry at snapshot_index.

r[log-compaction.suffix-preserved]
All entries at indices [snapshot_index, log.len()) MUST be preserved
in the result, with their terms and values unchanged. For all i in
[0, result.len()): result[i] == original[snapshot_index + i].

r[log-compaction.first-index-after]
first_log_index_after_compaction(prev_first_index, snapshot_index) MUST
return max(prev_first_index, snapshot_index). The first log index MUST
never decrease.

r[log-compaction.offset-translation]
translate_index(absolute_index, first_log_index) MUST return
absolute_index - first_log_index when absolute_index >= first_log_index,
and None otherwise. Indices before the compacted region are inaccessible.

r[log-compaction.compact-preserves-commit]
Compacting entries below the commit index MUST NOT change the commit
index. snapshot_index MUST be <= commit_index (never compact uncommitted
entries).

## Heartbeat

r[heartbeat.is-heartbeat]
is_heartbeat(entry_count) MUST return true if and only if entry_count == 0.
A heartbeat is an AppendEntries RPC carrying no log entries.

r[heartbeat.should-send]
should_send_heartbeat(elapsed_ms, heartbeat_interval_ms) MUST return true
when elapsed_ms >= heartbeat_interval_ms. The leader MUST send heartbeats
at the configured interval.

r[heartbeat.reset-election-timer]
should_reset_election_timer(msg_term, current_term, from_leader) MUST
return true when msg_term >= current_term AND from_leader is true. A
valid heartbeat from the current leader resets the follower's election
timer. Messages from non-leaders or stale terms MUST NOT reset the timer.

r[heartbeat.leader-alive]
is_leader_alive(last_heartbeat_time, now, election_timeout) MUST return
true when now - last_heartbeat_time < election_timeout (using saturating
subtraction). A follower considers the leader alive while heartbeats
arrive within the election timeout.

r[heartbeat.ack]
heartbeat_ack_valid(ack_term, leader_term) MUST return true when
ack_term == leader_term. Stale acks (ack_term < leader_term) MUST be
ignored. Higher-term acks trigger step-down via term_authority.

## Client Session

r[client-session.register]
register_session(next_session_id) MUST return (session_id, next_id + 1).
Session IDs MUST be monotonically increasing. Registration MUST be
logged as a Raft entry so all replicas track the same sessions.

r[client-session.is-duplicate]
is_duplicate_request(session_table_seq, request_seq) MUST return true
when request_seq <= session_table_seq. A request with a sequence number
at or below the last processed sequence for that session MUST be
treated as duplicate.

r[client-session.advance-seq]
advance_session_seq(current_seq, request_seq) MUST return request_seq
when request_seq > current_seq. The session's sequence number MUST
only move forward.

r[client-session.lookup]
session_exists(table_len, session_id) MUST return true when session_id
< table_len. Sessions are densely packed from 0.

r[client-session.expire]
is_session_expired(last_activity, now, session_timeout) MUST return
true when now - last_activity >= session_timeout (saturating
subtraction). Expired sessions MAY be reclaimed.

r[client-session.response-cache]
cached_response_valid(cached_seq, request_seq) MUST return true when
cached_seq == request_seq. The cached response for a duplicate request
MUST be returned without re-executing the command.

r[client-session.table-bounded]
session_count_within_limit(count, max_sessions) MUST return true when
count < max_sessions. Registration MUST be rejected when the table
is full.

## Lease Read

r[lease-read.is-valid]
is_lease_valid(lease_start, election_timeout, clock_drift_bound, now)
MUST return true when now < lease_start + election_timeout -
clock_drift_bound (all saturating arithmetic). The lease MUST expire
strictly before any other server's election timeout fires, accounting
for worst-case clock drift.

r[lease-read.start]
start_lease(now) MUST record now as the lease start time. The lease
begins when the leader confirms authority via a quorum of heartbeat
responses.

r[lease-read.disjoint]
Two leases from different leaders MUST NOT overlap in time. If leader A's
lease ends at time T_a and leader B's lease starts at T_b, then T_b >=
T_a. This follows from: the new leader's election required a quorum,
which includes at least one node that would have let leader A's lease
expire before voting.

r[lease-read.requires-commit]
lease_read_safe(lease_valid, commit_index, read_index) MUST return true
only when lease_valid AND commit_index >= read_index. A valid lease alone
is insufficient — the read must also wait for its commit point, same as
read_index.

r[lease-read.leader-only]
Only a node in Leader state MUST be permitted to serve lease reads.
Followers and candidates MUST reject lease read requests.

## Follower Redirect

r[follower-redirect.should-redirect]
should_redirect(state, known_leader_id) MUST return true when state !=
Leader AND known_leader_id is Some. Followers and candidates with a known
leader MUST redirect clients. A follower with no known leader MUST NOT
redirect (return error instead).

r[follower-redirect.leader-hint]
leader_hint(known_leader_id) MUST return the leader's node ID when known,
or a sentinel (u64::MAX) when unknown. The hint is best-effort — it MAY
be stale after a leader change.

r[follower-redirect.hint-freshness]
is_hint_fresh(hint_term, current_term) MUST return true when hint_term ==
current_term. A hint from a prior term MUST be considered stale. Clients
receiving a stale hint MUST retry discovery.

r[follower-redirect.leader-accepts]
should_accept_request(state) MUST return true if and only if state ==
Leader. Only the leader processes client requests directly.

## Snapshot Transfer

r[snapshot-transfer.chunk-valid]
is_chunk_valid(offset, chunk_size, total_size) MUST return true when
offset + chunk_size <= total_size (saturating addition to avoid overflow).
A zero-size chunk MUST be invalid. Offset beyond total_size MUST be
invalid.

r[snapshot-transfer.is-first-chunk]
is_first_chunk(offset) MUST return true if and only if offset == 0.
The first chunk starts at the beginning of the snapshot data.

r[snapshot-transfer.is-last-chunk]
is_last_chunk(offset, chunk_size, total_size) MUST return true when
offset + chunk_size == total_size. The last chunk ends exactly at
the snapshot boundary.

r[snapshot-transfer.next-offset]
next_expected_offset(current_offset, chunk_size) MUST return
current_offset + chunk_size. The receiver tracks the next expected
offset for in-order delivery. Returns None on overflow.

r[snapshot-transfer.all-chunks-cover]
For a complete transfer, the chunks MUST cover [0, total_size) with
no gaps and no overlaps. The sum of all chunk sizes MUST equal
total_size.

r[snapshot-transfer.chunk-count]
chunk_count(total_size, chunk_size) MUST return ceil(total_size /
chunk_size) when chunk_size > 0. This is the number of chunks needed
to transfer the full snapshot.

r[snapshot-transfer.progress]
transfer_progress(received_bytes, total_size) MUST return a value in
[0, 100] representing the percentage complete. 0 when received == 0.
100 when received >= total_size. Monotonically increasing as bytes
arrive.

## Cross-Module: Last Applied

r[properties.last-applied-bounded-by-commit]
advance_last_applied result MUST always be <= commit_index. The
apply cursor never overshoots the commit cursor.

r[properties.last-applied-advance-is-monotonic]
advance_last_applied satisfies monotonic::is_advance on last_applied.
The apply cursor never moves backward.

r[properties.pending-apply-is-saturating-sub]
pending_apply_count(last_applied, commit_index) MUST equal
arithmetic::saturating_sub_spec(commit_index, last_applied). The
pending count is the gap computed by generic saturating subtraction.

## Cross-Module: Leadership Transfer

r[properties.transfer-caught-up-is-advance]
is_target_caught_up(match, last) is equivalent to
monotonic::is_advance(last, match) — the target's match index has
reached the leader's log end.

r[properties.transfer-blocks-like-drain]
should_reject_proposal(true) mirrors drain::can_start_op(true) == false.
Both gate new work during a shutdown/transfer protocol.

r[properties.transfer-timeout-is-expiration]
is_transfer_expired(deadline, now) MUST equal
expiration::is_expired(deadline, now). Transfer timeout uses the same
predicate as lease/lock expiration.

## Cross-Module: Log Compaction

r[properties.compact-suffix-length-is-saturating-sub]
The length of compact_log result MUST equal
arithmetic::saturating_sub_spec(log.len(), snapshot_index). Compaction
length is a saturating subtraction.

r[properties.compact-first-index-is-max]
first_log_index_after_compaction(prev, snap) MUST equal
ordering::max_spec(prev, snap). The first index is the max of old and
new.

r[properties.compact-offset-uses-checked-sub]
translate_index returns None exactly when the absolute index is below
first_log_index — same condition as a checked subtraction underflow.

r[properties.compact-preserves-install-snapshot-trigger]
After compaction with snapshot_index S, should_install_snapshot(next, S)
holds for any follower with next <= S. Compaction creates the exact
condition that triggers snapshot installation.

## Cross-Module: Heartbeat

r[properties.heartbeat-interval-less-than-election]
raft_timing::validate_timing_order(heartbeat, election_min) ensures
heartbeat < election_min. Combined with heartbeat::should_send, a
leader sends multiple heartbeats per election timeout period.

r[properties.heartbeat-reset-prevents-election]
If should_reset_election_timer returns true and the follower resets,
then is_leader_alive returns true (leader still considered alive).
Valid heartbeats prevent spurious elections.

r[properties.heartbeat-ack-stale-triggers-step-down]
A heartbeat ack with ack_term > leader_term triggers
term_authority::apply_higher_term, which returns a TermUpdate with
new_state == FOLLOWER.

## Cross-Module: Client Session

r[properties.session-register-is-strict-advance]
Successive register_session calls produce session IDs satisfying
monotonic::is_strict_advance. Session IDs never repeat.

r[properties.session-duplicate-is-fencing]
is_duplicate_request(table_seq, req_seq) mirrors
fencing::is_valid_write(req_seq, table_seq) == false when req_seq
<= table_seq. Duplicate detection is stale-write rejection.

r[properties.session-expire-is-expiration]
is_session_expired(last_activity, now, timeout) MUST match
expiration::is_expired(last_activity + timeout, now). Session expiry
uses the same temporal predicate.

## Cross-Module: Lease Read

r[properties.lease-read-implies-read-index-safe]
When lease_read_safe returns true, read_index::read_safe_at(read_index,
commit_index) also returns true. Lease reads are strictly stronger than
ReadIndex — they add a lease validity check on top of the commit check.

r[properties.lease-disjoint-is-lease-disjointness]
The lease-read disjointness property relies on lease::leases_disjoint
when the same resource_id is used. The two modules agree on temporal
non-overlap.

## Check Quorum

r[check-quorum.recent-contact]
has_recent_contact(last_contact, now, election_timeout) MUST return
true when now - last_contact < election_timeout (saturating
subtraction). A follower that has contacted the leader within the
election timeout is considered recent.

r[check-quorum.count-recent]
count_recent_contacts(contacts, now, election_timeout, cluster_size)
MUST return the count of servers (including self) with recent contact.
The leader always counts itself (contact time == now).

r[check-quorum.should-step-down]
should_step_down(recent_count, cluster_size) MUST return true when
recent_count < majority(cluster_size). A leader that cannot confirm
a quorum of recent contacts MUST step down to prevent serving stale
data from a network partition.

r[check-quorum.leader-counts-self]
The leader's own contact time is always current (now), so it always
counts as 1 towards the recent quorum. A singleton cluster (size 1)
MUST never step down.

r[check-quorum.partitioned-leader]
A fully partitioned leader (only self responding) with cluster_size > 1
MUST step down. recent_count == 1 < majority(n) for n >= 2.

## Learner Readiness

r[learner-readiness.is-caught-up]
is_caught_up(learner_match, leader_log_len) MUST return true when
learner_match >= leader_log_len. The learner has replicated all
entries the leader currently has.

r[learner-readiness.replication-gap]
replication_gap(learner_match, leader_log_len) MUST return
leader_log_len - learner_match when leader_log_len >= learner_match,
else 0. This is the number of entries the learner still needs.

r[learner-readiness.is-within-threshold]
is_within_threshold(learner_match, leader_log_len, threshold) MUST
return true when replication_gap <= threshold. The learner is close
enough to be promoted without significant downtime.

r[learner-readiness.promotion-safe]
promotion_safe(learner_match, leader_log_len, cluster_size) MUST
return true when is_caught_up AND cluster_size > 0. Promotion requires
both replication completeness and a valid cluster.

r[learner-readiness.gap-monotonic]
If learner_match increases (more entries replicated), the replication
gap MUST decrease or stay the same. Monotonic progress toward readiness.

## Replication Pipeline

r[replication-pipeline.inflight-count]
inflight_count(next_index, match_index) MUST return next_index -
match_index - 1 when next_index > match_index + 1, else 0. This is
the number of entries sent but not yet acknowledged.

r[replication-pipeline.can-send-more]
can_send_more(next_index, match_index, max_inflight) MUST return true
when inflight_count < max_inflight. The leader can pipeline more
entries without exceeding the flow control window.

r[replication-pipeline.window-after-send]
window_after_send(next_index, match_index, entries_sent, max_inflight)
MUST return the new next_index (next_index + entries_sent). The caller
MUST ensure entries_sent <= available window (max_inflight -
inflight_count).

r[replication-pipeline.window-after-ack]
window_after_ack(match_index, new_match) MUST return new_match when
new_match > match_index. Acknowledging entries frees window capacity.
The new inflight count MUST be <= the old inflight count.

r[replication-pipeline.full-window-blocks]
When inflight_count >= max_inflight, can_send_more MUST return false.
A full pipeline blocks further sends until acks arrive.

r[replication-pipeline.empty-window]
When next_index == match_index + 1, inflight_count MUST be 0 and
can_send_more MUST return true (assuming max_inflight > 0). A fully
acknowledged pipeline has full capacity.

## Cross-Module: Check Quorum

r[properties.check-quorum-uses-quorum]
should_step_down delegates to !is_quorum(recent_count, cluster_size).
The step-down decision is the negation of the quorum predicate.

r[properties.check-quorum-singleton-safe]
A singleton cluster (size 1) with leader counting self MUST have
recent_count == 1 == majority(1), so should_step_down returns false.
Singletons never step down.

## Cross-Module: Learner Readiness

r[properties.learner-caught-up-is-advance]
is_caught_up(match, len) is equivalent to monotonic::is_advance(len,
match). The learner's match index has reached the leader's log length.

r[properties.learner-gap-is-saturating-sub]
replication_gap(match, len) equals arithmetic::saturating_sub_spec(len,
match). The gap is a saturating subtraction.

## Cross-Module: Replication Pipeline

r[properties.pipeline-inflight-is-saturating-sub]
inflight_count(next, match) equals
arithmetic::saturating_sub_spec(next, match + 1) when next > match.
Inflight tracking is a saturating subtraction.

r[properties.pipeline-empty-after-full-ack]
When new_match == next_index - 1 (all sent entries acknowledged),
inflight_count(next_index, new_match) == 0. Full acknowledgment
empties the pipeline.

## Cross-Module: Snapshot Transfer

r[properties.snapshot-chunk-count-is-div-ceil]
chunk_count(total, chunk_size) MUST equal arithmetic::div_ceil(total,
chunk_size). Chunking and ceiling division are the same computation.

r[properties.snapshot-transfer-triggers-install]
After a complete snapshot transfer (all chunks received), the follower
applies install_snapshot::apply_snapshot_commit. The transfer layer
feeds into the installation layer.

## Follower Commit Update

r[follower-commit.update]
follower_update_commit(current_commit, leader_commit, last_new_entry_index)
MUST return min(leader_commit, last_new_entry_index) when that value exceeds
current_commit, otherwise current_commit. This is the follower-side commit
advancement from Raft's AppendEntries receiver (§5.3).

r[follower-commit.monotonic]
The result of follower_update_commit MUST always be >= current_commit. The
follower's commit index MUST never decrease.

r[follower-commit.bounded-by-leader]
The result MUST always be <= leader_commit. A follower MUST NOT commit
beyond what the leader has committed.

r[follower-commit.bounded-by-log]
The result MUST always be <= last_new_entry_index. A follower MUST NOT
commit beyond the last entry it actually has in its log.

## Election Timer

r[election-timer.reset-on-append-entries]
should_reset_on_append_entries(msg_term, current_term, is_success) MUST
return true when msg_term >= current_term AND is_success is true. A valid
AppendEntries from the current leader resets the follower's election timer.
Stale-term messages MUST NOT reset the timer.

r[election-timer.reset-on-vote-grant]
should_reset_on_vote_grant(voted) MUST return true when voted is true.
Granting a vote resets the election timer because a valid candidate exists.

r[election-timer.reset-on-election-start]
should_reset_on_election_start(became_candidate) MUST return true when
became_candidate is true. Starting an election resets the timer so the
candidate waits a full timeout before retrying.

r[election-timer.expired]
is_election_timer_expired(elapsed_ms, election_timeout_ms) MUST return
true when elapsed_ms >= election_timeout_ms. The timer fires when the
full timeout has elapsed without a reset event.

r[election-timer.should-start-election]
should_start_election(is_follower, timer_expired) MUST return true when
both conditions hold. Only followers with expired timers start elections.
Leaders and candidates MUST NOT start new elections via timer.

## Candidate Election Initiation

r[election-init.new-term]
init_election_term(current_term) MUST return current_term + 1. The
candidate increments its term before requesting votes. MUST return None
when current_term == u64::MAX (overflow protection).

r[election-init.self-vote]
init_self_vote(candidate_id, new_term) MUST return a VoteState with
voted_term == new_term and voted_for == candidate_id. The candidate
votes for itself as the first step of the election.

r[election-init.initial-vote-count]
init_vote_count() MUST return 1. The candidate starts with one vote
(its own self-vote).

r[election-init.new-state]
init_election_state() MUST return STATE_TRANSITIONING. The candidate
transitions from Follower to Transitioning (candidate) state.

r[election-init.self-vote-is-valid]
The self-vote produced by init_self_vote MUST satisfy
can_grant_vote_spec when the candidate's log is compared against
itself. A node always considers its own log up-to-date enough.

## Vote Response Tally

r[vote-tally.record-vote]
record_vote(votes, server_id, cluster_size) MUST set votes[server_id]
to true when server_id < cluster_size. The operation MUST NOT modify
any other server's vote. Duplicate votes for the same server MUST be
idempotent (already true stays true).

r[vote-tally.count-votes]
count_votes(votes) MUST return the number of true entries in the vote
array. The count MUST be <= votes.len().

r[vote-tally.has-won]
has_won_election(votes, cluster_size) MUST return true when
count_votes(votes) constitutes a quorum of cluster_size. Delegates
to quorum::is_quorum.

r[vote-tally.is-stale-response]
is_stale_vote_response(response_term, current_term) MUST return true
when response_term != current_term. Votes from a different term MUST
be ignored. Only same-term votes count toward election victory.

r[vote-tally.init-votes]
init_vote_tally(cluster_size) MUST return a sequence of cluster_size
false values. No server has voted yet.

r[vote-tally.init-with-self-vote]
init_with_self_vote(cluster_size, self_id) MUST return a tally with
votes[self_id] set to true and all others false. The candidate starts
with its own vote counted.

## AppendEntries Response Dispatch

r[append-response.classify]
classify_append_response(response_term, current_term, success) MUST
return: HIGHER_TERM (0) when response_term > current_term, STALE (1)
when response_term < current_term, SUCCESS (2) when response_term ==
current_term and success is true, FAILURE (3) when response_term ==
current_term and success is false. The four cases MUST be exhaustive
and mutually exclusive.

r[append-response.on-success]
on_append_success(match_index, next_index, entries_sent) MUST return
(new_match, new_next) where new_match = match_index + entries_sent and
new_next = new_match + 1. The match index advances to cover the
acknowledged entries. The next index moves past them.

r[append-response.on-failure]
on_append_failure(next_index) MUST return max(next_index - 1, 1).
The leader backs off by one index, with a floor of 1. This is the
basic (non-accelerated) backtrack from §5.3.

r[append-response.match-advances]
After on_append_success, the new match index MUST be >= the old match
index. Match indices are monotonic.

r[append-response.next-after-success]
After on_append_success, new_next MUST equal new_match + 1. The next
index always points one past the match index after success.

## Log Entry Type Routing

r[entry-type.classify]
classify_entry(entry) MUST return ENTRY_NORMAL (0) when entry.value !=
NOOP_VALUE and entry.value != CONFIG_CHANGE_SENTINEL, ENTRY_NOOP (1)
when entry.value == NOOP_VALUE, ENTRY_CONFIG (2) when entry.value ==
CONFIG_CHANGE_SENTINEL. The three types MUST be exhaustive.

r[entry-type.should-apply]
should_apply_to_state_machine(entry_type) MUST return true for
ENTRY_NORMAL, false for ENTRY_NOOP and ENTRY_CONFIG. Only normal
command entries affect state machine output. No-ops and config changes
are handled separately.

r[entry-type.is-config-change]
is_config_change(entry_type) MUST return true only for ENTRY_CONFIG.
Config changes take effect when appended to the log, not when committed.

r[entry-type.takes-effect-on-append]
takes_effect_on_append(entry_type) MUST return true for ENTRY_CONFIG,
false for ENTRY_NORMAL and ENTRY_NOOP. Config entries are the only
type that changes system behavior on append rather than commit.

r[entry-type.is-noop]
is_noop_entry(entry_type) MUST return true only for ENTRY_NOOP. No-op
entries are identity operations inserted by new leaders.

## Cross-Module: Follower Commit

r[properties.follower-commit-monotonic-is-advance]
follower_update_commit result satisfies monotonic::is_advance on the
commit index. The follower's commit cursor never moves backward.

## Cross-Module: Election Timer

r[properties.election-timer-heartbeat-resets]
If should_reset_on_append_entries returns true and the follower resets
its timer, then is_election_timer_expired returns false (timer not
expired immediately after reset).

## Cross-Module: Vote Tally

r[properties.vote-tally-quorum-is-quorum]
has_won_election delegates to quorum::is_quorum. Election victory
requires the same majority threshold as any other quorum decision.

## Cross-Module: AppendEntries Response

r[properties.append-response-success-advances-match]
on_append_success produces a match index that satisfies
monotonic::is_advance on the previous match index.

## Cross-Module: Entry Type

r[properties.entry-type-noop-agrees]
classify_entry with NOOP_VALUE produces ENTRY_NOOP, which agrees with
noop_commit::is_noop_spec.

## Replication Mode

r[replication-mode.valid-mode]
is_valid_mode MUST return true if and only if mode is PROBE (0),
REPLICATE (1), or SNAPSHOT (2). All transitions MUST preserve validity.

r[replication-mode.can-pipeline]
can_pipeline MUST return true if and only if mode is REPLICATE. Probe
and Snapshot modes MUST NOT allow pipelining.

r[replication-mode.can-send-entries]
can_send_entries MUST return true for PROBE and REPLICATE, false for
SNAPSHOT. Snapshot mode sends snapshot chunks, not log entries.

r[replication-mode.transition-on-success]
transition_on_success MUST move PROBE to REPLICATE (match point
discovered). REPLICATE stays in REPLICATE. SNAPSHOT is unchanged.

r[replication-mode.transition-on-reject]
transition_on_reject MUST move REPLICATE back to PROBE (match point
wrong). PROBE stays in PROBE. SNAPSHOT is unchanged.

r[replication-mode.transition-to-snapshot]
should_send_snapshot MUST return true when next_index <= first_log_index.
The follower needs entries that have been compacted.

r[replication-mode.enter-snapshot]
enter_snapshot MUST switch PROBE or REPLICATE to SNAPSHOT. Already in
SNAPSHOT is unchanged.

r[replication-mode.snapshot-complete]
snapshot_complete MUST switch SNAPSHOT to PROBE (resume probing from
snapshot's last index). Other modes unchanged.

r[replication-mode.entries-per-message]
entries_per_message MUST return 1 for PROBE, max_batch for REPLICATE,
and 0 for SNAPSHOT.

## Pre-Vote Tally

r[pre-vote-tally.init]
init_pre_vote_tally MUST create a boolean sequence of cluster_size
with all values false.

r[pre-vote-tally.init-with-self]
init_with_self_pre_vote MUST create a tally with only the candidate's
own position set to true.

r[pre-vote-tally.record]
record_pre_vote MUST set the server's position to true. Recording MUST
be idempotent. Other positions MUST be unchanged.

r[pre-vote-tally.count]
count_pre_votes MUST return the number of true entries. The count MUST
be bounded by the tally length.

r[pre-vote-tally.has-quorum]
has_pre_vote_quorum MUST return true if and only if the pre-vote count
constitutes a quorum of cluster_size.

r[pre-vote-tally.stale-response]
is_stale_pre_vote_response MUST return true when the response term
differs from the prospective term.

## Follower Read

r[follower-read.register]
register_read MUST return the follower's current commit_index as the
read's minimum freshness guarantee.

r[follower-read.leader-confirms]
safe_read_point MUST return max(read_index, leader_commit). The safe
point MUST be >= both the original read_index and the leader's commit.

r[follower-read.can-serve]
can_serve_read MUST return true if and only if last_applied >= safe_point.
can_serve and entries_behind == 0 MUST be equivalent.

r[follower-read.stale-leader]
is_leader_stale MUST return true when the response term differs from
the registered term. Same term MUST return false.

r[follower-read.timeout]
is_read_timed_out MUST return true when now >= registered_at + timeout.
At registration time with timeout > 0, MUST return false.

r[follower-read.entries-behind]
entries_behind MUST return safe_point - last_applied when behind, 0 when
caught up. Applying entries MUST reduce the behind count.

## Request Forward

r[request-forward.can-forward]
can_forward MUST return true only when is_follower AND leader_known AND
pending_count < max_pending. Non-followers MUST NOT forward.

r[request-forward.after-forward]
after_forward MUST return pending_count + 1.

r[request-forward.after-response]
after_response MUST return pending_count - 1 when positive, 0 when zero.
Forward then response MUST be a roundtrip.

r[request-forward.expired]
is_forward_expired MUST return true when now >= forward_time + timeout.

r[request-forward.clear-on-leader-change]
should_clear_pending MUST return true when new_term > old_leader_term.
Same term MUST return false.

r[request-forward.decision]
forward_decision MUST return ACCEPT for leaders, FORWARD when
can_forward holds, REJECT otherwise. The result MUST be 0, 1, or 2.

## Commit Notify

r[commit-notify.count]
newly_committed_count MUST return new_commit - old_commit when
advanced, 0 otherwise. No advancement means zero.

r[commit-notify.is-newly-committed]
is_newly_committed MUST return true for indices in (old_commit,
new_commit]. The old commit and indices beyond new_commit MUST be false.

r[commit-notify.first-committed]
first_newly_committed MUST return old_commit + 1.

r[commit-notify.should-notify]
should_notify MUST return true if and only if newly_committed_count > 0.

r[commit-notify.proposal-status]
proposal_status MUST return COMMITTED when index <= commit_index (wins
over expired), EXPIRED when not committed and expired, PENDING otherwise.
Committed MUST be monotonic: once committed, stays committed.

r[commit-notify.count-ready]
count_ready_proposals MUST count proposals that are committed or expired.
The count MUST be bounded by the input length.

## Compaction Fence

r[compaction-fence.safe-to-compact]
is_safe_to_compact MUST return true when compact_index < min_next_index.
Compaction at 0 MUST always be safe.

r[compaction-fence.min-next-index]
min_active_next MUST return the minimum next_index across followers that
are NOT in snapshot mode, excluding self. All-snapshot MUST return MAX.

r[compaction-fence.needs-snapshot]
needs_snapshot_switch MUST return true when follower_next <= first_log_index.
Within-log followers MUST return false.

r[compaction-fence.safe-compact-index]
safe_compact_index MUST return min(desired, min_next - 1). The result
MUST be < min_next and <= desired.

## Witness

r[witness.valid-role]
is_valid_role MUST return true for VOTER (0), WITNESS (1), LEARNER (2).

r[witness.can-vote]
can_vote MUST return true for VOTER and WITNESS, false for LEARNER.

r[witness.can-be-leader]
can_be_leader MUST return true only for VOTER. Leader implies voter.
Leader implies full log.

r[witness.receives-log]
receives_full_log MUST return true for VOTER and LEARNER, false for
WITNESS.

r[witness.can-ack-heartbeat]
can_ack_heartbeat MUST return true for VOTER and WITNESS when
leader_term >= my_term. Stale heartbeats MUST be rejected.

r[witness.entry-retained]
entry_retained MUST return true when entry_index >= snapshot_index.

r[witness.quorum-with-witnesses]
quorum_members MUST return voter_count + witness_count. Witnesses
contribute to quorum count.

r[witness.reject-witness-candidate]
should_reject_candidate MUST return true for WITNESS and LEARNER.
Voter candidates MUST NOT be rejected.

## Priority Election

r[priority-election.should-yield]
should_yield MUST return true when other_priority > my_priority AND
other_log_current. Same priority MUST NOT yield. Outdated log MUST NOT
cause yield.

r[priority-election.delay]
election_delay MUST return base_timeout + (max_priority - my_priority)
* delay_per_level (saturating). Highest priority MUST get base_timeout.
Lower priority MUST get >= base_timeout.

r[priority-election.step-down]
should_step_down_for_priority MUST return true when a higher-priority
node is ready AND the grace period has expired.

r[priority-election.compare-candidates]
compare_candidates MUST prefer the candidate with the fresher log
regardless of priority. When logs are equal, higher priority wins.
Equal candidates MUST return 0.

r[priority-election.is-highest]
is_highest_priority MUST return true when my_priority >= max_cluster_priority.

## Learner Change

r[learner-change.can-add]
can_add_learner MUST return false for voters, during pending config
changes, and when at max learners. Adding an existing learner MUST
always be allowed (idempotent).

r[learner-change.add-result]
add_learner_result MUST return count + 1 for new learners, count for
existing learners (idempotent).

r[learner-change.can-remove]
can_remove_learner MUST return false for non-learners, promoting
learners, and during pending config changes.

r[learner-change.remove-result]
remove_learner_result MUST return count - 1 for learners with count > 0,
count for non-learners. Add then remove MUST be a roundtrip.

r[learner-change.quorum-unaffected]
quorum_unchanged MUST verify voter_count_before == voter_count_after.
Learner operations MUST NOT change the quorum calculation.

## Pre-Vote Lease

r[pre-vote-lease.reject]
should_reject_pre_vote MUST return true when has_leader_contact AND
candidate_term <= my_term. No contact MUST NOT reject. Higher candidate
term MUST bypass the check. Partitioned same-term nodes MUST be blocked.

r[pre-vote-lease.combined-decision]
combined_pre_vote_decision MUST compose CheckQuorum rejection with
standard pre-vote check. Rejection prevents grant. Without CheckQuorum,
reduces to standard pre-vote.

r[pre-vote-lease.leader-contact]
has_leader_within_timeout MUST return true when now - last_contact <
election_timeout. Zero elapsed with timeout > 0 MUST be true. Elapsed
>= timeout MUST be false.

## Heartbeat

r[heartbeat.is-heartbeat]
is_heartbeat(entry_count) MUST return true if and only if entry_count == 0.
A heartbeat is an AppendEntries RPC carrying no log entries.

r[heartbeat.should-send]
should_send_heartbeat(elapsed_ms, heartbeat_interval_ms) MUST return true
when elapsed_ms >= heartbeat_interval_ms. The leader MUST send heartbeats
at the configured interval.

r[heartbeat.reset-election-timer]
should_reset_election_timer(msg_term, current_term, from_leader) MUST
return true when msg_term >= current_term AND from_leader is true. A
valid heartbeat from the current leader resets the follower's election
timer. Messages from non-leaders or stale terms MUST NOT reset the timer.

r[heartbeat.leader-alive]
is_leader_alive(last_heartbeat_time, now, election_timeout) MUST return
true when now - last_heartbeat_time < election_timeout (using saturating
subtraction). A follower considers the leader alive while heartbeats
arrive within the election timeout.

r[heartbeat.ack]
heartbeat_ack_valid(ack_term, leader_term) MUST return true when
ack_term == leader_term. Stale acks (ack_term < leader_term) MUST be
ignored. Higher-term acks trigger step-down via term_authority.

## Cross-Module: Heartbeat

r[properties.heartbeat-interval-less-than-election]
raft_timing::validate_timing_order(heartbeat, election_min) ensures
heartbeat < election_min. Combined with heartbeat::should_send, a
leader sends multiple heartbeats per election timeout period.

r[properties.heartbeat-reset-prevents-election]
If should_reset_election_timer returns true and the follower resets,
then is_leader_alive returns true (leader still considered alive).
Valid heartbeats prevent spurious elections.

r[properties.heartbeat-ack-stale-triggers-step-down]
A heartbeat ack with ack_term > leader_term triggers
term_authority::apply_higher_term, which returns a TermUpdate with
new_state == FOLLOWER.

## Log Compaction

r[log-compaction.should-compact]
should_compact(log_len, snapshot_threshold) MUST return true when
log_len >= snapshot_threshold. A threshold of 0 MUST always trigger.
A log shorter than the threshold MUST NOT trigger.

r[log-compaction.compact]
compact_log(log, snapshot_index) MUST discard all entries at indices
[0, snapshot_index) and return the suffix starting at snapshot_index.
The result length MUST equal log.len() - snapshot_index when
snapshot_index <= log.len(). If snapshot_index > log.len(), the
result MUST be empty.

r[log-compaction.prefix-removed]
After compaction, no entry from the original [0, snapshot_index) range
is present in the result. The first entry of the result (if non-empty)
corresponds to the original entry at snapshot_index.

r[log-compaction.suffix-preserved]
All entries at indices [snapshot_index, log.len()) MUST be preserved
in the result, with their terms and values unchanged. For all i in
[0, result.len()): result[i] == original[snapshot_index + i].

r[log-compaction.first-index-after]
first_log_index_after_compaction(prev_first_index, snapshot_index) MUST
return max(prev_first_index, snapshot_index). The first log index MUST
never decrease.

r[log-compaction.offset-translation]
translate_index(absolute_index, first_log_index) MUST return
absolute_index - first_log_index when absolute_index >= first_log_index,
and None otherwise. Indices before the compacted region are inaccessible.

r[log-compaction.compact-preserves-commit]
Compacting entries below the commit index MUST NOT change the commit
index. snapshot_index MUST be <= commit_index (never compact uncommitted
entries).

## Cross-Module: Log Compaction

r[properties.compact-suffix-length-is-saturating-sub]
The length of compact_log result MUST equal
arithmetic::saturating_sub_spec(log.len(), snapshot_index). Compaction
length is a saturating subtraction.

r[properties.compact-first-index-is-max]
first_log_index_after_compaction(prev, snap) MUST equal
ordering::max_spec(prev, snap). The first index is the max of old and
new.

r[properties.compact-offset-uses-checked-sub]
translate_index returns None exactly when the absolute index is below
first_log_index — same condition as a checked subtraction underflow.

r[properties.compact-preserves-install-snapshot-trigger]
After compaction with snapshot_index S, should_install_snapshot(next, S)
holds for any follower with next <= S. Compaction creates the exact
condition that triggers snapshot installation.

## Follower Redirect

r[follower-redirect.should-redirect]
should_redirect(state, known_leader_id) MUST return true when state !=
Leader AND known_leader_id is Some. Followers and candidates with a known
leader MUST redirect clients. A follower with no known leader MUST NOT
redirect (return error instead).

r[follower-redirect.leader-hint]
leader_hint(known_leader_id) MUST return the leader's node ID when known,
or a sentinel (u64::MAX) when unknown. The hint is best-effort — it MAY
be stale after a leader change.

r[follower-redirect.hint-freshness]
is_hint_fresh(hint_term, current_term) MUST return true when hint_term ==
current_term. A hint from a prior term MUST be considered stale. Clients
receiving a stale hint MUST retry discovery.

r[follower-redirect.leader-accepts]
should_accept_request(state) MUST return true if and only if state ==
Leader. Only the leader processes client requests directly.

## Configuration Change

r[config-change.is-valid-phase]
is_valid_phase MUST return true if and only if phase is STABLE (0) or
JOINT (1). All transitions MUST preserve validity.

r[config-change.can-start]
can_start_config_change MUST return true if and only if current_phase is
STABLE AND new_config_size > 0. A change MUST NOT start during the joint
phase. An empty new config MUST be rejected.

r[config-change.enter-joint]
enter_joint MUST return PHASE_JOINT. Requires current_phase == STABLE.

r[config-change.leave-joint]
leave_joint MUST return PHASE_STABLE. Requires current_phase == JOINT.

r[config-change.is-joint-committed]
is_joint_committed MUST return true when commit_index >= joint_entry_index.

r[config-change.can-leave-joint]
can_leave_joint MUST return true when phase is JOINT AND the joint config
entry is committed.

r[config-change.config-for-quorum]
config_for_quorum MUST return QUORUM_JOINT for PHASE_JOINT, QUORUM_SINGLE
for PHASE_STABLE.

r[config-change.leader-in-new-config]
leader_in_new_config MUST return true if the leader's ID appears in the
new config member list.

r[config-change.should-step-down]
should_leader_step_down MUST return true when phase is STABLE, C_new is
committed, and the leader is NOT in the new config. The leader must step
down after completing a config change that removes it.

## Persistent State

r[persistent-state.vote-response]
must_persist_before_vote_response MUST return PERSIST_TERM | PERSIST_VOTED_FOR
when vote_granted is true (must persist term and votedFor before acking a
grant). MUST return PERSIST_TERM when vote is denied (persist updated term).

r[persistent-state.append-response]
must_persist_before_append_response MUST return PERSIST_LOG | PERSIST_TERM
when entries were appended (entries_appended true). MUST return PERSIST_TERM
when only the term changed (no new entries).

r[persistent-state.needs-persist]
needs_persist(persist_set, component) MUST return true when the component
bit is set in the persist_set bitmask. Components: PERSIST_TERM (1),
PERSIST_VOTED_FOR (2), PERSIST_LOG (4).

r[persistent-state.recover]
is_recovered_state_valid MUST check that recovered state is internally
consistent: term >= voted_term (cannot have voted in a future term).

r[persistent-state.vote-safety]
vote_safe_after_recovery MUST return true only when the recovered votedFor
for the given term allows voting. If the node already voted in this term
for a different candidate, it MUST return false.

r[persistent-state.term-recovery]
term_after_recovery MUST return the recovered term. The recovered term MUST
be >= any term seen before the crash.

## Fast Backtrack

r[fast-backtrack.conflict-hint]
conflict_hint MUST return the term and first index of the conflicting entry
from the follower's log. When the log is too short (no entry at prev_log_index),
MUST return conflict_term=0 and first_index=0 with the log length.

r[fast-backtrack.first-index-of-term]
first_index_of_term MUST scan backward from a starting position to find
the first entry with the given term. Used to build conflict hints.

r[fast-backtrack.fast-backtrack]
fast_backtrack MUST compute the new next_index using the conflict hint.
When the leader has entries with the conflicting term, next_index advances
to just past the leader's last entry for that term. When the leader has
no entry with the conflicting term, next_index jumps to the follower's
first_index_for_term. When the follower's log was too short,
next_index jumps to follower_log_length + 1.

r[fast-backtrack.leader-find-last]
leader_find_last_index_for_term MUST scan the leader's log to find the
last entry with the given term. Returns 0 when no entry with that term
exists.

## Client Routing

r[client-routing.route]
route_client_request MUST return ACCEPT for leaders, REDIRECT for
non-leaders with a known leader, and REJECT for non-leaders with no
known leader.

r[client-routing.should-accept]
should_accept MUST return true if and only if state == Leader.

r[client-routing.should-redirect]
should_redirect MUST return true when the node is not a leader and a
leader ID is known (leader_id != sentinel).

r[client-routing.leader-stale]
is_leader_stale MUST return true when the leader hint was learned in a
prior term (hint_term < current_term).

r[client-routing.route-read]
route_read_request MUST return ACCEPT_LOCAL for leaders, FORWARD for
non-leaders with a known leader (when follower reads are enabled),
and REJECT otherwise.

## Escrow

r[escrow.valid-state]
is_valid_state MUST return true for exactly: CREATED (0), FUNDED (1),
RELEASED (2), REFUNDED (3), DISPUTED (4). All transitions MUST
preserve validity.

r[escrow.terminal]
RELEASED and REFUNDED MUST be terminal — no outgoing transitions.
Terminal states MUST block all transitions.

r[escrow.transition]
Valid transitions: Created→Funded, Funded→Released, Funded→Refunded,
Funded→Disputed, Disputed→Released, Disputed→Refunded. All other
transitions MUST be rejected. Created has exactly one outgoing edge.
Funded has three. Disputed has two (both terminal).

r[escrow.fund]
fund MUST debit the buyer and transition to FUNDED. Requires state ==
CREATED, amount > 0, and buyer has sufficient funds. The buyer's new
balance MUST equal old - amount. Fund-then-refund MUST return the buyer
to the original balance.

r[escrow.release]
release MUST credit the seller and transition to RELEASED. Requires
state == FUNDED or DISPUTED, amount > 0, and seller balance won't
overflow. The seller's new balance MUST equal old + amount.

r[escrow.refund]
refund MUST credit the buyer and transition to REFUNDED. Requires
state == FUNDED or DISPUTED, amount > 0, and buyer balance won't
overflow. The buyer's new balance MUST equal old + amount.

r[escrow.dispute]
dispute MUST transition FUNDED to DISPUTED. All other states MUST
reject. Dispute is a state-only operation — no balance changes.

r[escrow.conservation]
Total value (buyer + seller + escrow) MUST be constant across all
operations. Fund-then-release preserves the three-party total.
Fund-then-refund preserves the three-party total. Fund-dispute-release
and fund-dispute-refund also preserve totals.

## Fee Computation

r[fee.compute]
compute_fee MUST return floor(amount * rate_bps / 10000). Fee MUST be
bounded: 0 <= fee <= amount. Zero amount yields zero fee. Zero rate
yields zero fee. Full rate (10000 bps) takes the entire amount. Fee
MUST be monotonic in both amount and rate.

r[fee.net]
compute_net MUST return amount - fee. Net MUST be non-negative. Zero
rate means net equals amount. Full rate means net equals zero.

r[fee.conservation]
fee + net MUST equal amount for all valid inputs. Value is neither
created nor destroyed by fee computation.

r[fee.valid-rate]
is_valid_rate MUST return true if and only if rate_bps <= 10000.

r[fee.split]
split_with_fee MUST return (fee, net) where fee + net = amount. Split
conservation MUST hold for all valid rates.

r[fee.tiered]
tiered_fee MUST return max(proportional_fee, min_fee) capped at amount.
When proportional exceeds min, use proportional. When min exceeds
proportional but not amount, use min. When min exceeds amount, use
amount. Tiered fee MUST never exceed amount. When amount >= min_fee,
the tiered fee MUST be >= min_fee.

## Cross-Module: Financial

r[properties.escrow-fund-uses-debit]
Escrow fund_buyer_after_spec(buyer, amount) MUST equal
balance::debit_spec(buyer, amount). Escrow funding is a balance debit.

r[properties.escrow-release-uses-credit]
Escrow release_seller_after_spec(seller, amount) MUST equal
balance::credit_spec(seller, amount). Escrow release is a balance credit.

r[properties.escrow-terminal-like-process]
Escrow is_terminal mirrors process_state::is_terminal — both define
absorbing states with no outgoing transitions.

r[properties.fee-conservation-matches-transfer]
For a valid fee split, fee + net = amount mirrors transfer conservation
(sender + receiver = total). Fee computation preserves value.

r[properties.fee-zero-rate-identity]
compute_net(amount, 0) MUST equal amount, matching the identity
property of zero-amount transfers.

## Multi-Account Ledger

r[ledger.total]
total_balance MUST return the sum of all account balances. Empty ledger
total MUST be zero. Single-account total MUST equal that balance. Sum
MUST be non-negative. Returns None on overflow.

r[ledger.balance-at]
balance_at MUST return the balance at the given account index.

r[ledger.account-count]
account_count MUST return the number of accounts in the ledger.

r[ledger.can-transfer]
can_transfer MUST return true if and only if: both indices are in range,
indices differ (no self-transfer), amount > 0, sender has sufficient funds,
and receiver won't overflow. Self-transfer MUST always be rejected. Zero
amount MUST always be rejected.

r[ledger.transfer-result]
ledger_transfer MUST return (new_sender, new_receiver) where sender is
debited by amount and receiver is credited by amount. The ledger length
MUST be preserved. Only the sender and receiver accounts change — all
others MUST be identical to the original.

r[ledger.conservation]
After any valid transfer, the sum of all account balances MUST equal the
sum before the transfer. Two successive valid transfers MUST preserve the
total. A transfer with fee (sender pays amount, receiver gets amount - fee,
fee collector gets fee) also MUST preserve the total.

r[ledger.transfer-with-fee]
ledger_transfer_with_fee MUST debit sender by amount, credit receiver by
amount - fee, and credit fee collector by fee. All three indices MUST be
distinct. Fee MUST be <= amount. The total across all accounts MUST be
preserved.

r[ledger.batch]
apply_transfers applies a sequence of transfers in order. When all
transfers are valid (all_transfers_valid), the batch MUST preserve the
total balance. The batch MUST preserve ledger length.

## Cross-Module: Ledger

r[properties.ledger-transfer-uses-balance]
Ledger transfer sender result MUST equal balance::debit_spec and receiver
result MUST equal balance::credit_spec. The ledger composes balance
operations.

r[properties.ledger-conservation-is-double-entry]
A ledger transfer debiting and crediting the same amount is a balanced
entry in the double_entry sense: is_balanced_spec(seq![amount], seq![amount]).

## Settlement and Netting

r[settlement.int-sum]
int_sum_spec MUST compute the recursive sum of a Seq<int>. Sum of empty
is zero. Sum of all-zeros is zero.

r[settlement.obligation-valid]
An obligation is valid when both party indices are in range, distinct,
and amount > 0. Swapping sender and receiver negates both parties'
positions.

r[settlement.all-valid]
all_obligations_valid MUST check every obligation in the sequence against
obligation_valid.

r[settlement.apply-obligation]
apply_obligation MUST debit the sender and credit the receiver in the
position vector. The operation MUST preserve the sum of positions.

r[settlement.compute-positions]
compute_positions MUST compute net positions by applying all obligations
to an all-zeros vector. The result length MUST equal num_parties. Empty
obligations produce all-zero positions.

r[settlement.positions-sum-zero]
Net positions MUST always sum to zero. This is the fundamental conservation
theorem: each obligation contributes +amount to one party and -amount to
another. Single obligations also sum to zero.

r[settlement.valid-settlement]
A settlement is valid when it produces the same net positions as the
original obligations. The original is always a valid settlement of
itself. A valid settlement preserves every party's net position.

r[settlement.obligation-count]
obligation_count MUST return the number of obligations.

r[settlement.net-position-at]
net_position_at MUST return the net position for a single party.

r[settlement.apply-preserves-sum]
Applying one obligation preserves the sum of positions. By induction,
applying all obligations preserves the sum.

r[settlement.apply-preserves-length]
Applying obligations preserves the position vector length.

r[settlement.self-settlement]
Every set of obligations is a valid settlement of itself (reflexivity).

## Verification Gap Inventory

53 items use `#[verifier::external_body]`.

### Permanent trust boundaries (15 items)
These remain by design or because the current abstraction boundary is the
point of the model, not a proof target.

#### Cryptographic axioms / exec bridges (6 items)
Cryptographic primitives with closed/uninterpreted spec functions.
These model idealized hash functions (determinism, collision resistance)
and runtime crypto implementations. No proof is possible without opening
the hash function, which would defeat the abstraction.

- `chain::hash_deterministic` — hash_spec determinism axiom
- `chain::hash_collision_free` — hash_spec collision-free axiom
- `chain::compute_entry_hash` — runtime hash (blake3 in production)
- `snapshot_integrity::dual_hash_deterministic` — dual_hash_spec determinism
- `snapshot_integrity::dual_hash_collision_resistant` — dual_hash_spec collision resistance
- `snapshot_integrity::dual_hash` — runtime dual hash

#### Z3 rlimit / complexity limits (7 items)
Proofs where Z3 cannot handle the spec complexity within reasonable rlimit.
Proof bodies are structurally correct; proptest covers runtime correctness.

- `nix_version::cv_fuel_antisymmetric` — 8+ nested let+if/else levels
- `nix_version::cv_fuel_reflexive` — same deep spec
- `nix_version::cv_fuel_range` — same deep spec
- `nix_version::compare_versions` — exec-spec connection through deep cv_fuel
- `log_invariant_inductive::leader_append_preserves_lmp` — quantifier through Seq::push
- `log_invariant_inductive::leader_append_preserves_lmp_right` — symmetric
- `log_invariant_inductive::leader_invariant_preserves_lmp` — system-level, multi-log

#### Hash-model bridge (2 items)
- `nix_store_path::sha256_deterministic` — uninterpreted SHA-256 determinism axiom
- `nix_store_path::sha256_collision_resistant` — uninterpreted SHA-256 collision-resistance axiom

### Closeable trust boundaries (38 items)
All remaining closeable trusted sites are in the Shamir secret-sharing stack:
`shamir_field`, `shamir_poly`, `shamir_lagrange`, `shamir_split`,
`shamir_reconstruct`, and `shamir_threshold`.

These are tracked individually in `openspec/specs/verification-gaps/spec.md`.

## WAL Segment

r[wal-segment.is-contiguous]
is_contiguous(prev_seq, current_seq) MUST return true if and only if
current_seq == prev_seq + 1. WAL entries MUST have consecutive sequence
numbers with no gaps. prev_seq MUST be less than u64::MAX.

r[wal-segment.space-remaining]
space_remaining(written, capacity) MUST return capacity - written when
written <= capacity, 0 otherwise. When written == 0, remaining MUST
equal capacity. When written == capacity, remaining MUST be 0.

r[wal-segment.entry-fits]
entry_fits(written, entry_size, capacity) MUST return true if and only if
written + entry_size <= capacity (using saturating addition to avoid
overflow). A zero-size entry always fits when written <= capacity. An
entry larger than the full capacity MUST NOT fit.

r[wal-segment.should-rotate]
should_rotate(written, capacity) MUST return true when written >= capacity.
A full segment MUST trigger rotation. An empty segment (written == 0) with
nonzero capacity MUST NOT trigger rotation.

r[wal-segment.write-advance]
write_advance(written, entry_size) MUST return written + entry_size, or
None on overflow. The result MUST be > written when entry_size > 0. The
result MUST equal written when entry_size == 0.

r[wal-segment.truncate-safe]
is_truncate_safe(truncate_to, committed_seq, applied_seq) MUST return true
if and only if truncate_to <= committed_seq AND truncate_to <= applied_seq.
Truncation MUST NOT discard committed-but-unapplied entries nor unapplied
entries. Truncating to 0 MUST always be safe.

r[wal-segment.replay-count]
replay_count(checkpoint_seq, end_seq) MUST return end_seq - checkpoint_seq
when end_seq >= checkpoint_seq, 0 otherwise. This is the number of entries
to replay during crash recovery. Replay from the same point MUST yield 0.

r[wal-segment.checkpoint-valid]
is_checkpoint_valid(checkpoint_seq, committed_seq) MUST return true if and
only if checkpoint_seq <= committed_seq. A checkpoint MUST NOT advance past
the committed sequence number.

## Cross-Module: WAL Segment

r[properties.wal-advance-is-monotonic]
write_advance(written, entry_size) when defined MUST satisfy
monotonic::is_advance(written, result). WAL write position is monotonic.

r[properties.wal-replay-is-saturating-sub]
replay_count(checkpoint, end) MUST equal
arithmetic::saturating_sub_spec(end, checkpoint). Replay count is a
saturating subtraction.

r[properties.wal-truncate-uses-min]
is_truncate_safe(t, c, a) MUST equal (t <= ordering::min_spec(c, a)).
Truncation safety is a comparison against the minimum of committed and
applied sequence numbers.

r[properties.wal-contiguous-is-strict-advance]
If is_contiguous(prev, current), then monotonic::is_strict_advance(prev,
current) holds. Contiguous WAL entries form a strictly advancing sequence.

## B-tree Balance

r[btree-balance.fill-pct]
fill_pct(count, capacity) MUST return count * 100 / capacity (integer
percentage). fill_pct MUST be 0 when count is 0 and 100 when count
equals capacity. capacity MUST be nonzero.

r[btree-balance.is-underfull]
is_underfull(count, capacity) MUST return true when count < capacity / 2.
A node with fewer than half its capacity is underfull. capacity MUST be
>= 2.

r[btree-balance.is-overfull]
is_overfull(count, capacity) MUST return true when count > capacity. A
node exceeding its capacity MUST be split.

r[btree-balance.should-split]
should_split(count, capacity) MUST return true if and only if count >
capacity. Splitting is required when the node exceeds its maximum.

r[btree-balance.should-merge]
should_merge(count, sibling_count, capacity) MUST return true when
count + sibling_count <= capacity. Two underfull nodes that together fit
in one node MUST be merged. count + sibling_count MUST NOT overflow.

r[btree-balance.split-point]
split_point(count) MUST return count / 2. The split divides the node
into two halves. split_point MUST be >= 1 when count >= 2.

r[btree-balance.split-left-size]
split_left_size(count) MUST return split_point(count). The left node
takes the first half.

r[btree-balance.split-right-size]
split_right_size(count) MUST return count - split_point(count). The
right node takes the remainder. left_size + right_size MUST equal count.

r[btree-balance.min-keys]
min_keys(capacity) MUST return capacity / 2. This is the minimum key
count for a non-root internal node (the B-tree invariant). capacity
MUST be >= 2.

r[btree-balance.can-borrow]
can_borrow(sibling_count, capacity) MUST return true when sibling_count >
min_keys(capacity). The sibling has keys to spare for redistribution.

## Cross-Module: B-tree Balance

r[properties.btree-split-conserves]
split_left_size(n) + split_right_size(n) MUST equal n for all n. No keys
are lost or created during a split.

r[properties.btree-fill-bounded]
fill_pct(count, capacity) MUST be in [0, 100] for count <= capacity.
The fill percentage is always a valid percentage.

r[properties.btree-min-keys-is-underfull-threshold]
is_underfull(min_keys(cap), cap) MUST be false. A node at exactly the
minimum key count is NOT underfull.

## LSM Level

r[lsm-level.target-size]
level_target_size(level, base_size, size_ratio) MUST return
base_size * size_ratio^level, saturating at u64::MAX. Level 0 MUST
return base_size. size_ratio MUST be >= 2. base_size MUST be > 0.

r[lsm-level.needs-compaction]
needs_compaction(level_size, target_size) MUST return true when
level_size > target_size. A level exceeding its target MUST trigger
compaction. An empty level MUST NOT need compaction.

r[lsm-level.space-amp-pct]
space_amp_pct(total_size, live_size) MUST return total_size * 100 /
live_size when live_size > 0. This is the space amplification as a
percentage. 100 means no amplification (total == live). live_size
MUST be nonzero.

r[lsm-level.write-amp-per-level]
write_amp_per_level(size_ratio) MUST return size_ratio. Each level
compaction rewrites approximately size_ratio times the data.

r[lsm-level.total-write-amp]
total_write_amp(num_levels, size_ratio) MUST return num_levels *
size_ratio, saturating at u64::MAX. Total write amplification is
the sum across levels.

r[lsm-level.level-count]
level_count_for_size(total_size, base_size, size_ratio) MUST return
the minimum number of levels k such that base_size * size_ratio^k >=
total_size. Returns 0 when total_size <= base_size. base_size > 0
and size_ratio >= 2.

r[lsm-level.size-ratio-valid]
is_valid_size_ratio(ratio) MUST return true if and only if ratio >= 2.
A ratio below 2 degenerates the LSM structure.

## Cross-Module: LSM Level

r[properties.lsm-target-monotonic]
If level_a <= level_b, level_target_size(level_a, base, ratio) MUST be <=
level_target_size(level_b, base, ratio). Higher levels have equal or
larger targets.

r[properties.lsm-space-amp-ge-100]
space_amp_pct(total, live) MUST be >= 100 when total >= live. Space
amplification is always at least 100%.

r[properties.lsm-write-amp-is-checked-mul]
total_write_amp(levels, ratio) MUST agree with
arithmetic::checked_mul_spec(levels, ratio) in the non-overflow case.

## Cross-Module: Financial Accounting (Extended)

r[properties.escrow-fund-refund-identity]
Funding an escrow then refunding MUST return the buyer to the original
balance. fund-then-refund is the identity on buyer balance.

r[properties.fee-split-agrees]
split_with_fee(amount, rate_bps) MUST return the same (fee, net)
as calling compute_fee and compute_net individually.

r[properties.simple-entry-is-balanced]
A double-entry with equal positive debit and credit amounts MUST be a
simple entry: is_simple_entry(amount, amount) is true when amount > 0.

r[properties.ledger-transfer-needs-funds]
If can_transfer returns true, the source account MUST have balance >=
amount. Transfers are only possible with sufficient funds.

## Cross-Module: Raft Consensus Pipeline

r[properties.election-win-implies-quorum]
Winning an election (has_won_election) MUST imply the vote count
reaches quorum (is_quorum). Election victory requires majority.

r[properties.heartbeat-resets-follower-timer]
A heartbeat from the current leader in the current term MUST trigger
an election timer reset.

## Cross-Module: Lock ↔ Lease ↔ Expiration

r[properties.lock-expired-matches-general-expired]
is_lock_expired(deadline, now) MUST equal is_expired(deadline, now).
Lock expiration is a special case of general expiration.

r[properties.lock-deadline-saturates]
compute_lock_deadline(acquired_at, ttl_ms) MUST be >= acquired_at.
On overflow, the deadline saturates to u64::MAX (never goes backward).

## Cross-Module: Drain ↔ Node Lifecycle

r[properties.draining-blocks-operations]
can_start_op(true) MUST return false. Draining blocks all new
operations regardless of node lifecycle status.

r[properties.operational-allows-non-drain-ops]
An operational node (online or degraded) that is not draining MUST
allow new operations via can_start_op(false).

## Cross-Module: Retry ↔ Circuit Breaker

r[properties.exhausted-means-no-retry]
When attempts are exhausted (is_exhausted), should_retry MUST return
false even if the error is retryable.

r[properties.open-circuit-blocks-requests]
An open circuit breaker MUST block all requests via allow_request.

r[properties.closed-circuit-allows-requests]
A closed circuit breaker MUST allow all requests via allow_request.

## Cross-Module: CAS ↔ Fencing ↔ Distributed Lock

r[properties.fencing-token-monotonic]
compute_next_fencing_token(Some(prev)) MUST return a value strictly
greater than prev, ensuring monotonic token advancement.

r[properties.valid-write-excludes-stale]
A valid write (is_valid_write) MUST NOT be stale (is_stale). These
predicates are mutually exclusive.

## Cross-Module: Vector Clock ↔ CRDT

r[properties.vc-merge-dominates-both]
After merging two vector clocks, each input clock's entries MUST be
<= the corresponding entry in the merged result.

r[properties.vc-merge-equals-gcounter-merge]
Vector clock merge and G-Counter merge MUST produce identical results
(both are pointwise max).

## Cross-Module: Noop Commit ↔ Log

r[properties.noop-roundtrip]
A noop entry constructed via make_noop_entry MUST be recognized as a
noop by is_noop.

r[properties.noop-commit-index-is-new-tail]
The noop commit index for a log of length n MUST be n+1 (the 1-based
position where the noop lands after append).

## Cross-Module: Snapshot Transfer ↔ Chunk

r[properties.first-chunk-zero-offset]
Offset zero MUST be identified as the first chunk.

r[properties.single-chunk-transfer]
A transfer where chunk_size == total MUST have offset=0 be both the
first and last chunk.

## Cross-Module: Concurrency Wave 5

r[properties.mpmc-occupancy-matches-mpsc]
bounded_mpmc occupancy (tail - head) MUST equal mpsc_bounds queue_occupancy
(total_sent - total_received) when given equivalent inputs.

r[properties.work-stealing-occupancy-matches-mpmc]
work_stealing deque_size (bottom - top) MUST equal bounded_mpmc occupancy
(tail - head) when given equivalent inputs.

r[properties.spin-completed-implies-cas-no-retry]
When spin_backoff is_completed, a CAS loop that respects the backoff MUST
stop retrying (attempts exhausted).

r[properties.seqlock-write-needs-release]
A seqlock write protocol requires at least Release memory ordering for
the store that ends the write.

r[properties.seqlock-read-needs-acquire]
A seqlock read protocol requires at least Acquire memory ordering for
the load that begins the read.

r[properties.rcu-grace-matches-epoch-advance]
For epochs 0 and 1, RCU grace period advance MUST produce the same result
as epoch_reclaim next_epoch (both return epoch + 1). They diverge at
epoch 2 where epoch_reclaim wraps to 0.

r[properties.ref-count-zero-implies-reclaimable]
When ref_count should_deallocate (strong_count == 0) and the address is
not protected by any hazard pointer, the address MUST be reclaimable.

r[properties.lock-ordering-prevents-ticket-deadlock]
When two threads acquire ticket locks in lock_ordering order, the
would_deadlock predicate MUST be false for the second acquisition.

r[properties.barrier-all-arrived-is-quorum]
When all threads arrive at a barrier (arrived == expected), the number
of arrivals MUST satisfy the quorum majority threshold.

r[properties.futex-notify-enables-consume]
After futex notify, the notified state MUST allow consume_notification
to return to idle.

r[properties.mpsc-drained-implies-mpmc-empty]
An MPSC channel that is drained (closed + empty) MUST also register as
empty in the bounded_mpmc model.

r[properties.cas-idempotent-needs-no-aba]
An idempotent CAS update (desired == current) always succeeds, making
ABA generation tracking unnecessary.

r[properties.ticket-lock-release-reduces-waiters]
Releasing a ticket lock MUST reduce the waiter count by exactly one.

## Paxos

r[paxos.ballot-gt]
ballot_gt MUST return true if and only if ballot a is strictly greater
than ballot b in lexicographic (round, proposer_id) order. The relation
MUST be irreflexive, transitive, and asymmetric. Trichotomy MUST hold:
exactly one of gt(a,b), eq(a,b), gt(b,a) is true for any pair.

r[paxos.ballot-eq]
ballot_eq MUST return true if and only if both round and proposer_id match.

r[paxos.ballot-ge]
ballot_ge MUST return true if and only if a is greater than or equal to b.
The relation MUST be reflexive, transitive, and antisymmetric.

r[paxos.can-promise]
can_promise MUST return true if and only if the ballot is strictly greater
than the acceptor's current promised ballot. After promising ballot b,
the acceptor MUST reject any ballot strictly less than b.

r[paxos.promise-next]
promise_next MUST update the promised ballot to the new ballot and
preserve the accepted value. The new promised ballot MUST be strictly
greater than the previous one.

r[paxos.can-accept]
can_accept MUST return true if and only if the ballot is at least as
high as the acceptor's current promised ballot. Acceptance of the same
ballot MUST be idempotent. Accepting a ballot MUST not block any higher
ballot from being accepted later.

r[paxos.accept-next]
accept_next MUST record the ballot and value, setting has_accepted to
true. Both the promised and accepted ballots MUST equal the acceptance
ballot. The accepted value MUST equal the provided value.

r[paxos.pick-value]
pick_value MUST return the accepted value when one exists, and the
proposer's own value otherwise.

r[paxos.higher-accepted]
higher_accepted MUST return the value associated with the higher
accepted ballot when comparing two promise responses. When neither
has accepted, it MUST return own_value. When exactly one has accepted,
it MUST return that value.

## Lease Mutex

r[lease-mutex.is-available]
is_available MUST return true if and only if the current time is at or
past the lock's lease expiry.

r[lease-mutex.can-acquire]
can_acquire MUST return true only when the lock is available, the TTL
is positive, the new expiry will not overflow, and the fence token can
be incremented.

r[lease-mutex.try-acquire]
try_acquire MUST succeed only when can_acquire holds. On success, it
MUST set the holder, compute expiry as now + ttl, and increment the
fence token by exactly one.

r[lease-mutex.is-owner]
is_owner MUST return true only when the requester matches the holder
AND the lease has not expired (now < expiry).

r[lease-mutex.write-permitted]
write_permitted MUST return true only when the requester is the current
owner AND the request's fence token is at least as high as the lock's.

r[lease-mutex.release]
release MUST set the expiry to now, immediately terminating the lease.
The fence token MUST be preserved on release. After release, the lock
MUST be available and the former holder MUST NOT be the owner.

r[lease-mutex.safety]
If is_owner returns true for node A at time t, then is_owner MUST
return false for any different node B at the same time t (mutual
exclusion). Successive leases MUST NOT overlap: if acquisition happens
at or after the old expiry, no point in time falls in both validity
windows. A previous holder's writes MUST be rejected after a new
acquisition due to the incremented fence token.

## Two-Phase Locking

r[two-phase-lock.valid-phase]
is_valid_phase MUST return true only for GROWING and SHRINKING.

r[two-phase-lock.can-acquire]
can_acquire MUST return true only in the growing phase.

r[two-phase-lock.after-acquire]
after_acquire MUST increment the held count by one and keep the phase
as GROWING. The phase MUST remain GROWING after any acquisition.

r[two-phase-lock.can-release]
can_release MUST return true only when held count is greater than zero.

r[two-phase-lock.after-release]
after_release MUST decrement the held count by one and set the phase
to SHRINKING.

r[two-phase-lock.phase-monotonic]
Once the phase transitions to SHRINKING, it MUST never return to
GROWING. Acquiring in the SHRINKING phase MUST be detected as a
violation and rejected.

r[two-phase-lock.is-complete]
is_complete MUST return true if and only if held count is zero and the
phase is SHRINKING. After acquiring, the transaction MUST NOT be
complete. Releasing the last lock MUST make the transaction complete.

r[two-phase-lock.would-violate]
would_violate MUST return true if and only if the operation is an
acquire in the shrinking phase. This is the only 2PL violation.

r[two-phase-lock.transition]
transition MUST apply one lock operation and return the new state, or
None if the operation is invalid. Valid transitions MUST preserve phase
validity. A balanced lifecycle (n acquires followed by n releases)
MUST end in the complete state.

## Serialization Injectivity

r[serialize-inj.encode-u64]
encode_u64 MUST produce an 8-byte little-endian encoding of a u64 value.
The encoding MUST be injective: equal byte sequences imply equal values.

r[serialize-inj.decode-u64]
decode_u64 MUST recover the original u64 from an 8-byte little-endian
encoding. decode(encode(v)) MUST equal v for all v.

r[serialize-inj.encode-u64-length]
encode_u64 output MUST be exactly 8 bytes for all inputs.

r[serialize-inj.encode-tagged]
encode_tagged MUST prepend a single tag byte to the payload. Different
tags MUST produce different encodings regardless of payload content.

r[serialize-inj.encode-tagged-injective]
Same tag with equal encodings MUST imply equal payloads.

r[serialize-inj.tag-disjoint]
Different tag bytes MUST produce different encodings for all payload
combinations.

r[serialize-inj.encode-len-prefixed]
encode_len_prefixed MUST prepend an 8-byte little-endian length header
to the data. The encoding MUST be injective.

r[serialize-inj.encode-pair]
encode_pair MUST concatenate two fixed-width u64 encodings into exactly
16 bytes. The encoding MUST be injective: equal byte sequences imply
equal field values.

r[serialize-inj.encode-quad]
encode_quad MUST concatenate four fixed-width u64 encodings into exactly
32 bytes. The encoding MUST be injective: equal byte sequences imply all
four fields are pairwise equal.

r[serialize-inj.chain-hash-compose]
The composition of encode_quad injectivity with chain hash collision
resistance MUST prove that equal chain hashes imply equal (prev, index,
term, data_id) fields.

r[serialize-inj.dual-hash-compose]
The composition of encode_pair injectivity with dual hash collision
resistance MUST prove that equal dual hashes imply equal (a, b) fields.

r[properties.chain-hash-implies-field-equality]
Chain hash equality MUST imply all four fields (prev, index, term, data_id)
are pairwise equal. This composes collision resistance with encoding
injectivity.

r[properties.dual-hash-implies-field-equality]
Dual hash equality MUST imply both fields (a, b) are equal. This composes
collision resistance with encoding injectivity.

r[properties.quad-encoding-matches-chain-layout]
Quad encoding MUST produce exactly 32 bytes for all inputs, matching the
chain hash field layout.

## Typed State

r[typed-state.stamp]
stamp MUST wrap raw data as Stamped<Unverified, T> preserving the
inner value. stamp(v).inner() MUST equal v.

r[typed-state.verify]
verify_with MUST transition from Unverified to Verified if and only if
the predicate returns true. It MUST return None when the predicate fails.

r[typed-state.unwrap]
unwrap_verified MUST extract the inner value from Stamped<Verified, T>.
It MUST only accept verified data.

r[typed-state.roundtrip]
stamp then verify_with(true predicate) then unwrap_verified MUST return
the original value.

r[typed-state.predicate-holds]
If verify_with returns Some, the predicate MUST have returned true for
the inner value.

r[typed-state.inner]
inner MUST give read access to the wrapped value regardless of state.

r[properties.snapshot-typed-verify]
verify_snapshot composing typed_state with snapshot_integrity MUST
return Some if and only if verify_integrity holds for the inner data.

## Job Retry

r[job-retry.is-retryable]
is_retryable_error MUST return true for error codes in [100, 200)
(transient failures). Error codes in [0, 100) are permanent failures
(non-retryable). Error codes >= 200 are unknown (non-retryable).

r[job-retry.attempts-remaining]
attempts_remaining MUST return max_attempts - attempts when attempts
< max_attempts, and 0 otherwise. The result MUST never be negative.

r[job-retry.cooldown-elapsed]
is_cooldown_elapsed MUST return true when now >= last_attempt_time +
cooldown_ms, using saturating addition to prevent overflow.

r[job-retry.should-retry]
should_retry MUST return true only when the error is retryable AND
attempts < max_attempts AND cooldown has elapsed. All three conditions
MUST hold.

r[job-retry.next-attempt]
next_attempt MUST return min(attempts + 1, max_attempts). The attempt
counter MUST never exceed max_attempts.

## Job Timeout

r[job-timeout.compute-deadline]
compute_deadline MUST return start_time + timeout_ms using saturating
addition. If the sum overflows u64, MUST return u64::MAX.

r[job-timeout.is-timed-out]
is_timed_out MUST return true when now >= deadline. Once timed out,
MUST stay timed out for all future time values (monotonic).

r[job-timeout.time-remaining]
time_remaining MUST return deadline - now when now < deadline, and 0
when now >= deadline. The result MUST never be negative.

r[job-timeout.elapsed-time]
elapsed_time MUST return now - start_time when now >= start_time,
and 0 otherwise.

r[job-timeout.extend-deadline]
extend_deadline MUST return min(deadline + grace, deadline +
max_extension) using saturating addition. The result MUST be >=
deadline (extensions only push forward). The result MUST NOT exceed
deadline + max_extension.

## Job Result

r[job-result.exit-code-classification]
Exit code classification MUST be mutually exclusive and exhaustive:
is_success_code (== 0), is_failure_code (1-127), is_error_code (>= 128).
Exactly one MUST return true for any u8 input.

r[job-result.output-hash-matches]
output_hash_matches MUST return true if and only if the computed hash
equals the expected hash.

r[job-result.output-size-valid]
output_size_valid MUST return true if and only if
min_size <= actual_size <= max_size.

r[job-result.validate-result]
validate_result MUST check exit code, then output hash, then output
size, returning the first failing criterion: 0=all pass, 1=bad exit,
2=bad hash, 3=bad size. MUST return 0 if and only if all criteria pass.

## Durable Execution

r[durable-execution.history-valid]
history_valid MUST return true if and only if durable execution events
form one contiguous append-only history: valid event kinds only,
sequence numbers starting at 0 and increasing by 1, and no event after a
durable terminal outcome.

r[durable-execution.prefix]
durable_prefix_len MUST return the largest prefix length with contiguous
sequence numbers and no post-terminal append. A gap, duplicate sequence
number, invalid event kind, or event after a terminal outcome MUST stop
the prefix at the first invalid position.

r[durable-execution.checkpoint-advance]
advance_checkpoint MUST never move backward. For a valid checkpoint
(old_checkpoint <= durable_prefix_len), it MUST advance to the durable
prefix. If the caller gives an already-invalid checkpoint beyond the
prefix, advance_checkpoint MAY leave it unchanged.

r[durable-execution.checkpoint-prefix]
A valid checkpoint MUST NOT claim durability past the contiguous durable
prefix. Checkpoint advancement MUST stop at the first gap and MUST NOT
skip missing prior steps.

r[durable-execution.replayable-event]
replayable_event_at MUST return Some(event) if and only if the index is
strictly within the durable prefix. Reads at or beyond the durable prefix
MUST return None.

r[durable-execution.effect-dedup]
should_execute_effect(seq_num, effect_id) MUST return false when the
durable prefix already contains a matching effect-completion marker for
that same (seq_num, effect_id) pair. Different effect ids MUST remain
independent.

r[durable-execution.resume-step]
resume_step MUST return Some(step) for the first step not yet covered by
the durable checkpoint / contiguous durable prefix. If durable history
already fixes a terminal outcome, resume_step MUST return None.

r[durable-execution.resume-bounds]
When resume_step returns Some(step), step MUST be at least the current
valid checkpoint and at most the contiguous durable prefix. Recovery MUST
NOT rewind below a valid checkpoint and MUST NOT skip over the first
missing durable step.

r[durable-execution.terminal]
is_terminal_execution MUST return true if and only if the valid durable
prefix ends in a terminal event (completed, failed, or cancelled).
Appending any later non-terminal event MUST be rejected by the append
rule.

r[durable-execution.terminal-recovery]
recovered_status MUST return the durable terminal outcome when one
exists, and None otherwise. Recovery of a terminal history MUST replay
that status rather than reopening the run.

## Cross-Module: Job Execution

r[properties.retry-non-retryable-never-retries]
should_retry with a non-retryable error code MUST always return false,
regardless of attempt count or cooldown.

r[properties.timeout-triggers-failure]
is_timed_out returning true for a RUNNING job means the job should
transition to FAILED (job_state::is_valid_transition(RUNNING, FAILED)).

r[properties.result-success-completes]
validate_result returning VALID (0) for a RUNNING job means the job
should transition to COMPLETED.

r[properties.result-failure-fails]
validate_result returning non-zero for a RUNNING job means the job
should transition to FAILED.

r[properties.retryable-disjoint-from-success]
Exit code 0 (success) MUST NOT be in the retryable error range [100,200).
Success and retryable errors are disjoint categories.

r[properties.durable-effect-uses-idempotency]
If durable replay suppresses an effect because a matching completion
marker already exists, then job_idempotency duplicate-prevention intuition
must agree: a consumed token for that same effect id would also reject a
fresh execution attempt.

r[properties.durable-history-seq-is-strict-advance]
Adjacent sequence numbers in a valid durable history MUST satisfy
monotonic::is_strict_advance after integer widening. Durable append-only
histories strictly advance their sequence numbers.

r[properties.durable-history-prefix-like-journal]
An allowed durable-history append MUST preserve the same strictly
increasing sequence-number shape used by journal-style append-only logs.
Durable history prefix growth is journal-like.

## Consensus Path Composition

r[consensus-path.append-preserves-invariant]
Appending an entry via leader_append_entry_spec MUST satisfy the
leader_log_invariant (new log is a prefix extension of the old).

r[consensus-path.appended-entry-committable]
An entry appended at the current term MUST create a committable entry:
can_commit_at_spec holds for the new index with the leader's term.

r[consensus-path.follower-commit-monotonic-bounded]
follower_update_commit_spec MUST return a value >= current_commit
(monotonic) and <= leader_commit (bounded by leader).

r[consensus-path.applied-bounded-by-commit]
advance_last_applied_spec MUST return a value <= commit_index and
>= last_applied. No entry is applied before it is committed.

r[consensus-path.write-path-end-to-end]
The write path composes: append preserves invariant, the new entry is
committable, and applied entries stay within commit bounds.

r[consensus-path.lease-subsumes-read-index]
lease_read_safe_spec returning true MUST imply read_safe_at_spec also
returns true. Lease reads are strictly stronger than ReadIndex.

r[consensus-path.committed-read-consistent]
If two servers have matching log prefixes through the commit point and
a read is safe, both servers agree on every entry below read_index.

r[consensus-path.read-path-end-to-end]
Authority confirmation + matching prefix through commit MUST imply
every entry below read_index agrees on both servers.

r[consensus-path.write-then-read]
A committed write at index N is visible to any subsequent read with
read_index > N, provided logs match through the commit point.

## B-tree Insert

r[btree-insert.is-sorted]
is_sorted MUST return true iff keys are strictly increasing. Empty and
singleton sequences MUST always be sorted.

r[btree-insert.find-position]
find_insert_position MUST return the index where a key should be inserted
to maintain sortedness. The position MUST be in [0, keys.len()]. All keys
before it MUST be <= key.

r[btree-insert.insert-at]
insert_at MUST insert a key at the given position, shifting elements right.
Length MUST increase by 1. Inserting at the correct position into a sorted
sequence MUST produce a sorted sequence (when key not already present).

r[btree-insert.split-node]
split_node MUST split a sorted sequence at a midpoint into two sorted
subsequences. left.len() + right.len() MUST equal the original length.
All keys in left MUST be < all keys in right. Split sizes MUST match
btree_balance::split_left/right_size.

## B-tree Delete

r[btree-delete.contains-key]
contains_key MUST return true iff the key exists in the sequence.

r[btree-delete.find-key-position]
find_key_position MUST return the index of the key, or keys.len() if
not found. The result MUST be in [0, keys.len()].

r[btree-delete.delete-at]
delete_at MUST remove the element at the given position, shifting left.
Length MUST decrease by 1. Deleting from a sorted sequence MUST produce
a sorted sequence.

r[btree-delete.merge-nodes]
merge_nodes MUST concatenate two sorted sequences where all keys in left
< all keys in right. The result MUST be sorted and have length
left.len() + right.len().

r[btree-delete.borrow-left]
borrow_left MUST take the last key from the left sibling and prepend it
to the node. Combined count MUST be unchanged.

r[btree-delete.borrow-right]
borrow_right MUST take the first key from the right sibling and append it
to the node. Combined count MUST be unchanged.
A committed write at index N is visible to any subsequent read with
read_index > N, provided logs match through the commit point.

## Concurrent Data Structure Composition

r[concurrent-composition.rwlock-write-excludes-readers]
In write mode with mutual exclusion, active readers MUST be zero — the
writer has exclusive access to the protected data structure.

r[concurrent-composition.rwlock-read-excludes-writer]
In read mode with mutual exclusion, no writer MUST hold the lock.

r[concurrent-composition.rwlock-write-release-to-free]
Releasing a write lock MUST transition to FREE mode, enabling subsequent
acquisitions by readers or writers.

r[concurrent-composition.rwlock-writer-token-is-strict-advance]
The writer fencing token MUST strictly advance on each write acquisition,
composing with monotonic::is_strict_advance.

r[concurrent-composition.rwlock-writer-token-is-fencing-advance]
The rwlock writer token advance MUST agree with fencing::issue_token.

r[concurrent-composition.rwlock-last-reader-frees]
When the last reader releases, mode MUST return to FREE and reader count
MUST drop to zero.

r[concurrent-composition.seqlock-write-cycle-strict-advance]
A complete seqlock write cycle MUST produce a strict advance in the
sequence counter.

r[concurrent-composition.seqlock-version-strict-advance]
Each write cycle MUST strictly advance the writer version.

r[concurrent-composition.seqlock-valid-read-no-write]
A valid seqlock read MUST imply no write was in progress at the sample
time.

r[concurrent-composition.seqlock-write-invalidates-read]
A begin_write between a reader's before/after samples MUST invalidate
the read.

r[concurrent-composition.cas-pack-succeed-unpack-roundtrip]
Pack → CAS succeed → unpack MUST recover the desired value and generation.

r[concurrent-composition.cas-aba-detected-by-generation]
ABA scenario (same value, different generation) MUST be detected by CAS
because packed words differ.

r[concurrent-composition.cas-retry-progress]
Each failed CAS retry MUST decrease attempts_remaining, guaranteeing
termination.

r[concurrent-composition.cas-success-with-generation-bump]
A successful CAS with bumped generation MUST produce a packed word
distinguishable from the old one.

r[concurrent-composition.hp-protected-blocks-rcu-free]
A hazard-protected address MUST NOT be reclaimable regardless of RCU
grace period status.

r[concurrent-composition.all-quiescent-all-empty-everything-reclaimable]
When all threads are RCU-quiescent AND all hazard slots are empty,
every retired address MUST be reclaimable under both protocols.

r[concurrent-composition.rcu-advance-enables-reclamation]
Advancing the RCU grace period MUST be a strict advance in the counter.

r[concurrent-composition.hp-unprotect-enables-reclaim]
After unprotecting a hazard slot (sole protector), the address MUST
become reclaimable.

r[concurrent-composition.2pl-growing-respects-ordering]
During the growing phase of 2PL, if lock ordering is respected, the
acquisition MUST be valid under both protocols.

r[concurrent-composition.2pl-shrinking-no-acquire]
In the shrinking phase, no acquisition MUST be permitted regardless of
lock ordering.

r[concurrent-composition.2pl-phase-irreversible]
The transition from growing to shrinking MUST be one-way.

r[concurrent-composition.ticket-take-is-strict-advance]
Taking a ticket MUST strictly advance the ticket counter.

r[concurrent-composition.ticket-release-is-strict-advance]
Releasing (advancing the serving counter) MUST be a strict advance.

r[concurrent-composition.ticket-fifo-order]
An earlier ticket holder MUST be served before a later one (FIFO).

r[concurrent-composition.mpmc-occupancy-bounded]
MPMC queue occupancy MUST NOT exceed capacity.

r[concurrent-composition.mpmc-full-blocks-enqueue]
A full MPMC queue MUST prevent enqueue.

r[concurrent-composition.mpmc-empty-blocks-dequeue]
An empty MPMC queue MUST prevent dequeue.

r[concurrent-composition.mpmc-enqueue-dequeue-occupancy-roundtrip]
Enqueue then dequeue MUST preserve occupancy (net effect zero).

r[concurrent-composition.mpmc-remaining-plus-occupancy]
Remaining capacity plus occupancy MUST equal total capacity.

## Counting Semaphore

r[semaphore.can-acquire]
can_acquire MUST return true if and only if `permits <= available`.
Zero permits MUST always be acquirable.

r[semaphore.after-acquire]
after_acquire MUST return `available - permits`. Acquiring a positive
number of permits MUST strictly decrease the available count.
Acquiring zero permits MUST be identity. Chained acquisitions
MUST equal a single combined acquisition of the sum.

r[semaphore.after-release]
after_release MUST return `available + permits`. Releasing a positive
number of permits MUST strictly increase the available count.
Releasing zero permits MUST be identity. The result MUST NOT exceed
capacity when the release is valid. Chained releases MUST equal
a single combined release of the sum.

r[semaphore.is-full]
is_full MUST return true if and only if `available == capacity`.
A full semaphore MUST have zero acquired permits.

r[semaphore.is-empty]
is_empty MUST return true if and only if `available == 0`.
An empty semaphore MUST block any positive acquisition.

r[semaphore.acquired-count]
acquired_count MUST return `capacity - available`. Acquiring k permits
MUST increase the acquired count by k. Releasing k permits MUST
decrease the acquired count by k.

## Binary Heap Index Arithmetic

r[heap-index.parent]
parent_index(i) MUST return (i - 1) / 2 for nodes with i > 0. The
parent index MUST always be strictly less than i.

r[heap-index.left-child]
left_child(i) MUST return 2 * i + 1. The left child index MUST always
be strictly greater than i.

r[heap-index.right-child]
right_child(i) MUST return 2 * i + 2. The right child index MUST always
be strictly greater than i.

r[heap-index.is-leaf]
is_leaf(i, size) MUST return true when node i has no children in a heap
of the given size. A node is a leaf when its left child index would be
at or beyond the heap size.

r[heap-index.has-parent]
has_parent(i) MUST return true for all nodes except the root (i > 0).

r[heap-index.is-left-child]
is_left_child(i) MUST return true when i > 0 and i is odd (left children
occupy odd indices in a zero-based heap array).

r[heap-index.is-right-child]
is_right_child(i) MUST return true when i > 0 and i is even (right children
occupy even indices in a zero-based heap array).

## Resource Quota

r[quota.remaining]
quota_remaining(limit, used) MUST return limit - used when used <= limit,
and 0 when used exceeds the limit.

r[quota.allows]
quota_allows(limit, used, request) MUST return true if and only if the
sum of used and request does not exceed limit.

r[quota.consume]
quota_consume(used, amount) MUST return used + amount. The caller MUST
ensure the sum does not overflow.

r[quota.release]
quota_release(used, amount) MUST return used - amount when amount <= used,
and 0 when amount exceeds used (saturating subtraction).

r[quota.utilization]
quota_utilization(limit, used) MUST return the integer percentage of
quota consumed, clamped to [0, 100]. When limit is 0, MUST return 100
if used is positive, and 0 otherwise.

r[quota.is-exhausted]
quota_is_exhausted(limit, used) MUST return true if and only if
used >= limit.

## Allocation Token

r[alloc-token.new]
alloc_token_new(id) MUST return a token in the Free state with the
given block id.

r[alloc-token.allocate]
alloc_token_allocate(id, state) MUST transition from Free to Allocated.
The precondition MUST require state == Free. Double-allocate MUST be
prevented by the precondition.

r[alloc-token.release]
alloc_token_release(id, state) MUST transition from Allocated to Freed.
The precondition MUST require state == Allocated. Double-free MUST be
prevented by the precondition. Freed is a terminal state.

r[alloc-token.is-live]
is_live(state) MUST return true if and only if the state is Allocated.

r[alloc-token.valid-state]
is_valid_state(state) MUST return true for Free, Allocated, and Freed,
and false for all other values.

## Size Class

r[size-class.class-to-size]
class_to_size(c) MUST return 2^c — the actual allocation size for class c.
Class size MUST be positive and MUST double with each class increment.

r[size-class.size-to-class]
size_to_class(size) MUST return the minimum class index where the class
size is at least the requested size. MUST be monotonic: larger request
implies same or larger class. Roundtrip: size_to_class(class_to_size(c))
MUST equal c.

r[size-class.waste]
waste(requested, class) MUST return class_to_size(class) - requested.
Waste MUST be non-negative and strictly less than the requested size
for any size > 0.

r[size-class.class-count]
class_count(max_size) MUST return the number of distinct size classes
needed. MUST be at least 1 for any positive max_size.

## Free List

r[free-list.all-distinct]
A well-formed free list MUST have all distinct elements (no duplicates).
This is the acyclicity invariant for linked free lists.

r[free-list.all-within]
All entries in a free list MUST fall within a bounding region.

r[free-list.contains]
free_list_contains(list, block) MUST return true if and only if block
is in the list.

r[free-list.len]
free_list_len(list) MUST return the number of elements in the list.

r[free-list.push]
free_list_push(list, block) MUST prepend block to the list. Push then
pop MUST return the same block (LIFO). Push MUST increment length by 1.
Push of an already-present block MUST be prevented by the precondition.

r[free-list.pop]
free_list_pop(list) MUST remove and return the first block. Pop MUST
decrement length by 1.

r[free-list.remove]
free_list_remove(list, block) MUST remove the block from the list.

## Split Coalesce

r[split-coalesce.split-block]
split_block(addr, order) MUST split a block into two buddies at order-1.
Split then coalesce MUST be identity. The two halves MUST be disjoint
and their sizes MUST sum to the parent block size.

r[split-coalesce.coalesce-blocks]
coalesce_blocks(left, right, order) MUST merge two buddy blocks into
a parent. Coalescing MUST be symmetric.

r[split-coalesce.split-tree-total]
Recursive split from order k to target order j MUST produce disjoint
blocks all at the target order, whose sizes sum to 2^k.

r[split-coalesce.blocks-disjoint]
blocks_disjoint(blocks) MUST return true when no two blocks overlap.

r[split-coalesce.blocks-cover]
blocks_cover(blocks, addr, order) MUST return true when the blocks'
total size equals the parent block size 2^order.

## Allocation Bitmap

r[alloc-bitmap.handle-pack]
handle_pack(slot, gen) MUST pack slot index (low 32) and generation
(high 32) into a u64. Pack/unpack MUST roundtrip. Different (slot, gen)
pairs MUST produce different handles (injectivity).

r[alloc-bitmap.handle-unpack-slot]
handle_unpack_slot(handle) MUST extract the slot index (low 32 bits).

r[alloc-bitmap.handle-unpack-gen]
handle_unpack_gen(handle) MUST extract the generation (high 32 bits).

r[alloc-bitmap.is-allocated]
bitmap_is_allocated(map, slot) MUST return true iff the slot is allocated.

r[alloc-bitmap.generation]
bitmap_generation(gens, slot) MUST return the current generation for slot.

r[alloc-bitmap.utilization]
bitmap_utilization(map) MUST return (used, total) where used + free == total.

r[alloc-bitmap.alloc]
bitmap_alloc(map, gens) MUST find the first free slot, mark it allocated,
increment the generation, and return a handle. MUST return None if full.

r[alloc-bitmap.free]
bitmap_free(map, gens, handle) MUST free a slot by handle. MUST reject
double-free (slot not allocated) and stale handles (generation mismatch).

## Arena

r[arena.alloc]
arena_alloc(mark, size) MUST bump-allocate: return the current mark as
pointer and advance mark by size. Successive allocs MUST produce disjoint
regions. Mark MUST advance monotonically.

r[arena.reset]
arena_reset(base, generation) MUST reset mark to base and increment
generation. After reset, remaining MUST equal capacity. Reset MUST
invalidate all pointers from prior generations. Generation MUST
monotonically increase.

r[arena.remaining]
arena_remaining(mark, base, capacity) MUST return base + capacity - mark.

r[arena.generation]
arena_generation(generation) MUST return the current generation.

r[arena.is-valid]
arena_is_valid(ptr_gen, current_gen) MUST return true iff the generations
match. Pointer from generation g MUST be invalid in generation g+1.

r[arena.fits]
arena_fits(mark, size, base, capacity) MUST return true iff size fits
in remaining space.

## Pool

r[pool.alloc]
pool_alloc(map, gens, base, obj_size) MUST allocate the first free slot,
compute its address via slab arithmetic, and return a generational handle.
MUST return None if full.

r[pool.free]
pool_free(map, gens, handle) MUST free a slot by generational handle.
MUST reject double-free and stale handles.

r[pool.is-full]
pool_is_full(map) MUST return true iff every slot is allocated.

r[pool.count]
pool_count(map) MUST return (allocated, free) where allocated + free
equals the total number of slots.

## Segregated Free

r[segregated-free.class-for]
seg_class_for(size, num_classes) MUST return the class index for a
requested size. MUST be deterministic and bounded by num_classes.

r[segregated-free.total-free]
seg_total_free(lists) MUST return the total number of free blocks
across all class lists. MUST be non-negative.

r[segregated-free.cross-class-unique]
No block MUST appear in two different class lists simultaneously.

r[segregated-free.alloc]
seg_alloc(lists, size, num_classes) MUST allocate from the appropriate
class's free list. MUST return None if that class is empty.

r[segregated-free.free]
seg_free(lists, block, class) MUST return the block to its class's
free list. Free to class c MUST NOT modify other classes' lists.

## Coalesce Policy

r[coalesce-policy.immediate]
should_coalesce_immediate(buddy_is_free) MUST return true iff buddy is
free. Immediate MUST subsume deferred (any deferred trigger also triggers
immediate).

r[coalesce-policy.deferred]
should_coalesce_deferred(buddy_is_free, free_count, threshold) MUST
return true iff buddy is free AND free_count >= threshold. With threshold
0 MUST be equivalent to immediate.

r[coalesce-policy.queue-add]
deferred_queue_add(queue, block) MUST append the block to the queue.
Queue length MUST increment by 1.

r[coalesce-policy.queue-len]
deferred_queue_len(queue) MUST return the number of pending blocks.

r[coalesce-policy.preserves-total]
Coalescing two blocks of size s into one block of size 2s MUST preserve
the total byte count.

## Fragmentation

r[fragmentation.external]
external_frag(free_total, largest_free_block) MUST return 0 when
largest_free == free_total (one contiguous chunk) and 100 when
largest_free == 0.

r[fragmentation.internal]
internal_frag(requested, allocated) MUST return allocated - requested.
MUST be zero for exact-fit allocations.

r[fragmentation.can-satisfy]
can_satisfy(largest_free, requested) MUST return true iff largest_free
>= requested. can_satisfy MUST imply free_total >= requested.

r[fragmentation.buddy-bound]
frag_bound_buddy(requested) MUST return the worst-case buddy internal
fragmentation. MUST be strictly less than requested for all sizes > 0.

r[fragmentation.slab-bound]
frag_bound_slab(obj_size, slot_size) MUST return slot_size - obj_size.
MUST be zero when obj_size == slot_size.

## TCP Connection State Machine

r[tcp-state.is-valid]
is_valid_state MUST return true for exactly the 11 TCP states: CLOSED,
LISTEN, SYN_SENT, SYN_RECEIVED, ESTABLISHED, FIN_WAIT_1, FIN_WAIT_2,
CLOSE_WAIT, CLOSING, LAST_ACK, TIME_WAIT.

r[tcp-state.can-transition]
can_transition MUST return true for exactly the valid transitions defined
in RFC 793. Self-transitions MUST be rejected. Both endpoints of a valid
transition MUST be valid states.

r[tcp-state.is-data-state]
is_data_state MUST return true for ESTABLISHED and CLOSE_WAIT only.
Data transfer is possible only in these states.

r[tcp-state.is-shutdown]
is_shutdown MUST return true for states in the connection teardown path:
FIN_WAIT_1, FIN_WAIT_2, CLOSING, TIME_WAIT, LAST_ACK, CLOSE_WAIT.

r[tcp-state.is-connected]
is_connected MUST return true for states where a connection exists:
SYN_RECEIVED, ESTABLISHED, FIN_WAIT_1, FIN_WAIT_2, CLOSE_WAIT, CLOSING,
LAST_ACK, TIME_WAIT. CLOSED, LISTEN, and SYN_SENT are not connected.

r[tcp-state.closed-terminal]
CLOSED MUST have no outgoing application-driven transitions. The only
transitions from CLOSED are via passive_open (→LISTEN) or active_open
(→SYN_SENT), which are initiation functions, not state machine edges.

r[tcp-state.established-reachable]
A path from CLOSED to ESTABLISHED MUST exist through the normal
three-way handshake (CLOSED→SYN_SENT→ESTABLISHED or
CLOSED→LISTEN→SYN_RECEIVED→ESTABLISHED).


## Transaction Isolation

r[txn-isolation.is-valid]
is_valid_level MUST return true for levels 0–3 (READ_UNCOMMITTED through SERIALIZABLE).

r[txn-isolation.allows-dirty-read]
allows_dirty_read MUST return true only for READ_UNCOMMITTED.

r[txn-isolation.allows-non-repeatable-read]
allows_non_repeatable_read MUST return true for levels below REPEATABLE_READ.

r[txn-isolation.allows-phantom]
allows_phantom MUST return true for levels below SERIALIZABLE.

r[txn-isolation.prevents-dirty-read]
prevents_dirty_read MUST be monotonic: higher levels prevent at least as much.

r[txn-isolation.prevents-non-repeatable-read]
prevents_non_repeatable_read MUST be monotonic.

r[txn-isolation.prevents-phantom]
prevents_phantom MUST be monotonic.

r[txn-isolation.anomaly-count]
Higher isolation levels MUST have fewer or equal allowed anomaly classes.

r[txn-isolation.strengthen]
strengthen MUST return the stricter of two levels and be commutative and idempotent.

r[txn-isolation.weaken]
weaken MUST return the more permissive of two levels and be commutative and idempotent.

## MVCC

r[mvcc.is-visible]
A version is visible iff begin_ts <= snapshot_ts < end_ts.

r[mvcc.is-committed]
A version's creator is committed iff the creator txn is not in the active set.

r[mvcc.snapshot-visible]
A version is snapshot-visible iff it is both timestamp-visible and committed.

r[mvcc.version-superseded]
A version is superseded iff its end_ts is not TS_INFINITY.

r[mvcc.can-gc]
A version can be GC'd iff it is superseded and end_ts <= oldest_active.

r[mvcc.chain-latest]
chain_latest MUST return the first visible version in a chain, or chain.len() if none.

## Snapshot Isolation

r[snapshot-iso.has-ww-conflict]
has_ww_conflict MUST return true iff two write sets share any key. MUST be symmetric.

r[snapshot-iso.si-allows-commit]
SI allows commit when no concurrent committed transaction's write set conflicts.

r[snapshot-iso.write-set-size]
write_set_size MUST return the number of keys in the write set.

r[snapshot-iso.rw-overlap]
rw_overlap MUST return true iff read set and write set share any key.

## Query Cost

r[query-cost.seq-scan]
seq_scan_cost MUST equal num_pages * cost_per_page. MUST be monotonic in both params.

r[query-cost.index-scan]
index_scan_cost MUST be zero when selectivity is zero.

r[query-cost.selectivity]
Higher distinct values MUST produce lower selectivity.

## Index Selection

r[index-select.is-covering]
is_covering MUST return true iff every query column appears in the index.

r[index-select.best-index]
best_index MUST return a valid index within bounds.

r[index-select.column-count]
column_count MUST return the number of columns.

## Transaction Dependencies

r[txn-dep.is-valid-type]
is_valid_dep_type MUST accept WR(0), WW(1), and RW(2).

r[txn-dep.has-dependency]
has_dependency MUST return true iff the edge (from, to) exists.

r[txn-dep.has-self-dep]
has_self_dep MUST return true iff any edge has from == to.

r[txn-dep.has-two-cycle]
has_two_cycle MUST return true iff edges contain A→B and B→A with A≠B.

r[txn-dep.ts-ordered]
ts_ordered MUST return true iff all edges respect timestamp ordering.
Timestamp ordering MUST imply no self-loops and no 2-cycles.

## Zeroize

r[zeroize.scrub]
zeroize MUST write zero to every byte in the buffer. After zeroize,
all bytes MUST be zero. Length MUST be preserved.

r[zeroize.idempotent]
Zeroizing an already-zeroed buffer MUST produce the same result.
zeroize(zeroize(buf)) == zeroize(buf).

r[zeroize.detects-nonzero]
is_zeroed MUST return true if and only if every byte is zero.
is_zeroed on a zeroized buffer MUST return true.

## Constant-Time Select

r[ct-select.u64]
ct_select_u64 MUST return a when flag is true, b when flag is false.
The result MUST be equivalent to branching.

r[ct-select.u8]
ct_select_u8 MUST return a when flag is true, b when flag is false.

r[ct-select.flag-true]
When flag is true, ct_select MUST return the first argument.

r[ct-select.flag-false]
When flag is false, ct_select MUST return the second argument.

## Nonce

r[nonce.issue]
issue_nonce MUST return the current counter value as the nonce and
increment the counter by one. The counter MUST NOT wrap.

r[nonce.uniqueness]
Two nonces issued from different counter states MUST be distinct.
For any c1 != c2, issue_nonce(c1).nonce != issue_nonce(c2).nonce.

r[nonce.monotonic]
Successive nonces MUST be strictly increasing. The new counter
MUST be a strict advance over the old counter.

r[nonce.freshness]
is_fresh MUST return true iff the nonce is strictly greater than the
last-seen value. A freshly issued nonce checked against its issuing
counter MUST be fresh.

## Zircon Kernel Primitives

r[zircon-rights.attenuation]
A derived handle MUST NOT gain rights not present on the source handle.
Attenuation MUST be subset-preserving. Duplicate and replace MUST accept
same-rights or reduced-rights requests and reject expanded-rights requests.

r[zircon-rights.operation-gates]
Handle operations MUST require the matching handle right: duplicate,
transfer, wait, signal, and signal-peer each need their corresponding bit.

r[zircon-handle.identity]
Every live object MUST have a stable koid. Duplicate, replace, transfer,
and receive MUST preserve the referenced koid.

r[zircon-handle.ownership]
A handle MUST be in exactly one ownership state: bound, in-transit, or
closed. Send moves a bound handle to in-transit. Receive rebinds an
in-transit handle to a process. Close moves any handle to closed.

r[zircon-handle.lifetime]
An object MUST remain live while any bound or in-transit handle refers to
it. If no live handles remain, the object is no longer live.

r[zircon-wait.readiness]
A Zircon wait MUST be ready exactly when observed_signals & requested_mask
is non-zero.

r[zircon-wait.wait-set]
A multi-handle wait MUST report ready exactly for entries whose observed
and requested signal masks intersect.

r[zircon-event.user-signals]
Events MUST carry only user-signal bits. Setting and clearing user signals
MUST affect only those bits and MUST be idempotent.

r[zircon-event.peer-closed]
Eventpairs MUST derive PEER_CLOSED from peer handle liveness: when the
peer is not live, PEER_CLOSED is asserted; when the peer remains live,
it is not asserted from handle state alone.

r[zircon-channel.datagram]
Channels MUST behave as datagram queues: each write appends one whole
message and each read consumes one whole message. Empty queues are not
readable.

r[zircon-channel.transfer]
Sending handles through a channel MUST preserve koid and rights while
changing ownership from bound to in-transit. Receiving MUST preserve koid
and rights while rebinding the handle to the destination process.

r[zircon-channel.discard]
Closing a destination endpoint with unread queued handle-bearing messages
MUST discard those messages and close their transferred handles.

r[zircon-channel.signals]
A channel endpoint MUST assert READABLE when its receive queue is non-empty
and PEER_CLOSED when the peer endpoint is gone.

r[zircon-futex.wait-compare]
A futex wait MUST block only when observed == expected. Mismatched values
MUST leave the waiter count unchanged.

r[zircon-futex.wake-bound]
A futex wake MUST release at most the requested count and never more than
existing waiters. Waking an empty futex MUST be a no-op.

r[zircon-futex.requeue]
A futex requeue MUST wake up to the requested wake count, move up to the
requested requeue count, and conserve total blocked waiters except those
explicitly woken.

r[zircon-socket.stream]
Sockets MUST behave as paired byte streams: writes append bytes to the
peer receive buffer, reads consume bytes in order, short writes respect
free capacity, and short reads return available bytes.

r[zircon-socket.disposition]
Each socket endpoint MUST independently track write-disabled state.
Local disable MUST reject writes and assert WRITE_DISABLED. The peer MUST
observe PEER_WRITE_DISABLED. Existing buffered data MUST remain readable.

r[zircon-socket.thresholds]
Sockets MUST support read and write threshold signals. Threshold 0 disables
that signal. Read-threshold readiness depends on queued bytes; write-
threshold readiness depends on free space.

r[zircon-socket.peer-closed]
Closing all handles to one socket endpoint MUST assert PEER_CLOSED on the
opposite endpoint.

r[zircon-port.async-wait]
An async wait registration MUST bind (koid, mask, key, mode) to a port.
When observed signals intersect the registered mask, delivery MUST enqueue
an async packet with that key and observed mask.

r[zircon-port.one-shot]
A one-shot async wait registration MUST be consumed after first delivery.
A repeating registration MUST remain active after delivery.

r[zircon-port.user-queue]
A port MUST accept explicitly queued user packets and deliver them in FIFO
order relative to other user-queued packets.

r[zircon-timer.lifecycle]
Timers MUST support set, cancel, reset, and deadline passage. When an
armed timer reaches its deadline it becomes signaled. The signaled state
MUST persist until cancel or reset.

r[zircon-fifo.element-aligned]
FIFOs MUST transfer whole fixed-size elements only. Accepted writes are
bounded by free element slots. Reads return whole queued elements in FIFO
order.

r[zircon-fifo.peer-closed]
Closing all handles to one FIFO endpoint MUST assert PEER_CLOSED on the
opposite endpoint.

r[zircon-fifo.signals]
A FIFO endpoint MUST assert READABLE when its receive queue is non-empty
and WRITABLE when free element slots remain.

r[zircon-job.containment]
Jobs MUST form a tree with at most one parent per non-root job. Each
process MUST belong to exactly one owning job.

r[zircon-job.kill-propagation]
Killing a job MUST cover the whole subtree: every descendant job and every
process owned by jobs in that subtree is included in the kill set.

r[zircon-job.critical-process]
A critical process exiting with a non-zero code MUST kill its owning job.
A clean exit or non-critical process MUST NOT trigger job kill.

r[zircon-job.parent-lifetime]
A job MUST stay live when any running process exists in its subtree, even
if the parent job itself has no direct handles.

r[properties.zircon-rights-handle]
Handle duplication/refinement properties MUST connect Zircon rights
attenuation to handle-local koid and rights preservation.

r[properties.zircon-signal-refinement]
Zircon event, channel, socket, and FIFO signal computations MUST refine
bitmask facts expressible in the generic signal algebra.

r[properties.zircon-channel-lifetime]
Channel transfer MUST preserve the lifetime rule that an object remains
live while a bound or in-transit handle exists.

r[properties.zircon-port-wait]
Async port delivery MUST correspond to a satisfied Zircon wait-readiness
predicate for the registered mask.

r[properties.zircon-timer-deadline]
If deadline A is earlier than or equal to deadline B and time has passed B,
then time has also passed A. Timer signaling order MUST respect deadline
ordering.

r[properties.zircon-job-process-state]
A process covered by job-kill propagation MUST admit the abstract
Running -> Terminated process-state transition.

r[properties.zircon-futex-process-state]
Any waiter released by futex wake or requeue MUST admit the abstract
Blocked -> Ready process-state transition.

## BEAM Primitives

r[beam-process.lifecycle]
A BEAM process MUST move only forward through `alive -> exiting -> dead`.
Once dead, it MUST remain dead.

r[beam-process.isolation]
BEAM processes with distinct pid identities MUST have distinct logical heap
identities. Mutating one process heap MUST NOT mutate any other process
heap. Inter-process term transfer MUST occur by copying.

r[beam-mailbox.ordering]
A BEAM mailbox MUST append new messages at the tail. Messages from the same
sender MUST remain in send order.

r[beam-mailbox.selective-receive]
Selective receive MUST scan from the saved cursor and return the first
matching message. Unmatched messages MUST remain in mailbox order.

r[beam-mailbox.save-offset]
When selective receive finds a match, the save offset MUST reset to 0.
When no match exists, the save offset MUST advance to the mailbox end so
already-scanned non-matches are not re-scanned.

r[beam-send.async]
BEAM send MUST always succeed from the sender's point of view. Sending to a
live process MUST append to its mailbox; sending to a dead process MUST
silently drop the message.

r[beam-link.symmetry]
Process links MUST be symmetric. Creating a link MUST add both directions,
and unlinking MUST remove both directions.

r[beam-link.propagation]
An abnormal exit received through a live link MUST cause a non-trapping
alive peer to begin exiting. A normal exit MUST NOT cascade.

r[beam-link.trap-exit]
A process with `trap_exit` enabled MUST convert incoming exit signals into
`EXIT` mailbox messages, except `kill`, which MUST terminate the process as
`killed`.

r[beam-monitor.unidirectional]
A monitor MUST be unidirectional: monitor creation MUST produce only
watcher-owned monitor state, leave the target process state unchanged, and
route any DOWN observation only through the watcher-side result/mailbox
path.

r[beam-monitor.down]
An active monitor MUST deliver exactly one `DOWN` message when the target
dies, and then become inactive.

r[beam-monitor.late]
Creating a monitor for an already-dead process MUST immediately yield a
`DOWN` message with reason `noproc`.

r[beam-monitor.demonitor]
Demonitor MUST disable future DOWN delivery. Demonitor with flush MUST
remove any queued DOWN for the monitor reference.

r[beam-scheduler.preemption]
Reduction charging MUST saturate at zero remaining budget. Zero remaining
budget, or explicit yield, MUST trigger preemption.

r[beam-scheduler.priority]
Scheduler priority selection MUST prefer `max` over `high`, `high` over
`normal`, and `normal` over `low`, subject to starvation-prevention for
lower queues. Only alive processes with work are runnable.

r[beam-registry.uniqueness]
The BEAM registry MUST remain bijective: no two entries may share a name,
and no two entries may share a pid.

r[beam-registry.cleanup]
Cleaning up a dead pid MUST remove any registry entry for that pid so the
name can be registered again.

## Cross-Module: BEAM Primitives

r[properties.beam-death-links]
A process death propagated through BEAM links MUST respect link rules:
non-trapping peers begin exiting on abnormal signals, trapped peers receive
EXIT messages, and unlinked peers observe nothing.

r[properties.beam-death-monitor]
A monitored process death MUST produce exactly one DOWN message for each
active monitor reference.

r[properties.beam-death-mailbox]
Sending to a dead BEAM process MUST leave its mailbox unchanged.

r[properties.beam-death-registry]
Cleaning up a dead BEAM pid in the registry MUST remove that pid's entry so
a register-then-cleanup roundtrip restores the prior registry and the same
name can be registered again.

r[properties.beam-scheduler-state]
The scheduler MUST treat only alive processes with pending work as runnable.

## Control Flow Graph

r[control-flow-graph.valid-program]
CFG construction MUST reject branch and conditional-branch targets that are
out of range or that target instruction 0. The entry block model assumes no
internal predecessor to the first instruction.

r[control-flow-graph.leader]
Leader identification MUST mark instruction 0, every branch target, and every
instruction immediately following a terminator as a basic-block leader.

r[control-flow-graph.partition]
Basic block construction MUST produce non-empty half-open ranges that cover the
instruction stream contiguously from 0 to N. Adjacent blocks MUST meet exactly
at a boundary, with no gaps or overlaps.

r[control-flow-graph.edges]
CFG edges MUST follow the terminator of each basic block: normal instructions
fall through to the next block, unconditional branches jump only to their
unique target block, conditional branches jump to the target and fall through
when distinct, and returns produce no successors.

r[control-flow-graph.entry]
A non-empty CFG MUST expose block 0 as its entry block.

r[control-flow-graph.exit]
A block MUST be classified as an exit block if and only if its final
instruction is a return.

r[control-flow-graph.well-formed]
A well-formed CFG MUST have a valid instruction stream and edge endpoints that
all refer to existing basic blocks.

r[control-flow-graph.reachability]
Reachability analysis MUST mark exactly the blocks reachable from entry block 0,
and dead-block detection MUST return exactly the complement set.

## Dominator Tree

r[dominator-tree.valid-graph]
Dominator analysis MUST reject edges whose endpoints fall outside the CFG node
range.

r[dominator-tree.entry-dominates]
The entry node MUST dominate every reachable node.

r[dominator-tree.partial-order]
The computed dominance relation MUST behave as a partial order over reachable
nodes: reflexive, transitive, and antisymmetric.

r[dominator-tree.idom-unique]
Every reachable non-entry node MUST have exactly one immediate dominator.

r[dominator-tree.tree-structure]
Immediate-dominator links MUST form a parent-pointer tree rooted at the entry.

r[dominator-tree.convergence]
The iterative dominator algorithm MUST converge to the same dominator sets and
immediate-dominator map as the path-based definition.

r[dominator-tree.dominance-frontier]
A node MUST appear in another node's dominance frontier exactly when one of its
predecessors is dominated by that node but the node does not strictly dominate
the join point itself.

## Liveness Analysis

r[liveness.convergence]
Backward liveness analysis MUST start from empty live sets and reach a fixed
point within 2 × V × B synchronous rounds for V variables and B blocks. Each
round MUST be monotonic: no live bit may flip from true back to false.

r[liveness.equations]
At a fixed point, live-out MUST equal the union of successor live-in rows, and
live-in MUST equal use ∪ (live-out \ def) for every block.

r[liveness.use-def]
A variable MUST be live-in at a block if and only if there exists a path from
that block to a block that uses the variable before any intervening block on
that path defines it.

r[liveness.connected]
If a def-free witness path explains why a variable is live-in at a block, then
that variable's projected live range MUST include every block on that witness
path.

r[liveness.def-kills]
If a block defines a variable and does not use it before that definition, the
variable MUST NOT be live-in to that block.

r[liveness.exit]
A block with no CFG successors MUST have empty live-out. Return values, if any,
MUST be modeled as local uses of the exit block rather than as post-exit
liveness.

## Loop Detection

r[loop.back-edge]
A CFG edge MUST be classified as a back edge if and only if its target
block dominates its source block.

r[loop.body]
Given a CFG, a reachability bitmap, and a dominance matrix for the same graph,
loop-body detection MUST return exactly the seeded backward closure for (B -> H):
start from {H, B}, then repeatedly add any reachable block dominated by H whose
successor is already in the current body, excluding H as an interior successor
for non-self loops. Randomized tests SHOULD match this closure against the
path-based natural-loop oracle for reachable non-self back edges.

r[loop.header-dominance]
Every reachable block in a detected non-self loop body MUST be dominated by
that loop's header.

r[loop.single-entry]
Given a reachability bitmap for the CFG entry and a detected non-self loop
body, single-entry checking MUST return true if and only if every reachable edge
entering the body from outside targets the header. Reachable non-header side
entries are not allowed.

r[loop.nesting]
If one detected loop is nested in another, the inner loop body MUST be a strict
subset of the outer loop body and the inner header MUST be contained in the
outer body.

r[loop.nesting-forest]
Loop-nesting parents MUST form an acyclic forest of strict-body-superset links:
any recorded parent of a loop MUST strictly contain that loop's body.

r[loop.exits]
Loop-exit detection MUST return exactly the CFG edges whose source is inside a
loop body and whose destination is outside that body.

## SSA Construction

r[ssa.single-assignment]
SSA construction MUST assign a fresh version to every phi definition and every
instruction definition, with no two explicit definitions sharing the same
version.

r[ssa.dominance-of-uses]
Every SSA use MUST be dominated by the definition of its version: instruction
uses by a dominating phi or earlier instruction definition, and phi arguments
by a definition that dominates the end of the predecessor edge they reference.

r[ssa.phi-placement]
Phi placement MUST insert a phi for variable V at exactly the blocks in the
iterated dominance frontier of V's definition set. Each placed phi MUST contain
exactly one argument slot per distinct predecessor block.

r[ssa.phi-semantics]
A phi node MUST select the version associated with the predecessor edge that was
actually taken.

r[ssa.destruction]
Phi elimination MUST replace each block's phi nodes with parallel copies on each
incoming edge so that executing the eliminated program along any CFG path
produces the same versioned environment as executing the SSA program.

r[ssa.idf-convergence]
Iterated dominance frontier computation MUST grow monotonically from the empty
set and reach a fixed point within at most N rounds for an N-block CFG.

## Dead Code Elimination

r[dce.side-effect]
Dead-code classification MUST treat raw opcodes `>= 128` as conservatively
effectful, with `128` = Store, `129` = Call, and `130` = Return.

r[dce.criterion]
An instruction MUST be classified as dead if and only if it has no side effect,
defines a variable, and that definition is not live-out at the instruction's
program point. Instructions without a definition MUST remain conservatively
live in this initial model.

r[dce.side-effect-never-dead]
An instruction with a side effect MUST never be classified as dead, regardless
of its live-out row.

r[dce.live-def-never-dead]
An instruction that defines a variable that is live-out MUST never be
classified as dead.

r[dce.no-live-removed]
Dead-code elimination MUST preserve every instruction whose output is live at
its program point or that has a side effect. Removing dead instructions MUST
preserve block count and CFG edges.

r[dce.semantics]
Under the raw IR's current straight-line block-order execution model, dead-code
elimination MUST preserve the observable trace of stores, calls, and returns on
any input environment compatible with the chosen live-out rows.

r[dce.retained-identical]
If two executions agree on a kept instruction's entry-live variables, that
instruction MUST emit the same observable event and produce the same live-out
values in both executions.

r[dce.removed-no-effect]
A removed dead instruction MUST emit no observable event and MUST preserve the
values of all variables that remain live after that instruction.

r[dce.idempotent]
A second elimination pass on the result of DCE MUST keep every instruction,
producing the same output as the first pass.

## Hypervisor Memslots

r[hypervisor-memslot.disjoint]
A memslot table MUST represent guest-frame-number (GFN) ranges as sorted,
non-overlapping half-open intervals `[base_gfn, base_gfn + npages)`. Adjacent
memslots MAY touch at a boundary, but no guest frame may belong to two
memslots at once.

r[hypervisor-memslot.lookup]
`lookup_memslot(slots, gfn)` MUST return the unique memslot whose guest-frame
range contains `gfn`, or `None` if no memslot contains it.

## Hypervisor GPA Translation

r[hypervisor-gpa.translate]
A successful GPA translation MUST preserve the byte offset inside the owning
memslot. The translated host address MUST be `host_base + (gfn - base_gfn)`
and MUST lie inside that memslot's host-backed region.

r[hypervisor-gpa.access]
GPA translation with access mode MUST distinguish at least three outcomes:
`Translated`, `Unmapped`, and `ReadonlyWrite`. A write into a readonly memslot
MUST return `ReadonlyWrite`. An address outside all memslots MUST return
`Unmapped`.

## Hypervisor Dirty Log

r[hypervisor-dirty-log.index]
The dirty-log bit index for a guest frame MUST be the frame's offset relative
to its memslot base: `dirty_index(base_gfn, gfn) == gfn - base_gfn`, and MUST
be in `[0, npages)` exactly when `gfn` belongs to the slot.

r[hypervisor-dirty-log.mark]
Marking a guest frame dirty MUST set exactly its relative dirty bit and leave
all other bits unchanged. Marking the same frame multiple times MUST be
idempotent.

r[hypervisor-dirty-log.union]
Merging dirty logs for the same memslot MUST behave as bitwise union. Clearing
a dirty log MUST reset all tracked bits to zero.

## Hypervisor vCPU State

r[hypervisor-vcpu.run-state]
A vCPU MUST be in exactly one of four abstract states: `Runnable`, `Halted`,
`Blocked`, or `Stopped`. `vcpu_can_enter_guest` MUST return true exactly in the
`Runnable` state. `Stopped` is terminal.

r[hypervisor-vcpu.halt-wakeup]
A halt from `Runnable` MUST move the vCPU to `Halted`. A deliverable wake event
MUST move a halted vCPU back to `Runnable`. Non-deliverable events leave the
state unchanged.

## Hypervisor Interrupts

r[hypervisor-interrupt.inject]
The pending virtual-interrupt structure MUST support enqueue-or-mark without
losing prior pending interrupts. Next-interrupt selection MUST be deterministic.
Re-marking an already pending interrupt MUST be idempotent.

r[hypervisor-interrupt.deliver]
A pending interrupt MUST be delivered only when the guest is interruptible.
Successful delivery MUST consume the selected pending interrupt. If the guest is
not interruptible, the pending set MUST remain unchanged.

## Hypervisor VM Exit

r[hypervisor-exit.classify]
VM-exit classification MUST map raw exit information into stable semantic
classes. The same raw input MUST always classify to the same class.

r[hypervisor-exit.disposition]
A semantic exit class MUST map to exactly one disposition: `ResumeGuest`,
`UserspaceHandle`, `Block`, or `Terminate`. IO/MMIO/Hypercall MUST map to
`UserspaceHandle`. Shutdown/InternalError MUST map to `Terminate`.

## Hypervisor Cross-Module Properties

r[properties.hypervisor-translation-region]
Every successful GPA translation MUST produce a host address contained in the
translated memslot's host-backed region.

r[properties.hypervisor-wakeup]
If a halted vCPU has a pending deliverable interrupt, wake evaluation MUST move
the vCPU to `Runnable`. If delivery is not permitted, the vCPU MUST remain
halted.

r[properties.hypervisor-exit-state]
Exit handling MUST preserve vCPU state consistency: `Block` leaves the vCPU
non-runnable, `Terminate` moves it to the terminal stopped state.

## Repository Index

r[repo-index.content-digest]
The repository index MUST compute a BLAKE3 digest from the raw bytes of each
tracked file and use that digest as the content identity key for the active
path view. If two tracked paths have identical bytes, their active path rows
MUST reference the same digest.

r[repo-index.persistent-store]
The repository index MUST persist committed path rows, extracted facts, and
stats in redb so a later process can reopen the database and answer queries
from the last successful refresh without reparsing tracked files first.

r[repo-index.atomic-refresh]
A repository-index refresh MUST publish at most one committed generation at a
time. Readers MUST observe either the prior committed generation or the newer
committed generation, never a mix. If a refresh fails before commit, the prior
committed generation MUST remain readable.

r[repo-index.incremental-refresh]
A repository-index refresh MUST update the active path view for added,
changed, unchanged, and deleted tracked files. When a tracked path keeps the
same digest and extractor kind across refreshes, the index MUST reuse the
stored extracted facts instead of reparsing the file.

r[repo-index.cross-reference]
The repository index MUST extract cross-reference facts needed for local repo
navigation: Rust items and `r[...]` reference sites from `src/**/*.rs`,
requirement definitions from `docs/requirements.md` and active OpenSpec
markdown, plus headings and active change/domain metadata from active
OpenSpec markdown.

r[repo-index.query]
The repository index MUST support deterministic local queries over committed
index state for `path`, `symbol`, `requirement`, `digest`, and `stats`.
Path queries MUST report the indexed facts for a tracked file. Requirement
queries MUST report the stored definition plus related implementation,
verification, dependency, or related reference sites.

## Web of Trust

r[web-of-trust.principal-fingerprint]
A principal identifier MUST be the full 32-byte BLAKE3 digest of the
principal's canonical public-key bytes. The same canonical bytes MUST always
produce the same identifier, and equality checks MUST use all 32 digest
bytes rather than a truncated prefix.

## seL4 ASID Pool

r[sel4-asid.uniqueness]
A successful ASID allocation MUST make the returned ASID live for the
requesting address-space object ID, change only that returned slot, and leave
all other live ASID ownership unchanged. One owner MAY hold multiple live
ASIDs, but each live ASID MUST be bound to at most one live owner at a time.

r[sel4-asid.release-state]
A valid release MUST require an exact `(asid, owner)` match, return success,
clear only the released ASID slot, and leave all unrelated live ownership
unchanged. Invalid, out-of-range, mismatched-owner, or duplicate release MUST
return failure and leave pool ownership unchanged.

r[sel4-asid.reuse]
A released ASID MAY be reused later, but only after the release step has
cleared the prior live binding for that numeric ASID.

r[sel4-asid.pool-bounds]
Allocation MUST succeed whenever some free ASID exists, return only ASIDs in
`0..capacity-1`, and fail exactly when no free ASIDs remain. Failed allocation
MUST leave pool ownership unchanged, including the zero-capacity case.

## seL4 Capability Rights

r[sel4-cap-rights.attenuation]
A derived capability MUST NOT grant any right absent from its source
capability. Copy MAY preserve rights and masking MAY drop rights, but
attenuation MUST NOT add authority.

r[sel4-cap-rights.operation-gates]
Kernel operations MUST require the corresponding authority on the acting
capability. Send requires send authority, receive requires receive authority,
grant-like transfer requires grant authority, mapping requires map authority,
and control operations require control authority.

r[sel4-cap-rights.masking]
Rights masking MUST be idempotent and monotone. Applying the same mask twice
MUST equal applying it once, and masking with a smaller mask MUST NOT produce
a larger rights set.

## seL4 CSpace Lookup

r[sel4-cspace.lookup]
CSpace lookup MUST resolve a slot by guarded radix traversal. Each level MUST
check the guard, consume the matching guard depth plus one slot-selection
segment, and select exactly one child slot within the current CNode.

r[sel4-cspace.unique-slot]
A successful lookup MUST identify exactly one slot, and that slot MUST be
within the bounds of the resolved CNode.

r[sel4-cspace.failure-no-mutation]
A failed lookup MUST NOT modify CSpace state, consumed-depth accounting, or
capability contents.
