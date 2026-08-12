## Why

Dead code elimination (DCE) removes instructions whose results are never
used and that have no side effects. It's the canonical "semantics
preservation" optimization — the proof that removing an instruction
doesn't change the program's observable behavior is the template for
every other optimization's correctness argument.

DCE depends on liveness analysis: an instruction is dead if it defines
a variable that is not live after the instruction and the instruction
has no side effects (no stores, no I/O, no calls with side effects).
The subtlety is in the side-effect classification — a pure arithmetic
instruction is dead if unused, but a store to memory is always live.

## What Changes

- **dead_code_elimination**: A new module modeling DCE on a CFG with
  liveness information.

  Proves:
  - **Semantics preservation**: Removing dead instructions does not change
    the program's observable behavior. Formally: for every observable
    event (store to memory, return value, I/O) in the original program,
    the same event occurs in the optimized program in the same order.
  - **Liveness criterion**: An instruction is dead iff it defines a
    variable that is not live-out of the instruction's program point and
    the instruction has no side effects.
  - **No live instruction removed**: Every instruction that defines a
    live variable or has side effects is preserved.
  - **Idempotence**: Running DCE twice produces the same result as running
    it once. The output of DCE has no dead instructions.

## Capabilities

### New Capabilities

- `dce.semantics`: Verified observable behavior preservation
- `dce.criterion`: Verified dead instruction classification
- `dce.no-live-removed`: Verified live instructions preserved
- `dce.idempotent`: Verified single-pass sufficiency

## Impact

- **Files**: One new `src/dead_code_elimination.rs`, module declaration
  in `lib.rs`
- **APIs**: `is_dead`, `eliminate_dead`, `has_side_effect`
- **Dependencies**: Depends on `liveness_analysis` for liveness information,
  operates on CFG, depends on `vstd`
- **Testing**: Verus proofs for all properties, proptest for DCE
  correctness (output behavior matches input) on random programs
