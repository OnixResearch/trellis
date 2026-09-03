## Phase 1: Attribute policy model

- [x] [serial] Define finite attribute names, targets, argument envelopes, parser descriptors, duplicate policies, safety policies, stability gates, and diagnostic reason codes. r[hir-attr.parser-envelope] r[hir-attr.policy]
- [x] [serial] Define parser-class specs for no-args, single, combine, and generic parser behavior. r[hir-attr.parser-classes]
- [x] [serial] Define early and late parsed metadata plus agreement predicates. r[hir-attr.early-late-agreement]

## Phase 2: Admission helpers and proofs

- [x] [serial] Implement checked exec helpers for parser admission, target checks, duplicate checks, safety wrapper checks, stability checks, and deterministic error metadata. r[hir-attr.admission]
- [x] [serial] Prove helper facts that admitted attributes satisfy all declared policies and that early/late agreement preserves parsed metadata when both succeed. r[hir-attr.policy] r[hir-attr.early-late-agreement]

## Phase 3: Evidence and documentation

- [x] [serial] Add positive tests for no-args, single, combine, generic parser descriptors, allowed targets, stable attributes, unsafe-wrapped attributes, and early/late agreement. r[hir-attr.evidence]
- [x] [serial] Add negative tests for bad target, duplicate rejection, missing unsafe wrapper, unstable attribute without gate, malformed arguments, unknown parser, and early/late disagreement. r[hir-attr.evidence]
- [x] [serial] Add requirements, annotations, export wiring, and README/module-catalog text. r[hir-attr.docs]

## Phase 4: Validation and archive

- [x] [serial] Run focused Cargo tests and Verus verification for attribute parsing helpers and capture evidence. r[hir-attr.final-validation]
- [x] [serial] Run Tracey uncovered/untested checks and Cairn validation/gates before archive. r[hir-attr.final-validation]
