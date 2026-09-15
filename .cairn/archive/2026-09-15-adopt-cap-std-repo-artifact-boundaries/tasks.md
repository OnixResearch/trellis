## Tasks

- [x] [serial] Add `cap-std` only to Trellis repository tooling that owns filesystem effects. r[trellis.repo_index.cap_std_boundary.dependency]
  - Evidence: `cap-std = "4.0.3"` is a package dependency; `src/tooling/locator.rs`, `src/tooling/repository_roots.rs`, and `src/tooling/evidence_roots.rs` are included only by `src/bin/repo-index.rs` and `src/bin/proof_evidence.rs`. `src/lib.rs` and every `verus!` module stay free of it.
- [x] [serial] Introduce typed capability roots for repository, index database, proof-evidence input, and artifact output directories. r[trellis.repo_index.cap_std_boundary.root_wrappers]
  - Evidence: `RepositoryRoot` and `IndexDatabaseRoot` wrap the repository-index roots; `ProofEvidenceInputRoot` and `ArtifactOutputRoot` wrap the proof-evidence roots. Each opens an operator-declared directory once with `cap_std::fs::Dir`.
- [x] [serial] Convert repo-index and proof-evidence path opens to capability-relative operations while keeping verified modules dependency-free. r[trellis.repo_index.cap_std_boundary.conversion]
  - Evidence: `src/repo_index/store.rs` reads, lists, and classifies through `RepositoryRoot`, and `RepoIndex::open` takes the opened roots plus a validated database locator. `src/bin/proof_evidence.rs` reads inputs, walks source directories, and writes the bundle through the typed roots. Neither binary opens an ambient path any more.
- [x] [serial] Add positive fixtures for valid repo-relative paths, index database access, proof-evidence inputs, and artifact outputs. r[trellis.repo_index.cap_std_boundary.tests.positive]
  - Evidence: `cargo test --bins` passes `positive_repository_read_stays_under_the_root`, `positive_index_database_root_bounds_the_database_path`, `positive_input_read_and_output_write_stay_under_the_roots`, `positive_locator_parses_and_keeps_its_text`, and the existing repo-index store suite.
- [x] [serial] Add negative fixtures for traversal, absolute paths, symlink escapes, missing roots, and database/output escape attempts. r[trellis.repo_index.cap_std_boundary.tests.negative]
  - Evidence: negative tests reject traversal, absolute, and drive-prefixed locators, a missing root, and a symlink escape from the repository root and from the input root. An escape locator cannot be constructed at all.
- [x] [serial] Document the repository-tooling filesystem boundary and verified-logic non-claims. r[trellis.repo_index.cap_std_boundary.docs]
  - Evidence: `docs/repo-tooling-filesystem-boundary.md` names the four roots, the locator rules, the fixture coverage, the verified-logic non-claims, and the Octet baseline re-keying that a dependency change causes.
- [x] [serial] Run focused repo-index/proof-evidence checks plus Cairn validation and gates. r[trellis.repo_index.cap_std_boundary.validation]
  - Evidence: `cargo test --bins` passes 47 tests across the touched binaries; `./scripts/verify.rs` reports `ALL CHECKS PASSED`; `cairn validate` exits 0 with zero issues; the Octet baseline check accepts 3867 of 3867 findings with zero new and zero stale after the pinned baseline refresh.
