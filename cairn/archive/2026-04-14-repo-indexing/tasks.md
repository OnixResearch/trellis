Validation evidence: see `evidence/final-tasks.md`, `evidence/validation.md`, and the raw transcripts in `evidence/*.txt`.

## Phase 1: Scaffold and schema

- [x] Add `redb` and `blake3` dependencies, choose the index file location, and scaffold the internal modules or CLI entrypoint
- [x] Define the redb schema, generation metadata, and serialized row formats for paths, digests, symbols, requirements, and edges
- [x] Finish Phase 1 with `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `tracey query uncovered`, and `tracey query untested` clean

## Phase 2: Scan and refresh

- [x] Implement tracked-root scanning and BLAKE3 digest computation over raw file bytes
- [x] Implement atomic refresh: write a new generation, swap the active pointer on commit, and preserve the prior generation on failure
- [x] Handle added, changed, unchanged, and deleted files in the active path view
- [x] Finish Phase 2 with `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `tracey query uncovered`, and `tracey query untested` clean

## Phase 3: Extraction and queries

- [x] Extract Rust items and `r[...]` references from `src/**/*.rs`
- [x] Extract requirement definitions from `docs/requirements.md` and active OpenSpec docs
- [x] Implement deterministic queries for path, symbol, requirement id, digest, and index stats
- [x] Finish Phase 3 with `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `tracey query uncovered`, and `tracey query untested` clean

## Phase 4: Hardening

- [x] Add unit tests for reopen persistence, unchanged-content reuse, and delete/update refresh behavior
- [x] Add property tests for idempotent rebuilds and one-shot vs incremental refresh equivalence
- [x] Add `docs/requirements.md` entries and Tracey annotations for any verified helpers or stable public surfaces introduced by the index work
- [x] If reusable verified model helpers are added, prove their contracts
- [x] Finish Phase 4 with `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `tracey query uncovered`, and `tracey query untested` clean
