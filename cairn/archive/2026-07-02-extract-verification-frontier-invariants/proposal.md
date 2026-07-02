## Why

The verification-frontier module classifies finite admission results as `Proven`, `Rejected`, or `Unknown`, but future callers need reusable invariants rather than ad-hoc case analysis. Explicit invariant lemmas will keep `Unknown` from being treated as proof, keep rejected metadata deterministic, and make cross-module consumers safer.

## What Changes

- Add a named invariant bundle for verification-frontier outcomes and admission records.
- Expose reusable proof helpers for proven, rejected, unknown, reason-code, and evidence-shape facts.
- Add positive and negative tests for every public admission family.

## Scope

In scope: `src/verification_frontier.rs`, accepted verification-frontier spec deltas, Tracey markers, tests/proptests for admission boundaries, and documentation of reusable invariant facts.

Out of scope:
- Trusting external Lattice/Salt implementation behavior.
- Adding solver calls or runtime integrations.
- Reclassifying `Unknown` as acceptable proof evidence.

## Verification

Run focused `cargo test --quiet --lib verification_frontier`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested checks, and Cairn validate/gates.
