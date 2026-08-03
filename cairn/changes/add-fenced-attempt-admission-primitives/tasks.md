# Tasks: Add fenced-attempt admission primitives

## Phase 1: Boundary and baseline

- [ ] [serial] V1 Record current fencing, lease, durable-job, idempotency, transition, proof-gap, Tracey, focused test, and full-library Verus baselines before source changes. r[fenced-attempt-admission.boundary] r[fenced-attempt-admission.evidence]
- [ ] [serial] I1 Add canonical `docs/requirements.md` entries, module documentation, named constants, supported variants, consumer obligations, and exact non-claims. r[fenced-attempt-admission.boundary]
- [ ] [parallel] V2 Add negative boundary fixtures for unsupported variants, unadmitted identity claims, transport/storage/crypto claims, liveness claims, and implementation-equivalence overclaims. r[fenced-attempt-admission.boundary]

## Phase 2: Model and executable refinement

- [ ] [serial] I2 Add finite phase, report, fence relation, event relation, authority, result-linkage, bounded history, state, and decision types. r[fenced-attempt-admission.refinement]
- [ ] [serial] I3 Add spec and executable functions for fence classification, event classification, phase transition, result linkage, full report application, and next-fence calculation. r[fenced-attempt-admission.refinement]
- [ ] [serial] I4 Add `ensures` clauses proving each executable decision equals its specification and changes only allowed state fields. r[fenced-attempt-admission.refinement]
- [ ] [parallel] V3 Add exhaustive finite phase/report tests plus positive accepted-transition and negative invalid-transition fixtures. r[fenced-attempt-admission.refinement] r[fenced-attempt-admission.terminal-closure]

## Phase 3: Safety proofs

- [ ] [serial] I5 Add direct proof functions for stale, future, wrong-job, and wrong-attempt rejection plus representable strict fence advance and maximum-fence exhaustion. r[fenced-attempt-admission.fence-safety]
- [ ] [serial] I6 Add direct proof functions for identical-event no-op and conflicting-event rejection. r[fenced-attempt-admission.event-idempotence]
- [ ] [serial] I7 Add direct proof functions for terminal-state closure across every report kind. r[fenced-attempt-admission.terminal-closure]
- [ ] [serial] I8 Add direct proof functions for result-ready completion, early completion rejection, missing output authority, and result identity mismatch. r[fenced-attempt-admission.completion-linkage]
- [ ] [serial] I9 Add one field-complete rejection-preservation theorem and helper lemmas for every rejection and already-applied class. r[fenced-attempt-admission.rejection-preservation]
- [ ] [parallel] V4 Add positive and negative runtime/property tests for all proof classes, including random bounded histories and mutation of one fact at a time. r[fenced-attempt-admission.fence-safety] r[fenced-attempt-admission.event-idempotence] r[fenced-attempt-admission.completion-linkage]

## Phase 4: Consumer and evidence profile

- [ ] [serial] I10 Add a product-neutral consumer mapping profile listing every required admitted fact and the fail-closed unsupported-variant rule. r[fenced-attempt-admission.boundary] r[fenced-attempt-admission.evidence]
- [ ] [serial] I11 Add the module export, README/module index, invariant catalog, property index, and proof-artifact manifest entries. r[fenced-attempt-admission.evidence]
- [ ] [parallel] V5 Export proof-evidence rows for every theorem with exact source, requirements, verifier, assumptions, trusted boundaries, tests, consumer profile, and non-claims. r[fenced-attempt-admission.evidence]

## Phase 5: Final validation and lifecycle

- [ ] [serial] V6 Run focused unit and property tests, direct `./scripts/verus.sh src/lib.rs`, ensures and coverage guards, Tracey uncovered and untested checks, proof-gap inventory, module/readme/proof-evidence drift checks, and the smallest relevant Octet/Nix checks. r[fenced-attempt-admission.evidence]
- [ ] [serial] V7 Capture non-contradictory validation transcripts, run Cairn validation plus proposal/design/tasks gates, reconcile the accepted spec and `docs/requirements.md`, verify scoped status, and archive only after every promised `proof fn` exists. r[fenced-attempt-admission.evidence]
