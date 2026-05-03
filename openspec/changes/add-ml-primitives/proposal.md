## Why

The domain-primitives umbrella calls out ML primitives as a focused family, and
we want OpenSpecs for primitive contracts before implementation. ML systems need
small verified admission layers around shapes, labels, batches, outputs, and
metrics so later inference or data-pipeline code can reuse exact safety facts
without claiming model quality or training correctness.

## What Changes

- Add a focused ML primitive OpenSpec covering:
  - `ml_tensor_shape`: rank, dimensions, strides, and flattened-index safety.
  - `ml_label_encoding`: declared vocabulary/class-index admission.
  - `ml_dataset_split`: deterministic disjoint train/validation/test partitions.
  - `ml_batch_iterator`: bounded contiguous batch windows over datasets.
  - `ml_feature_normalization`: range-safe normalization with invalid metadata rejection.
  - `ml_top_k`: deterministic bounded top-k/argmax selection.
  - `ml_inference_envelope`: bounded classification/regression result admission.
  - `ml_confusion_matrix`: bounded counts and denominator-safe metrics.
- Keep the change spec-only for now; implementation tasks remain unchecked.

## Capabilities

### New Capabilities

- `domain-primitives.ml-tensor-shape`
- `domain-primitives.ml-label-encoding`
- `domain-primitives.ml-dataset-split`
- `domain-primitives.ml-batch-iterator`
- `domain-primitives.ml-feature-normalization`
- `domain-primitives.ml-top-k`
- `domain-primitives.ml-inference-envelope`
- `domain-primitives.ml-confusion-matrix`

## Impact

- **Files**: OpenSpec artifacts now; later implementation adds `src/ml_*`, `src/lib.rs`, `docs/requirements.md`, tests, README entries, and evidence.
- **APIs**: no runtime APIs in this spec-only slice.
- **Dependencies**: no new runtime dependencies.
- **Testing**: OpenSpec validation now; implementation changes later capture cargo, Verus, and Tracey evidence.

## Out of Scope

- model training, automatic differentiation, GPU execution, probabilistic model quality, neural-network weight correctness, floating-point accuracy, or statistical fairness claims
