## Coverage Index

| Requirement ID | Module task |
|---|---|
| `domain-primitives.gis-coordinate-envelope` | `gis_coordinate_envelope` |
| `domain-primitives.gis-bounding-box` | `gis_bounding_box` |
| `domain-primitives.gis-tile-index` | `gis_tile_index` |
| `domain-primitives.gis-linestring-topology` | `gis_linestring_topology` |
| `domain-primitives.ml-tensor-shape` | `ml_tensor_shape` |
| `domain-primitives.ml-feature-normalization` | `ml_feature_normalization` |
| `domain-primitives.ml-inference-envelope` | `ml_inference_envelope` |
| `domain-primitives.ml-confusion-matrix` | `ml_confusion_matrix` |
| `domain-primitives.physics-units` | `physics_units` |
| `domain-primitives.physics-integrator-step` | `physics_integrator_step` |
| `domain-primitives.physics-collision-envelope` | `physics_collision_envelope` |
| `domain-primitives.physics-energy-budget` | `physics_energy_budget` |
| `domain-primitives.vision-image-buffer` | `vision_image_buffer` |
| `domain-primitives.vision-roi` | `vision_roi` |
| `domain-primitives.vision-convolution-window` | `vision_convolution_window` |
| `domain-primitives.vision-detection-nms` | `vision_detection_nms` |

Every module task below includes the same completion criteria:

- define the module's primary `*_valid_spec(...)` or `*_accepted_spec(...)`
- state constructor/checker rejection vs. explicit `requires` validity boundary
- provide at least one proof lemma named after the exported fact, when the task
  says "prove"
- add `docs/requirements.md` entry plus `r[impl]`, `r[depends]`, and `r[verify]`
  tags for public/runtime/spec/proof coverage as appropriate
- add at least one positive and one negative runtime test for public exec
  surfaces; proof-only modules must add proof evidence instead
- keep exec paths iterative for scans/windows and avoid broad `assume` or new
  `external_body` sites

## Phase 1: Specification foundation

- [x] Create proposal, design, tasks, and `domain-primitives` OpenSpec text for GIS, ML, physics, and machine-vision primitives.

## Phase 2: GIS primitives

- [ ] Implement and prove `gis_coordinate_envelope`; cover valid scaled lat/lon and out-of-range rejection. `[covers=domain-primitives.gis-coordinate-envelope]`
- [ ] Implement and prove `gis_bounding_box`; cover normalized containment/intersection and inverted-box rejection. `[covers=domain-primitives.gis-bounding-box]`
- [ ] Implement and prove `gis_tile_index`; cover valid tile address, parent/child containment, and zoom/x/y bounds rejection. `[covers=domain-primitives.gis-tile-index]`
- [ ] Implement and prove `gis_linestring_topology`; cover valid segment chain and degenerate/self-intersection rejection for the supported discrete model. `[covers=domain-primitives.gis-linestring-topology]`

## Phase 3: ML primitives

- [ ] Implement and prove `ml_tensor_shape`; cover rank/shape/stride validity and safe flattened-index bounds. `[covers=domain-primitives.ml-tensor-shape]`
- [ ] Implement and prove `ml_feature_normalization`; cover bounded normalized outputs and missing/zero-denominator rejection. `[covers=domain-primitives.ml-feature-normalization]`
- [ ] Implement and prove `ml_inference_envelope`; cover deterministic class/regression admission bounded by metadata and invalid-output rejection. `[covers=domain-primitives.ml-inference-envelope]`
- [ ] Implement and prove `ml_confusion_matrix`; cover bounded count updates and denominator-safe metric derivation. `[covers=domain-primitives.ml-confusion-matrix]`

## Phase 4: Physics primitives

- [ ] Implement and prove `physics_units`; cover dimension-compatible operations and incompatible-unit rejection. `[covers=domain-primitives.physics-units]`
- [ ] Implement and prove `physics_integrator_step`; cover monotone time, finite bounded state, and invalid-step rejection. `[covers=domain-primitives.physics-integrator-step]`
- [ ] Implement and prove `physics_collision_envelope`; cover conservative swept-envelope overlap and separated-object rejection. `[covers=domain-primitives.physics-collision-envelope]`
- [ ] Implement and prove `physics_energy_budget`; cover non-negative energy accounting and bounded drift rejection. `[covers=domain-primitives.physics-energy-budget]`

## Phase 5: Machine-vision primitives

- [ ] Implement and prove `vision_image_buffer`; cover width/height/stride/channel layout and out-of-bounds buffer rejection. `[covers=domain-primitives.vision-image-buffer]`
- [ ] Implement and prove `vision_roi`; cover bounded ROI cropping/mapping and invalid-region rejection. `[covers=domain-primitives.vision-roi]`
- [ ] Implement and prove `vision_convolution_window`; cover safe kernel window indexing and border-policy rejection/handling. `[covers=domain-primitives.vision-convolution-window]`
- [ ] Implement and prove `vision_detection_nms`; cover thresholded score ordering, non-overlap suppression, and invalid-box rejection. `[covers=domain-primitives.vision-detection-nms]`

## Phase 6: Validation and archive

- [ ] Sync completed requirement text to `docs/requirements.md` and canonical `openspec/specs/domain-primitives/spec.md`.
- [ ] Update `src/lib.rs` exports only as each verified module lands.
- [ ] Run focused module tests for all public exec primitives and capture positive/negative evidence.
- [ ] Run `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet` and capture evidence.
- [ ] Run `./scripts/verus.sh src/lib.rs` and capture full-library proof evidence.
- [ ] Run `tracey query uncovered` and `tracey query untested` and capture zero uncovered / zero untested evidence for the final tree.
- [ ] Run OpenSpec proposal/design/tasks gates after implementation evidence is synced.
- [ ] Archive only after all implementation, validation, and gate tasks are complete.
