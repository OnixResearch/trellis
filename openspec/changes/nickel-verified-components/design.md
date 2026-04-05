## Context

verified-logic already has 5 Nix modules (`nix_attrset`, `nix_list`,
`nix_string`, `nix_version`, `nix_closure`) that model Nix language
builtins. Nickel extends Nix's data model with merge priorities, gradual
typing, and contract blame tracking. The Nickel interpreter implements
these in `core/src/eval/merge.rs`, `core/src/typecheck/subtyping.rs`,
`core/src/label.rs`, and `core/src/term/pattern/compile.rs`.

These components have clear algebraic properties (total orders, lattice
bounds, associativity, involutions) that map directly to Verus spec/exec/proof
patterns.

## Goals / Non-Goals

**Goals:**
- Machine-checked reference implementations of Nickel merge, priority,
  subtyping, polarity, and pattern coverage semantics
- Proofs of algebraic properties (associativity, transitivity, identity,
  involution) that the Nickel runtime assumes but doesn't verify
- Proptest oracles matching each verified property

**Non-Goals:**
- Replacing or linking into the Nickel interpreter
- Modeling Nickel's full AST, evaluation, or closure semantics
- Verifying Nickel's parser or serialization

## Decisions

### 1. Model records as sorted key-value sequences

**Choice:** Reuse the `Seq<(Seq<u8>, u64)>` representation from `nix_attrset`
for record keys. Fields carry a priority tag alongside the value.

**Rationale:** The existing `nix_attrset` sorted-key invariant gives us
binary search, sorted merge, and intersection for free. Nickel's runtime
uses `IndexMap` (insertion-ordered), but for verification purposes the
ordering of keys doesn't affect merge correctness — only the key set
membership and priority resolution matter.

**Alternative:** Hash-map model. Rejected because Verus reasons better about
sequences and sorted invariants than hash-based structures.

### 2. Represent MergePriority as an enum with i64 numerals

**Choice:** Model `MergePriority` as a four-variant enum:
`Bottom | Neutral | Numeral(i64) | Top`. Implement `Ord` via a spec fn
that maps to integers: Bottom → i64::MIN, Neutral → 0, Numeral(n) → n,
Top → i64::MAX.

**Rationale:** Nickel's `MergePriority` uses arbitrary-precision `Number`
for numerals. For verification, i64 is sufficient — the algebraic
properties (total order, extrema) hold regardless of the numeric range,
and i64 gives Z3 native bitvector reasoning.

**Alternative:** Use `int` (mathematical integers) in specs. Considered,
but exec fns need a concrete type, and i64 keeps spec/exec correspondence
tight.

### 3. Type representation as a recursive enum

**Choice:** Model Nickel types as a Verus enum:
`Num | Bool | Str | Array(Type) | Dict(Type) | Record(fields) | Enum(tags) | Fun(Type, Type) | Dyn`.

**Rationale:** Nickel's `TypeF` is a large generic enum. We only need the
subset relevant to subtyping rules. The recursive structure maps to Verus
`decreases` proofs on type depth.

**Implementation:** Subtyping is a recursive function with `decreases`
on the sum of type depths. Each structural rule (array covariance, record
width, record-dict subsumption) is a match arm.

### 4. Polarity as a two-element type with flip

**Choice:** `enum Polarity { Positive, Negative }` with `flip` as
the only operation.

**Rationale:** Matches Nickel's `Polarity` enum exactly. The double-flip
identity (`flip(flip(p)) == p`) is a one-line Verus proof. Type path
tracking counts domain steps and proves parity.

### 5. Pattern coverage via set difference

**Choice:** Model pattern sets as sequences of pattern descriptors.
Coverage checking computes the uncovered set by subtracting each pattern
from the type's value space. Empty uncovered set → complete.

**Rationale:** This is simpler than full usefulness checking (Maranget's
algorithm) and sufficient for Nickel's enum and record patterns. The
uncovered-set approach maps to `Seq` operations that Verus handles well.

## Risks / Trade-offs

**[Recursive type depth]** Subtyping proofs on recursive types need
explicit `decreases` annotations. Deeply nested types may require fuel
bounds. → Keep the modeled type depth small; prove properties for
arbitrary depth via induction.

**[Merge associativity scope]** Full associativity with mixed priorities
is complex — priority resolution at equal levels requires value equality,
which is undecidable for general Nickel terms. → Prove associativity for
the uniform-priority case (all Neutral) and the disjoint-keys case.
Document the mixed-priority limitation.

**[Pattern coverage generality]** Nickel patterns include or-patterns,
guards, and nested records. Full coverage checking is equivalent to the
satisfiability problem for the pattern language. → Start with enum
coverage and wildcard completeness. Extend later if needed.
