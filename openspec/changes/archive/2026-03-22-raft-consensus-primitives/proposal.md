## Why

Raft consensus correctness chains through a small set of mathematical
arguments: quorum intersection guarantees leader uniqueness, monotonic
terms prevent stale leaders, log matching ensures replicas converge, and
epoch comparison decides elections. These arguments are reusable across
any Raft implementation — openraft, tikv/raft-rs, custom implementations.

Today verified-logic has arithmetic, bitwise, bounds, collections, and
data structures. None of it addresses the distributed consensus domain.
Adding verified consensus primitives turns the library into a foundation
for proving properties about real systems (e.g., the aspen Raft cluster).

## What Changes

Four new modules, each self-contained with spec fns, exec fns, proofs,
and property tests:

- **`quorum`** — majority threshold, quorum predicates, and the
  intersection theorem (two quorums of the same set always share a member).
  This is the mathematical heart of Raft's safety proof.

- **`monotonic`** — predicates and proofs for monotonically increasing
  values. Terms only increase, commit indices only advance. Composition:
  if a ≤ b and b ≤ c then a ≤ c. Bounded advancement.

- **`log`** — append-only log operations with term-tagged entries.
  Prefix matching, log comparison for elections, and the Log Matching
  Property (agreement at index i implies agreement at all j < i).

- **`epoch`** — lexicographic (term, index) comparison for determining
  which log is more up-to-date. Total ordering proofs, transitivity,
  antisymmetry.

Cross-module properties linking these to existing modules:
- `quorum::majority` ↔ `arithmetic::div_ceil`
- `epoch` comparison ↔ `ordering::compare_and_swap`
- `log` prefix matching ↔ `slice::prefix_eq`

## Capabilities

### New Capabilities
- `quorum-intersection`: Machine-checked proof that two majorities overlap
- `monotonic-values`: Verified monotonicity predicates with composition
- `log-matching`: Verified append-only log with the Log Matching Property
- `epoch-ordering`: Verified lexicographic (term, index) total order

### Modified Capabilities
- `cross-module-properties`: New properties linking consensus modules to
  existing arithmetic, ordering, and slice modules

## Impact

- **Files**: src/quorum.rs, src/monotonic.rs, src/log.rs, src/epoch.rs,
  src/properties.rs (additions), src/lib.rs, docs/requirements.md
- **APIs**: ~20 new pub fns across 4 modules
- **Dependencies**: None — pure logic using existing vstd
- **Testing**: Verus verification + proptest property tests + model oracles
