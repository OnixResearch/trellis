## Why

`verified-logic` now has 300+ modules, 1400+ tracked requirements, active
OpenSpec changes, and repo-local guidance files. `rg` is fine for one-off
searches, but repeated impact analysis still means reparsing the tree on
nearly every question: where a requirement is defined, which files mention
it, which module owns a symbol, which OpenSpec change covers an area, and
what changed since the last scan.

A local persistent index fixes that. BLAKE3 gives deterministic content
identity for tracked files, and redb gives a single embedded database with
transactional commits and fast reopen. That combination should let us keep
repo navigation local, incremental, and scriptable without adding a daemon
or network service.

## What Changes

- **repo-index.content-digest**: Compute a BLAKE3 digest from the raw bytes
  of every tracked file and use that digest as the content identity key.
- **repo-index.persistent-store**: Persist index state in redb so restart
  does not require full re-extraction before queries work.
- **repo-index.atomic-refresh**: Publish refreshes as committed generations;
  failed refreshes keep the previous committed generation readable.
- **repo-index.incremental-refresh**: Reuse extracted facts for unchanged
  digests while updating path rows for added, changed, and deleted files.
- **repo-index.cross-reference**: Index Rust items plus requirement ids and
  reference sites across `src/`, `docs/requirements.md`, `openspec/`,
  `README.md`, and `AGENTS.md`.
- **repo-index.query**: Add deterministic local queries for path, symbol,
  requirement id, digest, and basic index statistics.

## Capabilities

### New Capabilities

- `repo-index.content-digest`
- `repo-index.persistent-store`
- `repo-index.atomic-refresh`
- `repo-index.incremental-refresh`
- `repo-index.cross-reference`
- `repo-index.query`

## Impact

- **Files**: `Cargo.toml`, likely `src/repo_index_*.rs`, possibly
  `src/bin/repo-index.rs`, `tests/repo_index.rs`, `docs/requirements.md`,
  and `openspec/specs/repo-index/spec.md`
- **APIs**: Internal tool / CLI first; no new stable public library API is
  required in the first round
- **Dependencies**: `redb`, `blake3`
- **Testing**: Unit and property tests for rebuild idempotence,
  add/change/delete refresh behavior, and persistence across reopen; if we
  add reusable verified model helpers, they also get Verus proofs and Tracey
  coverage
