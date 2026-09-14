## Phase 1: Measured frontier

- [x] [serial] Define the frontier record and classifier inputs from recorded provider observations, with the class set `tier-1-primitive`, `tier-2-collection`, `tier-3-external`, `tier-4-trust-boundary`, `excluded-construct`, `blocked-<stage>`, and `unmeasured`. r[trellis.aeneas_lean_coverage.frontier]
- [x] [serial] Implement the frontier classifier over the top-level `src/*.rs` inventory, keep source-scan results as hypotheses, and require a recorded observation before a measured class. r[trellis.aeneas_lean_coverage.frontier]
- [x] [parallel] Add positive fixtures for each measured class and negative fixtures for a hypothesis-only module, a missing blocked stage, and an unknown class value. r[trellis.aeneas_lean_coverage.frontier] r[trellis.aeneas_lean_coverage.negative_evidence]
- [x] [parallel] Emit the frontier artifact with measured counts, hypothesis counts, provider identities, and the class of every module. r[trellis.aeneas_lean_coverage.frontier] r[trellis.aeneas_lean_coverage.receipt]

## Phase 2: Requirement statement registry

- [x] [serial] Define the registry schema and validate every row against the requirement IDs in `docs/requirements.md`, including statement BLAKE3, claim class, module class, model notes, trust closure, and receipt reference. r[trellis.aeneas_lean_coverage.registry]
- [x] [serial] Populate the registry for the batch targets and record an explicit non-target reason for every other requirement in scope. r[trellis.aeneas_lean_coverage.registry]
- [x] [parallel] Add negative fixtures for an unknown requirement ID, a duplicate statement hash across requirements, a missing trust closure, and a missing non-target reason. r[trellis.aeneas_lean_coverage.registry] r[trellis.aeneas_lean_coverage.negative_evidence]

## Phase 3: Bounded Tier-1 batch

- [ ] [serial] [depends:add-aeneas-cross-verifier-pilot] Consume the reviewed Octet provider profile and run one batch inside `modules_per_batch` (8) modules and `theorems_per_module` (4), with per-stage limits inherited from the provider profile. r[trellis.aeneas_lean_coverage.batch] r[trellis.aeneas_lean_coverage.reuse]
  - Progress 2026-09-14: the pilot archive records `supported` for `src/arithmetic.rs::abs_diff`. The Octet provider closure build is running in this environment, so no second observation is recorded yet. Next check: run the provider Charon stage with `--start-from crate::<module>::<item>` for the first Tier-1 candidate and record one observation per item.
- [ ] [serial] Register each passing theorem with its approved statement hash, claim class, trust closure, and pilot receipt reference. r[trellis.aeneas_lean_coverage.registry] r[trellis.aeneas_lean_coverage.batch]
- [ ] [parallel] Add the intentionally false theorem, the vacuous `True` theorem, and tamper fixtures for frontier class, registry row, statement hash, trust closure, provider identity, receipt, and budget. r[trellis.aeneas_lean_coverage.negative_evidence]
- [ ] [parallel] Record the per-module and per-stage budget identity and outcome, and classify exhaustion as a budget result. r[trellis.aeneas_lean_coverage.batch] r[trellis.aeneas_lean_coverage.receipt]

## Phase 4: Receipt, claim boundary, and validation

- [x] [serial] Render the coverage receipt with class counts, named bounds, provider identities, and explicit non-claims. r[trellis.aeneas_lean_coverage.receipt]
- [ ] [serial] Implement the claim-boundary check and reject whole-library proof coverage, progress-score, verifier-independence, semantic-equivalence, and trust-boundary-closure wording. r[trellis.aeneas_lean_coverage.claim_boundary]
  - Progress 2026-09-14: the artifact non-claims are pinned to the required set, so a changed artifact fails. A documentation-text overclaim scanner remains open.
- [x] [serial] Document the frontier classes and counts, registry rules, batch bounds, provider revision, commands, and non-claims. r[trellis.aeneas_lean_coverage.claim_boundary] r[trellis.aeneas_lean_coverage.registry]
- [ ] [serial] Run the provider preflight, frontier/registry/receipt/tamper checks, focused Cargo tests, the repository Verus command, `tracey query uncovered`, and Cairn validation; if blocked, record the exact blocked receipt and the smallest next discriminating check. r[trellis.aeneas_lean_coverage.final_validation]
  - Progress 2026-09-14: checker Clippy with `-D warnings`, `--self-test` (20 negative cases), `--write`, and `--check` pass. The checker is wired into `scripts/verify.rs`. The repository-wide Verus, Tracey, and Cairn runs remain open.
