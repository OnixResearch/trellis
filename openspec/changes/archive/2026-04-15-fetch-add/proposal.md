## Why

The existing atomic primitive in this repo is compare-and-swap (`cas.rs`,
`cas_loop.rs`). CAS is universal but not the only primitive that matters.
Fetch-and-add (FAA) is the other workhorse of lock-free programming — it
atomically reads the old value and replaces it with old + addend. Ticket
locks, reference counts, sequence numbers, statistics counters, and
lock-free queue indices all rely on fetch-and-add rather than CAS loops
because FAA is wait-free: it always completes in one step with no retry.

The interesting verification properties are different from CAS. CAS is
conditional (succeed or fail); FAA is unconditional. CAS is vulnerable to
ABA; FAA is not. CAS composes via retry loops; FAA composes via algebraic
properties — commutativity (concurrent FAAs produce the same final value
regardless of execution order) and associativity (sequential FAAs can be
collapsed). These algebraic properties are what make FAA safe to use without
ordering constraints in many concurrent algorithms.

## What Changes

- **fetch_add**: A new module modeling the atomic fetch-and-add operation
  over `u64` values with both wrapping and saturating variants.

  Proves:
  - **Return semantics**: FAA returns the old value; the cell holds old +
    addend afterward.
  - **Identity**: FAA with addend 0 is a no-op — returns the current value,
    cell unchanged.
  - **Commutativity**: Two concurrent FAAs produce the same final value
    regardless of which executes first. `faa(a); faa(b)` and `faa(b); faa(a)`
    yield the same cell content.
  - **Associativity**: Sequential `faa(a); faa(b)` on the final cell value
    equals a single `faa(a + b)`.
  - **Monotonicity**: When the addend is positive and no wrapping occurs,
    the cell value strictly increases.
  - **CAS refinement**: FAA can be expressed as a CAS loop that always
    eventually succeeds. Proves the CAS-loop implementation refines the
    atomic FAA spec.
  - **Wrapping correctness**: The wrapping variant satisfies modular
    arithmetic identities.
  - **Saturation correctness**: The saturating variant clamps at the
    maximum value and is idempotent once saturated.
  - **Fetch-sub duality**: Fetch-and-subtract is FAA with the two's
    complement negation. Proves the round-trip: `faa(n); fas(n)` restores
    the original value (when no wrapping occurs).

## Capabilities

### New Capabilities

- `fetch-add.semantics`: Verified return-old / store-new behavior
- `fetch-add.identity`: Verified zero-addend no-op
- `fetch-add.commutativity`: Verified execution-order independence
- `fetch-add.associativity`: Verified sequential collapse
- `fetch-add.monotonicity`: Verified strict increase under no-wrap conditions
- `fetch-add.cas-refinement`: Verified CAS-loop equivalence
- `fetch-add.wrapping`: Verified modular arithmetic for wrapping variant
- `fetch-add.saturating`: Verified clamping behavior for saturating variant
- `fetch-add.fetch-sub`: Verified add/sub duality and round-trip

## Impact

- **Files**: One new `src/fetch_add.rs`, module declaration in `lib.rs`
- **APIs**: `fetch_add_wrapping`, `fetch_add_saturating`, `fetch_sub_wrapping`,
  `fetch_sub_saturating`, plus spec-level `faa_result_spec` and algebraic
  proof lemmas
- **Dependencies**: Imports `cas_result_spec` from `cas_loop.rs` for the
  refinement proof, depends on `vstd`
- **Testing**: Verus proofs for all algebraic and refinement properties,
  proptest for runtime sanity checks on wrapping arithmetic and commutativity
