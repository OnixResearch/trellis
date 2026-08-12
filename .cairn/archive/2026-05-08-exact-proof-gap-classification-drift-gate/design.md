## Context

The proof-gap inventory gate now protects counts and the exact external-body site list, but the audit classification attached to those sites can still drift in prose or evidence. The next governance slice should make permanent-vs-closeable classification executable and reviewable.

## Goals

- Add an exact site-to-classification baseline for the 16 external-body sites.
- Extend the Rust proof-gap inventory checker to fail when a site classification changes without an intentional spec/update.
- Capture positive and classification-substitution negative evidence.

## Non-Goals

- Removing trusted crypto/hash/runtime/codec boundaries.
- Claiming cryptographic strength, codec correctness, or runtime semantics.

## Decisions

### Bounded finite semantics

The change MUST stay finite, deterministic, and mechanically checkable. Runtime/product claims and external-system conformance are excluded unless a checked predicate in this repo proves the property.

### Evidence boundary

Implementation tasks MUST capture positive and negative evidence before any task is marked complete. The final task remains open until sync, archive, commit, push, and clean-state verification are done.
