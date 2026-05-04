## Why

The domain-primitives umbrella calls out machine-vision primitives as a focused
family. Before implementation, we need OpenSpecs for exact buffer/layout,
region/window, color/layout transform, pyramid, component-label, and detection
result contracts.

This change now implements exact finite primitive requirements that support
later vision pipelines without claiming model accuracy, calibration correctness,
codec correctness, or real-time video behavior.

## What Changes

- Add focused machine-vision primitive requirements:
  - `vision_image_buffer`
  - `vision_roi`
  - `vision_convolution_window`
  - `vision_color_layout`
  - `vision_color_transform_envelope`
  - `vision_image_pyramid`
  - `vision_connected_components`
  - `vision_detection_nms`
- Implement exact finite Rust/Verus primitives and capture cargo, Verus, Tracey, and OpenSpec evidence.

## Capabilities

### New Capabilities

- `domain-primitives.vision-image-buffer`
- `domain-primitives.vision-roi`
- `domain-primitives.vision-convolution-window`
- `domain-primitives.vision-color-layout`
- `domain-primitives.vision-color-transform-envelope`
- `domain-primitives.vision-image-pyramid`
- `domain-primitives.vision-connected-components`
- `domain-primitives.vision-detection-nms`

## Impact

- **Files**: OpenSpec artifacts now; later implementation adds `src/vision_*`, `src/lib.rs`, `docs/requirements.md`, tests, README entries, and evidence.
- **APIs**: adds focused `src/vision_*` exact finite admission/checker APIs.
- **Dependencies**: no new runtime dependencies.
- **Testing**: focused cargo tests, Verus, Tracey, full verification, and OpenSpec validation.

## Out of Scope

- image codec verification, camera calibration solvers, OCR/segmentation model correctness, neural-network accuracy, real-time video scheduling, or floating-point color science claims
