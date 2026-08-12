## Why

The library covers arithmetic, data structures, consensus, and distributed
systems primitives. But a large class of pure functions with precise
mathematical semantics remains unverified: the Nix language builtins.

Nix builtins are pure functions over strings, lists, attribute sets, paths,
and versions. Their semantics are well-defined but only documented informally
in the Nix manual. Bugs in version comparison, store path calculation,
attribute set merging, and string manipulation have caused real breakage in
nixpkgs. Machine-checked specifications of these operations would serve as
an executable reference that downstream Nix tooling (lix, tvix, harmonia)
can verify against.

The operations break into four groups:

1. **Version comparison** — `builtins.compareVersions` and
   `builtins.parseDrvName` implement a non-trivial ordering over version
   strings (dot/dash-separated components, numeric vs lexicographic
   comparison per component). This ordering must be total, transitive,
   and antisymmetric. Getting it wrong breaks `lib.versionAtLeast` and
   every override that depends on version predicates.

2. **Store path arithmetic** — hash truncation, base32 encoding, path
   composition. These are the primitives behind `builtins.hashString`,
   `builtins.path`, and derivation output path calculation. Determinism
   and injectivity properties are critical for content-addressable storage.

3. **Attribute set operations** — `//` (merge), `builtins.intersectAttrs`,
   `builtins.removeAttrs`, `builtins.attrNames`, `builtins.listToAttrs`.
   These have algebraic properties (merge associativity, intersection
   commutativity, removeAttrs as complement) that nixpkgs relies on
   implicitly in overlay composition and module merging.

4. **List operations** — `builtins.map`, `builtins.filter`,
   `builtins.foldl'`, `builtins.sort`, `builtins.partition`,
   `builtins.concatLists`, `builtins.genList`, `builtins.groupBy`,
   `builtins.genericClosure`. Functor laws, fold fusion, sort stability,
   and closure termination are all provable properties.

All four groups map directly to existing Verus patterns. Version comparison
is a lexicographic ordering (like `epoch`). Store path hashing uses the
`chain` module's determinism axiom pattern. Attribute sets are sorted
key-value maps (like `collections`). List operations are sequence
transformations over `Seq` (like `slice`).

## What Changes

Eight new modules in three tiers, plus cross-module properties:

**Tier 1 — Version and string primitives**

- `nix_version` — version string comparison matching `builtins.compareVersions`
  semantics. Component splitting, numeric-vs-alpha classification, total ordering
  proofs.
- `nix_string` — `builtins.substring`, `builtins.stringLength`,
  `builtins.replaceStrings` over `Seq<u8>`. Length invariants, substring
  bounds, replacement correctness.

**Tier 2 — Attribute set operations**

- `nix_attrset` — sorted key-value map operations: merge (`//`),
  intersect, remove, names, values, `listToAttrs`. Algebraic proofs
  for merge associativity, intersection commutativity, remove complement.
- `nix_attrpath` — dotted attribute path traversal and existence checking.
  Path decomposition, nested lookup correctness.

**Tier 3 — List and closure operations**

- `nix_list` — `map`, `filter`, `foldl'`, `concatLists`, `genList`,
  `elem`, `elemAt`, `head`, `tail`, `length`. Functor law, fold
  homomorphism, concat associativity.
- `nix_sort` — `builtins.sort` with explicit comparator. Stability
  is not guaranteed by Nix (and we prove it is NOT stable), but
  output is sorted and a permutation of input.
- `nix_partition` — `builtins.partition` splitting a list by predicate
  into `{ right, wrong }`. Completeness: every element lands in exactly
  one output list. Length conservation.
- `nix_closure` — `builtins.genericClosure` fixed-point computation.
  Monotonic key-set growth, termination when key space is finite,
  all reachable keys included.

**Cross-module properties**

- `nix_version` ordering is consistent with `ordering.sign` semantics
- `nix_attrset.merge` over singleton sets reduces to `collections.sorted_insert`
- `nix_list.foldl'` with `nix_attrset.merge` computes the same result as
  chained `//` application — the foundation of `lib.mkMerge`
- `nix_partition` and `nix_list.filter` are duals: partition produces
  both filter results in one pass

## Capabilities

### New Capabilities

- `nix-version-cmp`: Machine-checked Nix version comparison with total ordering proofs
- `nix-string-ops`: Verified substring and replacement over byte sequences
- `nix-attrset-algebra`: Merge/intersect/remove with associativity, commutativity, complement proofs
- `nix-attrpath-lookup`: Verified dotted attribute path traversal
- `nix-list-ops`: Verified map, filter, fold, concat, genList with algebraic laws
- `nix-sort`: Verified sort producing sorted permutation of input
- `nix-partition`: Verified predicate-based list splitting with completeness proof
- `nix-generic-closure`: Verified fixed-point closure with monotonicity and termination

### Modified Capabilities

None. No existing modules are changed.

## Impact

- **Files**: Eight new `src/nix_*.rs` modules, additions to `src/lib.rs`,
  new requirements in `docs/requirements.md`, new tracey annotations
- **APIs**: Eight new public modules, all functions with verified contracts
- **Dependencies**: No new crate dependencies. Uses existing `vstd` Seq/Map types.
- **Testing**: Each module gets proptest oracles. Cross-module properties
  get dedicated property tests. `verus` and `tracey query uncovered` gates
  must pass after each phase.
