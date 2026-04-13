# Original Draft -> Landed Scope Mapping

This file makes the archived `complete` status auditable. The first draft of
this change used a stronger or less accurate description in four places. The
landed implementation corrected those items to match what was actually proved
and tested.

## Material scope corrections

| Original draft item | Landed item | Why changed | Landed evidence |
|---|---|---|---|
| Implement backward iterative analysis with reverse-postorder processing | Implement synchronous backward whole-graph rounds from empty live sets | Implementation uses whole-graph rounds, not a worklist / RPO scheduler | `src/liveness_analysis.rs`, `final-tasks.md`, `verify.txt` |
| Prove convergence within `V × B` iterations | Prove convergence within `2 × V × B` rounds | In this model `live_in` and `live_out` bits can stabilize one round apart; a self-loop with a local use is the counterexample to `V × B` | `src/liveness_analysis.rs`, `cargo-test.txt`, `verus-lib.txt`, `verify.txt` |
| Prove connected live ranges: live set forms connected subgraph | Prove witness-path blocks stay inside `live_range` | Landed proof/test obligation is path-local connectivity, which is what the module models directly | `src/liveness_analysis.rs`, `cargo-test.txt` |
| Prove exit boundary: non-return variables not live-out of exit | Prove successor-free blocks have empty `live_out`; return use modeled as local use | Model keeps exit `live_out` empty and represents returns inside the block's `use` row | `src/liveness_analysis.rs`, `cargo-test.txt`, `verify.txt` |

## Tasks that remained one-to-one

The remaining drafted work shipped without semantic change:
- `use_set` / `def_set` row projections
- live-in / live-out equations
- transfer monotonicity
- monotonic growth across rounds
- fixed-point stability check
- use-def characterization
- def-kills property
- `live_range` projection
- exec APIs `compute_liveness`, `compute_liveness_with_rounds`, `check_equations`, `is_live_at`
- unit tests and proptests covering equations, use-def, exit boundary, and witness-path connectivity

## Code and evidence pointers

- Implementation: `src/liveness_analysis.rs`
- Export: `src/lib.rs`
- Tracey requirements: `docs/requirements.md`
- Verification transcripts:
  - `evidence/cargo-test.txt`
  - `evidence/cargo-clippy.txt`
  - `evidence/verus-lib.txt`
  - `evidence/tracey-uncovered.txt`
  - `evidence/tracey-untested.txt`
  - `evidence/verify.txt`
- Review diffs:
  - `evidence/implementation.diff`
  - `evidence/implementation.stat`
