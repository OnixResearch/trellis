## Coverage Index

| Requirement ID | Module task |
|---|---|
| `domain-primitives.vision-image-buffer` | `vision_image_buffer` |
| `domain-primitives.vision-roi` | `vision_roi` |
| `domain-primitives.vision-convolution-window` | `vision_convolution_window` |
| `domain-primitives.vision-color-layout` | `vision_color_layout` |
| `domain-primitives.vision-color-transform-envelope` | `vision_color_transform_envelope` |
| `domain-primitives.vision-image-pyramid` | `vision_image_pyramid` |
| `domain-primitives.vision-connected-components` | `vision_connected_components` |
| `domain-primitives.vision-detection-nms` | `vision_detection_nms` |

## Phase 1: Spec foundation

- [x] Create focused machine-vision primitive OpenSpec with exact buffer/layout and evidence expectations.

## Phase 2: Layout and region primitives

- [x] Implement and prove `vision_image_buffer`; cover width/height/stride/channel layout and out-of-bounds buffer rejection. `[covers=domain-primitives.vision-image-buffer]`
- [x] Implement and prove `vision_roi`; cover bounded ROI cropping/mapping and invalid-region rejection. `[covers=domain-primitives.vision-roi]`
- [x] Implement and prove `vision_convolution_window`; cover safe kernel window indexing and border-policy rejection/handling. `[covers=domain-primitives.vision-convolution-window]`

## Phase 3: Color, pyramid, component, and detection primitives

- [x] Implement and prove `vision_color_layout`; cover channel-order/layout admission and invalid layout rejection. `[covers=domain-primitives.vision-color-layout]`
- [x] Implement and prove `vision_color_transform_envelope`; cover bounded integer color transform outputs and invalid coefficient/range rejection. `[covers=domain-primitives.vision-color-transform-envelope]`
- [x] Implement and prove `vision_image_pyramid`; cover monotone level dimensions, valid level mapping, and invalid-scale rejection. `[covers=domain-primitives.vision-image-pyramid]`
- [x] Implement and prove `vision_connected_components`; cover bounded component labels and merge/equivalence invariants. `[covers=domain-primitives.vision-connected-components]`
- [x] Implement and prove `vision_detection_nms`; cover thresholded score ordering, deterministic tie-breaking, non-overlap suppression, and invalid-box rejection. `[covers=domain-primitives.vision-detection-nms]`

## Phase 4: Validation and archive

- [x] Sync completed requirement text, run cargo/Verus/Tracey evidence, run OpenSpec gates, and archive only after implementation is complete.
