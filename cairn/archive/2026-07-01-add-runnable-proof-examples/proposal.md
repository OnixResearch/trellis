## Why

Trellis has a strong contract-first house style: a `spec fn` mathematical model, an `exec fn` implementation, and an `ensures result == spec(args)` refinement checked by Verus. The README shows the pattern, but users need runnable examples they can copy, execute, and intentionally break to understand how positive and negative proof evidence behaves.

Runnable examples are the Trellis-owned counterpart to Octet's adoption story. Trellis should teach how verified logic is authored and checked; Octet can later consume the resulting evidence in release gates.

## What Changes

- Add a small runnable proof-example suite that demonstrates `spec fn` + `exec fn` + `ensures`, proof functions, trusted-boundary avoidance, and requirement IDs.
- Include positive examples that pass normal repository verification and negative examples that are expected to fail under a documented command or checker.
- Document exactly which commands users run for normal Cargo tests, focused Verus checks, and intentional failing examples.
- Ensure examples are deterministic, dependency-light, and tied to Tracey requirement coverage without creating new proof gaps.

## Scope

In scope: example source files, example docs, positive and negative fixtures, focused checker commands, Tracey tags, and README/module-index links.

Out of scope:
- New verification theory beyond example-sized modules.
- Runtime benchmarking or ghost-erasure evidence already covered by `add-runtime-cost-evidence`.
- Domain-shaped external adapters already covered by `add-integration-adapters`.
- Marketing claims that examples prove whole downstream systems correct.

## Verification

Run the documented example commands, positive and negative example checks, cargo tests, clippy, Tracey uncovered/untested checks, Verus, Cairn validation/gates, and full repository verification as appropriate for touched files.
