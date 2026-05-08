## Phase 1: Spec Foundation

- [x] [serial] Capture the live trusted-boundary inventory and current active OpenSpec state.
- [x] [serial] Write proposal, design, and verification-gaps delta spec for a source-derived proof-gap audit/closure pass.
- [x] [depends:spec] Validate the active OpenSpec package and commit the spec-only baseline.

## Phase 2: Audit and Classification

- [ ] [serial] Save reviewable source-derived inventory evidence listing every remaining `#[verifier::external_body]` site and every zero-count weakened-boundary category.
- [ ] [depends:audit] Classify all 16 boundaries as permanent trust or closeable structural candidates, with a one-line proof/deferral rationale for each.
- [ ] [depends:audit] Select one closeable boundary family for implementation, or record that no safe closeable candidate exists in this audit slice.

## Phase 3: Closure Attempt and Evidence

- [ ] [depends:classification] Drain the selected closeable boundary family by removing trusted markers or adding verified structural wrappers without introducing replacement weakened markers.
- [ ] [depends:implementation] Update README/docs/OpenSpec inventory counts from source-derived evidence if any boundary count changes.
- [ ] [depends:implementation] Run focused Verus checks, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested checks, and `./scripts/verify.sh`; save reviewable evidence.
- [ ] [depends:verification] Mark tasks complete, sync/archive the change, commit, push, and verify clean state.
