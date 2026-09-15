## Design

Trellis will keep `cap-std` out of verified-logic modules and use it only in shell/adaptor code that opens local roots. The shell opens repository, index database, proof-evidence input, and artifact output roots as `cap_std::fs::Dir` values and passes typed wrappers to the relevant tooling.

The first conversion targets are `src/repo_index/store.rs` and `src/bin/proof_evidence.rs`. If additional helper modules are needed, they should stay outside Verus proof surfaces and feed in-memory bytes or validated relative locators into pure logic.

Repo-index refresh and proof-evidence bundle generation should preserve existing BLAKE3 identity behavior, Tracey requirement extraction, and proof-artifact manifests for valid inputs. Negative diagnostics should reject traversal, absolute paths, symlink escapes, missing roots, and database/output escape attempts without exposing host-private absolute paths.

The change does not prove new verified-logic properties, proof soundness, artifact truth, or consumer release readiness. It only bounds local filesystem authority for selected repository tooling.
