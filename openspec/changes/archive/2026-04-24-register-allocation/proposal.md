## Why

The original register-allocation draft bundled four separate compiler problems:
interference construction, K-coloring, spill code insertion, and move
coalescing. That scope is too broad for one verified landing. The repo already
has the right building blocks for a smaller, useful core: liveness-style live
ranges, graph checkers, and verified matrix/sequence reasoning.

This change narrows to the part that downstream passes need first: build the
interference graph from caller-supplied live ranges, assign each virtual
register either a physical register or a spill slot, and prove the resulting
assignment is conflict-free and total. It explicitly does **not** model spill
instruction insertion or coalescing rewrites.

## What Changes

- **`register_allocation`** — New verified core module for:
  - exact interference-graph construction from live-range overlap
  - allocation validity checking for register-or-spill assignments
  - deterministic first-fit allocation with spill fallback when no register is free
  - total-assignment guarantees: every variable gets a register or spill slot

## Capabilities

### New Capabilities

- `regalloc.interference`: Verified interference edges exactly match live-range overlap
- `regalloc.coloring`: Verified interfering variables never share a physical register
- `regalloc.completeness`: Verified every variable receives either a register or a spill slot
- `regalloc.spill-fallback`: Verified first-fit allocation spills exactly when no register is available in the current prefix

## Impact

- **Files**: New `src/register_allocation.rs`, `src/lib.rs` export,
  `docs/requirements.md` updates, canonical `openspec/specs/register-allocation/spec.md`, and archive evidence
- **APIs**: `InterferenceGraph`, `Allocation`, `build_interference`,
  `check_coloring`, `allocate_greedy`
- **Dependencies**: Reuses sequence/matrix proof style already used by
  `graph_color.rs` and `liveness_analysis.rs`
- **Testing**: Verus proofs and contract-carrying public functions (no trusted shortcuts in the landed module) plus unit/property tests for overlap graphs,
  valid/invalid assignments, and greedy spill fallback

## Out of Scope

- Spill/reload instruction insertion into programs
- Semantic preservation proofs for rewritten programs
- Move coalescing and Briggs-style criteria
- Calling-convention constraints or multi-class register files
- Optimality or minimum-spill guarantees