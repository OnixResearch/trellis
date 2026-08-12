## Phase 1: Traversal model

- [ ] [serial] Define finite MIR visit locations, visitor trace entries, traversal modes, and mutation descriptors over admitted MIR-core bodies. r[mir-visitor.locations] r[mir-visitor.trace]
- [ ] [serial] Define pure preorder and reverse-postorder traversal specs over admitted CFG edges and entry metadata. r[mir-visitor.traversal-order]
- [ ] [serial] Define immutable coverage and mutable shape-preservation specs. r[mir-visitor.coverage] r[mir-visitor.mut-shape]

## Phase 2: Admission helpers and proofs

- [ ] [serial] Implement checked exec helpers for visitor-trace admission, duplicate detection, location validation, traversal-order validation, and mutation-shape validation. r[mir-visitor.admission]
- [ ] [serial] Prove helper facts that admitted exactly-once traversals cover all requested MIR locations and no invalid location. r[mir-visitor.coverage]
- [ ] [serial] Prove helper facts that admitted mutable traces preserve declared body/block/local shape invariants. r[mir-visitor.mut-shape]

## Phase 3: Evidence and documentation

- [ ] [serial] Add positive tests for immutable full-body traversal, place/operand traversal, preorder block traversal, reverse-postorder block traversal, and shape-preserving mutation traces. r[mir-visitor.evidence]
- [ ] [serial] Add negative tests for missing visits, duplicate visits, invalid locations, unreachable-policy mismatch, and mutation that violates declared shape invariants. r[mir-visitor.evidence]
- [ ] [serial] Add requirements, annotations, export wiring, and README/module-catalog text. r[mir-visitor.docs]

## Phase 4: Validation and archive

- [ ] [serial] Run focused Cargo tests and Verus verification for visitor/traversal helpers and capture evidence. r[mir-visitor.final-validation]
- [ ] [serial] Run Tracey uncovered/untested checks and Cairn validation/gates before archive. r[mir-visitor.final-validation]
