## Phase 1: Attribute policy model

- [ ] [serial] Define finite attribute names, targets, argument envelopes, parser descriptors, duplicate policies, safety policies, stability gates, and diagnostic reason codes. r[hir-attr.parser-envelope] r[hir-attr.policy]
- [ ] [serial] Define parser-class specs for no-args, single, combine, and generic parser behavior. r[hir-attr.parser-classes]
- [ ] [serial] Define early and late parsed metadata plus agreement predicates. r[hir-attr.early-late-agreement]

## Phase 2: Admission helpers and proofs

- [ ] [serial] Implement checked exec helpers for parser admission, target checks, duplicate checks, safety wrapper checks, stability checks, and deterministic error metadata. r[hir-attr.admission]
- [ ] [serial] Prove helper facts that admitted attributes satisfy all declared policies and that early/late agreement preserves parsed metadata when both succeed. r[hir-attr.policy] r[hir-attr.early-late-agreement]

## Phase 3: Evidence and documentation

- [ ] [serial] Add positive tests for no-args, single, combine, generic parser descriptors, allowed targets, stable attributes, unsafe-wrapped attributes, and early/late agreement. r[hir-attr.evidence]
- [ ] [serial] Add negative tests for bad target, duplicate rejection, missing unsafe wrapper, unstable attribute without gate, malformed arguments, unknown parser, and early/late disagreement. r[hir-attr.evidence]
- [ ] [serial] Add requirements, annotations, export wiring, and README/module-catalog text. r[hir-attr.docs]

## Phase 4: Validation and archive

- [ ] [serial] Run focused Cargo tests and Verus verification for attribute parsing helpers and capture evidence. r[hir-attr.final-validation]
- [ ] [serial] Run Tracey uncovered/untested checks and Cairn validation/gates before archive. r[hir-attr.final-validation]
