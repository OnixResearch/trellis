# Signal Processing Primitives Delta Specification

## ADDED Requirements

### Requirement: Finite Sample Sequence [r[signal.finite-sample-sequence]]
The system MUST define a finite sample sequence primitive with explicit length,
channel/layout metadata, sample-domain bounds, and valid index admission.

#### Scenario: Checked sample sequence accepts bounded finite metadata [r[signal.finite-sample-sequence.scenario.checked-sequence-accepts-bounded-finite-metadata]]

- GIVEN finite length, channel count, layout metadata, and sample-domain bounds inside representable limits
- WHEN a checked sample sequence constructor or checker evaluates the metadata
- THEN it accepts the sequence metadata and exposes only indices within the finite length and channel layout

#### Scenario: Invalid sample metadata is rejected without mutation [r[signal.finite-sample-sequence.scenario.invalid-sample-metadata-rejected-without-mutation]]

- GIVEN a previously valid sample sequence value
- WHEN a public checked operation receives invalid length, channel, layout, or sample-domain metadata
- THEN it rejects the input explicitly and leaves the previously valid value unchanged

### Requirement: FIR Filter Envelope [r[signal.fir-filter-envelope]]
The system MUST define a finite FIR filter envelope primitive for kernel length,
coefficient bounds, input sample bounds, and conservative accumulator/output
bounds over exact discrete values.

#### Scenario: FIR envelope bounds every admitted tap accumulation [r[signal.fir-filter-envelope.scenario.fir-envelope-bounds-every-admitted-tap-accumulation]]

- GIVEN an admitted finite kernel, coefficient bounds, sample bounds, and accumulator envelope
- WHEN a tap index is within the kernel length and an input sample is within the sample bounds
- THEN the primitive states that the tap contribution and accumulated envelope remain within the declared conservative bounds

#### Scenario: FIR envelope avoids floating-point frequency-response claims [r[signal.fir-filter-envelope.scenario.fir-envelope-avoids-floating-point-frequency-response-claims]]

- GIVEN a finite FIR filter envelope requirement or implementation task
- WHEN it states exported guarantees
- THEN it does not claim floating-point filter accuracy, frequency response, phase response, or runtime DSP-library correctness

### Requirement: Discrete Convolution Window [r[signal.convolution-window]]
The system MUST define a discrete convolution window primitive that maps finite
input length, finite kernel length, padding/admission policy, and output index to
an exact support interval and conservative output envelope.

#### Scenario: Convolution window maps output index to finite support [r[signal.convolution-window.scenario.convolution-window-maps-output-index-to-finite-support]]

- GIVEN admitted finite input length, finite kernel length, padding policy, and output index
- WHEN the convolution window primitive evaluates the output index
- THEN it returns or specifies only support positions that are within admitted input/kernel bounds or explicitly marked as padded positions

#### Scenario: Invalid convolution dimensions are rejected [r[signal.convolution-window.scenario.invalid-convolution-dimensions-rejected]]

- GIVEN zero, overflowing, or otherwise invalid input/kernel/output dimensions
- WHEN a checked convolution-window operation evaluates those dimensions
- THEN it rejects the dimensions without constructing an invalid support mapping

### Requirement: Decimation Admission [r[signal.decimation-admission]]
The system MUST define a decimation admission primitive for finite input length,
positive down-sampling factor, phase/offset, output length, and deterministic
output-to-input index mapping.

#### Scenario: Decimation maps each output index to one admitted input index [r[signal.decimation-admission.scenario.decimation-maps-each-output-index-to-one-admitted-input-index]]

- GIVEN finite input length, positive decimation factor, admitted phase, and output index within the computed output length
- WHEN the decimation primitive maps the output index
- THEN the mapped input index is within the input length and follows the declared factor/phase formula

#### Scenario: Invalid decimation factor or phase is rejected [r[signal.decimation-admission.scenario.invalid-decimation-factor-or-phase-rejected]]

- GIVEN a zero factor, overflowing length computation, or phase outside the admitted factor/input range
- WHEN a checked decimation operation evaluates the metadata
- THEN it rejects the metadata and does not expose an output index mapping

### Requirement: Interpolation Admission [r[signal.interpolation-admission]]
The system MUST define an interpolation admission primitive for finite input
length, positive up-sampling factor, output length, gap-fill policy, and
deterministic output/input correspondence metadata.

#### Scenario: Interpolation classifies source-aligned and gap outputs [r[signal.interpolation-admission.scenario.interpolation-classifies-source-aligned-and-gap-outputs]]

- GIVEN finite input length, positive interpolation factor, admitted output length, and output index
- WHEN the interpolation primitive classifies the output index
- THEN it marks the output as either corresponding to an in-range source sample or to an admitted gap-fill position according to the declared factor policy

#### Scenario: Invalid interpolation metadata is rejected [r[signal.interpolation-admission.scenario.invalid-interpolation-metadata-rejected]]

- GIVEN a zero factor, overflowing output length, or gap-fill policy outside the supported finite model
- WHEN a checked interpolation operation evaluates the metadata
- THEN it rejects the metadata without exposing inconsistent output/input correspondence

### Requirement: Peak Candidate Detection [r[signal.peak-candidate]]
The system MUST define a peak-candidate primitive over finite sequences using a
bounded neighborhood, threshold/envelope metadata, and deterministic tie handling.

#### Scenario: Peak candidate satisfies local finite-neighborhood predicate [r[signal.peak-candidate.scenario.peak-candidate-satisfies-local-finite-neighborhood-predicate]]

- GIVEN a finite sequence, candidate index, admitted neighborhood radius, threshold metadata, and tie policy
- WHEN the peak-candidate predicate accepts the candidate index
- THEN every compared neighbor is within the finite sequence and the candidate satisfies the declared threshold and ordering relation against those neighbors

#### Scenario: Peak tie policy is deterministic [r[signal.peak-candidate.scenario.peak-tie-policy-is-deterministic]]

- GIVEN equal-valued samples inside a peak neighborhood
- WHEN the peak-candidate primitive evaluates candidates under the declared tie policy
- THEN it admits candidates according to that deterministic policy rather than relying on iteration-order ambiguity

### Requirement: Spectral Bin Metadata [r[signal.spectral-bin-metadata]]
The system MUST define spectral-bin metadata primitives for finite bin count,
bin index admission, range metadata, and bounded magnitude/energy envelopes over
exact discrete or fixed-point values.

#### Scenario: Spectral bin metadata admits only finite in-range bins [r[signal.spectral-bin-metadata.scenario.spectral-bin-metadata-admits-only-finite-in-range-bins]]

- GIVEN finite bin count, range metadata, and a candidate bin index
- WHEN the spectral-bin metadata primitive evaluates the bin index
- THEN it admits only indices within the finite bin count and associates them with the declared range metadata

#### Scenario: Spectral metadata avoids FFT numeric correctness claims [r[signal.spectral-bin-metadata.scenario.spectral-metadata-avoids-fft-numeric-correctness-claims]]

- GIVEN a spectral-bin metadata requirement or implementation task
- WHEN it states exported guarantees
- THEN it does not claim FFT algorithm correctness, floating-point spectral accuracy, leakage/noise behavior, or runtime DSP-library correctness
