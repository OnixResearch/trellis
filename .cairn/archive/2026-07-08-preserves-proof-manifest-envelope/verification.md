# Verification

## Baseline

Transcript: `/tmp/onix-trellis-preserves-envelope-baseline-20260708.txt`

- Proposal, design, and tasks gates passed.
- `nix run path:../cairn#cairn -- validate --root .` passed.
- Existing `./scripts/check-verified-primitive-manifest.rs` passed before the Preserves envelope implementation.

## Focused implementation checks

Transcript: `/tmp/onix-trellis-preserves-envelope-focused-20260708.txt`

- `nix develop -c rustfmt scripts/check-verified-primitive-manifest.rs`
- `./scripts/check-verified-primitive-manifest.rs --self-test`
- `./scripts/check-verified-primitive-manifest.rs`

Results: formatting passed; self-test passed; repository check passed with `2` primitives, `1` Preserves envelope, and `11` positive/negative fixtures.

## Archive and final checks

Transcripts:

- `/tmp/onix-trellis-preserves-envelope-gate-sync-archive-20260708.txt`
- `/tmp/onix-trellis-preserves-envelope-final-validation-20260708.txt`

Results:

- Proposal, design, and tasks gates passed.
- Cairn validation passed before sync, after sync, after archive, and after accepted-spec repair.
- The shared accepted `proof-artifact-manifest` spec was manually extended with the Preserves envelope requirements after sync preserved only the prior domain content.
- Scoped `nix develop -c rustfmt --check scripts/check-verified-primitive-manifest.rs` passed.
- `./scripts/check-verified-primitive-manifest.rs --self-test` and `./scripts/check-verified-primitive-manifest.rs` passed with `2` primitives, `1` Preserves envelope, and `11` fixtures.
- Full `traceability coverage` still fails under the broad repo profile due pre-existing unrelated Trellis source-vs-requirement references; the full failure output contains no `trellis.proof_artifact_manifest.preserves_envelope.*` IDs, and a focused audit confirmed each Preserves requirement ID has accepted spec and script evidence references.

## Non-claims

This proves the optional Preserves carrier contract, source-manifest linkage, fixture matrix, and carrier non-claims. It does not make Preserves a proof artifact, replace Verus/spec proof ownership, prove downstream authorization semantics, or certify release eligibility.
