## Context

The repository contains many verified building blocks. Downstream users often need a narrower API shaped like their domain rather than direct access to low-level predicates.

## Goals

- Provide adapters that are thin shells over existing verified functional cores.
- Keep adapter inputs finite, explicit, and deterministic.
- Document exactly what each adapter proves and what it does not prove.
- Include positive and negative tests for every public adapter.

## Non-Goals

- Do not depend on external service state, network I/O, clocks, or randomness.
- Do not add heavyweight optional dependencies unless a later spec justifies them.
- Do not claim conformance to a third-party project unless tests and docs name the supported subset.

## Decisions

### Adapter contract

Each adapter SHALL expose a domain-shaped public API that delegates core decisions to existing spec/exec/proof-backed primitives. Any non-trivial adapter logic SHALL be factored into a pure core with a thin imperative shell.

### Initial domains

The first adapter set SHOULD focus on domains with clear user value and existing proof coverage: consensus admission, UCAN/web-of-trust authorization, Nix/Nickel policy admission, and storage/WAL recovery checks.

### Validation

Adapters SHALL have positive and negative tests, Tracey requirement coverage, examples where practical, and no untracked proof-gap markers.
