## Coverage Index

| Requirement ID | Module task |
|---|---|
| `domain-primitives.ml-tensor-shape` | `ml_tensor_shape` |
| `domain-primitives.ml-label-encoding` | `ml_label_encoding` |
| `domain-primitives.ml-dataset-split` | `ml_dataset_split` |
| `domain-primitives.ml-batch-iterator` | `ml_batch_iterator` |
| `domain-primitives.ml-feature-normalization` | `ml_feature_normalization` |
| `domain-primitives.ml-top-k` | `ml_top_k` |
| `domain-primitives.ml-inference-envelope` | `ml_inference_envelope` |
| `domain-primitives.ml-confusion-matrix` | `ml_confusion_matrix` |

## Phase 1: Spec foundation

- [x] Create focused ML primitive OpenSpec with finite metadata, validity-boundary, and evidence expectations.

## Phase 2: Data shape and partition primitives

- [x] Implement and prove `ml_tensor_shape`; cover rank-1 shape validity and safe flattened-index bounds. `[covers=domain-primitives.ml-tensor-shape]`
- [x] Implement and prove `ml_label_encoding`; cover declared label/index roundtrip and unknown-label rejection. `[covers=domain-primitives.ml-label-encoding]`
- [x] Implement and prove `ml_dataset_split`; cover deterministic disjoint partitions and out-of-range split rejection. `[covers=domain-primitives.ml-dataset-split]`
- [x] Implement and prove `ml_batch_iterator`; cover contiguous batch ranges, complete/non-overlapping coverage, and out-of-bounds rejection. `[covers=domain-primitives.ml-batch-iterator]`

## Phase 3: Feature, output, and metric primitives

- [x] Implement and prove `ml_feature_normalization`; cover bounded normalized outputs and missing/zero-denominator rejection. `[covers=domain-primitives.ml-feature-normalization]`
- [x] Implement and prove `ml_top_k`; cover deterministic top-k/argmax ordering, tie-breaking, and invalid-k rejection. `[covers=domain-primitives.ml-top-k]`
- [x] Implement and prove `ml_inference_envelope`; cover deterministic class/regression admission bounded by metadata and invalid-output rejection. `[covers=domain-primitives.ml-inference-envelope]`
- [x] Implement and prove `ml_confusion_matrix`; cover bounded count updates and denominator-safe metric derivation. `[covers=domain-primitives.ml-confusion-matrix]`

## Phase 4: Validation and archive

- [x] Sync completed requirement text to `docs/requirements.md` and canonical `openspec/specs/domain-primitives/spec.md`.
- [x] Run focused cargo/Verus/Tracey evidence for implemented modules.
- [x] Run OpenSpec gates after implementation evidence is synced.
- [x] Archive only after implementation and validation tasks complete.
