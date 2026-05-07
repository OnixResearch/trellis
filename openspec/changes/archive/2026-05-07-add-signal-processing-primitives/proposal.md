## Why

`verified-logic` now has reusable numeric, finite-structure, graph,
time-series, probability/statistics, control-system, GIS, ML, physics, and
vision primitive OpenSpecs. Signal-processing logic is another high-value
cross-domain family: time-series, control, physics, ML, and vision slices all
need exact facts about finite sample buffers, filter windows, convolution
support, resampling admission, peak selection, and spectral-bin metadata.

This change adds a focused spec-only package for signal-processing primitives.
It deliberately uses exact finite/discrete models and conservative envelopes
instead of analog signal quality, floating-point DSP accuracy, noise-model, or
runtime library claims.

## What Changes

- Add `signal_*` domain-primitive requirements for:
  - finite sample sequences and channel/layout metadata
  - FIR filter kernel envelopes
  - discrete convolution window support and output-domain bounds
  - decimation admission and output length/index mapping
  - interpolation admission and gap-fill/index mapping
  - peak candidate detection over finite neighborhoods
  - spectral-bin metadata over bounded integer/fixed-point bins
- Leave all implementation, proofs, Tracey tags, tests, and evidence as future
  tasks.
- Update the `add-domain-primitives` umbrella taxonomy so reusable signal
  primitives have their own child OpenSpec instead of being duplicated in
  time-series, control, ML, physics, or vision changes.

## Capabilities

### New Capabilities

- `domain-primitives.signal-processing-family`: reusable finite
  signal-processing taxonomy and required child-change scope.
- `signal.finite-sample-sequence`: exact finite sequence shape, sample-domain,
  channel/layout, and index validity metadata.
- `signal.fir-filter-envelope`: exact finite kernel metadata and coefficient
  bounds without runtime floating-point accuracy claims.
- `signal.convolution-window`: deterministic support window and output-index
  admission for finite discrete convolution.
- `signal.decimation-admission`: checked down-sampling factor, phase, and output
  index mapping over finite input.
- `signal.interpolation-admission`: checked up-sampling/gap-fill metadata and
  output/input correspondence over finite input.
- `signal.peak-candidate`: finite-neighborhood peak candidate rules with stable
  tie handling.
- `signal.spectral-bin-metadata`: bounded bin index/range metadata and energy
  envelope without claiming FFT/runtime spectral accuracy.

## Impact

- **Files**: future implementation may add `src/signal_*`, `src/lib.rs`,
  `docs/requirements.md`, tests, README entries, and OpenSpec evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new runtime dependencies.
- **Testing**: this package validates through `openspec validate`; future
  implementation slices must add focused tests, Verus proofs, Tracey coverage,
  and final repo validation.

## Out of Scope

- implementing modules in this change
- floating-point filter accuracy, analog signal fidelity, noise/process-model
  correctness, FFT algorithm correctness, or stochastic signal-quality claims
- bindings to external DSP libraries
- cross-family signal-processing pipelines before reusable primitives land
