## Phase 1: Framework model

- [ ] [serial] Define finite dataflow domains, directions, block/location identifiers, analysis descriptors, and named convergence-budget metadata. r[mir-dataflow.domain] r[mir-dataflow.direction]
- [ ] [serial] Define semilattice-law admission for join, bottom, top, and finite-height metadata. r[mir-dataflow.lattice-laws]
- [ ] [serial] Define statement, terminator, before, primary, and per-edge effect specs with validity checks over admitted MIR-core locations and successor edges. r[mir-dataflow.effects] r[mir-dataflow.per-edge]

## Phase 2: Fixpoint and result inspection

- [ ] [serial] Implement checked exec admission helpers for finite analyses and transfer/effect tables. r[mir-dataflow.admission]
- [ ] [serial] Prove monotone finite fixed-point convergence under the declared budget and reject non-monotone or ill-formed analyses. r[mir-dataflow.convergence]
- [ ] [serial] Define pure results cursor/visitor inspection specs for entry, before-effect, primary-effect, and exit states. r[mir-dataflow.results]

## Phase 3: Evidence and documentation

- [ ] [serial] Add positive tests for admitted finite domains, forward analysis, backward analysis, before effects, per-edge effects, and fixed-point result inspection. r[mir-dataflow.evidence]
- [ ] [serial] Add negative tests for missing bottom/top, bad lattice laws, non-monotone transfer, invalid edge effect, invalid location, and exhausted convergence budget. r[mir-dataflow.evidence]
- [ ] [serial] Add `docs/requirements.md`, source annotations, test/proof annotations, `src/lib.rs` wiring, and README/module-catalog text. r[mir-dataflow.docs]

## Phase 4: Validation and archive

- [ ] [serial] Run focused Cargo tests and Verus verification for the dataflow framework and capture evidence. r[mir-dataflow.final-validation]
- [ ] [serial] Run Tracey uncovered/untested checks and Cairn validation/gates, then archive only after evidence matches the current tree. r[mir-dataflow.final-validation]
