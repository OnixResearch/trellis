## Why

Trellis has 10,611 Verus proofs and hundreds of proptests, but they are buried in `src/` files and `#[cfg(test)]` blocks. The Shoals `properties/` surface shows that making properties a first-class discoverable directory — indexed by module group, with source locations and proof-kind annotations — lets consumers find and run properties without reading the entire source tree. This change creates that discoverable surface for trellis, making it the dispatch target for cairn's EARS property vocabulary gate (Phase 3).

## What Changes

- Add `docs/property-spec-index.md`: the indexed catalog of property groups with source modules, property kinds (`proof fn`, `proptest`, `#[test]`), and approximate counts. Covers arithmetic, Raft, concurrency, finance, memory, storage, cross-module, and R1CS groups.
- Reference `docs/proof-gap-sites.tsv` and `docs/module-scope.md` for trusted-boundary and scope details.

## Impact

- **Files**: `docs/property-spec-index.md` (new).
- **Testing**: documentation-only change; no code or tests modified.
- **Non-claims**: the index is a discoverability surface, not a completeness claim. Properties listed are mechanically checked or randomized; they are not exhaustive correctness proofs.
