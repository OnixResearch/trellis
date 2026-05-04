# Machine Vision Domain Primitives Specification

## Purpose

Adds concrete machine-vision primitive requirements over exact image-buffer,
region, window, color-layout, pyramid, component-label, and detection-result
models. The requirements MUST NOT claim model accuracy, calibration correctness,
codec correctness, floating-point color science, OCR/segmentation quality, neural-network quality, or real-time video behavior.

## ADDED Requirements

### Requirement: Vision Image Buffer [r[domain-primitives.vision-image-buffer]]
The system MUST provide a machine-vision image-buffer primitive for checked
width, height, stride, channel, and backing-buffer layout.

#### Scenario: Valid pixel address is in bounds [r[domain-primitives.vision-image-buffer.scenario.valid-pixel-address-is-in-bounds]]

- GIVEN a valid image buffer and pixel coordinate/channel inside declared bounds
- WHEN the byte offset is computed
- THEN the offset is inside the backing buffer

#### Scenario: Bad stride or buffer length is rejected [r[domain-primitives.vision-image-buffer.scenario.bad-stride-or-buffer-length-is-rejected]]

- GIVEN an image layout whose stride or backing length cannot contain all rows
- WHEN the image-buffer constructor is evaluated
- THEN it rejects the layout

### Requirement: Vision ROI [r[domain-primitives.vision-roi]]
The system MUST provide a machine-vision ROI primitive for bounded regions of
interest and crop-coordinate mapping.

#### Scenario: ROI crop maps into source image [r[domain-primitives.vision-roi.scenario.roi-crop-maps-into-source-image]]

- GIVEN a valid image buffer and ROI inside the image bounds
- WHEN a crop-local coordinate is mapped to source coordinates
- THEN the source coordinate is inside the image buffer

#### Scenario: Out-of-image ROI is rejected [r[domain-primitives.vision-roi.scenario.out-of-image-roi-is-rejected]]

- GIVEN an ROI that extends outside the source image bounds
- WHEN the checked ROI constructor is evaluated
- THEN the ROI is rejected

### Requirement: Vision Convolution Window [r[domain-primitives.vision-convolution-window]]
The system MUST provide a machine-vision convolution-window primitive for safe
kernel indexing under declared border policy.

#### Scenario: Interior kernel samples are in bounds [r[domain-primitives.vision-convolution-window.scenario.interior-kernel-samples-are-in-bounds]]

- GIVEN a valid image buffer, kernel shape, and anchor such that the window lies inside the image
- WHEN window sample offsets are computed
- THEN every sample offset is inside the source image buffer

#### Scenario: Unsupported border sample is rejected [r[domain-primitives.vision-convolution-window.scenario.unsupported-border-sample-is-rejected]]

- GIVEN a kernel window that crosses the image boundary with a reject-border policy
- WHEN sampling is requested
- THEN the window is rejected

### Requirement: Vision Color Layout [r[domain-primitives.vision-color-layout]]
The system MUST provide a machine-vision color-layout primitive for declared
channel count, order, and per-channel byte offset admission.

#### Scenario: Declared channel offset is in bounds [r[domain-primitives.vision-color-layout.scenario.declared-channel-offset-is-in-bounds]]

- GIVEN a valid color layout and declared channel
- WHEN the channel offset is computed
- THEN the offset is inside the pixel stride

#### Scenario: Invalid channel layout is rejected [r[domain-primitives.vision-color-layout.scenario.invalid-channel-layout-is-rejected]]

- GIVEN duplicate channels, unknown channels, or channel offsets outside the pixel stride
- WHEN color-layout admission is checked
- THEN the layout is rejected

### Requirement: Vision Color Transform Envelope [r[domain-primitives.vision-color-transform-envelope]]
The system MUST provide a machine-vision color-transform-envelope primitive for
bounded integer color-space transform outputs.

#### Scenario: Transform output stays inside envelope [r[domain-primitives.vision-color-transform-envelope.scenario.transform-output-stays-inside-envelope]]

- GIVEN valid integer coefficients, input channel bounds, and output bounds
- WHEN the transform envelope is checked
- THEN every admitted output channel lies inside the declared output bounds

#### Scenario: Overflowing transform is rejected [r[domain-primitives.vision-color-transform-envelope.scenario.overflowing-transform-is-rejected]]

- GIVEN coefficients or input bounds whose transform can overflow the representation
- WHEN transform admission is checked
- THEN the transform is rejected

### Requirement: Vision Image Pyramid [r[domain-primitives.vision-image-pyramid]]
The system MUST provide a machine-vision image-pyramid primitive for bounded
multi-level image dimensions and level mapping.

#### Scenario: Pyramid levels decrease monotonically [r[domain-primitives.vision-image-pyramid.scenario.pyramid-levels-decrease-monotonically]]

- GIVEN a valid base image and pyramid scale plan
- WHEN pyramid dimensions are derived
- THEN each admitted level has positive dimensions no larger than the previous level

#### Scenario: Invalid pyramid scale is rejected [r[domain-primitives.vision-image-pyramid.scenario.invalid-pyramid-scale-is-rejected]]

- GIVEN a scale plan that yields zero dimensions or overflowing level metadata
- WHEN pyramid admission is checked
- THEN the pyramid is rejected

### Requirement: Vision Connected Components [r[domain-primitives.vision-connected-components]]
The system MUST provide a machine-vision connected-components primitive for
bounded component labels and merge/equivalence facts.

#### Scenario: Component labels stay in declared range [r[domain-primitives.vision-connected-components.scenario.component-labels-stay-in-declared-range]]

- GIVEN a valid label image and component-count bound
- WHEN labels are admitted
- THEN every non-background label is within the declared component range

#### Scenario: Invalid label or merge is rejected [r[domain-primitives.vision-connected-components.scenario.invalid-label-or-merge-is-rejected]]

- GIVEN a label outside the declared range or a merge referencing an unknown component
- WHEN component admission is checked
- THEN the label or merge is rejected

### Requirement: Vision Detection NMS [r[domain-primitives.vision-detection-nms]]
The system MUST provide a machine-vision detection/NMS primitive for deterministic
score thresholding and non-maximum suppression.

#### Scenario: Higher-scored overlapping box suppresses lower-scored box [r[domain-primitives.vision-detection-nms.scenario.higher-scored-overlapping-box-suppresses-lower-scored-box]]

- GIVEN two valid same-class boxes whose overlap exceeds the suppression threshold and whose scores are ordered
- WHEN NMS is evaluated
- THEN only the higher-scored box remains admitted

#### Scenario: Invalid detection is rejected [r[domain-primitives.vision-detection-nms.scenario.invalid-detection-is-rejected]]

- GIVEN a detection with an invalid box, unknown class, or score outside the configured range
- WHEN detection admission is checked
- THEN the detection is rejected before suppression
