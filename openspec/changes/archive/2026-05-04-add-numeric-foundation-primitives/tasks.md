## Coverage Index

| Requirement ID | Module task |
|---|---|
| `domain-primitives.numeric-fixed-point-scale` | `numeric_fixed_point_scale` |
| `domain-primitives.numeric-checked-arithmetic-envelope` | `numeric_checked_arithmetic_envelope` |
| `domain-primitives.numeric-denominator-safe-ratio` | `numeric_denominator_safe_ratio` |
| `domain-primitives.numeric-monotone-sequence` | `numeric_monotone_sequence` |
| `domain-primitives.numeric-histogram-counter` | `numeric_histogram_counter` |
| `domain-primitives.numeric-affine-transform-discrete` | `numeric_affine_transform_discrete` |
| `domain-primitives.numeric-prefix-sum` | `numeric_prefix_sum` |
| `domain-primitives.numeric-quantized-threshold` | `numeric_quantized_threshold` |

## Phase 1: Spec foundation

- [x] Create focused numeric foundation primitive OpenSpec with exact discrete model and evidence expectations.

## Phase 2: Arithmetic and ratio foundations

- [x] Implement and prove `numeric_fixed_point_scale`; cover scale metadata, checked conversion, range preservation, and invalid-scale rejection. `[covers=domain-primitives.numeric-fixed-point-scale]`
- [x] Implement and prove `numeric_checked_arithmetic_envelope`; cover add/sub/mul/safe finite divide-admission bound checks, overflow rejection, and accepted-result range preservation. `[covers=domain-primitives.numeric-checked-arithmetic-envelope]`
- [x] Implement and prove `numeric_denominator_safe_ratio`; cover non-zero denominator numeric result and zero-denominator empty/reject semantics. `[covers=domain-primitives.numeric-denominator-safe-ratio]`

## Phase 3: Sequence, bucket, and transform foundations

- [x] Implement and prove `numeric_monotone_sequence`; cover sortedness, lower-bound safety, and range-query containment. `[covers=domain-primitives.numeric-monotone-sequence]`
- [x] Implement and prove `numeric_histogram_counter`; cover bucket index admission, bounded count updates, and overflow rejection. `[covers=domain-primitives.numeric-histogram-counter]`
- [x] Implement and prove `numeric_affine_transform_discrete`; cover integer matrix/vector transform bounds and invalid coefficient/range rejection. `[covers=domain-primitives.numeric-affine-transform-discrete]`
- [x] Implement and prove `numeric_prefix_sum`; cover cumulative sum bounds, range-sum decomposition, and overflow rejection. `[covers=domain-primitives.numeric-prefix-sum]`
- [x] Implement and prove `numeric_quantized_threshold`; cover deterministic bucket/threshold admission and out-of-range rejection. `[covers=domain-primitives.numeric-quantized-threshold]`

## Phase 4: Validation and archive

- [x] Sync completed requirement text to `docs/requirements.md` and canonical `openspec/specs/domain-primitives/spec.md`.
- [x] Run focused cargo/Verus/Tracey evidence for implemented modules.
- [x] Run OpenSpec gates after implementation evidence is synced.
- [x] Archive only after implementation and validation tasks complete.
