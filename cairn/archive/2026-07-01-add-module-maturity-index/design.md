## Context

README tables document what each module verifies, and the proof-gap inventory documents trusted boundaries. These facts are separated. A module maturity index can join them into one user-facing table.

## Goals

- Define objective labels such as `runtime-helper`, `spec-model`, `composition-facade`, `example-backed`, and `trusted-boundary-adjacent`.
- Generate or validate a table with module, domain, declared status, examples, tests/proptests, requirement coverage, and proof-gap adjacency.
- Keep the scanner deterministic and independent of network or wall-clock state.
- Make stale or missing rows fail locally.

## Non-Goals

- Do not create an endorsement score.
- Do not weaken existing README module catalog checks.
- Do not require full Verus execution just to detect index drift.

## Decisions

### Objective source facts

The index SHALL derive facts from local files: `src/lib.rs`, `src/*.rs`, `docs/requirements.md`, `docs/proof-gap-sites.tsv`, README catalog text, and `examples/` when present.

### Functional core / imperative shell

Any checker SHALL split pure parsing/classification from filesystem traversal and command-line output. Magic numeric thresholds are not allowed; if limits are needed, name them.

### Checked-in output

The index MAY be generated, but the repository SHALL check in the rendered Markdown so users can browse it without running tooling.
