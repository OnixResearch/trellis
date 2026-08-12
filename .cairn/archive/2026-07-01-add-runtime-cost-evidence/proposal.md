## Why

The README correctly states that ghost code is erased in normal builds, but users benefit from reproducible evidence showing how to verify that claim locally. Runtime-size and benchmark-style documentation can make the zero-runtime-proof-cost story concrete without overclaiming performance.

## What Changes

- Add reproducible runtime-cost evidence documentation.
- Capture build/profile commands, artifact-size observations, and small runtime checks for representative examples.
- Add guardrails so evidence is command-backed and not a stale marketing claim.

## Scope

In scope: documentation, deterministic local commands, optional lightweight benchmark/example checks, and evidence refresh instructions.

Out of scope:
- Absolute performance guarantees.
- Comparing against unrelated crates.
- Changing proof code for performance optics.

## Verification

Run the documented commands or their checker-backed subset, plus cargo/clippy/doc checks and repository verification appropriate for touched files.
