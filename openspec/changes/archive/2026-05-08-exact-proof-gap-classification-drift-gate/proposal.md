## Why

The proof-gap inventory gate now protects counts and the exact external-body site list, but the audit classification attached to those sites can still drift in prose or evidence. The next governance slice should make permanent-vs-closeable classification executable and reviewable.

## What Changes

- Add an exact site-to-classification baseline for the 16 external-body sites.
- Extend the Rust proof-gap inventory checker to fail when a site classification changes without an intentional spec/update.
- Capture positive and classification-substitution negative evidence.

## Scope

In scope: the requirement and task set in this change.

Out of scope:
- Removing trusted crypto/hash/runtime/codec boundaries.
- Claiming cryptographic strength, codec correctness, or runtime semantics.

## Verification

Validate the active OpenSpec, implement only after the spec-foundation commit, capture evidence under this change, then sync/archive when all implementation tasks are complete.
