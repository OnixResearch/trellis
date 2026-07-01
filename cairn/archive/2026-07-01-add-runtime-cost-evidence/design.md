## Context

Verus ghost code is erased by `vstd` for normal cargo builds. This is important to users, but the repository currently states it mostly as prose.

## Goals

- Add `docs/runtime-cost.md` or equivalent evidence docs with reproducible commands.
- Show representative normal-build behavior for examples or small binaries.
- Record enough environment/profile information for evidence to be interpretable.
- Avoid unstable gates based on absolute nanosecond or byte thresholds.

## Non-Goals

- Do not promise that every API is allocation-free or constant-time unless already proven by a specific module.
- Do not require nightly benchmark harnesses.
- Do not add expensive performance gates to every local verification run.

## Decisions

### Evidence shape

Evidence SHALL be command-backed. It SHOULD include the command, profile, target, artifact path pattern, and observed size or timing summary. Absolute values SHALL be descriptive evidence, not pass/fail thresholds, unless a later spec defines a named bound.

### Validation split

A lightweight checker MAY validate that evidence sections contain required fields and referenced commands. Expensive benchmark reruns SHOULD remain opt-in unless the command is fast and deterministic.

### Relationship to examples

Runnable examples SHOULD be used as the primary evidence targets once the examples change lands.
