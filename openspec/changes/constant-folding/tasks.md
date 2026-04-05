## Phase 1: Lattice

- [ ] Define `ConstLat` enum: Top, Const(u64), Bottom
- [ ] Define lattice ordering: Top > Const(c) > Bottom
- [ ] Implement `meet(a, b) -> ConstLat`
- [ ] Prove commutativity, associativity, idempotence
- [ ] Prove Top is identity for meet
- [ ] Prove Bottom is absorbing for meet
- [ ] Prove finite height: any descending chain has length ≤ 2

## Phase 2: Transfer Function

- [ ] Define per-instruction transfer: `transfer(instr, env) -> env'`
- [ ] Prove monotonicity: env1 ≤ env2 → transfer(env1) ≤ transfer(env2)
- [ ] Handle arithmetic ops: both Const → fold, any Bottom → Bottom
- [ ] Handle assignments: update env for defined variable

## Phase 3: Fold Operations

- [ ] Implement `fold_expr(op, a, b) -> ConstLat` for add, sub, mul
- [ ] Implement fold for div (Bottom on divide by zero)
- [ ] Implement fold for bitwise ops: and, or, xor, shl, shr
- [ ] Implement fold for comparisons: eq, ne, lt, le
- [ ] Prove each fold matches runtime evaluation (using `arithmetic.rs`)

## Phase 4: Fixed-Point Analysis

- [ ] Implement forward iterative analysis with RPO processing
- [ ] Prove convergence within V × B iterations
- [ ] Prove fixed-point stability
- [ ] Prove soundness: Const(c) at point P iff c on all paths to P

## Phase 5: Tests

- [ ] Unit tests for lattice meet on all combinations
- [ ] Unit tests for fold: known arithmetic, edge cases, division by zero
- [ ] Unit tests for propagation on straight-line and branching CFGs
- [ ] Proptest: meet commutativity and associativity for random lattice values
- [ ] Proptest: fold correctness against runtime evaluation for random ops
- [ ] Proptest: soundness — constant classification agrees with concrete execution
