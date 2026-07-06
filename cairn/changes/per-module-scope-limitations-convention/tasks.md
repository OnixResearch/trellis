## Phase 1: Implementation

- [x] [serial] r[trellis.module_scope] Create `docs/module-scope.md` with the per-module-group scope/limitations convention (Verifies, Does not, Trust boundaries) and entries for the top-level categories: core arithmetic, Raft, concurrency, financial accounting, memory/allocator, storage, R1CS, Nix/Nickel, cross-module properties.
- [x] [serial] r[trellis.module_scope] Reference `docs/proof-gap-sites.tsv` as the authoritative per-site trusted-boundary source for groups with `#[verifier::external_body]` sites.

## Phase 2: Archive

- [ ] [serial] r[trellis.module_scope] Verify and archive the change.

## Verification Coverage

| Scenario | Evidence |
|---|---|
| `docs/module-scope.md` exists with convention | file created, 9 category entries |
| Convention has three sections per group | each entry has Verifies, Does not, Trust boundaries |
| References proof-gap-sites.tsv for trust boundaries | Nix/Nickel and cross-module entries cite `proof-gap-sites.tsv` |
| Documentation-only change, no code modified | no `.rs` files changed |
