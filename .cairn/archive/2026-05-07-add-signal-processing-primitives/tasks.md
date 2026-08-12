## Coverage Index

| Requirement ID | Future module |
|---|---|
| `signal.finite-sample-sequence` | `src/signal_sample.rs` |
| `signal.fir-filter-envelope` | `src/signal_fir.rs` |
| `signal.convolution-window` | `src/signal_convolution.rs` |
| `signal.decimation-admission` | `src/signal_resample.rs` |
| `signal.interpolation-admission` | `src/signal_resample.rs` |
| `signal.peak-candidate` | `src/signal_peak.rs` |
| `signal.spectral-bin-metadata` | `src/signal_spectral.rs` |

## Phase 1: Spec foundation

- [x] Add focused signal-processing primitive OpenSpec with exact finite/discrete requirements.

## Phase 2: Sample and filter primitives

- [x] Implement `signal_sample` finite sequence metadata with checked constructors, no-mutation rejection tests, and Verus shape/index proofs. `[covers=signal.finite-sample-sequence]`
- [x] Implement `signal_fir` finite kernel envelope metadata with coefficient/sample accumulator bounds and proof coverage. `[covers=signal.fir-filter-envelope]`
- [x] Implement `signal_convolution` support-window admission and output-domain envelope proofs. `[covers=signal.convolution-window]`

## Phase 3: Resampling primitives

- [x] Implement `signal_resample` decimation factor/phase admission, output length mapping, and rejection tests. `[covers=signal.decimation-admission]`
- [x] Implement `signal_resample` interpolation factor/gap-fill admission, output/input correspondence, and rejection tests. `[covers=signal.interpolation-admission]`

## Phase 4: Detection and spectral metadata

- [x] Implement `signal_peak` finite-neighborhood peak candidate rules with deterministic tie handling and proof coverage. `[covers=signal.peak-candidate]`
- [x] Implement `signal_spectral` bin-count/range metadata and bounded energy/accumulator envelopes without FFT correctness claims. `[covers=signal.spectral-bin-metadata]`

## Phase 5: Evidence and archive

- [x] Add `docs/requirements.md`, `r[impl]`, `r[depends]`, and `r[verify]` coverage for every signal primitive requirement.
- [x] Run focused cargo tests for signal primitives and capture evidence.
- [x] Run `./scripts/verus.sh src/lib.rs` and capture evidence.
- [x] Run Tracey uncovered/untested checks and final repo validation.
- [x] Sync OpenSpec deltas and archive `add-signal-processing-primitives`.
