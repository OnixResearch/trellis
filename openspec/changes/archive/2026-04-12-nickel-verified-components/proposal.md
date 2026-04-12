## Why

Nickel's core semantics — record merge, field priority, type subtyping,
contract polarity, pattern compilation — are specified in prose and tested
at runtime. The existing `nix_attrset`, `nix_list`, `nix_string`,
`nix_version`, and `nix_closure` modules in verified-logic already prove
Nix primitives. Nickel extends that foundation with merge semantics,
a priority lattice, gradual typing, and contract blame tracking. These
are self-contained decision procedures with clear algebraic properties
that Verus can check.

This change adds verified modules modeling Nickel's core logic. The goal
is not to rewrite the Nickel interpreter — it's to provide machine-checked
reference implementations of the semantics that Nickel's runtime depends on,
so bugs in merge conflict resolution, priority ordering, subtype checking,
and blame polarity tracking can be caught at the spec level.

## What Changes

- **nickel_merge_split**: Three-way record split (left-only, both, right-only)
  with partition completeness and disjointness proofs. Models the `split`
  function from `core/src/eval/merge.rs`.

- **nickel_priority**: MergePriority total order (Bottom < Neutral ≡ 0 <
  Numeral(n) < Top) with transitivity, antisymmetry, and totality proofs.
  Models `MergePriority` from `parser/src/ast/mod.rs`.

- **nickel_field_merge**: Field-level merge resolution using priority.
  Higher priority wins; equal priority recurses or requires equality.
  Models the priority-based conflict resolution in `core/src/eval/merge.rs`.

- **nickel_record_merge**: Full record merge via a direct two-pointer merge
  over sorted records, using `nickel_field_merge` to distinguish left-win
  from right-biased equal-key cases. In this simplified model, equal keys
  select the right field for `Right`, `Equal`, and `Conflict` outcomes.
  Proves neutral-record associativity, identity (merge with empty), and
  that right-biased override at equal priority matches Nickel's semantics.

- **nickel_subtype**: Subtyping relation for Nickel's gradual type system.
  Record-dictionary subsumption (`{a: T, b: U} <: {_: V}` when T,U <: V),
  array covariance, reflexivity, transitivity. Models
  `core/src/typecheck/subtyping.rs`.

- **nickel_polarity**: Contract blame polarity tracking. Polarity flips on
  function domain, stays on codomain. Double-flip is identity. Models
  the `Polarity` enum and type path navigation in `core/src/label.rs`.

- **nickel_pattern_coverage**: Pattern match completeness and overlap
  detection. A set of patterns covers a type when every value matches at
  least one pattern. Models the compilation in
  `core/src/term/pattern/compile.rs`.

## Capabilities

### New Capabilities

- `nickel-merge-split`: Three-way key-set partitioning with conservation proof
- `nickel-priority`: Total order on merge priorities with lattice bounds
- `nickel-field-merge`: Priority-based field conflict resolution
- `nickel-record-merge`: Full record merge with algebraic properties
- `nickel-subtype`: Subtype checking with structural rules
- `nickel-polarity`: Blame polarity tracking through higher-order contracts
- `nickel-pattern-coverage`: Pattern completeness and overlap checking

### Modified Capabilities

None.

## Impact

- **Files**: 7 new modules in `src/`, additions to `src/lib.rs`
- **APIs**: New public spec/exec fn pairs per module
- **Dependencies**: No new deps (vstd only)
- **Testing**: Each module gets unit tests + proptest oracles; Verus proofs
  for all ensures clauses; Tracey annotations linking to the new spec
