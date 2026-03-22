## Context

The verified-logic library provides Verus-verified building blocks. All
existing modules follow the same pattern: `spec fn` defines the math,
`exec fn` with `ensures result == *_spec(args)` is the runtime code,
`proof fn` establishes algebraic properties. Tests use proptest oracles.

Nix builtins are pure functions with well-defined semantics documented
informally in the Nix manual. No existing formal specification exists.
The tvix project re-implements Nix in Rust but without formal verification.

## Goals / Non-Goals

**Goals:**
- Verified implementations of core Nix builtin operations
- Machine-checked algebraic properties (ordering laws, functor laws, etc.)
- Executable reference that Nix implementations can test against
- Coverage of the operations most prone to bugs: version comparison,
  attribute set merging, generic closure

**Non-Goals:**
- Full Nix evaluator — only standalone pure operations
- Nix syntax parsing — inputs are already-parsed structures
- Lazy evaluation semantics — all operations are strict
- Nix type system / type errors — inputs are well-typed by construction
- Floating point operations — Nix has them but they aren't interesting to verify
- I/O builtins (readFile, fetchurl, etc.) — not pure

## Decisions

### 1. Type Representations

**Choice:** Model Nix types using Verus-native types:
- Strings → `Seq<u8>` (spec) / `Vec<u8>` (exec)
- Lists → `Seq<T>` (spec) / `Vec<T>` (exec)
- Attribute sets → `Seq<(Seq<u8>, V)>` sorted by key (spec) / `Vec<(Vec<u8>, V)>` (exec)
- Integers → `u64` / `i64` (matching existing module conventions)
- Paths → `Seq<u8>` with separator semantics

**Rationale:** These types have first-class Verus support with built-in
axioms for indexing, slicing, and equality. Using `Seq` directly avoids
needing to define custom ghost types or build translation layers. The
sorted key-value sequence for attribute sets mirrors `collections.sorted_insert`.

**Alternative:** Use `Map<Seq<u8>, V>` from vstd for attribute sets.
Rejected because Map is unordered — `builtins.attrNames` returns sorted
keys, so the sorted-sequence representation is more natural and avoids
a sort step in `attr_names`.

### 2. Version String Comparison Strategy

**Choice:** Split version strings into components at `.` and `-`
boundaries, classify each component as numeric or alpha, compare
pairwise. Implement as a loop over component pairs.

**Rationale:** This matches the C implementation in Nix's
`compareVersions` exactly. The component-by-component approach produces
natural loop invariants: "all components compared so far were equal."

**Implementation:**
```
spec fn compare_versions_spec(a: Seq<u8>, b: Seq<u8>) -> int
exec fn compare_versions(a: &Vec<u8>, b: &Vec<u8>) -> (result: i8)
    ensures result == compare_versions_spec(a@, b@)
proof fn compare_versions_total(a: Seq<u8>, b: Seq<u8>, c: Seq<u8>)
    ensures compare_versions_spec(a, b) + compare_versions_spec(b, a) == 0  // antisymmetric
proof fn compare_versions_transitive(a: Seq<u8>, b: Seq<u8>, c: Seq<u8>)
    requires compare_versions_spec(a, b) <= 0, compare_versions_spec(b, c) <= 0
    ensures compare_versions_spec(a, c) <= 0
```

### 3. Attribute Set Invariant Enforcement

**Choice:** The sorted-key invariant is a precondition (`requires`) on
all attribute set functions, not an internally-maintained struct invariant.

**Rationale:** Using `requires is_sorted_keys(attrs)` on each function
is the same pattern as `collections.sorted_insert`. It keeps the type
simple (just a `Vec` of pairs) and avoids the Verus struct-field-opacity
problem noted in the napkin. The constructor `from_unsorted` sorts and
deduplicates, establishing the invariant for callers.

**Alternative:** Wrap in a `NixAttrSet` struct with a private `data` field
and a `pub open spec fn well_formed` predicate. Rejected because every
field would need to be `pub` anyway (Verus opaque rule), eliminating the
encapsulation benefit.

### 4. Generic Closure Termination

**Choice:** Model `builtins.genericClosure` with an explicit fuel
parameter (max iterations) to guarantee termination. The fuel must be
>= the size of the key space for the closure to be complete.

**Rationale:** Verus requires termination proofs for recursive functions.
Nix's `genericClosure` terminates because keys are finite and each key
is visited at most once. But proving this in Verus requires bounding
the recursion. An explicit fuel parameter makes the bound visible in
the spec and avoids `decreases` complexity.

**Implementation:**
```
spec fn generic_closure_spec(start: Seq<(K, V)>, op: spec_fn((K, V)) -> Seq<(K, V)>, fuel: nat) -> Seq<(K, V)>
    decreases fuel
```

The `fuel` parameter decreases on each recursive call. When fuel reaches
zero, the function returns the current accumulated set. A proof function
establishes that for fuel >= key_space_size, the result is the true
fixed point.

**Alternative:** Use `decreases` on the size of unseen keys. Rejected
because it requires carrying a ghost set of all possible keys, which
doesn't have a natural upper bound in the general case.

### 5. Module Naming

**Choice:** Prefix all modules with `nix_`: `nix_version`, `nix_string`,
`nix_attrset`, `nix_attrpath`, `nix_list`, `nix_sort`, `nix_partition`,
`nix_closure`.

**Rationale:** Avoids collision with existing modules (`collections`,
`ordering`, `slice`, `partition`). Makes the Nix-specific scope obvious
in `src/lib.rs`. Groups naturally in file listings.

### 6. Cross-Module Property Placement

**Choice:** Nix cross-module properties go in `src/properties.rs`
alongside the existing cross-module proofs, inside a
`#[cfg(verus_keep_ghost)]` block.

**Rationale:** Follows the established pattern from the existing
properties module. Keeps all cross-module proofs in one place rather
than scattering them across Nix modules. The cfg gate ensures spec-only
imports compile cleanly.

### 7. FnSpec Usage for Higher-Order Functions

**Choice:** Use `spec_fn` (formerly `FnSpec`) for comparator and
predicate parameters in sort, filter, partition, map, fold.

**Rationale:** Verus requires spec-level function types for use in
`ensures` clauses. `spec_fn` is the current Verus API (the napkin
records that `FnSpec` is deprecated). The exec-level functions take
concrete function pointers or closures and the ensures clause references
the spec_fn version.

**Implementation:**
```
spec fn sort_spec(list: Seq<T>, cmp: spec_fn(T, T) -> bool) -> Seq<T>
exec fn sort(list: &Vec<T>, cmp: impl Fn(&T, &T) -> bool) -> (result: Vec<T>)
    ensures result@ == sort_spec(list@, |a, b| cmp_spec(a, b))
```

## Risks / Trade-offs

**[Version comparison complexity]** The Nix version comparison algorithm
has edge cases around pre-release suffixes, empty components, and mixed
alphanumeric segments. The loop invariant for pairwise component
comparison will be the hardest proof in the change. Mitigation: extensive
proptest oracles against the reference Nix implementation via
`nix-instantiate --eval`.

**[Attribute set merge associativity]** The associativity proof for merge
requires induction over three sorted sequences simultaneously. This is
more complex than any existing proof in the library. Mitigation: break
into lemmas — first prove merge preserves sortedness, then prove
associativity using the sorted invariant.

**[Generic closure fuel bound]** The fuel-based termination model is
sound but less elegant than a true key-set decreases proof. Callers
must provide adequate fuel. Mitigation: document the fuel requirement
clearly and provide a convenience wrapper that computes fuel from the
start set size.

**[Higher-order function verification]** Verus support for higher-order
exec functions with spec_fn ensures is less mature than first-order
verification. Some proof patterns may require `#[verifier::external_body]`
with proptest backup. Mitigation: start with the simplest case (map)
and establish the pattern before tackling fold and sort.

**[Module count]** Eight new modules is the largest single addition to
the library. Risk of scope creep. Mitigation: strict tier ordering —
Tier 1 must be complete and verified before starting Tier 2.
