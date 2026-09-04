# Trellis Requirements

This document defines the requirements for Trellis, the `verified-logic` Rust library.
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

r[chain.tip-commitment]
The chain-tip commitment MUST return the genesis hash for an empty chain
and the final entry hash for a non-empty chain. For a valid chain, the tip
commitment MUST commit to the full prefix under the existing agreement and
collision-free assumptions.

r[chain.extension-prefix]
A chain extension predicate MUST hold exactly when the shorter chain is an
unchanged prefix of the longer chain. Extension MUST remain separate from
hash validity, while valid full chains MUST yield valid selected prefixes.

r[chain.checkpoint-anchor]
A checkpoint anchor over an index and expected hash MUST be valid only when
the chain is valid, the index is in range, and the entry hash at that index
matches the expected hash. Matching checkpoint anchors MUST authenticate the
prefix through that index.

r[chain.fork-detection]
Fork detection MUST identify whether two chains diverge. For valid chains of
the same length, equal tip commitments MUST imply no divergence, and any
entry divergence MUST force different tip commitments.

r[chain.batch-append]
Batch append MUST fold correctly hashed entries over a valid base chain,
preserve validity at each append step, and leave the original prefix
unchanged. Incorrectly hashed batch entries MUST not receive a verified
append-preserves claim.

r[chain.log-shape]
Log-shape predicates MUST be separate from hash validity and MUST cover the
chosen contiguous/increasing index policy plus term policy. Hash validity
alone MUST NOT imply index or term well-formedness.

r[chain.tamper-classification]
Tamper-classification lemmas MUST classify detectable valid-chain divergence
as data identifier, index, term, or predecessor/hash tampering while reusing
existing collision-free, agreement, and divergence facts.

r[chain.domain-separated-hash]
Domain-separated hash input contracts MUST distinguish entry, checkpoint,
receipt, and snapshot-style commitments. A hash from one domain MUST NOT be
accepted as verified evidence for another domain without an explicit
compatibility wrapper.

r[chain.runtime-blake3-alignment]
Runtime chain hashing MUST either use the documented deterministic BLAKE3
serialization boundary or keep any deterministic test-mixer bridge explicit
as a trusted/runtime boundary with no cryptographic-strength claim.

r[chain.compact-history-proof]
An optional compact-history overlay MAY provide skip-link, Merkle, or MMR
membership/range proofs. The overlay MUST compose with base chain tips or
checkpoints and MUST NOT replace or weaken linear chain validity.

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

## Cache Partitioning

r[cache-partition.isolation]
A lookup for partition P MUST NOT hit a line stored in a way assigned to
partition Q (Q ≠ P). lookup_partitioned MUST only examine ways assigned
to the requesting partition.

r[cache-partition.eviction-containment]
Eviction within partition P MUST only select victim ways assigned to P.
Lines in other partitions MUST be untouched by the eviction.

r[cache-partition.capacity]
The sum of all partition sizes (in ways) MUST equal the total number of
ways. No capacity is lost or double-counted at partition boundaries.

r[cache-partition.assignment]
Every way MUST be assigned to exactly one valid partition. A valid
assignment maps every way to a partition ID in [0, num_partitions).

r[cache-partition.resize]
Resizing a partition MUST invalidate transferred ways before
reassignment. Non-transferred ways MUST remain unchanged in both
assignment and data. Total capacity MUST be preserved.

r[cache-partition.full-equivalence]
A single partition containing all ways MUST behave identically to an
unpartitioned cache: lookup examines all ways and eviction considers
all ways.

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

## False Sharing Detection

r[false-sharing.same-line]
same_cache_line MUST return true if and only if the two addresses are in
the same cache line. For power-of-two `line_size`, this is equivalent to
comparing the address bits above the offset.

r[false-sharing.distinct-line]
If two addresses are on different cache lines, they MUST NOT false-share.
For canonical tag/index decomposition, this means at least one of tag or
index differs.

r[false-sharing.alignment]
is_line_aligned MUST return true if and only if `addr % line_size == 0`.
line_start MUST clear the offset bits and return the first byte address of
the containing line.

r[false-sharing.padding]
padding_prevents_sharing(addr_a, size_a, addr_b, line_size) MUST return
true if and only if object B starts at least one full cache line after the
end of object A: `addr_b >= addr_a + size_a + line_size`. Under that
condition, no byte of A may share a cache line with any byte at or after
`addr_b`.

r[false-sharing.object-span]
cache_lines_spanned MUST return the number of cache lines touched by a
positive-size object, equal to `ceil((addr % line_size + size) /
line_size)`. For this computation, `(addr % line_size) + size` MUST fit
in `u64`.

r[false-sharing.decomposition-equivalence]
For `offset_bits < 64` and `offset_bits + index_bits <= 64`,
same_cache_line(a, b, 1 << offset_bits) MUST be equivalent to
`extract_tag(a, offset_bits, index_bits) == extract_tag(b, offset_bits,
index_bits)` and `extract_index(a, offset_bits, index_bits) ==
extract_index(b, offset_bits, index_bits)`.

## Write Policy

r[write-policy.write-back-dirty]
Under write-back policy, a store MUST set the dirty bit and a freshly
loaded line MUST be clean.

r[write-policy.write-back-eviction]
Evicting a dirty line MUST write its data to memory at the selected
address. Evicting a clean line MUST leave memory unchanged.

r[write-policy.write-through-clean]
Under write-through policy, stores and loads MUST leave the dirty bit
clear.

r[write-policy.write-through-memory]
A write-through store MUST update memory immediately at the selected
address, and the updated memory cell MUST equal the cache line's new data.

r[write-policy.policy-non-interference]
Updating one cache line in a mixed-policy cache MUST NOT change any other
line's state.

r[write-policy.flush-all]
When every clean line already matches memory, flush_all MUST clear every
line's dirty bit and leave memory matching every cache line's data.

r[write-policy.read-equivalence]
A read hit MUST return the cached data regardless of whether the line uses
write-back or write-through policy.

## MOESI Coherence

r[moesi.single-writer]
At most one cache MAY hold a line in a dirty-owner state (`Modified` or
`Owned`) at a time.

r[moesi.owned-entry]
A remote read against a `Modified` owner MUST move that owner to
`Owned` while the requester receives a shared copy.

r[moesi.owned-supply]
When any cache holds a line in `Owned`, reads MUST source dirty data from
some cache rather than from memory.

r[moesi.writeback]
Evicting an `Owned` line MUST write back to memory before the owner is
lost. Evicting a `Shared` line MUST NOT require a writeback.

r[moesi.transitions]
Every valid processor event and snoop bus action MUST produce a valid
next state and a valid bus action/result.

r[moesi.mesi-refinement]
For transitions that stay within the `M/E/S/I` subset, MOESI MUST agree
with the existing MESI transition model after mapping `Owned -> Shared`.

r[moesi.no-stale-reads]
Under the memory-consistency relation `memory_up_to_date <==>
!dirty_owner_exists`, reads MUST use memory only when memory is current;
otherwise some cache MUST be the read source.

## Cache Hierarchy

Cache hierarchy levels are modeled as canonical finite sets of line tags.
Exec code may represent a level as a deduplicated sequence, but the
requirements below are about line presence and absence.

r[hierarchy.inclusive-invariant]
Under the inclusive policy, every line present in L1 MUST also be present
in L2 after every modeled operation.

r[hierarchy.back-invalidation]
Under the inclusive policy, evicting a line from L2 MUST also invalidate
that line from L1 so the inclusive invariant is preserved.

r[hierarchy.exclusive-invariant]
Under the exclusive policy, no line may be present in both L1 and L2 at
the same time.

r[hierarchy.exclusive-swap]
Under the exclusive policy, promoting a line from L2 to L1 MUST remove it
from L2 in the same abstract step.

r[hierarchy.non-inclusive]
Under the non-inclusive policy, the hierarchy MUST impose no subset or
disjointness constraint between L1 and L2.

r[hierarchy.capacity]
For canonical levels, the inclusive effective capacity MUST equal |L2|,
and the exclusive effective capacity MUST equal |L1| + |L2|.

r[hierarchy.snoop-scope]
Under the inclusive policy, an L2 miss MUST imply an L1 miss for the same
line. Under the exclusive policy, checking L2 alone MUST NOT be assumed
sufficient.

## Store Buffer Forwarding

r[store-buffer.youngest-match]
When more than one buffered store can satisfy a load, forwarding MUST
select the youngest matching store.

r[store-buffer.full-coverage]
A load MUST forward from a buffered store only when that store fully
covers the load byte range.

r[store-buffer.cache-fallback]
When no buffered store matches a load, the load MUST use the cache path
instead of forwarding.

r[store-buffer.drain-ordering]
Buffered stores MUST drain in FIFO order: the oldest store commits first,
and draining one store MUST preserve the relative order of the remaining
stores.

r[store-buffer.fence]
A store fence MUST leave the buffer empty before any post-fence store is
enqueued.

r[store-buffer.capacity]
When the store buffer is full, enqueue MUST reject the new store rather
than silently dropping it. When the buffer is not full, enqueue MUST grow
the buffer by exactly one entry.

## Cache Prefetch

r[cache-prefetch.no-fault]
Prefetching an invalid or inaccessible address MUST be a no-op for the
modeled cache state and MUST NOT modify architectural state.

r[cache-prefetch.invisible]
Prefetch operations MUST leave registers, memory contents, and the program
counter unchanged. Prefetch only affects modeled cache state.

r[cache-prefetch.cache-effect]
A valid prefetch-for-read MUST leave the requester in a valid MESI state
matching a demand read miss, and a valid prefetch-for-write MUST leave the
requester in Exclusive while invalidating other modeled holders.

r[cache-prefetch.coherence]
A valid prefetch-for-read MUST issue BusRead and a valid prefetch-for-write
MUST issue BusReadX. Invalid-address prefetches MUST issue no bus action.

r[cache-prefetch.drop-safe]
Dropping a modeled prefetch step MUST leave both architectural state and
cache state unchanged.

r[cache-prefetch.eviction-safe]
Replacing a prefetched line MUST write back a dirty victim to memory before
installation. Replacing a clean victim MUST leave memory unchanged.

## Directory Coherence

r[directory.consistency]
The directory's owner and sharer bit-vector MUST agree with the actual MESI
state held by each modeled cache at all times.

r[directory.invalidation]
Granting a write request MUST emit targeted invalidations to every other cache
that currently holds the line, and the post-ack state MUST leave those caches
invalid.

r[directory.single-writer]
A completed write grant MUST leave exactly one modeled cache with the line in
Modified and every other cache Invalid.

r[directory.eviction-notify]
Evicting a tracked line MUST clear the corresponding owner or sharer record. A
Modified owner eviction MUST request writeback before the line becomes uncached.

r[directory.serialization]
Same-line write requests MUST be serialized by applying them one at a time in a
total order. Requests to different lines MUST remain independent.

r[directory.mesi-embedding]
The directory-implied per-cache state MUST always be a valid MESI state, and a
consistent exclusive owner MUST hold the line in either Exclusive or Modified.

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

## Raft Consensus Domain Primitives

r[domain-primitives.raft-consensus.model]
Raft finite-model envelope primitives MUST admit node ids only inside a non-empty
one-based node universe, MUST bound terms, log indexes, command identifiers,
message kinds, and configuration epochs by caller-provided finite limits, and
MUST report deterministic first-failure metadata before higher-level transition
predicates rely on those references.

r[domain-primitives.raft-consensus.node-state]
Raft node-state envelope primitives MUST admit only follower, candidate, or
leader roles inside the finite model. Leader role metadata MUST belong to the
node current term, candidate role metadata MUST record a self-vote in that term,
and applied/commit/log indexes MUST remain ordered.

r[domain-primitives.raft-consensus.applied-prefix]
Raft applied-prefix primitives MUST admit only monotone contiguous application
steps. Non-empty steps MUST start at the next unapplied index, the new
last-applied index MUST be committed and present in the local log, and no-op
steps MUST carry no first-applied index.

r[domain-primitives.raft-consensus.client-proposal]
Raft client-proposal primitives MUST admit finite command identifiers only when
the target node is model-valid and admitted as leader for the exact proposal
term. Proposals to followers, stale terms, future terms, unknown commands, or
out-of-model nodes MUST reject with deterministic metadata and MUST NOT claim
client retry, end-to-end linearizability, or application execution behavior.

r[domain-primitives.raft-consensus.term]
Raft term update primitives MUST keep the receiver current term monotone when
observing any term-bearing input. Observing a lower or equal term MUST keep the
current term unchanged; observing a higher term MUST advance to that term.

r[domain-primitives.raft-consensus.message]
Raft message primitives MUST define finite term-bearing envelopes for the modeled
Raft input classes and reject invalid envelope kinds or node-universe bounds.
Stale-message predicates MUST classify a term-bearing message as stale if and
only if the message term is lower than the receiver current term. Stale term
classification and envelope admission MUST NOT require network delivery, timing,
or transport claims.

r[domain-primitives.raft-consensus.vote]
Raft vote-grant primitives MUST enforce one vote per term and MUST admit a
candidate only when the candidate term is compatible with the recorded vote state
and the candidate log epoch is at least as up to date as the voter log epoch.

r[domain-primitives.raft-consensus.request-vote]
RequestVote admission MUST compose stale-term rejection with the deterministic
vote-grant predicate. A candidate with a stale log or a distinct same-term prior
vote MUST be rejected.

r[domain-primitives.raft-consensus.election-safety]
Raft election-safety proofs MUST show that finite vote histories satisfying
single-vote-per-term and quorum intersection admit at most one leader for a term.

r[domain-primitives.raft-consensus.log-entry]
Raft log-entry primitives MUST admit only explicit finite one-based indexes,
bounded terms, and finite command identifiers. Entry admission MUST reject index
zero, indexes outside the declared bound, and terms above the declared bound
without claiming client I/O or application command semantics.

r[domain-primitives.raft-consensus.log-prefix]
Raft log-prefix primitives MUST define contiguous no-gap logs, exact prefix
relations, and contiguous append preservation over finite indexed entries.
A gap in the one-based index sequence MUST be rejected, and appending the next
contiguous index MUST preserve the no-gap predicate.

r[domain-primitives.raft-consensus.persistent-state]
Raft persistent-state metadata primitives MUST model only finite persisted term
and vote metadata. Observing a higher term MUST advance the persisted current
term and clear the recorded vote; observing a non-higher term MUST leave the
metadata unchanged. These primitives MUST NOT claim fsync, crash recovery, or
storage durability behavior.

r[domain-primitives.raft-consensus.volatile-state]
Raft volatile-state metadata primitives MUST keep applied, committed, and local
log indexes ordered. Commit-index advance admission MUST reject backwards
movement and indexes beyond the local log without claiming durability,
performance, or application side effects.

r[domain-primitives.raft-consensus.append-entries]
Raft AppendEntries admission primitives MUST compose stale-term rejection with
the previous-log index/term consistency check. A stale leader term or a previous
log mismatch MUST reject the request. Conflict metadata MUST be bounded by the
finite follower-log length, and admitted follower log-update predicates MUST
preserve the retained previous-log prefix before appending finite entries
without claiming network delivery, storage durability, or follower mutation
semantics beyond this finite admission/update slice.

r[domain-primitives.raft-consensus.log-matching]
Raft log-matching primitives MUST show that agreement at a one-based log index
implies equal terms for every prior index in the finite prefix. Follower
log-update predicates admitted at a previous-log index MUST preserve that
matching prefix before any appended entries.

r[domain-primitives.raft-consensus.follower-log-update]
Raft follower log-update primitives MUST model rejected AppendEntries updates as
no-mutation outcomes and accepted updates as finite retained-prefix plus
contiguous-append outcomes. Accepted metadata MUST keep the retained previous-log
index inside the finite follower log and MUST report a successor length equal to
the retained prefix plus admitted entry count without claiming storage durability,
network delivery, or production follower mutation semantics beyond this finite
update slice.

r[domain-primitives.raft-consensus.commit]
Raft commit-index primitives MUST admit a candidate commit index only when a
finite quorum of replicas has matched that index, the candidate does not move
backwards from the current commit index, and the candidate is inside the local
log. Rejected quorum commits MUST leave the commit index unchanged.

r[domain-primitives.raft-consensus.leader-completeness]
Raft leader-completeness primitives MUST admit a later leader log only when the
finite committed entry at the modeled position is still present with the same
index, term, and command identifier. A later leader missing that committed entry
MUST be rejected by the finite predicate.

r[domain-primitives.raft-consensus.state-machine-safety]
Raft state-machine safety primitives MUST admit shared applied-prefix positions
only when both finite logs have applied that position and the command identifiers
at that position are equal. Divergent applied commands at the same position MUST
be rejected.

r[domain-primitives.raft-consensus.snapshot]
Raft snapshot primitives MUST model finite snapshot metadata with a coherent
empty/non-empty boundary, match non-empty snapshot boundaries against the
compacted log prefix term, and preserve the suffix after prefix compaction
without claiming transfer, disk durability, or application snapshot correctness.

r[domain-primitives.raft-consensus.install-snapshot]
Raft InstallSnapshot admission primitives MUST accept only non-stale leader
terms, strictly newer snapshot boundaries, and prefix-compatible snapshot
metadata. Stale or boundary-mismatched snapshots MUST be rejected by the finite
predicate.

r[domain-primitives.raft-consensus.membership]
Raft membership primitives MUST model finite configuration epochs, old/new voter
cardinalities, and joint-consensus quorum metadata bounded to a modeled node
universe. Membership epoch admission MUST reject regressions, non-joint
configuration metadata MUST keep old/new voter counts identical, and joint
consensus quorum admission MUST require both old-configuration and
new-configuration majorities without claiming dynamic reconfiguration liveness,
service discovery, persistence, or rollout behavior.

r[domain-primitives.raft-consensus.trace]
Raft transition-trace primitives MUST model finite adjacent steps whose input is
a valid finite Raft message envelope. Non-stale admitted steps MUST advance the
receiver term according to the monotone observed-term rule, while stale rejected
steps MUST preserve the receiver term and carry no unsafe mutation. Trace
admission MUST NOT claim network ordering, delivery, fairness, storage,
availability, or runtime scheduling behavior.

r[domain-primitives.raft-consensus.invariant-bundle]
Raft invariant-bundle primitives MUST compose finite local model validity from
term/persistent metadata consistency, volatile applied/commit/log bounds,
contiguous no-gap indexed logs, coherent snapshot metadata, and bounded
membership metadata. Invariant-preservation admission MUST expose a valid
post-transition bundle, no-mutation admissions MUST preserve the bundle
unchanged, and focused proof lemmas MUST show preservation for modeled
observed-term, commit-advance, membership-epoch, InstallSnapshot, and
transition-trace slices without claiming runtime networking, storage durability,
timing, liveness, availability, or production implementation correctness.

r[domain-primitives.raft-consensus.admission]
Raft deterministic-admission primitives MUST combine finite before-invariant,
message-envelope, transition-trace, and after-invariant checks in a stable order.
Admission results MUST expose a finite success/failure code, accepted candidates
MUST report the stable OK code, and rejected candidates MUST report the first
failing modeled finite check without claiming runtime network, clock, storage,
availability, or production implementation behavior.

r[domain-primitives.raft-consensus.openraft-log-pointers]
OpenRaft-inspired log-pointer primitives MUST model finite pointer ordering for
purged, snapshot, applied, committed, flushed, submitted, and accepted indexes.
The pointer lattice MUST reject regressions where applied exceeds committed,
committed exceeds submitted, flushed exceeds submitted, or submitted exceeds
accepted. Admission metadata MUST return OK exactly for the valid pointer
predicate and otherwise report the first pointer-order violation in stable
lattice order, without claiming disk durability or storage implementation
behavior.

r[domain-primitives.raft-consensus.openraft-io-progress]
OpenRaft-inspired I/O progress primitives MUST model finite flushed, submitted,
and accepted cursors. Progress metadata MUST satisfy flushed <= submitted <=
accepted, and admitted progress updates MUST move every cursor monotonically.
Admission metadata MUST return OK exactly for the valid monotone-update predicate
and otherwise report invalid old metadata, invalid new metadata, or the first
regressing cursor without claiming fsync, crash recovery, or storage durability.

r[domain-primitives.raft-consensus.openraft-vote-log-io]
OpenRaft-inspired vote/log I/O ordering primitives MUST admit a finite log write
only when term/vote metadata for that log term has already been ordered first.
Admission metadata MUST return OK exactly for the ordering predicate. These
predicates MUST NOT claim fsync, crash recovery, or storage durability.

r[domain-primitives.raft-consensus.openraft-effective-membership]
OpenRaft-inspired effective-membership primitives MUST model a committed epoch
and an effective epoch where committed <= effective. A conflicting uncommitted
effective membership rollback MUST restore the effective epoch to the committed
epoch. Admission metadata MUST return OK exactly for ordered committed/effective
metadata and MUST report older-effective metadata deterministically without
claiming runtime joint-consensus reconfiguration behavior.

r[domain-primitives.raft-consensus.openraft-replication-session]
OpenRaft-inspired replication-session primitives MUST isolate finite replication
progress by leader term, leader node, and governing membership log index. Stale
session updates, backwards matched-index movement, and matched indexes beyond the
leader log MUST be rejected. Admission metadata MUST return OK exactly for the
current-session bounded-update predicate and otherwise reject stale sessions
before matched-index movement or leader-log-bound checks.

r[domain-primitives.raft-consensus.openraft-read-linearizer]
OpenRaft-inspired read-linearizer primitives MUST model only finite readiness:
a read is ready exactly when local last-applied index has caught up to the read
log index. Admission metadata MUST return OK exactly for the local-apply catch-up
predicate. These predicates MUST NOT claim quorum RPC behavior, lease timing,
clock drift bounds, network delivery, or end-to-end application linearizability.

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

r[verification-gaps.nix-version-bridge-decomposition]
Nix version bridge decomposition MUST route admitted finite-model inputs through verified executable comparison helpers, keep out-of-model inputs on an explicit fallback trusted boundary, and preserve the trusted-boundary inventory without replacement weakened markers.

r[verification-gaps.nix-version-modeled-compare-bridge]
The modeled Nix version comparator MUST compute admitted finite-model inputs through verified executable separator, component, numeric, and lexicographic helpers, return None for out-of-model inputs, and ensure the public compare_versions API uses the modeled path for admitted inputs while leaving only the unmodeled fallback visible in the trusted-boundary inventory.

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

## De Bruijn Substitution

r[debruijn.well-scoped]
A de Bruijn term MUST be well scoped at depth d if and only if every
variable index appearing in the term is strictly smaller than d at the
point where it is interpreted. Entering an abstraction MUST increase the
available depth by one.

r[debruijn.shift-zero]
Shifting by zero MUST be the identity for every cutoff and term.

r[debruijn.shift-compose]
Shifting by m and then by n at the same cutoff MUST equal a single shift
by m + n at that cutoff.

r[debruijn.shift-preserves-scope]
If a term is well scoped at depth d and the shift cutoff is at most d,
then shifting by k MUST produce a term that is well scoped at depth
d + k.

r[debruijn.identity-substitution]
Lifting a term by one at cutoff c and then substituting the matching
variable c back in MUST recover the original term.

r[debruijn.subst-preserves-scope]
If a body is well scoped at depth d + 1 and the replacement term is well
scoped at depth d, then substituting the replacement for variable 0 MUST
produce a term that is well scoped at depth d.

r[debruijn.beta-scope]
Beta reduction of a well-scoped redex `App(Abs(body), arg)` MUST produce
an output term that is well scoped at the original outer depth. Non-redex
terms MUST not beta-reduce.

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

## Per-Core RWLock

r[per-core-rwlock.aggregation]
sum_readers computes the exact mathematical sum of all per-core reader counts.
aggregate_reader_count computes the same sum with u64 saturation in exec code.

r[per-core-rwlock.independence]
read_acquire_core and read_release_core update only the selected core's count.
All other core counts remain unchanged.

r[per-core-rwlock.mutual-exclusion]
Readers may acquire only when the write flag is clear. Aggregated mutual
exclusion holds exactly when either no writer flag is set or the aggregated
reader count is zero.

r[per-core-rwlock.writer-drain]
set_write_flag starts writer drain by setting the global write flag.
writer_drain_complete is true exactly when the aggregate reader count is zero.
Releasing one active reader decreases the mathematical aggregate by exactly 1.

r[per-core-rwlock.mode-equivalence]
per_core_mode maps aggregated per-core state to the same Free/Read/Write mode
surface as the global rwlock. clear_write_flag clears the writer flag, and if
no readers remain the resulting observable mode is Free.

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

## Verification Frontier

r[verification-frontier.contract-implication]
Contract implication admission MUST classify a callee precondition under finite caller facts, path facts, and type-bound facts as Proven, Rejected, or Unknown. Proven MUST mean the modeled facts cover the precondition and MAY be used as check-elision evidence inside the finite model. Rejected MUST expose deterministic violation metadata or a directly false precondition. Unknown MUST remain non-proof evidence.

r[verification-frontier.type-bound-facts]
Type-bound fact extraction MUST expose deterministic range facts for modeled boolean, unsigned integer, and signed integer domains. The extracted facts MUST use named domain constants, distinguish signed from unsigned domains, and compose with caller-provided ranges without widening a stricter caller bound.

r[verification-frontier.slice-access-certificate]
Slice-access certificate admission MUST accept only finite certificates proving the slice range is within the backing length and every fixed or bounded dynamic offset lies inside that slice. Inverted ranges, out-of-backing ranges, out-of-slice offsets, and unbounded dynamic offsets MUST be rejected or classified as Unknown rather than accepted.

r[verification-frontier.arena-scope-provenance]
Arena scope-provenance admission MUST track arenas, allocations, allocator wrappers, and containers by scope depth or explicit untracked provenance. A deeper local-scope value MUST NOT escape by return across a shallower boundary or be stored into a shallower-lived destination, while untracked heap-backed values MUST NOT be treated as arena-tainted without an explicit provenance fact.

r[verification-frontier.progress-budget]
Progress-budget admission MUST classify finite task/control-flow summaries using named operation-cost constants and named budget values. Admission MUST be Proven only when every modeled acyclic path reaches yield or return within budget and no modeled cycle can avoid yield or return indefinitely; over-budget paths and no-yield cycles MUST be rejected with deterministic metadata.

r[verification-frontier.external-reference-boundary]
Documentation and evidence for Lattice/Salt-inspired verification-frontier primitives MUST label Lattice/Salt as an external design reference only. Trellis MUST NOT treat Lattice code, benchmarks, or solver behavior as trusted validation, and every Trellis claim MUST be backed by repo-local specs, proofs, tests, Tracey coverage, or validation logs.

r[verification-frontier.outcome-invariant-bundle]
Verification-frontier admission helpers MUST expose reusable invariant facts for public admission outcomes. For every public admission family, Proven MUST imply the modeled safety predicate for that family, Unknown MUST remain non-proof evidence, and Rejected MUST NOT be accepted as proof or check-elision evidence.

r[verification-frontier.metadata-stability]
Verification-frontier admission helpers MUST use stable named reason codes and deterministic evidence fields for Rejected and Unknown outcomes. Evidence fields MUST be derived from modeled inputs or named constants and MUST NOT depend on external benchmarks, wall-clock time, process state, or solver behavior outside the Trellis proof surface.

## Verification Gap Inventory

16 total trusted/weakened/runtime-only proof-gap boundaries remain: 16
`#[verifier::external_body]` sites, 0 `assume_specification[...]` bridges, and
0 instruction-selection runtime-only public API exemptions. No
`@verified-logic:no-ensures` markers remain.

### Permanent trust boundaries (16 items)
These remain by design or because the current abstraction boundary is the
point of the model, not a proof target.

- 4 legacy cryptographic hash axioms / exec bridges in `chain.rs` and
  `snapshot_integrity.rs`; equal-input determinism now follows directly from
  pure spec equality.
- 1 version-comparison bridge in `nix_version.rs`; the recursive `cv_fuel`
  proof lemmas are now internalized, leaving only the exec/spec connection
  trusted.
- 10 UCAN codec/signature bridges in `ucan_did.rs` and `ucan_signature.rs`;
  `ucan_cid.rs` now reuses the shared DID base58btc contract for CID multibase
  round-trip instead of carrying a separate CID axiom, and public DID decode now
  composes verified prefix, base58, varint, and key-suffix primitives around the
  remaining base58btc contract.
- 1 web-of-trust crypto bridge in `web_of_trust_principal.rs`; attestation and
  revocation signature-input construction and comparison are now verified from
  canonical bytes.

### Closeable proof-gap boundaries (0 items)
The former instruction-selection runtime-only public API exemptions now have
direct implementation and verification-test coverage for their well-formedness,
matching, tiling, coverage, soundness, projection, evaluation, and validity
contracts.

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

## QuePaxa

r[consensus.quepaxa.boundary]
QuePaxa primitives MUST model only finite deterministic crash-stop safety/admission facts and MUST reject overclaims for real network liveness, probabilistic termination, random-number quality, benchmark results, production SMR behavior, adaptive MAB optimality, or Byzantine fault tolerance.

r[consensus.quepaxa.proposal-order]
QuePaxa proposal helpers MUST expose named constants for nil proposals, phase roles, initial round, phase count, and reserved leader priority; proposal ordering MUST be deterministic by priority, proposer identity, and value identity; reserved leader priority MUST be admitted only for the declared leader in the initial round.

r[consensus.quepaxa.isr]
QuePaxa interval summary register helpers MUST preserve same-step first values, aggregate same-step proposals by deterministic maximum, return the previous aggregate on immediate step advance, discard stale updates without changing state, and return nil for skipped-step prior aggregates.

r[consensus.quepaxa.tcast]
QuePaxa propagation admission MUST require quorum facts for existent, common, and universal proposal summaries and MUST establish the modeled universal subset common subset existent relationship.

r[consensus.quepaxa.abstract-round]
QuePaxa abstract round helpers MUST admit decisions only when the slot is not already decided, a proposed value is known, and the best existent and universal proposals match; divergent best proposals MUST carry forward the best common proposal instead of deciding.

r[consensus.quepaxa.concrete-phases]
QuePaxa concrete phase helpers MUST admit only named phases, prevent recorder catch-up from moving a proposer backward, allow catch-up to a higher recorder step with the recorder first value, and expose a phase-2 decision only when it refines the abstract decision guard.

r[consensus.quepaxa.fast-path]
QuePaxa fast-path helpers MUST admit only an initial-round declared-leader reserved-priority proposal first at quorum with a unique reserved priority and an abstract guard; non-leader, non-initial-round, missing-quorum, or non-unique reserved-priority attempts MUST be rejected.

r[consensus.quepaxa.hedging]
QuePaxa hedging admission MUST require non-decreasing activation delays, leader-first zero-delay placement when a leader is configured, and more scheduled proposers than the crash-fault budget, without claiming real-time recovery or probabilistic liveness.

r[consensus.quepaxa.fixtures]
QuePaxa validation MUST include positive and negative tests for boundary overclaims, proposal ordering, reserved leader priority, ISR transitions, propagation quorums, abstract decisions, concrete phase/catch-up guards, fast-path admission, and hedging schedules.

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

## Consensus Fast Path

r[consensus.fast-path.boundary]
Fast-path work MUST stay a finite crash-fault arithmetic, certificate, order,
conflict, recovery, convergence, and history proof surface. External protocol,
model-checking, benchmark, and capability overclaims MUST reject.

r[consensus.fast-path.quorum]
Named formulas over a fault budget MUST derive cluster size, majority, fast
quorum, recovery support, and failure tolerance. Valid inputs MUST give a
fast/recovery-majority intersection of at least the support threshold.

r[consensus.fast-path.certificate]
Certificates MUST have unique replica acknowledgements, one command, one
same-view cohort, equal acceleration and base views at commit, the fast
quorum, and every active proposer promise. Mixed or incomplete inputs reject.

r[consensus.fast-path.base-order]
Receive-to-propose, propose-to-log, and log-to-execute preservation MUST
compose into the execution-order promise. Reordering bases MUST move or reject
the fast acknowledgement boundary.

r[consensus.fast-path.conflict]
Non-conflict MUST preserve final state and each command's response under
both orders. Conservative classifiers MUST treat unknown effects as
conflicting; a false-negative witness MUST reject refinement.

r[consensus.fast-path.recovery]
Recovery MUST carry the accepted set across coordinator failure and MUST
admit normal new-view commands only after the recovery or no-op marker
commits. Recovered commands MUST precede conflicting new-view work.

r[consensus.fast-path.dual-path]
Fast and original convergence MUST apply one operation once and return the
cached response while the dedup record remains retained and identities match.

r[consensus.fast-path.linearization]
A conflicting later invocation MUST NOT execute before a validly responded
fast-committed operation. An adjacent non-conflicting swap MUST preserve state
and responses only under the application commutativity contract.

r[consensus.fast-path.fixtures]
Positive and negative fixtures MUST cover both replica profiles, certificate
gates, ordering capabilities, conflict classes, recovery transitions,
convergence identities, and history boundaries.

r[consensus.fast-path.evidence]
Every exported theorem MUST bind its requirement, proof function, source,
verifier, assumptions, tests, and non-claims before archive.

r[consensus.fast-path.non-claims]
The family MUST NOT claim external implementation correctness, arbitrary
application classifier soundness, network liveness, storage durability,
dynamic membership, Byzantine tolerance, transactions, performance, or whole-
system linearizability.

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

r[alloc-bitmap.clone-bool-slice]
clone_bool_slice(src) MUST return a vector with exactly the same boolean
sequence as the source slice.

r[alloc-bitmap.clone-u32-slice]
clone_u32_slice(src) MUST return a vector with exactly the same u32 sequence as
the source slice.

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

## Register Allocation

r[regalloc.interference]
The register-allocation core MUST build a symmetric interference graph with no
self-loops where two variables interfere exactly when their caller-supplied live
ranges overlap at some program point.

r[regalloc.coloring]
Any accepted allocation MUST assign distinct physical registers to interfering
variables, and every register assignment MUST use a register ID strictly less
than `k`.

r[regalloc.completeness]
Every variable MUST receive exactly one allocation outcome: either a physical
register or a spill slot. In this core model, spilled variables MUST use spill
slot `variable_index`.

r[regalloc.spill-fallback]
The greedy allocator MUST process variables in increasing variable-index order,
choose the lowest available register in `0 .. k-1`, and spill only when every
register in that range conflicts with an interfering earlier variable.

## WebAssembly MVP Primitives

r[wasm-value.types]
Core Wasm values MUST carry exact value-type information. A value tagged `i32`,
`i64`, `f32`, or `f64` MUST be accepted only when that exact type is required.

r[wasm-stack.discipline]
The modeled `i32.add` instruction MUST consume exactly two `i32` operands from
the operand-stack suffix and MUST produce exactly one `i32` result.

r[wasm-locals.access]
`local.get`, `local.set`, and `local.tee` MUST access only in-bounds local
indices and MUST preserve the declared type of each local slot.

r[wasm-memory.bounds]
A memory load or store MUST succeed only when every addressed byte lies within
linear-memory bounds. Out-of-bounds access MUST fail without modifying memory.

r[wasm-memory.grow]
`memory.grow` MUST increase linear memory only in whole 64KiB pages. On
success, old memory contents MUST remain unchanged as a prefix of the grown
memory and the appended bytes MUST be zero. On failure, memory MUST remain
unchanged.

r[wasm-control.labels]
Wasm control labels MUST distinguish function, block, and loop targets. Branch
target arity MUST use result types for function and block labels, and parameter
types for loop labels.

r[wasm-control.branch-arity]
A modeled branch MUST be admitted only when its depth names an in-bounds control
label and the operand-stack suffix exactly matches the target label types. Depth
0 MUST name the innermost label.

r[wasm-control.return-arity]
A modeled return MUST be admitted only when the control stack has a root
function label and the operand-stack suffix exactly matches that function
label's result types. Empty control stacks or non-function roots MUST be
rejected.

r[wasm-validation.unreachable-stack]
The Wasm validation model MUST track operand types, control labels, and current
reachability. Reachable input-pop checks MUST require an exact operand-type
suffix match, while unreachable input-pop checks MUST be admitted without that
suffix.

r[wasm-validation.i32-add]
The Wasm validation model MUST admit `i32.add` only when reachable state has two
trailing `i32` operand types, or when the current path is unreachable. On
success, validation MUST record one produced `i32` result type.

r[wasm-validation.control-transfer]
The Wasm validation model MUST validate `br` and `return` against the existing
control-label arity rules while the current path is reachable. Successful `br`,
`return`, and `unreachable` validation MUST mark the current path unreachable.
Malformed branch depths, missing function roots, and reachable suffix mismatches
MUST be rejected.

r[wasm-validation.control-frame]
The Wasm validation model MUST record structured control frames with label kind,
parameter types, result types, entry operand-stack height, and entry
reachability. Helpers that enter or end modeled blocks and loops MUST keep the
control-frame stack aligned with the branch-label stack.

r[wasm-validation.block-entry]
The Wasm validation model MUST admit block and loop entry only when reachable
state has a matching parameter suffix, or when the current path is unreachable.
Successful entry MUST remove consumed parameter types, push an aligned control
frame and branch label, and keep reachability unchanged.

r[wasm-validation.block-end]
The Wasm validation model MUST admit ending the innermost frame only when
reachable state has a result-type suffix matching that frame, or when the current
path is unreachable. Successful end MUST pop the frame and aligned label,
restore the operand-type stack to the frame entry height, append the frame result
types, and restore reachability to the frame's entry reachability.

r[wasm-table.signature]
The Wasm table model MUST represent a finite sequence of optional function
signatures. A table entry MUST be admitted for indirect-call use only when the
selected index is in bounds, initialized, and its parameter and result type
sequences exactly match the expected function type.

r[wasm-validation.call-indirect]
The Wasm validation model MUST admit modeled `call_indirect` only when
table-entry signature admission succeeds and, while reachable, the operand-type
stack suffix exactly matches the expected function parameters followed by an
`i32` table-index operand. Successful reachable validation MUST consume that
suffix and push the expected result types. Successful unreachable validation MUST
keep operand pops stack-polymorphic but still require a matching table entry and
synthesize the expected result types.

r[wasm-validation.br-table]
The Wasm validation model MUST admit modeled `br_table` only when the default
branch depth is in bounds, every listed branch depth is in bounds, every listed
target's branch-label types exactly match the default target's branch-label
types, and, while reachable, the operand-type stack suffix exactly matches the
default target types followed by an `i32` selector. Successful validation MUST
mark the current path unreachable. Unreachable validation MUST keep operand pops
stack-polymorphic but still reject malformed depths or mismatched target types.

## WebAssembly Runtime Bounds

r[wasm-runtime-bounds.model]
The runtime-bound model MUST use finite owned states, explicit host-observation
transition facts, and closed running, finished, trapped, paused, and fuel
outcomes. Admission MUST reject out-of-range or duplicate transition keys.

r[wasm-runtime-bounds.fuel]
Fuel consumption MUST subtract the exact admitted amount. Insufficient fuel and
zero-budget execution MUST not underflow or change the starting state.

r[wasm-runtime-bounds.segmentation]
For one admitted deterministic model and one unchanged observation sequence,
prefix-plus-suffix execution MUST match one run with the combined budget.
Terminal and explicit host-pause boundaries MUST absorb the unused suffix.

r[wasm-runtime-bounds.continuation]
A continuation MUST bind exact generation, artifact, profile, input, context,
and host-observation identities. Resume MUST reject any mismatch without
changing the continuation, and it MUST retain the pause or fuel reason.

r[wasm-runtime-bounds.allocation]
Page allocation MUST be monotone, capped, and initialization-only. Invalid
state, arithmetic overflow, capacity exhaustion, or a closed phase MUST return
an explicit error without changing the supplied allocator state.

r[wasm-runtime-bounds.streaming]
A chunk schedule MUST be an exact ordered partition of its declared byte range.
Admission MUST reject ambiguous empty input, zero lengths, gaps, overlap,
reordering, arithmetic overflow, extra ranges, and trailing or missing bytes.

r[wasm-runtime-bounds.growth-policy]
A memory-growth profile MUST explicitly allow or deny `memory.grow`. Denial
MUST preserve memory, while admission MUST delegate only to the existing
checked whole-page growth primitive and preserve its success and failure laws.

r[wasm-runtime-bounds.evidence]
Public runtime-bound decisions MUST have requirement links, positive and
negative tests, and direct Verus proof artifacts for their declared finite
laws.

r[wasm-runtime-bounds.non-claims]
Runtime-bound evidence MUST remain scoped to finite abstract laws. It MUST NOT
claim concrete interpreter, unsafe allocator, host scheduler, serialization,
migration, sandbox, certification, or downstream adapter correctness.

r[wasm-runtime-bounds.docs]
The component MUST have public exports, module guidance, requirement links,
proof evidence, and explicit consumer revalidation guidance.

r[wasm-runtime-bounds.final-validation]
Closeout MUST include focused tests, full-library Verus verification, Tracey
coverage, Cairn gates, accepted-spec synchronization, and archive evidence.

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

## Constant Folding

r[constfold.lattice]
Constant propagation MUST use a three-valued lattice with `Top`, `Const(c)`,
and `Bottom`, ordered so `Bottom <= Const(c) <= Top` and incomparable
constants meet at `Bottom`. The lattice meet MUST be commutative,
associative, and idempotent, with `Top` as identity and `Bottom` as
absorbing element.

r[constfold.finite-height]
The constant-propagation lattice MUST have height 2: there MUST NOT exist a
strict descending chain longer than `Top > Const(c) > Bottom`.

r[constfold.monotone]
Per-instruction transfer over the shared raw IR MUST be monotone under the
pointwise lattice order. `copy` MUST propagate an operand's current lattice
value. Supported binary ops MUST fold two constants, propagate `Bottom` if any
operand is `Bottom`, propagate `Top` when at least one operand is `Top` and no
operand is `Bottom`, and set malformed or unsupported definitions to
`Bottom`.

r[constfold.fold]
Compile-time folding MUST exactly match the modeled runtime semantics for the
supported raw opcodes: wrapping add/sub/mul, checked div-by-zero rejection,
bitwise and/or/xor, bounded shifts (`rhs < 64`), and comparisons encoded as
`0` or `1`.

r[constfold.convergence]
The current forward constant-propagation engine MUST process blocks in reverse
postorder, use the supplied entry environment for block 0, use pointwise meet
of predecessor out-environments for non-entry block inputs, and use a
conservative round budget of `2 × V × B + 1` for `V` variables and `B` blocks
while checking successive rounds for stability.

r[constfold.soundness]
For the current fixed-point soundness theorem, caller-supplied reachable entry
state MUST be modeled with a top-free entry environment: each entry slot is
`Const(c)` for an exact known value or `Bottom` for an unknown/overdefined
value, while `Top` is reserved for analysis-internal unreached rows. Along any
valid block path that starts at block `0` and does not later revisit block `0`,
every fixed-point `Const(c)` classification at block entry or block exit MUST
match the concrete value `c`, and `Bottom` MUST remain conservative.

## Instruction Selection

r[instruction-selection.well-formed-tree]
`is_well_formed_tree` MUST accept only expression trees whose node shapes match
the modeled operator contract and whose recursive `subtree_eval` succeeds:
constants have no children and a literal value, additions and multiplications
have exactly two children and no node literal, every child is well formed, and
checked arithmetic overflow makes the tree malformed for this surface.

r[instruction-selection.tiling-cost]
For a well-formed expression tree, `tiling_cost` MUST project the selected
`Tiling`'s `total_cost` without recomputing or mutating the tiling.

r[instruction-selection.subtree-eval]
`subtree_eval` MUST evaluate the modeled expression tree with checked `u64`
arithmetic: constants project their literal value, addition and multiplication
combine recursively evaluated operands, and malformed arity/value shapes or
arithmetic overflow return `None`.

r[instruction-selection.valid-tile-pattern]
`is_valid_tile_pattern` MUST reject a root wildcard tile, MUST accept wildcard
children as uncovered subtrees, and MUST recursively require every specific tile
node to satisfy the operator/literal/arity boundary: constants have no children
and a literal, while addition and multiplication have exactly two children and
no literal.

r[instruction-selection.tile-matches]
`tile_matches` MUST reject malformed trees and invalid tile patterns before
matching. For valid inputs it MUST return `None` when the root operator,
literal, or arity does not match; otherwise it MUST return uncovered roots in
preorder, using wildcard child patterns as uncovered subtree roots and recursing
through specific child patterns.

r[instruction-selection.optimal-tiling]
`optimal_tiling` MUST reject malformed trees and invalid tile lists, report the
lexicographically-first uncovered root when no complete tiling exists, and for
complete tilings choose the lowest total cost with deterministic tie-breaking by
tile rank and then tile index.

r[instruction-selection.coverage-check]
`check_coverage` MUST recompute each selection's covered paths, reject stale or
tampered selection coverage, reject duplicate coverage, and return true exactly
when the selected coverage is a sorted complete cover of the tree's node paths.

r[instruction-selection.soundness-check]
`check_soundness` MUST reject malformed trees and invalid tile lists, evaluate
the selected tiles according to their semantics with checked arithmetic, and
return true only when the selected tiling's value equals `subtree_eval` for the
whole tree.

## Stack Frame Layout

r[stack-frame.non-overlap]
Stack slot layouts MUST keep simultaneously-live locals or spills in disjoint
half-open byte ranges. Two locals or spills whose live ranges do not overlap MAY
reuse the same byte range.

r[stack-frame.alignment]
Every stack slot MUST start at an offset divisible by its declared alignment,
and every valid frame MUST keep the final frame size divisible by the stack
alignment.

r[stack-frame.frame-size]
The frame size MUST equal the stack-aligned rounding of the maximum slot end in
the layout. Invalid or misaligned slot inputs MUST be rejected instead of being
silently rounded.

r[stack-frame.growth]
Every slot end MUST lie within the computed frame boundary, and append-only slot
assignment MUST place each new slot at or after the current maximum slot
extent.

## Typed Configuration Normal Form

r[domain-primitives.typed-config.expression-envelope]
A finite node graph MUST declare each node, child, reference, root, and import
index, and MUST reject duplicates, dangling edges, cycles, or unsupported forms.

r[domain-primitives.typed-config.type-admission]
Each node kind MUST match its declared type. Required record fields MUST exist
and their child types MUST match before normalization.

r[domain-primitives.typed-config.explicit-failure]
Missing, malformed, unsupported, cyclic, policy-denied, and digest-mismatched
inputs MUST return deterministic table, index, and subject metadata.

r[domain-primitives.typed-config.normal-form]
Normal form MUST sort nodes by key and record fields by name, preserve list
order, canonicalize variants and options, and omit source labels and comments.

r[domain-primitives.typed-config.semantic-digest]
Digest input selection MUST contain only admitted normal-form metadata and the
ordered declared dependency digests. This core MUST NOT compute a hash.

r[domain-primitives.typed-config.import-closure]
Imports MUST be static and frozen. Remote imports MUST not reach local, home, or
environment targets, and headers MUST forward only to same-origin relatives.

r[domain-primitives.typed-config.semantic-refactoring]
Equal digest inputs MUST admit a source refactor. Changed inputs MUST require an
explicit compatibility or unresolved-dependent record.

r[domain-primitives.typed-config.positive]
Positive tests MUST cover typed records, canonical field order, source-only
refactors, stable digest input, frozen imports, and same-origin headers.

r[domain-primitives.typed-config.negative]
Negative tests MUST cover type and field failures, unsupported forms, graph
failures, denied imports, digest drift, and unauthorized refactors.

r[domain-primitives.typed-config.docs]
Public normal-form helpers MUST have matching requirement, source, test or proof,
README, module-guide, accepted-spec, and lifecycle references.

r[domain-primitives.typed-config.final-validation]
Final evidence MUST record focused Cargo and Verus results, Tracey coverage,
strict Clippy, repository checks, and Cairn lifecycle validation.

r[domain-primitives.typed-config.non-claims]
Normal-form evidence MUST NOT claim Dhall compatibility, Nickel evaluation,
parsing, import execution, network or filesystem security, or hash correctness.

## Fenced Attempt Admission

r[fenced-attempt-admission.boundary]
The closed model MUST cover only finite phase, report, epoch, event, scope,
linkage, count, and identity-relation facts. Unsupported variants MUST reject.

r[fenced-attempt-admission.refinement]
Spec and executable decisions for epoch and event classification, phase steps,
result linkage, report application, and next-epoch arithmetic MUST agree, and
applied reports MUST change only their named fields.

r[fenced-attempt-admission.fence-safety]
Stale, future, wrong-job, and wrong-run reports MUST reject unchanged.
Reassignment MUST produce a strictly greater nonzero epoch and MUST reject at
the fixed-width maximum.

r[fenced-attempt-admission.event-idempotence]
A repeated event and payload identity MUST replay as a no-op. An event identity
with a different payload MUST reject as a conflict without merging either.

r[fenced-attempt-admission.terminal-closure]
Completed, failed, and superseded phases MUST reject every later
state-changing report unchanged.

r[fenced-attempt-admission.completion-linkage]
Completion MUST require result-ready phase, current identity and epoch, worker
scope, output scope, and the exact retained result identity.

r[fenced-attempt-admission.rejection-preservation]
Every rejection and replay MUST preserve phase, epoch, event history, progress,
result linkage, and completion facts field for field.

r[fenced-attempt-admission.evidence]
Focused tests, bounded random histories, single-fact mutations, direct proof
functions, Tracey coverage, and full-library Verus runs MUST bind each theorem
to its requirement with explicit non-claims.

## S-expression Primitives

r[s-expression.ast]
Each admitted finite tree MUST be a canonical atom or an ordered list whose
children are all admitted trees.

r[s-expression.atom-admission]
Symbols MUST be non-empty and use only the declared ASCII class. Quoted text
MUST reject unsupported controls and normalize declared escapes.

r[s-expression.canonical-serialization]
Serialization MUST use one atom spelling, parentheses around lists, one space
between children, ordered children, and canonical text escapes.

r[s-expression.parse-admission]
Parsing MUST accept only the canonical grammar and return deterministic byte
offsets for atom, escape, delimiter, spacing, trailing-input, and budget errors.

r[s-expression.roundtrip]
Every admitted bounded value MUST return unchanged through canonical
serialization and canonical parsing, with stable re-serialization.

r[s-expression.budgets]
Node, depth, and UTF-8 byte budgets MUST be explicit and no larger than the
named core maxima. Exhaustion MUST fail before an identity claim.

r[s-expression.evidence.positive]
Positive tests MUST cover symbols, escaped text, empty and nested lists,
measures, canonical rendering, parsing, Unicode text, and roundtrip checks.

r[s-expression.evidence.negative]
Negative tests MUST cover invalid atoms and escapes, delimiters, spacing,
trailing input, malformed nesting, invalid budgets, and each budget exhaustion.

r[s-expression.docs]
Public canonical-tree helpers MUST have matching requirement, source, test or
proof, README, module-guide, accepted-spec, and lifecycle references.

r[s-expression.final-validation]
Final evidence MUST record focused Cargo and Verus results, Tracey coverage,
strict Clippy, repository checks, and Cairn lifecycle validation.

r[s-expression.non-claims]
Canonical-tree evidence MUST NOT claim evaluation, macros, hygiene, interning,
modules, bytecode, effects, or compatibility with an external Lisp dialect.

## Scheme Logic Primitives

r[scheme-logic.reader-token-span]
Reader tokens and parent-child spans MUST use ordered half-open byte ranges
inside one declared finite source length. Invalid, overlapping, or escaping
ranges MUST reject with deterministic metadata.

r[scheme-logic.reader-datum-structure]
Supported delimiter kinds MUST pair exactly once without crossing. Quote-like
and datum-comment prefixes MUST attach to the following datum token explicitly.

r[scheme-logic.interning]
Symbol and literal interning MUST use explicit bounded state with dense unique
keys and unique values. Existing values MUST retain their keys, and returned
keys MUST resolve to the original values.

r[scheme-logic.module-graph]
Finite module graphs MUST use unique module and export identities. Every import
MUST name a declared provider export, and transitive closure MUST use an
explicit visited set and expansion budget.

r[scheme-logic.contract-blame]
Contract metadata MUST distinguish flat and function descriptors, check callable
arity, preserve matching wrapper composition, and attribute domain failures to
callers and codomain failures to providers.

r[scheme-logic.bytecode-safety]
Constant pools MUST deduplicate values and preserve retained rollback prefixes.
Bytecode admission MUST reject invalid operands, stack underflow, arity errors,
and incorrect final stack summaries before execution.

r[scheme-logic.value-graph]
Pair, vector, symbol, scalar, empty-list, and procedure-placeholder nodes MUST
form a finite graph with in-range edges. Display and equality traversal MUST be
iterative, cycle-safe, and budgeted.

r[scheme-logic.collections-bytevectors]
Proper lists MUST terminate at the declared empty-list node. Immutable vector
updates MUST preserve length, hash keys and set members MUST be unique, and
bytevector elements MUST remain in the byte domain.

r[scheme-logic.evidence.positive]
Positive tests MUST cover every reader, interner, module, contract, bytecode,
value-graph, list, vector, hash, and bytevector family that the module exports.

r[scheme-logic.evidence.negative]
Negative tests MUST reject malformed spans, delimiters, intern tables, imports,
contracts, bytecode, graphs, lists, hashes, and bytevectors without panicking or
silently accepting invalid state.

r[scheme-logic.docs]
The public module MUST have requirement, source, test or proof, README,
module-guide, accepted-spec, proof-evidence, and lifecycle references.

r[scheme-logic.final-validation]
Closeout MUST include focused tests, strict Clippy, Verus verification, Tracey
coverage, Octet, Cairn gates, accepted-spec synchronization, and archive
evidence.

r[scheme-logic.non-claims]
Scheme-logic evidence MUST remain limited to Trellis-owned finite predicates. It
MUST NOT claim Steel compatibility, Scheme conformance, parser or macro
soundness, GC, JIT, FFI, module-loading, effect, or interpreter correctness.

## HIR Attribute Parsing

r[hir-attr.parser-envelope]
An inert parser descriptor MUST name its parser and template. It MUST contain a
supported non-empty target mask and complete class and policy metadata.

r[hir-attr.parser-classes]
No-args, single, combine, and generic parser classes MUST enforce their declared
argument shape and duplicate behavior before producing parsed metadata.

r[hir-attr.policy]
Every admitted occurrence MUST satisfy the descriptor's target, duplicate,
safety-wrapper, and stability-gate policies.

r[hir-attr.admission]
Admission MUST reject unknown parsers, invalid descriptors, malformed arguments,
name mismatch, bad targets, duplicates, missing wrappers, and gate mismatch with
deterministic reason and input-index metadata.

r[hir-attr.early-late-agreement]
Successful limited-early and later-HIR results MUST use the expected phase order
and contain equal finite parsed metadata. Failed results are not comparable.

r[hir-attr.evidence]
Positive and negative tests MUST cover all parser classes, descriptor failures,
policy checks, deterministic diagnostics, projection, and phase agreement.

r[hir-attr.non-claims]
The finite model MUST NOT claim active expansion, proc-macro behavior, full token
parsing, rendered diagnostics, feature stabilization, or whole-frontend
correctness.

## HIR ID Lowering

r[hir-id.owner-scope]
Every admitted HIR identifier MUST carry the active owner. A different owner
MUST fail before lowering facts become available.

r[hir-id.lowering-map]
Every declared source node MUST have exactly one source-to-HIR mapping, and each
mapping target MUST be unique in the active owner.

r[hir-id.admission]
Every mapping and generated target MUST name one created ID. Required IDs MUST
be used once, while explicit discarded IDs MUST remain unused.

r[hir-id.fresh-generated]
Generated targets MUST differ from all source-mapped targets and prior generated
targets, with unique generation provenance in the active owner.

r[hir-id.uniqueness]
Source mappings, generated nodes, created IDs, and used IDs MUST reject duplicate
HIR structure identities deterministically.

r[hir-id.def-correspondence]
Each definition-introducing source node MUST have one unique finite definition
correspondence. Unrequested or missing correspondences MUST fail.

r[hir-id.evidence]
Positive and negative tests MUST cover owner scope, mappings, created-ID use,
generated freshness, uniqueness, stale IDs, and definition metadata.

r[hir-id.docs]
Public HIR ID helpers MUST have matching requirement, source, test or proof,
README, module-guide, accepted-spec, and lifecycle references.

r[hir-id.final-validation]
Final evidence MUST record focused Cargo and Verus results, Tracey coverage,
strict Clippy, repository checks, and Cairn lifecycle validation.

r[hir-id.non-claims]
HIR ID evidence MUST NOT claim AST or HIR semantic equivalence, parsing, macro
expansion, name resolution, THIR construction, typing, or frontend correctness.

## MIR Core Primitives

r[mir-core.body-shape]
A finite MIR body MUST carry local and block tables that supply every width
needed for structural validation without reading compiler state.

r[mir-core.return-place]
The named `RETURN_PLACE` local MUST exist at the distinguished return index
before admission checks any statement or terminator reference.

r[mir-core.block-terminator]
Each admitted basic block MUST contain zero or more statements and exactly one
final terminator.

r[mir-core.local-reference]
Every place, projection, operand, statement, and terminator local reference
MUST name an in-range local declaration.

r[mir-core.place-projection]
An admitted place MUST have an in-range base and an ordered finite projection
chain. Unsupported projection markers MUST fail admission.

r[mir-core.rvalue-nonnesting]
Admitted rvalues MUST contain only operands or places. A raw nested-rvalue
candidate MUST fail before local transfer facts become available.

r[mir-core.successors]
Every admitted terminator successor and every extracted CFG edge MUST target an
in-range block in the same body.

r[mir-core.failure-metadata]
Admission MUST return the same first failure class and body-relative location
for repeated checks of the same malformed body.

r[mir-core.downstream-adapter]
The adapter MUST validate the body before it exposes local count, block count,
or ordered CFG edges to downstream analyses.

r[mir-core.evidence]
Positive and negative tests MUST cover every public MIR-core admission family,
including malformed tables, references, projections, rvalues, and successors.

r[mir-core.docs]
Public MIR-core helpers MUST have matching requirement, source, test or proof,
README, module-guide, accepted-spec, and lifecycle references.

r[mir-core.final-validation]
Final evidence MUST record focused Cargo and Verus results, Tracey coverage,
strict Clippy, repository checks, and Cairn lifecycle validation.

r[mir-core.non-claims]
MIR-core evidence MUST NOT claim full rustc MIR compatibility, Rust typing,
borrow checking, drop elaboration, optimization, code generation, text parsing,
or whole-compiler correctness.

## MIR Dataflow Framework

r[mir-dataflow.domain]
A finite domain MUST declare bounded states, row width, bottom, top, and height.
Malformed or missing bounds MUST fail before analysis.

r[mir-dataflow.lattice-laws]
The finite join table MUST satisfy idempotence, commutativity, associativity,
bottom identity, and top absorption for every declared state.

r[mir-dataflow.direction]
Forward analyses MUST compose locations in declaration order. Backward analyses
MUST reverse location order while keeping each before effect before its primary.

r[mir-dataflow.effects]
Each statement or terminator location MUST have total in-range before and primary
transfer tables, applied in that order.

r[mir-dataflow.per-edge]
Each edge-local transfer MUST name one admitted CFG edge and remain unique for
that source-target pair.

r[mir-dataflow.admission]
Every transfer MUST be monotone under the admitted join order. Invalid domains,
locations, edges, transfers, boundaries, and budgets MUST fail deterministically.

r[mir-dataflow.convergence]
Fixed-point iteration MUST stop only at stable block equations within a named
height-and-work-item budget. Exhaustion MUST withhold results.

r[mir-dataflow.results]
Fixed results MUST expose pure block-entry, block-exit, before, primary, edge,
and exactly-once requested-location inspection.

r[mir-dataflow.evidence]
Positive and negative tests MUST cover domains, both directions, ordered effects,
edge effects, convergence, cursors, and malformed finite tables.

r[mir-dataflow.docs]
Public analysis helpers MUST have matching requirement, source, test or proof,
README, module-guide, accepted-spec, and lifecycle references.

r[mir-dataflow.final-validation]
Final evidence MUST record focused Cargo and Verus results, Tracey coverage,
strict Clippy, repository checks, and Cairn lifecycle validation.

r[mir-dataflow.non-claims]
Framework evidence MUST NOT claim soundness for a borrow, move, initializedness,
or other concrete Rust analysis, compiler engine, or pass.

## MIR Destruction Decisions

r[mir-drop.obligations]
Each admitted structural path MUST have declared destruction metadata.
Initialization MUST establish, and moves MUST release, subtree obligations.

r[mir-drop.initializedness]
Initialization, move, and overwrite actions MUST update finite subtree
maybe-initialized, maybe-uninitialized, and obligation sets deterministically.

r[mir-drop.classification]
Finite maybe facts MUST classify a target as static, dead, conditional, or open.
Absent facts MUST fail instead of selecting a default class.

r[mir-drop.admission]
Body places, parent links, depths, metadata rows, child paths, state facts, and
obligations MUST be admitted before a decision becomes available.

r[mir-drop.drop-flags]
Conditional targets MUST emit guarded metadata with a deterministic flag. Dead
or no-glue targets MUST emit no call metadata.

r[mir-drop.open-drop]
Open field and active-enum paths MUST expand in declared order within an explicit
step budget, skipping dead and inactive paths.

r[mir-drop.drop-and-replace]
All old-target call or guarded metadata MUST precede exactly one replacement
write. An invalid target MUST fail before that write.

r[mir-drop.evidence]
Positive and negative tests MUST cover obligations, transitions, four classes,
guards, open struct/enum expansion, replacement, paths, state, and budgets.

r[mir-drop.docs]
Public destruction helpers MUST have matching requirement, source, test or proof,
README, module-guide, accepted-spec, and lifecycle references.

r[mir-drop.final-validation]
Final evidence MUST record focused Cargo and Verus results, Tracey coverage,
strict Clippy, repository checks, and Cairn lifecycle validation.

r[mir-drop.non-claims]
Decision metadata MUST NOT claim destructor effects, unwind cleanup, rustc shim
behavior, const-eval policy, unsafe-code soundness, or memory safety.

## MIR Visitor Traversal

r[mir-visitor.locations]
Every visit MUST name an admitted body, local, block, statement, terminator,
place, operand, rvalue, or successor location with fixed-width indices.

r[mir-visitor.trace]
A finite trace MUST declare its ordered entries, entry block, coverage mode,
reachability policy, block order, and mutation class.

r[mir-visitor.coverage]
An exactly-once trace MUST contain every requested location once, with no
duplicate, invalid, unreachable, or outside location.

r[mir-visitor.traversal-order]
Preorder and reverse postorder MUST start at the declared entry, follow admitted
CFG edges deterministically, and exclude unreachable blocks.

r[mir-visitor.mut-shape]
An immutable trace MUST have no changed body. A value-only mutable trace MUST
keep admitted local, block, statement, terminator, and successor counts.

r[mir-visitor.admission]
Admission MUST reject the same first invalid mode, mutation, request, entry,
coverage, reachability, or order condition for the same malformed trace.

r[mir-visitor.evidence]
Positive and negative tests MUST cover location families, full coverage,
duplicates, missing visits, traversal orders, reachability, and mutation shape.

r[mir-visitor.docs]
Public traversal helpers MUST have matching requirement, source, test or proof,
README, module-guide, accepted-spec, and lifecycle references.

r[mir-visitor.final-validation]
Final evidence MUST record focused Cargo and Verus results, Tracey coverage,
strict Clippy, repository checks, and Cairn lifecycle validation.

r[mir-visitor.non-claims]
Traversal evidence MUST NOT claim rustc visitor-macro correctness, compiler
borrow behavior, pass semantics, optimization correctness, or whole-compiler
correctness.

## MIR Pass Pipeline

r[mir-pass.phase-order]
A finite total order MUST admit only same-phase pass mutation or the next
abstract MIR dialect. It MUST reject phase reversal and skipped phases.

r[mir-pass.artifact-state]
Each current MIR artifact MUST have available, borrowed, or stolen state.
A stolen artifact MUST NOT be read or validated.

r[mir-pass.pass-compatibility]
Every pass MUST match its reviewed registration, current input phase, output
phase, mutation shape, structural effect, validation gate, and optimization gate.

r[mir-pass.admission]
The bounded trace helper MUST accept only traces whose phase, pass, artifact,
validation, optimization, and steal obligations all hold.

r[mir-pass.steal-order]
A steal that requires a dependent read MUST follow a borrow and recorded read.
Reads after steal, unresolved borrows, and repeated steals MUST be rejected.

r[mir-pass.optimization-gating]
An optional pass MUST run only when the selected finite optimization level meets
its registered minimum level.

r[mir-pass.failure-metadata]
Admission MUST return the same first failure class and zero-based step index for
the same malformed trace.

r[mir-pass.evidence]
Positive and negative tests MUST cover phase order, same-phase mutation,
validation, optimization gates, bounded traces, and borrow/read/steal ordering.

r[mir-pass.non-claims]
The finite model MUST NOT claim rustc query execution, incremental correctness,
pass semantics, optimization soundness, borrow-checker correctness, or
whole-compiler correctness.

## Backend Codegen Trace

r[backend-trace.handles]
Every block, function, type, and place use MUST name a declared typed handle.
Value uses MUST follow a unique prior value definition.

r[backend-trace.block-mapping]
Every admitted MIR block MUST have one direct declared builder block. Expansion
blocks MUST be declared, unique, and assigned to that MIR block explicitly.

r[backend-trace.lowering-trace]
Each finite operation MUST match an admitted MIR statement, operand, place,
rvalue, or terminator location and an allowed emission block.

r[backend-trace.handle-validity]
Value, function, type, and place handles MUST be valid at their operation index.
Dangling and repeated value definitions MUST fail deterministically.

r[backend-trace.terminators]
Each MIR block MUST finish once with branch, call, return, or unreachable
metadata. Control targets MUST map a MIR successor or declared expansion.

r[backend-trace.ssa-like-locals]
Every MIR local MUST have exactly one direct-value or memory-backed class.
Conflicting, missing, and out-of-range classifications MUST fail.

r[backend-trace.admission]
Admission MUST report the same first declaration, mapping, local, source,
operation-order, handle, control-target, or terminator failure.

r[backend-trace.evidence]
Positive and negative tests MUST cover direct and expanded blocks, operation
families, handle order, control flow, terminators, and local classes.

r[backend-trace.docs]
Public builder-record helpers MUST have matching requirement, source, test or
proof, README, module-guide, accepted-spec, and lifecycle references.

r[backend-trace.final-validation]
Final evidence MUST record focused Cargo and Verus results, Tracey coverage,
strict Clippy, repository checks, and Cairn lifecycle validation.

r[backend-trace.non-claims]
Builder-record evidence MUST NOT claim concrete backend IR, target ABI, machine
code, linker, LTO, debug-information, or performance correctness.

## Rustc Monomorphization

r[mono.item-envelope]
Each finite function, static, or shim item MUST have a unique key, compatible
concrete shape, bounded arguments, optional shim source, origin, and stability.

r[mono.collection]
Collection MUST equal deterministic finite reachability from declared roots
through references whose sources and targets are declared items.

r[mono.dedup]
Repeated roots and reachability paths MUST collapse to one collected key.
Duplicate collected output keys MUST fail deterministically.

r[mono.cgu-classification]
Local plain functions and statics MUST be stable. Generic, specialized, shim,
or external items MUST be volatile.

r[mono.cgu-partition]
Output units MUST be disjoint and complete over the collected set, and each
unit's stability MUST match every item it contains.

r[mono.admission]
Admission MUST report the same first item, root, reference, collected-output,
or unit failure for the same malformed record.

r[mono.evidence]
Positive and negative tests MUST cover concrete items, reachable collection,
deduplication, stability, partition overlap, omission, and malformed metadata.

r[mono.docs]
Public collection helpers MUST have matching requirement, source, test or proof,
README, module-guide, accepted-spec, and lifecycle references.

r[mono.final-validation]
Final evidence MUST record focused Cargo and Verus results, Tracey coverage,
strict Clippy, repository checks, and Cairn lifecycle validation.

r[mono.non-claims]
Collection evidence MUST NOT claim trait solving, full substitution, rustc
instance resolution, code generation, linking, LTO, size, or performance facts.

## Rustc Metadata Identity

r[rustc-metadata.library-kind]
Rlib and dylib candidates MUST declare metadata and linkable code. Rmeta
candidates MUST declare metadata without claiming linkable object code.

r[rustc-metadata.identity-fields]
Compatibility MUST compare the declared crate name, compiler version, strict
version hash, and stable crate ID without changing rustc-mandated algorithms.

r[rustc-metadata.compatibility]
A candidate MUST satisfy artifact shape, request completeness, identity fields,
usage-mode linkability, and encoded-MIR requirements before admission.

r[rustc-metadata.admission]
Admission MUST return deterministic reason and candidate metadata for missing
fields, invalid capabilities, incompatibility, linkability, and encoded-MIR
failures.

r[rustc-metadata.dependency-loading]
Dependency selection MUST scan no more than `MAX_CANDIDATES` artifacts. It MUST
select one unique compatible candidate or report no match, ambiguity, or a bound
failure with deterministic candidate identities.

r[rustc-metadata.rmeta-pipelining]
Rmeta MAY satisfy check, documentation, pipeline, or compatible codegen metadata
use. Rmeta alone MUST NOT satisfy final linking.

r[rustc-metadata.evidence]
Positive and negative tests MUST cover all library kinds, identity mismatches,
encoded-MIR policy, rmeta boundaries, unique selection, no match, ambiguity, and
candidate bounds.

r[rustc-metadata.non-claims]
The finite model MUST NOT claim archive parsing, rustc hash construction or
collision resistance, linking, object code, LTO, compression, or whole-build
correctness.

## Track Caller ABI

r[track-caller.attribute-validity]
Tracked functions MUST use the Rust calling convention. They MUST NOT be
closures, naked functions, or unsupported item kinds.

r[track-caller.location-metadata]
Caller locations MUST use finite metadata that distinguishes definition sites,
call sites, and forwarded locations.

r[track-caller.direct-call]
A direct call to a tracked function MUST supply the location selected from its
explicit caller context. An untracked callee MUST NOT receive a hidden location.

r[track-caller.propagation]
A non-empty tracked-call chain MUST preserve the nearest untracked call-site
identity. Missing, changed, or excessive chain entries MUST fail admission.

r[track-caller.reify-shim]
A tracked function-pointer use MUST name a matching reify shim with a declared
definition-site fallback. An untracked function-pointer use MUST NOT add a shim.

r[track-caller.trait-inheritance]
Prototype, default-body, and local annotations MUST produce one deterministic
effective trait-method tracking result. Observed metadata MUST match that result.

r[track-caller.const-codegen-agreement]
Const and codegen traces MUST contain the same bounded ordered edge and location
metadata. A length, edge, or location mismatch MUST fail agreement.

r[track-caller.admission]
Admission helpers MUST use deterministic failure classes and subject identities.
Repeated evaluation of the same metadata MUST return the same first failure.

r[track-caller.evidence]
Positive and negative tests MUST cover attribute, direct-call, chain, shim,
trait-inheritance, agreement, bound, and deterministic-failure behavior.

r[track-caller.non-claims]
The finite metadata model MUST NOT claim stack walking, panic formatting,
runtime location allocation, ABI lowering, intrinsic behavior, debug-information
correctness, or whole-codegen correctness.

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

r[web-of-trust.principal-digest-admission]
Raw principal identifier digest bytes MUST be admitted as a `PrincipalId` only
when they are exactly 32 bytes long. Shorter or longer byte vectors MUST be
rejected before any principal-ID equality or chain logic treats them as
well-formed identifiers.

r[web-of-trust.attestation-signature]
An attestation signature MUST authenticate the issuer's canonical public-key
bytes, the subject's canonical public-key bytes, trust role, validity window,
and `sequence_no` exactly, using the canonical attestation payload-byte
grammar. Changing any signed field MUST invalidate the signature, and inputs
whose canonical byte-string fields exceed `u32::MAX` bytes MUST be rejected as
non-canonical.

r[web-of-trust.attestation-window]
An attestation MUST be active at time `t` iff its signature is valid, `t` is
inside the half-open window `[not_before, not_after)`, and no effective
revocation targets it at or before `t`.

r[web-of-trust.revocation]
A revocation record MUST be authenticated over its issuer, target reference,
and effective time using the canonical revocation payload-byte grammar.
Tampering with issuer, target reference, or effective time MUST invalidate
verification, and inputs whose canonical byte-string fields exceed `u32::MAX`
bytes MUST be rejected as non-canonical. A revocation record MUST be issued
by the same principal as the targeted attestation or targeted attestation
family. A revocation record MUST carry either an exact-payload target
reference or a family target reference. An exact-payload target identifies
attestation content by canonical attestation payload bytes; if multiple
records carry identical canonical attestation payload bytes, they are the same
exact-payload target and the revocation disables all such copies. A family
target MUST disable exactly the attestations in the named `(issuer, subject,
role)` family. Once effective, the revocation MUST remain effective for all
later times.

r[web-of-trust.chain-validity]
A trust chain MUST be valid only when every attestation in the chain is an
active `trust` edge at the evaluation time, the first issuer is a trust root,
each attestation's subject is the next attestation's issuer, and no principal
repeats in the chain.

r[web-of-trust.rooted-resolution]
For a fixed trust-root set `Roots` and fixed evaluation time `t`, a
rooted-trust decision for subject `S` under natural-number depth bound `d`
MUST return trusted iff either `S` is itself in `Roots` and the witness is the
empty chain, or there exists a valid trust chain at time `t` from some root in
`Roots` to `S` of length at most `d`.

r[properties.web-of-trust.revocation-cuts-trust]
For fixed trust roots and fixed evaluation times, if a successful
rooted-trust witness chain `C` for subject `S` includes attestation `A`, then
once `A` is effectively revoked, that same witness chain `C` MUST no longer
justify trust for `S` at later times. This requirement is witness-local and
does not forbid a different valid chain from still trusting `S`.

r[properties.web-of-trust.depth-monotone]
For a fixed trust-root set and fixed evaluation time, if a subject resolves as
trusted under depth bound `d`, then it MUST also resolve as trusted under
every depth bound `d' >= d`.

r[web-of-trust.threshold]
A threshold policy is parameterized by a natural `k`, an evaluation time `t`,
and a natural-number depth bound `d`. The policy MUST reject when `k = 0`.
For `k > 0`, it MUST accept subject `S` iff there exist at least `k` distinct
introducers `I` such that `d > 0`, `I` itself resolves as trusted at time `t`
under bound `d - 1`, `I` has an active direct trust edge to `S` at time `t`,
and appending that final direct edge yields a valid simple trust chain to `S`,
so repeated-principal final hops MUST NOT count. Multiple active attestations
from the same introducer to the same subject MUST contribute at most one
count. When `d = 0`, the threshold policy MUST reject every subject because no
final trust-edge hop fits inside the bound. Threshold policy does not
special-case root subjects; a root subject is accepted only if the same
`k`-introducer rule is met.

r[properties.web-of-trust.threshold-dedup]
For fixed trust roots, evaluation time, depth bound, threshold `k`, and
subject `S`, once one active direct trust edge from introducer `I` to `S` has
already been counted, any later active direct trust edge from that same
introducer to `S` MUST leave both the remaining qualifying distinct-introducer
count and the final threshold accept/reject result unchanged. Equivalently,
replacing multiple such same-introducer final hops with a single counted edge
MUST not change the threshold outcome.

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

## seL4 Capability Derivation Tree

r[sel4-cdt.structure]
Capability derivation MUST form an acyclic parent-child relation over live
capabilities. Copy and mint create a new live child of the source capability,
and the source capability remains live.

r[sel4-cdt.move]
Moving a capability to a different slot MUST preserve its derivation ancestry
and descendant set.

r[sel4-cdt.revoke]
Revoking a capability MUST remove exactly the live target subtree,
including the target root itself, while preserving siblings, ancestors
outside that subtree, and unrelated branches.

r[sel4-cdt.recycle]
Recycling a capability MUST succeed only for a live leaf capability and MUST
clear exactly that capability while preserving all other live nodes.

## seL4 Endpoint IPC

r[sel4-endpoint.rendezvous]
When a sender and receiver are both available on the same endpoint, the
kernel MUST match exactly one sender with exactly one receiver and wake both
threads. Public send/receive helpers may require caller TCB IDs that are not
already queued on that endpoint.

r[sel4-endpoint.queue-discipline]
Unmatched waiters on an endpoint MUST preserve queue order under match and
cancellation. Cancelling a blocked sender or blocked receiver MUST remove
only that waiter from its queue.

r[sel4-endpoint.badge]
A successful endpoint send/receive rendezvous MUST deliver the matched
sender capability's badge to the matched receiver.

## seL4 Fault IPC

r[sel4-fault-ipc.preservation]
Encoding a structured fault into an IPC-visible message and then decoding it
MUST recover the same fault class and preserved payload fields.

r[sel4-fault-ipc.delivery]
A modeled fault delivery step MUST place the encoded fault information into
the configured handler context so the handler can recover the original fault
record.

r[sel4-fault-ipc.payload]
Fault payload fields preserved by the model, including the instruction
pointer, MUST remain unmodified across encode/decode and delivery.

## seL4 Notification

r[sel4-notification.pending-badge]
A notification object MUST accumulate undelivered badge bits by bitwise OR of
all delivered signals that have not yet been consumed.

r[sel4-notification.consume]
Waiting or polling a signaled notification MUST return exactly the pending
badge value being delivered and consume that delivered pending state.

r[sel4-notification.binding]
If a notification is bound to a TCB, delivery eligibility and wakeup MUST
respect that binding.

## seL4 Reply Capability

r[sel4-reply.one-shot]
A reply capability created by a call MUST authorize at most one successful
reply. After reply, the reply capability MUST NOT be reusable.

r[sel4-reply.pairing]
A live reply capability MUST remain associated with the caller and callee pair
that created it. It MUST NOT be used to reply to a different caller.

r[sel4-reply.invalidate]
Revocation or cancellation MUST invalidate a live reply capability so no later
reply may succeed through it.

## seL4 IRQ Control

r[sel4-irq.binding]
Each IRQ line MUST have at most one live handler binding at a time.

r[sel4-irq.masking]
An IRQ that is masked MUST NOT be deliverable even if it is pending.

r[sel4-irq.ack]
Acknowledgement MUST clear or consume the abstract pending delivery state for
the IRQ being acknowledged.

## seL4 TCB States

r[sel4-tcb.state]
A TCB MUST be in exactly one legal state: runnable, running, blocked on send,
blocked on receive, blocked on notification, blocked on reply, restart, or
inactive.

r[sel4-tcb.blocking-object]
Every blocked TCB state MUST identify the kernel object responsible for the
block.

r[sel4-tcb.transitions]
Block, wake, restart, and deactivate operations MUST move a TCB only along
allowed state transitions.

## seL4 Ready Queues

r[sel4-ready-queue.consistency]
A runnable TCB MUST appear in exactly one ready queue at its runnable priority,
and a TCB present in a ready queue MUST be runnable.

r[sel4-ready-queue.selection]
Scheduler selection MUST choose a runnable TCB from a highest-priority
non-empty ready queue.

r[sel4-ready-queue.order]
Within one priority queue, enqueue, dequeue, and remove operations MUST
preserve the relative order of unaffected TCBs.

## seL4 VSpace Mappings

r[sel4-vspace.map]
Installing a mapping into a VSpace MUST respect object alignment and MUST NOT
create overlapping live mappings in that VSpace.

r[sel4-vspace.unmap]
Unmapping MUST remove exactly the targeted mapping and leave unrelated
mappings unchanged.

r[sel4-vspace.rights]
Effective mapping rights MUST be no greater than the rights carried by the
capability used to install the mapping.

## TLB Shootdown

r[tlb-shootdown.freshness]
After a single-page shootdown completes, every targeted core MUST have the
stale cached translation removed for the target page.

r[tlb-shootdown.termination]
Under the responsiveness assumption, acknowledgments MUST make monotone
progress toward completion and completion MUST hold exactly when all
required acknowledgments have been recorded.

r[tlb-shootdown.initiator-safety]
The shootdown protocol MUST report completion only after every required
acknowledgment has been recorded, so callers may use completion as the guard
for later effects outside this module.

r[tlb-shootdown.self-invalidation]
The initiating core MUST invalidate its own cached translation for the target
page before waiting on other cores.

r[tlb-shootdown.isolation]
Handling a shootdown for one target page MUST NOT modify cached translations
for any different page.

r[tlb-shootdown.range]
Range invalidation MUST apply the single-page invalidation rule to every page
in `[start, end)`, preserve pages outside the range, be identity on empty
ranges, and match the single-page rule on singleton ranges.

## seL4 Invocation Decode

r[sel4-invocation.decode-soundness]
Invocation decode MUST return a typed kernel invocation only when the supplied
message shape, looked-up capability kinds, and rights satisfy that label's
preconditions. In this repo's starter set, the modeled typed invocations are
endpoint send, endpoint receive, frame map, and untyped retype. For frame-map,
authority is checked on the primary frame capability; the secondary VSpace
capability is a typed root reference in this abstract decode surface.

r[sel4-invocation.reject]
Malformed messages, wrong-type capabilities, missing required secondary
capabilities, or missing required rights MUST cause decode failure with no
typed invocation result. Decode-level malformed checks reject zero size / zero
alignment in the starter set, while exact alignment and layout validity remain
downstream obligations of the modeled operation modules.

r[sel4-invocation.no-invented-authority]
A successful decode MUST mention only object identifiers already present in the
supplied decode context. Endpoint and untyped invocations use the primary
capability object, and frame-map uses only the primary frame object plus the
supplied secondary VSpace object.

## seL4 Kernel Invariants

r[sel4-invariants.compose]
The kernel state MUST satisfy one composed invariant covering at least
object-reference validity, capability derivation-tree well-formedness, CSpace
root and slot validity, TCB state legality, ready-queue consistency,
untyped-child ownership and disjointness, ASID pool well-formedness, VSpace
mapping consistency, and IRQ binding uniqueness.

r[sel4-invariants.decomposition]
The composed kernel invariant MUST decompose into named sub-invariants so local
proofs can depend only on the obligations they use. For this repo's starter
surface, the named obligations are object-reference validity, CDT
well-formedness, CSpace validity, TCB legality, scheduler consistency,
untyped ownership and top-level disjointness, ASID pool well-formedness,
VSpace consistency, and IRQ uniqueness.

r[sel4-invariants.step-preservation]
Each modeled primitive step in this change's initial preservation set MUST
preserve the composed kernel invariant whenever its explicit preconditions
hold. The initial set is endpoint send that blocks a running sender on an
empty receive queue, endpoint send that rendezvous-matches a queued receiver
and wakes it onto the ready queue, and untyped revoke that clears one untyped
object's live children.

r[sel4-invariants.preservation-matrix]
The seL4 invariant layer MUST maintain a local preservation matrix for modeled kernel steps. Each matrix entry MUST identify the step name, required prestate facts, mutated state fields, unchanged state fields, consumed sub-invariants, and proof or test evidence that links the entry back to `kernel_invariant_spec`.

r[sel4-invariants.step-preservation-expansion]
The seL4 invariant layer SHOULD expand the modeled preservation set around endpoint, TCB, ready-queue, and untyped interactions. Every accepted step in this expanded set MUST preserve `kernel_invariant_spec` whenever its stated preconditions hold, and rejected or out-of-scope requests MUST preserve the prestate without mutating global kernel state.

## Fetch-and-Add

r[fetch-add.semantics]
Fetch-and-add MUST return the old cell value and the new cell value after
applying the chosen arithmetic variant.

r[fetch-add.identity]
Fetch-and-add with addend zero MUST leave the cell unchanged and return the
original value.

r[fetch-add.commutativity]
Applying two fetch-and-add operations with addends `a` and `b` MUST yield the
same final cell value regardless of order for both wrapping and saturating
variants.

r[fetch-add.associativity]
Sequential fetch-and-add with addends `a` then `b` MUST yield the same final
cell value as one fetch-and-add with the combined addend for both wrapping and
saturating variants.

r[fetch-add.monotonicity]
For wrapping arithmetic, a positive addend without overflow MUST strictly
increase the cell value.

r[fetch-add.cas-refinement]
Wrapping fetch-and-add MUST refine a single successful compare-and-swap that
uses the old value as expected and the wrapped sum as desired.

r[fetch-add.wrapping]
The wrapping-add helper MUST satisfy modular arithmetic over `u64`, including
commutativity, identity, and the `u64::MAX + 1 == 0` overflow case.

r[fetch-add.saturating]
The saturating-add helper MUST clamp at `u64::MAX` and remain fixed once
saturated.

r[fetch-add.fetch-sub]
Fetch-sub MUST equal wrapping fetch-and-add with the two's-complement negation
of the subtrahend, and add/sub round-trips MUST restore the original value in
non-wrapping cases.

## seL4 Untyped Memory

r[sel4-untyped.retype]
Retyping an untyped region into typed kernel objects MUST conserve address
space, and every created child region MUST lie wholly within the parent
untyped region.

r[sel4-untyped.child-layout]
Live child objects derived from one untyped parent MUST satisfy their required
alignment and MUST NOT overlap one another.

r[sel4-untyped.reclaim]
Revoking or resetting an untyped object MUST remove all live children and
restore the parent to an empty-child reclaimed state.

## Choreography

r[newtypes.choreography-payload.type]
Choreography payload positions MUST use a distinct `PayloadTag` structure over
the complete `u32` value domain. Role and branch-label representations MUST
remain `u32` aliases during this pilot.

r[newtypes.choreography-payload.preservation]
The typed payload boundary MUST preserve existing well-formedness, projection,
merge, projectability, global-step, local-step, duality, choice-agreement, and
step-correspondence semantics for equal raw payload values.

r[newtypes.choreography-payload.validation]
Full-library Verus verification MUST pass without a new external body,
assumption, unverified public function, or proof-gap bridge.

r[newtypes.choreography-payload.evidence]
The pilot MUST include positive semantic tests, negative runtime tests, and
compile-fail evidence for invalid cross-domain payload construction.

r[newtypes.choreography-payload.docs]
Canonical requirements, source annotations, test annotations, public API
documentation, and accepted Cairn text MUST describe the typed payload boundary.

r[newtypes.choreography-payload.scope]
The pilot MUST NOT migrate `RoleId` or `LabelId`, add payload validation, alter
choreography semantics, or require newtypes for all verified identifiers.

r[choreography.value_ownership.identity]
Owned choreography values MUST use a distinct `ValueId` structure over the
complete `u32` domain. Roles, labels, and payload tags MUST NOT enter a value
identity position without explicit `ValueId` construction.

r[choreography.value_ownership.state]
An admitted finite ownership state MUST list represented values in strict
`ValueId` order. Each represented value MUST have exactly one owner entry.
Admission MUST reject duplicate and out-of-order values.

r[choreography.value_ownership.transfer]
An owned-transfer step MUST succeed only when source and destination differ and
the source owns the selected value. Success MUST advance to the stored
continuation and atomically replace that value's owner with the destination.
Failure MUST return no replacement choreography or ownership state.

r[choreography.value_ownership.conservation]
A successful owned-transfer step MUST preserve ownership-state normalization
and entry count. It MUST change only the selected owner, exclude the former
owner, and preserve every unrelated value owner.

r[choreography.value_ownership.boundary]
Owned-transfer evidence MUST describe only an atomic semantic owner change over
supplied finite facts. It MUST NOT claim transport delivery, persistence,
memory reclamation, authorization, exactly-once execution, or liveness.

r[choreography.value_ownership.evidence]
The ownership core MUST include positive, negative, compile-fail, and property
evidence for identity separation, admission, transfer, conservation, and stale
source rejection.

r[choreography.value_ownership.validation]
Ownership proofs MUST constrain the production admission and transfer
implementations. Full-library verification MUST preserve prior choreography
theorems without a new trusted bridge, assumption, or proof gap.

r[choreography.participants]
Finite choreography primitives MUST operate on exact `u32` role identifiers
and branch labels plus typed `PayloadTag` metadata. A communication or owned-
transfer prefix is well formed only if `from != to`. A choice is well formed
only if its branch
list is non-empty and its branch labels are pairwise distinct in strict
increasing order. `participants(choreo)` MUST contain exactly the roles that
appear as communication endpoints or as a branch decider in `choreo`.

r[choreography.communication]
A global communication prefix
`Comm { from, to, label, payload_tag, next }` MUST denote one exact
point-to-point interaction from `from` to `to` carrying the exact symbolic
message metadata `(label, payload_tag)` and continuing with the exact stored
continuation `next`. An `OwnedTransfer` prefix MUST additionally preserve one
exact `ValueId`; its ownership effect is defined only by the ownership step.

r[choreography.merge]
For a non-decider role, local branch projections over exact local nodes
`End`, `Send`, `Recv`, `SendOwned`, `RecvOwned`, `InternalChoice`, and `Offer`
MUST merge by a total helper
`merge_local(decider, left, right) -> Option<LocalChoreo>`.
Identical communication heads merge only when direction, peer, label, and
payload tag all match. Owned heads MUST also match the exact value identity.
A successful prefix merge keeps that exact head and recursively merges its
continuations.
Distinct `Recv` heads from the decider with distinct labels MUST merge into one
normalized `Offer` from that decider whose branch labels stay in strict
increasing order. `Offer` heads from the same decider MUST merge by sorted
label-set union, recursively merging shared labels and preserving unshared
labels. Every other head conflict MUST fail the merge. Merge recursion MUST use
an explicit structurally decreasing local-size measure, and successful merges of
well-formed inputs MUST preserve local branch-order invariants.

r[choreography.projectability]
A finite choreography MUST be projectable exactly when it is well formed and,
for every choice and every non-decider role appearing in at least one branch,
that role's branch projections admit the exact projection-side merge: preserve
any shared local communication or owned-transfer prefix common to every branch
in stored order, and when behavior first diverges, expose an `Offer` from the
decider keyed by
the stored global branch labels for the remaining branch-local continuations.
If a projectable choreography takes one global head step, the post-step
choreography MUST remain projectable.

r[choreography.projection]
For every projectable finite choreography `g` and role `r`,
`project_endpoint(r, g)` MUST be a preconditioned-total helper that returns the
exact local view implied by `g`: `End -> End`; a communication prefix projects
to `Send` for the sender, `Recv` for the receiver, and the projected
continuation for every uninvolved role; an owned-transfer prefix projects to
`SendOwned` for the source, `RecvOwned` for the destination, and the projected
continuation for every uninvolved role; a choice projects to a normalized
`InternalChoice` for the decider; and a choice projects for every non-decider
role to the exact outer-label-preserving merge of the per-branch projections,
preserving any shared local prefix and introducing an `Offer` from the decider
whose branch labels are the stored global choice labels exactly at the first
branch-dependent point. A role outside `participants(g)` MUST project to `End`.

r[choreography.steps]
A projectable finite choreography MUST have exact one-step global and local
semantics. `global_comm_step` returns `Some(next)` exactly on a global `Comm`
head. `global_choice_step(g, label)` returns `Some(branch_l)` exactly when the
head is a `Choice` carrying branch label `label`. `local_send_step` and
`local_recv_step` return `Some(next)` exactly on `Send` and `Recv` heads.
`owned_transfer_step` also requires current ownership and returns the exact
continuation plus the updated ownership state. `local_send_owned_step` and
`local_recv_owned_step` return `Some(next)` only on their matching owned heads.
`local_internal_choice_step` and `local_offer_step` return `Some(branch_l)`
exactly when the head is `InternalChoice` or `Offer` carrying label `label`.
Missing labels or wrong-head calls MUST return `None`. Sender and receiver
heads, and any currently exposed `InternalChoice` or `Offer` heads, MUST stay
aligned with the exact endpoint projection of the post-step choreography.
Roles hidden behind shared prefixes MUST stay aligned without fabricating a
spurious local head step.

r[choreography.value_transfer_projection.shape]
A projectable owned-transfer prefix MUST project to an exact `SendOwned` head
for its source and an exact `RecvOwned` head for its destination. Both heads
MUST preserve the peers, label, payload tag, value identity, and continuation.
Every uninvolved role MUST project through to the continuation.

r[choreography.value_transfer_projection.merge]
Owned local heads MUST merge only when direction, peer, label, payload tag, and
value identity match exactly. Every owned-head mismatch MUST fail merge.

## Choreography Cross-Module Properties

r[properties.choreography-duality]
For any projectable communication prefix, projection to the sender and
receiver MUST yield exact dual local heads: the sender sees
`Send { peer = receiver, ... }` and the receiver sees
`Recv { peer = sender, ... }` with the same exact `label` and `payload_tag`.

r[properties.choreography_transfer_correspondence]
For every projectable owned-transfer prefix admitted by a normalized ownership
state, the production global step and matching projected local steps MUST agree
on source, destination, label, payload tag, value identity, and continuation.
The global result MUST satisfy ownership conservation.

r[properties.choreography_transfer_exclusion]
After a successful transfer moves one value from source to destination, a later
step over the unchanged result MUST reject the former source for that value and
MUST return no fabricated ownership state.

r[properties.choreography-choice-agreement]
For any projectable choice with decider `D` and global branch-label set `L`,
the decider's local projection MUST expose exactly `L` as a normalized
`InternalChoice`. Every non-decider role whose branch behavior is identical
across all branches MUST collapse to that exact common local behavior. Every
other non-decider role MUST preserve any exact shared local prefix produced by
projection-side merge and, at the first exposed `Offer` from `D` if one
appears, MUST expose exactly `L`.

r[properties.choreography-step-correspondence]
If a projectable choreography takes one global head step to `g'`, then the
projected local views MUST stay exactly aligned with `project_endpoint(_, g')`.
Sender and receiver heads, and any currently exposed `InternalChoice` or
`Offer` heads, MUST take one matching local step to those exact endpoint
projections. Roles whose projection already collapses the global head behind a
shared prefix or identical behavior MUST already equal the corresponding
endpoint projection without inventing an extra local step.

## ChaosControl Coverage

r[chaoscontrol-coverage.regions]
A coverage bitmap MUST be a 65536-byte array partitioned into three exact,
non-overlapping contiguous regions: code `[0, 32768)`, assertion
`[32768, 49152)`, and schedule `[49152, 65536)`. Any public helper that
records hits into one region MUST leave the other two regions unchanged.

r[chaoscontrol-coverage.union]
Coverage merge MUST be exact pointwise max over u8 counters. A new-edge
query MUST return the exact u32 count of indices i with branch[i] > 0
and global[i] == 0.

r[chaoscontrol-coverage.classify]
Coverage classification MUST use the exact AFL bucket map:
0 -> 0, 1 -> 1, 2 -> 2, 3 -> 3, 4..7 -> 4, 8..15 -> 8,
16..31 -> 16, 32..127 -> 32, 128..255 -> 128.

## ChaosControl Oracle

r[chaoscontrol-oracle.verdict]
Verdict evaluation MUST use hit_count > 0 as the meaning of "was hit".
always passes iff hit and no false. sometimes passes iff hit and at
least one true. reachable passes iff hit. unreachable passes iff never
hit. Every valid record maps to exactly one verdict.

r[chaoscontrol-oracle.accounting]
Aggregate accounting MUST preserve runs_satisfied <= runs_hit and
runs_hit == 0 iff hit_count == 0. Boolean records satisfy
hit_count == true_count + false_count and runs_hit <= hit_count.
Non-boolean records use exact canonical shapes.

## ChaosControl Scheduler

r[chaoscontrol-scheduler.turns]
Scheduler turn primitives require num_vcpus >= 1 and valid scheduler
state. Single-vCPU tick and advance are no-ops. SMP tick decrements
remaining by one and returns true on the positive-to-zero transition.
Advance moves to (active + 1) mod num_vcpus and reloads remaining.
Round-robin reloads default_quantum. Randomized advance consumes a
caller rng_word and assigns min_quantum + (rng_word mod (max_quantum -
min_quantum)) for [min_quantum, max_quantum).

r[chaoscontrol-scheduler.fingerprint]
The rolling schedule fingerprint updates by
fp.wrapping_mul(0x517cc1b72722_0a95).bitxor(active).bitxor(quantum)
on each SMP advance. Tick is fingerprint-neutral. apply_variant resets
fingerprint to 0.

## ChaosControl Fault Schedule

r[chaoscontrol-fault-schedule.replay]
A normalized fault schedule has unique ordinals, stable (time_ns, ordinal)
order, and 0 <= cursor <= len. drain_due(t) returns exactly the
not-yet-drained faults with time_ns <= t, advances cursor monotonically,
preserves the undrained suffix, and never re-delivers.

r[chaoscontrol-fault-schedule.subset]
Subset extraction clones the first occurrence of each in-bounds source
index, ignores out-of-bounds and repeated selections, carries source
ordinals, resets cursor to 0, and renormalizes by (time_ns, ordinal).

## ChaosControl Frontier

r[chaoscontrol-frontier.selection]
Frontier effective score is base_score / (1 + times_selected). Greedy
selection returns a maximal entry with lowest-index tie-break.
Re-selection is monotone non-increasing. Exec comparison uses exact
u64 cross-products.

## ChaosControl Input Tree

r[chaoscontrol-input-tree.alternatives]
Input-tree generation operates on normalized choice tuples with
n_options >= 2, original_value < n_options, arithmetic bounds on
sequence_id and n_options, and unique (vm_id, sequence_id) pairs.
Exact rational score uses constant-numerator denominator comparison.
Alternatives exclude the original value, use ascending distinct values,
and output is the exact prefix of the deterministic stream truncated
to min(budget, full_stream_len).

## ChaosControl Fault State

r[chaoscontrol-fault-state.quorum-health]
Valid fault states require 2 <= num_nodes <= 64, exact num_nodes x
num_nodes partition matrix, diagonal-false, and cleared high bits in
alive_mask. quorum_reachable is false when alive_count < majority and
otherwise true when some bidirectional alive pair exists.

r[chaoscontrol-fault-state.liveness-gate]
The healthy-period gate is alive_count == num_nodes AND
quorum_reachable.

## Cryptoshedding

r[cryptoshedding.tree-well-formedness]
well_formed_spec MUST accept only finite parent-indexed forests: every
parent reference is in bounds and no strict parent walk cycles back to
its start.

r[cryptoshedding.shed-definition]
shed MUST remove exactly the requested key from the keystore model and
leave the key tree unchanged.

r[cryptoshedding.chain-recoverability]
recoverable_spec MUST hold exactly when the given leaf is in bounds, is a
leaf, and every key on its leaf-to-root chain is present in the keystore.

r[cryptoshedding.shed-completeness]
Shedding any key on a leaf's chain, including the leaf itself, MUST make
that leaf irrecoverable.

r[cryptoshedding.shed-isolation]
Shedding a key not equal to the leaf and not on the leaf's strict
ancestor chain MUST leave that leaf's recoverability unchanged.

r[cryptoshedding.subtree-shedding]
For any in-bounds internal key K, shedding K MUST make every leaf in K's
subtree irrecoverable and MUST leave leaves outside that subtree
unaffected.

r[cryptoshedding.rotation-correctness]
rotate MUST update only the chosen leaf parent, make the new parent an
ancestor of that leaf in the rotated tree, and drive recoverability by
the rotated leaf's new chain.

r[cryptoshedding.idempotent-destruction]
Shedding the same key twice MUST leave both the keystore model and every
leaf's recoverability unchanged after the first shed.

## UCAN Signature

r[ucan-signature.round-trip]
Signing a message with a secret key and verifying with the corresponding
public key MUST succeed.

r[ucan-signature.key-binding]
Verifying a signature with a public key that does not correspond to the
signing secret key MUST fail.

r[ucan-signature.message-binding]
Verifying a signature against a different message than the one signed
MUST fail.

r[ucan-signature.deterministic]
Signing the same message with the same key MUST produce the same
signature.

r[ucan-signature.secret-key-length]
An Ed25519 secret key accepted by checked constructors MUST be exactly 32 bytes.

r[ucan-signature.signature-length]
An Ed25519 signature MUST always be exactly 64 bytes.

r[ucan-signature.public-key-length]
An Ed25519 public key derived from a secret key MUST be exactly 32 bytes.

r[ucan-signature.checked-byte-inputs]
Byte-level UCAN signature entrypoints MUST reject malformed key or signature
lengths before invoking trusted Ed25519 crypto. Valid byte inputs MUST be
converted through checked constructors before signing, verification, or
public-key derivation.

## UCAN DID

r[ucan-did.round-trip]
Encoding key bytes as a did:key MUST use the canonical byte layout
`did:key:z ++ base58btc(varint(codec) ++ key_bytes)`. The public encode
result MUST preserve the exact key bytes, codec, and encoded bytes, and
public/spec-level decode of those encoded bytes MUST recover the original key
bytes and codec.

r[ucan-did.injective]
Distinct public keys encoded with the same codec MUST produce distinct
did:key strings.

r[ucan-did.prefix-valid]
An encoded did:key MUST always start with the string "did:key:z".

r[ucan-did.codec-preserved]
The codec recovered from decoding a did:key MUST match the codec used
for encoding.

r[ucan-did.varint-round-trip]
Encoding a value as a LEB128 varint and decoding MUST recover the
original value and consumed encoded length. Decoding an encoded varint
followed by trailing bytes MUST consume only the encoded varint prefix.

r[ucan-did.varint-bounded-u64]
DID varint decoding MUST use bounded u64 LEB128 semantics: a terminating
byte within the u64 byte budget returns a representable value and consumed
length; empty, continuation-only, overlong, and overflowing inputs return
None; non-minimal representable encodings remain accepted.

r[ucan-did.varint-rejects-empty]
Decoding a varint from an empty byte sequence or truncated continuation
sequence MUST return None.

r[verification-gaps.close-ucan-did-varint-decode]
The UCAN DID `varint_decode` implementation MUST be verified directly and
MUST NOT use `external_body`, `assume(...)`, `assume_specification[...]`,
`@verified-logic:no-ensures`, or `@verified-logic:exempt` replacement trust.

r[verification-gaps.inventory-after-ucan-did-varint]
The proof-gap inventory and README counts MUST remove the closed UCAN DID
`varint_decode` boundary and publish source-derived non-double-counted totals.

r[ucan-did.base58btc-round-trip]
Encoding bytes as base58btc and decoding MUST recover the original bytes.

r[ucan-did.base58btc-alphabet]
Base58btc decode MUST reject encoded byte sequences containing bytes outside
the Bitcoin/base58 alphabet before reaching the trusted codec core. The checked
alphabet excludes `0`, `O`, `I`, `l`, and non-alphanumeric separator bytes.

## UCAN Token

r[ucan-token.temporal-validity]
Temporal validity MUST be the conjunction of not-expired (with tolerance)
and not-before-satisfied (with tolerance).

r[ucan-token.self-delegation-rejected]
A UCAN token where issuer equals audience MUST be rejected.

r[ucan-token.attenuation-monotone]
Each child token's capabilities MUST be contained by its parent's
capabilities.

r[ucan-token.validation-sound]
If validate_envelope succeeds, then signature is valid, temporal constraints
hold, identity constraints hold, and required fields are present.

r[ucan-token.validation-complete]
If all sub-checks pass individually, validate_envelope MUST succeed.

r[ucan-token.issuer-present]
A validated UCAN MUST have a non-empty issuer DID.

r[ucan-token.audience-present]
A validated UCAN MUST have a non-empty audience DID.

r[ucan-token.attenuations-present]
A validated UCAN MUST have at least one capability in att.

r[ucan-token.canonical-bytes-assumed]
Canonical payload-byte construction is outside the scope of token-envelope
validation. The token layer MUST consume a caller-supplied signature-valid
result rather than prove canonical payload-byte derivation from the payload
struct.

## UCAN Compact

r[ucan-compact.signing-input-framing]
Canonical compact-token signing input MUST be exactly the encoded header bytes,
one `.` separator byte, and the encoded payload bytes. This structural helper
MUST NOT claim to prove base64url, JSON, hashing, or Ed25519 internals.

r[ucan-compact.three-part-framing]
Canonical compact-token framing MUST be exactly encoded header bytes, one `.`
separator byte, encoded payload bytes, one `.` separator byte, and encoded
signature bytes.

r[ucan-compact.parse-signature-boundary]
A compact-token envelope check MUST accept only when both the caller-supplied
compact bytes and caller-supplied signing input bytes match the canonical
framing over the same already-encoded parts. The check MUST remain data-only and
MUST NOT decode base64url, parse JSON, derive hashes, sign, verify signatures,
perform I/O, or read clocks.

r[ucan-compact.byte-equality]
Compact-token structural checks MUST compare finite byte vectors exactly by
length and byte value without normalization or lossy decoding.

## UCAN Ability

r[ucan-ability.subsumption-reflexive]
A non-glob ability MUST subsume itself.

r[ucan-ability.subsumption-transitive]
If pattern A subsumes pattern B and B subsumes ability C, then A MUST
subsume C.

r[ucan-ability.top-subsumes-all]
The top ability pattern "*" MUST subsume every ability.

r[ucan-ability.namespace-isolation]
Abilities in different top-level namespaces MUST NOT subsume each other
unless one is the top pattern.

r[ucan-ability.glob-one-level]
A single "*" in an ability pattern MUST match exactly one path segment,
not deeper nesting.

r[ucan-ability.containment-monotone]
If parent ability contains child ability, then everything the child
subsumes is also subsumed by the parent.

## UCAN Caveat

r[ucan-caveat.satisfaction-decidable]
For each concrete caveat type, satisfaction against a context MUST be
a total, decidable function.

r[ucan-caveat.set-conjunction]
A caveat set is satisfied if and only if every caveat in the set is
individually satisfied.

r[ucan-caveat.containment-monotone]
If parent contains child, then any context satisfying the child also
satisfies the parent.

r[ucan-caveat.set-containment-monotone]
If parent set contains child set, then any context satisfying the child
set also satisfies the parent set.

r[ucan-caveat.delegation-only-adds]
A child caveat set in a delegation chain MUST be at least as restrictive
as its parent.

r[ucan-caveat.intersection-sound]
The intersection of two caveat sets MUST imply both original sets.

r[ucan-caveat.intersection-minimal]
The intersection MUST not be more restrictive than the conjunction of
both sets.

## UCAN Claims Presence

r[ucan-claims-presence.required-fields]
Parsed UCAN claims presence admission MUST accept exactly when issuer, audience, attenuations, proof-list, expiration, and not-before presence facts are all true.

## UCAN Parsed Temporal Order

r[ucan-temporal-order.not-before-precedes-expiration]
Parsed UCAN temporal-order admission MUST accept exactly when the parsed not-before timestamp is strictly earlier than the parsed expiration timestamp.

## UCAN Parsed Proof Reference List Shape

r[ucan-proof-reference-list-shape.entry-shape]
Parsed proof-reference list-shape admission MUST accept exactly when every shell-supplied entry fact has non-empty encoded bytes, base64url-no-pad structural admission, and non-empty decoded bytes.

r[ucan-proof-reference-list-shape.list-shape]
Parsed proof-reference list-shape admission over a list MUST accept exactly when every entry-shape fact accepts; empty root proof lists MAY accept.

## UCAN Proof Reference Collection

r[ucan-proof-reference-collection.collection-admission]
Proof-reference collection admission MUST accept exactly when the caller-supplied reference count is within the caller-supplied bound and every reference is non-empty.

## UCAN Parsed Capability List Shape

r[ucan-parsed-capability-list-shape.required-entry-fields]
Parsed capability-list shape admission MUST accept exactly when the list is non-empty and every parsed capability has resource and ability presence facts.

## UCAN Parsed Caveat List Shape

r[ucan-parsed-caveat-list-shape.required-entry-fields]
Parsed caveat-list shape admission MUST accept exactly when every parsed caveat has domain, type, key, and payload presence facts; empty caveat lists MAY accept.

## UCAN Parsed Caveat Payload Admission

r[ucan-parsed-caveat-payload-admission.payload-facts]
Parsed caveat payload admission MUST accept exactly when every shell-supplied payload fact has payload presence, encoded non-empty, encoded base64url-no-pad admission, and decoded non-empty bytes; empty caveat payload lists MAY accept.

## UCAN Parsed DID Claim Admission

r[ucan-parsed-did-claim.claim-admission]
Parsed DID claim admission MUST accept exactly when a shell-supplied DID claim fact is present, non-empty, and already trimmed.

r[ucan-parsed-did-claim.claims-admission]
Parsed DID claims admission MUST accept exactly when both issuer and audience DID claim facts are admitted.

## UCAN Proof Store Result Transcript

r[ucan-proof-store-result-transcript.result-admission]
Proof-store result admission MUST accept exactly when a non-empty requested reference has a successful backend result with returned compact-token bytes present.

## UCAN Revocation Check Transcript

r[ucan-revocation-check-transcript.result-admission]
Revocation-check admission MUST accept exactly when a non-empty token reference has a successful backend result and the token is not revoked.

## UCAN Caveat Evaluation Transcript

r[ucan-caveat-evaluation.fact-accepts]
A caveat evaluation fact MUST accept exactly when a policy is present and the policy satisfied the caveat.

r[ucan-caveat-evaluation.transcript-accepts]
A caveat evaluation transcript MUST accept exactly when every fact accepts.

r[ucan-caveat-evaluation.first-failed-index]
The first failed caveat index MUST identify the first missing-policy or unsatisfied fact, or the transcript length when all facts accept.

## UCAN Caveat Preservation

r[ucan-caveat-preservation.byte-equality]
Byte equality helpers MUST accept if and only if two caveat identifier byte
sequences have the same length and equal bytes at every index.

r[ucan-caveat-preservation.child-contains-parent-id]
A child caveat identifier list MUST contain a parent caveat identifier if and
only if some child identifier equals the parent identifier byte-for-byte.

r[ucan-caveat-preservation.parent-ids-preserved]
Parent caveat identifier preservation MUST accept if and only if every parent
caveat identifier appears byte-for-byte in the child caveat identifier list;
children MAY include additional caveat identifiers.

## UCAN CID

r[ucan-cid.cid-deterministic]
Constructing a CID from the same content, codec, and hash function MUST
always produce the same CID.

r[ucan-cid.cid-version]
Constructed CIDs MUST always have version 1.

r[ucan-cid.cid-round-trip]
Encoding a CID's canonical byte sequence as a multibase string and
decoding MUST recover the original CID bytes.

r[ucan-cid.cid-injectivity]
If two CIDs have the same byte representation, their version and codec
MUST be identical.

r[ucan-cid.multihash-digest-length]
The digest in a wrapped multihash MUST have the same length as the input
digest.

r[ucan-cid.multihash-round-trip]
Encoding a multihash to bytes and parsing MUST recover the original code
and digest.

r[ucan-cid.varint-round-trip]
Encoding a value as a varint and decoding MUST recover the original value.

r[ucan-cid.varint-rejects-empty]
Decoding a varint from an empty byte sequence MUST return None.

## UCAN Proof Chain

r[ucan-chain.issuer-audience-link]
Each hop in a delegation chain MUST have the child's issuer equal to the
parent's audience DID.

r[ucan-chain.signature-check]
Every token in the chain MUST carry a caller-provided valid-signature flag; the
structural chain checker MUST consume that flag without claiming to verify
cryptographic signature bytes or issuer keys.

r[ucan-chain.attenuation-narrowing]
Each delegation hop MUST narrow or preserve attenuations: every child capability
is contained in some parent capability, and public chain attenuation checking MUST
reject missing parent capabilities, resource escalation, and ability escalation.

r[ucan-chain.temporal-validity]
Every token in the chain MUST be temporally valid at the time of
evaluation.

r[ucan-chain.chain-validity]
A structural chain-validity checker MUST accept only non-empty chains whose hops
are linked, whose caller-provided signature-valid flags are all true, whose
attenuations narrow at every hop, and whose entries are temporally valid. This
structural checker MUST NOT claim to verify cryptographic signatures, DID
resolution, revocation, or external clock correctness.

r[ucan-chain.root-authority]
The root authority of a valid chain is the attenuation set of the
first (root) entry.

## UCAN Proof Traversal Transcript

r[ucan-proof-traversal.byte-equality]
Byte equality helpers MUST accept if and only if two byte sequences have the
same length and equal bytes at every index.

r[ucan-proof-traversal.capability-containment]
A traversal child capability MUST be contained by a parent capability when the
child resource has the parent resource as a prefix and the parent UCAN ability
contains the child UCAN ability, including slash-separated full-segment wildcard
abilities.

r[ucan-proof-traversal.graph-well-formedness]
A traversal proof edge MUST be well-formed only when the parent audience equals
the child issuer, the child temporal window narrows the parent window, child
capabilities are contained by parent capabilities, and parent caveat identifiers
are preserved in order at the start of the child caveat list.

r[ucan-proof-traversal.bounded-traversal]
A finite proof traversal transcript MUST respect caller-supplied maximum depth,
proof count, and input-byte budgets, MUST have one visited reference per entry,
and MUST reject duplicate visited references.

r[ucan-proof-traversal.effective-delegation-folding]
Effective delegation folding over a traversal transcript MUST require every
adjacent proof edge to be well-formed and MUST reject resource, ability,
temporal, or caveat-preservation escalation at any hop.

r[ucan-proof-traversal.chain-authorization]
A traversal transcript MUST authorize a request only when the folded chain is
valid, every entry is valid at the caller-supplied evaluation time plus tolerance,
and the leaf capability set authorizes the requested resource and ability.

r[ucan-proof-traversal.revocation-aware-chain]
A revocation-aware traversal transcript MUST reject if any caller-supplied entry
revocation fact is true. The primitive MUST NOT claim revocation registry
freshness or backend correctness.

r[ucan-proof-traversal.transcript-consistency]
A traversal transcript validity check MUST compose bounded traversal,
authorization, and revocation-aware denial over finite caller-supplied data, and
MUST NOT perform token parsing, proof fetching, revocation lookup, clock reads,
network I/O, filesystem I/O, or cryptographic verification.

r[ucan-proof-traversal.dag-transcript-consistency]
A DAG traversal transcript validity check MUST compose bounded traversal,
topologically ordered acyclic parent indices, per-parent issuer/audience and
temporal proof links, per-child capability attenuation through at least one
parent, per-parent caveat preservation, leaf authorization, and revocation-aware
denial over finite caller-supplied data, and MUST NOT perform token parsing,
proof fetching, revocation lookup, clock reads, network I/O, filesystem I/O, or
cryptographic verification.

## UCAN Revocation

r[ucan-revocation.cid-lookup]
A CID is revoked if and only if it appears in the revocation set.

r[ucan-revocation.chain-integration]
A chain is revocation-free if no proof CID in the chain is in the
revocation set. Public structural revocation admission MUST compose finite
structural chain validity with caller-provided proof-CID lookup, reject CID/entry
length mismatches, reject any revoked proof CID, and avoid claims about
revocation-list freshness or external registry correctness.

r[ucan-revocation.monotonicity]
Adding entries to the revocation set MUST NOT un-revoke a previously
revoked CID.

## UCAN Delegation Depth

r[ucan-delegation-depth.depth-definition]
Delegation depth is the number of hops (entries minus one) in a
proof chain. A single token has depth 0.

r[ucan-delegation-depth.budget-check]
A chain respects a depth budget if its depth does not exceed the
maximum allowed depth.

## UCAN Temporal Composition

r[ucan-temporal.window-valid]
A time window is non-empty if nbf <= exp.

r[ucan-temporal.window-intersection]
The intersection of two windows uses max(nbf) and min(exp).

r[ucan-temporal.chain-composition]
The effective validity window of a delegation chain is the
intersection of all individual token windows.

## UCAN Invocation

r[ucan-invocation.invoker-matches-audience]
An invocation is only authorized if the invoker DID matches the
leaf token's audience DID.

r[ucan-invocation.executor-is-invoker]
A valid receipt confirms that the executor (ran_by) matches the
invoker from the invocation.

r[ucan-invocation.receipt-ordering]
The receipt timestamp MUST be at or after the invocation timestamp.

r[ucan-invocation.receipt-validity]
A receipt is valid if the executor matches the invoker and
timestamps are properly ordered.

r[ucan-invocation.authorization-check]
An authorized invocation's resource MUST be a refinement of the leaf capability's
resource (prefix match) with matching ability, and the public authorization
checker MUST reject mismatched audiences, non-prefixed resources, or mismatched
abilities. Public chain-backed request admission MUST additionally bind the
invocation authorization CID to the resolved proof-chain leaf CID, compose
structural chain validity, proof-CID revocation admission, leaf-capability index
bounds, and invocation authorization without claiming cryptographic signature
verification, DID resolution, revocation freshness, or external registry
correctness.

## UCAN Principal Linking

r[ucan-principal-link.did-to-key]
Extracting a public key from a valid did:key encoding MUST recover
the original key bytes.

r[ucan-principal-link.key-to-principal]
Computing a principal fingerprint from a key is deterministic: the
same key always yields the same principal ID.

r[ucan-principal-link.did-to-principal]
Deriving a principal from a DID goes through key extraction then
BLAKE3 hashing.

r[ucan-principal-link.same-key-same-principal]
If two DID encodings represent the same key, they yield the same
principal fingerprint.

## UCAN Capability Merge

r[ucan-capability-merge.containment]
Capability containment: a child capability is contained in a parent
if the child's resource has the parent's resource as prefix and
abilities match.

r[ucan-capability-merge.union]
The union of two capability sets is their concatenation, and it
contains every capability authorized by either set.

r[ucan-capability-merge.authorization]
A capability is authorized by a set if some entry in the set contains it, and
public authorization checking MUST accept capabilities contained by either side
of a merged set while rejecting resource or ability escalation.

## Composite Primitives

r[composite-outcome.tags]
Composite outcome helpers MUST expose exactly the shared `Accept`, `Reject`, and
`Wait` tags for composite facades: each constructor helper returns its matching
disjoint tag, `composite_outcome_valid_spec` covers all three tags, and
`composite_outcome_code` maps them stably to codes `1`, `2`, and `3`.

r[composite-primitives.durable-job-executor]
The system MUST provide a durable job executor primitive for idempotent,
resume-aware, retry-budgeted job admission.

r[composite-primitives.fenced-distributed-lock]
The system MUST provide a fenced distributed lock primitive for stale-holder-
safe protected writes.

r[composite-primitives.linearizable-read-gate]
The system MUST provide a Raft read gate primitive for one-step linearizable
read admission.

r[composite-primitives.safe-snapshot-replication]
The system MUST provide a snapshot replication primitive for complete,
integrity-checked, compaction-safe installation.

r[composite-primitives.crash-safe-page-cache]
The system MUST provide a crash-safe page-cache primitive for WAL-backed dirty
page recovery.

r[composite-primitives.verified-lsm-ingest]
The system MUST provide an LSM ingest primitive for flush and compaction-safe
run admission.

r[composite-primitives.serializable-txn-guard]
The system MUST provide a transaction guard for commit admission from
isolation, dependency, and lock-discipline facts.

r[composite-primitives.web-of-trust-ucan-authorizer]
The system MUST provide a UCAN authorizer that combines token validity,
attenuation, caveats, DID/CID byte models, and rooted web-of-trust policy.

r[composite-primitives.policy-decision-engine]
The system MUST provide a policy decision engine for one allow/deny result
from capability and trust primitives.

r[composite-primitives.blocking-bounded-channel]
The system MUST provide a blocking bounded channel primitive that combines
queue capacity with wait/wake and backoff state.

r[composite-primitives.safe-reclamation-domain]
The system MUST provide a safe reclamation domain from RCU, epoch, hazard, and
reference-count protections.

r[composite-primitives.cache-coherent-memory-region]
The system MUST provide a cache-coherent memory-region primitive combining
cache protocol, directory, store-buffer, prefetch, and write-policy facts.

r[composite-primitives.verified-compiler-pipeline]
The system MUST provide a bounded compiler-pipeline primitive for the existing
raw IR model.

r[composite-primitives.backend-frame-allocator]
The system MUST provide a backend frame allocator for the existing live-range,
register, and local/spill stack-slot model.

r[composite-primitives.dependency-scheduler]
The system MUST provide a dependency scheduler that admits only ready,
capacity-safe work from graph and scheduling facts.

r[composite-primitives.raft-membership-manager]
The system MUST provide a Raft membership manager for safe configuration
changes, learner promotion, and leadership transfer.

r[composite-primitives.exactly-once-ledger-transfer]
The system MUST provide an exactly-once ledger transfer primitive for replay-
safe money movement.

r[composite-primitives.sel4-invocation-step]
The system MUST provide a seL4 invocation-step primitive that preserves kernel
invariants across a bounded modeled capability invocation slice: `CopyCap`,
`MapPage`, `EndpointSend`, and `BlockSenderOnEndpoint`. IRQ, notification,
ASID, untyped, reply, receive, scheduling, and deletion/revocation invocations
are out of scope for this composite.

r[composite-primitives.nix-eval-closure-policy]
The system MUST provide a Nix/Nickel closure-policy primitive for merged
config and reachable package closure admission.

r[composite-primitives.chaoscontrol-exploration-core]
The system MUST provide a ChaosControl exploration core for deterministic,
coverage-accounted, oracle-checked exploration steps.

## GIS Domain Primitives

r[domain-primitives.bounded-interval]
Bounded interval primitives MUST admit only normalized closed integer intervals, MUST reject inverted intervals at public constructors, MUST report containment and overlap consistently with endpoints, and MUST clamp accepted values into the accepted interval.

r[domain-primitives.rect-grid-index]
Rectangular grid indexing primitives MUST admit only non-zero finite dimensions and in-range coordinates. Accepted coordinates MUST flatten deterministically using row-major `y * width + x` arithmetic; this finite primitive does not claim a verified inverse/unflatten surface for arbitrary offsets.

r[domain-primitives.gis-coordinate-envelope]
GIS coordinate-envelope primitives MUST compose bounded interval admission for exact scaled integer coordinates. Public constructors MUST reject out-of-range latitude/longitude or projected components without claiming projection, geodesic, or floating-point accuracy.

r[domain-primitives.gis-bounding-box]
GIS bounding-box primitives MUST compose interval facts for normalized rectangular envelopes. Public constructors MUST reject inverted boxes, and accepted containment/intersection predicates MUST remain consistent with normalized axis bounds.

r[domain-primitives.gis-tile-index]
GIS tile-index primitives MUST admit only zoom/x/y addresses inside the finite quadtree range. Accepted parent computations MUST remain in range and preserve the discrete parent/child refinement relation without claiming map-provider semantics.

r[domain-primitives.gis-linestring-topology]
GIS linestring-topology primitives MUST admit only finite discrete non-degenerate segment chains in the first-suite model, reject duplicate consecutive points, and conservatively reject explicitly unsupported self-intersection patterns without claiming full computational-geometry robustness.

## ML Domain Primitives

### ML Tensor Shape
r[domain-primitives.ml-tensor-shape]
Tensor-shape primitives MUST admit only finite rank-1 shapes with non-zero dimensions and MUST map accepted indexes to flattened offsets inside the element count. They MUST reject rank mismatches, zero dimensions, and out-of-bounds indexes.

### ML Label Encoding
r[domain-primitives.ml-label-encoding]
Label-encoding primitives MUST admit declared dense class labels, map each declared label to a stable class index, decode that index back to the same label, and reject labels outside the declared class count.

### ML Dataset Split
r[domain-primitives.ml-dataset-split]
Dataset-split primitives MUST admit only split sizes whose train/validation/test sum fits within the bounded dataset size, and MUST classify dataset indexes into deterministic non-overlapping split ranges.

### ML Batch Iterator
r[domain-primitives.ml-batch-iterator]
Batch-iterator primitives MUST admit only positive batch sizes whose requested batch starts inside the dataset, and MUST return non-empty contiguous ranges bounded by the dataset size.

### ML Feature Normalization
r[domain-primitives.ml-feature-normalization]
Feature-normalization primitives MUST reject missing values and invalid or zero-width feature ranges, and accepted normalized endpoint outputs MUST stay inside the declared target interval.

### ML Top K
r[domain-primitives.ml-top-k]
Top-k primitives MUST reject zero or out-of-range `k` requests and MUST return deterministic bounded score ordering with stable tie-breaking for the finite supported score vector.

### ML Inference Envelope
r[domain-primitives.ml-inference-envelope]
Inference-envelope primitives MUST admit only classification or regression outputs that fit declared metadata bounds, including declared class counts, bounded confidence scores, and regression envelopes.

### ML Confusion Matrix
r[domain-primitives.ml-confusion-matrix]
Confusion-matrix primitives MUST update only bounded prediction-count cells and MUST reject numeric precision or recall availability when the corresponding denominator is zero.

### ML Non-Claims
r[domain-primitives.ml-non-claims]
ML primitives MUST NOT claim model quality, training correctness, fairness, probabilistic guarantees, floating-point accuracy, stochastic convergence, or production inference correctness.

## Physics Domain Primitives

### Requirement: Physics Units [r[domain-primitives.physics-units]]
The system MUST provide a physics-units primitive for dimension-safe scalar and
vector quantities.

#### Scenario: Compatible quantities combine safely [r[domain-primitives.physics-units.scenario.compatible-quantities-combine-safely]]

- GIVEN two quantities with the same dimension vector
- WHEN addition is requested
- THEN the result keeps that dimension vector

#### Scenario: Incompatible dimensions reject additive operation [r[domain-primitives.physics-units.scenario.incompatible-dimensions-reject-additive-operation]]

- GIVEN quantities with different dimension vectors
- WHEN addition or subtraction is requested
- THEN the operation is rejected

### Requirement: Physics Vector Bounds [r[domain-primitives.physics-vector-bounds]]
The system MUST provide a physics vector-bounds primitive for component-wise and
aggregate envelope admission over exact discrete vectors.

#### Scenario: Vector inside envelope is admitted [r[domain-primitives.physics-vector-bounds.scenario.vector-inside-envelope-is-admitted]]

- GIVEN vector components within every configured bound
- WHEN vector admission is checked
- THEN the vector is accepted

#### Scenario: Component outside envelope is rejected [r[domain-primitives.physics-vector-bounds.scenario.component-outside-envelope-is-rejected]]

- GIVEN any vector component outside its configured bound
- WHEN vector admission is checked
- THEN the vector is rejected

### Requirement: Physics Time Horizon [r[domain-primitives.physics-time-horizon]]
The system MUST provide a physics time-horizon primitive for bounded step-count and positive time-delta admission.

#### Scenario: Valid step count stays inside horizon [r[domain-primitives.physics-time-horizon.scenario.valid-step-count-stays-inside-horizon]]

- GIVEN a positive step size and bounded step horizon
- WHEN the step count is computed
- THEN every admitted step index is inside the configured finite horizon

#### Scenario: Invalid horizon parameters are rejected [r[domain-primitives.physics-time-horizon.scenario.overflowing-horizon-arithmetic-is-rejected]]

- GIVEN zero time-delta, zero step count, or a delta larger than the configured horizon
- WHEN horizon admission is checked
- THEN the horizon is rejected

### Requirement: Physics Integrator Step [r[domain-primitives.physics-integrator-step]]
The system MUST provide a physics integrator-step primitive for bounded-time
state advancement.

#### Scenario: Bounded step advances time [r[domain-primitives.physics-integrator-step.scenario.bounded-step-advances-time]]

- GIVEN a finite modeled state and a valid positive time delta and non-saturated finite timestamp
- WHEN one integration step is admitted
- THEN the output timestamp advances by one modeled tick and state fields remain bounded

#### Scenario: Invalid time step is rejected [r[domain-primitives.physics-integrator-step.scenario.invalid-time-step-is-rejected]]

- GIVEN a zero delta, invalid bounded state, or saturated timestamp
- WHEN integration is requested
- THEN the step is rejected

### Requirement: Physics Collision Envelope [r[domain-primitives.physics-collision-envelope]]
The system MUST provide a physics collision-envelope primitive for conservative
swept-volume overlap checks.

#### Scenario: Overlapping swept envelopes produce candidate [r[domain-primitives.physics-collision-envelope.scenario.overlapping-swept-envelopes-produce-candidate]]

- GIVEN two valid moving envelopes whose conservative swept volumes overlap
- WHEN collision admission is checked
- THEN the pair is returned as a collision candidate

#### Scenario: Separated envelopes are rejected [r[domain-primitives.physics-collision-envelope.scenario.separated-envelopes-are-rejected]]

- GIVEN two valid envelopes whose conservative swept volumes are disjoint
- WHEN collision admission is checked
- THEN the pair is rejected as non-colliding for the step

### Requirement: Physics Contact Manifold [r[domain-primitives.physics-contact-manifold]]
The system MUST provide a physics contact-manifold primitive for bounded contact
point, normal, and penetration records.

#### Scenario: Valid bounded contact record is accepted [r[domain-primitives.physics-contact-manifold.scenario.valid-bounded-contact-record-is-accepted]]

- GIVEN contact points, normals, and penetration values inside configured envelopes
- WHEN contact admission is checked
- THEN the contact manifold is accepted

#### Scenario: Invalid contact record is rejected [r[domain-primitives.physics-contact-manifold.scenario.invalid-contact-record-is-rejected]]

- GIVEN an invalid normal, unsupported contact count, or out-of-envelope penetration value
- WHEN contact admission is checked
- THEN the contact manifold is rejected

### Requirement: Physics Energy Budget [r[domain-primitives.physics-energy-budget]]
The system MUST provide a physics energy-budget primitive for non-negative energy
accounting and bounded drift.

#### Scenario: Energy-preserving transition is accepted [r[domain-primitives.physics-energy-budget.scenario.energy-preserving-transition-is-accepted]]

- GIVEN a valid pre-state and post-state whose energy difference is inside the configured drift budget
- WHEN the energy budget is checked
- THEN the transition is accepted

#### Scenario: Excess drift is rejected [r[domain-primitives.physics-energy-budget.scenario.excess-drift-is-rejected]]

- GIVEN a transition whose energy delta exceeds the configured budget
- WHEN the energy budget is checked
- THEN the transition is rejected

### Requirement: Physics Momentum Budget [r[domain-primitives.physics-momentum-budget]]
The system MUST provide a physics momentum-budget primitive for non-negative mass
and bounded momentum accounting.

#### Scenario: Momentum update inside budget is accepted [r[domain-primitives.physics-momentum-budget.scenario.momentum-update-inside-budget-is-accepted]]

- GIVEN valid masses and momentum values whose delta is inside the configured budget
- WHEN the momentum budget is checked
- THEN the update is accepted

#### Scenario: Invalid mass or momentum drift is rejected [r[domain-primitives.physics-momentum-budget.scenario.invalid-mass-or-momentum-drift-is-rejected]]

- GIVEN negative mass or momentum drift outside the configured budget
- WHEN momentum admission is checked
- THEN the update is rejected

### Requirement: Physics Non-Claims [r[domain-primitives.physics-non-claims]]
The system MUST limit physics primitives to exact discrete admission and budget predicates; it MUST NOT claim floating-point accuracy, continuous dynamics correctness, real-number differential-equation proofs, broad solver completeness, simulator fidelity, collision exactness beyond conservative interval candidates, or production physics-engine correctness.

#### Scenario: Continuous and floating-point claims are excluded [r[domain-primitives.physics-non-claims.scenario.continuous-and-floating-point-claims-are-excluded]]

- GIVEN a physics primitive result
- WHEN the result is interpreted
- THEN it is only evidence for the exact finite predicate and not for continuous or floating-point simulator correctness
## Machine Vision Domain Primitives

The following requirements define exact finite machine-vision primitives only. They MUST NOT claim model accuracy, calibration correctness, codec correctness, floating-point color science, OCR/segmentation quality, neural-network quality, or real-time video behavior.

### Requirement: Vision Image Buffer [r[domain-primitives.vision-image-buffer]]
The system MUST provide a machine-vision image-buffer primitive for checked
width, height, stride, channel, and backing-buffer layout.

#### Scenario: Valid pixel address is in bounds [r[domain-primitives.vision-image-buffer.scenario.valid-pixel-address-is-in-bounds]]

- GIVEN a valid image buffer and pixel coordinate/channel inside declared bounds
- WHEN the byte offset is computed
- THEN the offset is inside the backing buffer

#### Scenario: Bad stride or buffer length is rejected [r[domain-primitives.vision-image-buffer.scenario.bad-stride-or-buffer-length-is-rejected]]

- GIVEN an image layout whose stride or backing length cannot contain all rows
- WHEN the image-buffer constructor is evaluated
- THEN it rejects the layout

### Requirement: Vision ROI [r[domain-primitives.vision-roi]]
The system MUST provide a machine-vision ROI primitive for bounded regions of
interest and crop-coordinate mapping.

#### Scenario: ROI crop maps into source image [r[domain-primitives.vision-roi.scenario.roi-crop-maps-into-source-image]]

- GIVEN a valid image buffer and ROI inside the image bounds
- WHEN a crop-local coordinate is mapped to source coordinates
- THEN the source coordinate is inside the image buffer

#### Scenario: Out-of-image ROI is rejected [r[domain-primitives.vision-roi.scenario.out-of-image-roi-is-rejected]]

- GIVEN an ROI that extends outside the source image bounds
- WHEN the checked ROI constructor is evaluated
- THEN the ROI is rejected

### Requirement: Vision Convolution Window [r[domain-primitives.vision-convolution-window]]
The system MUST provide a machine-vision convolution-window primitive for safe
kernel indexing under declared border policy.

#### Scenario: Interior kernel samples are in bounds [r[domain-primitives.vision-convolution-window.scenario.interior-kernel-samples-are-in-bounds]]

- GIVEN a valid image buffer, kernel shape, and anchor such that the window lies inside the image
- WHEN window sample offsets are computed
- THEN every sample offset is inside the source image buffer

#### Scenario: Unsupported border sample is rejected [r[domain-primitives.vision-convolution-window.scenario.unsupported-border-sample-is-rejected]]

- GIVEN a kernel window that crosses the image boundary with a reject-border policy
- WHEN sampling is requested
- THEN the window is rejected

### Requirement: Vision Color Layout [r[domain-primitives.vision-color-layout]]
The system MUST provide a machine-vision color-layout primitive for declared
channel count, order, and per-channel byte offset admission.

#### Scenario: Declared channel offset is in bounds [r[domain-primitives.vision-color-layout.scenario.declared-channel-offset-is-in-bounds]]

- GIVEN a valid color layout and declared channel
- WHEN the channel offset is computed
- THEN the offset is inside the pixel stride

#### Scenario: Invalid channel layout is rejected [r[domain-primitives.vision-color-layout.scenario.invalid-channel-layout-is-rejected]]

- GIVEN duplicate channels, unknown channels, or channel offsets outside the pixel stride
- WHEN color-layout admission is checked
- THEN the layout is rejected

### Requirement: Vision Color Transform Envelope [r[domain-primitives.vision-color-transform-envelope]]
The system MUST provide a machine-vision color-transform-envelope primitive for
bounded integer color-space transform outputs.

#### Scenario: Transform output stays inside envelope [r[domain-primitives.vision-color-transform-envelope.scenario.transform-output-stays-inside-envelope]]

- GIVEN valid integer coefficients, input channel bounds, and output bounds
- WHEN the transform envelope is checked
- THEN every admitted output channel lies inside the declared output bounds

#### Scenario: Overflowing transform is rejected [r[domain-primitives.vision-color-transform-envelope.scenario.overflowing-transform-is-rejected]]

- GIVEN coefficients or input bounds whose transform can overflow the representation
- WHEN transform admission is checked
- THEN the transform is rejected

### Requirement: Vision Image Pyramid [r[domain-primitives.vision-image-pyramid]]
The system MUST provide a machine-vision image-pyramid primitive for bounded
multi-level image dimensions and level mapping.

#### Scenario: Pyramid levels decrease monotonically [r[domain-primitives.vision-image-pyramid.scenario.pyramid-levels-decrease-monotonically]]

- GIVEN a valid base image and pyramid scale plan
- WHEN pyramid dimensions are derived
- THEN each admitted level has positive dimensions no larger than the previous level

#### Scenario: Invalid pyramid scale is rejected [r[domain-primitives.vision-image-pyramid.scenario.invalid-pyramid-scale-is-rejected]]

- GIVEN a scale plan that yields zero dimensions or overflowing level metadata
- WHEN pyramid admission is checked
- THEN the pyramid is rejected

### Requirement: Vision Connected Components [r[domain-primitives.vision-connected-components]]
The system MUST provide a machine-vision connected-components primitive for
bounded component labels and merge/equivalence facts.

#### Scenario: Component labels stay in declared range [r[domain-primitives.vision-connected-components.scenario.component-labels-stay-in-declared-range]]

- GIVEN a valid label image and component-count bound
- WHEN labels are admitted
- THEN every non-background label is within the declared component range

#### Scenario: Invalid label or merge is rejected [r[domain-primitives.vision-connected-components.scenario.invalid-label-or-merge-is-rejected]]

- GIVEN a label outside the declared range or a merge referencing an unknown component
- WHEN component admission is checked
- THEN the label or merge is rejected

### Requirement: Vision Detection NMS [r[domain-primitives.vision-detection-nms]]
The system MUST provide a machine-vision detection/NMS primitive for deterministic
score thresholding and non-maximum suppression.

#### Scenario: Higher-scored overlapping box suppresses lower-scored box [r[domain-primitives.vision-detection-nms.scenario.higher-scored-overlapping-box-suppresses-lower-scored-box]]

- GIVEN two valid same-class boxes whose overlap exceeds the suppression threshold and whose scores are ordered
- WHEN NMS is evaluated
- THEN only the higher-scored box remains admitted

#### Scenario: Invalid detection is rejected [r[domain-primitives.vision-detection-nms.scenario.invalid-detection-is-rejected]]

- GIVEN a detection with an invalid box, unknown class, or score outside the configured range
- WHEN detection admission is checked
- THEN the detection is rejected before suppression
## Numeric Foundation Domain Primitives

The following requirements define exact finite/discrete numeric primitives only. They MUST NOT claim floating-point accuracy, probabilistic guarantees, arbitrary-precision behavior, symbolic algebra completeness, or hardware/SIMD/GPU performance properties.

### Requirement: Numeric Fixed Point Scale [r[domain-primitives.numeric-fixed-point-scale]]
The system MUST provide a numeric fixed-point scale primitive for checked scaled
integer metadata, conversion envelopes, and range preservation.

#### Scenario: Valid scaled conversion preserves envelope [r[domain-primitives.numeric-fixed-point-scale.scenario.valid-scaled-conversion-preserves-envelope]]

- GIVEN valid scale metadata and a raw integer inside the representable envelope
- WHEN the value is converted to the scaled representation
- THEN the scaled value is accepted and remains inside the declared envelope

#### Scenario: Invalid scale or conversion is rejected [r[domain-primitives.numeric-fixed-point-scale.scenario.invalid-scale-or-conversion-is-rejected]]

- GIVEN zero scale, overflowing scale metadata, or a raw value outside the representable envelope
- WHEN fixed-point admission is checked
- THEN the conversion is rejected

### Requirement: Numeric Checked Arithmetic Envelope [r[domain-primitives.numeric-checked-arithmetic-envelope]]
The system MUST provide a numeric checked-arithmetic-envelope primitive for
bounded add, subtract, multiply, and safe zero-numerator divide-admission operations.

#### Scenario: Accepted arithmetic result stays within bounds [r[domain-primitives.numeric-checked-arithmetic-envelope.scenario.accepted-arithmetic-result-stays-within-bounds]]

- GIVEN operands and an operation whose exact result is representable and inside the declared bounds
- WHEN checked arithmetic is evaluated
- THEN the result is accepted and lies inside the bounds

#### Scenario: Overflowing or invalid arithmetic is rejected [r[domain-primitives.numeric-checked-arithmetic-envelope.scenario.overflowing-or-invalid-arithmetic-is-rejected]]

- GIVEN operands whose operation overflows, leaves the declared bounds, divides by zero, or is outside the safe finite divide-admission surface
- WHEN checked arithmetic is evaluated
- THEN the operation is rejected

### Requirement: Numeric Denominator Safe Ratio [r[domain-primitives.numeric-denominator-safe-ratio]]
The system MUST provide a numeric denominator-safe-ratio primitive for explicit
non-zero denominator admission and empty/reject semantics.

#### Scenario: Non-zero denominator yields numeric ratio [r[domain-primitives.numeric-denominator-safe-ratio.scenario.non-zero-denominator-yields-numeric-ratio]]

- GIVEN a numerator and non-zero denominator whose ratio is representable
- WHEN ratio derivation is requested
- THEN a numeric ratio result is returned

#### Scenario: Zero denominator does not produce numeric ratio [r[domain-primitives.numeric-denominator-safe-ratio.scenario.zero-denominator-does-not-produce-numeric-ratio]]

- GIVEN a zero denominator
- WHEN ratio derivation is requested
- THEN the primitive rejects the request or returns an explicit empty result instead of a numeric ratio

### Requirement: Numeric Monotone Sequence [r[domain-primitives.numeric-monotone-sequence]]
The system MUST provide a numeric monotone-sequence primitive for sortedness,
lower-bound search, and range-query safety over finite three-slot sequences.

#### Scenario: Bound search returns in-range partition point [r[domain-primitives.numeric-monotone-sequence.scenario.bound-search-returns-in-range-partition-point]]

- GIVEN a finite sorted sequence and a query value
- WHEN lower-bound search is evaluated
- THEN the returned index is inside `0..=len` and partitions the sequence according to the query predicate

#### Scenario: Unsorted sequence is rejected [r[domain-primitives.numeric-monotone-sequence.scenario.unsorted-sequence-is-rejected]]

- GIVEN a sequence with an out-of-order adjacent pair
- WHEN monotone sequence admission is checked
- THEN the sequence is rejected

### Requirement: Numeric Histogram Counter [r[domain-primitives.numeric-histogram-counter]]
The system MUST provide a numeric histogram-counter primitive for bounded three-bucket indexing and count-update admission.

#### Scenario: Valid bucket update increments one count [r[domain-primitives.numeric-histogram-counter.scenario.valid-bucket-update-increments-one-count]]

- GIVEN a valid histogram and a value admitted to exactly one bucket
- WHEN the histogram is updated
- THEN exactly that bucket count increases and all other bucket counts are unchanged

#### Scenario: Invalid bucket or overflowing count is rejected [r[domain-primitives.numeric-histogram-counter.scenario.invalid-bucket-or-overflowing-count-is-rejected]]

- GIVEN a value outside all buckets or a bucket count at its maximum
- WHEN histogram update is requested
- THEN the update is rejected

### Requirement: Numeric Affine Transform Discrete [r[domain-primitives.numeric-affine-transform-discrete]]
The system MUST provide a numeric affine-transform primitive for bounded integer
matrix/vector transform envelopes.

#### Scenario: Transform output remains inside declared envelope [r[domain-primitives.numeric-affine-transform-discrete.scenario.transform-output-remains-inside-declared-envelope]]

- GIVEN valid integer coefficients, input bounds, and output bounds
- WHEN the affine transform is admitted
- THEN every produced output component lies inside the declared output envelope

#### Scenario: Overflowing transform envelope is rejected [r[domain-primitives.numeric-affine-transform-discrete.scenario.overflowing-transform-envelope-is-rejected]]

- GIVEN coefficients, offsets, or input bounds whose transform can overflow or leave the declared output envelope
- WHEN affine transform admission is checked
- THEN the transform is rejected

### Requirement: Numeric Prefix Sum [r[domain-primitives.numeric-prefix-sum]]
The system MUST provide a numeric prefix-sum primitive for bounded three-slot cumulative sums and range-sum admission.

#### Scenario: Prefix sums decompose range sum [r[domain-primitives.numeric-prefix-sum.scenario.prefix-sums-decompose-range-sum]]

- GIVEN a finite three-slot prefix tuple whose cumulative sums are representable
- WHEN a valid range sum is requested
- THEN the range sum equals the difference of the corresponding prefix sums

#### Scenario: Prefix sum overflow is rejected [r[domain-primitives.numeric-prefix-sum.scenario.prefix-sum-overflow-is-rejected]]

- GIVEN a sequence whose cumulative sum would overflow the representation
- WHEN prefix-sum construction is requested
- THEN the construction is rejected

### Requirement: Numeric Quantized Threshold [r[domain-primitives.numeric-quantized-threshold]]
The system MUST provide a numeric quantized-threshold primitive for deterministic
threshold and bucket admission over bounded values.

#### Scenario: Value maps to deterministic quantized bucket [r[domain-primitives.numeric-quantized-threshold.scenario.value-maps-to-deterministic-quantized-bucket]]

- GIVEN valid ordered three-bucket thresholds and a value inside the supported range
- WHEN quantization is evaluated
- THEN the value maps to exactly one deterministic bucket

#### Scenario: Invalid thresholds are rejected [r[domain-primitives.numeric-quantized-threshold.scenario.invalid-thresholds-are-rejected]]

- GIVEN unordered three-bucket thresholds, duplicate thresholds where disallowed, or a value outside the supported range
- WHEN threshold admission is checked
- THEN the request is rejected
## Data Structure Domain Primitives

The following requirements define exact finite data-structure primitives only. They MUST NOT claim concurrency, probabilistic hash quality, allocator behavior, asymptotic complexity, or hardware performance properties.

### Requirement: Data Structure Bounded Array [r[domain-primitives.ds-bounded-array]]
The system MUST provide a bounded-array primitive for finite sequence length,
capacity, index admission, and point-update preservation.

#### Scenario: Valid point update changes only the target index [r[domain-primitives.ds-bounded-array.scenario.valid-point-update-changes-only-target-index]]

- GIVEN a valid bounded array and an index inside its current length
- WHEN a point update is applied
- THEN the resulting array has the same length, stores the new value at that index, and preserves every other index

#### Scenario: Out-of-range index is rejected without mutation [r[domain-primitives.ds-bounded-array.scenario.out-of-range-index-is-rejected-without-mutation]]

- GIVEN a valid bounded array and an index outside its current length
- WHEN a read or point update is requested
- THEN the request is rejected and the array state is unchanged

### Requirement: Data Structure Key Value Map [r[domain-primitives.ds-key-value-map]]
The system MUST provide a finite key/value map primitive for unique keys,
lookup, insertion, replacement, and removal semantics.

#### Scenario: Insert creates or replaces exactly one key [r[domain-primitives.ds-key-value-map.scenario.insert-creates-or-replaces-exactly-one-key]]

- GIVEN a valid finite map and a key/value pair admitted by the map domain
- WHEN the pair is inserted
- THEN lookup for that key returns the inserted value and all other keys keep their previous lookup results

#### Scenario: Remove clears only the target key [r[domain-primitives.ds-key-value-map.scenario.remove-clears-only-target-key]]

- GIVEN a valid finite map and a target key
- WHEN the target key is removed
- THEN lookup for that key is empty and lookup for every other key is unchanged

### Requirement: Data Structure FIFO Queue [r[domain-primitives.ds-fifo-queue]]
The system MUST provide a bounded FIFO queue primitive for enqueue/dequeue order,
capacity admission, and empty/full rejection.

#### Scenario: Dequeue returns oldest enqueued element [r[domain-primitives.ds-fifo-queue.scenario.dequeue-returns-oldest-enqueued-element]]

- GIVEN a valid non-empty FIFO queue built from accepted enqueue operations
- WHEN one element is dequeued
- THEN the returned element is the oldest remaining element and the queue retains the remaining elements in FIFO order

#### Scenario: Queue overflow or underflow is rejected [r[domain-primitives.ds-fifo-queue.scenario.queue-overflow-or-underflow-is-rejected]]

- GIVEN a full queue for enqueue or an empty queue for dequeue
- WHEN the invalid operation is requested
- THEN the operation is rejected and the queue state is unchanged

### Requirement: Data Structure Ring Buffer [r[domain-primitives.ds-ring-buffer]]
The system MUST provide a ring-buffer primitive for bounded circular indexing,
occupied-slot validity, and wraparound push/pop behavior.

#### Scenario: Wraparound preserves logical order [r[domain-primitives.ds-ring-buffer.scenario.wraparound-preserves-logical-order]]

- GIVEN a valid ring buffer whose head or tail is near the physical end of storage
- WHEN accepted push and pop operations wrap around storage
- THEN logical element order is preserved and every occupied logical index maps to a valid physical slot

#### Scenario: Full or empty ring operation is rejected [r[domain-primitives.ds-ring-buffer.scenario.full-or-empty-ring-operation-is-rejected]]

- GIVEN a full ring buffer for push or an empty ring buffer for pop
- WHEN the invalid operation is requested
- THEN the operation is rejected and head, tail, length, and storage are unchanged

### Requirement: Data Structure Priority Queue [r[domain-primitives.ds-priority-queue]]
The system MUST provide a bounded priority-queue primitive for priority-order
validity, deterministic tie handling, bounded push, and pop-min behavior.

#### Scenario: Pop returns deterministic minimum-priority element [r[domain-primitives.ds-priority-queue.scenario.pop-returns-deterministic-minimum-priority-element]]

- GIVEN a valid non-empty priority queue and a declared deterministic tie policy
- WHEN pop-min is requested
- THEN the returned element has minimum priority and satisfies the tie policy among all minimum-priority elements

#### Scenario: Invalid priority queue operation is rejected [r[domain-primitives.ds-priority-queue.scenario.invalid-priority-queue-operation-is-rejected]]

- GIVEN a full priority queue for push or an empty priority queue for pop
- WHEN the invalid operation is requested
- THEN the operation is rejected and the queue state is unchanged

### Requirement: Data Structure Union Find [r[domain-primitives.ds-union-find]]
The system MUST provide a union-find primitive for finite parent/rank arrays,
root discovery, union connectivity, and invalid-index rejection.

#### Scenario: Union connects two valid sets [r[domain-primitives.ds-union-find.scenario.union-connects-two-valid-sets]]

- GIVEN a valid union-find forest and two valid element indices
- WHEN the sets containing those elements are unioned
- THEN the resulting forest remains valid and the two elements have the same representative root

#### Scenario: Invalid union-find index is rejected [r[domain-primitives.ds-union-find.scenario.invalid-union-find-index-is-rejected]]

- GIVEN a valid union-find forest and an index outside the element range
- WHEN find or union is requested with that index
- THEN the operation is rejected and the forest state is unchanged
## Access-Control Policy Domain Primitives

The following requirements define exact finite access-control policy primitives only. They MUST NOT claim cryptographic authentication, key ownership, whole-system security, information-flow security, side-channel resistance, external policy-engine correctness, distributed authorization consensus, or runtime enforcement guarantees.

### Requirement: Finite Principal Envelope [r[domain-primitives.access-control-policy.principal]]
The system MUST define finite principal admission metadata for policy decision
models.

#### Scenario: Principal is declared before rule use [r[domain-primitives.access-control-policy.principal.scenario.declared-before-rule-use]]

- GIVEN finite principal metadata and a policy rule
- WHEN the rule references a principal
- THEN the principal is admitted by the declared principal envelope

### Requirement: Finite Resource Envelope [r[domain-primitives.access-control-policy.resource]]
The system MUST define finite resource admission metadata for policy decision
models.

#### Scenario: Resource is declared before authorization [r[domain-primitives.access-control-policy.resource.scenario.declared-before-authorization]]

- GIVEN finite resource metadata and an authorization request
- WHEN the request references a resource
- THEN the resource is admitted by the declared resource envelope

### Requirement: Finite Action Envelope [r[domain-primitives.access-control-policy.action]]
The system MUST define finite action admission metadata for policy decision
models.

#### Scenario: Action is declared before authorization [r[domain-primitives.access-control-policy.action.scenario.declared-before-authorization]]

- GIVEN finite action metadata and an authorization request
- WHEN the request references an action
- THEN the action is admitted by the declared action envelope

### Requirement: Policy Rule Envelope [r[domain-primitives.access-control-policy.rule]]
The system MUST define policy-rule envelopes over finite principal, resource, and
action scopes with exact effect metadata.

#### Scenario: Rule scope uses admitted values [r[domain-primitives.access-control-policy.rule.scenario.scope-uses-admitted-values]]

- GIVEN a policy-rule envelope
- WHEN the rule scope is evaluated
- THEN every principal, resource, and action referenced by the rule is admitted by its declared envelope

### Requirement: Grant and Denial Decision Metadata [r[domain-primitives.access-control-policy.grant-denial]]
The system MUST define deterministic grant/denial decision metadata for admitted
authorization requests.

#### Scenario: Decision records matched rule metadata [r[domain-primitives.access-control-policy.grant-denial.scenario.records-matched-rule-metadata]]

- GIVEN an admitted authorization request and finite policy rules
- WHEN a grant or denial decision is produced
- THEN the decision metadata identifies the matched admitted rule or the deterministic no-match result

### Requirement: Conflict Resolution Envelope [r[domain-primitives.access-control-policy.conflict]]
The system MUST define deterministic conflict-resolution envelopes for overlapping
finite policy rules.

#### Scenario: Overlapping rules resolve deterministically [r[domain-primitives.access-control-policy.conflict.scenario.overlapping-rules-resolve-deterministically]]

- GIVEN overlapping admitted policy rules for the same principal, resource, and action
- WHEN conflict resolution is evaluated
- THEN the result follows declared priority or tie-break metadata deterministically

### Requirement: Bounded Delegation Envelope [r[domain-primitives.access-control-policy.delegation]]
The system MUST define bounded delegation metadata over finite principals,
resources, actions, scopes, and chain-depth envelopes.

#### Scenario: Delegation states scope and bound [r[domain-primitives.access-control-policy.delegation.scenario.states-scope-and-bound]]

- GIVEN delegation metadata between admitted principals
- WHEN the delegation is used in an authorization model
- THEN it states the admitted resource/action scope and finite depth or chain bound

### Requirement: Authorization Admission Envelope [r[domain-primitives.access-control-policy.authorization]]
The system MUST define deterministic authorization admission results for policy
metadata, including invalid principal, resource, action, rule, conflict, and
delegation shapes.

#### Scenario: Rejects undeclared principal deterministically [r[domain-primitives.access-control-policy.authorization.scenario.rejects-undeclared-principal-deterministically]]

- GIVEN authorization metadata with a request principal outside the declared principal envelope
- WHEN admission is evaluated
- THEN the metadata is rejected deterministically without claiming cryptographic authentication or whole-system security

## Configuration Feature Flag Domain Primitives

r[domain-primitives.configuration-feature-flag.key]
Finite config-key envelopes MUST record key identifiers, optional namespaces,
and deterministic known/unknown-key admission metadata without consulting an
external store.

r[domain-primitives.configuration-feature-flag.typed-value]
Typed-value envelopes MUST bind declared type tags to admitted finite value
shapes and reject type/value mismatches deterministically.

r[domain-primitives.configuration-feature-flag.default]
Default-value metadata MUST tie each default to a declared finite key envelope
and compatible typed-value envelope.

r[domain-primitives.configuration-feature-flag.override-precedence]
Override-precedence envelopes MUST select among finite admitted sources
deterministically and reject duplicate-source or tied-precedence metadata.

r[domain-primitives.configuration-feature-flag.gate]
Feature-gate predicate envelopes MUST map admitted keys, subjects, and finite
metadata to explicit enabled, disabled, or rejected decisions.

r[domain-primitives.configuration-feature-flag.rollout-bucket]
Rollout-bucket envelopes MUST admit finite subject buckets only when the bucket
range is declared and in bounds, without claiming randomness quality or
distributed rollout consistency.

r[domain-primitives.configuration-feature-flag.validation-decision]
Validation-decision metadata MUST record accepted entries and reason-coded
rejections over finite admitted configuration metadata without mutating inputs.

r[domain-primitives.configuration-feature-flag.admission]
Configuration admission envelopes MUST compose key, typed-value, default,
override, gate, rollout, and validation checks and reject when any required
finite check rejects.

## Constraint Solver Domain Primitives

r[domain-primitives.constraint-solver.variable]
Constraint-variable envelopes MUST admit only finite variable identifiers whose
declared domain identifier exists in the finite domain table.

r[domain-primitives.constraint-solver.domain]
Constraint-domain predicates MUST model finite value membership, emptiness, and
subset checks without claiming theory reasoning.

r[domain-primitives.constraint-solver.relation]
Constraint-relation envelopes MUST admit only finite tuples whose arity matches
the declared relation scope and relation table metadata.

r[domain-primitives.constraint-solver.assignment]
Constraint-assignment predicates MUST model partial/total assignment shape,
variable uniqueness, and value admission against declared finite domains.

r[domain-primitives.constraint-solver.consistency]
Constraint-consistency predicates MUST deterministically check assigned relation
scopes against finite relation tuples.

r[domain-primitives.constraint-solver.propagation]
Constraint-propagation envelopes MUST admit only monotone domain narrowing and
explicit contradiction metadata.

r[domain-primitives.constraint-solver.witness]
Constraint-witness metadata MUST distinguish satisfying and contradiction
witnesses over declared finite models without claiming solver search completeness.

r[domain-primitives.constraint-solver.admission]
Constraint-admission envelopes MUST compose finite variables, domains, relations,
assignments, propagation, and witness predicates into deterministic admission or
rejection metadata.

## Control-System Domain Primitives

r[domain-primitives.control-setpoint]
Control setpoint primitives MUST admit bounded target values, deterministic
setpoint updates, and invalid-bound rejection over exact finite integer values.

r[domain-primitives.control-error-signal]
Control error-signal primitives MUST derive checked target-minus-measurement
deltas, reject overflowing subtraction, and require errors inside declared
envelopes.

r[domain-primitives.control-actuator-saturation]
Control actuator-saturation primitives MUST clamp commands into valid actuator
bounds and expose deterministic saturation metadata without overdrive claims.

r[domain-primitives.control-deadband]
Control deadband primitives MUST use explicit inclusive finite thresholds for
deterministic no-action/action decisions and malformed-band rejection.

r[domain-primitives.control-hysteresis]
Control hysteresis primitives MUST use ordered finite thresholds, two-state mode
transitions, and hold-band state preservation.

r[domain-primitives.control-rate-limit]
Control rate-limit primitives MUST reject malformed or overflowing step bounds
and admit only outputs within the previous value plus/minus the maximum step.

r[domain-primitives.control-safety-envelope]
Control safety-envelope primitives MUST check finite state/input bounds and
reject envelope violations without claiming real-world physical safety.

## Data Validation Domain Primitives

r[domain-primitives.data-validation.schema-predicate]
Data-validation schema-predicate primitives MUST admit only declared finite
predicate identifiers and reject unknown predicates explicitly.

r[domain-primitives.data-validation.field-presence]
Data-validation field-presence primitives MUST distinguish missing required
fields, optional absence, present fields, and duplicate fields without mutating
candidate records.

r[domain-primitives.data-validation.enum-constraint]
Data-validation enum-constraint primitives MUST admit only values contained in a
declared finite member set.

r[domain-primitives.data-validation.range-constraint]
Data-validation range-constraint primitives MUST admit only scalar values inside
valid inclusive finite bounds.

r[domain-primitives.data-validation.cross-field]
Data-validation cross-field primitives MUST return satisfied, unsatisfied, or
inapplicable decisions over finite dependency metadata.

r[domain-primitives.data-validation.error-accumulation]
Data-validation error-accumulation primitives MUST preserve deterministic finite
error ordering and capacity metadata.

r[domain-primitives.data-validation.normalization-decision]
Data-validation normalization primitives MUST record unchanged, transformed, or
rejected decisions without claiming semantic repair correctness.

r[domain-primitives.data-validation.admission]
Data-validation admission primitives MUST compose finite schema, field, enum,
range, cross-field, error, and normalization metadata into deterministic
admission or rejection.

## Discrete Optimization Domain Primitives

r[domain-primitives.discrete-optimization.objective]
Discrete-optimization objective primitives MUST associate exact finite scores
only with candidate IDs admitted by the declared finite universe.

r[domain-primitives.discrete-optimization.feasible-set]
Discrete-optimization feasible-set primitives MUST admit or reject candidate IDs
against a finite declared candidate envelope.

r[domain-primitives.discrete-optimization.ranking]
Discrete-optimization ranking primitives MUST compare only admitted candidates
and use deterministic exact-score ordering metadata.

r[domain-primitives.discrete-optimization.dominance]
Discrete-optimization dominance primitives MUST name distinct compared
candidates and require exact no-worse-plus-strictly-better tuple metadata.

r[domain-primitives.discrete-optimization.bound]
Discrete-optimization bound primitives MUST validate lower/upper objective-bound
metadata against exact finite scores.

r[domain-primitives.discrete-optimization.witness]
Discrete-optimization optimum-witness primitives MUST reference an admitted
candidate and finite-scope bestness metadata only.

r[domain-primitives.discrete-optimization.frontier]
Discrete-optimization frontier primitives MUST mark only admitted nondominated
candidates as finite frontier members.

r[domain-primitives.discrete-optimization.admission]
Discrete-optimization admission primitives MUST compose objective, feasible-set,
ranking, dominance, bound, witness, and frontier metadata into deterministic
admission or rejection.

### Requirement: Path Segment Envelope [r[domain-primitives.filesystem-path.segment]]
The system MUST provide finite path-segment envelopes that record segment text or
bytes, separator policy metadata, empty/current/parent segment classifiers, and
deterministic segment admission results.

#### Scenario: Rejected segment is explicit [r[domain-primitives.filesystem-path.segment.scenario.rejected-segment-is-explicit]]

- GIVEN finite path-segment admission metadata
- WHEN a candidate segment is empty, forbidden, or outside the declared segment envelope
- THEN the path-segment primitive returns an explicit rejected-segment decision without consulting an OS filesystem

### Requirement: Path Form Envelope [r[domain-primitives.filesystem-path.form]]
The system MUST provide finite path-form envelopes for absolute, relative,
rooted, empty, and scoped path forms over declared segment sequences.

#### Scenario: Path form is admitted before containment [r[domain-primitives.filesystem-path.form.scenario.path-form-is-admitted-before-containment]]

- GIVEN finite path form metadata and a candidate segment sequence
- WHEN the candidate form is not admitted by the envelope
- THEN the path-form primitive rejects it before containment or lookup decisions are composed

### Requirement: Path Normalization Decision [r[domain-primitives.filesystem-path.normalization]]
The system MUST provide deterministic path-normalization decision envelopes that
record accepted, rejected, unchanged, or rewritten finite segment sequences under
declared normalization rules.

#### Scenario: Normalization rejection preserves original segments [r[domain-primitives.filesystem-path.normalization.scenario.normalization-rejection-preserves-original-segments]]

- GIVEN a finite candidate path and normalization metadata
- WHEN normalization is not admitted
- THEN the normalization primitive records rejection metadata while preserving the original segment sequence

### Requirement: Root Scope Containment Predicate [r[domain-primitives.filesystem-path.containment]]
The system MUST provide finite root/scope containment predicates over declared
root segment prefixes and candidate normalized segment sequences.

#### Scenario: Candidate outside root is rejected [r[domain-primitives.filesystem-path.containment.scenario.candidate-outside-root-is-rejected]]

- GIVEN a finite root scope and candidate normalized path segments
- WHEN the candidate does not have the declared root/scope prefix
- THEN the containment primitive returns an explicit outside-scope decision without claiming sandbox security

### Requirement: Relative Traversal Rejection [r[domain-primitives.filesystem-path.traversal]]
The system MUST provide deterministic relative traversal rejection metadata for
finite path segments that represent parent/current traversal under declared
segment classifier rules.

#### Scenario: Parent traversal is rejected by policy [r[domain-primitives.filesystem-path.traversal.scenario.parent-traversal-is-rejected-by-policy]]

- GIVEN finite path segments and a policy rejecting parent traversal segments
- WHEN a parent traversal segment appears before normalization or admission completes
- THEN the traversal primitive returns explicit rejection metadata without claiming exploit prevention beyond the declared finite predicate

### Requirement: Extension Type Metadata [r[domain-primitives.filesystem-path.extension-type]]
The system MUST provide finite extension/type metadata envelopes that classify
admitted final segments by declared suffix, extension, or type tags without
claiming MIME, content-sniffing, or OS file-type correctness.

#### Scenario: Extension metadata does not imply file contents [r[domain-primitives.filesystem-path.extension-type.scenario.extension-metadata-does-not-imply-file-contents]]

- GIVEN a finite path with admitted extension metadata
- WHEN the extension/type primitive classifies the final segment
- THEN the result records only declared segment metadata and does not claim runtime file contents or MIME correctness

### Requirement: Path Join Split Envelope [r[domain-primitives.filesystem-path.join-split]]
The system MUST provide deterministic path join/split shape envelopes over
finite segment sequences and separator metadata.

#### Scenario: Join split preserves admitted segment order [r[domain-primitives.filesystem-path.join-split.scenario.join-split-preserves-admitted-segment-order]]

- GIVEN finite admitted segment sequences and separator metadata
- WHEN a path join or split shape operation is modeled by the primitive
- THEN the resulting envelope records deterministic segment order without consulting host path APIs

### Requirement: Path Admission Envelope [r[domain-primitives.filesystem-path.admission]]
The system MUST provide deterministic path admission/rejection envelopes that
compose segment, form, normalization, containment, traversal, extension/type, and
join/split decisions over finite path metadata.

#### Scenario: Admission requires all finite path checks [r[domain-primitives.filesystem-path.admission.scenario.admission-requires-all-finite-path-checks]]

- GIVEN finite path metadata and a candidate path
- WHEN any segment, form, normalization, containment, traversal, extension/type, or join/split check rejects
- THEN the path-admission primitive returns an explicit rejection decision before any OS filesystem or security claim is made

### Requirement: Point Vector Envelope [r[domain-primitives.geometry-foundation.point-vector]]
The system MUST provide finite point/vector envelopes that record coordinate
count, coordinate bounds, scale metadata, dimension tags, and deterministic
point/vector admission results.

#### Scenario: Dimension mismatch is rejected explicitly [r[domain-primitives.geometry-foundation.point-vector.scenario.dimension-mismatch-is-rejected-explicitly]]

- GIVEN finite point/vector metadata and a declared dimension
- WHEN a candidate point/vector has a different coordinate count
- THEN the point/vector primitive returns explicit dimension-mismatch rejection metadata

### Requirement: Axis-Aligned Bounds Envelope [r[domain-primitives.geometry-foundation.bounds]]
The system MUST provide finite axis-aligned bounds and extent envelopes that
record lower/upper coordinate bounds, emptiness metadata, extent metadata, and
deterministic bounds admission results.

#### Scenario: Inverted bound is rejected [r[domain-primitives.geometry-foundation.bounds.scenario.inverted-bound-is-rejected]]

- GIVEN finite axis-aligned bounds metadata
- WHEN any lower coordinate is greater than its upper coordinate under the declared order
- THEN the bounds primitive returns explicit inverted-bound rejection metadata

### Requirement: Orientation Metadata [r[domain-primitives.geometry-foundation.orientation]]
The system MUST provide finite orientation/ordering metadata for declared point
or vector tuples under exact finite arithmetic envelopes.

#### Scenario: Degenerate orientation is explicit [r[domain-primitives.geometry-foundation.orientation.scenario.degenerate-orientation-is-explicit]]

- GIVEN finite orientation metadata for a declared tuple
- WHEN the tuple is degenerate under the exact finite predicate
- THEN the orientation primitive records explicit degenerate metadata rather than inventing a strict orientation

### Requirement: Containment Predicate Envelope [r[domain-primitives.geometry-foundation.containment]]
The system MUST provide finite containment predicate envelopes for point-in-bounds,
bounds-in-bounds, and candidate-within-extent checks over admitted finite
geometry metadata.

#### Scenario: Outside point is rejected by containment [r[domain-primitives.geometry-foundation.containment.scenario.outside-point-is-rejected-by-containment]]

- GIVEN finite point and bounds metadata
- WHEN the point is outside the admitted bounds under declared inclusive/exclusive edge policy
- THEN the containment primitive returns explicit outside metadata

### Requirement: Intersection Touch Metadata [r[domain-primitives.geometry-foundation.intersection]]
The system MUST provide finite intersection/touch metadata envelopes for admitted
axis-aligned bounds and finite primitive shapes without claiming complete
computational-geometry behavior.

#### Scenario: Disjoint bounds produce no-intersection metadata [r[domain-primitives.geometry-foundation.intersection.scenario.disjoint-bounds-produce-no-intersection-metadata]]

- GIVEN two finite admitted axis-aligned bounds envelopes
- WHEN the bounds are disjoint under declared edge policy
- THEN the intersection primitive records explicit no-intersection metadata

### Requirement: Transform Shape Envelope [r[domain-primitives.geometry-foundation.transform]]
The system MUST provide finite transform-shape envelopes for translation,
scaling, permutation, and affine-like metadata under declared exact arithmetic
bounds.

#### Scenario: Transform overflow is rejected [r[domain-primitives.geometry-foundation.transform.scenario.transform-overflow-is-rejected]]

- GIVEN finite transform metadata and coordinate bounds
- WHEN applying the declared transform would exceed the output envelope
- THEN the transform primitive returns explicit overflow rejection metadata

### Requirement: Grid Cell Envelope [r[domain-primitives.geometry-foundation.grid-cell]]
The system MUST provide finite grid-cell and index envelopes that record origin,
cell size metadata, grid extents, cell coordinates, and deterministic grid-cell
admission results.

#### Scenario: Cell index must fit grid extent [r[domain-primitives.geometry-foundation.grid-cell.scenario.cell-index-must-fit-grid-extent]]

- GIVEN finite grid-cell metadata and declared grid extents
- WHEN a candidate cell index is outside the grid extent
- THEN the grid-cell primitive returns explicit out-of-grid rejection metadata

### Requirement: Geometry Admission Envelope [r[domain-primitives.geometry-foundation.admission]]
The system MUST provide deterministic geometry admission/rejection envelopes that
compose point/vector, bounds, orientation, containment, intersection, transform,
and grid-cell metadata over finite geometry data.

#### Scenario: Admission requires all finite geometry checks [r[domain-primitives.geometry-foundation.admission.scenario.admission-requires-all-finite-geometry-checks]]

- GIVEN finite geometry metadata and a candidate geometry envelope
- WHEN any point/vector, bounds, orientation, containment, intersection, transform, or grid-cell check rejects
- THEN the geometry-admission primitive returns an explicit rejection before any floating-point, topology, rendering, physics, or external geometry claim is made

### Requirement: Graph Adjacency [r[domain-primitives.graph-adjacency]]
The system MUST provide a finite graph-adjacency primitive for node-count
validity, edge endpoint admission, and adjacency lookup.

#### Scenario: Valid edge is admitted into adjacency [r[domain-primitives.graph-adjacency.scenario.valid-edge-is-admitted-into-adjacency]]

- GIVEN a finite node count and an edge whose source and destination are valid node indices
- WHEN adjacency is constructed or queried
- THEN the edge is accepted and appears only in the source node adjacency relation

#### Scenario: Malformed edge is rejected [r[domain-primitives.graph-adjacency.scenario.malformed-edge-is-rejected]]

- GIVEN an edge with a source or destination outside the finite node range
- WHEN graph adjacency admission is checked
- THEN the edge is rejected and the graph state is unchanged

### Requirement: Graph Reachability [r[domain-primitives.graph-reachability]]
The system MUST provide a finite graph-reachability primitive for path-witness
validity, reachability extension, and explicit unreachable results.

#### Scenario: Valid path witness proves reachability [r[domain-primitives.graph-reachability.scenario.valid-path-witness-proves-reachability]]

- GIVEN a valid finite graph and a path witness whose adjacent nodes follow admitted edges
- WHEN reachability for the path endpoints is checked
- THEN the destination is reachable from the source

#### Scenario: Invalid or absent path yields explicit non-reachability result [r[domain-primitives.graph-reachability.scenario.invalid-or-absent-path-yields-explicit-non-reachability-result]]

- GIVEN invalid endpoints, a malformed path witness, or a bounded search that finds no path
- WHEN reachability is checked
- THEN the primitive rejects the request or returns an explicit unreachable/empty result

### Requirement: Graph Topological Order [r[domain-primitives.graph-topological-order]]
The system MUST provide a graph topological-order primitive for DAG admission,
permutation validity, and edge-forward ordering.

#### Scenario: Valid topological order places every edge forward [r[domain-primitives.graph-topological-order.scenario.valid-topological-order-places-every-edge-forward]]

- GIVEN a finite directed graph and an order containing every node exactly once
- WHEN topological-order validity is checked
- THEN every admitted edge points from an earlier node to a later node in the order

#### Scenario: Cycle or malformed order is rejected [r[domain-primitives.graph-topological-order.scenario.cycle-or-malformed-order-is-rejected]]

- GIVEN a graph with a directed cycle, duplicate order entries, missing nodes, or out-of-range nodes
- WHEN topological-order admission is checked
- THEN the order is rejected

### Requirement: Graph Component Partition [r[domain-primitives.graph-component-partition]]
The system MUST provide an undirected component-partition primitive for complete,
disjoint grouping of finite graph nodes by connectivity.

#### Scenario: Partition covers each node exactly once [r[domain-primitives.graph-component-partition.scenario.partition-covers-each-node-exactly-once]]

- GIVEN a valid finite undirected graph and a component partition
- WHEN partition validity is checked
- THEN every node appears in exactly one component and all nodes inside a component are connected

#### Scenario: Invalid component partition is rejected [r[domain-primitives.graph-component-partition.scenario.invalid-component-partition-is-rejected]]

- GIVEN a partition with missing nodes, duplicate nodes, out-of-range nodes, or a component containing disconnected nodes
- WHEN component-partition admission is checked
- THEN the partition is rejected

### Requirement: Graph Shortest Path Envelope [r[domain-primitives.graph-shortest-path-envelope]]
The system MUST provide a bounded shortest-path-envelope primitive for
non-negative weights, path-cost accumulation, and candidate-cost comparison.

#### Scenario: Candidate path cost is bounded and comparable [r[domain-primitives.graph-shortest-path-envelope.scenario.candidate-path-cost-is-bounded-and-comparable]]

- GIVEN a valid finite weighted graph with non-negative bounded edge weights and a valid path witness
- WHEN the path cost is accumulated
- THEN the resulting cost is accepted, bounded, and comparable against other accepted candidate costs

#### Scenario: Invalid or overflowing path cost is rejected [r[domain-primitives.graph-shortest-path-envelope.scenario.invalid-or-overflowing-path-cost-is-rejected]]

- GIVEN a negative weight, malformed edge, invalid path witness, or path whose accumulated cost would overflow the bound
- WHEN path-cost admission is checked
- THEN the cost request is rejected

### Requirement: Graph Frontier Cut [r[domain-primitives.graph-frontier-cut]]
The system MUST provide a graph frontier/cut primitive for traversal state,
visited/frontier disjointness, and expansion preservation.

#### Scenario: Valid expansion preserves visited frontier cut invariant [r[domain-primitives.graph-frontier-cut.scenario.valid-expansion-preserves-visited-frontier-cut-invariant]]

- GIVEN a valid traversal state with disjoint visited and frontier sets
- WHEN one valid frontier node is expanded through admitted outgoing edges
- THEN the resulting visited and frontier sets remain within the node range and preserve the declared cut invariant

#### Scenario: Invalid frontier expansion is rejected without mutation [r[domain-primitives.graph-frontier-cut.scenario.invalid-frontier-expansion-is-rejected-without-mutation]]

- GIVEN a traversal state with an out-of-range frontier node, overlapping visited/frontier sets, or malformed outgoing edge
- WHEN expansion is requested
- THEN the operation is rejected and the traversal state is unchanged

### Requirement: Identifier Envelope [r[domain-primitives.identifier-namespace.identifier]]
The system MUST provide finite identifier envelopes that record identifier bytes
or symbols, declared kind metadata, and deterministic ID-shape admission results.

#### Scenario: Malformed identifier rejected explicitly [r[domain-primitives.identifier-namespace.identifier.scenario.malformed-identifier-rejected-explicitly]]

- GIVEN finite identifier shape metadata
- WHEN a candidate identifier does not match the admitted shape
- THEN the identifier primitive returns an explicit malformed-ID decision without consulting an external identity service

### Requirement: Namespace Scope Envelope [r[domain-primitives.identifier-namespace.namespace]]
The system MUST provide finite namespace scope envelopes that define admitted
scope identifiers, parent/child scope metadata, and deterministic namespace
membership decisions.

#### Scenario: Unknown namespace rejected before lookup [r[domain-primitives.identifier-namespace.namespace.scenario.unknown-namespace-rejected-before-lookup]]

- GIVEN a finite admitted namespace set
- WHEN a candidate namespace is absent from that set
- THEN the namespace primitive rejects the namespace before any lookup or registration decision is made

### Requirement: Scoped Uniqueness Predicate [r[domain-primitives.identifier-namespace.scoped-uniqueness]]
The system MUST provide scoped uniqueness predicates over finite registry
snapshots and declared namespace scopes, returning explicit unique or duplicate
decisions.

#### Scenario: Duplicate only applies within declared scope [r[domain-primitives.identifier-namespace.scoped-uniqueness.scenario.duplicate-only-applies-within-declared-scope]]

- GIVEN two finite registry entries and a declared namespace scope
- WHEN the entries share an identifier inside that same scope
- THEN the scoped-uniqueness primitive returns a duplicate decision for that scope without claiming global uniqueness

### Requirement: Alias Map Envelope [r[domain-primitives.identifier-namespace.alias-map]]
The system MUST provide finite alias-map envelopes that map admitted aliases to
admitted canonical identifiers and expose explicit missing-target or cycle
metadata.

#### Scenario: Alias target must be admitted [r[domain-primitives.identifier-namespace.alias-map.scenario.alias-target-must-be-admitted]]

- GIVEN finite alias metadata and admitted canonical identifiers
- WHEN an alias points to a target outside the admitted identifier set
- THEN the alias-map primitive rejects the alias or records explicit missing-target metadata

### Requirement: Canonicalization Decision Envelope [r[domain-primitives.identifier-namespace.canonicalization]]
The system MUST provide canonicalization decision envelopes that record accepted,
rejected, unchanged, or mapped identifier forms without claiming semantic
equivalence beyond the declared finite map.

#### Scenario: Canonicalization decision preserves original identifier [r[domain-primitives.identifier-namespace.canonicalization.scenario.canonicalization-decision-preserves-original-identifier]]

- GIVEN a finite identifier and canonicalization metadata
- WHEN canonicalization maps or rejects the identifier
- THEN the canonicalization primitive records the original identifier and explicit decision metadata

### Requirement: Collision Metadata Envelope [r[domain-primitives.identifier-namespace.collision]]
The system MUST provide deterministic collision metadata for finite registry
snapshots, candidate identifiers, and declared namespace scopes without claiming
hash collision resistance or randomness quality.

#### Scenario: Collision metadata is deterministic [r[domain-primitives.identifier-namespace.collision.scenario.collision-metadata-is-deterministic]]

- GIVEN the same finite registry snapshot, candidate identifier, and namespace scope
- WHEN collision metadata is computed by the primitive
- THEN the primitive returns the same collision or no-collision decision metadata

### Requirement: Lookup Result Envelope [r[domain-primitives.identifier-namespace.lookup]]
The system MUST provide deterministic lookup result envelopes over finite
namespace scopes, admitted identifiers, aliases, and registry snapshots.

#### Scenario: Lookup distinguishes absent and rejected identifiers [r[domain-primitives.identifier-namespace.lookup.scenario.lookup-distinguishes-absent-and-rejected-identifiers]]

- GIVEN a finite registry snapshot and candidate identifier
- WHEN the identifier is malformed, outside the namespace, or absent from the registry
- THEN the lookup primitive returns distinct explicit decision metadata for the corresponding case

### Requirement: Registration Admission Envelope [r[domain-primitives.identifier-namespace.registration]]
The system MUST provide deterministic registration admission/rejection envelopes
that compose identifier-shape, namespace, uniqueness, alias, canonicalization,
collision, and lookup decisions over finite registry snapshots.

#### Scenario: Registration requires all finite namespace checks [r[domain-primitives.identifier-namespace.registration.scenario.registration-requires-all-finite-namespace-checks]]

- GIVEN finite identifier, namespace, alias, canonicalization, and registry metadata
- WHEN any identifier-shape, namespace, uniqueness, alias, collision, or lookup check rejects
- THEN the registration primitive returns an explicit rejection decision before any distributed-identity or security claim is made

### Requirement: Endpoint Address Metadata [r[domain-primitives.network-protocol.endpoint-address]]
The system MUST provide finite endpoint/address metadata envelopes that record
address-family tags, host/address identifiers, port/service metadata, scope tags,
and deterministic endpoint admission results without consulting DNS, sockets, or
network interfaces.

#### Scenario: Unknown endpoint family is rejected explicitly [r[domain-primitives.network-protocol.endpoint-address.scenario.unknown-endpoint-family-is-rejected-explicitly]]

- GIVEN finite endpoint/address metadata and a declared set of admitted address-family tags
- WHEN a candidate endpoint uses an unknown address-family tag
- THEN the endpoint primitive returns an explicit rejection without consulting DNS or the OS network stack

### Requirement: Packet Frame Envelope [r[domain-primitives.network-protocol.packet-frame]]
The system MUST provide finite packet/frame envelopes that record byte-span
bounds, frame-kind tags, declared header ranges, payload ranges, and deterministic
frame admission results.

#### Scenario: Payload range must fit inside frame [r[domain-primitives.network-protocol.packet-frame.scenario.payload-range-must-fit-inside-frame]]

- GIVEN a finite packet/frame envelope with header and payload ranges
- WHEN the payload range is outside the declared frame bounds
- THEN the packet-frame primitive rejects the envelope before parser admission is composed

### Requirement: Header Field Metadata [r[domain-primitives.network-protocol.header-field]]
The system MUST provide finite header-field metadata envelopes for field names or
IDs, field ranges, required/optional status, duplicate policy, and deterministic
header admission results.

#### Scenario: Missing required header is reported [r[domain-primitives.network-protocol.header-field.scenario.missing-required-header-is-reported]]

- GIVEN finite header-field metadata and required header definitions
- WHEN a required header field is absent
- THEN the header-field primitive returns explicit missing-header metadata

### Requirement: State-Free Parser Admission [r[domain-primitives.network-protocol.parser-admission]]
The system MUST provide deterministic state-free parser admission envelopes over
one finite packet, frame, or message under declared endpoint, header, payload,
and checksum-shape metadata.

#### Scenario: Parser admission does not claim stream completeness [r[domain-primitives.network-protocol.parser-admission.scenario.parser-admission-does-not-claim-stream-completeness]]

- GIVEN one finite packet/frame/message envelope
- WHEN parser admission accepts the envelope under declared finite rules
- THEN the result admits only that envelope and does not claim streaming parser completeness, reassembly, delivery, or session correctness

### Requirement: Message Sequencing Metadata [r[domain-primitives.network-protocol.sequence]]
The system MUST provide finite message sequencing metadata for declared sequence
numbers, acknowledgement-like fields, monotone/window bounds, duplicate markers,
and deterministic sequencing admission results.

#### Scenario: Sequence metadata does not imply delivery ordering [r[domain-primitives.network-protocol.sequence.scenario.sequence-metadata-does-not-imply-delivery-ordering]]

- GIVEN finite message sequencing metadata
- WHEN a candidate sequence field is admitted
- THEN the sequencing primitive records only declared metadata and does not claim transport delivery, ordering, replay protection, or liveness

### Requirement: Checksum Shape Metadata [r[domain-primitives.network-protocol.checksum-shape]]
The system MUST provide finite checksum-shape metadata envelopes for checksum
field location, width, covered ranges, stored value shape, and deterministic
shape admission without claiming checksum strength or corruption detection.

#### Scenario: Checksum shape does not imply integrity [r[domain-primitives.network-protocol.checksum-shape.scenario.checksum-shape-does-not-imply-integrity]]

- GIVEN finite checksum-shape metadata and a candidate packet/frame
- WHEN the checksum field shape is admitted
- THEN the checksum-shape primitive records field-shape metadata only and does not claim integrity, authentication, or corruption detection

### Requirement: Payload Bound Metadata [r[domain-primitives.network-protocol.payload-bound]]
The system MUST provide finite payload-bound metadata for payload byte ranges,
length-prefix agreement, payload-kind tags, empty-payload policy, and
deterministic payload admission results.

#### Scenario: Payload length mismatch is rejected [r[domain-primitives.network-protocol.payload-bound.scenario.payload-length-mismatch-is-rejected]]

- GIVEN finite payload-bound metadata and a declared payload length
- WHEN the payload range length does not match the declared payload length
- THEN the payload-bound primitive returns explicit length-mismatch rejection metadata

### Requirement: Protocol Admission Envelope [r[domain-primitives.network-protocol.admission]]
The system MUST provide deterministic protocol admission/rejection envelopes that
compose endpoint/address metadata, packet/frame envelopes, header fields,
state-free parser admission, sequencing metadata, checksum-shape metadata, and
payload bounds over finite protocol metadata.

#### Scenario: Admission requires all finite protocol checks [r[domain-primitives.network-protocol.admission.scenario.admission-requires-all-finite-protocol-checks]]

- GIVEN finite protocol metadata and a candidate packet/frame/message
- WHEN any endpoint, frame, header, parser, sequence, checksum-shape, or payload-bound check rejects
- THEN the protocol-admission primitive returns an explicit rejection decision before any transport, security, delivery, or external protocol claim is made


## Observability Metrics Domain Primitives

Exact finite observability and metrics primitives model only declared metric identity, bounded labels, counter/gauge metadata, histogram metadata, sample windows, aggregation envelopes, alert predicates, and deterministic admission metadata. They do not claim telemetry backend correctness, ingestion/export delivery, scraping protocols, dashboard semantics, distributed tracing, wall-clock accuracy, alert delivery, storage durability, performance, security, or probabilistic anomaly quality.

### Requirement: Metric Id

r[domain-primitives.observability-metrics.metric-id]

The implementation MUST provide an exact finite observability/metrics primitive for `metric id` with executable admission and verified proof metadata.

### Requirement: Labels

r[domain-primitives.observability-metrics.labels]

The implementation MUST provide an exact finite observability/metrics primitive for `labels` with executable admission and verified proof metadata.

### Requirement: Counter

r[domain-primitives.observability-metrics.counter]

The implementation MUST provide an exact finite observability/metrics primitive for `counter` with executable admission and verified proof metadata.

### Requirement: Gauge

r[domain-primitives.observability-metrics.gauge]

The implementation MUST provide an exact finite observability/metrics primitive for `gauge` with executable admission and verified proof metadata.

### Requirement: Histogram

r[domain-primitives.observability-metrics.histogram]

The implementation MUST provide an exact finite observability/metrics primitive for `histogram` with executable admission and verified proof metadata.

### Requirement: Window

r[domain-primitives.observability-metrics.window]

The implementation MUST provide an exact finite observability/metrics primitive for `window` with executable admission and verified proof metadata.

### Requirement: Aggregation

r[domain-primitives.observability-metrics.aggregation]

The implementation MUST provide an exact finite observability/metrics primitive for `aggregation` with executable admission and verified proof metadata.

### Requirement: Alert

r[domain-primitives.observability-metrics.alert]

The implementation MUST provide an exact finite observability/metrics primitive for `alert` with executable admission and verified proof metadata.

### Requirement: Admission

r[domain-primitives.observability-metrics.admission]

The implementation MUST provide an exact finite observability/metrics primitive for `admission` with executable admission and verified proof metadata.

### Requirement: Knowledge Graph Term Metadata [r[domain-primitives.knowledge-graph.term]]
The system MUST provide finite knowledge-graph term metadata envelopes for entity
identifiers, relation identifiers, literal values, blank-node-like local
identifiers, labels, property keys, namespace scopes, and deterministic term
admission results.

#### Scenario: Unknown predicate term is rejected [r[domain-primitives.knowledge-graph.term.scenario.unknown-predicate-term-is-rejected]]

- GIVEN a finite term registry and a candidate fact referencing a predicate absent from that registry
- WHEN term admission is evaluated
- THEN the term primitive returns explicit unknown-predicate rejection metadata

### Requirement: RDF-Like Fact Triple Envelope [r[domain-primitives.knowledge-graph.triple]]
The system MUST provide finite subject/predicate/object fact triple envelopes
that admit only declared subject terms, predicate terms, and object terms or
literal envelopes.

#### Scenario: Fact triple requires admitted endpoints [r[domain-primitives.knowledge-graph.triple.scenario.fact-triple-requires-admitted-endpoints]]

- GIVEN finite term metadata and a candidate subject/predicate/object fact
- WHEN the subject or object is outside the admitted term set
- THEN the triple primitive rejects the fact with endpoint metadata before any RDF semantics claim is made

### Requirement: Property Graph Metadata Envelope [r[domain-primitives.knowledge-graph.property]]
The system MUST provide finite property-graph metadata envelopes for node labels,
edge labels, node properties, edge properties, property key admission, and
property value envelopes.

#### Scenario: Edge property key must be declared [r[domain-primitives.knowledge-graph.property.scenario.edge-property-key-must-be-declared]]

- GIVEN finite property metadata and a candidate edge property
- WHEN the property key is outside the declared property-key envelope
- THEN the property primitive rejects the edge property with unknown-property-key metadata

### Requirement: Ontology Constraint Envelope [r[domain-primitives.knowledge-graph.ontology]]
The system MUST provide finite ontology/schema constraint envelopes for classes,
relations, domains, ranges, subtype edges, cardinality metadata, and deterministic
constraint admission results.

#### Scenario: Relation range violation is rejected [r[domain-primitives.knowledge-graph.ontology.scenario.relation-range-violation-is-rejected]]

- GIVEN finite ontology metadata declaring a relation range class
- WHEN a fact uses an object term outside the admitted range envelope
- THEN the ontology primitive rejects the fact with range-violation metadata

### Requirement: Provenance Metadata Envelope [r[domain-primitives.knowledge-graph.provenance]]
The system MUST provide finite provenance metadata envelopes for source
identifiers, extraction method identifiers, confidence envelopes, timestamps or
version counters, derivation links, and deterministic provenance admission.

#### Scenario: Fact without required source is rejected [r[domain-primitives.knowledge-graph.provenance.scenario.fact-without-required-source-is-rejected]]

- GIVEN a graph admission policy requiring source provenance
- WHEN a candidate fact lacks admitted source metadata
- THEN the provenance primitive rejects the fact with missing-source metadata

### Requirement: Entity Alignment Envelope [r[domain-primitives.knowledge-graph.alignment]]
The system MUST provide deterministic entity-alignment envelopes for aliases,
equivalence candidates, merge witnesses, canonical-term selection, conflict
metadata, and non-merge decisions.

#### Scenario: Conflicting merge witness prevents canonicalization [r[domain-primitives.knowledge-graph.alignment.scenario.conflicting-merge-witness-prevents-canonicalization]]

- GIVEN two entity terms with incompatible declared class or key metadata
- WHEN entity alignment is evaluated
- THEN the alignment primitive rejects the merge and returns explicit conflict metadata

### Requirement: Traversal and Query Shape Envelope [r[domain-primitives.knowledge-graph.query-shape]]
The system MUST provide finite traversal and graph-pattern query-shape envelopes
for start terms, edge or predicate filters, path-length bounds, variable slots,
projection slots, and deterministic query-shape admission.

#### Scenario: Query shape with unbounded path is rejected [r[domain-primitives.knowledge-graph.query-shape.scenario.query-shape-with-unbounded-path-is-rejected]]

- GIVEN finite query-shape metadata requiring explicit traversal bounds
- WHEN a candidate graph pattern contains an unbounded path expression
- THEN the query-shape primitive rejects the pattern before any query-engine or SPARQL/Cypher claim is made

### Requirement: Bounded Inference Witness Envelope [r[domain-primitives.knowledge-graph.inference]]
The system MUST provide bounded inference-witness envelopes for declared rules,
input fact identifiers, derived fact identifiers, closure-depth bounds, and
explicit inferred-fact admission metadata.

#### Scenario: Derived fact requires admitted rule witness [r[domain-primitives.knowledge-graph.inference.scenario.derived-fact-requires-admitted-rule-witness]]

- GIVEN finite rule metadata and candidate inferred fact metadata
- WHEN the inferred fact lacks an admitted rule witness or exceeds the declared closure-depth bound
- THEN the inference primitive rejects the fact with missing-or-overbound witness metadata

### Requirement: Embedding Index Metadata Envelope [r[domain-primitives.knowledge-graph.embedding-index]]
The system MUST provide finite embedding-index metadata envelopes for embedding
identifiers, vector dimension bounds, indexed term or fact identifiers, distance
metric identifiers, version metadata, and deterministic index admission without
claiming vector-search quality.

#### Scenario: Embedding dimension mismatch is rejected [r[domain-primitives.knowledge-graph.embedding-index.scenario.embedding-dimension-mismatch-is-rejected]]

- GIVEN embedding-index metadata declaring a vector dimension
- WHEN a candidate embedding vector envelope has a different dimension
- THEN the embedding-index primitive rejects the vector with dimension-mismatch metadata

### Requirement: Knowledge Graph Admission Envelope [r[domain-primitives.knowledge-graph.admission]]
The system MUST provide deterministic knowledge-graph admission/rejection
envelopes that compose term, triple, property, ontology, provenance, alignment,
query-shape, inference-witness, and embedding-index metadata over finite
knowledge-graph data.

#### Scenario: Admission requires all finite KG checks [r[domain-primitives.knowledge-graph.admission.scenario.admission-requires-all-finite-kg-checks]]

- GIVEN finite knowledge-graph metadata and a candidate fact, entity merge, query shape, inferred fact, or embedding record
- WHEN any term, triple, property, ontology, provenance, alignment, query-shape, inference, or embedding-index check rejects
- THEN the knowledge-graph-admission primitive returns explicit rejection before any graph database, standards-conformance, reasoning-completeness, LLM, RAG, or vector-search claim is made

### Requirement: Time-Series Timestamp Index [r[domain-primitives.timeseries-timestamp-index]]
The system MUST provide a time-series timestamp-index primitive for finite
integer timestamps, monotone ordering, duplicate policy, and lookup/partition
bounds.

#### Scenario: Monotone timestamp lookup returns valid partition [r[domain-primitives.timeseries-timestamp-index.scenario.monotone-timestamp-lookup-returns-valid-partition]]

- GIVEN a finite timestamp index admitted under its monotone and duplicate policy
- WHEN lookup or bound search is evaluated for a query timestamp
- THEN the returned position is inside `0..=len` and partitions timestamps according to the query predicate

#### Scenario: Unsorted or invalid timestamp index is rejected [r[domain-primitives.timeseries-timestamp-index.scenario.unsorted-or-invalid-timestamp-index-is-rejected]]

- GIVEN timestamps that violate the monotone policy, duplicate policy, or declared timestamp bounds
- WHEN timestamp-index admission is checked
- THEN the index is rejected

### Requirement: Time-Series Sample Series [r[domain-primitives.timeseries-sample-series]]
The system MUST provide a time-series sample-series primitive for aligned
timestamp/value sequences, length preservation, point updates, and invalid-sample
rejection.

#### Scenario: Valid point update preserves sample alignment [r[domain-primitives.timeseries-sample-series.scenario.valid-point-update-preserves-sample-alignment]]

- GIVEN a valid sample series and a valid sample index
- WHEN the value at that index is updated within the declared value bounds
- THEN timestamps remain unchanged, lengths remain aligned, and only the target value changes

#### Scenario: Malformed sample series is rejected [r[domain-primitives.timeseries-sample-series.scenario.malformed-sample-series-is-rejected]]

- GIVEN mismatched timestamp/value lengths, an invalid sample index, or a value outside declared bounds
- WHEN sample-series admission or update is requested
- THEN the request is rejected and the series state is unchanged

### Requirement: Time-Series Window [r[domain-primitives.timeseries-window]]
The system MUST provide a time-series window primitive for declared boundary
semantics, selected-index bounds, and complete finite in-window coverage.

#### Scenario: Valid window selects exactly in-range samples [r[domain-primitives.timeseries-window.scenario.valid-window-selects-exactly-in-range-samples]]

- GIVEN a valid timestamp index and a valid declared window boundary policy
- WHEN window selection is evaluated
- THEN every selected sample satisfies the boundary policy and every sample satisfying the boundary policy is selected

#### Scenario: Invalid window is rejected [r[domain-primitives.timeseries-window.scenario.invalid-window-is-rejected]]

- GIVEN reversed window bounds, bounds outside the supported timestamp range, or an unsupported boundary policy
- WHEN window admission is checked
- THEN the window request is rejected

### Requirement: Time-Series Resample Envelope [r[domain-primitives.timeseries-resample-envelope]]
The system MUST provide a time-series resample-envelope primitive for
deterministic bucket assignment over finite monotone timestamps.

#### Scenario: Sample maps to deterministic resample bucket [r[domain-primitives.timeseries-resample-envelope.scenario.sample-maps-to-deterministic-resample-bucket]]

- GIVEN valid monotone timestamps, a valid period, and declared bucket bounds
- WHEN resample bucket assignment is evaluated
- THEN each admitted sample maps to exactly one deterministic bucket inside the bucket range

#### Scenario: Invalid resample parameters are rejected [r[domain-primitives.timeseries-resample-envelope.scenario.invalid-resample-parameters-are-rejected]]

- GIVEN zero period, overflowing bucket arithmetic, reversed bucket bounds, or timestamps outside the declared resample range
- WHEN resample-envelope admission is checked
- THEN the request is rejected

### Requirement: Time-Series Rolling Aggregate [r[domain-primitives.timeseries-rolling-aggregate]]
The system MUST provide a time-series rolling-aggregate primitive for bounded
count, sum, minimum, and maximum envelopes over finite windows.

#### Scenario: Rolling aggregate stays within accumulator bounds [r[domain-primitives.timeseries-rolling-aggregate.scenario.rolling-aggregate-stays-within-accumulator-bounds]]

- GIVEN a valid sample series, valid rolling windows, and representable aggregate results
- WHEN rolling aggregates are evaluated
- THEN each aggregate result corresponds to its selected window and stays within the declared accumulator bounds

#### Scenario: Rolling aggregate overflow is rejected [r[domain-primitives.timeseries-rolling-aggregate.scenario.rolling-aggregate-overflow-is-rejected]]

- GIVEN a rolling window whose count or accumulated value would exceed the declared accumulator bounds
- WHEN rolling aggregation is requested
- THEN the aggregate request is rejected

### Requirement: Time-Series Gap Detection [r[domain-primitives.timeseries-gap-detection]]
The system MUST provide a time-series gap-detection primitive for finite monotone
indexes, maximum-gap thresholds, and missing-interval reporting.

#### Scenario: Gap detector reports intervals exceeding threshold [r[domain-primitives.timeseries-gap-detection.scenario.gap-detector-reports-intervals-exceeding-threshold]]

- GIVEN a valid monotone timestamp index and a positive maximum-gap threshold
- WHEN gap detection is evaluated
- THEN every reported gap has adjacent timestamps separated by more than the threshold and every such adjacent separation is reported

#### Scenario: Invalid gap detection request is rejected [r[domain-primitives.timeseries-gap-detection.scenario.invalid-gap-detection-request-is-rejected]]

- GIVEN an unsorted timestamp index, zero threshold, or overflowing timestamp difference
- WHEN gap detection is requested
- THEN the request is rejected

### Requirement: Byte Buffer Envelope [r[domain-primitives.serialization-codec.byte-buffer]]
The system MUST provide a finite byte-buffer envelope that records byte length,
capacity bounds, and admitted byte ranges without claiming ownership of any
external buffer implementation.

#### Scenario: Buffer range must fit finite length [r[domain-primitives.serialization-codec.byte-buffer.scenario.buffer-range-must-fit-finite-length]]

- GIVEN a finite byte buffer envelope and a candidate half-open byte range
- WHEN the range end exceeds the admitted buffer length
- THEN the byte-buffer primitive rejects the range or requires a proven bound before use

### Requirement: Framed Record Envelope [r[domain-primitives.serialization-codec.frame]]
The system MUST provide framed-record envelopes whose header, payload, and
optional trailer ranges are finite, nonnegative, and contained within the
admitted byte-buffer envelope.

#### Scenario: Frame sections must stay within buffer [r[domain-primitives.serialization-codec.frame.scenario.frame-sections-must-stay-within-buffer]]

- GIVEN a frame envelope with header, payload, and trailer metadata
- WHEN any section range falls outside the admitted buffer length
- THEN the frame primitive rejects the envelope or requires a proof that the section is in bounds

### Requirement: Length Prefix Envelope [r[domain-primitives.serialization-codec.length-prefix]]
The system MUST provide checked length-prefix metadata that ties a declared
finite payload length to the remaining admitted byte window.

#### Scenario: Declared length must fit remaining payload [r[domain-primitives.serialization-codec.length-prefix.scenario.declared-length-must-fit-remaining-payload]]

- GIVEN a length prefix and a remaining finite payload window
- WHEN the declared length is greater than the remaining window length
- THEN the length-prefix primitive rejects the envelope or exposes an explicit invalid-length decision

### Requirement: Schema Version Admission [r[domain-primitives.serialization-codec.schema-version]]
The system MUST provide deterministic schema/version admission metadata that
records finite schema identifiers, supported version ranges, and explicit
unknown-version rejection.

#### Scenario: Unknown version rejected deterministically [r[domain-primitives.serialization-codec.schema-version.scenario.unknown-version-rejected-deterministically]]

- GIVEN finite supported schema/version metadata
- WHEN a record declares an unknown schema or unsupported version
- THEN the schema-version primitive returns an explicit rejection decision without mutating the input envelope

### Requirement: Checksum Shape Metadata [r[domain-primitives.serialization-codec.checksum-shape]]
The system MUST provide checksum shape metadata for algorithm identifiers,
checksum byte ranges, and covered byte ranges without claiming checksum
integrity, corruption detection, collision resistance, or cryptographic
security.

#### Scenario: Checksum field range must be structural only [r[domain-primitives.serialization-codec.checksum-shape.scenario.checksum-field-range-must-be-structural-only]]

- GIVEN checksum metadata for an admitted finite buffer
- WHEN the checksum field range or covered range is outside the buffer
- THEN the checksum-shape primitive rejects the metadata without claiming whether any checksum value is correct

### Requirement: Encoder Shape Predicate [r[domain-primitives.serialization-codec.encoder-shape]]
The system MUST provide deterministic encoder shape predicates that relate an
admitted finite model value to finite output-buffer shape metadata.

#### Scenario: Encoder output must satisfy declared shape [r[domain-primitives.serialization-codec.encoder-shape.scenario.encoder-output-must-satisfy-declared-shape]]

- GIVEN an admitted finite model value and encoder shape predicate
- WHEN an encoder output length or section layout violates the declared shape
- THEN the encoder-shape primitive rejects the output metadata or requires a proof of shape preservation

### Requirement: Decoder Shape Predicate [r[domain-primitives.serialization-codec.decoder-shape]]
The system MUST provide deterministic decoder shape predicates that accept or
reject finite byte buffers based on declared byte-level envelopes before any
format-specific semantic claim is made.

#### Scenario: Decoder rejects malformed finite buffer shape [r[domain-primitives.serialization-codec.decoder-shape.scenario.decoder-rejects-malformed-finite-buffer-shape]]

- GIVEN a finite byte buffer and decoder shape predicate
- WHEN required frame, length, schema, or section metadata is malformed
- THEN the decoder-shape primitive returns an explicit rejection decision without claiming external format conformance

### Requirement: Bounded Round-Trip Envelope [r[domain-primitives.serialization-codec.round-trip]]
The system MUST provide bounded round-trip envelopes only for explicitly
admitted finite model values, schemas, versions, output buffers, and decoder
shape predicates.

#### Scenario: Round trip requires admitted finite model [r[domain-primitives.serialization-codec.round-trip.scenario.round-trip-requires-admitted-finite-model]]

- GIVEN a candidate model value and serialization/codec round-trip envelope
- WHEN the value, schema, version, or output shape is not admitted by the finite model
- THEN the round-trip primitive rejects the claim or requires the missing admission proof before asserting a bounded round trip

### Requirement: Grammar Symbol Admission [r[domain-primitives.parsing-format.grammar-symbol]]
The system MUST define finite grammar-symbol admission for terminal and
nonterminal identifiers without claiming grammar completeness or recognition of
an external language.

#### Scenario: Symbol admission rejects malformed symbol metadata [r[domain-primitives.parsing-format.grammar-symbol.scenario.rejects-malformed-symbol-metadata]]

- GIVEN a grammar symbol with a kind tag and bounded identifier metadata
- WHEN the kind tag or identifier bounds are invalid
- THEN the checked admission predicate rejects the symbol without mutating caller-owned token or grammar state

### Requirement: Token Stream Shape [r[domain-primitives.parsing-format.token-stream]]
The system MUST define finite token-stream shape checks for token kinds, source
spans, and monotone ordering over a finite token sequence.

#### Scenario: Token stream admits monotone non-overlapping spans [r[domain-primitives.parsing-format.token-stream.scenario.admits-monotone-nonoverlapping-spans]]

- GIVEN a finite token sequence with half-open source spans
- WHEN every token span is valid and ordered after the prior token span
- THEN the stream shape predicate admits the sequence as structurally well-formed

### Requirement: Source Span Predicate [r[domain-primitives.parsing-format.source-span]]
The system MUST define half-open source span predicates for bounded start/end
offsets, containment, overlap, and disjointness over a finite source length.

#### Scenario: Source span rejects out-of-bounds ranges [r[domain-primitives.parsing-format.source-span.scenario.rejects-out-of-bounds-ranges]]

- GIVEN a finite source length and a candidate half-open span
- WHEN the start is greater than the end or the end exceeds the source length
- THEN the span predicate rejects the candidate

### Requirement: AST Shape Envelope [r[domain-primitives.parsing-format.ast-shape]]
The system MUST define finite AST shape envelopes for node kind metadata, child
range bounds, arity limits, root admission, and acyclic child references.

#### Scenario: AST shape rejects dangling child ranges [r[domain-primitives.parsing-format.ast-shape.scenario.rejects-dangling-child-ranges]]

- GIVEN a finite AST node table and a node with a declared child range
- WHEN the child range exceeds the node table bounds or violates the arity limit
- THEN the AST shape envelope rejects the node table

### Requirement: Parse Error Envelope [r[domain-primitives.parsing-format.error-envelope]]
The system MUST define deterministic parse-error metadata envelopes containing a
bounded source position, error kind, and optional expected-symbol metadata.

#### Scenario: Error envelope admits bounded diagnostic metadata [r[domain-primitives.parsing-format.error-envelope.scenario.admits-bounded-diagnostic-metadata]]

- GIVEN a finite source length and deterministic parser decision metadata
- WHEN the error position is within bounds and the error kind is admitted
- THEN the error envelope admits the diagnostic metadata without claiming user-facing message quality

### Requirement: Format Version Admission [r[domain-primitives.parsing-format.version-admission]]
The system MUST define exact format-version admission for finite version tags,
accepted version sets, and closed-open supported version ranges.

#### Scenario: Version admission rejects unsupported versions [r[domain-primitives.parsing-format.version-admission.scenario.rejects-unsupported-versions]]

- GIVEN a finite accepted version set or supported version range
- WHEN an input version is outside the accepted metadata
- THEN the version admission predicate rejects the input before format-specific decoding proceeds

### Requirement: Serializer Shape Envelope [r[domain-primitives.parsing-format.serialize-shape]]
The system MUST define serializer output-shape envelopes for finite emitted
segments, length bounds, ordering metadata, and deterministic failure admission.

#### Scenario: Serializer shape preserves declared segment order [r[domain-primitives.parsing-format.serialize-shape.scenario.preserves-declared-segment-order]]

- GIVEN a finite list of serializer output segments with declared offsets and lengths
- WHEN the segment metadata is admitted
- THEN the output-shape envelope states that segment offsets are monotone and within the declared output length

### Requirement: Deserializer Shape Envelope [r[domain-primitives.parsing-format.deserialize-shape]]
The system MUST define deserializer input-shape and structural result envelopes
for finite fields, consumed ranges, remaining ranges, and deterministic
admission or rejection.

#### Scenario: Deserializer shape separates consumed and remaining input [r[domain-primitives.parsing-format.deserialize-shape.scenario.separates-consumed-and-remaining-input]]

- GIVEN a finite input and a structural deserializer result
- WHEN the result is admitted
- THEN the consumed range and remaining range are bounded, non-overlapping, and cover only admitted portions of the input

### Requirement: Bounded Round-Trip Envelope [r[domain-primitives.parsing-format.roundtrip-envelope]]
The system MUST define bounded canonical round-trip predicates only for
explicitly admitted finite values, versions, and shape envelopes.

#### Scenario: Round-trip envelope stays canonical and bounded [r[domain-primitives.parsing-format.roundtrip-envelope.scenario.stays-canonical-and-bounded]]

- GIVEN an admitted finite value, an accepted format version, and a declared canonical serializer/deserializer envelope
- WHEN a later implementation claims round-trip preservation
- THEN the claim is scoped to the admitted canonical value set and does not imply arbitrary external-format compatibility

### Requirement: Statistics Count Table [r[domain-primitives.stats-count-table]]
The system MUST provide a statistics count-table primitive for finite category
admission, bounded count updates, and overflow rejection.

#### Scenario: Valid category update increments one count [r[domain-primitives.stats-count-table.scenario.valid-category-update-increments-one-count]]

- GIVEN a valid finite count table and an admitted category
- WHEN the count table is updated for that category
- THEN exactly that category count increases and all other category counts are unchanged

#### Scenario: Invalid category or overflowing count is rejected [r[domain-primitives.stats-count-table.scenario.invalid-category-or-overflowing-count-is-rejected]]

- GIVEN an unknown category or a category count at its declared maximum
- WHEN a count update is requested
- THEN the update is rejected and the table state is unchanged

### Requirement: Statistics Frequency Table [r[domain-primitives.stats-frequency-table]]
The system MUST provide a frequency-table primitive for denominator-safe
count-to-frequency derivation over finite non-negative counts.

#### Scenario: Non-zero total count yields frequencies [r[domain-primitives.stats-frequency-table.scenario.non-zero-total-count-yields-frequencies]]

- GIVEN finite non-negative counts whose total count is non-zero and representable
- WHEN frequencies are derived
- THEN each frequency is admitted under the declared ratio envelope and uses the total count as denominator

#### Scenario: Zero or overflowing total count is rejected [r[domain-primitives.stats-frequency-table.scenario.zero-or-overflowing-total-count-is-rejected]]

- GIVEN all-zero counts or counts whose total would overflow the declared accumulator
- WHEN frequency derivation is requested
- THEN the request is rejected or returns an explicit empty result instead of numeric frequencies

### Requirement: Statistics Discrete Distribution [r[domain-primitives.stats-discrete-distribution]]
The system MUST provide a discrete-distribution primitive for finite
non-negative weights, positive total weight admission, and normalization
envelopes.

#### Scenario: Valid weights form admitted finite distribution [r[domain-primitives.stats-discrete-distribution.scenario.valid-weights-form-admitted-finite-distribution]]

- GIVEN finite non-negative weights with a positive representable total weight
- WHEN distribution admission is checked
- THEN the distribution is accepted and every normalized weight lies inside the declared probability envelope

#### Scenario: Invalid distribution weights are rejected [r[domain-primitives.stats-discrete-distribution.scenario.invalid-distribution-weights-are-rejected]]

- GIVEN a negative weight, all-zero weights, or weights whose total overflows the accumulator
- WHEN distribution admission is checked
- THEN the distribution is rejected

### Requirement: Statistics CDF Quantile [r[domain-primitives.stats-cdf-quantile]]
The system MUST provide a CDF/quantile primitive for monotone cumulative weights,
partition lookup, and deterministic tie policy.

#### Scenario: Quantile lookup returns deterministic bucket [r[domain-primitives.stats-cdf-quantile.scenario.quantile-lookup-returns-deterministic-bucket]]

- GIVEN a valid monotone cumulative distribution and an admitted query threshold
- WHEN quantile lookup is evaluated
- THEN the lookup returns exactly one bucket according to the declared boundary and tie policy

#### Scenario: Invalid CDF or threshold is rejected [r[domain-primitives.stats-cdf-quantile.scenario.invalid-cdf-or-threshold-is-rejected]]

- GIVEN a non-monotone CDF, zero final cumulative weight, overflowing cumulative weight, or query threshold outside the admitted range
- WHEN CDF or quantile admission is checked
- THEN the request is rejected

### Requirement: Statistics Bounded Moments [r[domain-primitives.stats-bounded-moments]]
The system MUST provide a bounded-moments primitive for finite count, sum, mean,
and variance-envelope derivation with checked accumulators.

#### Scenario: Valid sample set yields bounded moments [r[domain-primitives.stats-bounded-moments.scenario.valid-sample-set-yields-bounded-moments]]

- GIVEN finite bounded samples whose count and accumulators are representable
- WHEN moment envelopes are derived
- THEN count, sum, mean envelope, and variance envelope are accepted within declared bounds

#### Scenario: Moment derivation rejects empty or overflowing samples [r[domain-primitives.stats-bounded-moments.scenario.moment-derivation-rejects-empty-or-overflowing-samples]]

- GIVEN an empty sample set where a mean is required or samples whose accumulators overflow
- WHEN moment derivation is requested
- THEN the request is rejected or returns an explicit empty result instead of numeric moments

### Requirement: Statistics Deterministic Sampler [r[domain-primitives.stats-deterministic-sampler]]
The system MUST provide a deterministic sampler-admission primitive for mapping
admitted integer thresholds or sample indices into finite distribution buckets.

#### Scenario: Admitted threshold maps to exactly one bucket [r[domain-primitives.stats-deterministic-sampler.scenario.admitted-threshold-maps-to-exactly-one-bucket]]

- GIVEN a valid finite cumulative distribution and an admitted deterministic threshold
- WHEN sample admission is evaluated
- THEN the threshold maps to exactly one bucket according to the declared cumulative partition

#### Scenario: Out-of-range deterministic sample is rejected [r[domain-primitives.stats-deterministic-sampler.scenario.out-of-range-deterministic-sample-is-rejected]]

- GIVEN a threshold outside the admitted range or a malformed cumulative distribution
- WHEN deterministic sample admission is checked
- THEN the sample request is rejected

### Requirement: Statistics Confidence Interval Envelope [r[domain-primitives.stats-confidence-interval-envelope]]
The system MUST provide a confidence-interval-envelope primitive for ordered
interval endpoints and bounded confidence metadata without coverage-probability
claims.

#### Scenario: Valid interval metadata is admitted [r[domain-primitives.stats-confidence-interval-envelope.scenario.valid-interval-metadata-is-admitted]]

- GIVEN ordered interval endpoints and confidence metadata inside declared bounds
- WHEN confidence-interval envelope admission is checked
- THEN the interval metadata is accepted as a bounded interval record

#### Scenario: Malformed interval metadata is rejected [r[domain-primitives.stats-confidence-interval-envelope.scenario.malformed-interval-metadata-is-rejected]]

- GIVEN reversed endpoints, confidence metadata outside declared bounds, or non-representable endpoint arithmetic
- WHEN confidence-interval envelope admission is checked
- THEN the interval metadata is rejected

### Requirement: Resource Budget Envelope [r[domain-primitives.resource-accounting.budget]]
The system MUST define finite resource-budget envelopes for unit metadata,
limits, consumed amounts, remaining amounts, and no-overflow arithmetic.

#### Scenario: Budget rejects consumption beyond limit [r[domain-primitives.resource-accounting.budget.scenario.rejects-consumption-beyond-limit]]

- GIVEN a finite budget with declared unit metadata and limit
- WHEN a candidate consumed amount exceeds the declared limit
- THEN the budget envelope rejects the state without claiming runtime enforcement

### Requirement: Resource Counter Predicate [r[domain-primitives.resource-accounting.counter]]
The system MUST define bounded usage-counter predicates for finite counter
values, monotone increments, reset metadata, and maximum bounds.

#### Scenario: Counter increment stays within bound [r[domain-primitives.resource-accounting.counter.scenario.increment-stays-within-bound]]

- GIVEN a bounded counter value and a finite increment amount
- WHEN the increment result is inside the declared maximum bound
- THEN the counter predicate admits the increment and records deterministic usage metadata

### Requirement: Resource Quota Envelope [r[domain-primitives.resource-accounting.quota]]
The system MUST define quota envelopes for finite subjects, scopes, limits,
current usage, and deterministic admission metadata.

#### Scenario: Quota admission rejects scoped overuse [r[domain-primitives.resource-accounting.quota.scenario.rejects-scoped-overuse]]

- GIVEN a finite quota for a subject and scope
- WHEN a candidate usage amount would exceed the quota limit for that subject and scope
- THEN the quota envelope rejects the candidate with deterministic overage metadata

### Requirement: Resource Lease Envelope [r[domain-primitives.resource-accounting.lease]]
The system MUST define finite lease envelopes for holders, resource identifiers,
half-open validity intervals, renewal metadata, and expiry predicates.

#### Scenario: Lease expires outside validity interval [r[domain-primitives.resource-accounting.lease.scenario.expires-outside-validity-interval]]

- GIVEN a finite lease with a holder, resource identifier, and half-open validity interval
- WHEN a candidate bucket is outside the lease interval
- THEN the lease envelope treats the lease as expired for that bucket

### Requirement: Resource Usage Window [r[domain-primitives.resource-accounting.usage-window]]
The system MUST define bounded usage-window predicates for finite aggregation
windows, admitted samples, sum/count metadata, and no-overflow envelopes.

#### Scenario: Usage window aggregates admitted samples [r[domain-primitives.resource-accounting.usage-window.scenario.aggregates-admitted-samples]]

- GIVEN a finite usage window and usage samples admitted by the window predicate
- WHEN the aggregate amount is computed in the declared bounded domain
- THEN the usage-window predicate admits only aggregate metadata that stays within the no-overflow envelope

### Requirement: Resource Reservation Envelope [r[domain-primitives.resource-accounting.reservation]]
The system MUST define reservation envelopes for finite resource identifiers,
reserved amounts, capacity limits, and admitted reservation sets.

#### Scenario: Reservation rejects capacity overflow [r[domain-primitives.resource-accounting.reservation.scenario.rejects-capacity-overflow]]

- GIVEN a finite resource capacity and admitted reservations
- WHEN adding a candidate reservation would make the reserved amount exceed capacity
- THEN the reservation envelope rejects the candidate without claiming scheduling optimality

### Requirement: Resource Overage Classification [r[domain-primitives.resource-accounting.overage]]
The system MUST define deterministic overage classifications for finite limits,
usage amounts, margins, and severity metadata.

#### Scenario: Overage classification records exact margin [r[domain-primitives.resource-accounting.overage.scenario.records-exact-margin]]

- GIVEN a finite limit and usage amount in the same unit domain
- WHEN usage exceeds the limit
- THEN the overage classification records the exact finite margin and deterministic severity metadata

### Requirement: Resource Admission Envelope [r[domain-primitives.resource-accounting.admission]]
The system MUST define deterministic resource-admission envelopes that return
admit/reject metadata from finite budgets, quotas, leases, usage windows,
reservations, and overage predicates.

#### Scenario: Admission rejects exhausted budget deterministically [r[domain-primitives.resource-accounting.admission.scenario.rejects-exhausted-budget-deterministically]]

- GIVEN a finite accounting state and a candidate resource request
- WHEN the request would exceed an admitted budget, quota, lease, usage-window, or reservation predicate
- THEN the admission envelope rejects the request with deterministic metadata and does not claim fairness or runtime enforcement

### Requirement: Calendar Field Admission [r[domain-primitives.scheduling-calendar.calendar-field]]
The system MUST define bounded calendar-field admission for finite field kinds,
minimum/maximum values, and closed or closed-open value ranges.

#### Scenario: Calendar field rejects out-of-range value [r[domain-primitives.scheduling-calendar.calendar-field.scenario.rejects-out-of-range-value]]

- GIVEN a finite calendar field with declared minimum and maximum bounds
- WHEN a candidate value falls outside the declared bounds
- THEN the field admission predicate rejects the value without claiming timezone or locale-calendar correctness

### Requirement: Time Bucket Metadata [r[domain-primitives.scheduling-calendar.time-bucket]]
The system MUST define discrete time-bucket metadata for finite bucket indexes,
positive bucket durations, and bounded bucket ranges.

#### Scenario: Time bucket admits positive bounded duration [r[domain-primitives.scheduling-calendar.time-bucket.scenario.admits-positive-bounded-duration]]

- GIVEN a bucket index, bucket duration, and maximum bucket count
- WHEN the duration is positive and the index is inside the finite bucket range
- THEN the bucket metadata predicate admits the bucket

### Requirement: Schedule Interval Predicate [r[domain-primitives.scheduling-calendar.interval]]
The system MUST define half-open schedule interval predicates for bounded start
and end buckets, containment, overlap, and disjointness.

#### Scenario: Interval conflict uses half-open overlap [r[domain-primitives.scheduling-calendar.interval.scenario.uses-half-open-overlap]]

- GIVEN two admitted half-open intervals over the same finite bucket domain
- WHEN the first interval end equals the second interval start
- THEN the interval predicate treats the intervals as non-overlapping

### Requirement: Recurrence Window Envelope [r[domain-primitives.scheduling-calendar.recurrence-window]]
The system MUST define finite recurrence-window envelopes for candidate schedule
buckets, horizon bounds, stride metadata, and deterministic enumeration limits.

#### Scenario: Recurrence window stays within horizon [r[domain-primitives.scheduling-calendar.recurrence-window.scenario.stays-within-horizon]]

- GIVEN a recurrence start bucket, stride, maximum candidates, and finite horizon
- WHEN candidate buckets are admitted by the recurrence envelope
- THEN every candidate bucket is within the horizon and respects the declared stride metadata

### Requirement: Deadline Order Predicate [r[domain-primitives.scheduling-calendar.deadline-order]]
The system MUST define deterministic deadline and priority ordering predicates
for finite jobs or reservations.

#### Scenario: Deadline order uses stable tie-break metadata [r[domain-primitives.scheduling-calendar.deadline-order.scenario.uses-stable-tie-break-metadata]]

- GIVEN two finite scheduling items with equal deadlines and priorities
- WHEN the ordering predicate compares them
- THEN the predicate uses declared stable tie-break metadata rather than nondeterministic runtime order

### Requirement: Schedule Conflict Predicate [r[domain-primitives.scheduling-calendar.conflict]]
The system MUST define conflict predicates over admitted schedule intervals and
finite resource identifiers.

#### Scenario: Conflict requires shared resource and overlapping interval [r[domain-primitives.scheduling-calendar.conflict.scenario.requires-shared-resource-and-overlap]]

- GIVEN two admitted reservations with resource identifiers and half-open intervals
- WHEN the reservations use different resources or non-overlapping intervals
- THEN the conflict predicate reports no conflict

### Requirement: Resource Slot Envelope [r[domain-primitives.scheduling-calendar.resource-slot]]
The system MUST define finite resource-slot envelopes for resource identifiers,
capacity bounds, reservation counts, and admitted occupancy.

#### Scenario: Resource slot rejects capacity overflow [r[domain-primitives.scheduling-calendar.resource-slot.scenario.rejects-capacity-overflow]]

- GIVEN a finite resource slot with declared capacity and admitted reservations
- WHEN the reservation count or declared occupancy exceeds capacity
- THEN the resource-slot envelope rejects the slot

### Requirement: Schedule Admission Envelope [r[domain-primitives.scheduling-calendar.admission]]
The system MUST define deterministic schedule admission envelopes that return
admit/reject metadata from finite intervals, deadlines, priorities, resources,
and conflict predicates.

#### Scenario: Admission rejects conflicting reservation deterministically [r[domain-primitives.scheduling-calendar.admission.scenario.rejects-conflicting-reservation-deterministically]]

- GIVEN a finite schedule state and a candidate reservation
- WHEN the candidate conflicts with an admitted reservation under the declared resource and interval predicates
- THEN the admission envelope rejects the candidate with deterministic conflict metadata and does not claim optimal rescheduling

### Requirement: Finite Sample Sequence [r[signal.finite-sample-sequence]]
The system MUST define a finite sample sequence primitive with explicit length,
channel/layout metadata, sample-domain bounds, and valid index admission.

#### Scenario: Checked sample sequence accepts bounded finite metadata [r[signal.finite-sample-sequence.scenario.checked-sequence-accepts-bounded-finite-metadata]]

- GIVEN finite length, channel count, layout metadata, and sample-domain bounds inside representable limits
- WHEN a checked sample sequence constructor or checker evaluates the metadata
- THEN it accepts the sequence metadata and exposes only indices within the finite length and channel layout

#### Scenario: Invalid sample metadata is rejected without mutation [r[signal.finite-sample-sequence.scenario.invalid-sample-metadata-rejected-without-mutation]]

- GIVEN a previously valid sample sequence value
- WHEN a public checked operation receives invalid length, channel, layout, or sample-domain metadata
- THEN it rejects the input explicitly and leaves the previously valid value unchanged

### Requirement: FIR Filter Envelope [r[signal.fir-filter-envelope]]
The system MUST define a finite FIR filter envelope primitive for kernel length,
coefficient bounds, input sample bounds, and conservative accumulator/output
bounds over exact discrete values.

#### Scenario: FIR envelope bounds every admitted tap accumulation [r[signal.fir-filter-envelope.scenario.fir-envelope-bounds-every-admitted-tap-accumulation]]

- GIVEN an admitted finite kernel, coefficient bounds, sample bounds, and accumulator envelope
- WHEN a tap index is within the kernel length and an input sample is within the sample bounds
- THEN the primitive states that the tap contribution and accumulated envelope remain within the declared conservative bounds

#### Scenario: FIR envelope avoids floating-point frequency-response claims [r[signal.fir-filter-envelope.scenario.fir-envelope-avoids-floating-point-frequency-response-claims]]

- GIVEN a finite FIR filter envelope requirement or implementation task
- WHEN it states exported guarantees
- THEN it does not claim floating-point filter accuracy, frequency response, phase response, or runtime DSP-library correctness

### Requirement: Discrete Convolution Window [r[signal.convolution-window]]
The system MUST define a discrete convolution window primitive that maps finite
input length, finite kernel length, padding/admission policy, and output index to
an exact support interval and conservative output envelope.

#### Scenario: Convolution window maps output index to finite support [r[signal.convolution-window.scenario.convolution-window-maps-output-index-to-finite-support]]

- GIVEN admitted finite input length, finite kernel length, padding policy, and output index
- WHEN the convolution window primitive evaluates the output index
- THEN it returns or specifies only support positions that are within admitted input/kernel bounds or explicitly marked as padded positions

#### Scenario: Invalid convolution dimensions are rejected [r[signal.convolution-window.scenario.invalid-convolution-dimensions-rejected]]

- GIVEN zero, overflowing, or otherwise invalid input/kernel/output dimensions
- WHEN a checked convolution-window operation evaluates those dimensions
- THEN it rejects the dimensions without constructing an invalid support mapping

### Requirement: Decimation Admission [r[signal.decimation-admission]]
The system MUST define a decimation admission primitive for finite input length,
positive down-sampling factor, phase/offset, output length, and deterministic
output-to-input index mapping.

#### Scenario: Decimation maps each output index to one admitted input index [r[signal.decimation-admission.scenario.decimation-maps-each-output-index-to-one-admitted-input-index]]

- GIVEN finite input length, positive decimation factor, admitted phase, and output index within the computed output length
- WHEN the decimation primitive maps the output index
- THEN the mapped input index is within the input length and follows the declared factor/phase formula

#### Scenario: Invalid decimation factor or phase is rejected [r[signal.decimation-admission.scenario.invalid-decimation-factor-or-phase-rejected]]

- GIVEN a zero factor, overflowing length computation, or phase outside the admitted factor/input range
- WHEN a checked decimation operation evaluates the metadata
- THEN it rejects the metadata and does not expose an output index mapping

### Requirement: Interpolation Admission [r[signal.interpolation-admission]]
The system MUST define an interpolation admission primitive for finite input
length, positive up-sampling factor, output length, gap-fill policy, and
deterministic output/input correspondence metadata.

#### Scenario: Interpolation classifies source-aligned and gap outputs [r[signal.interpolation-admission.scenario.interpolation-classifies-source-aligned-and-gap-outputs]]

- GIVEN finite input length, positive interpolation factor, admitted output length, and output index
- WHEN the interpolation primitive classifies the output index
- THEN it marks the output as either corresponding to an in-range source sample or to an admitted gap-fill position according to the declared factor policy

#### Scenario: Invalid interpolation metadata is rejected [r[signal.interpolation-admission.scenario.invalid-interpolation-metadata-rejected]]

- GIVEN a zero factor, overflowing output length, or gap-fill policy outside the supported finite model
- WHEN a checked interpolation operation evaluates the metadata
- THEN it rejects the metadata without exposing inconsistent output/input correspondence

### Requirement: Peak Candidate Detection [r[signal.peak-candidate]]
The system MUST define a peak-candidate primitive over finite sequences using a
bounded neighborhood, threshold/envelope metadata, and deterministic tie handling.

#### Scenario: Peak candidate satisfies local finite-neighborhood predicate [r[signal.peak-candidate.scenario.peak-candidate-satisfies-local-finite-neighborhood-predicate]]

- GIVEN a finite sequence, candidate index, admitted neighborhood radius, threshold metadata, and tie policy
- WHEN the peak-candidate predicate accepts the candidate index
- THEN every compared neighbor is within the finite sequence and the candidate satisfies the declared threshold and ordering relation against those neighbors

#### Scenario: Peak tie policy is deterministic [r[signal.peak-candidate.scenario.peak-tie-policy-is-deterministic]]

- GIVEN equal-valued samples inside a peak neighborhood
- WHEN the peak-candidate primitive evaluates candidates under the declared tie policy
- THEN it admits candidates according to that deterministic policy rather than relying on iteration-order ambiguity

### Requirement: Spectral Bin Metadata [r[signal.spectral-bin-metadata]]
The system MUST define spectral-bin metadata primitives for finite bin count,
bin index admission, range metadata, and bounded magnitude/energy envelopes over
exact discrete or fixed-point values.

#### Scenario: Spectral bin metadata admits only finite in-range bins [r[signal.spectral-bin-metadata.scenario.spectral-bin-metadata-admits-only-finite-in-range-bins]]

- GIVEN finite bin count, range metadata, and a candidate bin index
- WHEN the spectral-bin metadata primitive evaluates the bin index
- THEN it admits only indices within the finite bin count and associates them with the declared range metadata

#### Scenario: Spectral metadata avoids FFT numeric correctness claims [r[signal.spectral-bin-metadata.scenario.spectral-metadata-avoids-fft-numeric-correctness-claims]]

- GIVEN a spectral-bin metadata requirement or implementation task
- WHEN it states exported guarantees
- THEN it does not claim FFT algorithm correctness, floating-point spectral accuracy, leakage/noise behavior, or runtime DSP-library correctness

### Requirement: State Set Envelope [r[domain-primitives.state-machine-workflow.state]]
The system MUST define finite state-set envelopes that admit declared states and
record initial and terminal-state metadata before transitions or traces reference
them.

#### Scenario: Initial state must be declared [r[domain-primitives.state-machine-workflow.state.scenario.initial-state-must-be-declared]]

- GIVEN a finite state-machine declaration with an initial state
- WHEN state-set admission is evaluated
- THEN the declaration is admitted only if the initial state belongs to the finite declared state set

### Requirement: Transition Relation Envelope [r[domain-primitives.state-machine-workflow.transition]]
The system MUST define transition relation envelopes over finite source state,
event, target state, and optional guard metadata.

#### Scenario: Transition endpoints must be declared states [r[domain-primitives.state-machine-workflow.transition.scenario.endpoints-must-be-declared-states]]

- GIVEN a transition whose source or target state is absent from the declared state set
- WHEN transition admission is evaluated
- THEN the transition is rejected with endpoint metadata

### Requirement: Event Envelope [r[domain-primitives.state-machine-workflow.event]]
The system MUST define finite event envelopes that admit declared event symbols
before transitions or workflow steps reference them.

#### Scenario: Unknown event is rejected [r[domain-primitives.state-machine-workflow.event.scenario.unknown-event-rejected]]

- GIVEN a transition referencing an event absent from the finite event set
- WHEN event admission is evaluated
- THEN the event reference is rejected with unknown-event metadata

### Requirement: Guard Predicate Envelope [r[domain-primitives.state-machine-workflow.guard]]
The system MUST define deterministic guard predicate envelopes over admitted
transition context without claiming external side effects or runtime execution.

#### Scenario: Failed guard blocks transition admission [r[domain-primitives.state-machine-workflow.guard.scenario.failed-guard-blocks-transition-admission]]

- GIVEN an admitted source state, event, target state, and guard predicate metadata
- WHEN the guard predicate evaluates to false in the finite transition context
- THEN transition admission rejects the guarded transition with failed-guard metadata

### Requirement: Transition Admission Envelope [r[domain-primitives.state-machine-workflow.admission]]
The system MUST define deterministic transition admission and rejection envelopes
for public exec surfaces that combine state, transition, event, and guard checks.

#### Scenario: Undeclared transition is rejected deterministically [r[domain-primitives.state-machine-workflow.admission.scenario.undeclared-transition-rejected-deterministically]]

- GIVEN a transition request that is absent from the finite transition relation
- WHEN transition admission is evaluated
- THEN the request is rejected with undeclared-transition metadata

### Requirement: Workflow Step Envelope [r[domain-primitives.state-machine-workflow.step]]
The system MUST define finite workflow step envelopes that reference declared
states, events, and transition metadata without claiming runtime step execution.

#### Scenario: Workflow step references declared transition metadata [r[domain-primitives.state-machine-workflow.step.scenario.references-declared-transition-metadata]]

- GIVEN a workflow step with source state, event, and target state references
- WHEN workflow step admission is evaluated
- THEN the step is admitted only if those references match declared transition metadata

### Requirement: Workflow Dependency Envelope [r[domain-primitives.state-machine-workflow.dependency]]
The system MUST define finite workflow dependency envelopes over declared steps
with deterministic dependency-shape admission metadata.

#### Scenario: Dependency references declared workflow steps [r[domain-primitives.state-machine-workflow.dependency.scenario.references-declared-workflow-steps]]

- GIVEN a workflow dependency edge between two step identifiers
- WHEN dependency admission is evaluated
- THEN the dependency is admitted only if both step identifiers are declared in the finite workflow step set

### Requirement: Reachability Path Envelope [r[domain-primitives.state-machine-workflow.reachability]]
The system MUST define bounded reachability/path envelopes over finite admitted
transition systems and explicit path-witness metadata.

#### Scenario: Reachability witness follows admitted transitions [r[domain-primitives.state-machine-workflow.reachability.scenario.witness-follows-admitted-transitions]]

- GIVEN a finite path witness from one state to another
- WHEN reachability admission is evaluated
- THEN each adjacent state pair in the witness follows an admitted transition relation entry

### Requirement: Terminal State Predicate Envelope [r[domain-primitives.state-machine-workflow.terminal]]
The system MUST define terminal-state predicate envelopes over finite declared
states without claiming liveness, eventual completion, or runtime scheduling.

#### Scenario: Terminal predicate only accepts declared terminal state [r[domain-primitives.state-machine-workflow.terminal.scenario.only-accepts-declared-terminal-state]]

- GIVEN a state candidate
- WHEN terminal-state predicate evaluation is performed
- THEN the predicate matches only if the candidate is declared as terminal in the finite state metadata

### Requirement: Transition Trace Envelope [r[domain-primitives.state-machine-workflow.trace]]
The system MUST define finite transition trace envelopes whose consecutive trace
items are stepwise admitted by the transition relation and guard metadata.

#### Scenario: Trace rejects invalid step [r[domain-primitives.state-machine-workflow.trace.scenario.rejects-invalid-step]]

- GIVEN a finite transition trace containing one step absent from the admitted transition relation
- WHEN trace admission is evaluated
- THEN the trace is rejected with the invalid step index and transition metadata

### Requirement: Page Range [r[storage.page-range]]
The system MUST define a finite page-range primitive with checked page
identifier bounds, span length metadata, and half-open page interval admission.

#### Scenario: Checked page range admits finite half-open span [r[storage.page-range.scenario.checked-page-range-admits-finite-half-open-span]]

- GIVEN finite page-count metadata, a start page, and an end page with start less than or equal to end
- WHEN a checked page-range constructor evaluates the span
- THEN it admits only page identifiers within the half-open interval and within the finite page count

#### Scenario: Invalid page range is rejected without mutation [r[storage.page-range.scenario.invalid-page-range-rejected-without-mutation]]

- GIVEN a previously valid page-range value
- WHEN a public checked operation receives overflowing, reversed, or out-of-bounds page metadata
- THEN it rejects the input explicitly and leaves the previously valid value unchanged

### Requirement: Record Slot Table [r[storage.record-slot-table]]
The system MUST define a finite record-slot table primitive with slot-count
bounds, occupied/free state metadata, and valid slot-index admission.

#### Scenario: Slot table admits only in-range occupied slots [r[storage.record-slot-table.scenario.slot-table-admits-only-in-range-occupied-slots]]

- GIVEN finite slot-count metadata and occupied/free state for each admitted slot
- WHEN a slot lookup evaluates a candidate slot index
- THEN it admits occupied-slot access only for indices within the finite slot count whose state is occupied

#### Scenario: Free slot remains explicit state [r[storage.record-slot-table.scenario.free-slot-remains-explicit-state]]

- GIVEN a finite slot table with a free slot
- WHEN the slot-state primitive evaluates that slot
- THEN it exposes the free state explicitly rather than treating the slot as an occupied record

### Requirement: Key Range [r[storage.key-range]]
The system MUST define an ordered finite key-range primitive with lower and upper
bounds, half-open membership, and overlap/disjointness metadata.

#### Scenario: Key range membership follows half-open bounds [r[storage.key-range.scenario.key-range-membership-follows-half-open-bounds]]

- GIVEN an admitted ordered key range with lower bound less than or equal to upper bound
- WHEN membership is evaluated for a candidate key
- THEN the key is admitted exactly when it is greater than or equal to the lower bound and strictly less than the upper bound

#### Scenario: Invalid key range is rejected [r[storage.key-range.scenario.invalid-key-range-rejected]]

- GIVEN unordered or non-representable key bounds
- WHEN a checked key-range operation evaluates the bounds
- THEN it rejects them without exposing inconsistent membership metadata

### Requirement: Cursor Bounds [r[storage.cursor-bounds]]
The system MUST define a cursor-bounds primitive for finite scan ranges, current
cursor position, end state, and monotone cursor movement.

#### Scenario: Cursor movement stays within admitted scan range [r[storage.cursor-bounds.scenario.cursor-movement-stays-within-admitted-scan-range]]

- GIVEN an admitted finite scan range and cursor position within or at the end of that range
- WHEN the cursor advances under the declared movement policy
- THEN the resulting cursor is either within the same range at a later position or explicitly at the end state

#### Scenario: Cursor rejects movement from invalid position [r[storage.cursor-bounds.scenario.cursor-rejects-movement-from-invalid-position]]

- GIVEN a cursor position outside the admitted scan range and not equal to the declared end state
- WHEN a checked cursor operation evaluates the position
- THEN it rejects the operation without constructing an invalid next cursor

### Requirement: Segment Map [r[storage.segment-map]]
The system MUST define a finite segment-map primitive from logical key/page
ranges to physical segment/page spans with explicit coverage and disjointness
metadata.

#### Scenario: Segment map associates logical range with finite physical span [r[storage.segment-map.scenario.segment-map-associates-logical-range-with-finite-physical-span]]

- GIVEN an admitted logical range and finite physical segment/page span
- WHEN the segment-map primitive evaluates the mapping
- THEN it exposes only physical positions within the declared segment/page bounds for that logical range

#### Scenario: Segment map rejects overlapping inconsistent spans [r[storage.segment-map.scenario.segment-map-rejects-overlapping-inconsistent-spans]]

- GIVEN segment-map metadata with overlapping logical ranges that map to inconsistent physical spans under the declared policy
- WHEN a checked segment-map constructor evaluates the metadata
- THEN it rejects the map instead of exposing ambiguous logical-to-physical lookup metadata

### Requirement: Lookup Result Admission [r[storage.lookup-result]]
The system MUST define a lookup-result primitive that admits present, absent, and
out-of-range outcomes according to finite table/range metadata and deterministic
decision rules.

#### Scenario: Present lookup result references admitted record [r[storage.lookup-result.scenario.present-lookup-result-references-admitted-record]]

- GIVEN finite table/range metadata and a lookup result marked present
- WHEN the lookup-result primitive validates the result
- THEN the referenced key, slot, page, or record metadata is within the admitted finite bounds and matches the declared lookup key/range relation

#### Scenario: Absent and out-of-range results are distinguished [r[storage.lookup-result.scenario.absent-and-out-of-range-results-are-distinguished]]

- GIVEN a lookup key and admitted key-range metadata
- WHEN the lookup-result primitive validates an unsuccessful result
- THEN it distinguishes an in-range absent result from an out-of-range rejected result according to the declared bounds

### Requirement: Compaction Envelope [r[storage.compaction-envelope]]
The system MUST define a finite compaction envelope primitive that preserves live
logical records and admitted key/slot membership while allowing physical page or
segment spans to be remapped.

#### Scenario: Compaction preserves live logical record identity [r[storage.compaction-envelope.scenario.compaction-preserves-live-logical-record-identity]]

- GIVEN pre-compaction and post-compaction finite metadata plus a declared live-record set
- WHEN the compaction envelope validates the remap
- THEN every live logical record admitted before compaction is admitted after compaction with the same logical identity and allowed physical span remapping

#### Scenario: Compaction may drop non-live records only by explicit policy [r[storage.compaction-envelope.scenario.compaction-may-drop-non-live-records-only-by-explicit-policy]]

- GIVEN a record not included in the declared live-record set
- WHEN the compaction envelope omits that record from post-compaction metadata
- THEN the omission is valid only when the declared compaction policy explicitly allows dropping non-live records

### Requirement: Versioned Slot [r[storage.versioned-slot]]
The system MUST define a versioned-slot primitive over finite version metadata,
slot state, visibility bounds, and deterministic latest-visible selection.

#### Scenario: Latest-visible selection chooses maximal admitted version [r[storage.versioned-slot.scenario.latest-visible-selection-chooses-maximal-admitted-version]]

- GIVEN a finite set of versions for a slot and an admitted read/version bound
- WHEN the versioned-slot primitive selects a visible entry
- THEN the selected entry is visible under the bound and no other admitted visible entry has a greater version under the declared ordering

#### Scenario: Versioned slot rejects ambiguous latest entries [r[storage.versioned-slot.scenario.versioned-slot-rejects-ambiguous-latest-entries]]

- GIVEN two distinct entries that are both maximal under the declared version ordering without a deterministic tie policy
- WHEN a checked versioned-slot operation evaluates latest-visible selection
- THEN it rejects the metadata instead of exposing an ambiguous latest-visible result

### Requirement: Table Schema Metadata [r[domain-primitives.table-relational.table-schema]]
The system MUST provide finite table/schema metadata envelopes that record table
identifiers, declared columns, row-count bounds, schema-version metadata, and
deterministic table admission results.

#### Scenario: Unknown table schema is rejected explicitly [r[domain-primitives.table-relational.table-schema.scenario.unknown-table-schema-is-rejected-explicitly]]

- GIVEN finite table/schema metadata and a declared schema registry envelope
- WHEN a candidate table references an unknown schema identifier
- THEN the table-schema primitive returns explicit rejection metadata without consulting a database engine

### Requirement: Column Metadata Envelope [r[domain-primitives.table-relational.column]]
The system MUST provide finite column metadata envelopes for column identifiers,
declared value envelopes, nullability or presence policy, ordering position, and
deterministic column admission results.

#### Scenario: Column type mismatch is rejected before row admission [r[domain-primitives.table-relational.column.scenario.column-type-mismatch-is-rejected-before-row-admission]]

- GIVEN finite column metadata and a candidate cell value envelope
- WHEN the value envelope does not match the declared column envelope
- THEN the column primitive rejects the cell before row admission is composed

### Requirement: Row Shape Envelope [r[domain-primitives.table-relational.row]]
The system MUST provide finite row shape envelopes that record row identifiers,
cell positions, required/optional cell presence, and deterministic row admission
results.

#### Scenario: Missing required cell is reported [r[domain-primitives.table-relational.row.scenario.missing-required-cell-is-reported]]

- GIVEN finite row shape metadata and required column positions
- WHEN a required cell is absent
- THEN the row primitive returns explicit missing-cell metadata

### Requirement: Key Metadata Envelope [r[domain-primitives.table-relational.key]]
The system MUST provide finite key metadata envelopes for primary-key-like,
foreign-key-like, compound-key, and uniqueness predicates over declared row and
column metadata without claiming database enforcement.

#### Scenario: Duplicate finite key is rejected [r[domain-primitives.table-relational.key.scenario.duplicate-finite-key-is-rejected]]

- GIVEN finite key metadata and admitted row envelopes
- WHEN two rows violate a declared finite uniqueness predicate
- THEN the key primitive returns explicit duplicate-key metadata without claiming database uniqueness enforcement

### Requirement: Projection Filter Shape [r[domain-primitives.table-relational.projection-filter]]
The system MUST provide deterministic projection/filter shape envelopes over
finite column sets, row predicates, and admitted table metadata.

#### Scenario: Projection references admitted columns only [r[domain-primitives.table-relational.projection-filter.scenario.projection-references-admitted-columns-only]]

- GIVEN finite table metadata and a projection envelope
- WHEN the projection references a column outside the admitted schema
- THEN the projection/filter primitive rejects the operation shape

### Requirement: Join Shape Metadata [r[domain-primitives.table-relational.join]]
The system MUST provide finite join-shape metadata for declared input tables,
join-key metadata, cardinality bounds, unmatched-row policy, and deterministic
join-shape admission results.

#### Scenario: Join key must be present in both inputs [r[domain-primitives.table-relational.join.scenario.join-key-must-be-present-in-both-inputs]]

- GIVEN finite join metadata and two admitted table schemas
- WHEN a declared join key is absent from either input schema
- THEN the join primitive returns explicit join-key rejection metadata

### Requirement: Grouping Metadata Envelope [r[domain-primitives.table-relational.grouping]]
The system MUST provide finite grouping/aggregate-shape metadata for group keys,
aggregate input columns, aggregate result metadata, and deterministic grouping
admission without claiming aggregate numeric accuracy beyond declared exact
models.

#### Scenario: Grouping references admitted columns only [r[domain-primitives.table-relational.grouping.scenario.grouping-references-admitted-columns-only]]

- GIVEN finite grouping metadata and an admitted table schema
- WHEN a group key or aggregate input references an unknown column
- THEN the grouping primitive returns explicit unknown-column rejection metadata

### Requirement: Relational Admission Envelope [r[domain-primitives.table-relational.admission]]
The system MUST provide deterministic relational admission/rejection envelopes
that compose table/schema, column, row, key, projection/filter, join, and grouping
metadata over finite relational data.

#### Scenario: Admission requires all finite relational checks [r[domain-primitives.table-relational.admission.scenario.admission-requires-all-finite-relational-checks]]

- GIVEN finite relational metadata and candidate table or operation shape
- WHEN any schema, column, row, key, projection/filter, join, or grouping check rejects
- THEN the relational-admission primitive returns an explicit rejection before any database engine, SQL, transaction, or query-planner claim is made

### Requirement: Finite Byte String [r[text.finite-byte-string]]
The system MUST define a finite byte-string primitive with explicit length
bounds, valid byte-index admission, and no-mutation rejection semantics for
checked public operations.

#### Scenario: Checked byte string admits finite length and indices [r[text.finite-byte-string.scenario.checked-byte-string-admits-finite-length-and-indices]]

- GIVEN a finite byte sequence and representable length metadata
- WHEN a checked byte-string constructor or checker evaluates the sequence
- THEN it admits only byte indices within the finite length and exposes the length as representable metadata

#### Scenario: Invalid byte string metadata is rejected without mutation [r[text.finite-byte-string.scenario.invalid-byte-string-metadata-rejected-without-mutation]]

- GIVEN a previously valid byte-string value
- WHEN a public checked operation receives overflowing length, invalid slice, or out-of-bounds index metadata
- THEN it rejects the input explicitly and leaves the previously valid value unchanged

### Requirement: UTF-8 Boundary Range [r[text.utf8-boundary-range]]
The system MUST define UTF-8 boundary/range metadata over finite byte strings for
admitted character-boundary indices and half-open character/byte spans.

#### Scenario: UTF-8 boundary range admits only character boundaries [r[text.utf8-boundary-range.scenario.utf8-boundary-range-admits-only-character-boundaries]]

- GIVEN a finite byte string and a candidate byte range
- WHEN the UTF-8 boundary-range primitive admits the range
- THEN the range start and end are in bounds and occur on declared UTF-8 character boundaries

#### Scenario: Invalid UTF-8 range is rejected [r[text.utf8-boundary-range.scenario.invalid-utf8-range-rejected]]

- GIVEN a byte range that is out of bounds, reversed, or not aligned to admitted UTF-8 character boundaries
- WHEN a checked UTF-8 range operation evaluates the range
- THEN it rejects the range without exposing character-index metadata for it

### Requirement: Token Span [r[text.token-span]]
The system MUST define token-span metadata over finite text with token index,
byte span, optional character span, and deterministic span-order constraints.

#### Scenario: Token span maps token to admitted byte and character ranges [r[text.token-span.scenario.token-span-maps-token-to-admitted-byte-and-character-ranges]]

- GIVEN finite text, token metadata, and an admitted token index
- WHEN the token-span primitive evaluates the token
- THEN it exposes a finite half-open byte span and, when UTF-8 metadata is present, a corresponding admitted character span

#### Scenario: Token spans are ordered or explicitly overlapping by policy [r[text.token-span.scenario.token-spans-are-ordered-or-explicitly-overlapping-by-policy]]

- GIVEN multiple token spans in finite text
- WHEN the token-span metadata declares a non-overlap policy
- THEN each later token span starts at or after the previous token span end

### Requirement: Prefix and Suffix Predicate [r[text.prefix-suffix]]
The system MUST define finite prefix and suffix predicates with length bounds,
byte/character layer declaration, and exact sequence equality over admitted
positions.

#### Scenario: Prefix predicate compares only admitted prefix positions [r[text.prefix-suffix.scenario.prefix-predicate-compares-only-admitted-prefix-positions]]

- GIVEN finite haystack and prefix sequences with admitted byte or character layer metadata
- WHEN the prefix predicate accepts the prefix
- THEN the prefix length is less than or equal to the haystack length and every admitted prefix position equals the corresponding haystack position

#### Scenario: Suffix predicate compares only admitted suffix positions [r[text.prefix-suffix.scenario.suffix-predicate-compares-only-admitted-suffix-positions]]

- GIVEN finite haystack and suffix sequences with admitted byte or character layer metadata
- WHEN the suffix predicate accepts the suffix
- THEN the suffix length is less than or equal to the haystack length and every admitted suffix position equals the corresponding haystack position at the declared offset

### Requirement: Substring Span [r[text.substring-span]]
The system MUST define substring containment and match-span admission over finite
text with deterministic span metadata for accepted matches.

#### Scenario: Substring match span is finite and in bounds [r[text.substring-span.scenario.substring-match-span-is-finite-and-in-bounds]]

- GIVEN finite haystack and needle sequences plus a declared byte or character layer
- WHEN the substring-span primitive admits a match
- THEN the match span is within the haystack bounds and every admitted needle position equals the corresponding haystack position inside that span

#### Scenario: Substring absence does not expose match span [r[text.substring-span.scenario.substring-absence-does-not-expose-match-span]]

- GIVEN finite haystack and needle sequences
- WHEN the substring-span primitive reports no admitted match
- THEN it exposes absence metadata without constructing an out-of-bounds or inconsistent match span

### Requirement: Edit Distance Envelope [r[text.edit-distance-envelope]]
The system MUST define a bounded edit-distance envelope primitive for finite
source/target lengths, operation-budget metadata, and conservative lower/upper
bounds on accepted edit operations.

#### Scenario: Edit-distance envelope admits only within declared budget [r[text.edit-distance-envelope.scenario.edit-distance-envelope-admits-only-within-declared-budget]]

- GIVEN finite source and target lengths plus an edit-operation budget
- WHEN the edit-distance envelope accepts a candidate edit path or summary
- THEN the accepted operation count is representable, within the declared budget, and consistent with the finite source/target length envelope

#### Scenario: Edit-distance envelope avoids unproved optimality claims [r[text.edit-distance-envelope.scenario.edit-distance-envelope-avoids-unproved-optimality-claims]]

- GIVEN an edit-distance envelope requirement or implementation task
- WHEN it states exported guarantees
- THEN it does not claim algorithmic optimality or Levenshtein distance equality unless a dedicated proof for that algorithm is also specified

### Requirement: Normalization Envelope [r[text.normalization-envelope]]
The system MUST define normalization envelope metadata for finite source/output
spans, length-growth bounds, supported normalization form tags, and explicit
unsupported-form rejection.

#### Scenario: Normalization envelope bounds output span metadata [r[text.normalization-envelope.scenario.normalization-envelope-bounds-output-span-metadata]]

- GIVEN finite source text, a supported normalization form tag, and declared output length bounds
- WHEN the normalization-envelope primitive admits normalized output metadata
- THEN the output span is finite, representable, and within the declared conservative length-growth envelope

#### Scenario: Normalization envelope avoids Unicode table correctness claims [r[text.normalization-envelope.scenario.normalization-envelope-avoids-unicode-table-correctness-claims]]

- GIVEN a normalization-envelope requirement or implementation task
- WHEN it states exported guarantees
- THEN it does not claim full Unicode normalization table correctness, locale collation, or semantic text equivalence unless a dedicated model owns those claims

### Requirement: Delimiter Split Join Shape [r[text.delimiter-split-join]]
The system MUST define delimiter split/join shape metadata over finite text,
finite delimiter spans, segment bounds, and deterministic empty-segment policy.

#### Scenario: Delimiter split exposes finite ordered segments [r[text.delimiter-split-join.scenario.delimiter-split-exposes-finite-ordered-segments]]

- GIVEN finite text, finite delimiter metadata, and a declared empty-segment policy
- WHEN the delimiter split-shape primitive accepts the metadata
- THEN it exposes finite ordered segment spans that are in bounds and consistent with the delimiter positions and empty-segment policy

#### Scenario: Join shape accounts for delimiters between segments [r[text.delimiter-split-join.scenario.join-shape-accounts-for-delimiters-between-segments]]

- GIVEN finite segment lengths and finite delimiter length metadata
- WHEN the delimiter join-shape primitive computes or validates output length metadata
- THEN the output length envelope accounts for every segment and every inserted delimiter between adjacent segments without overflow

### Requirement: Bounded Graph Query Model [r[composite-primitives.graph-query.model]]
The system MUST define a finite graph-query model over admitted graph nodes, edge labels, triples, variable identifiers, literal atoms, query bounds, and explicit validity predicates before any query answer or witness references them.

#### Scenario: Query reference outside the model is rejected [r[composite-primitives.graph-query.model.scenario.out-of-model-reference-rejected]]

- GIVEN a query pattern or witness referencing a node, label, literal, or variable outside the finite model
- WHEN graph-query admission is evaluated
- THEN the reference is rejected with out-of-model metadata

### Requirement: Triple Pattern Match Predicate [r[composite-primitives.graph-query.triple-pattern]]
The system MUST provide a triple-pattern predicate that admits a graph triple against constants and variables only when every constant equals the corresponding triple field and every repeated variable is assigned consistently.

#### Scenario: Repeated variable must bind consistently [r[composite-primitives.graph-query.triple-pattern.scenario.repeated-variable-consistent]]

- GIVEN a pattern where the subject and object positions use the same variable
- WHEN a candidate triple is matched
- THEN the match is accepted only if the triple subject and object are equal under the admitted binding

### Requirement: Binding Extension and Compatibility [r[composite-primitives.graph-query.binding-extension]]
The system MUST define binding-extension predicates that preserve existing variable assignments, add only admitted variable/value pairs, and reject conflicting assignments during pattern matching or joins.

#### Scenario: Conflicting join assignment is rejected [r[composite-primitives.graph-query.binding-extension.scenario.conflicting-join-assignment-rejected]]

- GIVEN two partial bindings that assign different values to the same variable
- WHEN graph-query join compatibility is evaluated
- THEN the joined binding is rejected as conflicting

### Requirement: Path Witness Validity [r[composite-primitives.graph-query.path-witness]]
The system MUST admit a path witness only when every consecutive step is backed by an admitted graph triple, the path endpoints match the requested pattern, and the path length stays within the finite query bound.

#### Scenario: Broken path step is rejected [r[composite-primitives.graph-query.path-witness.scenario.broken-step-rejected]]

- GIVEN a path witness containing adjacent nodes with no admitted edge triple between them
- WHEN path-witness admission is evaluated
- THEN the witness is rejected with broken-step metadata

### Requirement: Join and Filter Composition [r[composite-primitives.graph-query.join-filter]]
The system MUST compose triple-pattern result rows with table-style joins and filters so accepted rows preserve source-pattern witnesses, binding compatibility, row bounds, and filter predicate truth.

#### Scenario: Accepted joined row preserves source witnesses [r[composite-primitives.graph-query.join-filter.scenario.joined-row-preserves-witnesses]]

- GIVEN two pattern result rows with compatible bindings and admitted source witnesses
- WHEN the graph-query join is evaluated
- THEN the accepted joined row carries bindings from both rows and preserves the source witness references

### Requirement: Index Lookup Scan Equivalence [r[composite-primitives.graph-query.index-equivalence]]
The system MUST admit an index-backed graph-query result only when the index hit set is equivalent to the finite scan result for the same triple-pattern predicate and query bounds.

#### Scenario: Missing scan hit rejects index result [r[composite-primitives.graph-query.index-equivalence.scenario.missing-scan-hit-rejects-index-result]]

- GIVEN an index hit set that omits a triple accepted by the finite scan predicate
- WHEN index-backed result admission is evaluated
- THEN the index-backed result is rejected as not scan-equivalent

### Requirement: Serialized Query Envelope [r[composite-primitives.graph-query.serialized-envelope]]
The system MUST admit serialized graph-query requests and result windows only when the serialization envelope has an admitted schema version, bounded header/payload/trailer ranges, result-window metadata, and a payload shape that references only admitted finite query-model elements.

#### Scenario: Unsupported query schema is rejected [r[composite-primitives.graph-query.serialized-envelope.scenario.unsupported-schema-rejected]]

- GIVEN a serialized graph-query envelope with an unsupported schema version or out-of-bounds payload length
- WHEN serialized envelope admission is evaluated
- THEN the envelope is rejected before any query-result witness is accepted

### Requirement: Query Result Soundness Facade [r[composite-primitives.graph-query.result-soundness]]
The system MUST expose a composite graph-query result facade whose accepted result implies finite model validity, pattern-match correctness, binding compatibility, path-witness validity where requested, filter truth, optional index-scan equivalence, and serialized envelope validity where supplied.

#### Scenario: Accepted result carries all required proofs [r[composite-primitives.graph-query.result-soundness.scenario.accepted-result-carries-proofs]]

- GIVEN a graph-query result with pattern witnesses, bindings, optional path witnesses, optional index evidence, and optional serialized envelope metadata
- WHEN the result facade accepts the result
- THEN every exported result row is backed by the corresponding finite primitive witnesses and no rejected predicate is bypassed

r[verification-gaps.boundary-audit.inventory]
The system MUST maintain a source-derived inventory of remaining trusted or weakened proof boundaries before any proof-gap closeout attempt updates published counts. Inventory evidence MUST list every `#[verifier::external_body]` site and separately report `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, and runtime-only exemption counts.

r[verification-gaps.boundary-audit.classification]
The system MUST classify each remaining proof boundary as permanent trust or a closeable structural candidate before implementation work removes, rewrites, or re-counts the boundary. Cryptographic runtime trust, digest implementation behavior, codec correctness, and collision resistance MUST NOT be overclaimed as verified unless a concrete structural wrapper proof is scoped separately.

r[verification-gaps.boundary-audit.closeout-slice]
The system MUST drain at most one selected closeable boundary family per proof-gap audit closeout slice, and MUST NOT introduce `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, runtime-only exemption markers, or new `#[verifier::external_body]` sites as replacements. If no safe closeable candidate exists, the audit MUST record that decision and preserve source-derived counts.

r[verification-gaps.boundary-audit.inventory-sync]
The system MUST synchronize README, requirements, and canonical OpenSpec proof-gap totals from source-derived count evidence whenever the trusted-boundary inventory changes.

r[verification-gaps.inventory-drift-gate]
The system MUST provide a Rust-script inventory gate that scans source files for trusted or weakened proof-gap markers and fails when live counts drift from the published expected inventory. The gate MUST report the current 16 `#[verifier::external_body]` sites and zero `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, and `@verified-logic:exempt` markers, and it MUST be included in `./scripts/verify.rs` before later verification stages claim success.

r[verification-gaps.inventory-site-list-drift-gate]
The system MUST make the Rust proof-gap inventory gate validate the exact external-body file/function site list, not only aggregate counts. The gate MUST fail closed when a same-count substitution removes one expected external-body site and introduces a new unexpected site, without mutating source files during negative fixture evidence.
### Requirement: Composite Receipt Envelope Semantics [r[composite-primitives.receipt-envelope]]

The system MUST provide bounded composite receipt and evidence-envelope semantics that admit a receipt only when its serialized envelope, artifact references, validation window, and optional query/table evidence are all finite and internally consistent.

- **Composes:** `src/composite_receipt_envelope.rs`, `src/serialization_codec.rs`, `src/storage_index.rs`, `src/table_relational.rs`, and `src/graph_query_composition.rs`.
- **Validity boundary:** schema/version, serialized round-trip shape, artifact declaration/range, storage segment, cursor/window bounds, table admission, graph-query result facade, and lookup admission are checked as finite predicates.
- **Non-claims:** no runtime artifact storage, external CI/provider conformance, performance, or cryptographic-strength guarantee beyond existing trusted hash/crypto boundaries.

#### Scenario: Well-formed receipt is admitted [r[composite-primitives.receipt-envelope.scenario.well-formed-receipt-is-admitted]]

- GIVEN a receipt with admitted schema version, bounded serialized sections, declared artifacts, and consistent validation-window metadata
- WHEN receipt-envelope admission is evaluated
- THEN the receipt is accepted and every referenced artifact lies within the declared finite envelope

#### Scenario: Dangling artifact reference is rejected [r[composite-primitives.receipt-envelope.scenario.dangling-artifact-reference-is-rejected]]

- GIVEN a receipt whose evidence row references an undeclared artifact or out-of-window result
- WHEN receipt-envelope admission is evaluated
- THEN the receipt is rejected without accepting partial evidence

#### Scenario: Query evidence composes with receipt envelope [r[composite-primitives.receipt-envelope.scenario.query-evidence-composes-with-receipt-envelope]]

- GIVEN a graph-query or table result witness embedded in a receipt envelope
- WHEN receipt admission succeeds
- THEN the accepted receipt implies the underlying finite query/table witness predicates
### Requirement: Index Query Pagination Semantics [r[composite-primitives.index-query-pagination]]

The system MUST provide bounded index/query pagination semantics where accepted result pages are scan-equivalent for their finite windows, continuation cursors advance monotonically, and repeated pages cannot introduce duplicates outside the checked finite model.

- **Validity boundary:** page windows, cursor bounds, storage page ranges, lookup admission, table admission, graph-query result facades, and index-scan equivalence are checked as finite predicates.
- **Non-claims:** no database execution engine behavior, external query-language conformance, storage-layout/performance guarantee, or unbounded result-stream semantics.

#### Scenario: First page matches bounded scan [r[composite-primitives.index-query-pagination.scenario.first-page-matches-bounded-scan]]

- GIVEN an admitted query model and a finite page limit
- WHEN the first page is accepted from an index-backed result
- THEN every row in the page is admitted by the corresponding finite scan/window predicate

#### Scenario: Continuation cursor advances [r[composite-primitives.index-query-pagination.scenario.continuation-cursor-advances]]

- GIVEN an accepted page with a continuation cursor
- WHEN the next page is evaluated from that cursor
- THEN the cursor position advances monotonically and previously emitted rows are not re-emitted

#### Scenario: Invalid cursor is rejected [r[composite-primitives.index-query-pagination.scenario.invalid-cursor-is-rejected]]

- GIVEN a cursor outside the finite result envelope or inconsistent with the query model
- WHEN pagination admission is evaluated
- THEN the page is rejected
### Requirement: Proof-Gap Classification Drift Gate

r[verification-gaps.inventory-classification-drift-gate]
The system MUST validate the classification attached to every expected proof-gap site, so permanent trust and closeable-candidate labels cannot silently drift independently of the exact site list.

#### Scenario: Current classifications pass

- GIVEN the current audited 16-site proof-gap baseline with classifications
- WHEN the Rust inventory checker validates classifications
- THEN every live site matches its expected classification and the checker exits successfully

#### Scenario: Classification substitution fails closed

- GIVEN a controlled fixture that preserves the exact site list but changes one classification
- WHEN the Rust checker evaluates classification drift
- THEN it exits non-zero and reports the changed classification without mutating source files

#### Scenario: Full verification enforces classifications

- GIVEN `./scripts/verify.rs` runs
- WHEN it invokes the proof-gap inventory drift stage
- THEN classification drift is checked before full verification can pass
### Requirement: Proof-Gap Site Manifest Source

The system MUST load the expected proof-gap external-body site baseline from a checked repository manifest and fail closed when the manifest is malformed, stale, or inconsistent with live source.

r[verification-gaps.inventory-site-manifest-source]

#### Scenario: Manifest and source match

- GIVEN a checked manifest listing the current 16 expected external-body sites
- WHEN the Rust inventory checker scans source and reads the manifest
- THEN validation succeeds only if the manifest and live site list match exactly

#### Scenario: Malformed manifest fails closed

- GIVEN a manifest with duplicate, missing, malformed, or unsorted site entries
- WHEN the Rust checker reads the manifest
- THEN it exits non-zero with a manifest validation diagnostic

#### Scenario: Stale manifest fails closed

- GIVEN live source changes one external-body site but the manifest is not updated
- WHEN the Rust checker compares manifest and source
- THEN it rejects the drift before full verification can pass

### Requirement: Proof-Gap Manifest Rationale Source

r[verification-gaps.manifest-rationale-source]
The proof-gap site manifest MUST include path, function signature, classification, and a non-empty rationale for every expected trusted or weakened boundary so each remaining trust boundary has checked per-site justification.

### Requirement: Proof-Gap Manifest Rationale Drift Gate

r[verification-gaps.manifest-rationale-drift-gate]
The Rust proof-gap inventory checker MUST reject missing, blank, malformed, stale, duplicate, or unsorted rationale-bearing manifest rows before full verification can pass, while preserving exact-site and classification drift checks.

### Requirement: Published Proof-Gap Rationale Alignment

r[verification-gaps.manifest-rationale-publication]
Published proof-gap documentation MUST identify `docs/proof-gap-sites.tsv` as the checked per-site rationale source and the canonical OpenSpec verification-gaps spec MUST record the rationale manifest requirements after archive.

## GPU Programming Primitives

r[gpu.block-dim]
GPU block dimension admission MUST require positive x/y/z dimensions and MUST reject blocks whose finite thread product exceeds the declared maximum threads per block.

r[gpu.grid-dim]
GPU grid dimension admission MUST require positive x/y/z dimensions and MUST reject grids whose finite block product exceeds the declared maximum block count.

r[gpu.thread-index]
GPU thread-index admission MUST require an admitted block and grid, local thread coordinates strictly within the block dimensions, and block coordinates strictly within the grid dimensions.

r[gpu.coalesced-access]
GPU coalesced memory-access admission MUST require a positive lane count, positive element size, positive segment size, a lane-span no larger than the segment, and a base/span range contained in the buffer without relying on wrapping arithmetic.

r[gpu.shared-tile]
GPU shared-tile admission MUST require positive rows, columns, and element size, and MUST reject tiles whose finite byte footprint exceeds the declared shared-memory capacity.

r[gpu.launch-envelope]
GPU launch-envelope admission MUST compose admitted block dimensions, grid dimensions, and shared-tile metadata, and MUST reject launches whose shared-tile capacity exceeds the launch-level maximum shared memory per block.

r[gpu.pitched-buffer]
GPU pitched-buffer admission MUST require positive width, height, and element size, MUST require each row byte footprint to fit in the pitch, and MUST require the finite pitch-by-height backing footprint to fit in the buffer.

r[gpu.pitched-address]
GPU pitched-address admission MUST compose an admitted pitched buffer with in-bounds x/y coordinates and MUST require the addressed element span to fit inside the backing buffer.

r[gpu.tensor3d-layout]
GPU tensor3D layout admission MUST require positive width, height, depth, and element size, and MUST reject tensors whose finite row-major byte footprint exceeds the declared buffer size.

r[gpu.tensor3d-index]
GPU tensor3D index admission MUST compose an admitted tensor3D layout with x/y/z coordinates strictly inside the finite tensor shape.

r[gpu.tile-projection]
GPU tile-projection admission MUST compose an admitted pitched buffer with positive tile rows/columns and MUST reject tile origins whose finite rectangle extends past the buffer width or height.

r[gpu.elementwise-program]
GPU elementwise-program admission MUST require a positive element count, positive thread count, and positive elements-per-thread, and MUST reject programs whose finite thread capacity cannot cover all elements.

r[gpu.grid-stride-loop]
GPU grid-stride-loop admission MUST require positive length, stride, and iteration count, MUST require the start index to be in bounds, and MUST reject loop shapes whose last visited index would exceed the finite input length.

r[gpu.tiled-kernel2d]
GPU tiled-kernel2D admission MUST require positive output dimensions, tile dimensions, and grid dimensions, and MUST require the finite tile grid to cover the output domain in both axes.

r[gpu.block-reduction-program]
GPU block-reduction-program admission MUST require positive input, thread, per-thread, block, and element sizes, MUST require finite work capacity to cover the input, and MUST require per-block shared-memory bytes to fit the declared shared capacity.

r[domain-primitives.gpu.reduction-tree-program]
GPU reduction-tree program admission MUST compose an admitted block-reduction envelope with a non-empty finite stage sequence, MUST require every stage to have positive non-expanding input/output cardinalities and bounded active lanes, MUST require adjacent stages to chain by output-to-input cardinality, and MUST require the final stage output to equal the declared finite output count without claiming operator algebra or numerical accuracy.

r[domain-primitives.gpu.stencil-boundary-program]
GPU stencil-boundary program admission MUST define finite 2D stencil domains, output coordinates, radii, and read windows such that full-window admission guarantees every modeled input read stays inside the declared input domain, unguarded boundary windows are rejected, and guarded boundary admission only claims in-domain reads without asserting numeric stencil semantics.

r[domain-primitives.gpu.write-disjoint-program]
GPU write-disjoint program admission MUST define finite output write regions with positive byte spans inside a declared output buffer, MUST admit same-logical-output aliases only when explicitly identified by the same logical index, MUST reject overlapping writes for distinct logical indices, and MUST compose elementwise and tiled output coordinates with the existing finite GPU program/layout envelopes without claiming CUDA scheduling or memory-order semantics.

r[domain-primitives.algebraic-effect.signature]
Algebraic-effect signature admission MUST admit calls only when a finite signature declares an operation with the exact same effect and operation identifiers, without claiming runtime stack capture, typechecker correctness, or handler execution semantics.

r[domain-primitives.algebraic-effect.arity]
Algebraic-effect invocation admission MUST admit an invocation only when a finite signature declares the same effect and operation identifiers with exactly matching input and output arities, and MUST expose the call identity separately from arity validation without claiming runtime argument typechecking.

r[domain-primitives.algebraic-effect.effect-row]
Algebraic-effect effect rows MUST define finite sets of permitted effect identifiers, MUST admit calls only when their effect is present in the row, MUST admit handlers only when they are available and target an effect in the row, and MUST remove handled effects while preserving the order of remaining row entries without claiming type inference or runtime effect tracking.

r[domain-primitives.algebraic-effect.effect-summary]
Algebraic-effect summaries MUST define finite required and residual effect rows, MUST admit a summary only when every required effect is present in the ambient row, and MUST admit sequential summary composition only when the left residual row admits the right summary requirements, producing the left required row and right residual row without claiming full effect inference.

r[domain-primitives.algebraic-effect.summary-handler-application]
Algebraic-effect summary handler application MUST admit only available handlers whose handled effect appears in the summary required row, and applying an admitted handler MUST preserve the summary required row while removing that handled effect from the residual row without claiming handler execution semantics.

r[domain-primitives.algebraic-effect.handler-coverage]
Algebraic-effect handler coverage MUST require an available handler and a matching handler clause for every declared operation belonging to the handled effect while leaving other effects outside the coverage claim.

r[domain-primitives.algebraic-effect.handler-discharge]
Algebraic-effect handler discharge MUST remove only calls for the handled effect and declared operation clauses, preserving calls for other effects or missing clauses in residual order.

r[domain-primitives.algebraic-effect.handler-composition]
Algebraic-effect handler composition MUST admit composition only for available handlers of the same effect whose clause sets are disjoint, MUST reject duplicate cross-handler clauses as ambiguous, and MUST produce a composed handler with the union of clauses while preserving the handled effect.

r[domain-primitives.algebraic-effect.continuation-token]
Algebraic-effect continuation-token admission MUST allow resume only for unused tokens whose effect matches an available handler, and a consumed token MUST be rejected for any later resume check.

r[domain-primitives.algebraic-effect.continuation-resume-mode]
Algebraic-effect continuation resume-mode admission MUST distinguish resumable from non-resumable operations, MUST admit continuation resume only for unused matching tokens under a resumable operation mode, and MUST reject handler clauses that attempt to resume a non-resumable operation without claiming runtime continuation capture or re-entry semantics.

## Rust Async Primitives

r[rust-primitives.async-task-state]
Rust async task-state primitives MUST model a finite lifecycle with pending, ready, completed, and cancelled states. Admission helpers MUST reject transitions that mutate terminal completed or cancelled tasks and MUST refine the documented state predicate rather than a separate runtime policy.

r[rust-primitives.task-waker]
Task-waker slot primitives MUST model finite empty, registered, notified, and taken states. Registration MUST bind the current generation, replacement MUST supersede stale generations, wake admission MUST be idempotent for an already-notified generation, and take admission MUST consume at most one pending notification.

r[rust-primitives.no-lost-wake]
No-lost-wake bridge predicates MUST connect task readiness with waker-slot state: when readiness is admitted after a generation is registered, the model MUST record either notification for that generation or an already-ready consumer observation. The bridge MUST NOT claim callback execution, scheduler fairness, or OS wake delivery.

r[rust-primitives.oneshot-channel]
Oneshot channel primitives MUST model a single finite slot with open, sent, received, closed, and cancelled states. Sending MUST succeed at most once, receiving MUST consume at most one sent value, closing MUST reject future sends, and cancellation MUST preserve no-duplicate-delivery evidence.

r[rust-primitives.oneshot-readiness]
Oneshot readiness composition MUST connect successful sends to receiver task readiness and waker-slot notification. Rejected sends MUST NOT create new receiver readiness or wake evidence.

r[rust-primitives.drop-guard]
Drop-guard primitives MUST model finite armed, disarmed, cleaned, and terminal states. Cleanup admission MUST run at most once for an armed guard, disarm MUST prevent later cleanup, and terminal states MUST remain stable under repeated cleanup attempts.

r[rust-primitives.verification-coverage]
Rust async primitive public helpers MUST carry explicit `r[impl]` coverage, proof functions MUST carry matching `r[verify]` coverage for the stated finite predicates, and focused tests MUST include positive and negative transition cases before README/catalog text claims implemented verification.

## R1CS Circuit Primitives

r[rust-primitives.r1cs-row-shape]
A finite single-row R1CS envelope MUST expose explicit row and witness lengths,
a bounded modulus, three coefficient slots for each of A/B/C, and three witness
slots. Admission MUST reject zero lengths, lengths above three, invalid moduli,
and coefficient or witness values outside the supported finite element bound.

r[rust-primitives.r1cs-dot-product]
The R1CS dot-product helper MUST compute the active pairwise coefficient/witness
sum modulo the row modulus for admitted row/witness shapes, and inactive slots
MUST NOT affect shorter-row results.

r[rust-primitives.r1cs-row-satisfaction]
A one-row R1CS satisfaction helper MUST return true exactly when row and witness
shapes are admitted, lengths match, and `<A,w> * <B,w> == <C,w>` modulo the row
modulus. It MUST reject malformed shapes, mismatched lengths, and unsatisfied
equations, and MUST NOT claim multi-row constraint-system or proving-system
soundness.

r[rust-primitives.r1cs-verification-coverage]
The base R1CS row primitive MUST carry non-vacuous implementation, verification, and runtime coverage for length predicates, element predicates, modulus predicates, row/witness shape admission, dot-product helpers, row satisfaction, malformed row rejection, malformed witness rejection, and length mismatch rejection before module/catalog claims are made.

### Requirement: R1CS System Shape

The system MUST expose a finite multi-row R1CS envelope with explicit system length, shared modulus, and three physical row slots. System admission MUST reject zero lengths, lengths above the finite row-slot count, invalid system moduli, invalid active rows, and active rows whose modulus differs from the system modulus. Inactive rows MUST NOT affect system-shape admission.

r[rust-primitives.r1cs-system-shape]

#### Scenario: Valid active rows are admitted

- GIVEN a system with length in `1..=3`, a valid modulus, and active rows using that modulus
- WHEN system-shape admission is evaluated
- THEN the system shape is admitted

#### Scenario: Modulus mismatch rejects active row

- GIVEN an otherwise valid system with an active row over a different modulus
- WHEN system-shape admission is evaluated
- THEN the system is rejected

### Requirement: R1CS System Satisfaction

The system MUST expose a satisfaction helper that returns true exactly when the system shape is admitted, the witness is admitted, and every active row is satisfied by the same witness. The helper MUST reject malformed witnesses, failed active rows, and malformed active rows without claiming proving-system soundness.

r[rust-primitives.r1cs-system-satisfaction]

#### Scenario: All active rows satisfied

- GIVEN an admitted system and admitted witness where every active row satisfies the R1CS equation
- WHEN system satisfaction is evaluated
- THEN the helper returns true

#### Scenario: One failed active row rejects the system

- GIVEN an admitted system and admitted witness where one active row fails the R1CS equation
- WHEN system satisfaction is evaluated
- THEN the helper returns false

### Requirement: R1CS System Verification Coverage

The R1CS system primitive MUST provide non-vacuous `r[impl]`, `r[verify]`, and runtime coverage for system length admission, shared-modulus row admission, malformed system rejection, malformed witness rejection, failed active-row rejection, inactive-row irrelevance, and satisfied-system decomposition before README/catalog claims are made.

r[rust-primitives.r1cs-system-verification-coverage]

#### Scenario: Public helper has traceable coverage

- GIVEN a public helper in `r1cs_system`
- WHEN repo-wide coverage checks are run
- THEN each helper MUST have a matching implementation or verification reference

#### Scenario: Coverage proof states concrete system implications

- GIVEN invalid system lengths, active modulus mismatches, malformed systems, malformed witnesses, failed active rows, or satisfied systems
- WHEN the system coverage proof is reviewed
- THEN the proof postconditions MUST state the corresponding validity, rejection, or satisfaction-decomposition implications instead of only `true`

### Requirement: R1CS Public Input Binding Shape

The system MUST expose a finite public-input binding envelope with explicit active length, witness-index slots, and public-value slots. Binding-shape admission MUST accept only active lengths within the bounded slot count, active indices inside the admitted witness prefix, admitted public values, and an admitted modulus. Inactive binding slots MUST NOT affect binding-shape admission.

r[rust-primitives.r1cs-public-input-binding-shape]

#### Scenario: Matching public inputs are admitted

- GIVEN admitted public bindings whose active indices are inside the witness prefix and whose active public values match the indexed witness values modulo the modulus
- WHEN public-input binding admission is evaluated
- THEN the binding helper returns true

#### Scenario: Out-of-range binding index is rejected

- GIVEN an active public binding whose witness index is outside the active witness prefix
- WHEN public-input binding admission is evaluated
- THEN the binding helper returns false

### Requirement: R1CS Public Statement Satisfaction

The system MUST expose a public statement satisfaction helper that returns true exactly when the R1CS system is satisfied by the witness and the public-input bindings are satisfied against that same witness and system modulus. The helper MUST reject malformed systems, malformed witnesses, unsatisfied systems, mismatched public values, and out-of-range public-input bindings without claiming proving-system soundness.

r[rust-primitives.r1cs-public-statement-satisfaction]

#### Scenario: Satisfied system with matching public inputs is admitted

- GIVEN an admitted R1CS system, an admitted witness satisfying every active row, and public bindings matching selected witness slots
- WHEN public statement satisfaction is evaluated
- THEN the helper returns true

#### Scenario: Mismatched public value rejects statement

- GIVEN an otherwise satisfied R1CS system and admitted witness with one active public value different from the bound witness slot modulo the system modulus
- WHEN public statement satisfaction is evaluated
- THEN the helper returns false

### Requirement: R1CS Public Input Verification Coverage

The R1CS public-input primitive MUST provide non-vacuous `r[impl]`, `r[verify]`, and runtime coverage for public-input length admission, active binding-index admission, public-value matching, malformed modulus rejection, malformed witness rejection, malformed statement-system rejection, malformed statement-public-input rejection, inactive-slot irrelevance, empty public-input admission, and satisfied-statement decomposition before README/catalog claims are made.

r[rust-primitives.r1cs-public-input-verification-coverage]

#### Scenario: Public helper has traceable coverage

- GIVEN a public helper in `r1cs_public_inputs`
- WHEN repo-wide coverage checks are run
- THEN each helper MUST have a matching implementation or verification reference

#### Scenario: Coverage proof states concrete public-input implications

- GIVEN invalid public-input lengths, invalid moduli, malformed witnesses, out-of-range active bindings, mismatched public values, malformed statement systems, malformed statement public inputs, or satisfied public statements
- WHEN the public-input coverage proof is reviewed
- THEN the proof postconditions MUST state the corresponding validity, rejection, or satisfaction-decomposition implications instead of only `true`

### Requirement: R1CS Gate Shape

A bounded R1CS gate adapter MUST admit only supported finite gate kinds, valid witness lengths, valid moduli, in-range active witness indices, and bounded constant values.

r[rust-primitives.r1cs-gate-shape]

#### Scenario: Malformed gate rejection

- GIVEN a gate with an unsupported shape, out-of-range witness index, invalid witness length, invalid modulus, or overlarge constant value
- WHEN gate admission is checked
- THEN the gate MUST be rejected before satisfaction is claimed

### Requirement: R1CS Gate Row Encoding

A bounded R1CS gate adapter MUST encode multiplication, addition, and constant gates into a single R1CS row with the same witness length and modulus as the admitted gate.

r[rust-primitives.r1cs-gate-row-encoding]

#### Scenario: Multiplication row encoding

- GIVEN an admitted multiplication gate over witness indices x, y, and z
- WHEN the gate is encoded to a row
- THEN the row MUST enforce `w[x] * w[y] = w[z]` through the existing R1CS row satisfaction predicate

#### Scenario: Addition and constant row encoding

- GIVEN an admitted addition or constant gate and a witness whose slot zero is the constant-one wire
- WHEN the gate is encoded to a row
- THEN addition MUST enforce `(w[x] + w[y]) * 1 = w[z]` and constant MUST enforce `w[x] * 1 = c` through the existing R1CS row satisfaction predicate

### Requirement: R1CS Gate Satisfaction

A bounded R1CS gate satisfaction helper MUST return true exactly when the gate is admitted, the witness is admitted with the same active length, any required constant-one wire is present, and the encoded R1CS row is satisfied.

r[rust-primitives.r1cs-gate-satisfaction]

#### Scenario: Gate satisfaction and rejection

- GIVEN admitted multiplication, addition, or constant gates and a witness
- WHEN the gate satisfaction helper is evaluated
- THEN matching witnesses MUST be accepted and mismatched witness values MUST be rejected

### Requirement: R1CS Gate Verification Coverage

The R1CS gate primitive MUST include non-vacuous verification and runtime coverage for gate admission, row encoding, satisfaction, malformed gate rejection, malformed witness rejection, one-wire rejection, and valid-but-unsatisfied encoded-row rejection.

r[rust-primitives.r1cs-gate-verification-coverage]

#### Scenario: Coverage remains catalogued

- GIVEN the R1CS gate primitive is present
- WHEN repo-wide requirement coverage checks run
- THEN the gate requirements MUST have implementation and verification references

#### Scenario: Coverage proof states concrete rejections

- GIVEN malformed gates, malformed witnesses, missing one-wire witnesses, or valid gates whose encoded row is unsatisfied
- WHEN the gate coverage proof is reviewed
- THEN the proof postconditions MUST state the corresponding satisfaction rejection implications instead of only `true`

### Requirement: R1CS Boolean Shape

A bounded R1CS boolean primitive MUST admit only valid witness lengths, valid moduli whose `modulus - 1` coefficient fits the bounded row model, and a nonzero boolean witness index inside the active witness prefix.

r[rust-primitives.r1cs-boolean-shape]

#### Scenario: Malformed boolean shape rejection

- GIVEN an invalid witness length, invalid modulus, overlarge `modulus - 1` coefficient, zero boolean index, or out-of-range boolean index
- WHEN boolean shape admission is checked
- THEN the shape MUST be rejected before satisfaction is claimed

### Requirement: R1CS Boolean Row Encoding

A bounded R1CS boolean primitive MUST encode booleanity as a single R1CS row equivalent to `b * (1 - b) = 0` using the existing constant-one witness slot convention.

r[rust-primitives.r1cs-boolean-row-encoding]

#### Scenario: Boolean row encoding

- GIVEN an admitted boolean index and modulus
- WHEN the booleanity row is encoded
- THEN the row MUST use the active witness length and modulus and MUST be valid under the existing R1CS row-shape predicate

### Requirement: R1CS Boolean Satisfaction

A bounded R1CS boolean satisfaction helper MUST return true exactly when the boolean shape is admitted, the witness is admitted with the same active length, witness slot zero is one, and the encoded row is satisfied.

r[rust-primitives.r1cs-boolean-satisfaction]

#### Scenario: Boolean values accepted and non-boolean values rejected

- GIVEN admitted boolean shapes and witnesses
- WHEN boolean satisfaction is evaluated
- THEN witness values 0 and 1 at the boolean slot MUST be accepted and non-boolean values MUST be rejected

### Requirement: R1CS Boolean Verification Coverage

The R1CS boolean primitive MUST include non-vacuous implementation, verification, and runtime coverage for shape admission, row encoding, satisfaction, malformed shape rejection, malformed witness rejection, missing one-wire rejection, non-boolean witness rejection, and valid-but-unsatisfied encoded-row rejection.

r[rust-primitives.r1cs-boolean-verification-coverage]

#### Scenario: Boolean primitive coverage remains catalogued

- GIVEN the R1CS boolean primitive is present
- WHEN repo-wide requirement coverage checks run
- THEN the boolean requirements MUST have implementation and verification references

#### Scenario: Coverage proof states concrete boolean implications

- GIVEN malformed boolean shapes, malformed witnesses, missing one-wire witnesses, or admitted booleans whose encoded row is unsatisfied
- WHEN the boolean coverage proof is reviewed
- THEN the proof postconditions MUST state the corresponding validity or rejection implications instead of only `true`

### Requirement: R1CS Bit Decomposition Shape

A bounded R1CS bit-decomposition primitive MUST admit only valid witness lengths, valid boolean-compatible moduli, a bounded value below four, and two distinct nonzero bit witness indices inside the active witness prefix.

r[rust-primitives.r1cs-bit-decomposition-shape]

#### Scenario: Malformed bit-decomposition shape rejection

- GIVEN an invalid witness length, invalid boolean-compatible modulus, value outside the two-bit range, zero bit index, out-of-range bit index, or duplicate bit indices
- WHEN bit-decomposition shape admission is checked
- THEN the shape MUST be rejected before satisfaction is claimed

### Requirement: R1CS Bit Boolean Composition

A bounded R1CS bit-decomposition primitive MUST require each active bit witness slot to satisfy the existing R1CS boolean primitive under the same active witness length and modulus.

r[rust-primitives.r1cs-bit-decomposition-boolean-composition]

#### Scenario: Non-boolean bit rejection

- GIVEN an admitted bit-decomposition shape and a witness with a non-boolean value in either bit slot
- WHEN bit-decomposition satisfaction is evaluated
- THEN satisfaction MUST be rejected

### Requirement: R1CS Bit Recomposition Satisfaction

A bounded R1CS bit-decomposition satisfaction helper MUST return true exactly when the shape and witness are admitted, both bit slots are boolean, witness slot zero is one, and `value = bit0 + 2 * bit1`.

r[rust-primitives.r1cs-bit-decomposition-satisfaction]

#### Scenario: Recomposition accepts matching bits and rejects mismatches

- GIVEN an admitted bit-decomposition shape and boolean bit witness slots
- WHEN bit-decomposition satisfaction is evaluated
- THEN matching two-bit recompositions MUST be accepted and mismatched recompositions MUST be rejected

### Requirement: R1CS Bit Decomposition Verification Coverage

The R1CS bit-decomposition primitive MUST include implementation and verification coverage for shape admission, boolean-bit composition, recomposition satisfaction, malformed shape rejection, non-boolean bit rejection, and mismatched value rejection.

r[rust-primitives.r1cs-bit-decomposition-verification-coverage]

#### Scenario: Bit-decomposition primitive coverage remains catalogued

- GIVEN the R1CS bit-decomposition primitive is present
- WHEN repo-wide requirement coverage checks run
- THEN the bit-decomposition requirements MUST have implementation and verification references

### Requirement: R1CS Lookup Shape

A bounded R1CS lookup-membership primitive MUST admit only valid witness lengths, a nonzero lookup witness index inside the active witness prefix, a table length from one to three, and bounded R1CS elements in every active table entry.

r[rust-primitives.r1cs-lookup-shape]

#### Scenario: Malformed lookup shape rejection

- GIVEN an invalid witness length, zero lookup index, out-of-range lookup index, zero table length, table length above three, or overlarge active table element
- WHEN lookup shape admission is checked
- THEN the shape MUST be rejected before membership satisfaction is claimed

### Requirement: R1CS Lookup Active Table Membership

A bounded R1CS lookup-membership primitive MUST compare the selected witness-slot value only against active table entries and MUST ignore inactive table entries.

r[rust-primitives.r1cs-lookup-active-membership]

#### Scenario: Active table entries determine membership

- GIVEN an admitted lookup shape, an admitted witness, and a selected witness value
- WHEN lookup membership satisfaction is evaluated
- THEN satisfaction MUST hold exactly when the selected value equals at least one active table entry

### Requirement: R1CS Lookup Satisfaction

A bounded R1CS lookup-membership satisfaction helper MUST return true exactly when the lookup shape and witness are admitted, the witness length matches the lookup length, and the selected witness value is a member of the active table prefix.

r[rust-primitives.r1cs-lookup-satisfaction]

#### Scenario: Lookup accepts members and rejects non-members

- GIVEN an admitted lookup shape and admitted witness
- WHEN lookup satisfaction is evaluated
- THEN active-table member values MUST be accepted and non-member values MUST be rejected

### Requirement: R1CS Lookup Verification Coverage

The R1CS lookup-membership primitive MUST include implementation and verification coverage for shape admission, active table membership, satisfaction, malformed shape rejection, malformed witness rejection, inactive-entry ignoring, and non-member rejection.

r[rust-primitives.r1cs-lookup-verification-coverage]

#### Scenario: Lookup primitive coverage remains catalogued

- GIVEN the R1CS lookup-membership primitive is present
- WHEN repo-wide requirement coverage checks run
- THEN the lookup requirements MUST have implementation and verification references

## Hardware Output

r[hardware-output.ir]
The system MUST provide a finite hardware IR for synthesizable module shapes.
Every fixed-width value, boolean expression, module port, combinational
expression, register, clock, reset, and FSM state needed by the admitted subset
MUST be explicit in the IR metadata. IR admission MUST reject unsupported or
malformed shapes before evaluation or emission, and rejection MUST NOT expose
partial RTL output.

r[hardware-output.evaluator]
The system MUST provide pure deterministic evaluation semantics for admitted
hardware IR modules. The evaluator MUST compute combinational outputs from
explicit inputs and MUST compute the next registered or FSM state from explicit
current state and input values. Evaluation MUST reject missing inputs,
incompatible widths, invalid states, and malformed admitted-data boundaries
without panicking.

r[hardware-output.adapter-subset]
The system MUST expose explicit adapter eligibility metadata for Trellis
primitives that may lower to hardware IR. The first supported adapter subset
MUST include finite bitwise operations, bitfield operations, simple arithmetic
operations, and small FSM-style admission helpers. The adapter layer MUST reject
heap allocation, unbounded vectors, generated-hardware recursion, network or
file IO, async callbacks, distributed protocol execution, crypto trust-boundary
execution, and spec-only existential predicates.

r[hardware-output.adapter-refinement]
The system MUST ensure each supported Trellis-to-hardware adapter refines the
source Trellis semantics over its admitted finite input space. For every emitted
adapter, the hardware IR evaluator result MUST match the source spec or
executable helper result for the same admitted inputs, and rejected adapter
candidates MUST leave the source primitive unchanged.

r[hardware-output.adapter-emission]
The system MUST expose a pure adapter-emission planning decision that returns an
HDL emission request only when adapter eligibility, finite-input refinement,
emitted-shape support, and the requested emitter mode all pass. Planning MUST
fail closed for refined adapters without emitted HDL support, unsupported emitter
modes, unsupported adapter candidates, out-of-domain adapter inputs, evaluator
rejection, and emitter rejection, and MUST NOT expose a partial HDL request on
rejection.

r[hardware-output.verilog-emission]
The system MUST emit deterministic Verilog text only from admitted hardware IR
modules. Verilog emission MUST use canonical module, port, declaration,
assignment, and process ordering so repeated emission of the same IR produces
byte-identical output. Verilog emission MUST fail closed for unsupported IR
constructs.

r[hardware-output.systemverilog-emission]
The system MUST emit deterministic SystemVerilog text from admitted hardware IR
modules. SystemVerilog emission SHOULD use `logic`, `always_comb`, and
`always_ff` for supported combinational and registered modules, and MAY emit
assertion snippets for admitted safety properties. SystemVerilog emission MUST
preserve the same IR semantics as the Verilog emitter for shared supported
constructs.

r[hardware-output.testbench]
The system MUST generate optional testbench or vector collateral from the
hardware IR evaluator for admitted modules. Generated collateral MUST identify
the source module, input assignments, expected outputs or next states, and the
emitter mode under test. Collateral generation MUST be deterministic and MUST
NOT depend on clocks, filesystem order, random seeds, or external tool output.

r[hardware-output.validation]
The system MUST provide validation helpers or commands that classify Verilator,
Yosys, and SymbiYosys syntax/simulation/formal checks for generated HDL when
those tools are available. If a tool is unavailable, validation evidence MUST
record an explicit skipped-tool result; it MUST NOT report success for a check
that did not run.

r[hardware-output.backend-admission]
The system MUST expose a hardware-output backend admission decision requiring
IR admission, adapter subset admission, adapter refinement evidence,
deterministic emitter support, generated collateral eligibility, and validation
result recording before claiming that a Trellis primitive has HDL output
support. Backend admission MUST NOT imply timing closure, vendor implementation
quality, downstream synthesis preservation, or support for modules outside the
admitted adapter subset.

## Hardware Output Testing

r[hardware-output.testing.matrix]
The system MUST maintain a requirement-linked hardware-output test matrix. Each
hardware-output backend requirement MUST map to at least one positive test or
proof artifact and at least one negative test or proof artifact unless the
requirement explicitly documents why one side is inapplicable. Each mapped
artifact MUST carry a Tracey `r[verify ...]` marker for the covered requirement.

r[hardware-output.testing.fixtures]
The system MUST use deterministic fixture naming and metadata for
hardware-output tests. Source IR fixtures, golden HDL fixtures,
generated-vector fixtures, and validation evidence MUST identify the
requirement ID, emitted module name, emitter mode, and expected oracle source so
reviewers can trace each artifact back to the requirement it verifies.

r[hardware-output.testing.evaluator-oracle]
The system MUST test the pure hardware IR evaluator as the semantic oracle for
backend validation. Positive tests MUST cover admitted combinational modules,
registered modules, and FSM steps. Negative tests MUST cover missing inputs,
incompatible widths, invalid current states, malformed admitted-data boundaries,
and any evaluator rejection that should block generated collateral.

r[hardware-output.testing.emitter-golden]
The system MUST test Verilog and SystemVerilog emitters with deterministic
golden-output fixtures. Golden tests MUST cover canonical module ordering, port
ordering, declaration ordering, expression formatting, combinational
assignments, registered update blocks, reset behavior, reserved identifier
rejection, and unsupported-node rejection. Repeated emission of the same
admitted IR MUST produce byte-identical output.

r[hardware-output.testing.adapter-property]
The system MUST property-test or exhaustively finite-test each supported
Trellis-to-hardware adapter over admitted input domains. Adapter tests MUST
compare the source Trellis helper result with the lowered hardware IR evaluator
result and MUST include edge values for zero, maximum representable values,
width boundaries, state boundaries, and rejected out-of-domain candidates.

r[hardware-output.testing.fail-closed]
The system MUST test fail-closed behavior for the hardware-output backend.
Unsupported adapters, malformed IR, unsupported emitter constructs, evaluator
rejections, invalid validation inputs, and unavailable output destinations MUST
produce explicit errors and MUST NOT write partial HDL, partial testbenches,
partial vectors, or misleading success evidence.

r[hardware-output.testing.tool-evidence]
The system MUST classify external HDL tool validation outcomes as pass, fail,
or skipped. Tool evidence MUST name the tool, invocation class or input
artifact, result classification, and reason for failure or skip. Missing
Verilator, Yosys, or SymbiYosys MUST be recorded as skipped with an
unavailable-tool reason and MUST NOT be reported as success.

r[hardware-output.testing.tracey-gates]
The system MUST require Tracey and Cairn gate evidence before claiming
hardware-output test coverage is complete. Hardware-output test completion MUST
include focused backend tests, generated fixture checks, Verus proof checks for
touched verified modules, Tracey uncovered and untested queries, Cairn
validation, Cairn proposal/design/tasks gates, and cargo/clippy checks as
applicable to touched Rust code.

## Spreadsheet Logic

r[domain-primitives.spreadsheet.address]
The system MUST provide finite spreadsheet sheet-bound and cell-address
envelopes that record row bounds, column bounds, cell coordinates, and
deterministic address-admission results. A checked cell address MUST be admitted
only when both coordinates are inside the declared finite sheet bounds, and
rejected address admission MUST leave prior admitted sheet metadata unchanged.

r[domain-primitives.spreadsheet.range]
The system MUST provide finite rectangular-range envelopes that normalize two
admitted endpoint addresses into a canonical rectangle, expose deterministic
range membership, and expand only to admitted cell coordinates inside the
declared sheet bounds.

r[domain-primitives.spreadsheet.formula]
The system MUST provide a finite spreadsheet formula and value model covering
blank values, exact numeric values, booleans, text, explicit errors, constants,
cell references, rectangular ranges, arithmetic operations, and finite aggregate
operations. Formula admission MUST reject syntactically or structurally invalid
formulas before dependency extraction or evaluation.

r[domain-primitives.spreadsheet.dependency]
The system MUST provide deterministic dependency-extraction predicates that
return exactly the finite set of precedent cell coordinates named by an admitted
formula, including all cells contained in admitted referenced ranges. Dependency
extraction MUST be pure over formula structure and declared sheet bounds, and
MUST NOT depend on current formatted values, cached results, external workbook
files, clocks, or volatile runtime state.

r[domain-primitives.spreadsheet.cycle]
The system MUST reject spreadsheet formula sets whose dependency graph contains
a cycle before formula evaluation begins. An admitted formula set MUST have an
acyclic dependency graph over the finite formula-cell coordinates and their
extracted precedents.

r[domain-primitives.spreadsheet.recalc]
The system MUST provide recalculation-order predicates that admit only finite
orders where each selected formula cell appears after every selected formula
precedent required by its extracted dependency set. Successful recalculation
order admission MUST refine the acyclic dependency graph and MUST NOT introduce
formula cells outside the declared finite sheet metadata.

r[domain-primitives.spreadsheet.dirty-propagation]
The system MUST provide dirty-cell propagation predicates that include each
changed cell and every formula cell reachable from changed cells through reverse
spreadsheet dependencies, while excluding formula cells that are not reachable
dependents under the admitted finite dependency graph.

r[domain-primitives.spreadsheet.error]
The system MUST provide explicit deterministic spreadsheet error envelopes for
bad references, division by zero, value-shape mismatches, and cycle rejection.
Error-producing checks MUST return the same error metadata for the same admitted
inputs, MUST NOT panic, and MUST NOT mutate admitted sheet, formula, dependency,
or recalculation metadata.

r[domain-primitives.spreadsheet.admission]
The system MUST provide a deterministic spreadsheet-admission composition that
requires address admission, range admission, formula admission, dependency
extraction, cycle rejection, recalculation-order admission, dirty-propagation
admission, and error-envelope determinism before any spreadsheet evaluation
claim is exposed.

## Runnable Proof Examples

r[runnable-proof-examples.basic]
The system MUST provide a deterministic proof example that demonstrates the
Trellis `spec fn` / runtime function / `ensures` refinement pattern through a
Verus command that succeeds and an isolated negative fixture that fails for the
expected proof reason.

r[runnable-proof-examples.commands]
The system MUST document exact positive, negative, wrapper, and opt-in commands
for runnable proof examples so users can reproduce both passing proof evidence
and expected failing proof evidence without treating the negative fixture as part
of the normal passing path.

## Proof-Evidence Bundles

r[proof-evidence-bundles.export]
The system SHOULD export deterministic proof-evidence bundle records that name
schema version, scope, requirement IDs, source/proof paths, verifier command,
proof outcome, Tracey coverage status, proof-gap references, BLAKE3 content
identities, maturity facts when available, scoped claims, and explicit non-claims.
A bundle MUST NOT grant release approval by itself.

r[proof-evidence-bundles.validation]
The system MUST validate proof-evidence bundles fail-closed for missing required
fields, stale BLAKE3 identities, unsupported schema versions, missing requirement
coverage, missing proof-gap references, and overbroad claims such as release
approval, whole-program correctness, certification, third-party conformance, or
production safety.

## Proof-Evidence IR Chain

r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.contract]
The system MUST represent proof evidence as linked source envelopes, canonical proof evidence IR, and verifier receipts that bind source identity, proof IR identity, verifier receipt identity, declared property IDs, assumptions, dependencies, result status, and non-claims.

r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.verifier_receipt]
The system MUST bind proof result status to verifier identity, toolchain identity, solver identity, command identity, input roots, and result identity, and MUST reject proof evidence with an empty verifier identity, empty input roots, or an unsupported result status using deterministic diagnostics.

r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.assumptions]
The system MUST represent assumptions, trusted boundaries, external bodies, proof gaps, dependencies, requirement IDs, and property IDs as typed fields in proof evidence IR, and MUST change the evidence identity or report stale assumption linkage when the assumption set changes.

r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.handoff]
The system SHOULD emit a deterministic proof evidence-chain handoff that carries canonical source, proof IR, verifier receipt, and assumption identities plus non-claims so a consumer can canonicalize it without parsing Verus source.

r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.positive]
The system MUST accept proof evidence IR documents whose source identities, verifier receipt identities, chain identities, and handoff identities are consistent and BLAKE3-bound.

r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.negative]
The system MUST reject proof evidence that claims whole downstream system correctness, compiler correctness, verifier soundness, release eligibility, or properties outside the declared proof scope, and MUST mark recorded-only reference evidence without counting it as a promoted formal proof.

r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.evidence]
The implementation MUST include positive and negative evidence covering consistent linked evidence, missing verifier identity, unsupported result status, overbroad claims, stale chain identities, assumption drift, and inconsistent handoff identities.

r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.docs]
The system MUST document the proof evidence IR chain surface, commands, claim boundary, and non-claims in repository documentation and accepted Cairn spec text.

r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.validation]
The change MUST capture validation evidence for focused Cargo tests, Verus verification, Tracey uncovered and untested checks, and Cairn validation and gates before archive.

## Proof Artifact Provenance

r[trellis.proof_artifact_provenance.identity]
Exportable Trellis proof artifacts MUST carry a stable artifact ID, canonical BLAKE3 metadata identity, source/proof input digests, verifier receipt digest, verifier/toolchain identity, normalized invocation, declared local property ID, verification status, and failure reason when the local proof did not pass.

r[trellis.proof_artifact_provenance.boundary]
Proof artifact provenance MUST state the checked Trellis-local property and exact content scope, MUST include non-claims excluding downstream certification, release eligibility, and whole-stack correctness, and MUST reject unsupported or overclaiming roles fail-closed.

r[trellis.proof_artifact_provenance.handoff]
Proof artifact handoff metadata MUST identify Trellis exports as downstream manual-review or reference inputs unless the downstream repo records its own verifier receipt; downstream-certified handoff roles without such a receipt MUST be rejected.

r[trellis.proof_artifact_provenance.fixtures]
The proof artifact provenance surface MUST include positive and negative fixtures covering passing local proof metadata, failing local proof metadata with a failure reason, stale digests, missing property scope, missing verifier identity, missing non-claims, unsupported claim roles, and downstream overclaims.

## Integration Adapters

r[integration-adapters.user-facing-adapters]
The system MUST provide deterministic user-facing adapter surfaces only when they
compose existing verified Trellis primitives or explicit pure cores, reject
malformed or out-of-subset inputs deterministically, and document supported
subsets, validation evidence, and non-goals without claiming full external
ecosystem conformance.

## Durable Reference Identity Separation

r[newtypes.durable-identities.types]
The typed durable reference model MUST use distinct structures for reference, value, type, codec, closure, and schema identities in every envelope, request, catalog, result, and conversion witness, so that cross-domain substitution fails to compile.

r[newtypes.durable-identities.invalid-inputs]
Each identity structure MUST retain the complete raw u64 value domain, including zero, so malformed finite metadata remains expressible and rejectable by explicit admission predicates; construction alone MUST NOT imply validity.

r[newtypes.durable-identities.preservation]
For equivalent raw field values, the typed implementation MUST preserve finite metadata, closure availability, missing-closure reporting, unpersist type matching, and conversion-witness admission results.

r[newtypes.durable-identities.evidence]
The change MUST include positive semantic tests, negative malformed-input tests, and compile-fail evidence for representative cross-domain substitutions.

r[newtypes.durable-identities.docs]
The change MUST update canonical requirements, source and test annotations, public documentation, and accepted Cairn text for every new identity boundary.

r[newtypes.durable-identities.validation]
The migration MUST pass focused and full Cargo tests, compile-fail checks, Clippy, full-library Verus, Tracey, proof-gap inventory, and Cairn validation without adding trusted proof boundaries.

r[newtypes.durable-identities.non-claims]
Identity wrappers MUST NOT be presented as cryptographic digests, collision-resistance evidence, persistence evidence, codec correctness, type-checker correctness, dependency retrieval, or migration execution; they provide domain separation and preserved finite-model admission behavior only.

## Parking-Lot Identity Separation

r[newtypes.parking-lot-identities]
The parking-lot core MUST use distinct `ParkKey` and `ThreadId` types. Each type
MUST support explicit raw construction and raw access without an implicit
conversion to the other identity domain.

r[newtypes.parking-lot-api]
Every public parking-lot core and raw-mutex function MUST use `ParkKey` for
queue positions and `ThreadId` for waiter or owner positions.

r[newtypes.parking-lot-semantics]
For equivalent raw values, typed parking-lot operations MUST preserve queue
validity, waiter uniqueness, waiter counts, requeue results, and raw-mutex
state transitions.

r[newtypes.parking-lot-evidence]
The change MUST include positive and negative runtime tests plus compile-fail
evidence for both directions of identity exchange.

r[newtypes.parking-lot-docs]
Canonical requirements, source annotations, tests, public module text, and
accepted Cairn text MUST describe the typed identity boundary.

r[newtypes.parking-lot-non-claims]
The identity types provide Rust domain separation only. They MUST NOT be
presented as a Verus, scheduler, blocking, fairness, or operating-system proof.

r[newtypes.parking-lot-validation]
The migration MUST pass focused and full Cargo tests, compile-fail checks,
Clippy, Tracey uncovered and untested checks, and Cairn validation before
archive.

## Bounded Modular Tree Fold

r[bounded-modular-tree-fold.boundary]
Bend 0.2.38 and HVM2 2.0.22 MUST remain pinned external conformance references. Local Trellis proofs MUST NOT prove their implementations or generated artifacts.

r[bounded-modular-tree-fold.u24-domain]
The module MUST define named `u24` width, modulus, maximum, mask, and widened-sum bounds. Normalization MUST be closed, identity-preserving, and idempotent.

r[bounded-modular-tree-fold.u24-add]
Executable `u24` addition MUST accept admitted operands, avoid runtime overflow, stay in the domain, and equal mathematical addition modulo the named modulus.

r[bounded-modular-tree-fold.algebra]
Concrete `u24` wrapping addition MUST have verified zero-identity, commutativity, and associativity proof functions over its declared domain.

r[bounded-modular-tree-fold.admission]
The versioned profile MUST admit only non-empty power-of-two leaf vectors within a nonzero limit. It MUST reject invalid leaves and unrepresentable accounting deterministically.

r[bounded-modular-tree-fold.refinement]
The iterative sequential oracle and iterative adjacent-pair reducer MUST refine one ordered `u24` modular-sum specification for admitted input.

r[bounded-modular-tree-fold.equivalence]
A verified proof function MUST show that one adjacent-pair round preserves the ordered modular sum. The balanced result MUST equal the sequential result.

r[bounded-modular-tree-fold.work]
For admitted `n` leaves, the balanced schedule MUST shrink each nonterminal round and perform exactly `n - 1` abstract combines.

r[bounded-modular-tree-fold.evidence]
Proof evidence MUST bind source, requirements, proof functions, verifier, proof gaps, BLAKE3 identities, algorithm profile, executable oracle symbol, and downstream validation requirements.

r[bounded-modular-tree-fold.fixtures]
Positive, negative, boundary, and property tests MUST cover arithmetic, admission, singleton and multi-round reduction, exact work, stale handoffs, and three-oracle agreement.

r[bounded-modular-tree-fold.non-claims]
Evidence MUST NOT claim Bend, HVM2, generated-C, host-compiler, arbitrary-tree, benchmark, resource, interaction-count, release, or downstream-decoding correctness.

r[bounded-modular-tree-fold.docs]
The repository MUST document the public modules, exact external references, profile, oracle symbol, reproduction commands, proof handoff, and bounded claims.

r[bounded-modular-tree-fold.validation]
Closeout MUST pass focused tests, property tests, full Verus, Tracey, proof evidence, proof gaps, catalogs, Clippy, Octet, Nix, and Cairn gates.

## Aeneas Cross-Verifier Pilot

r[trellis.aeneas_cross_verifier.provider_consumption]
The pilot MUST consume Octet revision `890f424cd7b1b2015f39c0eb5747f1bd1e576028`. It MUST bind the reviewed provider, policy, compatibility, and consumer-contract identities.

r[trellis.aeneas_cross_verifier.exact_source]
The pilot MUST select `src/arithmetic.rs::abs_diff` from normal Cargo macro erasure. It MUST reject copied or rewritten proof targets.

r[trellis.aeneas_cross_verifier.translation_manifest]
The pilot MUST bind one non-errorful LLBC declaration to one generated Lean declaration through measured `translation.json` bytes. Charon `def_id` values MUST remain run-local.

r[trellis.aeneas_cross_verifier.feasibility]
Typed stage observations MUST classify the exact-source attempt as supported, blocked, or failed. Only complete reviewed unsupported evidence MAY produce blocked status.

r[trellis.aeneas_cross_verifier.generated_boundary]
Generated LLBC, translation metadata, and Lean definitions MUST remain separate from maintained theorem source. Freshness checks MUST bind every stage.

r[trellis.aeneas_cross_verifier.theorem_map]
The pilot MUST map symmetry and self-zero to exact requirement, source, statement, model, Verus symbol, Lean symbol, and receipt identities.

r[trellis.aeneas_cross_verifier.theorem_adequacy]
Selected theorem source MUST be non-vacuous and free of forbidden proof shortcuts. The trust closure MUST list all observed axioms.

r[trellis.aeneas_cross_verifier.resource_budget]
Each provider stage MUST use the named timeout, retained-log, retained-artifact, environment, and network-denial policy. Exhaustion MUST produce failure.

r[trellis.aeneas_cross_verifier.manifest]
The Trellis manifest MUST link separate Verus and Aeneas facts through one runtime source identity. It MUST NOT merge their authority.

r[trellis.aeneas_cross_verifier.negative_evidence]
Lean MUST reject the false theorem. Octet adequacy MUST reject the vacuous theorem. Tamper fixtures MUST fail before proof promotion.

r[trellis.aeneas_cross_verifier.claim_boundary]
Evidence MUST NOT claim verifier independence, semantic equivalence, translation soundness, verifier soundness, whole-program correctness, or release eligibility.

r[trellis.aeneas_cross_verifier.final_validation]
Closeout MUST pass exact-source generation, both positive theorems, false and vacuous controls, trust audit, freshness, tamper, Octet, Verus, Tracey, Nix, and Cairn gates.

## Standalone Nickel Export Shell

r[trellis.nickel_export_cutover.source]
Release inputs MUST pin Nickel export revision `257fafc1c746f1faf156207043a4c826bfb16d49` and the exact Nickel 1.17.0 evaluator cohort. Release checks MUST reject floating or local overrides.

r[trellis.nickel_export_cutover.boundary]
Only Trellis checker shells MAY invoke the exporter, read source files, run Nickel, or write destinations. Verified modules and pure proof models MUST NOT gain that authority.

r[trellis.nickel_export_cutover.authority]
Trellis MUST retain policy meaning, proof-corpus semantics, Verus and Tracey gates, fixture expectations, destinations, and release decisions after export.

r[trellis.nickel_export_cutover.dual_run]
Legacy and standalone paths MUST use identical declared inputs. Verification policy, lifecycle YAML, and JSON contract outputs MUST match by exact bytes and BLAKE3 identity.

r[trellis.nickel_export_cutover.rollback]
Legacy checkers MUST remain required through one complete positive and negative cycle. Any output or diagnostic drift MUST keep legacy behavior authoritative and fail closed.

r[trellis.nickel_export_cutover.validation]
Validation MUST cover exact output, freshness, malformed requests, invalid contracts, stale and tampered outputs, path escape, symlinks, evaluator drift, secret markers, and proof or release overclaims.
