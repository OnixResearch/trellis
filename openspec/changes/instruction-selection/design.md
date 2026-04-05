## Context

Instruction selection maps the IR to machine instructions. The IR is
modeled as an expression tree; machine instructions are tiles covering
subtrees. Dynamic programming on the tree finds the optimal tiling.

## Goals / Non-Goals

**Goals:**
- Expression tree model (nodes are IR operations)
- Tile model (pattern + cost + semantics)
- Pattern matching: tile pattern matches subtree shape
- DP algorithm: compute optimal tiling bottom-up
- Completeness: every node covered
- Soundness: tile semantics matches subtree semantics
- Optimality: minimum total cost

**Non-Goals:**
- DAG-based instruction selection (tree covering only)
- BURG/BURS table generation
- Target-specific instruction encodings
- Instruction scheduling (post-selection pass)

## Decisions

### 1. Expression Tree

**Choice:** Recursive type:
```
ExprNode { op: u32, children: Seq<ExprNode>, value: Option<u64> }
```
Leaf nodes have empty children and may carry a constant value. Internal
nodes have an opcode and one or more children.

**Rationale:** Generic tree. The opcode is abstract — the module doesn't
define a fixed instruction set. Tiles match based on opcode and tree shape.

### 2. Tile Definition

**Choice:**
```
Tile { pattern: TilePattern, cost: u32, semantics: spec fn(Seq<u64>) -> u64 }
TilePattern { op: u32, child_patterns: Seq<TileChildPattern> }
TileChildPattern::Any | TileChildPattern::Specific(TilePattern)
```

`Any` matches any subtree (that subtree becomes an "input" to the tile's
generated instruction, tiled separately). `Specific` matches a particular
opcode and structure.

**Rationale:** This captures tiles of different sizes. A simple tile
matches one node (`op: Add, children: [Any, Any]`). A complex tile
matches a subtree (`op: Add, children: [Specific(op: Mul, ...), Any]`
for a fused multiply-add).

### 3. Pattern Matching

**Choice:** `tile_matches(tile, node) -> Option<Seq<ExprNode>>` returns
the uncovered children (leaves of the tile pattern that matched `Any`)
on success, or None on failure. Recursive structural comparison.

**Rationale:** The uncovered children are what the DP needs to tile
recursively. The proof shows: the union of the tile's coverage and the
uncovered children's coverage equals the entire subtree.

### 4. DP Algorithm

**Choice:** Bottom-up traversal. At each node:
1. Find all tiles whose pattern matches this node
2. For each matching tile: total_cost = tile.cost + sum of optimal costs
   at uncovered children
3. Select the tile with minimum total_cost
4. Record the selection

**Rationale:** Standard bottom-up tree DP. Optimal substructure: the
optimal tiling of a subtree only depends on the optimal tilings of its
sub-subtrees. The proof shows: the minimum-cost selection at each node
produces a globally optimal tiling.

### 5. Completeness Proof

**Choice:** Prove: after the DP, every node has at least one matching
tile. Requires a "base tile" assumption: for every opcode, there exists
a tile matching a single node of that opcode with all children as Any.

**Rationale:** Without base tiles, some nodes might have no matching
pattern and the algorithm fails. The assumption is that the tile set is
complete — every IR operation has at least a basic one-node tile.

### 6. Soundness Proof

**Choice:** Prove: for each tile in the selected tiling,
`tile.semantics(inputs) == subtree_eval(node)` where `subtree_eval`
recursively evaluates the expression tree.

**Rationale:** The tile's semantics is a spec function. The subtree
evaluation is the reference semantics. Equality means the tiled program
computes the same values.

## Risks / Trade-offs

**[Tree only, not DAG]** → Expression DAGs (common subexpressions shared)
require extension. Tree covering duplicates shared subtrees. Real
compilers handle this with CSE before instruction selection.

**[Cost model is static]** → Real instruction costs depend on pipeline
state. The DP uses fixed costs per tile. This is standard for instruction
selection; scheduling handles pipeline effects.

**[Base tile assumption]** → If the tile set is incomplete (no tile for
some opcode), the algorithm fails. The proof is conditional on tile set
completeness.
