## Phase 1: Lattice

- [x] Define `ConstLat` enum: Top, Const(u64), Bottom
- [x] Define lattice ordering: Top > Const(c) > Bottom
- [x] Implement `meet(a, b) -> ConstLat`
- [x] Prove commutativity, associativity, idempotence
- [x] Prove Top is identity for meet
- [x] Prove Bottom is absorbing for meet
- [x] Prove finite height: any descending chain has length ≤ 2

## Phase 2: Transfer Function

- [x] Define per-instruction transfer: `transfer(instr, env) -> env'`
- [x] Prove monotonicity: env1 ≤ env2 → transfer(env1) ≤ transfer(env2)
- [x] Handle arithmetic ops: both Const → fold, any Bottom → Bottom
- [x] Handle assignments: update env for defined variable

## Phase 3: Fold Operations

- [x] Implement `fold_expr(op, a, b) -> ConstLat` for add, sub, mul
- [x] Implement fold for div (Bottom on divide by zero)
- [x] Implement fold for bitwise ops: and, or, xor, shl, shr
- [x] Implement fold for comparisons: eq, ne, lt, le
- [x] Prove each fold matches runtime evaluation (using local wrapping specs, bit-vector lemmas, and `fetch_add::u64_wrapping_add` reuse)

## Phase 4: Fixed-Point Analysis

- [x] Implement forward iterative analysis with RPO processing
- [x] Prove convergence within `2 × V × B + 1` rounds
- [x] Prove fixed-point stability
- [x] Prove soundness on valid block paths from entry under the current top-free entry-environment boundary: `Const(c)` at block entry/exit implies concrete value `c` on that path

## Phase 5: Tests

- [x] Unit tests for lattice meet on all combinations
- [x] Unit tests for fold: known arithmetic, edge cases, division by zero
- [x] Unit tests for propagation on straight-line and branching CFGs
- [x] Proptest: meet commutativity and associativity for random lattice values
- [x] Proptest: fold correctness against runtime evaluation for random ops
- [x] Proptest: soundness — constant classification agrees with concrete execution

## Phase 6: Coverage and Validation

- [x] Add `constfold.*` requirement anchors to `docs/requirements.md` and matching `r[impl]`/`r[verify]` tags to the current `src/constant_folding.rs` public surface
- [x] Run `cargo clippy --all-targets -- -D warnings`, `cargo test --quiet`, `./scripts/verus.sh src/lib.rs`, and `./scripts/verify.sh` for the current implemented slice; capture the results in `openspec/changes/constant-folding/evidence/validation.md`
- [x] Re-run the Phase 6 validation commands after Phase 4/5 completion and refresh `openspec/changes/constant-folding/evidence/validation.md`
