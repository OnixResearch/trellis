## Context

The existing primitive taxonomy includes numeric, ML, time-series, and control
families that need finite statistical facts. The reusable proof substrate is not
"statistics is correct" in a broad sense; it is bounded finite counts,
denominator-safe ratios, non-negative weights, monotone cumulative sums, and
checked accumulator envelopes.

## Goals / Non-Goals

**Goals:**

- define exact finite models for category counts, frequencies, distributions,
  CDFs/quantiles, bounded moments, deterministic sampling admission, and interval
  metadata
- specify validity boundaries and invalid-input behavior before implementation
- keep each primitive small enough for independent Verus contracts and Tracey
  coverage
- support later composition by ML, monitoring, time-series, simulation, and
  scientific changes

**Non-Goals:**

- implement modules in this spec-only pass
- prove randomness quality, stochastic convergence, statistical inference
  validity, confidence coverage, or calibration
- model floating-point statistical accuracy, arbitrary precision, or external
  statistics libraries
- claim asymptotic complexity, allocation behavior, or performance

## Decisions

### 1. Use finite counts and rational/scaled envelopes

**Choice:** Requirements are stated over finite categories, bounded non-negative
counts/weights, denominator-safe ratios, monotone cumulative weights, and checked
accumulators.

**Rationale:** These facts compose with numeric foundation primitives and avoid
unsupported floating-point or probabilistic semantics.

**Alternative:** Specify high-level statistical APIs. Rejected because inference
validity depends on assumptions outside local primitive verification.

### 2. Separate distributions, CDFs, moments, samplers, and intervals

**Choice:** Count/frequency tables, discrete distribution admission,
CDF/quantile lookup, moment envelopes, deterministic sample admission, and
confidence-interval metadata each get separate requirements.

**Rationale:** Each has a different proof shape. Splitting them keeps later
implementation slices small and lets ML/time-series users depend only on the
needed primitive.

### 3. Deterministic sampling admission before RNG claims

**Choice:** The sampler primitive maps admitted integer thresholds or sample
indices into distribution buckets. It does not generate randomness or claim
randomness quality.

**Rationale:** Mapping a deterministic threshold into a finite cumulative
partition is verifiable; RNG quality is a separate trust/model boundary.

## Risks / Trade-offs

**Probability wording overclaiming** → Requirements use admission/envelope
language and explicitly exclude stochastic correctness and confidence coverage
claims.

**Denominator and normalization edge cases** → Frequency and distribution
requirements reject zero total weight/count unless an explicit empty result is
specified.

**Accumulator overflow** → Moment and cumulative-weight requirements use checked
bounded accumulators and reject overflow instead of wrapping.

## Validation Plan

- `openspec validate add-probability-statistics-primitives --strict --json`
- umbrella validation after taxonomy updates
- future implementation evidence: focused tests, `./scripts/verus.sh src/lib.rs`,
  Tracey uncovered/untested checks, and final repo validation before archive
