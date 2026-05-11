# Verification

## Commands / Oracles

- command: `./scripts/check-aspen-evidence-index.rs --write-report openspec/changes/archive/2026-05-11-add-aspen-verified-logic-snapshot/evidence/aspen-index-pass.md`
- result: pass — index validated 8 Aspen coordination evidence rows against recorded source commits, paths, and blob ids.

- command: `./scripts/check-aspen-evidence-index.rs --fixture-missing-source-path --write-report openspec/changes/archive/2026-05-11-add-aspen-verified-logic-snapshot/evidence/aspen-index-negative-missing-source.md > openspec/changes/archive/2026-05-11-add-aspen-verified-logic-snapshot/evidence/aspen-index-negative-missing-source.out 2> openspec/changes/archive/2026-05-11-add-aspen-verified-logic-snapshot/evidence/aspen-index-negative-missing-source.err`
- result: fail as expected — checker exited 1 and reported a missing source path at the recorded Aspen commit.

- command: `./scripts/check-aspen-evidence-index.rs --fixture-malformed-row > openspec/changes/archive/2026-05-11-add-aspen-verified-logic-snapshot/evidence/aspen-index-negative-malformed.out 2> openspec/changes/archive/2026-05-11-add-aspen-verified-logic-snapshot/evidence/aspen-index-negative-malformed.err`
- result: fail as expected — checker exited 1 and reported a missing required `verus_result` field.

- command: `./scripts/check-aspen-evidence-index.rs && bash -n scripts/verify.sh && ./scripts/check-proof-gap-inventory.rs && openspec validate add-aspen-verified-logic-snapshot --strict --json && git diff --check`
- result: pass — Aspen index drift gate, shell syntax, proof-gap inventory, OpenSpec strict validation, and whitespace checks all passed.

## Snapshot decision

No Aspen proof code is snapshotted in this implementation slice. `docs/aspen-proof-snapshots.md` defines the manifest-backed layout for future opt-in snapshots, and every current index row uses `local_snapshot_manifest` = `-`, so no local Verus snapshot command applies.
