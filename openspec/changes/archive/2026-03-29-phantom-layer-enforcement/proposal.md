## Why

Verus `requires`/`ensures` catches misuse at verification time but
only if the caller is also verified. A downstream crate that skips
Verus verification gets no protection — nothing stops it from passing
an unverified `SnapshotIntegrity` to code that assumes it was checked,
or committing a log entry that was never type-checked against the
leader's term.

The phantom type trick, borrowed from HashMath's `Expr<Empty>` vs
`Expr<Hash>`, makes certain states unrepresentable at the Rust type
level. A `Verified<T>` wrapper can only be constructed by calling the
verification function — no backdoor, no `unsafe`, no Verus needed in
the consumer. The type system carries the proof obligation across crate
boundaries where Verus can't reach.

This is the difference between "the verifier checks that you called
verify" (Verus) and "you literally cannot have a Verified<T> without
calling verify" (phantom types). Both are useful; the second works
even when the caller isn't verified.

## What Changes

A new `typed_state` module providing:

**Marker types and sealed construction**

- `Unverified` / `Verified` — zero-sized marker types
- `Stamped<S, T>` — a wrapper that pairs data `T` with a phantom
  state marker `S`. `Stamped<Unverified, T>` is freely constructable.
  `Stamped<Verified, T>` can only be produced by a transition function
  that takes `Stamped<Unverified, T>` and a predicate.
- The `Verified` marker's constructor is private — downstream crates
  cannot forge a `Stamped<Verified, T>`.

**Transition functions**

- `stamp` — wraps raw data as `Stamped<Unverified, T>`
- `verify_with` — takes `Stamped<Unverified, T>` and a predicate
  `fn(&T) -> bool`, returns `Option<Stamped<Verified, T>>`. Returns
  `None` if the predicate fails.
- `unwrap_verified` — extracts the inner `T` from `Stamped<Verified, T>`.
  Only callable on verified data.
- `inner` — read access to `&T` regardless of state (for inspection
  before verification).

**Verus proofs**

- `verify_then_unwrap` roundtrip: stamp → verify_with(p) → unwrap
  returns the original value when the predicate holds
- `verify_implies_predicate`: if `verify_with(p)` returns `Some`,
  the predicate held at verification time
- `no_forge`: `Stamped<Verified, T>` cannot exist without going
  through `verify_with` (structural — the private constructor means
  no proof needed, just a compilation test)
- `stamp_preserves_value`: stamp(v).inner() == v

**Concrete instantiation for snapshot integrity**

- `verify_snapshot` — takes `Stamped<Unverified, SnapshotIntegrity>`,
  calls `verify_integrity`, returns `Stamped<Verified, SnapshotIntegrity>`
  or `None`
- Cross-module proof: `verify_snapshot` composes `typed_state::verify_with`
  with `snapshot_integrity::verify_integrity`

## Capabilities

### New Capabilities
- `typed-state.stamp`: Wrap raw data as unverified
- `typed-state.verify`: Transition from unverified to verified via predicate
- `typed-state.unwrap`: Extract verified data
- `typed-state.roundtrip`: stamp → verify → unwrap identity
- `typed-state.predicate-holds`: verification implies predicate held
- `typed-state.snapshot-compose`: concrete instantiation for snapshots

### Modified Capabilities
- None — additive only

## Impact

- **Files**: `src/typed_state.rs` (new), `src/lib.rs` (module decl),
  `src/properties.rs` (snapshot composition proof)
- **APIs**: ~5 spec fns, ~5 exec fns, ~5 proof fns
- **Dependencies**: Uses `snapshot_integrity::verify_integrity` for the
  concrete instantiation
- **Testing**: Verus proofs + proptest + compilation tests for forge
  prevention
