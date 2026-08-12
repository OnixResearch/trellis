# Durable Execution Specification

## Purpose

Defines correctness properties for durable execution decision logic:
append-only execution history, monotonic durable checkpoints,
replay-vs-execute decisions, crash-recovery resume points, and terminal
execution persistence. These are pure functions with no storage or I/O —
the runtime executor calls them to decide what is safe after failures
and restarts.

## Requirements

### Durable History

### Requirement: History Is Append-Only And Ordered
r[durable-execution.history-is-append-only-and-ordered]

`history_valid` MUST return true only when durable execution events are
strictly ordered by sequence number and each new event extends the prior
history instead of rewriting earlier events.

#### Scenario: Strict append

- GIVEN history with last sequence number 7
- WHEN appending a new durable event with sequence number 8
- THEN the extended history remains valid

#### Scenario: Gap in sequence

- GIVEN history with last sequence number 7
- WHEN appending a new durable event with sequence number 9
- THEN the history MUST be invalid

#### Scenario: Duplicate sequence number

- GIVEN history with last sequence number 7
- WHEN appending a new durable event with sequence number 7
- THEN the history MUST be invalid

### Requirement: Durable Prefix Is Contiguous
r[durable-execution.durable-prefix-is-contiguous]

`durable_prefix_len` MUST return the largest prefix length such that all
steps before that point are durably recorded with no gaps.

#### Scenario: Fully contiguous history

- GIVEN durable steps [0, 1, 2, 3]
- WHEN computing durable_prefix_len
- THEN MUST return 4

#### Scenario: Gap after step 1

- GIVEN durable steps [0, 1, 3]
- WHEN computing durable_prefix_len
- THEN MUST return 2

#### Scenario: Empty history

- GIVEN no durable events
- WHEN computing durable_prefix_len
- THEN MUST return 0

### Durable Checkpoints

### Requirement: Checkpoint Advance Is Monotonic
r[durable-execution.checkpoint-advance-is-monotonic]

`advance_checkpoint` MUST never move backward. A new checkpoint MUST be
at least the old checkpoint and at most the contiguous durable prefix.

#### Scenario: Advance to next durable step

- GIVEN old checkpoint = 2 and durable prefix covers steps [0, 1, 2, 3]
- WHEN advancing the checkpoint
- THEN the new checkpoint MUST be 3 or 4, but not less than 2

#### Scenario: Gap blocks checkpoint

- GIVEN old checkpoint = 2 and durable steps [0, 1, 2, 4]
- WHEN advancing the checkpoint
- THEN the new checkpoint MUST remain 3 at most; it MUST NOT skip to 5

#### Scenario: No durable progress

- GIVEN old checkpoint already equals the durable prefix
- WHEN advancing the checkpoint
- THEN the checkpoint MUST stay unchanged

### Requirement: Checkpoint Cannot Skip Uncommitted Work
r[durable-execution.checkpoint-cannot-skip-uncommitted-work]

A checkpoint MUST NOT claim durability for a step unless all prior steps
in the execution are already durably recorded.

#### Scenario: Prefix commit allowed

- GIVEN durable steps [0, 1, 2]
- WHEN checkpointing through step 2
- THEN the checkpoint is valid

#### Scenario: Hole before target

- GIVEN durable steps [0, 2]
- WHEN checkpointing through step 2
- THEN the checkpoint MUST be invalid

### Replay Decisions

### Requirement: Replay Reads Only Durable History
r[durable-execution.replay-reads-only-durable-history]

`replayable_event_at` MUST return recorded history only for indices
strictly within the durable prefix. Reads beyond that boundary MUST be
rejected.

#### Scenario: Read durable event

- GIVEN durable prefix length = 3
- WHEN replay reads event index 1
- THEN the recorded event MUST be returned

#### Scenario: Read beyond prefix

- GIVEN durable prefix length = 3
- WHEN replay reads event index 3
- THEN the read MUST be rejected

### Requirement: Side Effects Are Executed At Most Once
r[durable-execution.side-effects-are-executed-at-most-once]

`should_execute_effect` MUST return false when a matching durable effect
completion marker already exists. In that case replay MUST use the
recorded result instead of authorizing a fresh effect execution.

#### Scenario: First execution of effect

- GIVEN no durable marker for effect E at step 5
- WHEN checking should_execute_effect
- THEN MUST return true

#### Scenario: Replay of completed effect

- GIVEN durable history already contains completion marker for effect E at step 5
- WHEN checking should_execute_effect
- THEN MUST return false
- AND replay MUST use the recorded result

#### Scenario: Different effect id

- GIVEN durable marker for effect A at step 5
- WHEN checking effect B at step 5
- THEN effect B is unaffected by effect A's marker

### Crash Recovery Resume

### Requirement: Resume Starts At First Non-Durable Step
r[durable-execution.resume-starts-at-first-non-durable-step]

`resume_step` MUST return `Some(step)` for the first step not yet covered
by the durable checkpoint / contiguous durable prefix. If durable history
already fixes a terminal outcome, `resume_step` MUST return `None`.

#### Scenario: Resume after three durable steps

- GIVEN durable prefix covers steps [0, 1, 2]
- WHEN computing resume_step
- THEN MUST return Some(3)

#### Scenario: Fresh execution

- GIVEN empty durable history
- WHEN computing resume_step
- THEN MUST return Some(0)

#### Scenario: Terminal history

- GIVEN durable history contains a terminal event at step 4
- WHEN computing resume_step
- THEN MUST return None

### Requirement: Resume Never Rewinds Or Skips Ahead
r[durable-execution.resume-never-rewinds-or-skips-ahead]

When `resume_step` returns `Some(step)` for a caller-provided valid
checkpoint, the step MUST be at least that checkpoint and at most the
contiguous durable prefix. If the caller provides an already-invalid
checkpoint beyond the durable prefix, recovery MAY preserve it unchanged
rather than silently rewinding it. Recovery MUST NOT repeat durably
committed prefix work as new work, and MUST NOT skip over missing
durable steps.

#### Scenario: No rewind

- GIVEN valid durable checkpoint = 6
- WHEN computing resume_step on non-terminal history
- THEN the result MUST be Some(step) with step >= 6

#### Scenario: No skip over gap

- GIVEN durable history has a gap before step 8
- WHEN computing resume_step from a valid checkpoint on non-terminal history
- THEN the result MUST stay at or before the first gap

### Terminal Durability

### Requirement: Durable Terminal State Is Sticky
r[durable-execution.durable-terminal-state-is-sticky]

`is_terminal_execution` MUST return true once a terminal durable event
(success, failure, or cancellation) appears in the valid durable
history. Adding more non-terminal events after that MUST be rejected.

#### Scenario: Durable completion

- GIVEN durable history contains a completed event
- WHEN checking is_terminal_execution
- THEN MUST return true

#### Scenario: Durable failure

- GIVEN durable history contains a failed event
- WHEN checking is_terminal_execution
- THEN MUST return true

#### Scenario: Attempted post-terminal append

- GIVEN durable history already contains a terminal event
- WHEN appending a new work event
- THEN the append MUST be invalid

### Requirement: Replay Of Terminal Execution Is Deterministic
r[durable-execution.replay-of-terminal-execution-is-deterministic]

When an execution is terminal in durable history, `recovered_status`
MUST reproduce that terminal outcome rather than reopening the run.

#### Scenario: Recover completed run

- GIVEN durable history ends in completion
- WHEN recovering after a crash
- THEN recovered_status MUST be completed

#### Scenario: Recover cancelled run

- GIVEN durable history ends in cancellation
- WHEN recovering after a crash
- THEN recovered_status MUST be cancelled
