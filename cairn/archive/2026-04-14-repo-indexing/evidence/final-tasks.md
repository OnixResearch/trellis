# Repo Indexing — Final Task Checklist

## Phase 1: Scaffold and schema
- [x] Add `redb` and `blake3` dependencies, choose the index file location, and scaffold the internal modules or CLI entrypoint
- [x] Define the redb schema, generation metadata, and serialized row formats for paths, digests, symbols, requirements, and edges
- [x] Finish Phase 1 validation

## Phase 2: Scan and refresh
- [x] Implement tracked-root scanning and BLAKE3 digest computation over raw file bytes
- [x] Implement atomic refresh: write a new generation, swap the active pointer on commit, and preserve the prior generation on failure
- [x] Handle added, changed, unchanged, and deleted files in the active path view
- [x] Finish Phase 2 validation

## Phase 3: Extraction and queries
- [x] Extract Rust items and `r[...]` references from `src/**/*.rs`
- [x] Extract requirement definitions from `docs/requirements.md` and active OpenSpec docs
- [x] Implement deterministic queries for path, symbol, requirement id, digest, and index stats
- [x] Finish Phase 3 validation

## Phase 4: Hardening
- [x] Add unit tests for reopen persistence, unchanged-content reuse, and delete/update refresh behavior
- [x] Add property tests for idempotent rebuilds and one-shot vs incremental refresh equivalence
- [x] Add `docs/requirements.md` entries and Tracey annotations for any verified helpers or stable public surfaces introduced by the index work
- [x] If reusable verified model helpers are added, prove their contracts
- [x] Finish Phase 4 validation

## Evidence
- Full-tree validation: `validation.md`
- Raw command output: `test-transcript.txt`, `clippy-transcript.txt`, `verus-transcript.txt`, `tracey-transcript.txt`
- Active-change archive absence check: `archive-status.txt`
- Active OpenSpec requirement-definition smoke query: `openspec-query-transcript.txt`
