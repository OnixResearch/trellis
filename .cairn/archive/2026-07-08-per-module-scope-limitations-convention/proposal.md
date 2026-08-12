## Why

Trellis has 10,611 Verus proofs across 602 modules, but its scope/limitations
documentation lives only at the trust-boundary level (`docs/proof-gap-sites.tsv`
for the 16 `#[verifier::external_body]` sites). The Shoals "Scope and
limitations" discipline shows that per-library scope docs — "what this
verifies / what it does not" — are valuable for every module group, not just
the ones with trusted boundaries. This change generalizes the existing
proof-gap discipline to all major module groups so reviewers and consumers
know what each category guarantees and where to reach for something else.

## What Changes

- Add `docs/module-scope.md`: per-module-group scope/limitations convention
  with three sections per group (Verifies, Does not, Trust boundaries).
- Document the top-level categories: core arithmetic, Raft consensus,
  concurrency primitives, financial accounting, memory management/allocator,
  storage engine primitives, circuit primitives (R1CS), Nix/Nickel semantics,
  and cross-module properties/composition.
- Reference `proof-gap-sites.tsv` as the authoritative per-site trusted-boundary
  source for groups that have `#[verifier::external_body]` sites.

## Impact

- **Files**: `docs/module-scope.md` (new).
- **Testing**: documentation-only change; no code or tests modified.
- **Non-claims**: scope docs describe what Verus mechanically checks and what
  it does not. They are not correctness proofs, performance claims, or
  completeness guarantees. The 16 trusted boundaries remain listed in
  `proof-gap-sites.tsv`.
