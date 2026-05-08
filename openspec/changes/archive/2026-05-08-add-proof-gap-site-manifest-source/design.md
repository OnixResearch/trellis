## Context

The exact proof-gap site list is currently embedded in the Rust checker. That is executable, but review diffs for baseline changes would be clearer if the expected site list lived in a small manifest consumed by the Rust script.

## Goals

- Introduce a checked proof-gap site manifest as the expected-site source of truth.
- Update the Rust checker to parse the manifest and compare it to live source.
- Add stale/malformed manifest negative evidence.

## Non-Goals

- Replacing the Rust checker with Python or shell logic.
- Changing trusted-boundary counts or classifications.
- Introducing dynamic network or external data sources.

## Decisions

### Bounded finite semantics

The change MUST stay finite, deterministic, and mechanically checkable. Runtime/product claims and external-system conformance are excluded unless a checked predicate in this repo proves the property.

### Evidence boundary

Implementation tasks MUST capture positive and negative evidence before any task is marked complete. The final task remains open until sync, archive, commit, push, and clean-state verification are done.
