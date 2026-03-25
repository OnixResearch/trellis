# Verification Gaps Specification

## Purpose

Tracks every `#[verifier::external_body]` item and `closed spec fn` in
the codebase. Each gap is classified as closeable or permanent, with a
rationale. The goal is to minimize trusted code — every external_body is
an unverified assertion that Verus accepts on faith.

## Requirements

### Cryptographic Hash Axioms

#### Requirement: Hash Functions Are Uninterpreted

`chain::hash_spec` and `snapshot_integrity::dual_hash_spec` MUST remain
`closed spec fn` with no definition body. They model cryptographic hash
functions (Blake3) whose internals are deliberately opaque to the prover.

The axioms `hash_deterministic` and `dual_hash_deterministic` assert
that equal inputs produce equal outputs. The axioms `hash_collision_free`
and `dual_hash_collision_resistant` assert the converse. These are the
standard cryptographic assumptions and cannot be proved from first
principles.

The exec wrappers `compute_entry_hash` and `dual_hash` MUST remain
external_body because their ensures clauses reference closed specs
that Z3 cannot inspect.

##### Classification: Permanent

These 8 items (2 closed specs, 4 proof axioms, 2 exec fns) are
permanent. They encode the irreducible trust boundary between formal
verification and cryptographic assumptions.

### Rate Limit Nonlinear Arithmetic

#### Requirement: Refill Calculation Correctness

`tokens_after_refill(tokens, elapsed_ms, rate_per_sec, burst)` MUST
return `min(tokens + elapsed_ms * rate_per_sec / 1000, burst)`. The
exec fn splits the multiplication to avoid u64 overflow:
`(elapsed_ms / 1000) * rate + (elapsed_ms % 1000) * rate / 1000`.
The split identity `a*c/b == (a/b)*c + (a%b)*c/b` MUST be proved
using vstd's `lemma_fundamental_div_mod` and `lemma_mul_is_distributive`.

##### Scenario: Split identity

- GIVEN elapsed_ms=2500, rate_per_sec=100
- WHEN the split computation runs
- THEN (2500/1000)*100 + (2500%1000)*100/1000 == 250 == 2500*100/1000

#### Requirement: Wait Time Correctness

`tokens_to_wait_ms(tokens, cost, rate_per_sec)` MUST return
`ceil((cost - tokens) * 1000 / rate_per_sec)` when tokens < cost.
The exec fn guards against overflow with division-based checks.
The overflow guard `shortage > u64::MAX / 1000` MUST be proved
equivalent to `shortage * 1000 > u64::MAX`.

#### Requirement: Wait Enables Consumption

After waiting `tokens_to_wait_ms` milliseconds and refilling, the
resulting token count MUST be >= cost. This chains the refill and
wait specs and requires nonlinear reasoning about ceiling division
rounding up sufficiently.

##### Classification: Closeable (3 items)

The mathematical identities are standard. vstd provides the necessary
lemmas. Prior attempts failed on connecting int-spec arithmetic to
overflow-guarded exec paths. A step-by-step proof with intermediate
assertions should work.

### Version Comparison Unfolding

#### Requirement: Version Comparison Properties

`compare_versions` compares Nix version strings byte-by-byte with
numeric and lexicographic sub-comparisons. The spec `cv_fuel` uses
fuel-bounded recursion with 8+ levels of nested let/if/else.

The proofs `cv_fuel_antisymmetric`, `cv_fuel_reflexive`, and
`cv_fuel_range` have correct inductive structure but Z3 cannot unfold
the spec deeply enough. At each recursion level, Z3 must case-split
on separator detection, digit detection, numeric parsing, and
lexicographic comparison — producing combinatorial explosion in
quantifier instantiation.

##### Classification: Permanent (4 items)

The spec is open and the proof bodies are preserved as documentation.
The limitation is Z3's quantifier instantiation budget, not a logical
gap. Proptest covers runtime correctness.

### Buddy Allocator Theory Bridge

#### Requirement: Order-for-Size Correctness

`order_for_size(size)` MUST return the minimum order where
`block_size(order) >= size`. The exec fn uses a shift loop that
proves `1u64 << order >= size` via bit_vector theory. However,
`block_size_spec(order)` defines `1u64 << order` at the int level.
Z3 cannot bridge between bitvector-level `<<` and integer-level `<<`.

##### Classification: Permanent (1 item)

This is a known Z3 theory bridge limitation. The bitvector solver
operates on fixed-width representations while the integer solver
works with unbounded integers. Neither can reason about the other's
shift operator. Proptest covers runtime correctness.

### Glob Pattern Matching

#### Requirement: Glob Match Specification

`glob_match_spec` MUST define the full semantics of glob pattern
matching with `*` wildcards. The current spec returns `true` for any
starred pattern — a degenerate definition that makes the ensures
clause vacuously true.

A correct spec MUST:
1. Split the pattern by `*` into segments
2. Require the first segment anchors at the input start
3. Require the last segment anchors at the input end
4. Require each middle segment appears in order in the input
5. Handle adjacent `*` (empty segments) correctly
6. Match exact (no-star) patterns by equality

##### Scenario: Multi-star pattern

- GIVEN pattern `a*b*c` and input `axbyzc`
- WHEN glob_match is applied
- THEN it MUST return true (a=prefix, c=suffix, b found in middle)

##### Scenario: Anchoring

- GIVEN pattern `*.txt` and input `readme.txt`
- WHEN glob_match is applied
- THEN it MUST return true (empty prefix, .txt suffix)

##### Classification: Closeable (1 item)

Requires writing a recursive spec over `Seq<u8>` with segment
splitting. The exec fn already implements the correct algorithm.

### String Replacement

#### Requirement: Replace Strings Specification

`replace_strings_spec` MUST be opened with a recursive definition
that scans left-to-right, checking each pattern at the current
position in priority order (first match wins), advancing past the
match, and copying unmatched bytes verbatim.

The spec MUST handle:
1. Empty patterns (match everywhere, insert replacement before each byte)
2. Overlapping patterns (first-listed wins at each position)
3. Non-overlapping replacement (advance past full match length)
4. No match (copy one byte and advance)

##### Scenario: First match wins

- GIVEN s="aab", from=["a","aa"], to=["x","y"]
- WHEN replace_strings is applied
- THEN result MUST be "xab" (not "yb") because "a" is checked first

##### Scenario: Empty pattern

- GIVEN s="ab", from=[""], to=["x"]
- WHEN replace_strings is applied
- THEN result MUST be "xaxb" (insert x before each byte)

##### Classification: Closeable (1 item)

Requires opening the closed spec with a position-indexed recursive
definition, then proving the exec loop matches the spec.
