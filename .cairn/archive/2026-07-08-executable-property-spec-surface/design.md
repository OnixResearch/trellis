## Design: executable property spec surface

### Decision: index, not reorganization

Trellis's properties live in `src/` as `proof fn` and `proptest!` blocks.
Reorganizing them into a `properties/` directory would be a massive,
high-risk change with no verification benefit. Instead, this change creates
a discoverable index (`docs/property-spec-index.md`) that catalogs the
property groups, their source modules, and their proof kinds. The index makes
existing properties discoverable without moving them.

### Connection to cairn Phase 3

cairn's EARS property vocabulary gate (Phase 3 `ears-property-vocabulary-gate`)
needs dispatch targets — properties that EARS requirement responses can
resolve to. This index provides the mapping: when cairn classifies an EARS
response as `is_monotone_increasing`, it can point to the trellis property
group (e.g., `raft` / `src/commit_advance.rs`) that contains the corresponding
`proof fn` or `proptest`.

### Future: runner

A `trellis prove --properties <group>` runner would require a CLI binary,
which trellis doesn't currently have. The index is the foundation; the runner
can be added in a follow-up change when a CLI binary is introduced.

### Non-claims

The index is a discoverability surface, not a completeness claim. It does not
list every property, every module, or every proof. See `docs/module-scope.md`
for per-group scope and limitations, and `docs/proof-gap-sites.tsv` for the
16 trusted boundaries.
