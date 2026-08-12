## Why

The GIS, ML, physics, and vision primitive OpenSpecs all depend on reusable
numeric proof shapes: scaled fixed-point ranges, checked arithmetic envelopes,
affine transforms, monotone sequences, bucketed counts, and denominator-safe
ratios. We should specify these foundations before implementation so later
family changes can reference common requirements instead of duplicating local
versions.

This change now implements a focused numeric-foundation OpenSpec
for exact finite/discrete helpers that are Verus-friendly and reusable across domain
families.

## What Changes

- Add focused numeric foundation requirements:
  - `numeric-fixed-point-scale`: scaled integer metadata, conversion envelopes,
    and checked arithmetic.
  - `numeric-checked-arithmetic-envelope`: reusable add/sub/mul/div overflow and
    bound admission.
  - `numeric-denominator-safe-ratio`: numerator/denominator validity and empty
    result semantics for zero denominators.
  - `numeric-monotone-sequence`: sortedness, lower/upper-bound, and range-query
    safety.
  - `numeric-histogram-counter`: bounded bucket indexing and count updates.
  - `numeric-affine-transform-discrete`: bounded integer matrix/vector transform
    envelopes.
  - `numeric-prefix-sum`: bounded cumulative sums and range-sum decomposition.
  - `numeric-quantized-threshold`: deterministic threshold/quantization bucket
    admission.
- Implement exact finite Rust/Verus primitives and capture cargo, Verus, Tracey, and OpenSpec evidence.

## Capabilities

### New Capabilities

- `domain-primitives.numeric-fixed-point-scale`
- `domain-primitives.numeric-checked-arithmetic-envelope`
- `domain-primitives.numeric-denominator-safe-ratio`
- `domain-primitives.numeric-monotone-sequence`
- `domain-primitives.numeric-histogram-counter`
- `domain-primitives.numeric-affine-transform-discrete`
- `domain-primitives.numeric-prefix-sum`
- `domain-primitives.numeric-quantized-threshold`

## Impact

- **Files**: OpenSpec artifacts now; later implementation adds `src/numeric_*`,
  `src/lib.rs`, `docs/requirements.md`, tests, README entries, and evidence.
- **APIs**: adds focused `src/numeric_*` exact finite admission/checker APIs.
- **Dependencies**: no new runtime dependencies.
- **Testing**: focused cargo tests, Verus proof evidence, Tracey coverage, and final gates.

## Out of Scope

- floating-point error analysis, arbitrary-precision arithmetic libraries,
  symbolic algebra, probabilistic numeric claims, or hardware SIMD/GPU kernels
- replacing family-specific requirements already scoped to concrete GIS/ML/
  physics/vision APIs
