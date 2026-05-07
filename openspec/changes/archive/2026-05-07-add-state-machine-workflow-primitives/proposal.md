## Why

`verified-logic` frequently needs reusable reasoning about finite states,
transitions, events, guards, and workflow-like steps, but those concepts should
not be mixed with runtime orchestration engines or distributed liveness claims. A
focused OpenSpec lets future slices add exact finite state-machine/workflow
primitives with small proof obligations.

This change is spec-only. It defines the reusable state-machine/workflow family
and leaves implementation, proof evidence, Tracey coverage, and archive work to
later small slices.

## What Changes

- Add finite state-machine/workflow primitive requirements for:
  - state-set and initial/terminal-state admission
  - transition relation envelopes
  - event and guard metadata
  - deterministic transition admission/rejection
  - workflow step and dependency envelopes
  - finite reachability/path envelopes
  - terminal-state predicates
  - transition trace envelopes
- Require exact finite models, deterministic validity boundaries, and explicit
  non-goals for runtime workflow-engine or distributed orchestration claims.

## Capabilities

### New Capabilities

- `domain-primitives.state-machine-workflow.state`: finite state-set and initial/terminal-state admission.
- `domain-primitives.state-machine-workflow.transition`: finite transition relation envelopes.
- `domain-primitives.state-machine-workflow.event`: finite event metadata.
- `domain-primitives.state-machine-workflow.guard`: deterministic guard predicate envelopes.
- `domain-primitives.state-machine-workflow.admission`: deterministic transition admission/rejection.
- `domain-primitives.state-machine-workflow.step`: finite workflow step metadata.
- `domain-primitives.state-machine-workflow.dependency`: finite workflow dependency envelopes.
- `domain-primitives.state-machine-workflow.reachability`: bounded reachability/path envelopes.
- `domain-primitives.state-machine-workflow.terminal`: terminal-state predicate envelopes.
- `domain-primitives.state-machine-workflow.trace`: finite transition trace envelopes.

## Impact

- **Files**: OpenSpec artifacts now; future implementation may add
  `src/state_*`, `src/workflow_*`, `src/lib.rs`, `docs/requirements.md`, tests,
  and evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new runtime dependencies.
- **Testing**: strict OpenSpec validation now; future slices must add focused
  tests, Verus proof evidence, Tracey coverage, and final repo validation.

## Out of Scope

- implementing state-machine/workflow modules in this change
- runtime workflow-engine semantics, distributed orchestration, queue/worker
  behavior, wall-clock scheduling, liveness/fairness, exactly-once execution,
  retry backoff quality, external storage durability, business-process
  completeness, saga/transaction correctness, or external system correctness
- probabilistic scheduling or workload-quality claims
