## Phase 1: Priority and Merge Primitives

- [x] Add `nickel_priority` module: MergePriority enum, compare spec/exec fn, total order proofs (reflexive, antisymmetric, transitive), extrema proofs (Bottom ≤ all, Top ≥ all), Neutral == Numeral(0) proof
- [x] Add `nickel_merge_split` module: three-way split spec/exec fn over sorted key-value sequences, disjointness proof, completeness proof, value preservation proof
- [x] Add `nickel_field_merge` module: priority-based field resolution spec/exec fn, higher-wins proof, equal-recurse semantics, default-override proof

## Phase 2: Record Merge

- [x] Add `nickel_record_merge` module: full merge spec/exec fn as a direct two-pointer merge over sorted records with equal-key field resolution, identity proof, key coverage proof, disjoint commutativity proof
- [x] Prove merge associativity for uniform-priority (all Neutral) records
- [x] Add cross-module proofs in `properties.rs`: nickel_priority ↔ nickel_field_merge ordering consistency, nickel_merge_split ↔ nix_attrset merge equivalence for disjoint case

## Phase 3: Type Subtyping

- [x] Add `nickel_subtype` module: type enum, subtype relation spec fn, reflexivity proof, transitivity proof
- [x] Implement record-dictionary subsumption rule with proof
- [x] Implement array and dictionary covariance rules with proofs
- [x] Implement exact closed-record subtyping with proof

## Phase 4: Contract Polarity and Pattern Coverage

- [x] Add `nickel_polarity` module: Polarity enum, flip spec/exec fn, double-flip identity proof, path-based polarity computation with domain-flip counting proof
- [x] Add `nickel_pattern_coverage` module: pattern descriptor type, coverage checking spec/exec fn, wildcard completeness proof, enum coverage proof, overlap detection

## Phase 5: Integration

- [x] Add Tracey annotations (r[impl], r[verify], r[depends]) to all new modules, run check-coverage.sh clean
- [x] Add all modules to src/lib.rs, run verus verification, run full test suite
- [x] Update README.md module table with new Nickel section
