## Phase 1: Phase and artifact model

- [x] [serial] Define finite MIR phase identifiers, phase-order predicates, artifact states, pass descriptors, and optimization-level metadata. r[mir-pass.phase-order] r[mir-pass.artifact-state]
- [x] [serial] Define pass compatibility specs for input phase, output phase, same-phase mutation, required validation, and optimization-level gates. r[mir-pass.pass-compatibility] r[mir-pass.optimization-gating]
- [x] [serial] Define read/borrow/steal transition specs for intermediate MIR artifacts. r[mir-pass.steal-order]

## Phase 2: Pipeline admission

- [x] [serial] Implement checked exec helpers for pipeline-trace admission, pass admission, artifact-state transition, and phase-order validation. r[mir-pass.admission]
- [x] [serial] Prove helper facts that admitted traces never read stolen artifacts and never run a pass outside its declared phase/gate. r[mir-pass.steal-order] r[mir-pass.pass-compatibility]
- [x] [serial] Define deterministic rejection metadata for phase, pass, validation, optimization gate, and steal-order failures. r[mir-pass.failure-metadata]

## Phase 3: Evidence and documentation

- [x] [serial] Add positive tests for valid query pipeline traces, same-phase mutating passes, validation passes, optimization-gated passes, and borrow-before-steal sequences. r[mir-pass.evidence]
- [x] [serial] Add negative tests for out-of-order phases, invalid pass registration, missing validation, disabled optimization pass execution, read-after-steal, and double-steal attempts. r[mir-pass.evidence]
- [x] [serial] Add requirements, annotations, export wiring, and README/module-catalog text. r[mir-pass.docs]

## Phase 4: Validation and archive

- [x] [serial] Run focused Cargo tests and Verus verification for the pass-pipeline module and capture evidence. r[mir-pass.final-validation]
- [x] [serial] Run Tracey uncovered/untested checks and Cairn validation/gates before archive. r[mir-pass.final-validation]
