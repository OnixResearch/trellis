## Why

Trellis is currently strongest as a proof catalog, but a new user cannot quickly run a small program that demonstrates the public APIs. Runnable examples make the crate easier to evaluate, teach the spec/exec style, and catch documentation drift through normal cargo checks.

## What Changes

- Add a curated `examples/` directory for representative stable APIs.
- Cover core arithmetic, quorum/consensus helpers, UCAN/web-of-trust admission, R1CS checks, and allocator/storage-style primitives.
- Wire example compilation into lightweight validation without changing proof semantics.

## Scope

In scope: small runnable examples, README links, and validation that examples compile.

Out of scope:
- New verified algorithms.
- External service integrations.
- Performance claims beyond compiling and running the examples.

## Verification

Run example-focused cargo checks before and after the change, then run the repository documentation/catalog and standard verification gates before sync/archive.
