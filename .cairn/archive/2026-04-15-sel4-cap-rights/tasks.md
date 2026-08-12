## Phase 1: Core model and proofs

- [x] Compare this change against `src/lib.rs`, `docs/requirements.md`, and `openspec/specs/`; the repo-sync audit in `evidence/repo-sync.txt` confirmed `sel4_cap_rights` and `sel4-cap-rights.*` were net-new rather than already shipped.
- [x] Create `src/sel4_cap_rights.rs` with explicit right bits, subset/mask/attenuate helpers, and operation-gating predicates for send, receive, grant-like transfer, map, and control.
- [x] Add real `proof fn` artifacts for no-privilege-gain attenuation, copy-preserves-rights, subset-request roundtrips, masking idempotence, masking non-amplification, and masking monotonicity.
- [x] Add unit tests and proptests for copy preservation, rights dropping, masking idempotence/non-amplification, and send/map/control gate checks; capture the explicit module run in `evidence/cargo-test-sel4-cap-rights.txt`.
- [x] Mirror `r[sel4-cap-rights.attenuation]`, `r[sel4-cap-rights.operation-gates]`, and `r[sel4-cap-rights.masking]` into `docs/requirements.md`, add Tracey anchors on the defining helpers/proofs/tests, and export the module from `src/lib.rs`.
- [x] Run repo-level validation with fresh captured output: `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`, `./scripts/verify.sh`, then `tracey query uncovered` and `tracey query untested` sequentially. Saved evidence: `evidence/cargo-test.txt`, `evidence/clippy.txt`, `evidence/verus.txt`, `evidence/check-ensures.txt`, `evidence/verify.txt`, `evidence/tracey-uncovered.txt`, `evidence/tracey-untested.txt`.
- [x] Sync the landed spec text into `openspec/specs/sel4-cap-rights/spec.md` and archive `openspec/changes/sel4-cap-rights` after validation passes.
