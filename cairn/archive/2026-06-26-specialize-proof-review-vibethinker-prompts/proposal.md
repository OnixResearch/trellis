## Why

The proof-review scanner is effective at selecting targets, but its generated VibeThinker instruction is generic. Recent runs showed VibeThinker is most useful when asked narrow, rule-specific questions, and least useful when asked to broadly critique already-obvious `ensures true` findings.

## What Changes

- Generate specialized VibeThinker prompt guidance per finding rule in `scripts/proof-review-candidates.rs`.
- Include output constraints that forbid generic repetition, testing proof functions, or suggesting new trust boundaries.
- Keep the scanner deterministic and repo-owned; VibeThinker remains advisory and validation remains cargo, Verus, Tracey, Cairn, and repo checkers.
- Extend the script self-test to assert that generated packets include the specialized prompt sections.

## Impact

- **Files**: `scripts/proof-review-candidates.rs`, proof-review generated output/evidence, and Cairn archive artifacts.
- **Testing**: script self-test, focused scanner run with `--check-policy`, Cairn validation/gates, and `TMPDIR=/tmp ./scripts/verify.rs` before archive.
