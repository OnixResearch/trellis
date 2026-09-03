## Phase 1: Obligation and initializedness model

- [x] [serial] Define finite structural paths, type drop metadata, drop obligations, and obligation establishment/release specs. r[mir-drop.obligations]
- [x] [serial] Define initializedness transition specs for initialization, move, overwrite, and path-descendant effects. r[mir-drop.initializedness]
- [x] [serial] Define drop classification specs for `Static`, `Dead`, `Conditional`, and `Open` cases from finite maybe-init and maybe-uninit facts. r[mir-drop.classification]

## Phase 2: Elaboration decisions

- [x] [serial] Implement checked exec helpers for obligation admission, initializedness updates, and drop classification. r[mir-drop.admission]
- [x] [serial] Implement or mechanize drop-flag allocation and guarded conditional drop metadata. r[mir-drop.drop-flags]
- [x] [serial] Implement or mechanize open-drop expansion for fields and active enum variants with explicit depth/fuel budgets. r[mir-drop.open-drop]
- [x] [serial] Implement or mechanize deterministic `DropAndReplace` splitting into old-target drop behavior followed by replacement initialization. r[mir-drop.drop-and-replace]

## Phase 3: Evidence and documentation

- [x] [serial] Add positive tests for static, dead, conditional, open field, open enum, partial move, and drop-and-replace cases. r[mir-drop.evidence]
- [x] [serial] Add negative tests for malformed paths, unsupported type metadata, inconsistent initializedness facts, invalid enum active-variant metadata, and exhausted open-drop budget. r[mir-drop.evidence]
- [x] [serial] Add requirements, source annotations, test/proof annotations, `src/lib.rs` wiring, and README/module-catalog text. r[mir-drop.docs]

## Phase 4: Validation and archive

- [x] [serial] Run focused Cargo tests and Verus verification for the drop-elaboration module and capture evidence. r[mir-drop.final-validation]
- [x] [serial] Run Tracey uncovered/untested checks and Cairn validation/gates before archive. r[mir-drop.final-validation]
