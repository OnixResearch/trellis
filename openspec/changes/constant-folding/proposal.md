## Why

Constant folding evaluates expressions with known-constant operands at
compile time, replacing them with the result. Constant propagation extends
this by tracking which variables hold constants through the CFG using a
lattice-based dataflow analysis. Together, they're the simplest and most
widely applied optimization — present in every compiler from tcc to LLVM.

The analysis uses a three-valued lattice per variable: ⊤ (unknown, not yet
analyzed), a constant value, or ⊥ (overdefined, multiple values reach this
point). The meet rule is: ⊤ ∧ c = c, c ∧ c = c, c₁ ∧ c₂ = ⊥ (c₁ ≠ c₂),
⊥ ∧ anything = ⊥. The analysis converges because the lattice has height 2
and the transfer function is monotone.

The verification value is in proving soundness: the compiler must never
replace a non-constant expression with a constant. An unsound constant
propagation miscompiles programs silently.

## What Changes

- **constant_folding**: A new module modeling the constant propagation
  lattice, transfer function, and fold operation.

  Proves:
  - **Lattice properties**: The three-valued lattice is a bounded lattice
    with ⊤ as top, ⊥ as bottom, and meet is commutative, associative,
    and idempotent.
  - **Finite height**: The lattice has height 2, so any chain of meets
    terminates in at most 2 steps per variable.
  - **Transfer function monotonicity**: The transfer function is monotone
    with respect to the lattice ordering. If input A ≤ input B, then
    transfer(A) ≤ transfer(B).
  - **Fixed-point convergence**: The iterative analysis terminates within
    O(V × B) steps (V variables, B blocks).
  - **Soundness**: If the analysis says variable V is constant C at
    program point P, then on every execution path reaching P, V holds C.
  - **Fold correctness**: Folding an expression with constant operands
    produces the same value as evaluating the expression at runtime.
    Proved per operation (add, sub, mul, div, comparisons, bitwise ops).

## Capabilities

### New Capabilities

- `constfold.lattice`: Verified three-valued lattice with meet properties
- `constfold.finite-height`: Verified lattice height bound
- `constfold.monotone`: Verified transfer function monotonicity
- `constfold.convergence`: Verified fixed-point termination
- `constfold.soundness`: Verified constant classification correctness
- `constfold.fold`: Verified expression folding matches runtime evaluation

## Impact

- **Files**: One new `src/constant_folding.rs`, module declaration in `lib.rs`
- **APIs**: `ConstLattice` enum (Top/Const/Bottom), `meet`, `transfer`,
  `fold_expr`, `propagate`, `is_sound`
- **Dependencies**: Uses `algebraic.rs` for lattice property predicates,
  `arithmetic.rs` for fold correctness, depends on `vstd`
- **Testing**: Verus proofs for all properties, proptest for fold
  correctness against runtime evaluation and soundness on random CFGs
