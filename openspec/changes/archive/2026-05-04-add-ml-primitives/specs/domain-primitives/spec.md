# ML Domain Primitives Specification

## Purpose

Adds concrete ML primitive requirements over exact finite metadata, bounded
counts, deterministic ordering, and checked admission. The requirements MUST NOT
claim model quality, training correctness, fairness, or floating-point accuracy.

## ADDED Requirements

### Requirement: ML Tensor Shape [r[domain-primitives.ml-tensor-shape]]
The system MUST provide an ML tensor-shape primitive for exact finite rank-1 dimensions
and flattened-index safety.

#### Scenario: Valid multi-index maps into buffer [r[domain-primitives.ml-tensor-shape.scenario.valid-multi-index-maps-into-buffer]]

- GIVEN a valid finite rank-1 tensor shape and an index within its dimension
- WHEN the flattened offset is computed
- THEN the offset is less than the tensor element count

#### Scenario: Invalid rank or index is rejected [r[domain-primitives.ml-tensor-shape.scenario.invalid-rank-or-index-is-rejected]]

- GIVEN a rank mismatch, zero dimension, or out-of-bounds index
- WHEN tensor-shape admission is checked
- THEN the request is rejected

### Requirement: ML Label Encoding [r[domain-primitives.ml-label-encoding]]
The system MUST provide an ML label-encoding primitive for declared vocabulary
membership and class-index roundtrips.

#### Scenario: Declared label maps to stable index [r[domain-primitives.ml-label-encoding.scenario.declared-label-maps-to-stable-index]]

- GIVEN a finite vocabulary with unique labels
- WHEN a declared label is encoded and decoded
- THEN the decoded label equals the original label

#### Scenario: Unknown label is rejected [r[domain-primitives.ml-label-encoding.scenario.unknown-label-is-rejected]]

- GIVEN a label absent from the declared vocabulary
- WHEN label encoding is requested
- THEN the label is rejected

### Requirement: ML Dataset Split [r[domain-primitives.ml-dataset-split]]
The system MUST provide an ML dataset-split primitive for deterministic disjoint
train, validation, and test partitions over bounded dataset indices.

#### Scenario: Split partitions are disjoint [r[domain-primitives.ml-dataset-split.scenario.split-partitions-are-disjoint]]

- GIVEN a valid dataset size and split plan
- WHEN partitions are produced
- THEN no dataset index appears in more than one partition

#### Scenario: Invalid split bounds are rejected [r[domain-primitives.ml-dataset-split.scenario.invalid-split-bounds-are-rejected]]

- GIVEN split sizes whose sum overflows or exceeds the dataset size
- WHEN split admission is checked
- THEN the split is rejected

### Requirement: ML Batch Iterator [r[domain-primitives.ml-batch-iterator]]
The system MUST provide an ML batch-iterator primitive for bounded contiguous
batch ranges over dataset indices.

#### Scenario: Batches cover dataset range without overlap [r[domain-primitives.ml-batch-iterator.scenario.batches-cover-dataset-range-without-overlap]]

- GIVEN a valid dataset size and positive batch size
- WHEN batches are enumerated
- THEN each index in the dataset range appears in exactly one batch

#### Scenario: Invalid batch size is rejected [r[domain-primitives.ml-batch-iterator.scenario.invalid-batch-size-is-rejected]]

- GIVEN a zero batch size or overflowing range computation
- WHEN batch iteration is requested
- THEN the request is rejected

### Requirement: ML Feature Normalization [r[domain-primitives.ml-feature-normalization]]
The system MUST provide an ML feature-normalization primitive for bounded feature
scaling and missing-value rejection.

#### Scenario: Bounded feature normalizes into target interval [r[domain-primitives.ml-feature-normalization.scenario.bounded-feature-normalizes-into-target-interval]]

- GIVEN valid feature metadata with non-zero span and a feature value inside the metadata range
- WHEN normalization is performed
- THEN the normalized value lies within the configured target interval

#### Scenario: Missing value or invalid span is rejected [r[domain-primitives.ml-feature-normalization.scenario.missing-value-or-invalid-span-is-rejected]]

- GIVEN a missing feature value or equal/inverted min/max metadata
- WHEN normalization is requested
- THEN the request is rejected

### Requirement: ML Top K [r[domain-primitives.ml-top-k]]
The system MUST provide an ML top-k primitive for deterministic bounded selection
from finite score vectors.

#### Scenario: Top k results are ordered and in range [r[domain-primitives.ml-top-k.scenario.top-k-results-are-ordered-and-in-range]]

- GIVEN a finite score vector and valid k
- WHEN top-k is computed
- THEN every returned index is in range and results are ordered by score with deterministic tie-breaking

#### Scenario: Invalid k is rejected [r[domain-primitives.ml-top-k.scenario.invalid-k-is-rejected]]

- GIVEN k equal to zero or greater than the score-vector length
- WHEN top-k admission is checked
- THEN the request is rejected

### Requirement: ML Inference Envelope [r[domain-primitives.ml-inference-envelope]]
The system MUST provide an ML inference-envelope primitive for deterministic
classification or regression result admission bounded by model metadata.

#### Scenario: Declared class result is accepted [r[domain-primitives.ml-inference-envelope.scenario.declared-class-result-is-accepted]]

- GIVEN model metadata with a finite class count and a result whose class index and score are in range
- WHEN result admission is checked
- THEN the classification result is accepted

#### Scenario: Output outside metadata is rejected [r[domain-primitives.ml-inference-envelope.scenario.output-outside-metadata-is-rejected]]

- GIVEN an unknown class index, invalid confidence score, or regression value outside the declared envelope
- WHEN result admission is checked
- THEN the result is rejected

### Requirement: ML Confusion Matrix [r[domain-primitives.ml-confusion-matrix]]
The system MUST provide an ML confusion-matrix primitive for bounded metric counts
and denominator-safe derived metrics.

#### Scenario: Valid prediction updates bounded count [r[domain-primitives.ml-confusion-matrix.scenario.valid-prediction-updates-bounded-count]]

- GIVEN valid expected and predicted labels within the matrix class count
- WHEN the confusion matrix is updated
- THEN exactly the corresponding cell count increases within bounds

#### Scenario: Zero-denominator metric is not produced as numeric [r[domain-primitives.ml-confusion-matrix.scenario.zero-denominator-metric-is-not-produced-as-numeric]]

- GIVEN a class with no positive predictions or no actual positives
- WHEN precision or recall is requested
- THEN the primitive rejects the numeric metric or returns an explicit empty metric result
