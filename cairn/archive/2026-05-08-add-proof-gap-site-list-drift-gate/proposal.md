## Why

The proof-gap inventory drift gate now fails when marker counts change, but count equality alone cannot detect a same-count substitution. A future patch could remove one documented `#[verifier::external_body]` site and introduce another site elsewhere while the count gate still passes.

## What Changes

- Extend the Rust proof-gap inventory checker to validate the exact expected external-body site list by file and function signature.
- Add a fail-closed negative fixture that simulates a same-count site-list substitution without mutating source files.
- Sync verification-gaps requirements and archive evidence for the stricter gate.

## Scope

In scope:
- Rust-script source inventory validation.
- Exact external-body site-list checking against the current audited 16-site baseline.
- Positive and negative evidence under the OpenSpec archive.

Out of scope:
- Removing or reclassifying trusted crypto/hash/runtime/codec boundaries.
- Modeling cryptographic strength, codec correctness, or runtime implementation semantics.
- Changing the expected count baseline unless source inventory actually changes.

## Verification

Validate the active change and canonical `verification-gaps` spec, run the Rust checker in positive and same-count negative fixture modes, run focused Verus/Tracey checks, run `./scripts/verify.sh`, then archive and push.