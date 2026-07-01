## Why

Trellis has finite table/relational metadata and graph/topological-order primitives, but it does not yet model the spreadsheet correctness edges that cause real spreadsheet engines to fail: out-of-bounds references, range normalization drift, formula dependency extraction errors, circular references, stale recalculation, and nondeterministic error propagation.

This change adds a compact spreadsheet-logic primitive slice that stays finite, deterministic, and verification-friendly. It covers address and range admission, a small exact formula/value model, dependency extraction, cycle rejection, topological recalculation order, dirty-cell propagation, and explicit spreadsheet error envelopes without claiming a full workbook runtime or floating-point compatibility with any existing spreadsheet product.

## What Changes

- Add spreadsheet cell-address and rectangular-range envelopes over finite sheet bounds.
- Add a small formula/value model for constants, references, ranges, arithmetic, and basic aggregates over exact values.
- Add dependency extraction that expands admitted ranges into finite precedent sets.
- Add cycle admission checks that reject circular formulas before evaluation.
- Add recalculation-order and dirty-propagation predicates that compose with dependency graphs.
- Add deterministic error propagation for malformed references, division by zero, value-shape mismatches, and cycles.
- Defer copy/fill relative-reference rewriting, dynamic-array spill behavior, formatting, floating-point compatibility, and external workbook I/O to later changes.

## Impact

- **Files**: expected source modules such as `src/spreadsheet_address.rs`, `src/spreadsheet_formula.rs`, `src/spreadsheet_dependency.rs`, `src/spreadsheet_recalc.rs`, `docs/requirements.md`, `cairn/specs/domain-primitives/spec.md`, legacy OpenSpec sync if still required, README, crate docs, and this Cairn package.
- **APIs**: new finite spreadsheet helper surfaces rather than a full spreadsheet engine.
- **Testing**: positive and negative unit/property tests for admitted references, rejected references, range expansion, formula dependency extraction, cycle rejection, topological recalc, dirty propagation, and error determinism; Verus proof checks; Tracey coverage; Cairn validation/gates.
