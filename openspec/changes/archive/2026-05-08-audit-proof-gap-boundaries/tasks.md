## Phase 1: Spec Foundation

- [x] [serial] Capture the live trusted-boundary inventory and current active OpenSpec state.
- [x] [serial] Write proposal, design, and verification-gaps delta spec for a source-derived proof-gap audit/closure pass.
- [x] [depends:spec] Validate the active OpenSpec package and commit the spec-only baseline.

## Phase 2: Audit and Classification

- [x] [serial] Save reviewable source-derived inventory evidence listing every remaining `#[verifier::external_body]` site and every zero-count weakened-boundary category. Evidence: `evidence/proof-gap-inventory.md`.
- [x] [depends:audit] Classify all 16 boundaries as permanent trust or closeable structural candidates, with a one-line proof/deferral rationale for each. Evidence: `evidence/proof-gap-inventory.md` and `evidence/classification.md`.
- [x] [depends:audit] Select one closeable boundary family for implementation, or record that no safe closeable candidate exists in this audit slice. Recorded no safe closeable candidate for this slice in `evidence/closure-attempt.md`.

## Phase 3: Closure Attempt and Evidence

- [x] [depends:classification] Drain the selected closeable boundary family by removing trusted markers or adding verified structural wrappers without introducing replacement weakened markers. No closeable family selected; `src/proof_gap_boundary_audit.rs` adds verified audit constants/proofs and source audit confirms no replacement weakened markers.
- [x] [depends:implementation] Update README/docs/OpenSpec inventory counts from source-derived evidence if any boundary count changes. Counts unchanged at 16; canonical docs/OpenSpec gained audit requirements.
- [x] [depends:implementation] Run focused Verus checks, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested checks, and `./scripts/verify.sh`; save reviewable evidence. Evidence: `evidence/verification.md`, summaries/logs under `evidence/`.
- [x] [depends:verification] Mark tasks complete, sync/archive the change, commit, push, and verify clean state.
