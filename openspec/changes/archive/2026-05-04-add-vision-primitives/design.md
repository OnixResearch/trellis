## Context

Machine-vision bugs often come from invalid image layout, stride/channel
arithmetic, ROI/window out-of-bounds access, invalid color-layout transforms,
pyramid dimension drift, component label ranges, and detection suppression edge
cases. These can be specified as exact discrete primitives before implementation.

## Goals / Non-Goals

**Goals:** specify exact buffer/layout and geometry/result admission primitives.

**Non-Goals:** prove model accuracy, calibration, codec correctness, floating-point color science, or real-time video behavior.

## Decisions

### 1. Buffer and ROI first

**Choice:** `vision_image_buffer` and `vision_roi` are foundational for every
other vision primitive.

**Rationale:** Windowing, pyramids, components, and detection boxes all depend on
safe image/region indexing.

### 2. Deterministic admission over quality claims

**Choice:** Detection/NMS requirements only state score thresholding, tie-breaking,
valid boxes/classes, and overlap suppression.

**Rationale:** These are verifiable output-admission facts, not model-quality
claims.

## Validation Plan

- `openspec validate add-vision-primitives --strict --json`
- focused cargo tests, Verus, Tracey, full verification, and OpenSpec validation
