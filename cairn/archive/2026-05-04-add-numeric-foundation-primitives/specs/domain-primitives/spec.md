# Numeric Foundation Domain Primitives Specification

## Purpose

Adds reusable exact numeric foundation requirements for domain primitives. These
requirements cover scaled integers, checked arithmetic, safe ratios, monotone
sequences, histograms, affine transforms, prefix sums, and quantized thresholds.
They MUST NOT claim floating-point accuracy, probabilistic guarantees,
arbitrary-precision behavior, or hardware performance properties.

## ADDED Requirements

### Requirement: Numeric Fixed Point Scale [r[domain-primitives.numeric-fixed-point-scale]]
The system MUST provide a numeric fixed-point scale primitive for checked scaled
integer metadata, conversion envelopes, and range preservation.

#### Scenario: Valid scaled conversion preserves envelope [r[domain-primitives.numeric-fixed-point-scale.scenario.valid-scaled-conversion-preserves-envelope]]

- GIVEN valid scale metadata and a raw integer inside the representable envelope
- WHEN the value is converted to the scaled representation
- THEN the scaled value is accepted and remains inside the declared envelope

#### Scenario: Invalid scale or conversion is rejected [r[domain-primitives.numeric-fixed-point-scale.scenario.invalid-scale-or-conversion-is-rejected]]

- GIVEN zero scale, overflowing scale metadata, or a raw value outside the representable envelope
- WHEN fixed-point admission is checked
- THEN the conversion is rejected

### Requirement: Numeric Checked Arithmetic Envelope [r[domain-primitives.numeric-checked-arithmetic-envelope]]
The system MUST provide a numeric checked-arithmetic-envelope primitive for
bounded add, subtract, multiply, and safe zero-numerator divide-admission operations.

#### Scenario: Accepted arithmetic result stays within bounds [r[domain-primitives.numeric-checked-arithmetic-envelope.scenario.accepted-arithmetic-result-stays-within-bounds]]

- GIVEN operands and an operation whose exact result is representable and inside the declared bounds
- WHEN checked arithmetic is evaluated
- THEN the result is accepted and lies inside the bounds

#### Scenario: Overflowing or invalid arithmetic is rejected [r[domain-primitives.numeric-checked-arithmetic-envelope.scenario.overflowing-or-invalid-arithmetic-is-rejected]]

- GIVEN operands whose operation overflows, leaves the declared bounds, divides by zero, or is outside the safe finite divide-admission surface
- WHEN checked arithmetic is evaluated
- THEN the operation is rejected

### Requirement: Numeric Denominator Safe Ratio [r[domain-primitives.numeric-denominator-safe-ratio]]
The system MUST provide a numeric denominator-safe-ratio primitive for explicit
non-zero denominator admission and empty/reject semantics.

#### Scenario: Non-zero denominator yields numeric ratio [r[domain-primitives.numeric-denominator-safe-ratio.scenario.non-zero-denominator-yields-numeric-ratio]]

- GIVEN a numerator and non-zero denominator whose ratio is representable
- WHEN ratio derivation is requested
- THEN a numeric ratio result is returned

#### Scenario: Zero denominator does not produce numeric ratio [r[domain-primitives.numeric-denominator-safe-ratio.scenario.zero-denominator-does-not-produce-numeric-ratio]]

- GIVEN a zero denominator
- WHEN ratio derivation is requested
- THEN the primitive rejects the request or returns an explicit empty result instead of a numeric ratio

### Requirement: Numeric Monotone Sequence [r[domain-primitives.numeric-monotone-sequence]]
The system MUST provide a numeric monotone-sequence primitive for sortedness,
lower-bound search, and range-query safety over finite three-slot sequences.

#### Scenario: Bound search returns in-range partition point [r[domain-primitives.numeric-monotone-sequence.scenario.bound-search-returns-in-range-partition-point]]

- GIVEN a finite sorted sequence and a query value
- WHEN lower-bound search is evaluated
- THEN the returned index is inside `0..=len` and partitions the sequence according to the query predicate

#### Scenario: Unsorted sequence is rejected [r[domain-primitives.numeric-monotone-sequence.scenario.unsorted-sequence-is-rejected]]

- GIVEN a sequence with an out-of-order adjacent pair
- WHEN monotone sequence admission is checked
- THEN the sequence is rejected

### Requirement: Numeric Histogram Counter [r[domain-primitives.numeric-histogram-counter]]
The system MUST provide a numeric histogram-counter primitive for bounded three-bucket indexing and count-update admission.

#### Scenario: Valid bucket update increments one count [r[domain-primitives.numeric-histogram-counter.scenario.valid-bucket-update-increments-one-count]]

- GIVEN a valid histogram and a value admitted to exactly one bucket
- WHEN the histogram is updated
- THEN exactly that bucket count increases and all other bucket counts are unchanged

#### Scenario: Invalid bucket or overflowing count is rejected [r[domain-primitives.numeric-histogram-counter.scenario.invalid-bucket-or-overflowing-count-is-rejected]]

- GIVEN a value outside all buckets or a bucket count at its maximum
- WHEN histogram update is requested
- THEN the update is rejected

### Requirement: Numeric Affine Transform Discrete [r[domain-primitives.numeric-affine-transform-discrete]]
The system MUST provide a numeric affine-transform primitive for bounded integer
matrix/vector transform envelopes.

#### Scenario: Transform output remains inside declared envelope [r[domain-primitives.numeric-affine-transform-discrete.scenario.transform-output-remains-inside-declared-envelope]]

- GIVEN valid integer coefficients, input bounds, and output bounds
- WHEN the affine transform is admitted
- THEN every produced output component lies inside the declared output envelope

#### Scenario: Overflowing transform envelope is rejected [r[domain-primitives.numeric-affine-transform-discrete.scenario.overflowing-transform-envelope-is-rejected]]

- GIVEN coefficients, offsets, or input bounds whose transform can overflow or leave the declared output envelope
- WHEN affine transform admission is checked
- THEN the transform is rejected

### Requirement: Numeric Prefix Sum [r[domain-primitives.numeric-prefix-sum]]
The system MUST provide a numeric prefix-sum primitive for bounded three-slot cumulative sums and range-sum admission.

#### Scenario: Prefix sums decompose range sum [r[domain-primitives.numeric-prefix-sum.scenario.prefix-sums-decompose-range-sum]]

- GIVEN a finite three-slot prefix tuple whose cumulative sums are representable
- WHEN a valid range sum is requested
- THEN the range sum equals the difference of the corresponding prefix sums

#### Scenario: Prefix sum overflow is rejected [r[domain-primitives.numeric-prefix-sum.scenario.prefix-sum-overflow-is-rejected]]

- GIVEN a sequence whose cumulative sum would overflow the representation
- WHEN prefix-sum construction is requested
- THEN the construction is rejected

### Requirement: Numeric Quantized Threshold [r[domain-primitives.numeric-quantized-threshold]]
The system MUST provide a numeric quantized-threshold primitive for deterministic
threshold and bucket admission over bounded values.

#### Scenario: Value maps to deterministic quantized bucket [r[domain-primitives.numeric-quantized-threshold.scenario.value-maps-to-deterministic-quantized-bucket]]

- GIVEN valid ordered three-bucket thresholds and a value inside the supported range
- WHEN quantization is evaluated
- THEN the value maps to exactly one deterministic bucket

#### Scenario: Invalid thresholds are rejected [r[domain-primitives.numeric-quantized-threshold.scenario.invalid-thresholds-are-rejected]]

- GIVEN unordered three-bucket thresholds, duplicate thresholds where disallowed, or a value outside the supported range
- WHEN threshold admission is checked
- THEN the request is rejected
