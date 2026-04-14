# Dead Code Elimination — Validation Evidence

## Verus

6629 verified, 0 errors. See `verus-transcript.txt`.

## Tests

3563 passed, 0 failed. Includes 18 DCE-specific tests:
- 6 unit tests (classification, elimination, semantics)
- 6 proptests (classification, oracle comparison, trace preservation, idempotence)
- 6 edge-case unit tests (boundaries, cascading, blocks/edges, idempotence)

See `test-transcript.txt`.

## Clippy

Clean under `-D warnings`. See `clippy-transcript.txt`.

## Tracey

0 uncovered out of 1430 rules, 0 untested. See `tracey-transcript.txt`.

## Requirements Covered

| Requirement | impl | verify |
|---|---|---|
| `dce.side-effect` | `has_side_effect` | unit + proptest |
| `dce.criterion` | `is_dead` | unit + proptest |
| `dce.side-effect-never-dead` | `side_effect_never_dead` proof | unit + proptest |
| `dce.live-def-never-dead` | `live_def_never_dead` proof | unit + proptest |
| `dce.no-live-removed` | `eliminate_dead` | unit + proptest |
| `dce.semantics` | `dce_preserves_observable_trace` proof | unit + proptest |
| `dce.retained-identical` | `retained_instruction_identical` proof | via semantics |
| `dce.removed-no-effect` | `dead_no_observable_effect` proof | via semantics |
| `dce.idempotent` | `eliminate_block_idempotent` proof | unit + proptest |
