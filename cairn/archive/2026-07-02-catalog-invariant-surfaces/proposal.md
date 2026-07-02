## Why

Invariant evidence is spread across requirements, Verus predicates, preservation lemmas, tests, and module docs. A reviewer can find individual proofs with search, but there is no deterministic catalog that says which modules own invariant surfaces, what kind of invariant each surface represents, and what local evidence backs it.

## What Changes

- Add a deterministic invariant-surface catalog for public modules with invariant, well-formedness, preservation, and soundness predicates.
- Define objective source facts and labels so catalog rows do not become subjective maturity claims.
- Add drift checks that reject missing, stale, or overclaiming invariant rows.

## Scope

In scope: a checked-in catalog document, a deterministic checker/generator, source-fact extraction from local files, README/module-index links, and positive/negative checker fixtures.

Out of scope:
- Proving new invariants solely to improve the catalog.
- Treating tests or search hits as equivalent to Verus proof evidence.
- Replacing Tracey coverage or the module maturity index.

## Verification

Run the catalog checker self-tests, catalog drift check, README/module-index checks, Tracey uncovered/untested queries when requirement IDs are added, and the smallest cargo/clippy/Verus checks needed for touched scripts or source files.
