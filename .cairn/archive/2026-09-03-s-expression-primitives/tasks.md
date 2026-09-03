## Phase 1: Core model and canonicalization

- [x] [serial] Define the finite S-expression AST, atom envelope, list envelope, size measure, depth measure, and well-formedness specs. r[s-expression.ast] r[s-expression.budgets]
- [x] [serial] Implement checked exec predicates for atom admission, list admission, size/depth budget admission, and structural well-formedness. r[s-expression.atom-admission] r[s-expression.budgets]
- [x] [serial] Define canonical serialization specs for atoms, lists, separators, delimiter placement, and escape handling. r[s-expression.canonical-serialization]
- [x] [serial] Implement checked canonical serialization helpers with postconditions tying output shape to the canonical serialization spec. r[s-expression.canonical-serialization]

## Phase 2: Bounded parsing and roundtrip facts

- [x] [serial] Define deterministic parse/admission result metadata for valid values and failure classes. r[s-expression.parse-admission]
- [x] [serial] Implement bounded parser/admission helpers for the canonical grammar, including explicit rejection for invalid atoms, bad escapes, unbalanced delimiters, trailing input, and size/depth budget exhaustion. r[s-expression.parse-admission] r[s-expression.budgets]
- [x] [serial] Prove or mechanize the canonical roundtrip property for admitted finite values: parsing canonical serialization yields the original S-expression. r[s-expression.roundtrip]
- [x] [serial] Add local proof lemmas for serialization injectivity boundaries that are needed by the roundtrip proof, without claiming compatibility with external dialects. r[s-expression.roundtrip] r[s-expression.non-claims]

## Phase 3: Evidence and documentation

- [x] [serial] Add positive tests for admitted atoms, nested lists, empty lists, escaped atoms, canonical serialization shape, budget-admitted values, and canonical parse/serialize roundtrips. r[s-expression.evidence.positive]
- [x] [serial] Add negative tests for invalid atoms, unterminated escapes, non-canonical spacing/escaping if rejected by the chosen surface, unbalanced delimiters, trailing input, malformed nesting, size-budget rejection, and depth-budget rejection. r[s-expression.evidence.negative]
- [x] [serial] Add `docs/requirements.md` entries, `r[impl ...]` source annotations, `r[verify ...]` test annotations, `src/lib.rs` export wiring, and README/module-catalog text for the public module. r[s-expression.docs]
- [x] [serial] Sync the accepted Cairn spec after implementation lands and preserve the non-claim boundary in docs and evidence. r[s-expression.docs] r[s-expression.non-claims]

## Phase 4: Validation and archive

- [x] [serial] Run focused Cargo tests for the S-expression module plus any affected public exports, and capture command/output evidence. r[s-expression.final-validation]
- [x] [serial] Run Verus verification for `src/lib.rs` or the narrowest sound command that covers the S-expression module, and capture command/output evidence. r[s-expression.final-validation]
- [x] [serial] Run Tracey uncovered and untested checks, and reconcile any missing requirement/source/test links before archive. r[s-expression.final-validation]
- [x] [serial] Run Cairn validation and proposal/design/tasks gates for this change, then archive only after fresh evidence matches the current tree. r[s-expression.final-validation]
