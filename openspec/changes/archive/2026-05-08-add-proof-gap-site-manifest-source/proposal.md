## Why

The exact proof-gap site list is currently embedded in the Rust checker. That is executable, but review diffs for baseline changes would be clearer if the expected site list lived in a small manifest consumed by the Rust script.

## What Changes

- Introduce a checked proof-gap site manifest as the expected-site source of truth.
- Update the Rust checker to parse the manifest and compare it to live source.
- Add stale/malformed manifest negative evidence.

## Scope

In scope: the requirement and task set in this change.

Out of scope:
- Replacing the Rust checker with Python or shell logic.
- Changing trusted-boundary counts or classifications.
- Introducing dynamic network or external data sources.

## Verification

Validate the active OpenSpec, implement only after the spec-foundation commit, capture evidence under this change, then sync/archive when all implementation tasks are complete.
