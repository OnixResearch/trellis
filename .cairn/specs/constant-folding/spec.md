# Constant Folding / Propagation Specification

## Purpose

Defines requirements for verified constant propagation using a lattice-based
forward dataflow analysis and compile-time expression folding. Covers
lattice properties, convergence, soundness, and per-operation fold
correctness.

## Requirements

### Requirement: Lattice Properties

The constant propagation lattice MUST satisfy bounded lattice axioms.

#### Scenario: Commutativity

- GIVEN lattice values A and B
- WHEN `meet(A, B)` and `meet(B, A)` are computed
- THEN the results are equal

#### Scenario: Associativity

- GIVEN lattice values A, B, C
- WHEN `meet(meet(A, B), C)` and `meet(A, meet(B, C))` are computed
- THEN the results are equal

#### Scenario: Idempotence

- GIVEN lattice value A
- WHEN `meet(A, A)` is computed
- THEN the result is A

#### Scenario: Top identity

- GIVEN any lattice value A
- WHEN `meet(Top, A)` is computed
- THEN the result is A

### Requirement: Finite Height

The lattice MUST have bounded height so iteration terminates.

#### Scenario: Height bound

- GIVEN any descending chain Top ≥ v1 ≥ v2 ≥ ...
- WHEN the chain length is measured
- THEN it is at most 2 (Top → Const → Bottom)

### Requirement: Transfer Function Monotonicity

The transfer function MUST be monotone with respect to the lattice ordering.

#### Scenario: Monotone

- GIVEN input environments env1 ≤ env2 (pointwise lattice ordering)
- WHEN `transfer(instr, env1)` and `transfer(instr, env2)` are compared
- THEN `transfer(instr, env1) ≤ transfer(instr, env2)`

### Requirement: Fixed-Point Convergence

The iterative analysis MUST reach a fixed point.

#### Scenario: Termination

- GIVEN a CFG with V variables and B blocks
- WHEN the analysis iterates
- THEN it terminates within 2 × V × B + 1 iterations

#### Scenario: Stability

- GIVEN the fixed-point environments
- WHEN one more iteration is applied
- THEN no environment changes

### Requirement: Soundness

If the analysis classifies a variable as a constant, that classification
MUST be correct on all execution paths covered by the current fixed-point
soundness model.

#### Scenario: Constant is correct on valid block paths

- GIVEN a valid block path that starts at block `0` and does not later revisit block `0`
- AND a caller-supplied top-free entry environment where each reachable entry slot is either `Const(c)` or `Bottom`
- AND the fixed-point analysis classifies variable V as `Const(c)` at a block-entry or block-exit point on that path
- WHEN the corresponding concrete execution follows that path
- THEN V holds value `c`

#### Scenario: Top is reserved for unreached rows

- GIVEN the current fixed-point soundness theorem
- WHEN soundness is checked for reachable executions
- THEN caller-supplied reachable entry state does not use `Top`, and any remaining `Top` values are reserved for analysis-internal unreached rows

#### Scenario: Bottom is conservative

- GIVEN variable V classified as Bottom at point P
- WHEN soundness is checked
- THEN V may hold any value (conservative, always sound)

### Requirement: Fold Correctness

Folding a constant expression at compile time MUST produce the same value
as runtime evaluation.

#### Scenario: Addition

- GIVEN `fold_expr(Add, a, b)` where a and b are constants
- WHEN the result is compared to `a + b` (with wrapping semantics)
- THEN the results are equal

#### Scenario: Division by zero

- GIVEN `fold_expr(Div, a, 0)`
- WHEN folding is attempted
- THEN the result is Bottom (not foldable)

#### Scenario: Comparison

- GIVEN `fold_expr(Eq, a, b)` where a and b are constants
- WHEN the result is compared to `(a == b) as u64`
- THEN the results are equal
