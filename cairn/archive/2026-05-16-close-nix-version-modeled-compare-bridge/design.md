## Context

`compare_versions_spec` is already modeled by recursive spec functions and proof lemmas. The full runtime `compare_versions` remains an `external_body` due to solver/loop complexity. A previous decomposition added finite admission checks and `compare_versions_modeled`, but admitted inputs still call the trusted runtime comparator. That leaves the finite wrapper useful as a boundary marker, not as a verified executable comparator.

## Goals / Non-Goals

**Goals:**
- Add verified executable helper(s) for the finite admitted subset.
- Preserve the existing spec semantics and result range `{-1, 0, 1}`.
- Make the admitted modeled path independent of the trusted `compare_versions` body.
- Keep proof-gap inventory honest if the full bridge remains trusted.

**Non-Goals:**
- Close the full `compare_versions` external body in this slice.
- Change public Nix version ordering behavior.
- Model arbitrary-large numeric components beyond the admitted finite bound.

## Decisions

### 1. Verified modeled path, trusted general path

**Choice:** Introduce a separate modeled comparator path for admitted inputs and keep `compare_versions` as the broader trusted runtime API.

**Rationale:** This maximizes verified evidence without pretending the harder unbounded bridge is closed.

### 2. Mirror spec decomposition

**Choice:** Reuse or mirror `skip_seps`, component end, numeric parse, and lexicographic comparison with small executable helpers whose postconditions connect to the existing spec functions.

**Rationale:** Aligning executable helpers with existing spec functions reduces semantic drift and lets focused Verus evidence prove the wrapper result.

### 3. Preserve explicit `None` for out-of-model inputs

**Choice:** Keep `compare_versions_modeled` as `Option<i8>` and return `None` outside finite admission.

**Rationale:** The wrapper should state exactly where verified executable evidence applies and avoid overclaiming broader Nix runtime behavior.

## Risks / Trade-offs

**Solver churn in loop/spec refinement** → Keep helpers small, use bounded admitted preconditions, and add bridge lemmas incrementally.

**Inventory overclaiming** → Do not remove or recount `compare_versions` unless the full external body is actually closed and the manifest/checker are updated.

**Tracey drift** → Add the new requirement ID to implementation and proof annotations before final validation.
