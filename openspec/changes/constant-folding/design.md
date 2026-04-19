## Context

Constant folding is the simplest lattice-based dataflow analysis. Each
variable maps to a value in `{⊤, constants, ⊥}`. The analysis propagates
constants forward through the CFG. When all operands of an expression are
constants, the expression is folded. This slice proves the lattice laws by
explicit case analysis and proves wrapping arithmetic folds with local
bit-vector lemmas plus `fetch_add::u64_wrapping_add` reuse.

## Goals / Non-Goals

**Goals:**
- Three-valued lattice: Top (unknown), Const(value), Bottom (overdefined)
- Meet operation with all lattice properties
- Finite height and convergence bound
- Transfer function per instruction
- Forward dataflow iteration to fixed point
- Soundness: analysis result matches all concrete executions
- Per-operation fold correctness

**Non-Goals:**
- Sparse conditional constant propagation (SCCP) — branches on constants
- Interprocedural constant propagation
- Floating-point folding
- Constant expression tree simplification (algebraic identities)

## Decisions

### 1. Lattice Element

**Choice:** Enum `ConstLat { Top, Const(u64), Bottom }` with ordering
`Top > Const(c) > Bottom` for all c. All `Const` values are incomparable
with each other.

**Rationale:** Height 2 (Top → Const → Bottom). Any chain has at most
2 steps downward. Finite height guarantees termination.

### 2. Meet Operation

**Choice:**
```
meet(Top, x) = x
meet(x, Top) = x
meet(Const(a), Const(b)) = if a == b { Const(a) } else { Bottom }
meet(Bottom, _) = Bottom
meet(_, Bottom) = Bottom
```

**Rationale:** Standard lattice meet. Proves: commutative, associative,
idempotent (lattice axioms) by explicit case analysis over `Top`, `Const`,
and `Bottom`.

### 3. Transfer Function

**Choice:** Per instruction `transfer(instr, env) -> env'` where
`env: Seq<ConstLat>` maps variable indices to lattice values. For
`v = op(a, b)`: if `env[a]` and `env[b]` are both `Const`, compute
the result and set `env'[v] = Const(result)`. If either operand is
`Bottom`, set `env'[v] = Bottom`. Otherwise propagate `Top`.
Malformed or unsupported instructions conservatively set the defined
variable to `Bottom`.

**Rationale:** This keeps the transfer monotone on the standard
`Bottom <= Const(c) <= Top` lattice while staying sound: two constants fold,
a conflicting operand stays conflicting, and a not-yet-known operand does not
prematurely collapse to `Bottom`.

### 4. Fold Operation

**Choice:** `fold_expr(op, a: u64, b: u64) -> u64` evaluates the operation
at compile time. Supported ops: add, sub, mul, div (checked), and, or,
xor, shl, shr, eq, ne, lt, le.

**Rationale:** Each fold must match the runtime evaluation exactly.
Wrapping add reuses `fetch_add::u64_wrapping_add`; wrapping sub/mul use local
specs plus `by(bit_vector)` lemmas; the remaining ops are proved by the exec
wrapper's exact `ensures`. Division by zero returns Bottom (not foldable).

### 5. Forward Iteration

**Choice:** Process blocks in reverse postorder. Initialize entry block's
env from a caller-supplied entry environment. At each block, apply transfer
functions sequentially. At merge points, meet incoming environments. The
current engine uses synchronous rounds over predecessor out-environments and a
conservative `2 × V × B + 1` budget while checking successive rounds for
stability.

**Rationale:** Forward analysis plus RPO improves practical convergence. With
height-2 lattice slots, each block-variable entry can strictly descend at most
`Top -> Const -> Bottom`; the extra `+ 1` round is for the final no-change
stability check. Later proof work can tighten the exact bound if desired.

### 6. Soundness Statement

**Choice:** Prove soundness on valid block paths that start at block `0`
and do not later revisit block `0`, under a top-free caller-supplied
entry environment: if a block-entry or block-exit environment classifies
`env[v] == Const(c)` on such a path, then the corresponding concrete
execution on that path holds value `c`.

**Rationale:** This is what prevents miscompilation while matching the
current executable model. The proof is by path induction: the entry env is
assumed sound and top-free (`Const(c)` for exact known values, `Bottom`
for unknown/overdefined reachable values), transfer preserves both
soundness and top-freeness, and predecessor meet becomes sound once one
actual predecessor on the path supplies a sound top-free environment.
Remaining `Top` values are treated as internal unreached-row sentinels,
not as caller-supplied reachable inputs.

## Risks / Trade-offs

**[No conditional branches]** → SCCP uses branch conditions to narrow
analysis at join points. This module treats all paths as possible,
which is conservative but sound. SCCP could be a separate module.

**[u64 only]** → Modeling with u64 limits the constant domain. Booleans
are 0/1. Pointers and floats are not modeled.
