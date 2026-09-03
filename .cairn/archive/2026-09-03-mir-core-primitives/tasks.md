## Phase 1: Core MIR envelope

- [x] [serial] Define finite MIR body, basic-block, statement, terminator, local-declaration, place, projection, operand, and rvalue types with named constants for distinguished roles. r[mir-core.body-shape] r[mir-core.return-place]
- [x] [serial] Add pure admission specs for body shape, block shape, local references, place/projection chains, operand use, rvalue shape, statement shape, and terminator successor ranges. r[mir-core.block-terminator] r[mir-core.local-reference] r[mir-core.place-projection] r[mir-core.successors]
- [x] [serial] Implement checked exec admission helpers whose results refine the pure specs and return deterministic failure metadata. r[mir-core.failure-metadata]

## Phase 2: MIR invariants and adapters

- [x] [serial] Prove helper lemmas that admitted terminators induce only in-range CFG edges and that admitted blocks contain exactly one final terminator. r[mir-core.successors] r[mir-core.block-terminator]
- [x] [serial] Prove or mechanize that admitted rvalues are non-nested and reference only admitted operands or places. r[mir-core.rvalue-nonnesting]
- [x] [serial] Add minimal adapter specs for downstream analyses to consume block-successor and local-width metadata without depending on rustc internals. r[mir-core.downstream-adapter]

## Phase 3: Evidence and documentation

- [x] [serial] Add positive tests for single-block return bodies, multi-block branches, local and temporary references, place projections, operands, and rvalues. r[mir-core.evidence]
- [x] [serial] Add negative tests for missing return place, empty block table, missing terminator, invalid local index, dangling projection base, malformed rvalue, and out-of-range successor. r[mir-core.evidence]
- [x] [serial] Add `docs/requirements.md` entries, `r[impl ...]` source annotations, `r[verify ...]` test/proof annotations, `src/lib.rs` export wiring, and README/module-catalog text. r[mir-core.docs]

## Phase 4: Validation and archive

- [x] [serial] Run focused Cargo tests for the MIR core module and any public adapters, and capture command/output evidence. r[mir-core.final-validation]
- [x] [serial] Run Verus verification for the narrowest sound command covering the MIR core module or `src/lib.rs`, and capture command/output evidence. r[mir-core.final-validation]
- [x] [serial] Run Tracey uncovered and untested checks, reconcile missing links, then run Cairn validation and gates before archive. r[mir-core.final-validation]
