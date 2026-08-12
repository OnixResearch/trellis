## Context

The module maturity index exposes broad source facts, but invariant evidence has a different review shape: predicates, preservation lemmas, exec checkers, negative cases, and trusted-boundary adjacency need to be visible together.

## Goals

- Classify invariant surfaces using objective local facts.
- Keep each catalog row traceable to a requirement ID, source predicate, evidence kind, and validation status.
- Distinguish Verus proof evidence from unit/property tests and prose-only claims.
- Fail locally when public invariant surfaces drift without catalog updates.

## Non-Goals

- Do not create a subjective proof-quality score.
- Do not require full Verus execution to detect basic catalog drift.
- Do not hide trusted boundaries or downgrade them to ordinary invariants.

## Decisions

### Source facts

The catalog SHALL derive facts from local files only: `docs/requirements.md`, `cairn/specs/**/spec.md`, `src/lib.rs`, `src/**/*.rs`, `docs/module-index.md`, and the proof-gap inventory. Network access, wall-clock time, and external benchmarks are not inputs.

### Labels

Rows SHALL use stable labels such as `state-invariant`, `well-formedness`, `preservation`, `soundness`, `admission-boundary`, and `catalog-only`. A row MAY carry more than one label when the source facts justify each label.

### Evidence separation

The checker SHALL render proof, test, Tracey, and trusted-boundary facts as separate columns. A row with tests but no proof MUST NOT be described as proven unless a matching Verus proof marker or proof artifact is present.

### Functional core / imperative shell

Parsing, classification, and rendering SHALL be pure over in-memory text inputs. The command-line shell owns filesystem traversal, file reads, writes, diagnostics, and process exits.
