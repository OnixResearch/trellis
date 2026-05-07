## Why

ML, monitoring, simulation, control, and scientific workflows need finite
probability/statistics primitives: count tables, normalized frequencies,
discrete distributions, CDF/quantile envelopes, bounded moments, and deterministic
sampling admission. These should be specified as exact finite arithmetic helpers
instead of broad probabilistic or model-quality claims.

This change adds a focused spec-only probability/statistics primitive package. It
uses finite counts, rational/scaled weights, bounded accumulators, deterministic
ordering, and explicit admission/rejection semantics first. It avoids stochastic
correctness, randomness quality, floating-point statistical accuracy, and
inference-validity claims.

## What Changes

- Add a focused `add-probability-statistics-primitives` OpenSpec for reusable
  finite statistics modules:
  - `stats_count_table`: category/count admission and bounded count updates.
  - `stats_frequency_table`: denominator-safe frequency derivation.
  - `stats_discrete_distribution`: finite non-negative weights and normalization envelopes.
  - `stats_cdf_quantile`: monotone cumulative weights and deterministic quantile lookup.
  - `stats_bounded_moments`: bounded count/sum/mean/variance-envelope derivation.
  - `stats_deterministic_sampler`: admitted sample-index/threshold mapping without RNG-quality claims.
  - `stats_confidence_interval_envelope`: bounded interval metadata admission without coverage claims.
- Require public exec surfaces to reject invalid categories, negative weights,
  zero denominators, non-monotone CDFs, overflowing accumulators, and malformed
  probability intervals, or return explicit empty/error decisions.
- Keep implementation, evidence, and canonical spec sync as future tasks.

## Capabilities

### New Capabilities

- `domain-primitives.stats-count-table`: bounded category count primitive.
- `domain-primitives.stats-frequency-table`: denominator-safe frequency primitive.
- `domain-primitives.stats-discrete-distribution`: finite weighted distribution admission primitive.
- `domain-primitives.stats-cdf-quantile`: cumulative distribution / quantile lookup primitive.
- `domain-primitives.stats-bounded-moments`: bounded moment-envelope primitive.
- `domain-primitives.stats-deterministic-sampler`: deterministic sample admission primitive.
- `domain-primitives.stats-confidence-interval-envelope`: bounded interval metadata primitive.

## Impact

- **Files**: OpenSpec artifacts now; future implementation may add
  `src/stats_count_table.rs`, `src/stats_frequency_table.rs`,
  `src/stats_discrete_distribution.rs`, `src/stats_cdf_quantile.rs`,
  `src/stats_bounded_moments.rs`, `src/stats_deterministic_sampler.rs`,
  `src/stats_confidence_interval_envelope.rs`, `src/lib.rs`,
  `docs/requirements.md`, tests, and evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new runtime dependencies.
- **Testing**: this change validates with OpenSpec now; implementation tasks must
  later run focused tests, Verus, Tracey, and repo validation.

## Out of Scope

- implementing statistics modules in this change
- random-number-generator quality, stochastic convergence, statistical inference
  validity, confidence-interval coverage probability, calibration/model quality,
  or floating-point statistical accuracy
- arbitrary-precision arithmetic or external statistics libraries
- cross-family statistics facades before primitive modules exist
