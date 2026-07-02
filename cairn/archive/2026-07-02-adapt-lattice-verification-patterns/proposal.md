## Why

The Lattice/Salt repository has useful verification-product patterns around compile-time contract implication, type-derived range facts, slice-bound certificates, arena scope provenance, and progress-budget checks. Trellis already has a stronger Verus-centered proof model, so the value is not adopting Salt or its Z3 pass wholesale. The value is adapting the portable ideas into small, mechanically checked Trellis primitives with explicit admission outcomes.

## What Changes

- Add a new `verification-frontier` Cairn spec for Lattice-inspired verification patterns that Trellis may adapt.
- Specify deterministic contract-elision outcomes over finite facts: `Proven`, `Rejected`, and `Unknown`.
- Specify type-bound fact extraction, slice-access certificates, arena scope provenance, and finite progress-budget admission as pure models.
- Require documentation and evidence to describe Lattice as an external design reference only, not as trusted code or validation evidence.

## Scope

In scope: Cairn requirements, future Verus specs/exec helpers, positive and negative tests, README/catalog documentation, Tracey coverage, and validation evidence for the adapted primitives.

Out of scope:

- Replacing Verus with Lattice/Salt's Z3 compiler pass.
- Vendoring or executing Lattice code.
- Claiming end-to-end compiler, OS, scheduler, or runtime-check-elision correctness outside the finite Trellis models.
- Treating an `Unknown` admission result as a proof.

## Verification

Future implementation MUST run relevant baseline checks before core changes, rerun focused positive and negative tests after changes, and finish with the repository's normal cargo, clippy, Tracey, Cairn, and Verus checks appropriate for touched files.
