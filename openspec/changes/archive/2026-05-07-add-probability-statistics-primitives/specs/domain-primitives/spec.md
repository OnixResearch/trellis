# Probability and Statistics Domain Primitives Specification

## Purpose

Adds reusable exact finite probability/statistics requirements for domain
primitives. These requirements cover count tables, frequency tables, finite
distribution admission, CDF/quantile envelopes, bounded moments, deterministic
sampling admission, and confidence-interval metadata. They MUST NOT claim random
number generator quality, stochastic convergence, statistical inference validity,
coverage probability, floating-point statistical accuracy, calibration/model
quality, or external statistics-library behavior.

## ADDED Requirements

### Requirement: Statistics Count Table [r[domain-primitives.stats-count-table]]
The system MUST provide a statistics count-table primitive for finite category
admission, bounded count updates, and overflow rejection.

#### Scenario: Valid category update increments one count [r[domain-primitives.stats-count-table.scenario.valid-category-update-increments-one-count]]

- GIVEN a valid finite count table and an admitted category
- WHEN the count table is updated for that category
- THEN exactly that category count increases and all other category counts are unchanged

#### Scenario: Invalid category or overflowing count is rejected [r[domain-primitives.stats-count-table.scenario.invalid-category-or-overflowing-count-is-rejected]]

- GIVEN an unknown category or a category count at its declared maximum
- WHEN a count update is requested
- THEN the update is rejected and the table state is unchanged

### Requirement: Statistics Frequency Table [r[domain-primitives.stats-frequency-table]]
The system MUST provide a frequency-table primitive for denominator-safe
count-to-frequency derivation over finite non-negative counts.

#### Scenario: Non-zero total count yields frequencies [r[domain-primitives.stats-frequency-table.scenario.non-zero-total-count-yields-frequencies]]

- GIVEN finite non-negative counts whose total count is non-zero and representable
- WHEN frequencies are derived
- THEN each frequency is admitted under the declared ratio envelope and uses the total count as denominator

#### Scenario: Zero or overflowing total count is rejected [r[domain-primitives.stats-frequency-table.scenario.zero-or-overflowing-total-count-is-rejected]]

- GIVEN all-zero counts or counts whose total would overflow the declared accumulator
- WHEN frequency derivation is requested
- THEN the request is rejected or returns an explicit empty result instead of numeric frequencies

### Requirement: Statistics Discrete Distribution [r[domain-primitives.stats-discrete-distribution]]
The system MUST provide a discrete-distribution primitive for finite
non-negative weights, positive total weight admission, and normalization
envelopes.

#### Scenario: Valid weights form admitted finite distribution [r[domain-primitives.stats-discrete-distribution.scenario.valid-weights-form-admitted-finite-distribution]]

- GIVEN finite non-negative weights with a positive representable total weight
- WHEN distribution admission is checked
- THEN the distribution is accepted and every normalized weight lies inside the declared probability envelope

#### Scenario: Invalid distribution weights are rejected [r[domain-primitives.stats-discrete-distribution.scenario.invalid-distribution-weights-are-rejected]]

- GIVEN a negative weight, all-zero weights, or weights whose total overflows the accumulator
- WHEN distribution admission is checked
- THEN the distribution is rejected

### Requirement: Statistics CDF Quantile [r[domain-primitives.stats-cdf-quantile]]
The system MUST provide a CDF/quantile primitive for monotone cumulative weights,
partition lookup, and deterministic tie policy.

#### Scenario: Quantile lookup returns deterministic bucket [r[domain-primitives.stats-cdf-quantile.scenario.quantile-lookup-returns-deterministic-bucket]]

- GIVEN a valid monotone cumulative distribution and an admitted query threshold
- WHEN quantile lookup is evaluated
- THEN the lookup returns exactly one bucket according to the declared boundary and tie policy

#### Scenario: Invalid CDF or threshold is rejected [r[domain-primitives.stats-cdf-quantile.scenario.invalid-cdf-or-threshold-is-rejected]]

- GIVEN a non-monotone CDF, zero final cumulative weight, overflowing cumulative weight, or query threshold outside the admitted range
- WHEN CDF or quantile admission is checked
- THEN the request is rejected

### Requirement: Statistics Bounded Moments [r[domain-primitives.stats-bounded-moments]]
The system MUST provide a bounded-moments primitive for finite count, sum, mean,
and variance-envelope derivation with checked accumulators.

#### Scenario: Valid sample set yields bounded moments [r[domain-primitives.stats-bounded-moments.scenario.valid-sample-set-yields-bounded-moments]]

- GIVEN finite bounded samples whose count and accumulators are representable
- WHEN moment envelopes are derived
- THEN count, sum, mean envelope, and variance envelope are accepted within declared bounds

#### Scenario: Moment derivation rejects empty or overflowing samples [r[domain-primitives.stats-bounded-moments.scenario.moment-derivation-rejects-empty-or-overflowing-samples]]

- GIVEN an empty sample set where a mean is required or samples whose accumulators overflow
- WHEN moment derivation is requested
- THEN the request is rejected or returns an explicit empty result instead of numeric moments

### Requirement: Statistics Deterministic Sampler [r[domain-primitives.stats-deterministic-sampler]]
The system MUST provide a deterministic sampler-admission primitive for mapping
admitted integer thresholds or sample indices into finite distribution buckets.

#### Scenario: Admitted threshold maps to exactly one bucket [r[domain-primitives.stats-deterministic-sampler.scenario.admitted-threshold-maps-to-exactly-one-bucket]]

- GIVEN a valid finite cumulative distribution and an admitted deterministic threshold
- WHEN sample admission is evaluated
- THEN the threshold maps to exactly one bucket according to the declared cumulative partition

#### Scenario: Out-of-range deterministic sample is rejected [r[domain-primitives.stats-deterministic-sampler.scenario.out-of-range-deterministic-sample-is-rejected]]

- GIVEN a threshold outside the admitted range or a malformed cumulative distribution
- WHEN deterministic sample admission is checked
- THEN the sample request is rejected

### Requirement: Statistics Confidence Interval Envelope [r[domain-primitives.stats-confidence-interval-envelope]]
The system MUST provide a confidence-interval-envelope primitive for ordered
interval endpoints and bounded confidence metadata without coverage-probability
claims.

#### Scenario: Valid interval metadata is admitted [r[domain-primitives.stats-confidence-interval-envelope.scenario.valid-interval-metadata-is-admitted]]

- GIVEN ordered interval endpoints and confidence metadata inside declared bounds
- WHEN confidence-interval envelope admission is checked
- THEN the interval metadata is accepted as a bounded interval record

#### Scenario: Malformed interval metadata is rejected [r[domain-primitives.stats-confidence-interval-envelope.scenario.malformed-interval-metadata-is-rejected]]

- GIVEN reversed endpoints, confidence metadata outside declared bounds, or non-representable endpoint arithmetic
- WHEN confidence-interval envelope admission is checked
- THEN the interval metadata is rejected
