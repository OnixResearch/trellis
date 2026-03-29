## Context

Verus postconditions enforce invariants within verified code but
can't protect against misuse by unverified callers. Phantom type
parameters encode state into the type system, giving compile-time
enforcement that crosses crate boundaries without Verus.

HashMath uses `Expr<Empty>` (kernel) vs `Expr<Hash>` (storage) to
make `href` nodes unconstructable in the kernel. We adapt the same
idea to a generic `Stamped<State, T>` wrapper.

## Goals / Non-Goals

**Goals:** Provide a reusable typed-state wrapper that downstream
crates can use to enforce verification pipelines at compile time.
Prove the key properties (roundtrip, predicate preservation) in
Verus. Show a concrete instantiation with snapshot_integrity.

**Non-Goals:** Replacing Verus ensures clauses — they're complementary.
Not building a full typestate library with arbitrary state machines.
Two states (Unverified/Verified) covers the dominant pattern.

## Decisions

### 1. Generic wrapper, not per-module newtypes

**Choice:** One `Stamped<S, T>` type parameterized over state and
payload, not separate `UnverifiedSnapshot` / `VerifiedSnapshot` types
per module.
**Rationale:** The pattern is identical across all modules — only the
predicate differs. A generic wrapper avoids N copies of the same
boilerplate.
**Alternative:** Per-module newtypes. Rejected — duplicates the
transition logic. If we find a module needs custom state beyond
Verified/Unverified, we can add it later.

### 2. Private Verified constructor, public Unverified

**Choice:** `Unverified` is a public zero-sized struct anyone can
name. `Verified` is also public (for type annotations) but its
presence inside `Stamped` can only be created by `verify_with`.
This is enforced by making the `Stamped` struct's fields private
and only exposing constructors that enforce the invariant.
**Rationale:** Downstream code needs to write `Stamped<Verified, T>`
in function signatures but must not be able to construct it directly.
Private fields + public type name achieves this.

### 3. Predicate is exec fn, not spec fn

**Choice:** `verify_with` takes `fn(&T) -> bool`, not a spec-level
predicate.
**Rationale:** The verification must happen at runtime. Spec-level
predicates are erased. The Verus proof shows that when `verify_with`
returns `Some`, the exec predicate returned `true`. The connection
between the exec predicate and a spec-level property is the caller's
responsibility (they proved their predicate's ensures clause).

### 4. Verus models Stamped as struct with ghost state

**Choice:** Inside `verus!`, model `Stamped` with a ghost `verified: bool`
field. Proof functions reason about the ghost field. Exec functions
maintain the real/ghost correspondence.
**Rationale:** Verus can't directly reason about phantom type
parameters. A ghost bool is the standard encoding — `verified == true`
corresponds to `S = Verified` in the type system. The type-level
enforcement is structural (tested by compilation), the logical
properties are proved via the ghost field.

### 5. Concrete snapshot instantiation in properties.rs

**Choice:** The `verify_snapshot` function that composes `typed_state`
with `snapshot_integrity` goes in `properties.rs` as a cross-module
proof, not in `typed_state.rs`.
**Rationale:** `typed_state` should be dependency-free. The
composition proof imports both modules and lives where other
cross-module proofs live.

## Risks / Trade-offs

**Verus can't verify phantom type properties** → The "no forge"
guarantee is structural (private fields) not logical (Verus proof).
We test it with a compile-fail test. This is fine — the type system
guarantee is stronger than a Verus proof because it holds even in
unverified code.

**Generic predicate loses specificity** → `verify_with` takes any
`fn(&T) -> bool`. The Verus proof only knows "the predicate returned
true", not which specific property was checked. This is by design —
the caller chooses and proves their predicate separately.
