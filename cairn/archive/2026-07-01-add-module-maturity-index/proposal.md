## Why

A large verified library needs more than a module list. Users need to know whether a module is a runtime helper, model-only surface, composition facade, example-backed API, or near a trusted boundary before relying on it.

## What Changes

- Add a deterministic module maturity index, checked into docs.
- Define stable maturity labels and the source facts used to assign them.
- Add drift checking so the index stays aligned with `src/lib.rs`, tests, examples, and proof-gap inventory.

## Scope

In scope: generated or manually refreshed `docs/module-index.md`, source scanners/checkers, maturity label definitions, and README/module-guide links.

Out of scope:
- Changing module APIs solely to improve labels.
- Hiding trusted boundaries behind marketing language.
- Making subjective quality rankings.

## Verification

Run the maturity-index checker, README/catalog checks, proof-gap inventory checks, and standard repository verification gates for touched scripts/docs.
