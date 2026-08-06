## Phase 1: Owner and ID model

- [ ] [serial] Define finite owner identifiers, AST node identifiers, HIR identifiers, created-ID sets, used-ID sets, and source-to-HIR lowering maps. r[hir-id.owner-scope] r[hir-id.lowering-map]
- [ ] [serial] Define freshness and per-owner uniqueness specs for synthetic HIR nodes. r[hir-id.fresh-generated] r[hir-id.uniqueness]
- [ ] [serial] Define optional finite definition-correspondence metadata for AST nodes that introduce definitions. r[hir-id.def-correspondence]

## Phase 2: Admission helpers and proofs

- [ ] [serial] Implement checked exec helpers for owner-scope admission, NodeId lowering admission, generated-ID admission, created-ID usage, and duplicate detection. r[hir-id.admission]
- [ ] [serial] Prove helper facts that admitted maps never assign one source node to multiple HIR IDs in the same owner and never reuse a generated ID for a source node. r[hir-id.lowering-map] r[hir-id.fresh-generated]

## Phase 3: Evidence and documentation

- [ ] [serial] Add positive tests for valid owner-local lowering maps, generated nodes, used created IDs, and definition-correspondence metadata. r[hir-id.evidence]
- [ ] [serial] Add negative tests for wrong-owner lowering, duplicate HIR IDs, unused created IDs, unlowered source NodeIds, generated/source aliasing, and missing definition correspondence. r[hir-id.evidence]
- [ ] [serial] Add requirements, annotations, export wiring, and README/module-catalog text. r[hir-id.docs]

## Phase 4: Validation and archive

- [ ] [serial] Run focused Cargo tests and Verus verification for the HIR ID-lowering module and capture evidence. r[hir-id.final-validation]
- [ ] [serial] Run Tracey uncovered/untested checks and Cairn validation/gates before archive. r[hir-id.final-validation]
