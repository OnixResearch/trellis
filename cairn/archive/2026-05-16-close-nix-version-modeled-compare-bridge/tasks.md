## Phase 1: Specification

- [x] [serial] Create proposal, design, tasks, and verification-gaps delta for the modeled comparator bridge slice.

## Phase 2: Implementation

- [x] [depends:spec] Add verified executable helpers for finite modeled separator skipping, component bounds, numeric parsing, and lexicographic comparison as needed.
- [x] [depends:spec] Replace the admitted branch of `compare_versions_modeled` with the verified modeled comparator path, leaving out-of-model inputs as `None`.
- [x] [depends:spec] Add `r[impl]` and `r[verify]` coverage for `verification-gaps.nix-version-modeled-compare-bridge` without adding trusted/weakened markers.
- [x] [depends:spec] Add focused positive and negative tests for admitted equal/less/greater cases and out-of-model rejection.
- [x] [depends:spec] Sync `docs/requirements.md` and `openspec/specs/verification-gaps/spec.md` with the implemented claim and inventory wording.

## Phase 3: Validation / Archive

- [x] [depends:implementation] Run `openspec validate close-nix-version-modeled-compare-bridge --strict`, focused Verus/tests, Tracey uncovered/untested, proof-gap inventory, and full verification evidence.
- [x] [depends:validation] Archive the change, commit, push `main`, and verify clean Git/JJ state.
