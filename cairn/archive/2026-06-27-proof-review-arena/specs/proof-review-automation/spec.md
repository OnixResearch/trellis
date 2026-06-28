## ADDED Requirements

### Requirement: Bounded Proof Review Iteration for src/arena.rs

The proof-review iteration for `src/arena.rs` MUST strengthen the selected target without adding trust boundaries, MUST include positive and negative executable tests when the target has runtime behavior, and MUST archive scanner, advisory packet, validation, sync, archive, and final status evidence before claiming completion.

r[proof-review.iteration.proof.review.arena]

#### Scenario: Iteration archives only after validation

- GIVEN `src/arena.rs` has been selected by the proof-review loop
- WHEN the change is prepared for archive
- THEN focused checks, Verus, Tracey, Cairn gates, and repo verification evidence MUST be present and successful
