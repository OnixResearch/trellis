# Verification

## Focused checks

Transcript: `/tmp/onix-trellis-ucan-bridge-focused-20260708.txt`

- `nix develop -c rustfmt scripts/check-ucan-bridge-coverage.rs scripts/check-verified-primitive-manifest.rs scripts/verify.rs`
- `./scripts/check-verified-primitive-manifest.rs`
- `./scripts/check-ucan-bridge-coverage.rs --self-test`
- `./scripts/check-ucan-bridge-coverage.rs`

Results: formatting passed; primitive manifest refresh passed with `4` primitives and `1` Preserves envelope; UCAN bridge self-test passed; UCAN bridge repo check passed with `2` rows and `5` fixtures.

## Archive and final checks

Transcripts:

- `/tmp/onix-trellis-ucan-bridge-gate-sync-archive-20260708.txt`
- `/tmp/onix-trellis-ucan-bridge-final-validation-20260708.txt`

Results:

- Proposal, design, and tasks gates passed.
- Cairn validation passed before sync, after sync, after archive, and after accepted-spec repair.
- The accepted `ucan-bridge-coverage` spec was repaired from the archived delta after sync created an empty wrapper.
- Scoped rustfmt checks, primitive manifest refresh, UCAN bridge self-test, UCAN bridge repo check, and focused ID audit passed.
- Full `traceability coverage` still fails under the broad repo profile due pre-existing unrelated Trellis source-vs-requirement references; the full failure output contains no `trellis.ucan_bridge_coverage.*` IDs.

## Non-claims

This proves UCAN bridge-to-Trellis primitive identity coverage, fixture behavior, and ownership boundary text. It does not prove UCAN parser correctness, resolver truth, replay-store durability, token orchestration, or consumer authorization completeness.
