## Why

Path handling often mixes segment parsing, normalization, root containment,
relative traversal checks, extension/type metadata, and admission decisions with
runtime filesystem operations. A focused filesystem-path primitive family gives
future changes exact finite building blocks for path segments, normalized path
decisions, root/scope containment, relative traversal rejection, extension/type
metadata, and deterministic path admission without claiming OS filesystem,
symlink, permission, durability, or security behavior.

## What Changes

- Add a spec-only `add-filesystem-path-primitives` child OpenSpec under the
  domain-primitives umbrella.
- Define exact finite models for path segments, path form metadata,
  normalization decisions, root/scope containment, relative traversal rejection,
  extension/type metadata, path join/split envelopes, and deterministic path
  admission.
- Leave implementation, Verus proofs, Tracey coverage, and evidence as future
  tasks.

## Capabilities

### New Capabilities

- `domain-primitives.filesystem-path.segment`: finite path-segment envelopes.
- `domain-primitives.filesystem-path.form`: finite absolute/relative/rooted path
  form metadata.
- `domain-primitives.filesystem-path.normalization`: explicit normalization
  decision envelopes.
- `domain-primitives.filesystem-path.containment`: root/scope containment
  predicates over finite segments.
- `domain-primitives.filesystem-path.traversal`: deterministic relative
  traversal rejection metadata.
- `domain-primitives.filesystem-path.extension-type`: extension/type metadata
  envelopes.
- `domain-primitives.filesystem-path.join-split`: deterministic path join/split
  shape envelopes.
- `domain-primitives.filesystem-path.admission`: deterministic path
  admission/rejection envelopes.

## Impact

- **Files**: OpenSpec artifacts now; future implementation may add
  `src/path_*`, `src/fs_path_*`, `docs/requirements.md`, tests, and evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new dependencies.
- **Testing**: strict OpenSpec validation now; future implementation must run
  focused tests, Verus, Tracey, and final repo validation.

## Out of Scope

- implementing path modules now
- OS filesystem semantics, realpath/canonicalize behavior, symlink resolution,
  mount points, case-folding filesystems, permissions, ACLs, durability, or
  storage behavior
- sandbox security, path traversal exploit prevention beyond declared finite
  rejection predicates, TOCTOU behavior, or runtime enforcement claims
- URI/URL semantics, package manager paths, database paths, remote object stores,
  or external standard conformance
- performance, caching, locale behavior, encoding correctness beyond declared
  finite segment envelopes, or UI path rendering
