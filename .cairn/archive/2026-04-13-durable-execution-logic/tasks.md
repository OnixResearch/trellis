## Phase 1: Durable History Model

- [x] Define durable event record and event-kind constants for work,
      effect completion, and terminal outcomes
- [x] Add `history_valid` spec+exec for append-only ordered histories
- [x] Add `durable_prefix_len` spec+exec for contiguous durable prefix
- [x] Add `can_append_event` / `append_allowed` rule for ordered growth

## Phase 2: Checkpoint Logic

- [x] Add `advance_checkpoint` spec+exec bounded by durable prefix
- [x] Prove checkpoint monotonicity
- [x] Prove checkpoint never exceeds contiguous durable history
- [x] Prove checkpoint cannot skip a gap in prior steps

## Phase 3: Replay And Effect Dedup

- [x] Add `replayable_event_at` spec+exec for bounded replay reads
- [x] Add `should_execute_effect` spec+exec keyed by durable effect markers
- [x] Prove durable effect marker implies no fresh re-execution
- [x] Prove unrelated effect ids do not block each other

## Phase 4: Crash Recovery And Terminality

- [x] Add `is_terminal_execution` spec+exec from durable history
- [x] Add `resume_step` spec+exec derived from checkpoint + prefix + terminal state
- [x] Prove resume never rewinds below checkpoint
- [x] Prove resume never skips over first non-durable step
- [x] Prove post-terminal appends are rejected
- [x] Prove recovery from terminal history reproduces terminal outcome

## Phase 5: Cross-Module Properties And Tests

- [x] Cross-module property: durable effect suppression agrees with
      `job_idempotency` duplicate-prevention intuition
- [x] Cross-module property: durable history sequence numbers satisfy
      `monotonic::is_strict_advance`
- [x] Cross-module property: append-only history composes with
      `journal`-style prefix preservation
- [x] Unit tests for contiguous prefixes, gaps, replay bounds, and
      post-terminal rejection
- [x] Proptest for random histories: checkpoint boundedness, resume
      boundedness, and at-most-once effect execution
- [x] Verus clean, cargo test clean, tracey clean
