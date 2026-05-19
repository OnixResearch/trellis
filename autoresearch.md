# Autoresearch: reduce active OpenSpec change count

## Objective
Reduce the number of active directories under `openspec/changes/` by actually finishing work, syncing repo specs/docs, and archiving completed changes. Primary metric is active change count. Lower only counts if landed tree still validates.

## Metrics
- **Primary**: `active_changes` (lower is better)
- **Secondary**: validation pass/fail, Tracey uncovered count, Tracey untested count

## How to Run
`./autoresearch.sh`

`autoresearch.checks.sh` runs correctness gates after the metric command:
- `cargo test --quiet`
- `cargo clippy --all-targets -- -D warnings`
- `./scripts/verus.sh src/lib.rs`
- `./scripts/check-ensures.rs`
- `tracey query uncovered`
- `tracey query untested`

## Files in Scope
- `openspec/changes/` — active change artifacts to finish or archive
- `openspec/specs/` — synced canonical specs after archive
- `docs/requirements.md` — Tracey requirement source
- `src/` — verified implementations and proofs needed to close changes
- `tests/` — supporting tests when requirements call for runtime evidence
- `AGENTS.md`, `.agent/napkin.md` — session memory when hard-won repo facts change

## Off Limits
- Benchmark cheating by deleting active changes without syncing code/spec/docs/evidence
- Unrelated churn outside files needed to close the current change
- Weak validation claims without fresh command output

## Constraints
- Active change count only improves when change is truly archived out of `openspec/changes/`
- Keep README/docs/spec counts synced with live tree
- Preserve Tracey coverage: `0 uncovered`, `0 untested`
- Prefer `jj` for repo inspection; autoresearch tool may still commit through git
- Do not claim green from stale logs alone

## What's Been Tried
- Baseline: 34 active changes before sel4-asid-pools remediation.
- Kept: archived `sel4-asid-pools` by landing `src/sel4_asid.rs`, syncing `docs/requirements.md` and `openspec/specs/sel4-asid/spec.md`, and moving the change under `openspec/changes/archive/2026-04-15-sel4-asid-pools/` with evidence. New active count: 33.
- Kept: archived `sel4-cap-rights` by landing `src/sel4_cap_rights.rs`, syncing `docs/requirements.md` and `openspec/specs/sel4-cap-rights/spec.md`, and moving the change under `openspec/changes/archive/2026-04-15-sel4-cap-rights/` with captured validation evidence. New active count: 32.
- Kept: archived `sel4-cspace-lookup` by landing `src/sel4_cspace.rs`, using explicit path-segment guard matching plus slot-selection depth accounting, syncing `docs/requirements.md` and `openspec/specs/sel4-cspace/spec.md`, and moving the change under `openspec/changes/archive/2026-04-15-sel4-cspace-lookup/` with captured validation evidence. New active count: 31.
- Kept: archived `sel4-cap-derivation` by landing `src/sel4_cdt.rs`, syncing `docs/requirements.md` and `openspec/specs/sel4-cdt/spec.md`, capturing targeted and full validation evidence, and moving the change under `openspec/changes/archive/2026-04-15-sel4-cap-derivation/`. New active count: 30.
- Kept: archived `sel4-endpoint-ipc` by landing `src/sel4_endpoint.rs`, syncing `docs/requirements.md` and `openspec/specs/sel4-endpoint/spec.md`, capturing targeted and full validation evidence, and moving the change under `openspec/changes/archive/2026-04-15-sel4-endpoint-ipc/`. New active count: 29.
- Next shortlist from repo scan: larger follow-on changes like `moesi-protocol` or more seL4 queues/state work.
- Saturation: archived `wasm-primitives`; `openspec/changes/` now has `0` active directories. No further honest improvement is possible under the `active_changes` metric without changing the optimization target.
