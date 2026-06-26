## Why

The 2026-04-08 proof audit found that the repository's advertised proof
posture no longer matches the current tree.

Current evidence from the audit:

- `./scripts/verify.sh` is not green.
  - `cargo test` passes: `3160 passed; 0 failed`
  - `cargo clippy --all-targets -- -D warnings` fails
  - Verus fails in `src/mesi.rs` and `src/page.rs`
- `tracey query status` reports `1191 of 1197 requirements are covered`
  and `1192 of 1197 have a verification reference`
- `tracey query uncovered` reports 6 uncovered rules:
  - `ct-select.flag-true`
  - `ct-select.flag-false`
  - `nonce.monotonic`
  - `nonce.uniqueness`
  - `query-cost.index-scan`
  - `snapshot-iso.si-allows-commit`
- `tracey query untested` reports 5 untested rules:
  - `index-select.best-index`
  - `index-select.column-count`
  - `mvcc.chain-latest`
  - `mvcc.is-committed`
  - `snapshot-iso.write-set-size`
- The current tree contains 29 `#[verifier::external_body]` items across
  11 modules, but `README.md` still claims 22 trusted axioms and
  `openspec/specs/verification-gaps/spec.md` still claims 13 external_body
  items across 4 files.
- The trust surface is larger than the current inventory:
  - `assume(...)` soundness oracles in `src/topo_sort.rs` and
    `src/reachability.rs`
  - `@verified-logic:no-ensures` exemptions in `src/index_select.rs`,
    `src/nix_closure.rs`, `src/settlement.rs`, and `src/segregated_free.rs`
  - a real missing `ensures` on `src/cycle_detect.rs:has_cycle`
  - weakened/spec-placeholder surfaces in `src/index_select.rs` and
    `src/nix_closure.rs`
- `scripts/check-ensures.sh` currently produces false positives on long
  signatures such as `src/cron_match.rs:matches_schedule` and
  `src/cron_next.rs:next_occurrence` because it only scans 15 lines past
  the function signature.

Without a tracked remediation plan, future work will keep rediscovering the
same mismatch between the live proof state, the trust inventory, and the
published documentation.

## What Changes

- Create a remediation plan for the proof audit findings.
- Expand the verification-gap inventory so it tracks all trusted or weakened
  proof boundaries, not just a stale subset of `external_body` items.
- Restore a truthful verification status by fixing the current Clippy,
  Verus, and Tracey failures.
- Tighten audit tooling so the project can distinguish real contract holes
  from checker false positives.

## Capabilities

### New Capabilities

- `proof-audit.inventory`: Enumerate every trusted or weakened proof boundary
  with classification and rationale
- `proof-audit.status`: Report the live proof posture from tool output rather
  than stale documentation
- `proof-audit.annotation-closure`: Track and close Tracey uncovered/untested
  gaps found during a proof audit
- `proof-audit.tooling`: Make proof-audit scripts accurate enough to trust in
  CI and manual audits

## Impact

- **Files**: `openspec/specs/verification-gaps/spec.md`, `README.md`,
  `scripts/check-ensures.sh`, and the audited source modules
- **APIs**: No intended public API expansion; the change is about proof
  posture, inventory accuracy, and auditability
- **Dependencies**: None
- **Testing**: `./scripts/verify.sh`, `tracey query uncovered`,
  `tracey query untested`, and targeted scans for `external_body`,
  `assume(...)`, and `@verified-logic:no-ensures`
