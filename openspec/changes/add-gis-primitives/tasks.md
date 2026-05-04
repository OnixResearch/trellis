## Coverage Index

| Requirement ID | Module task |
|---|---|
| `domain-primitives.bounded-interval` | `bounded_interval` |
| `domain-primitives.rect-grid-index` | `rect_grid_index` |
| `domain-primitives.gis-coordinate-envelope` | `gis_coordinate_envelope` |
| `domain-primitives.gis-bounding-box` | `gis_bounding_box` |
| `domain-primitives.gis-tile-index` | `gis_tile_index` |
| `domain-primitives.gis-linestring-topology` | `gis_linestring_topology` |

Every module task includes the same completion criteria:

- define primary `*_valid_spec(...)` / `*_accepted_spec(...)` predicates
- state constructor/checker rejection vs. proof-only `requires` boundary
- add `docs/requirements.md` entry plus `r[impl]`, `r[depends]`, and `r[verify]` tags
- add positive and negative runtime tests for public exec surfaces
- avoid new `assume`, `external_body`, or runtime-only exemption sites
- capture focused cargo and Verus evidence under this change before marking done

## Phase 1: Foundation primitives

- [x] Implement and prove `bounded_interval`; cover normalized admission, containment, overlap symmetry, disjoint rejection, and clamp/range preservation. `[covers=domain-primitives.bounded-interval]`
- [x] Implement and prove `rect_grid_index`; cover valid coordinate admission, deterministic row-major flattening for accepted coordinates, and out-of-range coordinate rejection. `[covers=domain-primitives.rect-grid-index]`

## Phase 2: GIS range and rectangle primitives

- [x] Implement and prove `gis_coordinate_envelope`; compose bounded interval facts for valid scaled lat/lon or projected coordinates and out-of-range rejection. `[covers=domain-primitives.gis-coordinate-envelope]`
- [x] Implement and prove `gis_bounding_box`; compose interval facts for normalized containment/intersection and inverted-box rejection. `[covers=domain-primitives.gis-bounding-box]`

## Phase 3: GIS tile and topology primitives

- [x] Implement and prove `gis_tile_index`; compose grid/index facts for valid tile address, parent/child containment, zoom bounds, and x/y rejection. `[covers=domain-primitives.gis-tile-index]`
- [x] Implement and prove `gis_linestring_topology`; cover non-degenerate segment chains and explicit degenerate/unsupported-self-intersection rejection for the supported discrete model. `[covers=domain-primitives.gis-linestring-topology]`

## Phase 4: Validation and archive

- [x] Sync completed requirement text to `docs/requirements.md` and canonical `openspec/specs/domain-primitives/spec.md`.
- [x] Update `src/lib.rs` exports only as each verified module lands.
- [x] Run focused tests for all six modules and capture positive/negative evidence.
- [x] Run `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet` and capture evidence.
- [x] Run `./scripts/verus.sh src/lib.rs` and capture full-library proof evidence.
- [x] Run `tracey query uncovered` and `tracey query untested` and capture zero uncovered / zero untested evidence for the final tree.
- [x] Run OpenSpec proposal/design/tasks gates after implementation evidence is synced.
- [x] Archive only after all implementation, validation, and gate tasks are complete.
