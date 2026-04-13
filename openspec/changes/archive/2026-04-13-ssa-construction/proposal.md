## Why

Static Single Assignment (SSA) form is the universal intermediate
representation for modern compilers. In SSA, every variable is assigned
exactly once, and merge points in the control flow use phi functions to
select among incoming values. GCC, LLVM, GraalVM, and every serious
optimizing compiler convert to SSA early and stay in SSA through most
optimization passes.

The correctness of SSA construction is subtle: phi functions must be placed
at exactly the iterated dominance frontiers of each variable's definitions,
and every use must be dominated by its definition. A missed phi means a use
sees a stale value. An extra phi is benign but wastes registers. A phi in
the wrong place can make the program miscompile.

## What Changes

- **ssa_construction**: A new module modeling SSA form, phi placement via
  iterated dominance frontiers, and SSA destruction (phi elimination).

  Proves:
  - **Single assignment**: Every variable has exactly one definition point
    in the SSA form.
  - **Dominance of uses**: Every use of a variable is dominated by its
    (unique) definition.
  - **Phi placement correctness**: Phi functions appear exactly at iterated
    dominance frontiers of each variable's definition set.
  - **Phi semantics**: A phi function at block B selects the value from
    the predecessor that was actually taken at runtime.
  - **SSA destruction preserves semantics**: Replacing phi functions with
    copies along predecessor edges produces a program with identical
    semantics to the original (pre-SSA) program.
  - **Iterated dominance frontier convergence**: The IDF computation
    terminates (the frontier set grows monotonically in a finite domain).

## Capabilities

### New Capabilities

- `ssa.single-assignment`: Verified unique definition per variable
- `ssa.dominance-of-uses`: Verified every use dominated by its def
- `ssa.phi-placement`: Verified phi at iterated dominance frontiers
- `ssa.phi-semantics`: Verified phi selects correct predecessor value
- `ssa.destruction`: Verified phi elimination preserves semantics
- `ssa.idf-convergence`: Verified IDF computation terminates

## Impact

- **Files**: One new `src/ssa_construction.rs`, module declaration in `lib.rs`
- **APIs**: `SsaDef`, `PhiNode`, `place_phis`, `check_ssa_property`,
  `eliminate_phis`, `iterated_dominance_frontier`
- **Dependencies**: Depends on `dominator_tree` for dominance and
  dominance frontiers, `reachability.rs` for CFG paths, `vstd`
- **Testing**: Verus proofs for all properties, proptest for SSA validity
  on random CFGs with random variable defs
