## Context

Time-series behavior appears across scientific and operational domains, but the
stable proof substrate is smaller than a full database or signal-processing
engine. Monotone finite timestamp indexes, aligned samples, bounded windows,
deterministic bucketization, rolling aggregates, and gap checks are reusable and
Verus-friendly.

## Goals / Non-Goals

**Goals:**

- define exact finite models for timestamped samples and bounded windows
- specify validity boundaries and invalid-input behavior before implementation
- keep each primitive small enough for independent Verus contracts and Tracey
  coverage
- support later composition by physics, ML, monitoring, scheduling, and sensor
  pipeline changes

**Non-Goals:**

- implement modules in this spec-only pass
- prove clock synchronization, timezone, leap-second, or distributed-time
  semantics
- model floating-point interpolation, stochastic forecasting, or database storage
- claim asymptotic complexity, allocation behavior, or performance

## Decisions

### 1. Use integer timestamps and finite sequences

**Choice:** Requirements are stated over integer timestamp values, finite
sequences, monotone ordering, explicit value bounds, and representable aggregate
accumulators.

**Rationale:** Integer time and finite sequences compose with numeric monotone
sequence and bounded-array primitives. This keeps proofs exact and avoids
platform clock semantics.

**Alternative:** Specify wall-clock APIs with timezone/leap-second behavior.
Rejected because those semantics are environment-specific and not needed for
first reusable proof primitives.

### 2. Separate sample alignment, windows, resampling, aggregates, and gaps

**Choice:** Timestamp indexes, aligned sample series, window selection,
resampling envelopes, rolling aggregates, and gap detection get separate
requirements.

**Rationale:** Each has a different proof shape. Splitting them keeps later
implementation slices small and independently archiveable.

### 3. Deterministic bucket envelopes before interpolation quality

**Choice:** Resampling covers timestamp-to-bucket admission and deterministic
bucket membership. It does not claim interpolation accuracy or signal quality.

**Rationale:** Bucket boundaries and no-dup/no-missing membership are verifiable;
interpolation quality needs a dedicated numeric model.

## Risks / Trade-offs

**Overlap with numeric monotone sequence** → Later implementation should reuse
numeric monotone-sequence requirements for timestamp ordering rather than
re-proving generic partition facts.

**Ambiguous window boundaries** → Requirements must state whether windows are
half-open or closed for each helper before implementation tasks are marked done.

**Aggregate overflow** → Rolling aggregate requirements use bounded accumulator
envelopes and explicit rejection instead of arbitrary-precision claims.

## Validation Plan

- `openspec validate add-timeseries-primitives --strict --json`
- umbrella validation after taxonomy updates
- future implementation evidence: focused tests, `./scripts/verus.sh src/lib.rs`,
  Tracey uncovered/untested checks, and final repo validation before archive
