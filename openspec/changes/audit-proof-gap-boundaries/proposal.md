## Why

The primitive and graph-query OpenSpec queue is drained, leaving the trusted-boundary inventory as the highest-value verification target. The live source audit currently reports 16 `#[verifier::external_body]` sites and no `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, or runtime-only exemption markers. The next slice needs a reviewable inventory and a bounded closeout plan before attempting proof changes.

## What Changes

- Add a source-derived proof-gap audit requirement for the remaining trusted boundaries.
- Classify each boundary as permanent cryptographic/runtime trust or closeable structural proof candidate before implementation.
- Require the first closeable boundary attempt to update code, Tracey coverage, Verus evidence, and published inventory counts together.

## Scope

In scope: inventory scripts/evidence, classification, one narrow closeable-boundary implementation slice, canonical count synchronization, and full verification.

Out of scope: proving cryptographic strength, replacing BLAKE3/Ed25519/base58 implementations, changing runtime codec semantics, or reducing permanent trust boundaries without a concrete structural proof plan.

## Verification

Validate this OpenSpec package, then drain it with focused Verus checks for the touched module, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested checks, source-derived count scripts, and `./scripts/verify.sh`.
