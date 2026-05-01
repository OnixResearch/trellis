# Verification

## Drain Verification Matrix

| Rail | Command | Status | Artifact | Scope rationale | Next best check |
|---|---|---|---|---|---|
| Shared outcome scaffold runtime test | `cargo test --quiet composite_outcome` | PASS | `openspec/changes/compose-verified-primitives/evidence/composite-outcome-cargo-test.txt` | Runs the new module's public helper regression without expanding to unrelated crate tests. | `cargo test --quiet` during final full validation. |
| Shared outcome scaffold Verus proof | `./scripts/verus.sh src/composite_outcome.rs` | PASS | `openspec/changes/compose-verified-primitives/evidence/composite-outcome-verus.txt` | Verifies the new proof-facing outcome tag helpers and disjoint/exhaustive lemmas in isolation. | `./scripts/verus.sh src/lib.rs` during final full-library validation. |
| Requirements document sync | `tracey query uncovered` | EXPECTED GAP | `openspec/changes/compose-verified-primitives/evidence/requirements-doc-uncovered-expected.txt` | Confirms the twenty new `docs/requirements.md` entries are visible to Tracey; all twenty are expected uncovered until their module facades land. | Per-module `r[impl]` / `r[verify]` tags, then final `tracey query uncovered` and `tracey query untested` zero-gap runs. |
| Requirements document OpenSpec audit | `python ~/.hermes/skills/agentkit-port/openspec/scripts/openspec_helper.py verify compose-verified-primitives --json` | EXPECTED GAP | `openspec/changes/compose-verified-primitives/evidence/requirements-doc-openspec-verify.json` | Rechecks the active change after adding requirement docs; remaining findings are incomplete tasks and known standalone-ID helper warnings. | Resolve task completion and helper/gate findings before archive. |
| Library export policy | Inline Python audit of `src/lib.rs` against the twenty planned facade module names | PASS | `openspec/changes/compose-verified-primitives/evidence/lib-export-policy.txt` | Confirms no unlanded composite facade modules are exported before their implementation tasks land. | Re-run or update this audit as each facade module is added. |
