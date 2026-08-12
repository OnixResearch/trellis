## Tasks

- [ ] [serial] Add `cap-std` only to Trellis repository tooling that owns filesystem effects. r[trellis.repo_index.cap_std_boundary.dependency]
- [ ] [serial] Introduce typed capability roots for repository, index database, proof-evidence input, and artifact output directories. r[trellis.repo_index.cap_std_boundary.root_wrappers]
- [ ] [serial] Convert repo-index and proof-evidence path opens to capability-relative operations while keeping verified modules dependency-free. r[trellis.repo_index.cap_std_boundary.conversion]
- [ ] [serial] Add positive fixtures for valid repo-relative paths, index database access, proof-evidence inputs, and artifact outputs. r[trellis.repo_index.cap_std_boundary.tests.positive]
- [ ] [serial] Add negative fixtures for traversal, absolute paths, symlink escapes, missing roots, and database/output escape attempts. r[trellis.repo_index.cap_std_boundary.tests.negative]
- [ ] [serial] Document the repository-tooling filesystem boundary and verified-logic non-claims. r[trellis.repo_index.cap_std_boundary.docs]
- [ ] [serial] Run focused repo-index/proof-evidence checks plus Cairn validation and gates. r[trellis.repo_index.cap_std_boundary.validation]
