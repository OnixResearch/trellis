## Tasks

- [ ] [serial] Define finite reader token, delimiter, datum attachment, and span envelopes for the supported Scheme reader subset. r[scheme-logic.reader-token-span] r[scheme-logic.reader-datum-structure]
- [ ] [serial] Implement pure reader admission specs and checked exec predicates for span bounds, delimiter matching, datum-comment attachment, quote attachment, and child-span containment. r[scheme-logic.reader-token-span] r[scheme-logic.reader-datum-structure]
- [ ] [serial] Define explicit symbol and literal interner state with key uniqueness, value uniqueness, resolve/intern roundtrip, and deterministic rejection metadata for malformed tables. r[scheme-logic.interning]
- [ ] [serial] Implement module `require`/`provide` graph admission over finite modules, export sets, import edges, transitive closure metadata, duplicate export rejection, and no unprovided-name leakage. r[scheme-logic.module-graph]
- [ ] [serial] Define finite contract and blame descriptors for flat contracts, function arity, wrapper composition, and blame polarity boundaries. r[scheme-logic.contract-blame]
- [ ] [serial] Implement constant-pool and bytecode admission for deduplicated constants, rollback prefix preservation, operand bounds, stack-effect summaries, local/capture slot bounds, and call arity checks. r[scheme-logic.bytecode-safety]
- [ ] [serial] Define finite Scheme value graph predicates for pair/vector nodes, edge bounds, visited-set or fuel-bounded traversal, and cycle-safe display/equality admission. r[scheme-logic.value-graph]
- [ ] [serial] Implement Scheme-specific list, vector, and bytevector shape laws that complement existing generic collection primitives. r[scheme-logic.collections-bytevectors]
- [ ] [serial] Add positive tests for balanced reader spans, stable intern/resolve, provided module imports, contract blame polarity, bytecode safety, cycle-safe value traversal, and collection/bytevector preservation. r[scheme-logic.evidence.positive]
- [ ] [serial] Add negative tests for unmatched delimiters, invalid spans, dangling intern keys, duplicate exports, unprovided imports, arity mismatch, bytecode operand/stack failures, under-budget cyclic traversal, improper lists, and invalid bytevector elements. r[scheme-logic.evidence.negative]
- [ ] [serial] Add `docs/requirements.md`, README/module-catalog, and accepted Cairn spec updates for any public module and requirement IDs that land in source. r[scheme-logic.docs]
- [ ] [serial] Run focused Cargo tests, Verus verification for the new modules or full library as needed, Tracey uncovered/untested checks, and Cairn validation/gates; archive evidence before closing the change. r[scheme-logic.final-validation]
