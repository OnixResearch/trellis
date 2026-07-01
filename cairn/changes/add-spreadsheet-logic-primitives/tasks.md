## Phase 1: Address and range envelopes

- [ ] [serial] Add finite spreadsheet sheet-bound and cell-address admission helpers with positive and negative tests for in-bounds and out-of-bounds coordinates. r[domain-primitives.spreadsheet.address]
- [ ] [serial] Add rectangular range normalization, finite membership, and range-expansion helpers with positive and negative tests for reversed endpoints and invalid endpoints. r[domain-primitives.spreadsheet.range]

## Phase 2: Formula and dependency model

- [ ] [serial] Add exact spreadsheet value and formula syntax predicates for constants, references, ranges, arithmetic, and basic aggregates. r[domain-primitives.spreadsheet.formula]
- [ ] [serial] Add dependency extraction helpers that return exactly the finite precedent coordinates named by admitted formulas and ranges, including tests for nested formulas and rejected references. r[domain-primitives.spreadsheet.dependency]

## Phase 3: Cycle rejection and recalculation

- [ ] [serial] Add cycle-admission predicates that reject circular formula graphs before evaluation and prove admitted formula sets are acyclic. r[domain-primitives.spreadsheet.cycle]
- [ ] [serial] Add topological recalculation-order predicates and tests proving formulas are evaluated only after their precedents. r[domain-primitives.spreadsheet.recalc]
- [ ] [serial] Add dirty-cell propagation helpers that include changed cells and reachable dependents while excluding unrelated formula cells. r[domain-primitives.spreadsheet.dirty-propagation]

## Phase 4: Evaluation errors and integration

- [ ] [serial] Add deterministic spreadsheet error envelopes for bad references, division by zero, value-shape mismatches, and cycle rejection, with positive and negative tests for error propagation. r[domain-primitives.spreadsheet.error]
- [ ] [serial] Sync `docs/requirements.md`, `cairn/specs/domain-primitives/spec.md`, legacy OpenSpec specs if required, README, and crate docs for the implemented spreadsheet modules. r[domain-primitives.spreadsheet.admission]
- [ ] [serial] Run focused spreadsheet tests, Verus over `src/lib.rs`, Tracey uncovered/untested, Cairn validation/gates, cargo/clippy checks as needed, and archive evidence before completion. r[domain-primitives.spreadsheet.admission]
