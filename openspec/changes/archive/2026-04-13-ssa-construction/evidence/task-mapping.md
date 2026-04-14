# Draft Checklist -> Landed Scope Mapping

This file makes the SSA archive auditable after the scope correction. The
original draft checklist asked for fully proved SSA correctness. The landed
module keeps the runtime SSA pipeline, thin verified wrapper contracts, and
executable/manual-oracle evidence instead.

## Scope-correction summary

| Draft wording | Landed wording | Why changed |
|---|---|---|
| Prove end-to-end IDF, phi placement, renaming, and destruction correctness | Keep thin verified wrapper contracts and back semantic claims with runtime/manual-oracle tests | Full mutable SSA proofs did not close in this pass; archive now reflects actual trust boundary |
| Phi destruction inserts copy instructions into predecessor blocks | Phi destruction records per-edge parallel copies in `PhiEliminatedProgram.edge_copies` | Landed API models edge copies directly and does not require critical-edge splitting |
| Complete proof-only validation story | Capture validation transcripts plus test/oracle evidence | Review needs the real command outputs and the actual proof/test split |

## Per-draft-item mapping

### Phase 1: IDF Computation

- [draft] Define `def_sites(var, program) -> Set<u32>` collecting blocks that define var
  - landed: `src/ssa_construction.rs` defines `def_sites(var, program) -> Vec<bool>` and uses `block_defines_var_runtime` underneath
  - evidence: implementation in `src/ssa_construction.rs`; validation in `evidence/verus-lib.txt`, `evidence/cargo-test.txt`

- [draft] Implement iterated dominance frontier: `IDF(defs) = fixpoint(DF(defs ∪ IDF_prev))`
  - landed: `iterated_dominance_frontier_with_rounds` + runtime helper implement that fixpoint shape
  - evidence: `test_idf_diamond`, `prop_idf_matches_manual`; transcripts in `evidence/cargo-test.txt`

- [draft] Prove IDF set grows monotonically
  - landed scope: executable/manual-oracle evidence rather than a closed proof
  - evidence: `prop_idf_matches_manual` plus the archived scope note in `proposal.md` / `design.md`

- [draft] Prove IDF converges within N iterations (N = node count)
  - landed: thin wrapper contract keeps `rounds <= blocks.len()`; manual oracle tests cover semantic agreement
  - evidence: `iterated_dominance_frontier_with_rounds` ensures, `prop_idf_matches_manual`, `evidence/verus-lib.txt`, `evidence/cargo-test.txt`

### Phase 2: Phi Placement

- [draft] Implement `place_phis(program, domtree) -> program_with_phis`
  - landed: `place_phis(program, dom)` backed by `runtime_place_phis`
  - evidence: implementation + `test_place_phis_diamond`

- [draft] Prove phi placed at every block in IDF of each variable's def set
  - landed scope: executable oracle check instead of closed proof
  - evidence: `prop_place_phis_hits_manual_idf`

- [draft] Prove no phi placed at blocks outside the IDF
  - landed scope: executable oracle check instead of closed proof
  - evidence: `prop_place_phis_hits_manual_idf`

- [draft] Prove every phi has exactly one argument per predecessor
  - landed scope: executable oracle check against distinct predecessors
  - evidence: `prop_place_phis_hits_manual_idf` checks `phi.args.len() == preds.len()` and exactly one arg per predecessor

### Phase 3: Renaming

- [draft] Implement dominator-tree-walk renaming with version stacks
  - landed: `runtime_construct_ssa` performs stack-based renaming over the dominator tree
  - evidence: implementation + `test_construct_ssa_simple_branch`

- [draft] Prove single-assignment: each version defined exactly once
  - landed scope: runtime checker + manual-oracle tests instead of closed proof
  - evidence: `check_ssa_property`, `manual_check_ssa`, `prop_constructed_program_is_ssa`

- [draft] Prove dominance-of-uses: each use dominated by its def
  - landed scope: runtime checker + manual-oracle tests instead of closed proof
  - evidence: `check_dominance_of_uses_with_dom`, `manual_check_dom_uses`, `prop_constructed_program_has_dominated_uses`

- [draft] Prove phi uses dominated: each phi argument's def dominates end of predecessor
  - landed scope: same runtime/manual-oracle checker, not a separate Verus proof artifact
  - evidence: `test_dominance_of_uses_branch_phi`, `prop_constructed_program_has_dominated_uses`

### Phase 4: SSA Destruction

- [draft] Implement phi elimination via copy insertion on predecessor edges
  - landed: `runtime_eliminate_phis` removes phis from blocks and records copies in `edge_copies`
  - evidence: implementation + live/archive SSA spec wording

- [draft] Prove copy insertion preserves semantics
  - landed scope: executable path-semantics equality between SSA execution and the edge-copy model
  - evidence: `test_phi_elimination_roundtrip`, `prop_eliminate_phis_preserves_path_semantics`

- [draft] Handle critical edges (document precondition: edges must be split)
  - landed revision: public API uses an edge-copy table, so no critical-edge-splitting precondition is exposed
  - evidence: revised `design.md` decision 6 and revised SSA domain spec

- [draft] Prove parallel copy semantics for multiple phis in same block
  - landed scope: path executor applies edge copies as a parallel edge step and tests compare resulting environments
  - evidence: `exec_eliminated_path`, `prop_eliminate_phis_preserves_path_semantics`

### Phase 5: Tests

- [draft] Unit tests for IDF on diamond, if-else, loop CFGs
  - landed: `test_idf_diamond` plus random/manual-oracle coverage on broader CFGs
  - evidence: `evidence/cargo-test.txt`

- [draft] Unit tests for SSA construction on simple programs
  - landed: `test_construct_ssa_simple_branch`, `test_dominance_of_uses_branch_phi`
  - evidence: `evidence/cargo-test.txt`

- [draft] Unit tests for phi elimination roundtrip
  - landed: `test_phi_elimination_roundtrip`
  - evidence: `evidence/cargo-test.txt`

- [draft] Proptest: single-assignment property on random CFGs with random defs
  - landed: `prop_constructed_program_is_ssa`
  - evidence: `evidence/cargo-test.txt`

- [draft] Proptest: dominance-of-uses on random SSA programs
  - landed: `prop_constructed_program_has_dominated_uses`
  - evidence: `evidence/cargo-test.txt`

- [draft] Proptest: SSA destruction produces same observable output
  - landed: `prop_eliminate_phis_preserves_path_semantics`
  - evidence: `evidence/cargo-test.txt`
