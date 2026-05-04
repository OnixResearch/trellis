## Context

The domain-primitives umbrella now tracks identifier/namespace, text/string,
serialization, and storage/index families. Filesystem path handling is a
cross-cutting primitive family that should be separated from those families so
future proofs can reason about finite path shapes without inheriting OS
filesystem or security claims.

## Goals / Non-Goals

**Goals:**

- specify exact finite path-segment and path-form envelopes
- specify deterministic normalization decisions without claiming OS
  canonicalization
- specify finite root/scope containment and relative traversal rejection
- implement and verify extension/type metadata, join/split envelopes, and path admission as exact finite primitives

**Non-Goals:**

- proving OS filesystem behavior, symlink resolution, permissions, durability,
  security, or sandbox enforcement
- proving URI/URL semantics, remote object store behavior, or external standard
  conformance
- claiming locale/case-folding, performance, caching, or UI rendering behavior

## Decisions

### 1. Model paths as finite segment sequences

**Choice:** Path primitives use finite segment sequences plus explicit form
metadata rather than host `Path` canonicalization or filesystem access.

**Rationale:** Segment sequences are Verus-friendly and compose with text/string,
identifier/namespace, and storage/index primitives.

**Alternative:** Start with OS path canonicalization semantics. Rejected because
OS-specific filesystem behavior is too broad and impure for this reusable
primitive slice.

### 2. Treat normalization as metadata

**Choice:** Normalization primitives record accepted, rejected, unchanged, or
rewritten segment sequences under declared rules.

**Rationale:** This captures deterministic bookkeeping without claiming symlink,
mount, realpath, case-folding, or encoding correctness.

### 3. Keep containment and traversal finite

**Choice:** Containment and traversal checks operate only over declared finite
root/scope segment prefixes and explicit `.`/`..`-style traversal metadata.

**Rationale:** This supports useful admission proofs without claiming sandbox
security or runtime enforcement.

## Risks / Trade-offs

**Overlap with text/string** → This child owns path segment/form semantics;
text/string owns byte strings, spans, delimiters, and normalization envelopes.

**Security drift** → Requirements explicitly avoid sandbox, permission,
anti-traversal exploit, and TOCTOU claims. Security policies must live in later
focused changes.

**OS-specific overclaim** → Requirements must not rely on host filesystem,
symlink, mount, case-folding, or canonicalization behavior.

## Validation Plan

- `cargo fmt`
- `cargo test --quiet fs_path_`
- `./scripts/verus.sh src/lib.rs`
- `tracey query uncovered --prefix domain-primitives.filesystem-path`
- `tracey query untested --prefix domain-primitives.filesystem-path`
- `openspec validate add-filesystem-path-primitives --strict --json`
- `openspec validate domain-primitives --strict --json`
- `git diff --check`
- `./scripts/verify.sh`
