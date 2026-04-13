## Context

SSA construction takes a CFG with arbitrary variable assignments and
produces a form where every variable has exactly one definition. Merge
points get phi functions. The standard algorithm (Cytron et al.) places
phis at iterated dominance frontiers of each variable's definition set,
then renames variables with a dominator tree walk.

## Goals / Non-Goals

**Goals:**
- Phi placement at iterated dominance frontiers
- Single-assignment property after construction
- Dominance-of-uses property
- Phi semantics (selects correct predecessor value)
- SSA destruction (phi elimination via copies)
- IDF convergence

**Non-Goals:**
- Pruned SSA (removing dead phis — that's DCE)
- Minimal SSA vs. semi-pruned SSA tradeoffs
- Gated SSA or other SSA variants
- SSA-based optimization passes

## Decisions

### 1. Pre-SSA IR Model

**Choice:** Instructions are `(var_def: Option<u32>, var_uses: Seq<u32>, opcode: u32)`. Blocks contain a sequence of instructions. The CFG is
`Seq<Block>` with block 0 as entry.

**Rationale:** Minimal model: each instruction optionally defines one
variable and uses zero or more. The opcode is opaque — SSA construction
doesn't care about the operation, only about defs and uses.

### 2. Phi Representation

**Choice:** `PhiNode { var: u32, args: Seq<(u32, u32)> }` where each arg
is `(predecessor_block, variable_version)`. A block has a sequence of
phi nodes before its regular instructions.

**Rationale:** The predecessor association is essential for phi semantics:
at runtime, the phi selects the argument from the actually-taken
predecessor.

### 3. Iterated Dominance Frontier

**Choice:** Compute `IDF(defs)` as the fixed point of `DF(defs ∪ IDF_prev)`.
Start with `IDF₀ = ∅`, compute `DF(defs)`, add new nodes, repeat.

**Rationale:** Standard Cytron algorithm. Convergence follows from: the
set of nodes is finite, each iteration adds at least one node or stops,
and the set only grows (monotone in the powerset lattice).

### 4. Renaming

**Choice:** Dominator tree walk with a per-variable version stack. At each
block: assign fresh versions to phi defs, walk instructions (rename uses
from current version, push new version on def), then recurse to dominated
children, then pop versions.

**Rationale:** The dominator tree walk ensures that at each use, the
correct reaching definition is on top of the stack (because the def
dominates the use). The push/pop stack discipline matches the dominator
tree's DFS structure.

### 5. SSA Destruction

**Choice:** Replace each phi with copies along predecessor edges. For
`phi(v, [(p1, v1), (p2, v2)])`, insert `v = v1` at the end of p1 and
`v = v2` at the end of p2 (before the branch).

**Rationale:** Standard phi elimination. The key proof is: the copy
at the end of predecessor P assigns the same value that the phi would
have selected. This is immediate from phi semantics.

### 6. Single-Assignment Property

**Choice:** `is_ssa(program) -> bool` checks: for every variable version,
exactly one instruction (or phi) defines it.

**Rationale:** This is the SSA invariant. The proof shows construction
produces a program satisfying this predicate, and destruction produces a
correct non-SSA program.

## Risks / Trade-offs

**[Critical edges]** → Phi elimination inserts copies on predecessor
edges. If an edge is critical (predecessor has multiple successors), the
copy must go on a new block splitting the edge. The model assumes critical
edges are split before destruction.

**[Phi ordering]** → Multiple phis in the same block read values
simultaneously (not sequentially). The destruction must insert copies in
parallel-copy semantics. The model handles this with a simultaneous
assignment step.
