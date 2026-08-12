## Phase 1: Core model and proofs

- [x] Compare this change against `src/lib.rs`, `docs/requirements.md`, and `openspec/specs/`; the repo-sync audit in `evidence/repo-sync.txt` pins baseline commit `d0cedf8b` and confirms `sel4_cdt` / `sel4-cdt.*` were net-new rather than already shipped.
- [x] Create `src/sel4_cdt.rs` with parent-child ancestry, liveness, strict-ancestor, subtree-membership, revoke, and recycle predicates; export the module from `src/lib.rs`.
- [x] Model copy, mint, move, revoke, and recycle transitions with exact exec/spec postconditions over the live tree.
- [x] Add real `proof fn` artifacts for structural ancestry, move ancestry preservation, exact revoke-on-live-subtree semantics, and leaf-only recycle preservation of the live-parent invariant.
- [x] Add unit tests and proptests for copy/mint child creation, move ancestry preservation, sibling preservation under revoke, descendant removal under revoke, leaf-only recycle success, and non-leaf recycle rejection; capture the explicit module Verus run in `evidence/verus-sel4-cdt.txt` and targeted cargo test run in `evidence/cargo-test-sel4-cdt.txt`.
- [x] Mirror `r[sel4-cdt.structure]`, `r[sel4-cdt.move]`, `r[sel4-cdt.revoke]`, and `r[sel4-cdt.recycle]` into `docs/requirements.md`, add `r[impl]` / `r[verify]` tags on the defining helpers/proofs/tests, sync the landed spec text into `openspec/specs/sel4-cdt/spec.md`, and keep the change-local spec copy aligned.
- [x] Run repo-level validation with fresh captured output: `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`, `./scripts/verify.sh`, then `tracey query uncovered` and `tracey query untested` sequentially. Saved evidence under `openspec/changes/archive/2026-04-15-sel4-cap-derivation/evidence/`.
- [x] Archive `openspec/changes/sel4-cap-derivation` once validation passes and active/canonical specs stay in sync.
