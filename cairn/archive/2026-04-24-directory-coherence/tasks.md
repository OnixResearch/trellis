## Phase 1: Core model

- [x] Add `src/directory_coherence.rs` with `DirectoryEntry`, request/message structs, and explicit directory state constants
- [x] Implement `process_read_request`, `process_write_request`, and `process_eviction`
- [x] Model full-map sharers with a verified `Vec<bool>` bit-vector plus optional exclusive owner
- [x] Export `pub mod directory_coherence;` from `src/lib.rs`
- [x] Add `check_consistency` and `directory_to_mesi`

## Phase 2: Requirements and tests

- [x] Add `directory.*` requirements to `docs/requirements.md`
- [x] Add focused unit tests for read/write transitions, invalidation generation, and eviction cleanup
- [x] Add negative tests for invalid or stale directory/cache combinations, including `r[directory.consistency.scenario-stale-record-rejected]`
- [x] Add proptests for random-operation consistency, single-writer outcomes, MESI embedding validity, and the different-line independence scenario over separate `DirectoryEntry` values
- [x] Add the canonical spec at `openspec/specs/directory-coherence/spec.md`
- [x] Update `README.md` headline counts and module tables for `directory_coherence`
- [x] Verify the updated README counts with explicit grep/cargo/verus command outputs

## Phase 3: Validation and archive

- [x] Run focused cargo tests with the repo napkin `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet directory_coherence` pattern and capture evidence
- [x] Verify that `docs/requirements.md` top-level `r[directory.*]` IDs match the change spec's top-level requirement IDs
- [x] Run a Tracey traceability scan confirming every `r[directory.*]` ID in the change spec is annotated in at least one implementation site and one test in `src/directory_coherence.rs`
- [x] Run `./scripts/verus.sh src/lib.rs` and capture evidence for the new `directory_coherence` module plus its imported MESI compatibility surface
- [ ] Run OpenSpec proposal/design/tasks gates and capture evidence
- [ ] Archive the change under `openspec/changes/archive/` with evidence
