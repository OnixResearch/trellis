## Context

Control-system behavior can be made useful without modeling continuous dynamics:
most safety-facing wrappers first need bounded setpoints, bounded error signals,
clamping, deadband decisions, hysteresis transitions, rate limits, and envelope
guards. These are exact finite-state or bounded-arithmetic facts that compose
with numeric, time-series, and physics primitives.

## Goals / Non-Goals

**Goals:**

- define exact discrete models for bounded control decisions
- specify validity boundaries and invalid-input behavior before implementation
- keep each primitive small enough for independent Verus contracts and Tracey
  coverage
- support later composition by physics, robotics, resource-governor, scheduler,
  and automation changes

**Non-Goals:**

- prove continuous-control stability, convergence, or physical safety
- model real-time scheduling, hardware actuator behavior, or plant dynamics
- claim floating-point accuracy, controller tuning quality, or performance

## Decisions

### 1. Use bounded discrete envelopes instead of continuous dynamics

**Choice:** Requirements are stated over bounded integer/scaled values, finite
modes, explicit thresholds, and conservative envelope predicates.

**Rationale:** These facts are Verus-friendly and immediately useful as guards
around domain-specific control logic. Continuous dynamics need separate models.

**Alternative:** Specify a PID/controller framework. Rejected because tuning,
stability, and plant assumptions would make the first OpenSpec too broad.

### 2. Separate guard, transform, and mode-transition primitives

**Choice:** Setpoint admission, error derivation, saturation, deadband,
hysteresis, rate limiting, and safety envelopes get separate requirements.

**Rationale:** Each primitive has a compact proof shape and can be implemented or
reused independently by later changes.

### 3. Deterministic decisions before safety claims

**Choice:** The primitives specify deterministic accept/reject, clamp/no-action,
mode-transition, and envelope-guard behavior. They do not claim a controlled
system is stable or physically safe.

**Rationale:** Deterministic local guards are verifiable. System-level safety
requires plant, timing, and composition assumptions outside this package.

## Risks / Trade-offs

**False sense of safety** → Requirement wording explicitly limits guarantees to
local bounded decisions and rejects real-world safety/stability claims.

**Threshold ambiguity** → Later implementation must state inclusive/exclusive
threshold semantics for deadbands and hysteresis before tasks are complete.

**Overflow in error/rate calculations** → Error and rate primitives use checked
arithmetic envelopes and reject overflowing deltas instead of wrapping.

## Validation Plan

- `openspec validate add-control-system-primitives --strict --json`
- umbrella validation after taxonomy updates
- focused tests, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested
  checks, and final repo validation before archive
