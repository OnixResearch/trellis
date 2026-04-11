## Phase 1: Durable History Model

- [ ] Define durable event record and event-kind constants for work,
      effect completion, and terminal outcomes
- [ ] Add `history_valid` spec+exec for append-only ordered histories
- [ ] Add `durable_prefix_len` spec+exec for contiguous durable prefix
- [ ] Add `can_append_event` / `append_allowed` rule for ordered growth

## Phase 2: Checkpoint Logic

- [ ] Add `advance_checkpoint` spec+exec bounded by durable prefix
- [ ] Prove checkpoint monotonicity
- [ ] Prove checkpoint never exceeds contiguous durable history
- [ ] Prove checkpoint cannot skip a gap in prior steps

## Phase 3: Replay And Effect Dedup

- [ ] Add `replayable_event_at` spec+exec for bounded replay reads
- [ ] Add `should_execute_effect` spec+exec keyed by durable effect markers
- [ ] Prove durable effect marker implies no fresh re-execution
- [ ] Prove unrelated effect ids do not block each other

## Phase 4: Crash Recovery And Terminality

- [ ] Add `is_terminal_execution` spec+exec from durable history
- [ ] Add `resume_step` spec+exec derived from checkpoint + prefix + terminal state
- [ ] Prove resume never rewinds below checkpoint
- [ ] Prove resume never skips over first non-durable step
- [ ] Prove post-terminal appends are rejected
- [ ] Prove recovery from terminal history reproduces terminal outcome

## Phase 5: Cross-Module Properties And Tests

- [ ] Cross-module property: durable effect suppression agrees with
      `job_idempotency` duplicate-prevention intuition
- [ ] Cross-module property: durable history sequence numbers satisfy
      `monotonic::is_strict_advance`
- [ ] Cross-module property: append-only history composes with
      `journal`-style prefix preservation
- [ ] Unit tests for contiguous prefixes, gaps, replay bounds, and
      post-terminal rejection
- [ ] Proptest for random histories: checkpoint boundedness, resume
      boundedness, and at-most-once effect execution
- [ ] Verus clean, cargo test clean, tracey clean
