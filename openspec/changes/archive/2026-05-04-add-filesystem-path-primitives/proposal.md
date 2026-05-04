## Why

Path handling often mixes segment parsing, normalization, root containment,
relative traversal checks, extension/type metadata, and admission decisions with
runtime filesystem operations. A focused filesystem-path primitive family gives
future changes exact finite building blocks for path segments, normalized path
decisions, root/scope containment, relative traversal rejection, extension/type
metadata, and deterministic path admission without claiming OS filesystem,
symlink, permission, durability, or security behavior.

## What Changes

- Add implemented exact finite filesystem-path primitives under the domain-primitives umbrella.
- Define and verify finite models for path segments, path form metadata, normalization decisions, root/scope containment, relative traversal rejection, extension/type metadata, path join/split envelopes, and deterministic path admission.
- Capture focused tests, Verus proofs, Tracey coverage, strict OpenSpec validation, and full repository verification evidence.

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

- **Files**: Adds `src/fs_path_*` modules, domain-primitives requirements, canonical OpenSpec sync, tests, and evidence.
- **APIs**: Adds small exact finite Rust/Verus helper predicates only; no host filesystem APIs are called.
- **Dependencies**: no new dependencies.
- **Testing**: focused Rust tests, Verus, Tracey, strict OpenSpec validation, and full repo validation.

## Out of Scope

- OS filesystem semantics, realpath/canonicalize behavior, symlink resolution,
  mount points, case-folding filesystems, permissions, ACLs, durability, or
  storage behavior
- sandbox security, path traversal exploit prevention beyond declared finite
  rejection predicates, TOCTOU behavior, or runtime enforcement claims
- URI/URL semantics, package manager paths, database paths, remote object stores,
  or external standard conformance
- performance, caching, locale behavior, encoding correctness beyond declared
  finite segment envelopes, or UI path rendering
