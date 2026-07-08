## Phase 1: QuePaxa finite model and boundaries

- [x] [serial] r[consensus.quepaxa.boundary] Document the finite crash-stop QuePaxa model boundary in requirements and module docs, including explicit non-claims for network liveness, RNG quality, benchmarks, production SMR, MAB optimality, and Byzantine faults.
  - Evidence 2026-07-08: `src/quepaxa.rs` module docs and `docs/requirements.md` state finite deterministic crash-stop scope and reject liveness/RNG/benchmark/SMR/MAB/Byzantine overclaims.
- [x] [serial] r[consensus.quepaxa.proposal-order] Add proposal metadata, named protocol constants, deterministic proposal ordering, leader-priority admission, and tie-policy helpers with Tracey implementation coverage.
  - Evidence 2026-07-08: `QuePaxaProposal`, named protocol constants, `best_proposal`, `proposal_leq`, and `reserved_leader_priority_admitted` are implemented with proof/test coverage.
- [x] [serial] r[consensus.quepaxa.isr] Add pure ISR transition specs/exec helpers for monotone step advancement, first-value capture, current-step max aggregation, prior-step aggregate return, stale-step discard, and skipped-step nil aggregation.
  - Evidence 2026-07-08: `isr_record` covers same-step aggregation, immediate advance prior return, stale preservation, and skipped-step nil prior, with proof lemmas and positive/negative tests.

## Phase 2: Abstract and concrete consensus guards

- [x] [serial] r[consensus.quepaxa.tcast] Add abstract quorum/tcast propagation predicates for finite proposal sets and the universal/common/existent subset relationship.
  - Evidence 2026-07-08: `QuePaxaPropagationFacts` and `propagation_admitted` require all quorum and subset-chain facts; proof/test coverage checks missing quorum rejection.
- [x] [serial] r[consensus.quepaxa.abstract-round] Add abstract round decision and next-candidate helpers, plus proof lemmas for validity, agreement under the modeled decision guard, and integrity via decided-state admission.
  - Evidence 2026-07-08: `abstract_decision_guard`, `abstract_decision_value`, and `abstract_next_candidate` implement decision/carry-forward behavior with validity, agreement, and integrity proofs.
- [x] [serial] r[consensus.quepaxa.concrete-phases] Add concrete phase/round admission helpers for phase arithmetic, phase-specific proposal carry-forward, monotone catch-up, and phase-2 asynchronous decision refinement.
  - Evidence 2026-07-08: `valid_phase`, `catch_up`, and `concrete_phase2_decision_refines` cover named phases, monotone catch-up, no-backward behavior, and abstract-guard refinement.
- [x] [serial] r[consensus.quepaxa.fast-path] Add first-round fast-path helpers proving a unique reserved-priority leader decision refines the abstract decision boundary, and rejecting non-leader or non-initial fast-path attempts.
  - Evidence 2026-07-08: `fast_path_decision_admitted` and `fast_path_refines_abstract` admit only initial leader/quorum/unique-reserved-priority facts and reject non-leader, non-initial, and non-unique cases.
- [x] [serial] r[consensus.quepaxa.hedging] Add hedging-schedule admission for non-decreasing activation delays, zero-delay leader placement, and enough scheduled proposers for the declared crash-fault budget, without real-time liveness claims.
  - Evidence 2026-07-08: `hedging_schedule_admitted` requires nondecreasing delays, leader-first zero-delay when configured, and proposer count greater than crash-fault budget; tests reject unordered and insufficient schedules.

## Phase 3: Tests, docs, and validation

- [x] [parallel] r[consensus.quepaxa.fixtures] Add positive tests for valid proposal ordering, ISR same-step and step-advance transitions, successful propagation relationships, phase-2 decisions, leader fast-path decisions, and valid hedging schedules.
  - Evidence 2026-07-08: `cargo test --quiet --lib quepaxa` passed positive paths for proposal ordering, ISR same-step/advance, propagation, phase-2 refinement, fast path, and hedging.
- [x] [parallel] r[consensus.quepaxa.fixtures] Add negative tests for stale ISR updates, skipped-step prior aggregate reset, tied or non-unique fast-path proposals, insufficient quorum/proposer schedules, invalid phase transitions, non-leader fast-path attempts, and overclaimed liveness/benchmark metadata.
  - Evidence 2026-07-08: focused QuePaxa tests reject stale/skipped ISR cases, missing quorum, invalid phase, non-leader/non-initial/non-unique fast path, insufficient hedging, unordered hedging, and overclaim metadata.
- [x] [serial] r[consensus.quepaxa.boundary] Sync `docs/requirements.md`, `cairn/specs/consensus/spec.md`, README/module catalog docs, and any generated module-index artifacts to the implemented QuePaxa boundary.
  - Evidence 2026-07-08: updated `docs/requirements.md`, `README.md`, `src/lib.rs`, `docs/module-index.md`, and `docs/proof-evidence-bundle.json`; accepted spec sync is run before archive.
- [x] [serial] r[consensus.quepaxa.fixtures] Run focused QuePaxa tests, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested checks, Cairn validation/gates, and the repo verification wrapper before archive.
  - Evidence 2026-07-08: `cargo test --quiet --lib quepaxa` passed 10 tests; `cargo clippy --lib -- -D warnings` passed; `./scripts/check-coverage.rs`, `./scripts/check-ensures.rs`, `tracey query uncovered`, and `tracey query untested` passed; `./scripts/verus.sh src/lib.rs` reported `10713 verified, 0 errors`; Cairn validate and proposal/design/tasks gates passed.

## Verification Coverage

| Scenario | Planned Evidence |
|---|---|
| Valid finite model boundary is visible | Requirements/docs grep evidence |
| Overclaimed liveness or benchmark metadata rejected | Negative test |
| Proposal ordering is deterministic | Positive and negative tests |
| Reserved leader priority cannot be used by non-leader fast path | Negative test |
| ISR same-step aggregation preserves first value and max aggregate | Positive test and proof lemma |
| ISR stale update is discarded | Negative test |
| ISR skipped step resets prior aggregate to nil | Negative test |
| Universal/common/existent subset relationship holds for admitted propagation | Proof lemma |
| Missing quorum propagation is rejected | Negative test |
| Phase-2 decision refines abstract decision guard | Proof lemma and positive test |
| Invalid phase transition is rejected | Negative test |
| Fast-path leader decision refines abstract decision boundary | Proof lemma and positive test |
| Hedging schedule admits leader-first non-decreasing delays | Positive test |
| Hedging schedule rejects insufficient or unordered schedules | Negative tests |
