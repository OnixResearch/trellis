## Context

The repo already documents requirements and verification policy, but cookbook-style guidance is different: it gives short recipes with expected inputs, commands, outputs, and failure modes.

## Goals

- Add recipes for common tasks: use a public helper, add a verified primitive, write tests/proptests, run Verus for one module, update Tracey annotations, and handle proof-gap boundaries.
- Show both happy-path and sad-path examples.
- Keep snippets small enough to remain maintainable.
- Link to authoritative requirements, scripts, and examples instead of copying long catalogs.

## Non-Goals

- Do not teach all of Rust, Verus, or SMT solving.
- Do not make cookbook commands normative unless they are wired into validation.
- Do not introduce shell/Python helper scripts when a repository Rust checker or existing script is the right validation surface.

## Decisions

### Recipe structure

Each recipe SHALL state the goal, files touched, minimal commands, success criteria, and at least one common failure case.

### Validation

Executable snippets SHOULD either be checked directly or clearly marked as illustrative. Any newly introduced snippet checker SHALL use a pure parsing core and a thin filesystem shell.

### Relationship to requirements

The cookbook SHALL not replace `docs/requirements.md`; it SHALL point readers to requirement IDs when a recipe changes traceability.
