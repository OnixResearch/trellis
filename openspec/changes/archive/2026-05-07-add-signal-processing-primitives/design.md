## Context

Signal-processing helpers are likely to be reused by time-series, bounded
control, physics, ML, and machine-vision primitives. Without a focused child
OpenSpec, those domains would duplicate local definitions for sample buffers,
convolution support, resampling admission, peak detection, or spectral-bin
metadata.

## Goals / Non-Goals

**Goals:**

- define exact finite/discrete signal-processing primitive requirements
- state validity boundaries before implementation
- keep reusable signal helpers outside domain-specific child changes
- avoid claims that require floating-point DSP, analog/noise models, FFT
  algorithm proofs, or stochastic signal-quality assumptions

**Non-Goals:**

- implement signal-processing modules now
- prove runtime library DSP correctness
- guarantee analog fidelity, frequency response accuracy, or denoising quality
- provide full end-to-end ML/vision/control pipelines

## Decisions

### 1. Model signals as finite indexed sample sequences

**Choice:** Signal primitives use finite sample sequences with explicit length,
channel/layout metadata, sample-domain bounds, and index validity predicates.

**Rationale:** Finite sequences are Verus-friendly and compose with existing
numeric, time-series, control, ML, physics, and vision primitives.

**Alternative:** Model continuous signals or unbounded streams. Rejected because
those require separate approximation and liveness models.

### 2. Treat filtering/convolution as support and envelope primitives first

**Choice:** FIR filters and convolution windows specify kernel length,
coefficient bounds, support windows, output-domain admission, and conservative
accumulator envelopes.

**Rationale:** These facts are useful and exactly verifiable without claiming
floating-point frequency-response accuracy or runtime DSP correctness.

**Alternative:** Specify full DSP algorithm correctness now. Rejected as too
broad for a reusable primitive foundation.

### 3. Specify resampling as deterministic admission/mapping

**Choice:** Decimation and interpolation primitives specify valid factors,
phases, lengths, and deterministic input/output index correspondence.

**Rationale:** Index mapping and shape preservation are cross-domain and exact;
quality claims about anti-aliasing or reconstruction need later dedicated specs.

### 4. Keep spectral metadata structural

**Choice:** Spectral-bin primitives specify bin counts, bin ranges, bounded
energy/accumulator metadata, and index admission only.

**Rationale:** Structural metadata composes with finite buffers and histograms
without claiming FFT numeric accuracy.

## Risks / Trade-offs

**Accidental DSP quality claims** → Requirements explicitly forbid floating-point,
frequency-response, denoising, alias-rejection, or FFT-accuracy claims unless a
dedicated later numeric/DSP model owns them.

**Duplicate time-series or vision requirements** → This child owns reusable
signal primitives; family-specific time-series/vision requirements should
reference these IDs when they need the same facts.

**Over-broad implementation slice** → Future tasks split sample sequence,
filter/convolution, resampling, peak detection, and spectral metadata into
separate proof/evidence steps.

## Validation Plan

- `openspec validate add-signal-processing-primitives --strict --json`
- validate every active primitive-family change after umbrella updates
- future implementation slices run focused cargo tests, `./scripts/verus.sh
  src/lib.rs`, Tracey uncovered/untested checks, and final repo validation
