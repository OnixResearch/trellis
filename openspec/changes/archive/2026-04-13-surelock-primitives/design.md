## Context

Surelock prevents deadlocks by breaking circular wait with two mechanisms:
canonical same-level acquisition and compile-time upward-only level ordering.
The external crate expresses those rules through Rust traits, lifetimes,
compile-fail tests, and a small amount of runtime sorting. This repo is a
better fit for the underlying semantic core than for the exact surface API.

The useful verification target is therefore the core model:

- each lock has a stable identity and a level
- each key carries the highest level reached so far
- each lock operation must start at or above the current key level
- same-level groups are acquired through a duplicate-free canonical order
- admitted waits always move forward in a well-founded order

If those facts hold, circular wait is impossible.

## Goals / Non-Goals

**Goals:**
- Prove lock IDs are unique, monotonic, and induce a deterministic order
- Prove level ordering is a strict total order with correct `new_higher`
  derivation
- Prove key transitions are upward-only and subscope-preserving
- Prove lock-set canonicalization sorts same-level locks and rejects duplicates
- Prove any admitted acquisition trace is acyclic

**Non-Goals:**
- Mirror surelock's exact trait bounds, lifetimes, or macro-generated impls
- Verify backend-specific blocking, spinning, poisoning, or `thread_local!`
  behavior
- Model every ergonomic wrapper in the external crate (`Locksmith`,
  `KeyVoucher`, ambient scope helpers)
- Prove Rust compile errors directly; instead prove the semantic predicate that
  those compile errors enforce
- Cover the external crate's roadmap items

## Decisions

### 1. Verify a core semantic model instead of the external API

**Choice:** Introduce new verified modules that model locks, levels, keys, and
canonical acquisition plans directly.

**Rationale:** The external crate relies on Rust type-system features that are
useful for ergonomics but not themselves the interesting theorem. The theorem
is that admitted acquisitions cannot form a cycle. A direct model keeps the
proof burden on the concurrency logic rather than on Rust surface mechanics.

**Alternative:** Recreate surelock's exact API in Verus. Rejected because the
borrow-checker and compile-fail parts are not the right proof interface for
this repo.

### 2. Use `(level, id)` as the forward-progress rank

**Choice:** Define a well-founded order from lock metadata: first by level,
then by lock ID within a level.

**Rationale:** Cross-level acquisition moves forward by level. Same-level
lock sets move forward by canonical ID order. A single lexicographic rank lets
us phrase the no-cycle proof as "every wait edge strictly increases rank".

**Alternative:** Prove separate theorems for same-level and cross-level waits
without a shared rank. Rejected because the composition proof becomes more
awkward.

### 3. Keep `LockSet` verification focused on same-level groups

**Choice:** The first change verifies duplicate-free canonical ordering for
same-level groups.

**Rationale:** That is the core case described in the surelock README and blog:
multiple sibling locks acquired atomically after sorting by stable ID. It is
also the cleanest place to prove membership preservation, duplicate rejection,
and canonical ordering.

**Alternative:** Include every special case from the external crate, including
mixed-level 2-tuples, in the first pass. Rejected because it expands the model
before the core theorem is in place.

### 4. Model key flow as explicit state transitions

**Choice:** Represent a key as a witness carrying the current level. Single
lock and lock-set operations consume a key at level `L` and return a key at the
maximum level of the acquired lock group. `subscope` returns a key at the same
level it entered.

**Rationale:** This matches the external crate's "consumed and re-emitted"
shape while staying easy to reason about in Verus.

**Alternative:** Encode key behavior only as global trace predicates.
Rejected because local transition lemmas are easier to reuse.

### 5. Prove deadlock-freedom through circular-wait exclusion

**Choice:** State the top theorem in terms of wait-for graphs: any graph built
from admitted acquisitions is acyclic.

**Rationale:** This matches surelock's public claim and the Coffman-condition
argument in the blog post. It also composes well with existing graph and cycle
proof patterns already present in this repo.

**Alternative:** Prove only pairwise ordering facts and leave the graph theorem
implicit. Rejected because the user-facing claim is about deadlocks, not just
local ordering.

## Risks / Trade-offs

**[Core model drifts from upstream surelock]** -> Mitigate by anchoring the
spec to the documented primitives in the README, design docs, blog post, and
compile-fail examples, not to undocumented internals.

**[Same-level-only `LockSet` scope is narrower than the full crate]** -> State
that limit explicitly in the proposal and tasks so the first verified slice is
honest.

**[Deadlock theorem depends on the chosen rank definition]** -> Keep the rank
small and concrete: `(level, id)` only. Avoid scheduler assumptions.

**[API wrappers may still need follow-on work]** -> Treat wrappers such as
`KeyHandle` and `Locksmith` as a later change once the core logic is stable.
