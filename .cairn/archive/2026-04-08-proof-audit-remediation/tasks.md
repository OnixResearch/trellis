## Phase 1: Capture the audited state

- [x] Capture `./scripts/verify.sh` output for the failing Clippy, test, Verus, and Tracey gates
- [x] Capture `tracey query uncovered` and `tracey query untested` output in the change evidence
- [x] Scan the tree for `#[verifier::external_body]`, `assume(...)`, `@verified-logic:no-ensures`, and real missing-`ensures` sites
- [x] Classify each audited item as permanent, closeable, or tooling

## Phase 2: Restore a green verification pipeline

- [x] Fix the current Clippy failures reported by `./scripts/verify.sh`
- [x] Fix the current Verus failures in `src/mesi.rs`
- [x] Fix the current Verus failures in `src/page.rs`
- [x] Re-run `./scripts/verify.sh` until Clippy, tests, and Verus are green

## Phase 3: Close Tracey annotation drift

- [x] Add the missing `r[impl]` references for `ct-select.flag-true`, `ct-select.flag-false`, `nonce.monotonic`, and `nonce.uniqueness`
- [x] Add the missing `r[impl]` references for `query-cost.index-scan` and `snapshot-iso.si-allows-commit`
- [x] Add the missing `r[verify]` references for `index-select.best-index`, `index-select.column-count`, `mvcc.chain-latest`, `mvcc.is-committed`, and `snapshot-iso.write-set-size`
- [x] Re-run `tracey query uncovered` until it is zero
- [x] Re-run `tracey query untested` until it is zero

## Phase 4: Tighten the trust surface inventory

- [x] Decide the disposition of the `assume(...)` oracles in `src/topo_sort.rs` and `src/reachability.rs`
- [x] Replace or justify the weakened proof surfaces in `src/index_select.rs`, `src/nix_closure.rs`, `src/settlement.rs`, `src/segregated_free.rs`, and `src/cycle_detect.rs`
- [x] Reclassify the current `external_body` items in `src/bitwise.rs`, `src/cron_field.rs`, and `src/cron_next.rs` in the main gap inventory with rationale and test coverage

## Phase 5: Fix audit tooling and documentation

- [x] Fix `scripts/check-ensures.sh` so long signatures do not trigger false positives
- [x] Update `openspec/specs/verification-gaps/spec.md` to reflect the full audited inventory
- [x] Update `README.md` proof-posture counts and trust-boundary description from the regenerated inventory
