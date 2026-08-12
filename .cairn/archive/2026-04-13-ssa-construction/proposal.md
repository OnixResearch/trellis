## Why

Static Single Assignment (SSA) form is the universal intermediate
representation for modern compilers. In SSA, every variable is assigned
exactly once, and merge points in the control flow use phi functions to
select among incoming values. GCC, LLVM, GraalVM, and every serious
optimizing compiler convert to SSA early and stay in SSA through most
optimization passes.

The correctness of SSA construction is subtle: phi functions must be placed
at the iterated dominance frontiers of each variable's definitions, every
use must be renamed to a reaching version, and phi elimination must preserve
path semantics. A missed phi means a use sees a stale value. A wrong rename
or phi copy means silent miscompilation.

## Scope Revision Note

The first draft of this change aimed for fully proved SSA construction and
phi destruction. The landed module stops short of that: the heavy SSA
algorithms live in plain Rust helpers, the public Verus wrappers keep only
thin structural contracts, and the semantic obligations are covered with
unit tests and proptests against manual or executable oracles. This archive
reflects the implemented scope rather than the earlier proof-heavy sketch.

The landed module also models SSA destruction with per-edge parallel-copy
records (`PhiEliminatedProgram.edge_copies`) instead of literally inserting
copy instructions into predecessor blocks. That edge-copy model avoids a
critical-edge-splitting precondition in the public API while preserving the
same path-by-path phi semantics in the tests.

## What Changes

- **ssa_construction**: A new module modeling SSA form, phi placement via
  iterated dominance frontiers, SSA renaming, dominance-of-use checking,
  and phi elimination into per-edge parallel-copy records.

  Lands:
  - **Thin verified wrapper guarantees**: block/edge shape preservation,
    bounded IDF round counts, and exact predecessor lookup for phi sources.
  - **Executable SSA checks**: runtime helpers and public checkers for
    explicit-definition uniqueness and dominance of uses.
  - **Executable phi-placement checks**: unit tests and proptests compare
    placed phis against a manual IDF oracle and verify one phi argument per
    distinct predecessor.
  - **Executable SSA-destruction checks**: unit tests and proptests compare
    SSA-path execution with the phi-eliminated edge-copy model.
  - **Tracked trust surface**: `src/ssa_construction.rs` introduces seven
    closeable `assume_specification[...]` bridges, now inventoried in the
    verification-gap docs and README counts.

## Capabilities

### New Capabilities

- `ssa.single-assignment`: SSA construction/checker/test coverage for unique
  explicit SSA definitions
- `ssa.dominance-of-uses`: checker/test coverage that every SSA use reaches a
  dominating definition
- `ssa.phi-placement`: IDF-based phi placement plus oracle-backed tests for
  exact placement and predecessor coverage
- `ssa.phi-semantics`: exact phi-argument lookup helper plus tests for
  predecessor selection
- `ssa.destruction`: edge-copy phi elimination plus path-semantics tests
- `ssa.idf-convergence`: runtime IDF computation with bounded-round wrapper
  contract and oracle-backed tests

## Impact

- **Files**: One new `src/ssa_construction.rs`, module declaration in `lib.rs`
- **APIs**: `def_sites`, `iterated_dominance_frontier`, `place_phis`,
  `construct_ssa`, `check_ssa_property`, `check_dominance_of_uses`,
  `phi_source_for_pred`, `eliminate_phis`
- **Dependencies**: Depends on `dominator_tree` for dominators/frontiers,
  plus `vstd` for the thin verified surface
- **Testing**: unit tests and proptests against manual IDF / SSA / dominance /
  phi-elimination oracles, plus whole-tree validation transcripts archived in
  `openspec/changes/archive/2026-04-13-ssa-construction/evidence/`
