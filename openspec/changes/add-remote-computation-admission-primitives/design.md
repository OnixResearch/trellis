## Context

Unison remote execution depends on content-addressed dependency availability plus ability/resource authorization. This change models only the deterministic finite admission predicate for remote execution eligibility.

## Goals / Non-Goals

**Goals:**
- Compose computation identity, dependency availability, ability coverage, and resource bounds.
- Separate ready, fetch-pending, and rejected dependency states.
- Provide a later composition point for content-addressed code and ability-effect primitives.

**Non-Goals:**
- Real network transfer, sandbox execution, authentication, or scheduler behavior.
- Code safety beyond finite ability/resource metadata.
- Storage, durable values, or Cloud API behavior.

## Decisions

### 1. Eligibility, not execution

**Choice:** The public predicate admits only metadata-level execution eligibility.

**Rationale:** Real remote execution has runtime and security boundaries that should remain out of scope.

### 2. Dependency state is tri-valued

**Choice:** Distinguish ready, fetch-pending, and rejected dependency availability.

**Rationale:** This mirrors Unison's missing-dependency sync story while keeping fetch as a separate state transition.

### 3. Ability and resource gates are conjunctive

**Choice:** Remote eligibility requires both ability coverage and resource-bound fit.

**Rationale:** This prevents a successful dependency closure from implying permission to run.

## Risks / Trade-offs

**Security overclaiming** → Do not claim authentication, code safety, or sandbox implementation correctness.

**Composition sequencing** → Keep dependency/ability inputs opaque enough to implement before the other primitives land, then add composition lemmas later.

**Resource modeling churn** → Use small finite bounds and checked comparisons, not performance or metering semantics.
