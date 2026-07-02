## Context

Trellis already has many verified modules and a concise README example. The missing adoption surface is a maintained set of runnable examples with commands and expected outcomes. Users should be able to see a correct proof pass, a malformed proof fail, and a runtime test exercise the executable surface without needing to understand every large domain module.

This change should stay small and educational. It should compose existing proof patterns rather than introduce broad new APIs.

## Goals

- Provide copyable examples for Trellis's spec/exec/proof pattern.
- Pair every positive proof example with at least one negative example or fixture that demonstrates fail-closed behavior.
- Make commands deterministic and local, with no network, clock, randomness, or external-service dependency.
- Keep examples aligned with Tracey requirements and module maturity indexing.

## Non-Goals

- Do not use examples to bypass the normal `src/` proof standards.
- Do not add expensive examples to every default developer command unless they are fast and stable.
- Do not encode broad performance or release-readiness claims; those belong to runtime-cost evidence and downstream gates.
- Do not add trusted bodies or `assume(...)` surfaces to make examples pass.

## Decisions

### Example shape

Each example SHOULD name a requirement ID, include a small mathematical spec, a runtime implementation, and at least one Verus-checked `ensures` or proof function. Runtime examples SHOULD also include positive and negative tests through public APIs when runtime behavior exists.

### Negative examples

Negative examples SHALL be isolated from normal passing verification. They MAY live under a fixture directory or be invoked through a checker that expects failure. The expected failure mode SHOULD be documented so users can tell a successful negative check from an accidental pass.

### Command contract

The example docs SHALL name the exact commands for passing examples and expected-failure examples. If a command is intentionally opt-in, the docs SHALL state that it is not part of the default fast path.

### Integration with active changes

The examples SHOULD feed the module maturity index once `add-module-maturity-index` lands and SHOULD provide representative targets for runtime-cost evidence once `add-runtime-cost-evidence` lands. This change does not depend on those packages completing first.

## Risks / Trade-offs

- Negative fixtures can confuse contributors if they are included in normal builds. Mitigate by isolating them behind explicit expected-failure commands.
- Examples can become stale when module APIs change. Mitigate with checker-backed docs and Tracey references.
- Too many examples can slow verification. Mitigate with a small curated initial set and opt-in heavyweight runs.

## Validation

- Positive example command passes and records which files were checked.
- Negative example command fails for the expected reason and records the expected diagnostic shape.
- Cargo tests and clippy pass for example-facing runtime code.
- Tracey reports no uncovered or untested requirement IDs introduced by the examples.
- Verus verification passes without new trusted/proof-gap boundaries.
- Cairn validation and gates pass for this package.
