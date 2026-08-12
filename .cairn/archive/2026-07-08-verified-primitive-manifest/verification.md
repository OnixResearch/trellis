# Verification

## Baseline

Transcript: `/tmp/onix-trellis-primitive-manifest-baseline-20260708.txt`

- `nix run path:../cairn#cairn -- gate proposal verified-primitive-manifest --root .`
- `nix run path:../cairn#cairn -- gate design verified-primitive-manifest --root .`
- `nix run path:../cairn#cairn -- gate tasks verified-primitive-manifest --root .`
- `nix run path:../cairn#cairn -- validate --root .`
- `./scripts/check-kani-boundary.rs --self-test`

Results: gates and Cairn validation passed; the existing boundary fixture self-test passed before implementation.

## Focused implementation checks

Transcript: `/tmp/onix-trellis-primitive-manifest-focused-20260708.txt`

- `./scripts/check-verified-primitive-manifest.rs --self-test`
- `./scripts/check-verified-primitive-manifest.rs`

Results: self-test passed; repository manifest check passed with `2` primitives and `5` positive/negative fixtures.

## Applicability

No Verus source module was changed in this slice; the new work is a manifest checker, JSON manifest, Nickel contract, docs, and fixtures. Full Verus/Tracey proof reruns are therefore not required for this archive beyond the Cairn lifecycle/traceability checks recorded during archive.

## Archive, accepted-spec repair, and traceability

Transcripts:

- `/tmp/onix-trellis-primitive-manifest-gate-sync-archive-20260708.txt`
- `/tmp/onix-trellis-primitive-manifest-post-repair-20260708.txt`
- `/tmp/onix-trellis-primitive-manifest-final-validation-20260708.txt`

Results:

- Proposal, design, and tasks gates passed.
- Cairn validation passed before sync, after sync, after archive, after accepted-spec repair, and in final focused validation.
- Sync initially created an empty accepted spec wrapper; `cairn/specs/proof-artifact-manifest/spec.md` was repaired from the archived spec delta.
- Full `traceability coverage` still fails under the repo's broad profile because of pre-existing unrelated Trellis source-vs-requirement references; after accepted-spec repair, the full failure output contains no `trellis.proof_artifact_manifest.*` IDs, and a focused audit confirmed every manifest requirement ID has both an accepted spec definition and script evidence marker.
- `nix develop -c cargo fmt --check` is blocked by unrelated existing formatting drift in `src/raft_consensus_core.rs`; scoped `nix develop -c rustfmt --check scripts/check-verified-primitive-manifest.rs scripts/verify.rs` passed.
- `./scripts/check-verified-primitive-manifest.rs --self-test` and `./scripts/check-verified-primitive-manifest.rs` passed in final validation.

## Non-claims

This proves the manifest contract, deterministic freshness check, fixture matrix, and downstream non-claim visibility. It does not prove downstream integration correctness, consumer release eligibility, or Verus soundness outside the declared proof boundary.
