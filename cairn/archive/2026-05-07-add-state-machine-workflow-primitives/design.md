## Context

State-machine and workflow reasoning is broadly reusable, but proof-friendly
primitives should stay exact and finite: admitted states, transition relations,
events, guards, workflow steps, dependencies, reachability envelopes, terminal
predicates, and traces. Runtime orchestration systems are deliberately outside
this change.

## Goals / Non-Goals

**Goals:**

- define reusable finite state-machine/workflow primitive requirements
- make validity boundaries explicit for states, transitions, events, guards,
  workflow steps, dependencies, paths, terminals, traces, and admission decisions
- keep future implementation slices small enough for Verus/Tracey evidence

**Non-Goals:**

- proving runtime workflow-engine, queue, worker, scheduler, or orchestrator behavior
- proving liveness, fairness, distributed consensus, exactly-once execution, or wall-clock guarantees
- proving business-process completeness, saga correctness, or external system behavior
- adding implementation in this spec-only change

## Decisions

### 1. Use exact finite transition models

**Choice:** States, transitions, events, guards, and traces are finite admitted
sets/sequences with deterministic predicates.

**Rationale:** This keeps the family reusable and Verus-friendly while avoiding
runtime orchestration semantics.

**Alternative:** Model a complete workflow runtime. Rejected because engine,
worker, queue, clock, retry, and storage behavior are separate systems.

### 2. Separate state-machine facts from workflow metadata

**Choice:** State/transition/event/guard/admission/reachability/terminal/trace
requirements cover the state-machine core; step/dependency requirements cover
workflow shape metadata.

**Rationale:** State transition proofs and workflow dependency proofs have
different proof shapes and can be implemented independently.

### 3. Admission is deterministic and local

**Choice:** Admission primitives reject undeclared states, invalid transitions,
unknown events, failed guards, cyclic dependencies where disallowed, or malformed
traces with explicit metadata.

**Rationale:** Deterministic local admission is useful for public APIs without
claiming runtime execution, scheduling, or delivery behavior.

## Risks / Trade-offs

**Over-broad orchestration claims** → Requirements explicitly exclude runtime
workflow engines, distributed orchestration, queues, wall-clock scheduling,
liveness/fairness, exactly-once execution, retries, storage durability,
business-process completeness, saga correctness, and external systems.

**Overlap with graph primitives** → Graph primitives own generic adjacency,
reachability, components, shortest paths, and frontier/cut facts. This change
owns state-labeled transition systems and workflow-shaped transition metadata.

**Overlap with scheduling/calendar** → Scheduling/calendar owns time buckets,
recurrence, deadlines, intervals, and resource slots. This change may reference
ordered workflow steps but does not claim wall-clock or scheduler behavior.

**Tracey drift** → Future implementation slices must add `docs/requirements.md`,
`r[impl]`, `r[depends]`, and `r[verify]` tags in the same commit as code.

## Validation Plan

- `openspec validate add-state-machine-workflow-primitives --strict --json`
- `git diff --check`
- helper verification may report incomplete tasks until implementation lands
- future implementation slices run focused cargo tests, Verus, Tracey, and final
  repo validation before archive
