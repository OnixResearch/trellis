## Context

SSA construction takes a CFG with arbitrary variable assignments and
produces a form where every variable is assigned once. Merge points get
phi functions. The classic Cytron algorithm places phis at iterated
dominance frontiers of each variable's definition set, then renames
variables with a dominator-tree walk.

The landed module keeps that runtime shape, but not a fully internalized
proof story. The heavy SSA algorithms are plain Rust helpers. Thin public
Verus wrappers expose structural contracts and one exact lookup helper,
while tests and proptests check the semantic obligations against manual or
executable oracles.

## Goals / Non-Goals

**Goals:**
- Runtime phi placement at iterated dominance frontiers
- Runtime SSA renaming with fresh versions
- Runtime checkers for explicit-definition uniqueness and dominance of uses
- Exact phi-source lookup by predecessor
- Phi elimination into a per-edge parallel-copy model
- Unit tests and proptests covering IDF, phi placement, SSA uniqueness,
  dominance of uses, and phi-elimination path semantics
- Accurate documentation of the trust surface and evidence

**Non-Goals:**
- Fully internalized Verus proofs for end-to-end SSA correctness
- Pruned SSA (removing dead phis)
- Minimal SSA vs. semi-pruned SSA tradeoffs
- Gated SSA or other SSA variants
- SSA-based optimization passes

## Decisions

### 1. Pre-SSA IR Model

**Choice:** Instructions are `(def: Option<u32>, uses: Vec<u32>, opcode: u8)`.
Blocks contain a sequence of instructions. The CFG is `Vec<RawBlock>` plus
an explicit `edges: Vec<(usize, usize)>`, with block `0` as entry.

**Rationale:** Minimal model: each instruction optionally defines one base
variable and uses zero or more. SSA construction only cares about defs,
uses, and control-flow shape.

### 2. Phi Representation

**Choice:** `PhiNode { base, dest, args }` where each arg is
`(predecessor_block, version)`. A block stores phis separately from regular
instructions.

**Rationale:** The predecessor association is essential for phi semantics.
`dest` stores the fresh SSA version defined by the phi; `args` says which
incoming edge supplies which reaching version.

### 3. Iterated Dominance Frontier

**Choice:** Compute `IDF(defs)` as the fixed point of
`DF(defs ∪ current)`. Start from the empty set, grow monotonically, and stop
when a round adds nothing.

**Rationale:** Standard Cytron algorithm. The verified wrapper only promises
shape plus a round bound `<= blocks.len()`. The semantic behavior is checked
against a manual IDF oracle in unit tests and proptests.

### 4. Renaming

**Choice:** Use a runtime dominator-tree walk with a per-variable version
stack. Phi definitions get fresh versions first, instruction uses read the
current stack top, new instruction defs push fresh versions, dominated
children recurse, then the block pops what it defined.

**Rationale:** This is the standard SSA renaming algorithm. Verus currently
handles the supporting shape contracts more robustly than the full mutable
stack/tree proof, so semantic obligations stay in executable tests and
checkers for now.

### 5. Public Verification Boundary

**Choice:** Keep the heavy algorithms in plain Rust helpers and bridge them
into the public Verus wrappers with `assume_specification[...]` declarations
behind `#[cfg(verus_keep_ghost)]`.

**Rationale:** The module needs public APIs callable from the verified crate,
but nested mutable vectors, stack-based renaming, and path-sensitive SSA
reasoning were not closeable in the current pass. The wrappers still prove
useful facts: edge validity preconditions, output shape, bounded IDF rounds,
and exact phi-source lookup. The bridge sites are explicitly tracked as
closeable trust boundaries in the verification-gap docs.

### 6. SSA Destruction Model

**Choice:** `eliminate_phis` removes phis from blocks and records the needed
parallel copies in `PhiEliminatedProgram.edge_copies`, one entry per CFG
edge.

**Rationale:** This models the same semantics as predecessor-edge copy
insertion without rewriting predecessor blocks or requiring explicit
critical-edge splitting in the public API. Tests interpret `edge_copies` as
copies applied on the traversed edge before the successor executes.

### 7. Single-Assignment / Dominance Checking

**Choice:** Keep `check_ssa_property` and `check_dominance_of_uses` as
runtime predicates over the constructed SSA graph, with manual-oracle tests.

**Rationale:** The module still needs executable auditors for the semantic
claims even though the full proofs are deferred. The public wrappers expose
these checkers while the tests confirm they agree with independent manual
implementations on random small graphs.

## Risks / Trade-offs

**[Closeable trust surface]** → Seven `assume_specification[...]` bridges now
exist in `src/ssa_construction.rs`. The archive and verification-gap docs
must keep that trust surface explicit.

**[Edge-copy abstraction]** → `PhiEliminatedProgram` models copies on edges,
not literal predecessor-block instruction insertion. Specs/docs/tests must
say edge copies, not inserted instructions.

**[No full proof yet]** → The semantic properties are supported by runtime
checkers and random/manual-oracle tests, not by end-to-end Verus proofs.
This is acceptable only because the archive reflects that weaker scope
honestly.
