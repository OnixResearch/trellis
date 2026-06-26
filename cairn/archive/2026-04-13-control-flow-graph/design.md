## Context

The CFG is the input to every compiler analysis. This module verifies CFG
construction from a linear instruction sequence and the structural
properties that downstream analyses depend on. The repo has graph
primitives but no basic block or CFG model.

## Goals / Non-Goals

**Goals:**
- Basic block identification from leader instructions
- Edge construction from branches and fall-through
- Instruction-to-block partition
- Entry and exit block properties
- Well-formedness invariants
- Dead block identification via reachability

**Non-Goals:**
- Exception handling edges (EH is complex, separate module)
- Indirect branch targets (jump tables)
- Interprocedural CFG (call graph)
- CFG modification (edge splitting, block merging — separate module)

## Decisions

### 1. Instruction Model

**Choice:** Instructions are indexed by position (u32). Each instruction
has a `kind`: `Normal`, `Branch(target)`, `ConditionalBranch(target)`,
or `Return`. The instruction list is `Seq<InstrKind>`. Branch targets
must be in range and may not target instruction 0.

**Rationale:** Minimal model that captures control flow. Normal
instructions fall through. Branches transfer control. Conditional
branches have fall-through and a target. Return ends the function. The
`target != 0` restriction preserves the simple "entry has no internal
predecessors" model used by downstream analyses.

### 2. Leader Identification

**Choice:** An instruction is a leader if: (1) it's the first instruction,
(2) it's the target of a branch, or (3) it immediately follows a
terminator (`Branch`, `ConditionalBranch`, or `Return`).

**Rationale:** Standard basic block construction rule with `Return`
counted as a terminator so later instructions never stay inside a block
that already exited. Leaders partition the instruction sequence into
maximal straight-line blocks.

### 3. Block Representation

**Choice:** `BasicBlock { start: u32, end: u32 }` where `start` is the
leader index and `end` is one past the last instruction (half-open).
The block list is sorted by start index.

**Rationale:** Half-open intervals compose cleanly. The partition proof
is: blocks cover [0, n) with no gaps and no overlaps.

### 4. Edge Construction

**Choice:** An edge from block A to block B exists iff:
- A's last instruction is `Branch(target)` and target is B's leader, or
- A's last instruction is `ConditionalBranch(target)` and target is B's
  leader (taken edge), or
- A's last instruction is `ConditionalBranch(target)` or `Normal` and
  B's leader is A.end (fall-through edge).

**Rationale:** Direct from instruction semantics. The proof checks that
every edge corresponds to a valid control transfer and that no valid
transfer is missing.

### 5. Entry and Exit Blocks

**Choice:** Block 0 is the entry. Exit blocks are blocks whose last
instruction is `Return`. Entry has no predecessors from within the
function because valid programs forbid branches to instruction 0. Exit
blocks have no successors.

**Rationale:** Convention that the first instruction is the entry.
Multiple exit blocks are allowed (multiple return points). The validity
restriction keeps the entry model simple and explicit.

### 6. Well-Formedness

**Choice:** `well_formed(cfg) -> bool` checks:
- All edge targets are valid block indices
- Every block has at least one instruction
- Block ranges are contiguous and non-overlapping
- All branch targets are in range and avoid instruction 0
- All edge targets are valid block indices

**Rationale:** Downstream analyses assume these properties. Checking them
at construction time means they don't need to be reproved by every
consumer.

## Risks / Trade-offs

**[No indirect branches]** → Jump tables require a set of targets per
indirect branch. The model uses direct branches only. Indirect branches
can be added as `Branch(Set<target>)` in a future extension.

**[No exception edges]** → Languages with exceptions (Java, C++) have
implicit edges from any throwing instruction to exception handlers.
These create many additional edges. Out of scope for the base CFG model.
